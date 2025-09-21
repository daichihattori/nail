use bigint_rs::Uint;

fn main() {
    let a = Uint::<4>::from_u64(0xFFFF_FFFF_FFFFu64); // ~48bit
    let b = Uint::<4>::from_u64(0x1000);

    // 48bit mod なので 2^48 で剰余
    let r = a.mulmod_bits(&b, 48);

    println!("{:x?}", r.limbs);
}