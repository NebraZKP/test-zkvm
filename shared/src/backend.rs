// use ark_ff::{
//     BigInt, BigInteger, Field, Fp, Fp256, FpConfig, MontBackend, PrimeField, SqrtPrecomputation,
//     Zero,
// };
// use core::marker::PhantomData;

// #[cfg(target_vendor = "risc0")]
// extern "C" {
//     fn sys_bigint(
//         result: *mut [u32; 8],
//         op: u32,
//         x: *const [u32; 8],
//         y: *const [u32; 8],
//         modulus: *const [u32; 8],
//     );
// }

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

// type B = BigInt<4>;

// // Per-field properties, created by a macro
// pub trait Fp256PlainConfig: 'static + Sync + Send + Sized {
//     const MODULUS: B;
//     const GENERATOR: Fp256<Fp256PlainBackend<Self>>;
//     const ZERO: Fp256<Fp256PlainBackend<Self>>;
//     const ONE: Fp256<Fp256PlainBackend<Self>>;
//     const TWO_ADICITY: u32;
//     const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>>;
//     type FullImplConfig: FpConfig<4>;

//     #[inline(always)]
//     fn __add_with_carry(a: &mut B, b: &B) -> bool {
//         use ark_ff::biginteger::arithmetic::adc_for_add_with_carry as adc;
//         let mut carry = 0;
//         carry = adc(&mut a.0[0], b.0[0], carry);
//         carry = adc(&mut a.0[1], b.0[1], carry);
//         carry = adc(&mut a.0[2], b.0[2], carry);
//         carry = adc(&mut a.0[3], b.0[3], carry);
//         carry != 0
//     }

//     #[inline(always)]
//     fn __sub_with_borrow(a: &mut B, b: &B) -> bool {
//         use ark_ff::biginteger::arithmetic::sbb_for_sub_with_borrow as sbb;
//         let mut borrow = 0;
//         borrow = sbb(&mut a.0[0], b.0[0], borrow);
//         borrow = sbb(&mut a.0[1], b.0[1], borrow);
//         borrow = sbb(&mut a.0[2], b.0[2], borrow);
//         borrow = sbb(&mut a.0[3], b.0[3], borrow);
//         borrow != 0
//     }

//     #[inline(always)]
//     fn __subtract_modulus(a: &mut Fp256<Fp256PlainBackend<Self>>) {
//         if a.is_geq_modulus() {
//             Self::__sub_with_borrow(&mut a.0, &Self::MODULUS);
//         }
//     }

//     #[inline(always)]
//     fn __subtract_modulus_with_carry(a: &mut Fp256<Fp256PlainBackend<Self>>, carry: bool) {
//         if a.is_geq_modulus() || carry {
//             Self::__sub_with_borrow(&mut a.0, &Self::MODULUS);
//         }
//     }
// }

// // Implementation based on a PlainFp256Config
// pub struct Fp256PlainBackend<T: Fp256PlainConfig>(PhantomData<T>);

// impl<T: Fp256PlainConfig> FpConfig<4> for Fp256PlainBackend<T> {
//     // 21888242871839275222246405745257275088696311157297823662689037894645226208583
//     const MODULUS: BigInt<4> = BigInt([
//         4332616871279656263,
//         10917124144477883021,
//         13281191951274694749,
//         3486998266802970665,
//     ]);

//     //
//     const GENERATOR: Fp256<Self> = Fp(BigInt([3, 0, 0, 0]), PhantomData);

//     //
//     const ZERO: Fp256<Self> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

//     //
//     const ONE: Fp256<Self> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

//     const TWO_ADICITY: u32 = 2;

//     // u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208582)
//     // = Fq(-1)
//     const TWO_ADIC_ROOT_OF_UNITY: Fp256<Self> = Fp(
//         BigInt([
//             4332616871279656262,
//             10917124144477883021,
//             13281191951274694749,
//             3486998266802970665,
//         ]),
//         PhantomData,
//     );

//     // const TWO_ADICITY: u32 = 42;
//     // const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N> = value;
//     const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp256<Self>>> = None;

