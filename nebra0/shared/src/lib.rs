#![no_std] // std support is experimental

use ark_bn254::{G1Affine, G2Affine};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::{BigInt, Fp, Fp2, Fp2Config, FpConfig, PrimeField};

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

pub type Inputs = (
    /* a1 */ <G1Affine as HasRepr>::Repr,
    /* a2 */ <G2Affine as HasRepr>::Repr,
    /* b1 */ <G1Affine as HasRepr>::Repr,
    /* b2 */ <G2Affine as HasRepr>::Repr,
);
