use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rug::Integer;

mod common;
use common::*;

// rug specific test data creation function
fn create_rug_integer(bits: usize) -> Integer {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    Integer::from_digits(&data, rug::integer::Order::Lsf)
}

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