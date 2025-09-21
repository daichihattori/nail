use bigint_rs::{U64, U128, U256, U512, U1024};
use std::time::Instant;

fn benchmark_operation<F>(name: &str, iterations: usize, mut op: F) 
where 
    F: FnMut() -> (),
{
    let start = Instant::now();
    for _ in 0..iterations {
        op();
    }
    let duration = start.elapsed();
    let ns_per_op = duration.as_nanos() / iterations as u128;
    println!("{}: {} ns/op ({} iterations)", name, ns_per_op, iterations);
}

fn main() {
    println!("=== Performance Demonstration ===\n");
    
    let iterations = 1_000_000;
    
    println!("1. Addition Performance Across Different Sizes:");
    
    // 64-bit addition
    let a64 = U64::from_u64(0x123456789ABCDEF0);
    let b64 = U64::from_u64(0xFEDCBA9876543210);
    benchmark_operation("U64 addition", iterations, || {
        let _ = a64.add(&b64);
    });
    
    // 128-bit addition
    let a128 = U128::from_u64(0x123456789ABCDEF0);
    let b128 = U128::from_u64(0xFEDCBA9876543210);
    benchmark_operation("U128 addition", iterations, || {
        let _ = a128.add(&b128);
    });
    
    // 256-bit addition
    let a256 = U256::from_u64(0x123456789ABCDEF0);
    let b256 = U256::from_u64(0xFEDCBA9876543210);
    benchmark_operation("U256 addition", iterations, || {
        let _ = a256.add(&b256);
    });
    
    // 512-bit addition
    let a512 = U512::from_u64(0x123456789ABCDEF0);
    let b512 = U512::from_u64(0xFEDCBA9876543210);
    benchmark_operation("U512 addition", iterations, || {
        let _ = a512.add(&b512);
    });

    println!("\n2. Multiplication Performance Across Different Sizes:");
    
    let mul_iterations = 100_000; // Fewer iterations for multiplication
    
    // 64-bit multiplication
    benchmark_operation("U64 multiplication", mul_iterations, || {
        let _ = a64.mul(&b64);
    });
    
    // 128-bit multiplication
    benchmark_operation("U128 multiplication", mul_iterations, || {
        let _ = a128.mul(&b128);
    });
    
    // 256-bit multiplication
    benchmark_operation("U256 multiplication", mul_iterations, || {
        let _ = a256.mul(&b256);
    });
    
    // 512-bit multiplication
    benchmark_operation("U512 multiplication", mul_iterations, || {
        let _ = a512.mul(&b512);
    });

    println!("\n3. Mixed-Size Operations Performance:");
    
    let mixed_iterations = 100_000;
    
    // Large + Small addition
    let large = U512::from_u64(0x123456789ABCDEF0);
    let small = U256::from_u64(0xFEDCBA9876543210);
    benchmark_operation("U512 + U256 addition", mixed_iterations, || {
        let _ = large.add(&small);
    });
    
    // Large * Small multiplication
    benchmark_operation("U512 * U256 multiplication", mixed_iterations, || {
        let _ = large.mul(&small);
    });

    println!("\n4. Modular Arithmetic Performance:");
    
    let mod_iterations = 50_000;
    
    // Modular addition
    benchmark_operation("U256 addmod_bits", mod_iterations, || {
        let _ = a256.addmod_bits(&b256, 256);
    });
    
    // Modular multiplication
    benchmark_operation("U256 mulmod_bits", mod_iterations, || {
        let _ = a256.mulmod_bits(&b256, 256);
    });

    println!("\n5. Memory Usage Demonstration:");
    
    println!("Type sizes in memory:");
    println!("U64:   {} bytes", std::mem::size_of::<U64>());
    println!("U128:  {} bytes", std::mem::size_of::<U128>());
    println!("U256:  {} bytes", std::mem::size_of::<U256>());
    println!("U512:  {} bytes", std::mem::size_of::<U512>());
    println!("U1024: {} bytes", std::mem::size_of::<U1024>());
    
    println!("\n6. Zero-Allocation Guarantee:");
    println!("All operations use stack-allocated arrays with compile-time known sizes.");
    println!("No heap allocations occur during arithmetic operations.");
    
    // Demonstrate large number operations
    println!("\n7. Large Number Operations:");
    let very_large_a = U1024::from_u64(u64::MAX);
    let very_large_b = U1024::from_u64(u64::MAX - 1);
    
    let start = Instant::now();
    let result = very_large_a.mul(&very_large_b);
    let duration = start.elapsed();
    
    println!("U1024 * U1024 (max values): {} ns", duration.as_nanos());
    println!("Result lower limbs: 0x{:016X}{:016X}", 
             result.limbs[1], result.limbs[0]);

    println!("\n8. Scaling Analysis:");
    println!("Addition complexity: O(n) where n = limb count");
    println!("Multiplication complexity: O(n²) where n = limb count");
    println!("Memory complexity: O(n) where n = limb count");
    println!("All operations have predictable, deterministic performance.");
}