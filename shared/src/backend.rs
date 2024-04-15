use ark_ff::{BigInt, Field, Fp, Fp256, FpConfig, PrimeField, SqrtPrecomputation};
use core::{marker::PhantomData, str::FromStr};

// #[derive(MontConfig)]
// #[modulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
// #[generator = "3"]
// pub struct FqConfig;

// Fq
// u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208583)
// [4332616871279656263,
//  10917124144477883021,
//  13281191951274694749,
//  3486998266802970665]

type B = BigInt<4>;

// Per-field properties, created by a macro
pub trait Fp256PlainConfig: 'static + Sync + Send + Sized {
    const MODULUS: B;
    const GENERATOR: Fp256<Fp256PlainBackend<Self>>;
    const ZERO: Fp256<Fp256PlainBackend<Self>>;
    const ONE: Fp256<Fp256PlainBackend<Self>>;
    const TWO_ADICITY: u32;
    const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>>;

    #[inline(always)]
    fn __add_with_carry(a: &mut B, b: &B) -> bool {
        use ark_ff::biginteger::arithmetic::adc_for_add_with_carry as adc;
        let mut carry = 0;
        carry = adc(&mut a.0[0], b.0[0], carry);
        carry = adc(&mut a.0[1], b.0[1], carry);
        carry = adc(&mut a.0[2], b.0[2], carry);
        carry = adc(&mut a.0[3], b.0[3], carry);
        carry != 0
    }

    #[inline(always)]
    fn __sub_with_borrow(a: &mut B, b: &B) -> bool {
        use ark_ff::biginteger::arithmetic::sbb_for_sub_with_borrow as sbb;
        let mut borrow = 0;
        borrow = sbb(&mut a.0[0], b.0[0], borrow);
        borrow = sbb(&mut a.0[1], b.0[1], borrow);
        borrow = sbb(&mut a.0[2], b.0[2], borrow);
        borrow = sbb(&mut a.0[3], b.0[3], borrow);
        borrow != 0
    }

    #[inline(always)]
    fn __subtract_modulus(a: &mut Fp256<Fp256PlainBackend<Self>>) {
        if a.is_geq_modulus() {
            Self::__sub_with_borrow(&mut a.0, &Self::MODULUS);
        }
    }

    #[inline(always)]
    fn __subtract_modulus_with_carry(a: &mut Fp256<Fp256PlainBackend<Self>>, carry: bool) {
        if a.is_geq_modulus() || carry {
            Self::__sub_with_borrow(&mut a.0, &Self::MODULUS);
        }
    }
}

// Implementation based on a PlainFp256Config
pub struct Fp256PlainBackend<T: Fp256PlainConfig>(PhantomData<T>);

impl<T: Fp256PlainConfig> FpConfig<4> for Fp256PlainBackend<T> {
    // 21888242871839275222246405745257275088696311157297823662689037894645226208583
    const MODULUS: BigInt<4> = BigInt([
        4332616871279656263,
        10917124144477883021,
        13281191951274694749,
        3486998266802970665,
    ]);

    //
    const GENERATOR: Fp256<Self> = Fp(BigInt([3, 0, 0, 0]), PhantomData);

    //
    const ZERO: Fp256<Self> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

    //
    const ONE: Fp256<Self> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

    const TWO_ADICITY: u32 = 2;

    // u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208582)
    // = Fq(-1)
    const TWO_ADIC_ROOT_OF_UNITY: Fp256<Self> = Fp(
        BigInt([
            4332616871279656262,
            10917124144477883021,
            13281191951274694749,
            3486998266802970665,
        ]),
        PhantomData,
    );

    // const TWO_ADICITY: u32 = 42;
    // const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N> = value;
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp256<Self>>> = None;

