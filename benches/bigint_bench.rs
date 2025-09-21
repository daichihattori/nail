use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale};
use num_bigint::BigUint;
use rug::Integer;
use malachite::Natural;
use bigint_rs::Uint;

fn create_biguint(bits: usize) -> BigUint {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    BigUint::from_bytes_le(&data)
}

fn create_rug_integer(bits: usize) -> Integer {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    Integer::from_digits(&data, rug::integer::Order::Lsf)
}

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

fn create_test_u64() -> u64 {
    0x123456789ABCDEF0u64
}

fn create_test_u128() -> u128 {
    0x123456789ABCDEF0123456789ABCDEF0u128
}

fn create_fixed_array_uint_64() -> Uint<1> {
    Uint::<1>::from_u64(0x123456789ABCDEF0u64)
}

fn create_fixed_array_uint_128() -> Uint<2> {
    let mut uint = Uint::<2>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint
}

fn create_fixed_array_uint_256() -> Uint<4> {
    let mut uint = Uint::<4>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint.limbs[2] = 0x123456789ABCDEF0u64;
    uint.limbs[3] = 0x123456789ABCDEF0u64;
    uint
}

fn bench_primitive_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primitive Addition");
    
    let u64_a = create_test_u64();
    let u64_b = create_test_u64();
    let u128_a = create_test_u128();
    let u128_b = create_test_u128();
    
    group.bench_function("u64", |b| {
        b.iter(|| {
            let result = black_box(u64_a).wrapping_add(black_box(u64_b));
            black_box(result)
        })
    });
    
    group.bench_function("u128", |b| {
        b.iter(|| {
            let result = black_box(u128_a).wrapping_add(black_box(u128_b));
            black_box(result)
        })
    });
    
    group.finish();
}

fn bench_addition_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition Comparison");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        let malachite_a = create_malachite_natural(*bits);
        let malachite_b = create_malachite_natural(*bits);
        
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&bigint_a) + black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&rug_a) + black_box(&rug_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("malachite", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&malachite_a) + black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
        
        // Add primitive types for comparison at 64 and 128 bit
        if *bits == 64 {
            let u64_a = create_test_u64();
            let u64_b = create_test_u64();
            group.bench_with_input(
                BenchmarkId::new("primitive-u64", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(u64_a).wrapping_add(black_box(u64_b));
                        black_box(result)
                    })
                },
            );
            
            let fixed_a = create_fixed_array_uint_64();
            let fixed_b = create_fixed_array_uint_64();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).add(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
        if *bits == 128 {
            let u128_a = create_test_u128();
            let u128_b = create_test_u128();
            group.bench_with_input(
                BenchmarkId::new("primitive-u128", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(u128_a).wrapping_add(black_box(u128_b));
                        black_box(result)
                    })
                },
            );
            
            let fixed_a = create_fixed_array_uint_128();
            let fixed_b = create_fixed_array_uint_128();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).add(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
        if *bits == 256 {
            let fixed_a = create_fixed_array_uint_256();
            let fixed_b = create_fixed_array_uint_256();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).add(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
    }
    group.finish();
}

fn bench_primitive_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("Primitive Multiplication");
    
    let u64_a = create_test_u64();
    let u64_b = create_test_u64();
    let u128_a = create_test_u128();
    let u128_b = create_test_u128();
    
    group.bench_function("u64", |b| {
        b.iter(|| {
            let result = black_box(u64_a).wrapping_mul(black_box(u64_b));
            black_box(result)
        })
    });
    
    group.bench_function("u128", |b| {
        b.iter(|| {
            let result = black_box(u128_a).wrapping_mul(black_box(u128_b));
            black_box(result)
        })
    });
    
    group.finish();
}

