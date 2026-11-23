# Bancada de benchmarks — gitlab-wrapper-rs

Este documento descreve como reproduzir medições de forma determinística.
Os benchmarks cobrem apenas funções puras (codificação de query, conversão de
filtros e serialização de parâmetros de paginação); não há I/O, rede, relógio ou
aleatoriedade no caminho medido.

## Ambiente fixo

- Toolchain: Rust **1.100** (`rust-toolchain.toml`, Edition 2024).
- Perfil de release do `Cargo.toml`: `opt-level = 3`, `lto = "fat"`,
  `codegen-units = 1`, `strip = "symbols"`, `panic = "abort"`.
- Sistema operacional, CPU, frequência e modo de energia devem ser registrados
  junto com cada execução (fora do escopo deste arquivo).

## Comandos

### Estado anterior (baseline)

O estado anterior ao upgrade é registrado pelo agente principal em
`/tmp/gitlab-wrapper-baseline`. Para reproduzir a partir de um checkout limpo
desse estado:

```bash
cd /tmp/gitlab-wrapper-baseline
cargo +1.100 bench -- --warm-up-time 2 --measurement-time 5 --sample-size 100 --noplot
cp -r target/criterion /tmp/criterion-baseline
```

### Estado atual

```bash
cd /home/one/projects/one/gitlab-wrapper-rs
make bench BENCH_ARGS="--warm-up-time 2 --measurement-time 5 --sample-size 100 --noplot"
cp -r target/criterion /tmp/criterion-current
```

Ou, sem o Makefile:

```bash
cargo +1.100 bench -- --warm-up-time 2 --measurement-time 5 --sample-size 100 --noplot
```

### Iteração rápida (sem comparar)

```bash
make bench-quick
# equivalente a:
# cargo +1.100 bench -- --warm-up-time 0.5 --measurement-time 1 --sample-size 10 --noplot
```

## Parâmetros Criterion fixados

| Parâmetro        | Valor | Finalidade |
|------------------|-------|------------|
| `--warm-up-time` | 2 s   | estabiliza caches e JIT antes da medição |
| `--measurement-time` | 5 s | janela de coleta por caso |
| `--sample-size`  | 100   | número de amostras estatísticas |
| `--noplot`       | ativo | evita dependência de renderização gráfica; mantém JSON/HTML de texto |

Estes valores devem ser idênticos entre baseline e estado atual. A opção
`html_reports` do Criterion gera `target/criterion/report/index.html`.

## Grupos medidos

| Grupo | Casos |
|-------|-------|
| `encoding/encode_query_param` | ASCII simples, Unicode, caracteres especiais, query longa, string vazia |
| `encoding/filter_to_query` | filtro simples e filtro ausente |
| `encoding/filter_to_query_large` | filtro com 64 folhas e 5 níveis de aninhamento |
| `parsing/pagination_params` | `PaginationParams` → JSON e → pares de query |

## Resultados

Medições executadas em 2026-11-23, toolchain 1.100.0, parâmetros
`--warm-up-time 1 --measurement-time 5 --sample-size 100 --noplot`.
Baseline: crate sem alterações (commit `8ae13cc`), 2 benchmarks antigos.
Atual: crate após upgrade; 10 benchmarks com entradas representativas.
Os casos novos não têm baseline direto — a coluna `—` indica isso, não zero.
O A/B isolado da ordenação determinística está descrito após a tabela.

| Grupo/caso | Baseline (mediana) | Atual (mediana) | Delta |
|------------|--------------------|-----------------|-------|
| `encode_query_param` (string antiga) | 194,80 ns | — | não comparável |
| `filter_to_query_simple` (filtro antigo) | 749,14 ns | — | não comparável |
| `encoding/encode_query_param/ascii_simples` | — | 45,28 ns | — |
| `encoding/encode_query_param/unicode` | — | 249,23 ns | — |
| `encoding/encode_query_param/especiais` | — | 232,64 ns | — |
| `encoding/encode_query_param/query_longa` | — | 330,74 ns | — |
| `encoding/encode_query_param/vazio` | — | 8,14 ns | — |
| `encoding/filter_to_query/filter_simples` | — | 2,48 µs | — |
| `encoding/filter_to_query/filter_ausente` | — | 4,31 ns | — |
| `encoding/filter_to_query_large/filter_aninhado_64_folhas` | — | 111,87 µs | — |
| `parsing/pagination_params/para_json` | — | 224,63 ns | — |
| `parsing/pagination_params/para_query` | — | 607,85 ns | — |

### A/B isolado: custo da ordenação determinística

Harness temporário fora do repositório, mesmo filtro de 15 folhas,
20.000 iterações por amostra, release `lto=fat`/`codegen-units=1`:

| Variante | Amostra 1 | Amostra 2 | Interpretação |
|----------|-----------|-----------|---------------|
| sem ordenação (baseline) | 2017,9 ns/op | 2041,8 ns/op | ordem dependente do `HashMap` interno |
| com ordenação (atual) | 2105,8 ns/op | 2099,8 ns/op | (+~3%) ordem lexical estável |

A instrumentação foi removida das funções puras de `src/utils/encoding.rs`
após medir ~2,19 µs contra ~749 ns do baseline no caso antigo; spans agora
ficam nos chamadores HTTP/OAuth, onde a latência de rede domina.
