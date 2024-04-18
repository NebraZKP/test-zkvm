#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

#[cfg(all(target_os = "zkvm", target_arch = "riscv32"))]
use crypto_bigint::risc0;

use ark_bn254::{Bn254, Fq, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{Field, One};
use risc0_zkvm::guest::env;
use shared::{
    backend,
    field::{add_mod_q, inv_mod_q, u256_from_u64s, MyFq},
    g1_add_refs_affine, halo2curvesfn, HasRepr, Inputs,
};
// use serde::{Deserialize, Serialize};
use crypto_bigint::U256;

risc0_zkvm::guest::entry!(main);

extern "C" {
    /// The risc0 syscall for mulmod
    fn sys_bigint(
        result: *mut [u64; 4],
        op: u32,
        x: *const [u64; 4],
        y: *const [u64; 4],
        modulus: *const [u64; 4],
    );
}

const Q: [u64; 4] = [
    4332616871279656263,
    10917124144477883021,
    13281191951274694749,
    3486998266802970665,
];

fn g1_add(a: &[MyFq; 2], b: &[MyFq; 2]) -> [MyFq; 2] {
    let x1 = &a[0];
    let y1 = &a[1];
    let x2 = &b[0];
    let y2 = &b[1];

    let x2_minus_x1 = x2 - x1;
    let (x2_minus_x1_inv, _) = x2_minus_x1.invert();
    let lambda = (y2 - y1) * x2_minus_x1_inv;
    let lambda_squared = lambda * lambda;

    let xr = lambda_squared - x1 - x2;
    let yr = lambda * (x1 - xr) - y1;

    [xr, yr]
}

fn g1_add_with_hint(a: &[MyFq; 2], b: &[MyFq; 2], x2_minus_x1_inv: &MyFq) -> [MyFq; 2] {
    let x1 = &a[0];
    let y1 = &a[1];
    let x2 = &b[0];
    let y2 = &b[1];

    let x2_minus_x1 = x2 - x1;
    if x2_minus_x1 * x2_minus_x1_inv != MyFq::ONE {
        panic!("invalid hint");
    }

    let lambda = (y2 - y1) * x2_minus_x1_inv;
    let lambda_squared = lambda * lambda;

    let xr = lambda_squared - x1 - x2;
    let yr = lambda * (x1 - xr) - y1;

    [xr, yr]
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    // let (a1_repr, a2_repr, b1_repr, b2_repr): (G1Repr, G2Repr, G1Repr, G2Repr) = env::read();

    // // Pairing check
    // let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    // let pairing_result = Bn254::final_exponentiation(multi_miller_result);
    // if let Some(value) = pairing_result {
    //     if value.0 == Bn254::PairingOutput::one() {
    //         env::exit(0);
    //     }
    // }

    // let result = 7u32;
    // env::commit(&result);
    // env::exit(0);

    // // TODO: do something with the input
    // let input: (u32, u64, <Fr as HasRepr>::Repr) = env::read();
    // let f1 = Fr::from_repr(&input.2);
    // let result0: u32 = input.0 ^ ((input.1 >> 32) as u32) ^ (input.1 as u32);
    // let result1: Fr = f1 + f1;

    let inputs: Inputs = env::read();

    let a1 = G1Affine::from_repr(&inputs.0);
    let a2 = G2Affine::from_repr(&inputs.1);
    let b1 = G1Affine::from_repr(&inputs.2);
    let b2 = G2Affine::from_repr(&inputs.3);

    // let a1_x_bigint = u256_from_u64s(&inputs.0[0]);
    // let a1_y_bigint = u256_from_u64s(&inputs.0[1]);

    // let a1_x_residue = MyFq::new(&a1_x_bigint);
    // let a1_y_residue = MyFq::new(&a1_y_bigint);
    // let b1_x_residue = MyFq::new(&u256_from_u64s(&inputs.2[0]));
    // let b1_y_residue = MyFq::new(&u256_from_u64s(&inputs.2[1]));

    // let a1_residue = [a1_x_residue, a1_y_residue];
    // let b1_residue = [b1_x_residue, b1_y_residue];
    // let a1_plus_b1_hint = MyFq::new(&u256_from_u64s(&inputs.4));

    // let a1_x: Fq = a1.x;
    // let a1_y: Fq = a1.y;

    // Invert input[0]
    // {
    //     let x6 = env::cycle_count();

    //     let r = a1_x.inverse().unwrap();
    //     // let r = inv_mod_q(&a1_x_bigint);
    //     // let (r, _) = a1_x_residue.invert();

    //     let x7 = env::cycle_count();

    //     // env::commit(&r.as_montgomery().to_words());
    //     // env::commit(&r.to_words());
    //     env::commit(&r.to_repr());

    //     println!("cycles: {}", x7 - x6);
    // }

    // Mul Fq values
    // {
    //     let mut r = [0u64; 4];

    //     let x6 = std::hint::black_box(env::cycle_count());

    //     // Naive
    //     // let r_naive = std::hint::black_box(a1_x * a1_y);
    //     // let r_naive_0 = a1_y * a1_x;
    //     // let r_naive_1 = a1_x * r_naive_0;
    //     // let r_naive_2 = a1_x * r_naive_1;
    //     // let r_naive_3 = a1_x * r_naive_2;

    //     // let r = mul_mod_q(&a1_x_bigint, &a1_y_bigint);

    //     // Syscall via Residue class
    //     // let r_residue = std::hint::black_box(a1_x_residue * a1_y_residue);

    //     // Syscall
    //     // let r = risc0::modmul_u256(&a1_x_bigint, &a1_y_bigint, &Q);

    //     // Direct syscall
    //     std::hint::black_box(unsafe {
    //         let mod_ptr: *const [u64; 4] = &Q as *const [u64; 4];
    //         let a_ptr: *const [u64; 4] = &inputs.0[0];
    //         let b_ptr: *const [u64; 4] = &inputs.0[1];
    //         let r_ptr: *mut [u64; 4] = &mut r;
    //         sys_bigint(r_ptr, 0, a_ptr, b_ptr, mod_ptr);
    //     });

    //     // backend::Fq
    //     // let r_backend = {
    //     //     let a1_x = <backend::Fq as HasRepr>::from_repr(&inputs.0[0]);
    //     //     let a1_y = <backend::Fq as HasRepr>::from_repr(&inputs.0[1]);
    //     //     a1_x * a1_y
    //     // };

    //     let x7 = std::hint::black_box(env::cycle_count());

    //     // env::commit(&r_residue.as_montgomery().to_words());
    //     env::commit(&r);
    //     // env::commit(&r_residue.to_words());

    //     // env::commit(&r_naive.to_repr());
    //     // env::commit(&r_naive_0.to_repr());
    //     // env::commit(&r_naive_1.to_repr());
    //     // env::commit(&r_naive_2.to_repr());
    //     // env::commit(&r_naive_3.to_repr());
    //     // env::commit(&r_backend.to_repr());

    //     println!("cycles: {}", x7 - x6);
    // }

    // Sum G1 points
    // {
    //     let x6 = env::cycle_count();
    //     // ark_bn254 (refs)
    //     // let ab = sum_refs(&a1, &b1);

    //     // let ab = sum_vals(a1, b1);

    //     // With Residue

    //     let ab_residue = g1_add(&a1_residue, &b1_residue);
    //     // let ab_residue = g1_add_with_hint(&a1_residue, &b1_residue, &a1_plus_b1_hint);

    //     let x7 = env::cycle_count();

    //     // env::commit(&ab.to_repr());

    //     env::commit(&[
    //         ab_residue[0].as_montgomery().to_words(),
    //         ab_residue[1].as_montgomery().to_words(),
    //     ]);

    //     println!("cycles: {}", x7 - x6);
    // }

    // G1 (affine) add * 10
    // {
    //     const NUM_ITERATIONS: u32 = 10;

    //     // ark_bn256
    //     {
    //         let mut ab_sum = a1;
    //         let x6 = env::cycle_count();
    //         for _ in 0..NUM_ITERATIONS {
    //             ab_sum = g1_add_refs_affine(&ab_sum, &b1);
    //         }
    //         let x7 = env::cycle_count();

    //         env::commit(&ab_sum.to_repr());
    //         println!("cycles: {}", x7 - x6);
    //     }

    //     // With Residue
    //     // {
    //     //     let mut ab_sum = a1_residue;
    //     //     let x6 = env::cycle_count();
    //     //     for _ in 0..10 {
    //     //         // NOTE: could be more efficient with in-place add, but for now the
    //     //         // goal is not to get the most efficient possible impl of G1, but
    //     //         // rather get time estimates for various worklodas..
    //     //         ab_sum = g1_add(&ab_sum, &b1_residue);
    //     //     }
    //     //     let x7 = env::cycle_count();

    //     //     env::commit(&[
    //     //         ab_sum[0].as_montgomery().to_words(),
    //     //         ab_sum[1].as_montgomery().to_words(),
    //     //     ]);
    //     //     println!("cycles: {}", x7 - x6);
    //     // }
    // }

    // 2-pairing
    {
        // arkworks
        {
            let x6 = env::cycle_count();
            let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
            let pairing_result = Bn254::final_exponentiation(multi_miller_result);
            let x7 = env::cycle_count();

            // Check pairing result
            if let Some(target_field_value) = pairing_result {
                env::commit(&target_field_value.0.c0.c0.c0.to_repr());

                // if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
                //     env::exit(0)
                // }
            }
            // env::exit(1);

            println!("cycles: {}", x7 - x6);
        }

        // halo2curves
        // {
        //     let x6 = env::cycle_count();
        //     let r = halo2curvesfn::do_2_pairing(&inputs.0, &inputs.1, &inputs.2, &inputs.3);
        //     let x7 = env::cycle_count();

        //     env::commit(&r);
        //     println!("cycles: {}", x7 - x6);
        // }
    }

    // env::log("f1_repr:");
    // env::write(f1.fmt());
    // env::write("\n");
    // env::log(format!("f1: {f1:?}"));

    // // write public output to the journal
    // env::commit(&(result0, result1.to_repr()));
}
