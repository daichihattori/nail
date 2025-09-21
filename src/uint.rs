//! Fixed-size unsigned integer implementation with configurable limb count.
//! 
//! This module provides a generic `Uint<LIMBS>` type for arbitrary precision
//! unsigned integer arithmetic with compile-time determined size.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uint<const LIMBS: usize> {
    pub limbs: [u64; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Creates a new `Uint` with all limbs set to zero.
    pub fn zero() -> Self {
        Self { limbs: [0; LIMBS] }
    }

    /// Creates a new `Uint` from a single `u64` value.
    /// 
    /// The value is placed in the least significant limb, with all other limbs set to zero.
    pub fn from_u64(x: u64) -> Self {
        let mut limbs = [0; LIMBS];
        limbs[0] = x;
        Self { limbs }
    }

    /// Addition where left operand has >= limbs than right operand
    /// Returns result with same limb count as left operand (self)
    pub fn add<const OTHER_LIMBS: usize>(&self, other: &Uint<OTHER_LIMBS>) -> Self
    where
        [(); LIMBS]:,
        [(); OTHER_LIMBS]:,
    {
        // Ensure left operand has at least as many limbs as right operand
        assert!(LIMBS >= OTHER_LIMBS, "Left operand must have >= limbs than right operand");
        
        let mut out = [0u64; LIMBS];
        let mut carry = false;

        for i in 0..LIMBS {
            let other_limb = if i < OTHER_LIMBS { other.limbs[i] } else { 0 };
            
            let (s1, c1) = self.limbs[i].overflowing_add(other_limb);
            let (s2, c2) = s1.overflowing_add(carry as u64);

            out[i] = s2;
            carry = c1 || c2;
        }

        Self { limbs: out }
    }

    /// Multiplication where left operand has >= limbs than right operand
    /// Returns result with same limb count as left operand (self)
    /// Result may overflow if the true product exceeds LIMBS * 64 bits
    pub fn mul<const OTHER_LIMBS: usize>(&self, other: &Uint<OTHER_LIMBS>) -> Self
    where
        [(); LIMBS]:,
        [(); OTHER_LIMBS]:,
    {
        // Ensure left operand has at least as many limbs as right operand
        assert!(LIMBS >= OTHER_LIMBS, "Left operand must have >= limbs than right operand");
        let mut out = [0u64; LIMBS];

        for i in 0..LIMBS {
            for j in 0..OTHER_LIMBS {
                if i + j >= LIMBS { break; }
                
                // Calculate full 128-bit product manually
                let a = self.limbs[i] as u128;
                let b = other.limbs[j] as u128;
                let product = a * b;
                let low = product as u64;
                let high = (product >> 64) as u64;
                
                // Add low part to current position
                let (sum, overflow) = out[i + j].overflowing_add(low);
                out[i + j] = sum;
                
                // Propagate overflow to next position
                if overflow && i + j + 1 < LIMBS {
                    let (sum2, _) = out[i + j + 1].overflowing_add(1);
                    out[i + j + 1] = sum2;
                }
                
                // Add high part to next position if it fits
                if i + j + 1 < LIMBS {
                    let (sum3, _) = out[i + j + 1].overflowing_add(high);
                    out[i + j + 1] = sum3;
                }
            }
        }

        Self { limbs: out }
    }

     /// (a + b) mod 2^bitsize
    pub fn addmod_bits(&self, other: &Self, bitsize: usize) -> Self {
        assert!(bitsize <= LIMBS * 64);

        let mut out = [0u64; LIMBS];
        let mut carry = false;

        for i in 0..LIMBS {
            let (s1, c1) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (s2, c2) = s1.overflowing_add(carry as u64);

            out[i] = s2;
            carry = c1 || c2;
        }

        // Apply bit mask to the last limb
        let excess_bits = LIMBS * 64 - bitsize;
        if excess_bits > 0 && excess_bits < 64 {
            let mask = u64::MAX >> excess_bits;
            let last = (bitsize - 1) / 64;
            out[last] &= mask;
        }
        
        // Zero out higher limbs
        let last_limb = (bitsize + 63) / 64;
        for i in last_limb..LIMBS {
            out[i] = 0;
        }

        Self { limbs: out }
    }
     pub fn mulmod_bits(&self, other: &Self, bitsize: usize) -> Self {
        assert!(bitsize <= LIMBS * 64);

        let nlimbs = (bitsize + 63) / 64;
        let mut out = [0u64; LIMBS];

        for i in 0..nlimbs {
            let mut carry: u128 = 0;
            for j in 0..=i {
                let a = self.limbs[j] as u128;
                let b = other.limbs[i - j] as u128;
                let sum = out[i] as u128 + a * b + carry;
                out[i] = sum as u64;
                carry = sum >> 64;
            }
            // Even if carry remains, we don't need to propagate to out[i+1] because of mod 2^(nlimbs*64)
        }

        // Mask the last limb according to bitsize
        let excess_bits = nlimbs * 64 - bitsize;
        if excess_bits > 0 && excess_bits < 64 {
            let mask: u64 = u64::MAX >> excess_bits;
            out[nlimbs - 1] &= mask;
        }
        // Zero out higher limbs
        for i in nlimbs..LIMBS {
            out[i] = 0;
        }

        Self { limbs: out }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let a = Uint::<4>::from_u64(0x1234_5678_9ABC_DEF0);
        let b = Uint::<4>::from_u64(0x0FED_CBA9_8765_4321);
        let result = a.add(&b);
        
        // 0x1234_5678_9ABC_DEF0 + 0x0FED_CBA9_8765_4321 = 0x2222_2222_2222_2211
        assert_eq!(result.limbs[0], 0x2222_2222_2222_2211);
        assert_eq!(result.limbs[1], 0);
    }

    #[test]
    fn test_addition_with_carry() {
        let a = Uint::<4>::from_u64(u64::MAX);
        let b = Uint::<4>::from_u64(1);
        let result = a.add(&b);
        
        // u64::MAX + 1 should overflow to next limb
        assert_eq!(result.limbs[0], 0);
        assert_eq!(result.limbs[1], 1);
    }

    #[test]
    fn test_basic_multiplication() {
        let a = Uint::<4>::from_u64(0x1000);
        let b = Uint::<4>::from_u64(0x2000);
        let result = a.mul(&b);
        
        // 0x1000 * 0x2000 = 0x2000000
        assert_eq!(result.limbs[0], 0x2000000);
        assert_eq!(result.limbs[1], 0);
    }

    #[test]
    fn test_multiplication_large() {
        let a = Uint::<4>::from_u64(0xFFFF_FFFF);
        let b = Uint::<4>::from_u64(0xFFFF_FFFF);
        let result = a.mul(&b);
        
        // 0xFFFF_FFFF * 0xFFFF_FFFF = 0xFFFFFFFE00000001
        assert_eq!(result.limbs[0], 0xFFFFFFFE00000001);
        assert_eq!(result.limbs[1], 0);
    }

    #[test]
    fn test_zero() {
        let zero = Uint::<4>::zero();
        let a = Uint::<4>::from_u64(42);
        
        let add_result = a.add(&zero);
        let mul_result = a.mul(&zero);
        
        assert_eq!(add_result.limbs[0], 42);
        assert_eq!(mul_result.limbs[0], 0);
    }

    #[test]
    fn test_different_limb_sizes() {
        // Test with 1 limb (64-bit)
        let a1 = Uint::<1>::from_u64(100);
        let b1 = Uint::<1>::from_u64(200);
        let result1 = a1.add(&b1);
        assert_eq!(result1.limbs[0], 300);

        // Test with 8 limbs (512-bit)
        let a8 = Uint::<8>::from_u64(100);
        let b8 = Uint::<8>::from_u64(200);
        let result8 = a8.add(&b8);
        assert_eq!(result8.limbs[0], 300);
        for i in 1..8 {
            assert_eq!(result8.limbs[i], 0);
        }
    }

    #[test]
    fn test_mixed_limb_sizes() {
        // Test larger + smaller (should work)
        let a4 = Uint::<4>::from_u64(1000);
        let b2 = Uint::<2>::from_u64(2000);
        let result = a4.add(&b2);
        assert_eq!(result.limbs[0], 3000);
        assert_eq!(result.limbs[1], 0);

        // Test multiplication with different sizes
        let mul_result = a4.mul(&b2);
        assert_eq!(mul_result.limbs[0], 2_000_000);
        assert_eq!(mul_result.limbs[1], 0);
    }

    #[test]
    #[should_panic(expected = "Left operand must have >= limbs than right operand")]
    fn test_invalid_limb_size_addition() {
        let a2 = Uint::<2>::from_u64(100);
        let b4 = Uint::<4>::from_u64(200);
        let _ = a2.add(&b4); // Should panic: 2 < 4
    }

    #[test]
    #[should_panic(expected = "Left operand must have >= limbs than right operand")]
    fn test_invalid_limb_size_multiplication() {
        let a2 = Uint::<2>::from_u64(100);
        let b4 = Uint::<4>::from_u64(200);
        let _ = a2.mul(&b4); // Should panic: 2 < 4
    }

    #[test]
    fn test_max_multiplication_preserves_high_bits() {
        // Test u64::MAX * u64::MAX to ensure high bits are preserved
        let max_val = u64::MAX;
        let a = Uint::<2>::from_u64(max_val);
        let b = Uint::<2>::from_u64(max_val);
        let result = a.mul(&b);
        
        // u64::MAX * u64::MAX = 0xFFFFFFFFFFFFFFFE0000000000000001
        // Low limb should be 0x0000000000000001
        // High limb should be 0xFFFFFFFFFFFFFFFE
        assert_eq!(result.limbs[0], 0x0000000000000001);
        assert_eq!(result.limbs[1], 0xFFFFFFFFFFFFFFFE);
    }

    #[test]
    fn test_large_multiplication_with_carry() {
        // Test multiplication that generates significant carries
        let a = Uint::<4>::from_u64(0xFFFFFFFFFFFFFFFF);
        let b = Uint::<4>::from_u64(0xFFFFFFFFFFFFFFFF);
        let result = a.mul(&b);
        
        // Should produce: limbs[0] = 1, limbs[1] = 0xFFFFFFFFFFFFFFFE
        assert_eq!(result.limbs[0], 1);
        assert_eq!(result.limbs[1], 0xFFFFFFFFFFFFFFFE);
        assert_eq!(result.limbs[2], 0);
        assert_eq!(result.limbs[3], 0);
    }
}