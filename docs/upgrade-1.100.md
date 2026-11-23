# Upgrade para Rust 1.100 — estado anterior e posterior

Este registro consolida o upgrade do crate para Rust 1.100 / Edition 2024 com
determinismo rigoroso, instrumentação em nível de função, clippy rigoroso e
bancada reproduzível. Os números de performance e o método estão em
[`benchmarks.md`](benchmarks.md).

## Estado anterior (commit `8ae13cc`)

| Item | Resultado observado |
|------|---------------------|
| Toolchain | rustc/cargo 1.100 já disponíveis, mas sem `rust-toolchain.toml`; `rust-version = "1.85"` |
| Formatação | `cargo fmt --all -- --check` falhava |
| Build | `cargo build --locked --all-features` passava, com 2 avisos do Cargo 1.100 (`futures` e `tokio-stream` não usadas) |
| Testes | 35 testes passavam (14 + 9 + 7 + 5) |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` falhava com 101: `unwrap` em testes/examples e variáveis não usadas no exemplo `comprehensive_test` |
| Instrumentação | apenas eventos esparsos (`debug!`/`error!`); nenhum `#[instrument]`, span ou correlation ID nos erros |
| Determinismo | `filter_to_query` herdava a ordem não especificada do `serde_json::Map`; IDs de erro dependedo relógio |
| Panics em biblioteca | `HttpClient::new`, `RateLimiter::acquire` e `generate_code_verifier` usavam `expect` |
| Benchmarks | 2 casos (`encode_query_param`, `filter_to_query_simple`) |

## Estado posterior

| Item | Resultado observado |
|------|---------------------|
| Toolchain | `rust-toolchain.toml` fixa channel `1.100`; `rust-version`, `clippy.toml::msrv` e CI casados |
| Formatação | `cargo fmt --all -- --check` passa com `rustfmt.toml` estável/2024 |
| Build | `cargo build --locked --all-features` passa sem avisos; dependências não usadas removidas |
| Testes | 49 testes passam (17 + 9 + 11 + 7 + 5), incluindo 11 novos testes deterministas |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` passa com grupos `all`/`correctness`/`suspicious`/`style`/`complexity`/`perf` em deny e `pedantic`/`nursery`/`cargo` em warn |
| Instrumentação | spans `#[instrument]` nos fluxos HTTP/OAuth/paginação/rate limit; helpers de correlação em `core::telemetry` com `operation`, `instance`, `ok`, `category` e `count` |
| Determinismo | chaves de `filter_to_query` ordenadas lexicalmente; UUID v7 de produção e sequência determinística para testes, sem env global |
| Panics em biblioteca | eliminados nos construtores e no gerador PKCE; `generate_code_verifier()` agora retorna `Result<String, GitLabError>` |
| Benchmarks | 10 casos nomeados, grupos de codificação/conversão/paginação e parâmetros fixados |

## Performance

A comparação direta dos benchmarks antigos não é justa porque as entradas
mudaram; os valores absolutos e o A/B isolado da mudança determinística estão
em [`benchmarks.md`](benchmarks.md). Resumo:

- a instrumentação em funções puras custou ~3x em `filter_to_query` e foi
  removida; spans ficaram nos chamadores, onde o custo é amortizado por I/O;
- a ordenação das chaves para determinismo custou cerca de 3% no A/B isolado;
- os casos `filter_ausente`, `vazio` e `parsing/pagination_params/para_query`
  ficaram mais rápidos na execução final, mas a comparação com o baseline
  inclui mudanças de compilador/perfil e não deve ser lida como ganho causal
  isolado.

## Como reproduzir

```bash
make check
cargo +1.100 test --locked --all-features
make bench BENCH_ARGS="--warm-up-time 1 --measurement-time 5 --sample-size 100 --noplot"
```

O baseline bruto permanece em `/tmp/gitlab-wrapper-baseline` nesta sessão;
o procedimento de reproução está em [`benchmarks.md`](benchmarks.md).
