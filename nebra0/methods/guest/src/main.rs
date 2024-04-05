#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

use ark_bn254::{Bn254, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use risc0_zkvm::guest::env;
use shared::{HasRepr, Inputs};
// use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);

fn sum_refs(a: &G1Projective, b: &G1Projective) -> G1Projective {
    a + b
}

fn sum_vals(a: G1Projective, b: G1Projective) -> G1Projective {
    a + b
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

    // let x1 = env::cycle_count();
    // let x2 = env::cycle_count();

    let a1 = G1Affine::from_repr(&inputs.0).into_group();
    // let x3 = env::cycle_count();

    let a2 = G2Affine::from_repr(&inputs.1);
    // let x4 = env::cycle_count();

    let b1 = G1Affine::from_repr(&inputs.2).into_group();
    // let x5 = env::cycle_count();

    let b2 = G2Affine::from_repr(&inputs.3);
    // let x6 = env::cycle_count();

    // Sum G1 points
    // {
    //     // let x6 = env::cycle_count();
    //     // let ab1_vals = sum_vals(a1, b1);
    //     let ab1_refs = sum_refs(&a1, &b1);
    //     // let x7 = env::cycle_count();

    //     // let x8 = env::cycle_count();

    //     // let ab1: G1Affine = ab1_vals.into();
    //     let ab1: G1Affine = ab1_refs.into();

    //     env::commit(&ab1.to_repr());
    //     // let x9 = env::cycle_count();

    //     // println!("cycles1: {x1}");
    //     // println!("cycles2: {x2}");
    //     // println!("cycles3: {x3}");
    //     // println!("cycles4: {x4}");
    //     // println!("cycles5: {x5}");
    //     // println!("cycles6: {x6}");
    //     // println!("cycles7: {x7}");
    //     // println!("cycles8: {x8}");
    //     // println!("cycles9: {x9}");
    // }

    // 2-pairing
    {
        let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
        let pairing_result = Bn254::final_exponentiation(multi_miller_result);

        // Check pairing result
        if let Some(target_field_value) = pairing_result {
            env::commit(&target_field_value.0.c0.c0.c0.to_repr());

            // if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
            //     env::exit(0)
            // }
        }
        // env::exit(1);
    }

    // env::log("f1_repr:");
    // env::write(f1.fmt());
    // env::write("\n");
    // env::log(format!("f1: {f1:?}"));

    // // write public output to the journal
    // env::commit(&(result0, result1.to_repr()));
}