//     fn add_assign(a: &mut ark_ff::Fp<Self, 4>, b: &ark_ff::Fp<Self, 4>) {
//         T::__add_with_carry(&mut a.0, &b.0);
//         T::__subtract_modulus(a);
//     }
//     fn sub_assign(a: &mut ark_ff::Fp<Self, 4>, b: &ark_ff::Fp<Self, 4>) {
//         // If `other` is larger than `self`, add the modulus to self first.
//         if b.0 > a.0 {
//             T::__add_with_carry(&mut a.0, &Self::MODULUS);
//         }
//         T::__sub_with_borrow(&mut a.0, &b.0);
//     }
//     fn double_in_place(a: &mut ark_ff::Fp<Self, 4>) {
//         let b = a.0;
//         T::__add_with_carry(&mut a.0, &b);
//         T::__subtract_modulus(a);
//     }
//     fn neg_in_place(a: &mut ark_ff::Fp<Self, 4>) {
//         if *a != Fp::<Self, 4>::ZERO {
//             let mut tmp = Self::MODULUS;
//             T::__sub_with_borrow(&mut tmp, &a.0);
//             a.0 = tmp;
//         }
//     }
//     fn mul_assign(a: &mut Fp<Self, 4>, b: &Fp<Self, 4>) {
//         #[cfg(target_vendor = "risc0")]
//         {
//             let a_copy = a.clone();
//             unsafe {
//                 let mod_ptr: *const [u32; 8] =
//                     (&Self::MODULUS.0) as *const [u64; 4] as *const [u32; 8];
//                 let a_ptr: *const [u32; 8] = (&a_copy.0 .0) as *const [u64; 4] as *const [u32; 8];
//                 let b_ptr: *const [u32; 8] = (&b.0 .0) as *const [u64; 4] as *const [u32; 8];
//                 let out_ptr: *mut [u32; 8] = (&mut a.0 .0) as *mut [u64; 4] as *mut [u32; 8];
//                 sys_bigint(out_ptr, 0, a_ptr, b_ptr, mod_ptr);
//             }
//         }
//         #[cfg(not(target_vendor = "risc0"))]
//         {
//             // Naive implementation for now
//             let mut aa = Fp256::<T::FullImplConfig>::from_bigint(a.0).unwrap();
//             let bb = Fp256::<T::FullImplConfig>::from_bigint(b.0).unwrap();
//             aa *= bb;

//             a.0 = aa.into_bigint();
//         }
//     }
//     fn sum_of_products<const N: usize>(a: &[Fp256<Self>; N], b: &[Fp256<Self>; N]) -> Fp256<Self> {
//         // Naive implementation
//         if N == 0 {
//             Fp256::<Self>::ZERO
//         } else {
//             let mut sum = a[0] * b[0];
//             for i in 1..N {
//                 sum += a[i] * b[i];
//             }

//             sum
//         }
//     }
//     fn square_in_place(a: &mut Fp256<Self>) {
//         #[cfg(target_vendor = "risc0")]
//         {
//             todo!()
//         }
//         #[cfg(not(target_vendor = "risc0"))]
//         {
//             // Naive implementation for now
//             let b = a.clone();
//             Self::mul_assign(a, &b)
//         }
//     }
//     fn inverse(a: &Fp256<Self>) -> Option<Fp256<Self>> {
//         if a.is_zero() {
//             None
//         } else {
//             // Guajardo Kumar Paar Pelzl
//             // Efficient Software-Implementation of Finite Fields with Applications to
//             // Cryptography
//             // Algorithm 16 (BEA for Inversion in Fp)

//             let one = BigInt::<4>::from(1u64);

//             let mut u = a.0;
//             let mut v = Self::MODULUS;
//             let mut b = Fp::ONE;
//             let mut c = Fp::ZERO;

//             while u != one && v != one {
//                 while u.is_even() {
//                     u.div2();

//                     if b.0.is_even() {
//                         b.0.div2();
//                     } else {
//                         b.0.add_with_carry(&Self::MODULUS);
//                         b.0.div2();
//                     }
//                 }

//                 while v.is_even() {
//                     v.div2();

//                     if c.0.is_even() {
//                         c.0.div2();
//                     } else {
//                         c.0.add_with_carry(&Self::MODULUS);
//                         c.0.div2();
//                     }
//                 }

//                 if v < u {
//                     u.sub_with_borrow(&v);
//                     b -= &c;
//                 } else {
//                     v.sub_with_borrow(&u);
//                     c -= &b;
//                 }
//             }

//             if u == one {
//                 Some(b)
//             } else {
//                 Some(c)
//             }
//         }
//     }
//     fn from_bigint(v: BigInt<4>) -> Option<Fp256<Self>> {
//         let v = Fp::<Self, 4>(v, PhantomData);
//         if v.is_geq_modulus() {
//             None
//         } else {
//             Some(v)
//         }
//     }
//     fn into_bigint(v: ark_ff::Fp<Self, 4>) -> BigInt<4> {
//         v.0
//     }
// }

