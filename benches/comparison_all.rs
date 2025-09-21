use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod common;
use common::*;

fn bench_all_libraries_addition(c: &mut Criterion) {
    let mut group = setup_benchmark_group("All Libraries Addition Comparison", c);
    
    for bits in get_test_bit_sizes() {
        // num-bigint
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
        
        // rug
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
        
        // malachite
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
        
        // bigint-rs
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            128 => {
                let a = create_fixed_array_uint_128();
                let b = create_fixed_array_uint_128();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            256 => {
                let a = create_fixed_array_uint_256();
                let b = create_fixed_array_uint_256();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).add(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            _ => unreachable!()
        }
        
        // primitives for reference
        if bits == 64 {
            let u64_a = create_test_u64();
            let u64_b = create_test_u64();
            group.bench_with_input(
                BenchmarkId::new("primitive-u64", bits),
                &bits,
                |bench, _| {
                    bench.iter(|| {
                        let result = black_box(u64_a).wrapping_add(black_box(u64_b));
                        black_box(result)
                    })
                },
            );
        }
        if bits == 128 {
            let u128_a = create_test_u128();
            let u128_b = create_test_u128();
            group.bench_with_input(
                BenchmarkId::new("primitive-u128", bits),
                &bits,
                |bench, _| {
                    bench.iter(|| {
                        let result = black_box(u128_a).wrapping_add(black_box(u128_b));
                        black_box(result)
                    })
                },
            );
        }
    }
    group.finish();
}

fn bench_all_libraries_multiplication(c: &mut Criterion) {
    let mut group = setup_benchmark_group("All Libraries Multiplication Comparison", c);
    
    for bits in get_test_bit_sizes() {
        // num-bigint
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
        
        // rug
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
        
        // malachite
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
        
        // bigint-rs
        match bits {
            64 => {
                let a = create_fixed_array_uint_64();
                let b = create_fixed_array_uint_64();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            128 => {
                let a = create_fixed_array_uint_128();
                let b = create_fixed_array_uint_128();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            256 => {
                let a = create_fixed_array_uint_256();
                let b = create_fixed_array_uint_256();
                group.bench_with_input(
                    BenchmarkId::new("bigint-rs", bits),
                    &bits,
                    |bench, _| {
                        bench.iter(|| {
                            let result = black_box(&a).mul(black_box(&b));
                            black_box(result)
                        })
                    },
                );
            },
            _ => unreachable!()
        }
        
        // primitives for reference
        if bits == 64 {
            let u64_a = create_test_u64();
            let u64_b = create_test_u64();
            group.bench_with_input(
                BenchmarkId::new("primitive-u64", bits),
                &bits,
                |bench, _| {
                    bench.iter(|| {
                        let result = black_box(u64_a).wrapping_mul(black_box(u64_b));
                        black_box(result)
                    })
                },
            );
        }
        if bits == 128 {
            let u128_a = create_test_u128();
            let u128_b = create_test_u128();
            group.bench_with_input(
                BenchmarkId::new("primitive-u128", bits),
                &bits,
                |bench, _| {
                    bench.iter(|| {
                        let result = black_box(u128_a).wrapping_mul(black_box(u128_b));
                        black_box(result)
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_all_libraries_addition,
    bench_all_libraries_multiplication
);
criterion_main!(benches);