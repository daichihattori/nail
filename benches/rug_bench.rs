use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod common;
use common::*;

fn bench_rug_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Rug-GMP Addition", c);
    
    for bits in get_test_bit_sizes() {
        let rug_a = create_rug_integer(bits);
        let rug_b = create_rug_integer(bits);
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) + black_box(&rug_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_rug_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Rug-GMP Multiplication", c);
    
    for bits in get_test_bit_sizes() {
        let rug_a = create_rug_integer(bits);
        let rug_b = create_rug_integer(bits);
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) * black_box(&rug_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_rug_addition,
    bench_rug_multiplication
);
criterion_main!(benches);