// // Fq

// pub struct FqPlainConfig;

// impl Fp256PlainConfig for FqPlainConfig {
//     const MODULUS: BigInt<4> = BigInt([
//         4332616871279656263,
//         10917124144477883021,
//         13281191951274694749,
//         3486998266802970665,
//     ]);

//     //
//     const GENERATOR: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([3, 0, 0, 0]), PhantomData);

//     //
//     const ZERO: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([0, 0, 0, 0]), PhantomData);

//     //
//     const ONE: Fp256<Fp256PlainBackend<Self>> = Fp(BigInt([1, 0, 0, 0]), PhantomData);

//     const TWO_ADICITY: u32 = 2;

//     // u256_to_u64s(21888242871839275222246405745257275088696311157297823662689037894645226208582)
//     // = Fq(-1)
//     const TWO_ADIC_ROOT_OF_UNITY: Fp256<Fp256PlainBackend<Self>> = Fp(
//         BigInt([
//             4332616871279656262,
//             10917124144477883021,
//             13281191951274694749,
//             3486998266802970665,
//         ]),
//         PhantomData,
//     );

//     type FullImplConfig = MontBackend<ark_bn254::fq::FqConfig, 4>;

//     // const TWO_ADICITY: u32 = 42;
//     // const TWO_ADIC_ROOT_OF_UNITY: Fp<Self, N> = value;
//     // const SQRT_PRECOMP: Option<SqrtPrecomputation<Fp256<Fp256PlainBackend<Self>>>> = None;
// }

// pub type Fq = Fp256<Fp256PlainBackend<FqPlainConfig>>;

// #[cfg(test)]
// mod test {
//     use super::*;
//     use ark_bn254;

//     #[test]
//     fn test_fq() {
//         let x = BigInt::new([12341234134, 431243124321, 987698769876, 678968976789]);
//         let y = BigInt::new([12341234134, 431243124321, 987698769876, 678968976789]);

//         let x_fq = Fq::from_bigint(x).unwrap();
//         let y_fq = Fq::from_bigint(y).unwrap();

//         let x_ark_fq = ark_bn254::Fq::from_bigint(x).unwrap();
//         let y_ark_fq = ark_bn254::Fq::from_bigint(y).unwrap();

//         let expect_x_plus_y = x_ark_fq + y_ark_fq;
//         let expect_x_squared = x_ark_fq * x_ark_fq;
//         let expect_x_times_y = x_ark_fq * y_ark_fq;
//         let expect_x_minus_y = x_ark_fq - y_ark_fq;
//         let expect_inv = (-y_ark_fq).inverse().unwrap();

//         {
//             let x_plus_y = x_fq + y_fq;
//             assert_eq!(expect_x_plus_y.into_bigint(), x_plus_y.into_bigint());
//         }

//         {
//             let mut x_plus_y = x_fq;
//             x_plus_y += y_fq;
//             assert_eq!(expect_x_plus_y.into_bigint(), x_plus_y.into_bigint(),);
//         }

//         {
//             let x_minus_y = x_fq - y_fq;
//             assert_eq!(expect_x_minus_y.into_bigint(), x_minus_y.into_bigint());
//         }

//         {
//             let mut x_minus_y = x_fq;
//             x_minus_y -= y_fq;
//             assert_eq!(expect_x_minus_y.into_bigint(), x_minus_y.into_bigint());
//         }

//         {
//             let mut y_minus_x = y_fq;
//             y_minus_x -= x_fq;
//             assert_eq!((-expect_x_minus_y).into_bigint(), y_minus_x.into_bigint());
//         }

//         {
//             let x_times_y = x_fq * y_fq;
//             assert_eq!((expect_x_times_y).into_bigint(), x_times_y.into_bigint());
//         }

//         {
//             let minus_y = -y_fq;
//             let inv = minus_y.inverse().unwrap();
//             assert_eq!(expect_inv.into_bigint(), inv.into_bigint());

//             assert_eq!(Fq::from(-1).inverse(), Some(Fq::from(-1)));
//         }

//         {
//             let mut x = x_fq;
//             x.square_in_place();
//             assert_eq!(expect_x_squared.into_bigint(), x.into_bigint());
//         }
//     }
// }
