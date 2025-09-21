use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod common;
use common::*;

fn bench_malachite_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Malachite Addition", c);
    
    for bits in get_test_bit_sizes() {
        let malachite_a = create_malachite_natural(bits);
        let malachite_b = create_malachite_natural(bits);
        group.bench_with_input(
            BenchmarkId::new("malachite", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&malachite_a) + black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

fn bench_malachite_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("Malachite Multiplication", c);
    
    for bits in get_test_bit_sizes() {
        let malachite_a = create_malachite_natural(bits);
        let malachite_b = create_malachite_natural(bits);
        group.bench_with_input(
            BenchmarkId::new("malachite", bits),
            &bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&malachite_a) * black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_malachite_addition,
    bench_malachite_multiplication
);
criterion_main!(benches);