fn bench_multiplication_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Multiplication Comparison");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        let malachite_a = create_malachite_natural(*bits);
        let malachite_b = create_malachite_natural(*bits);
        
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("num-bigint", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&bigint_a) * black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&rug_a) * black_box(&rug_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("malachite", bits),
            bits,
            |b, _| {
                b.iter(|| {
                    let result = black_box(&malachite_a) * black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
        
        // Add primitive types and fixed array for comparison
        if *bits == 64 {
            let u64_a = create_test_u64();
            let u64_b = create_test_u64();
            group.bench_with_input(
                BenchmarkId::new("primitive-u64", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(u64_a).wrapping_mul(black_box(u64_b));
                        black_box(result)
                    })
                },
            );
            
            let fixed_a = create_fixed_array_uint_64();
            let fixed_b = create_fixed_array_uint_64();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
        if *bits == 128 {
            let u128_a = create_test_u128();
            let u128_b = create_test_u128();
            group.bench_with_input(
                BenchmarkId::new("primitive-u128", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(u128_a).wrapping_mul(black_box(u128_b));
                        black_box(result)
                    })
                },
            );
            
            let fixed_a = create_fixed_array_uint_128();
            let fixed_b = create_fixed_array_uint_128();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
        if *bits == 256 {
            let fixed_a = create_fixed_array_uint_256();
            let fixed_b = create_fixed_array_uint_256();
            group.bench_with_input(
                BenchmarkId::new("fixed-array", bits),
                bits,
                |b, _| {
                    b.iter(|| {
                        let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                        black_box(result)
                    })
                },
            );
        }
    }
    group.finish();
}

fn bench_bigint_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigInt Scaling");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    
    for bits in [64, 128, 256].iter() {
        let bigint_a = create_biguint(*bits);
        let bigint_b = create_biguint(*bits);
        let rug_a = create_rug_integer(*bits);
        let rug_b = create_rug_integer(*bits);
        let malachite_a = create_malachite_natural(*bits);
        let malachite_b = create_malachite_natural(*bits);
        
        group.throughput(Throughput::Bytes(*bits as u64 / 8));
        
        group.bench_with_input(
            BenchmarkId::new("num-bigint-add", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) + black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("num-bigint-mul", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&bigint_a) * black_box(&bigint_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp-add", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) + black_box(&rug_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("rug-gmp-mul", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&rug_a) * black_box(&rug_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("malachite-add", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&malachite_a) + black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("malachite-mul", bits),
            bits,
            |bench, _| {
                bench.iter(|| {
                    let result = black_box(&malachite_a) * black_box(&malachite_b);
                    black_box(result)
                })
            },
        );
        
        // Add fixed array benchmarks
        group.bench_with_input(
            BenchmarkId::new("fixed-array-add", bits),
            bits,
            |bench, bits| {
                match *bits {
                    64 => {
                        let fixed_a = create_fixed_array_uint_64();
                        let fixed_b = create_fixed_array_uint_64();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).add(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    128 => {
                        let fixed_a = create_fixed_array_uint_128();
                        let fixed_b = create_fixed_array_uint_128();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).add(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    256 => {
                        let fixed_a = create_fixed_array_uint_256();
                        let fixed_b = create_fixed_array_uint_256();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).add(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    _ => unreachable!()
                }
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("fixed-array-mul", bits),
            bits,
            |bench, bits| {
                match *bits {
                    64 => {
                        let fixed_a = create_fixed_array_uint_64();
                        let fixed_b = create_fixed_array_uint_64();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    128 => {
                        let fixed_a = create_fixed_array_uint_128();
                        let fixed_b = create_fixed_array_uint_128();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    256 => {
                        let fixed_a = create_fixed_array_uint_256();
                        let fixed_b = create_fixed_array_uint_256();
                        bench.iter(|| {
                            let result = black_box(&fixed_a).mul(black_box(&fixed_b));
                            black_box(result)
                        })
                    },
                    _ => unreachable!()
                }
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_primitive_addition,
    bench_primitive_multiplication,
    bench_addition_comparison,
    bench_multiplication_comparison,
    bench_bigint_scaling
);
criterion_main!(benches);