use core::marker::PhantomData;

use ark_bn254::Fq;
use ark_ff::{BigInt, Fp, Fp2, Fp256, Fp2Config, FpConfig, PrimeField, SqrtPrecomputation};

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

pub struct R0FqConfig;

impl FpConfig<4> for R0FqConfig {
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

    fn add_assign(_: &mut ark_ff::Fp<Self, 4>, _: &ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn sub_assign(_: &mut ark_ff::Fp<Self, 4>, _: &ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn double_in_place(_: &mut ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn neg_in_place(_: &mut ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn mul_assign(_: &mut ark_ff::Fp<Self, 4>, _: &ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn sum_of_products<const T: usize>(_: &[Fp256<Self>; T], _: &[Fp256<Self>; T]) -> Fp256<Self> {
        todo!()
    }
    fn square_in_place(_: &mut ark_ff::Fp<Self, 4>) {
        todo!()
    }
    fn inverse(_: &ark_ff::Fp<Self, 4>) -> Option<ark_ff::Fp<Self, 4>> {
        todo!()
    }
    fn from_bigint(_: BigInt<4>) -> Option<ark_ff::Fp<Self, 4>> {
        todo!()
    }
    fn into_bigint(_: ark_ff::Fp<Self, 4>) -> BigInt<4> {
        todo!()
    }
}

// pub struct R0FieldBackend<R0FqConfig>(PhantomData<R0FqConfig>;

pub type R0Fq = Fp256<R0FqConfig>;
