use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod common;
use common::*;

fn bench_num_bigint_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("num-bigint Addition", c);
    
    for bits in get_test_bit_sizes() {
        let bigint_a = create_biguint(bits);
        let bigint_b = create_biguint(bits);
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) + black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_num_bigint_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("num-bigint Multiplication", c);
    
    for bits in get_test_bit_sizes() {
        let bigint_a = create_biguint(bits);
        let bigint_b = create_biguint(bits);
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) * black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_num_bigint_addition,
    bench_num_bigint_multiplication
);
criterion_main!(benches);