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