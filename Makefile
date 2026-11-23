# gitlab-wrapper-rs — alvos de qualidade e bancada (Rust 1.100, crate único).

CARGO ?= cargo +1.100
BENCH_ARGS ?=

.PHONY: check fmt clippy test bench bench-quick clean

## Portão local: formatação, lints e testes.
check: fmt clippy test

## Verifica formatação sem alterar arquivos.
fmt:
	$(CARGO) fmt --all -- --check

## Lints de todos os alvos com warnings como erro.
clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

## Testes da crate (inclui doc-tests e testes de integração).
test:
	$(CARGO) test --all-features

## Bancada Criterion completa: relatório HTML em target/criterion.
## Flags adicionais: make bench BENCH_ARGS="--sample-size 200 --noplot"
bench:
	$(CARGO) bench $(BENCH_ARGS)

## Bancada rápida para iteração local (menos amostras, sem gráficos).
bench-quick:
	$(CARGO) bench -- --warm-up-time 0.5 --measurement-time 1 --sample-size 10 --noplot $(BENCH_ARGS)

## Remove artefatos de build.
clean:
	$(CARGO) clean
