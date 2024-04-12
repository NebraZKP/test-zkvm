// #[cfg(all(target_os = "zkvm", target_arch = "riscv32"))]
// use crypto_bigint::risc0;
use crypto_bigint::{
    impl_modulus,
    modular::constant_mod::{self, Residue, ResidueParams},
    U256,
};

pub const R: U256 =
    U256::from_be_hex("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001");
pub const Q: U256 =
    U256::from_be_hex("30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47");

impl_modulus!(
    MyFqParams,
    U256,
    "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47"
);

pub type MyFq = Residue<MyFqParams, { MyFqParams::LIMBS }>;

pub fn split_u64(v: u64) -> (u32, u32) {
    (v as u32, (v >> 32) as u32)
}

pub fn u256_from_u64s(data: &[u64; 4]) -> U256 {
    #[cfg(target_pointer_width = "32")]
    {
        let (w00, w01) = split_u64(data[0]);
        let (w10, w11) = split_u64(data[1]);
        let (w20, w21) = split_u64(data[2]);
        let (w30, w31) = split_u64(data[3]);
        U256::from_words([w00, w01, w10, w11, w20, w21, w30, w31])
    }

    #[cfg(target_pointer_width = "64")]
    {
        U256::from_words(data.clone())
    }
}

pub fn add_mod_q(a: &U256, b: &U256) -> U256 {
    a.add_mod(&b, &Q)
    // risc0::mod_add(
}

#[cfg(all(target_os = "zkvm", target_arch = "riscv32"))]
pub fn mul_mod_q(a: &U256, b: &U256) -> U256 {
    // let p = a.mul_wide(&b);
    // montgomery_reduction(&p, &Q, , modulus, mod_neg_inv)
    // let prod = a.mul(b);
    // a.mul_mod_special(rhs, c)

    // risc0::modmul_uint_256(&a, &b, &Q)
    todo!()
}

pub fn inv_mod_q(a: &U256) -> U256 {
    let (r, _) = a.inv_odd_mod_bounded(&Q, 254, 254);
    r
}
