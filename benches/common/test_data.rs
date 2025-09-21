use num_bigint::BigUint;
use rug::Integer;
use malachite::Natural;
use bigint_rs::Uint;

pub fn create_biguint(bits: usize) -> BigUint {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    BigUint::from_bytes_le(&data)
}

pub fn create_rug_integer(bits: usize) -> Integer {
    let bytes = bits / 8;
    let mut data = vec![0u8; bytes];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i * 7 + 123) as u8;
    }
    Integer::from_digits(&data, rug::integer::Order::Lsf)
}

pub fn create_malachite_natural(bits: usize) -> Natural {
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

pub fn create_test_u64() -> u64 {
    0x123456789ABCDEF0u64
}

pub fn create_test_u128() -> u128 {
    0x123456789ABCDEF0123456789ABCDEF0u128
}

pub fn create_bigint_rs_uint(bits: usize) -> Box<dyn std::any::Any> {
    match bits {
        64 => {
            let mut uint = Uint::<1>::zero();
            uint.limbs[0] = 0x123456789ABCDEF0u64;
            Box::new(uint)
        },
        128 => {
            let mut uint = Uint::<2>::zero();
            uint.limbs[0] = 0x123456789ABCDEF0u64;
            uint.limbs[1] = 0x123456789ABCDEF0u64;
            Box::new(uint)
        },
        256 => {
            let mut uint = Uint::<4>::zero();
            uint.limbs[0] = 0x123456789ABCDEF0u64;
            uint.limbs[1] = 0x123456789ABCDEF0u64;
            uint.limbs[2] = 0x123456789ABCDEF0u64;
            uint.limbs[3] = 0x123456789ABCDEF0u64;
            Box::new(uint)
        },
        512 => {
            let mut uint = Uint::<8>::zero();
            for i in 0..8 {
                uint.limbs[i] = 0x123456789ABCDEF0u64;
            }
            Box::new(uint)
        },
        1024 => {
            let mut uint = Uint::<16>::zero();
            for i in 0..16 {
                uint.limbs[i] = 0x123456789ABCDEF0u64;
            }
            Box::new(uint)
        },
        _ => panic!("Unsupported bit size: {}", bits)
    }
}

// Specific creation functions for type safety
pub fn create_fixed_array_uint_64() -> Uint<1> {
    Uint::<1>::from_u64(0x123456789ABCDEF0u64)
}

pub fn create_fixed_array_uint_128() -> Uint<2> {
    let mut uint = Uint::<2>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint
}

pub fn create_fixed_array_uint_256() -> Uint<4> {
    let mut uint = Uint::<4>::zero();
    uint.limbs[0] = 0x123456789ABCDEF0u64;
    uint.limbs[1] = 0x123456789ABCDEF0u64;
    uint.limbs[2] = 0x123456789ABCDEF0u64;
    uint.limbs[3] = 0x123456789ABCDEF0u64;
    uint
}

pub fn create_fixed_array_uint_512() -> Uint<8> {
    let mut uint = Uint::<8>::zero();
    for i in 0..8 {
        uint.limbs[i] = 0x123456789ABCDEF0u64;
    }
    uint
}

pub fn create_fixed_array_uint_1024() -> Uint<16> {
    let mut uint = Uint::<16>::zero();
    for i in 0..16 {
        uint.limbs[i] = 0x123456789ABCDEF0u64;
    }
    uint
}