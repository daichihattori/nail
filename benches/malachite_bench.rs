use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use malachite::Natural;

mod common;
use common::*;

// malachite specific test data creation function
fn create_malachite_natural(bits: usize) -> Natural {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    Natural::from_limbs_asc(&data.chunks(8)
        .map(|chunk| {
            let mut limb = 0u64;
            for (i, &byte) in chunk.iter().enumerate() {
                limb |= (byte as u64) << (i * 8);
            }
            limb
        })
        .collect::<Vec<u64>>())
}

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