    fn add_assign(a: &mut ark_ff::Fp<Self, 4>, b: &ark_ff::Fp<Self, 4>) {
        T::__add_with_carry(&mut a.0, &b.0);
        T::__subtract_modulus(a);
    }
    fn sub_assign(a: &mut ark_ff::Fp<Self, 4>, b: &ark_ff::Fp<Self, 4>) {
        // If `other` is larger than `self`, add the modulus to self first.
        if b.0 > a.0 {
            T::__add_with_carry(&mut a.0, &Self::MODULUS);
        }
        T::__sub_with_borrow(&mut a.0, &b.0);
    }
    fn double_in_place(_: &mut ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn neg_in_place(a: &mut ark_ff::Fp<Self, 4>) {
        if *a != Fp::<Self, 4>::ZERO {
            let mut tmp = Self::MODULUS;
            T::__sub_with_borrow(&mut tmp, &a.0);
            a.0 = tmp;
        }
    }
    fn mul_assign(_: &mut ark_ff::Fp<Self, 4>, _: &ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn sum_of_products<const N: usize>(_: &[Fp256<Self>; N], _: &[Fp256<Self>; N]) -> Fp256<Self> {
        todo!()
    }
    fn square_in_place(_: &mut ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn inverse(_: &ark_ff::Fp<Self, 4>) -> Option<ark_ff::Fp<Self, 4>> {
        todo!()
    }
    fn from_bigint(v: BigInt<4>) -> Option<Fp256<Self>> {
        let v = Fp::<Self, 4>(v, PhantomData);
        if v.is_geq_modulus() {
            None
        } else {
            Some(v)
        }
    }
    fn into_bigint(v: ark_ff::Fp<Self, 4>) -> BigInt<4> {
        v.0
    }
}

// Fq

pub struct FqPlainConfig;

impl Fp256PlainConfig for FqPlainConfig {
    const MODULUS: BigInt<4> = BigInt([
        4332616871279656263,
        10917124144477883021,
        13281191951274694749,
        3486998266802970665,
    ]);

    //
    const GENERATOR: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([3, 0, 0, 0]), PhantomData);

    //
    const ZERO: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

    //
    const ONE: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

    const TWO_ADICITY: u32 = 2;

    // u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208582)
    // = Fq(-1)
    const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>> = Fp(
        BigInt([
            4332616871279656262,
            10917124144477883021,
            13281191951274694749,
            3486998266802970665,
        ]),
        PhantomData,
    );

    // const TWO_ADICITY: u32 = 42;
    // const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N> = value;
    // const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp256<Fp256PlainBackend<Self>>>> = None;
}

pub type Fq = Fp256<Fp256PlainBackend<FqPlainConfig>>;

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254;

    #[test]
    fn test_fq() {
        let x = BigInt::new([12341234134, 431243124321, 987698769876, 678968976789]);
        let y = BigInt::new([12341234134, 431243124321, 987698769876, 678968976789]);

        let x_fq = Fq::from_bigint(x).unwrap();
        let y_fq = Fq::from_bigint(y).unwrap();

        let x_ark_fq = ark_bn254::Fq::from_bigint(x).unwrap();
        let y_ark_fq = ark_bn254::Fq::from_bigint(y).unwrap();

        let expect_x_plus_y = x_ark_fq + y_ark_fq;
        let expect_x_minus_y = x_ark_fq - y_ark_fq;

        {
            let x_plus_y = x_fq + y_fq;
            assert_eq!(expect_x_plus_y.into_bigint(), x_plus_y.into_bigint());
        }

        {
            let mut x_plus_y = x_fq;
            x_plus_y += y_fq;
            assert_eq!(expect_x_plus_y.into_bigint(), x_plus_y.into_bigint(),);
        }

        {
            let x_minus_y = x_fq - y_fq;
            assert_eq!(expect_x_minus_y.into_bigint(), x_minus_y.into_bigint(),);
        }

        {
            let mut x_minus_y = x_fq;
            x_minus_y -= y_fq;
            assert_eq!(expect_x_minus_y.into_bigint(), x_minus_y.into_bigint(),);
        }

        {
            let mut y_minus_x = y_fq;
            y_minus_x -= x_fq;
            assert_eq!((-expect_x_minus_y).into_bigint(), y_minus_x.into_bigint(),);
        }

        assert_eq!(
            (-ark_bn254::Fq::from(7) + ark_bn254::Fq::from(1)).into_bigint(),
            (-Fq::from(7) + Fq::from(1)).into_bigint()
        );
    }
}
