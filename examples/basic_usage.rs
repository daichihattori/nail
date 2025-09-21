use nail::{U64, U128, U256, U512, U1024};

fn main() {
    println!("=== Nail Examples ===\n");

    // Basic creation and arithmetic
    println!("1. Basic Operations:");
    let a = U256::from_u64(0x1234_5678_9ABC_DEF0);
    let b = U256::from_u64(0x0FED_CBA9_8765_4321);
    
    println!("a = 0x{:016X}", a.limbs[0]);
    println!("b = 0x{:016X}", b.limbs[0]);
    
    let sum = a.add(&b);
    println!("a + b = 0x{:016X}", sum.limbs[0]);
    
    let product = a.mul(&b);
    println!("a * b = 0x{:016X}{:016X}", product.limbs[1], product.limbs[0]);
    
    // Different bit sizes
    println!("\n2. Different Bit Sizes:");
    let small = U64::from_u64(1000);
    let medium = U128::from_u64(2000);
    let large = U512::from_u64(3000);
    let xlarge = U1024::from_u64(4000);
    
    println!("U64:   {} bits, value = {}", 64, small.limbs[0]);
    println!("U128:  {} bits, value = {}", 128, medium.limbs[0]);
    println!("U512:  {} bits, value = {}", 512, large.limbs[0]);
    println!("U1024: {} bits, value = {}", 1024, xlarge.limbs[0]);

    // Mixed-size operations (larger on left)
    println!("\n3. Mixed-Size Operations:");
    let big = U256::from_u64(1_000_000);
    let small = U128::from_u64(999_999);
    
    let mixed_sum = big.add(&small);
    println!("U256 + U128 = {} (result is U256)", mixed_sum.limbs[0]);
    
    let mixed_product = big.mul(&small);
    println!("U256 * U128 = {} (result is U256)", mixed_product.limbs[0]);

    // Large number multiplication
    println!("\n4. Large Number Multiplication:");
    let max_val = u64::MAX;
    let x = U128::from_u64(max_val);
    let y = U128::from_u64(max_val);
    
    println!("max_u64 = 0x{:016X}", max_val);
    let large_product = x.mul(&y);
    println!("max_u64 * max_u64 = 0x{:016X}{:016X}", 
             large_product.limbs[1], large_product.limbs[0]);

    // Modular arithmetic
    println!("\n5. Modular Arithmetic:");
    let a_mod = U256::from_u64(0xFFFFFFFFFFFFFFFF);
    let b_mod = U256::from_u64(0x1000000000000000);
    
    // Addition mod 2^128 (128 bits)
    let add_mod_result = a_mod.addmod_bits(&b_mod, 128);
    println!("Addition mod 2^128:");
    println!("Result: 0x{:016X}{:016X}", add_mod_result.limbs[1], add_mod_result.limbs[0]);
    
    // Multiplication mod 2^128 (128 bits)
    let mul_mod_result = a_mod.mulmod_bits(&b_mod, 128);
    println!("Multiplication mod 2^128:");
    println!("Result: 0x{:016X}{:016X}", mul_mod_result.limbs[1], mul_mod_result.limbs[0]);

    // Zero values
    println!("\n6. Zero Operations:");
    let zero = U256::zero();
    let non_zero = U256::from_u64(42);
    
    let zero_sum = non_zero.add(&zero);
    let zero_product = non_zero.mul(&zero);
    
    println!("42 + 0 = {}", zero_sum.limbs[0]);
    println!("42 * 0 = {}", zero_product.limbs[0]);
}