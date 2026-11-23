//! Bancadas de codificação percentual e conversão de filtros.
//!
//! Todos os benchmarks são puramente computacionais: nenhuma requisição de rede
//! ou operação de arquivo é realizada. Entradas representativas cobrem ASCII,
//! Unicode e alta densidade de caracteres especiais.
#![allow(missing_docs)] // função gerada por criterion_group!
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use gitlab_wrapper::utils::encoding::{encode_query_param, filter_to_query};
use serde::Serialize;

/// Entradas representativas para codificação de parâmetros de consulta.
const ENCODE_CASES: [(&str, &str); 5] = [
    ("ascii_simples", "hello world"),
    ("unicode", "café — © 2026 🚀 / naïve"),
    ("especiais", "a&b=c?d#e/f:g+h%i@j"),
    (
        "query_longa",
        "search=gitlab&page=42&labels=rust,api&sort=desc&scope=all",
    ),
    ("vazio", ""),
];

/// Benchmark de codificação percentual.
fn bench_encode_query_param(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding/encode_query_param");
    for (name, input) in ENCODE_CASES {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| encode_query_param(black_box(input)));
        });
    }
    group.finish();
}

#[derive(Serialize)]
struct ComplexFilter {
    name: String,
    active: bool,
    count: u32,
    tags: Vec<String>,
    nested: NestedFilter,
}

#[derive(Serialize)]
struct NestedFilter {
    group: String,
    labels: Vec<String>,
    metadata: Vec<(String, String)>,
}

fn sample_filter() -> ComplexFilter {
    ComplexFilter {
        name: "gitlab-wrapper-rs".into(),
        active: true,
        count: 42,
        tags: vec!["rust".into(), "api".into(), "gitlab".into(), "http".into()],
        nested: NestedFilter {
            group: "core".into(),
            labels: vec!["encoding".into(), "query".into(), "bench".into()],
            metadata: vec![
                ("owner".into(), "platform".into()),
                ("tier".into(), "1".into()),
            ],
        },
    }
}

/// Benchmark de conversão de filtros.
fn bench_filter_to_query(c: &mut Criterion) {
    let filter = sample_filter();
    let mut group = c.benchmark_group("encoding/filter_to_query");
    group.bench_function("filter_simples", |b| {
        b.iter(|| filter_to_query(black_box(Some(&filter))));
    });
    group.bench_function("filter_ausente", |b| {
        b.iter(|| filter_to_query(black_box(None::<&ComplexFilter>)));
    });
    group.finish();
}

/// Filtro grande e aninhado: 5 níveis de profundidade e 8 filhos por nível.
/// Benchmark da conversão com filtro aninhado.
#[derive(Serialize)]
struct LargeFilter {
    level_1: Vec<LargeBranch>,
    zebra: u32,
    alpha: u32,
}

#[derive(Serialize)]
struct LargeBranch {
    level_2: Vec<LargeLeaf>,
    flag: bool,
}

#[derive(Serialize)]
struct LargeLeaf {
    id: u32,
    name: String,
    value: f64,
    tags: Vec<String>,
}

fn large_filter() -> LargeFilter {
    let branch = |branch_index: u32| LargeBranch {
        level_2: (0..8)
            .map(|leaf_index| LargeLeaf {
                id: branch_index * 8 + leaf_index,
                name: format!("item-{branch_index}-{leaf_index}"),
                value: f64::from(leaf_index) / 8.0,
                tags: vec!["alpha".into(), "beta".into(), "gamma".into()],
            })
            .collect(),
        flag: branch_index.is_multiple_of(2),
    };
    LargeFilter {
        level_1: (0..8).map(branch).collect(),
        zebra: 99,
        alpha: 1,
    }
}

/// Benchmark de filtro grande.
fn bench_filter_to_query_large(c: &mut Criterion) {
    let filter = large_filter();
    let mut group = c.benchmark_group("encoding/filter_to_query_large");
    group.throughput(Throughput::Elements(8 * 8));
    group.bench_function("filter_aninhado_64_folhas", |b| {
        b.iter(|| filter_to_query(black_box(Some(&filter))));
    });
    group.finish();
}
/// Benchmark das conversões de paginação.
///
/// Serialização de `PaginationParams`
/// e conversão para pares de query (o parse de cabeçalhos é `pub(crate)`).
fn bench_pagination_params(c: &mut Criterion) {
    use gitlab_wrapper::PaginationParams;

    let params = PaginationParams {
        page: Some(42),
        per_page: Some(100),
        pagination: Some("offset".into()),
        order_by: Some("created_at".into()),
        sort: Some("desc".into()),
    };
    let mut group = c.benchmark_group("parsing/pagination_params");
    group.bench_function("para_json", |b| {
        b.iter(|| serde_json::to_value(black_box(&params)));
    });
    group.bench_function("para_query", |b| {
        b.iter(|| filter_to_query(black_box(Some(&params))));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_encode_query_param,
    bench_filter_to_query,
    bench_filter_to_query_large,
    bench_pagination_params
);
criterion_main!(benches);
