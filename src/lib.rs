#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uint<const LIMBS: usize> {
    pub limbs: [u64; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
    pub fn zero() -> Self {
        Self { limbs: [0; LIMBS] }
    }

    pub fn from_u64(x: u64) -> Self {
        let mut limbs = [0; LIMBS];
        limbs[0] = x;
        Self { limbs }
    }

    /// Simple addition without modular arithmetic
    pub fn add(&self, other: &Self) -> Self {
        let mut out = [0u64; LIMBS];
        let mut carry = false;

        for i in 0..LIMBS {
            let (s1, c1) = self.limbs[i].overflowing_add(other.limbs[i]);
            let (s2, c2) = s1.overflowing_add(carry as u64);

            out[i] = s2;
            carry = c1 || c2;
        }

        Self { limbs: out }
    }

    /// Simple multiplication without modular arithmetic
    pub fn mul(&self, other: &Self) -> Self {
        let mut out = [0u64; LIMBS];

        for i in 0..LIMBS {
            let mut carry: u128 = 0;
            for j in 0..LIMBS {
                if i + j >= LIMBS { break; }
                
                let a = self.limbs[i] as u128;
                let b = other.limbs[j] as u128;
                let sum = out[i + j] as u128 + a * b + carry;
                out[i + j] = sum as u64;
                carry = sum >> 64;
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

        // 最後の limb にビットマスクをかける
        let excess_bits = LIMBS * 64 - bitsize;
        if excess_bits > 0 {
            let mask = u64::MAX >> excess_bits;
            let last = (bitsize - 1) / 64;
            out[last] &= mask;

            // さらに上位 limb をゼロに
            for i in last + 1..LIMBS {
                out[i] = 0;
            }
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
            // もし carry が残っても、mod 2^(nlimbs*64) なので out[i+1] に繰り上げしなくていい
        }

        // 最後の limb を bitsize に合わせてマスク
        let excess_bits = nlimbs * 64 - bitsize;
        if excess_bits > 0 {
            let mask: u64 = u64::MAX >> excess_bits;
            out[nlimbs - 1] &= mask;
        }
        // 上位 limb はゼロに
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
}