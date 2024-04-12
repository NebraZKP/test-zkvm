#![no_std] // std support is experimental

use ark_bn254::{Fq, G1Affine, G1Projective, G2Affine};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::{BigInt, Fp, Fp2, Fp2Config, FpConfig, PrimeField};

pub mod field;

/// An object which has a representation in terms of primitive serializable
/// objects.
pub trait HasRepr {
    type Repr;

    fn to_repr(&self) -> Self::Repr;

    fn from_repr(repr: &Self::Repr) -> Self;
}

impl<P: FpConfig<N>, const N: usize> HasRepr for Fp<P, N> {
    type Repr = [u64; N];

    fn to_repr(&self) -> Self::Repr {
        self.into_bigint().0
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self::from_bigint(BigInt(*repr)).unwrap()
    }
}

impl<P: Fp2Config> HasRepr for Fp2<P>
where
    P::Fp: HasRepr,
{
    type Repr = [<P::Fp as HasRepr>::Repr; 2];

    fn to_repr(&self) -> Self::Repr {
        [self.c0.to_repr(), self.c1.to_repr()]
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            c0: P::Fp::from_repr(&repr[0]),
            c1: P::Fp::from_repr(&repr[1]),
        }
    }
}

impl<P: SWCurveConfig> HasRepr for Affine<P>
where
    P::BaseField: HasRepr,
{
    type Repr = [<P::BaseField as HasRepr>::Repr; 2];

    fn to_repr(&self) -> Self::Repr {
        assert!(!self.infinity);
        [self.x.to_repr(), self.y.to_repr()]
    }

    fn from_repr(repr: &Self::Repr) -> Self {
        Self {
            x: P::BaseField::from_repr(&repr[0]),
            y: P::BaseField::from_repr(&repr[1]),
            infinity: false,
        }
    }
}

pub fn fp_from_u32s<P: FpConfig<4>>(values: &[u32; 8]) -> Fp<P, 4>
where
    Fp<P, 4>: HasRepr<Repr = [u64; 4]>,
{
    let u64s = [
        (values[0] as u64) + ((values[1] as u64) << 32),
        (values[2] as u64) + ((values[3] as u64) << 32),
        (values[4] as u64) + ((values[5] as u64) << 32),
        (values[6] as u64) + ((values[7] as u64) << 32),
    ];
    <Fp<P, 4> as HasRepr>::from_repr(&u64s)
}

pub type Inputs = (
    /* a1 */ <G1Affine as HasRepr>::Repr,
    /* a2 */ <G2Affine as HasRepr>::Repr,
    /* b1 */ <G1Affine as HasRepr>::Repr,
    /* b2 */ <G2Affine as HasRepr>::Repr,
    /* witness (b1.x - a1.x)^-1 */ <Fq as HasRepr>::Repr,
);

pub fn g1_add_refs_proj(a: &G1Projective, b: &G1Projective) -> G1Projective {
    a + b
}

pub fn g1_add_vals_proj(a: G1Projective, b: G1Projective) -> G1Projective {
    a + b
}

pub fn g1_add_refs_affine(a: &G1Affine, b: &G1Affine) -> G1Affine {
    (*a + b).into()
}

pub fn g1_add_vals_affine(a: G1Affine, b: G1Affine) -> G1Affine {
    (a + b).into()
}
