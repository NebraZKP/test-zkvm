#![no_std] // std support is experimental

use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{BigInt, Fp, FpConfig, PrimeField, QuadExtField};

pub type FpRepr<const N: usize> = [u64; N];

pub trait HasRepr {
    type Repr;

    fn to_repr(&self) -> Self::Repr;

    fn from_repr(repr: &Self::Repr) -> Self;
}

impl<P: FpConfig<N>, const N: usize> HasRepr for Fp<P, N> {
    type Repr = FpRepr<N>;

    fn to_repr(&self) -> Self::Repr {
        self.into_bigint().0
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self::from_bigint(BigInt(*repr)).unwrap()
    }
}

// impl<P: QuadExtConfig> HasRepr for QuadExtField<P> {
//     type Repr = Fp2Repr<

// #[derive(Deserialize, Serialize)]

// impl<P: FpConfig<N>, const N: usize> From<&FpRepr<N>> for Fp<P, N> {
//     fn from(repr: &FpRepr<N>) -> Self {
//         todo!()
//     }
// }

// pub fn fp_from_repr(repr: &FpRepr<N>) -> Fp<P, N> {
//     Fp::<P, N>::from_bigint(BigInt(*repr)).unwrap()
// }

// pub fn fp_to_repr<P: FpConfig<N>, const N: usize>(fp: &Fp<P, N>) -> FpRepr<N> {
//     fp.into_bigint().0
// }

// #[derive(Deserialize, Serialize)]
pub type Fp2Repr<const N: usize> = [[u64; N]; 2];

// fn fp2_from_repr<P: FpConfig<N>, const N: usize>(repr: &Fp2Repr<N>) -> Fp<P, N> {
//     Fp::<P, N>::from_bigint(BigInt(*repr)).unwrap()
// }

// #[derive(Deserialize, Serialize)]
pub struct G1Repr {
    pub x: FpRepr<4>,
    pub y: FpRepr<4>,
}

// #[derive(Deserialize, Serialize)]
pub struct G2Repr {
    x: Fp2Repr<4>,
    y: Fp2Repr<4>,
}

// #[derive(Deserialize, Serialize)]
pub struct Inputs {}
