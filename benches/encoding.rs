use criterion::{Criterion, black_box, criterion_group, criterion_main};
use gitlab_wrapper::utils::encoding::{encode_query_param, filter_to_query};
use serde::Serialize;

fn bench_encode_query_param(c: &mut Criterion) {
    c.bench_function("encode_query_param", |b| {
        b.iter(|| encode_query_param(black_box("hello world & more=special?chars")))
    });
}

#[derive(Serialize)]
struct ComplexFilter {
    name: String,
    active: bool,
    count: u32,
    tags: Vec<String>,
}

fn bench_filter_to_query(c: &mut Criterion) {
    let filter = ComplexFilter {
        name: "test-project".into(),
        active: true,
        count: 42,
        tags: vec!["rust".into(), "api".into(), "gitlab".into()],
    };

    c.bench_function("filter_to_query_simple", |b| {
        b.iter(|| filter_to_query(black_box(Some(&filter))))
    });
}

criterion_group!(benches, bench_encode_query_param, bench_filter_to_query);
criterion_main!(benches);
