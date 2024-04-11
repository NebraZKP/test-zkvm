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
    field::{add_mod_q, inv_mod_q, u256_from_u64s, MyFq, Q},
    HasRepr, Inputs,
};
// use serde::{Deserialize, Serialize};
use crypto_bigint::U256;

risc0_zkvm::guest::entry!(main);

fn sum_refs(a: &G1Projective, b: &G1Projective) -> G1Projective {
    a + b
}

fn sum_vals(a: G1Projective, b: G1Projective) -> G1Projective {
    a + b
}

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

    let a1 = G1Affine::from_repr(&inputs.0).into_group();
    let a2 = G2Affine::from_repr(&inputs.1).into_group();
    let b1 = G1Affine::from_repr(&inputs.2).into_group();
    let b2 = G2Affine::from_repr(&inputs.3).into_group();

    let a1_x_bigint = u256_from_u64s(&inputs.0[0]);
    let a1_y_bigint = u256_from_u64s(&inputs.0[1]);

    let a1_x_residue = MyFq::new(&a1_x_bigint);
    let a1_y_residue = MyFq::new(&a1_y_bigint);
    let b1_x_residue = MyFq::new(&u256_from_u64s(&inputs.2[0]));
    let b1_y_residue = MyFq::new(&u256_from_u64s(&inputs.2[1]));

    let a1_residue = [a1_x_residue, a1_y_residue];
    let b1_residue = [b1_x_residue, b1_y_residue];
    let a1_plus_b1_hint = MyFq::new(&u256_from_u64s(&inputs.4));

    let a1_x = a1.x;
    let a1_y = a1.y;

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
    //     let x6 = env::cycle_count();

    //     // Naive
    //     let r_naive = a1_x * a1_y;
    //     // let r_naive_0 = a1_y * a1_x;
    //     // let r_naive_1 = a1_x * r_naive_0;
    //     // let r_naive_2 = a1_x * r_naive_1;
    //     // let r_naive_3 = a1_x * r_naive_2;

    //     // let r = mul_mod_q(&a1_x_bigint, &a1_y_bigint);

    //     // Syscall
    //     // let r = risc0::modmul_u256(&a1_x_bigint, &a1_y_bigint, &Q);

    //     // Syscall via Residue
    //     // let r = a1_x_residue * a1_y_residue;

    //     let x7 = env::cycle_count();

    //     // env::commit(&r.as_montgomery().to_words());
    //     // env::commit(&r.to_words());
    //     env::commit(&r_naive.to_repr());
    //     env::commit(&r_naive_0.to_repr());
    //     env::commit(&r_naive_1.to_repr());
    //     env::commit(&r_naive_2.to_repr());
    //     env::commit(&r_naive_3.to_repr());

    //     println!("cycles: {}", x7 - x6);
    // }

    // Sum G1 points
    {
        let x6 = env::cycle_count();
        // ark_bn254 (refs)
        // let ab1_refs = sum_refs(&a1, &b1);

        // let ab1_vals = sum_vals(a1, b1);

        // With Residue

        // let ab_residue = g1_add(&a1_residue, &b1_residue);
        let ab_residue = g1_add_with_hint(&a1_residue, &b1_residue, &a1_plus_b1_hint);

        let x7 = env::cycle_count();

        let ab1: G1Affine = ab1_refs.into();
        env::commit(&ab1.to_repr());

        // env::commit(&[
        //     ab_residue[0].as_montgomery().to_words(),
        //     ab_residue[1].as_montgomery().to_words(),
        // ]);

        println!("cycles: {}", x7 - x6);
    }

    // 2-pairing
    // {
    //     let x6 = env::cycle_count();
    //     let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    //     let pairing_result = Bn254::final_exponentiation(multi_miller_result);
    //     let x7 = env::cycle_count();

    //     // Check pairing result
    //     if let Some(target_field_value) = pairing_result {
    //         env::commit(&target_field_value.0.c0.c0.c0.to_repr());

    //         // if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
    //         //     env::exit(0)
    //         // }
    //     }
    //     // env::exit(1);

    //     println!("cycles: {}", x7 - x6);
    // }

    // env::log("f1_repr:");
    // env::write(f1.fmt());
    // env::write("\n");
    // env::log(format!("f1: {f1:?}"));

    // // write public output to the journal
    // env::commit(&(result0, result1.to_repr()));
}
