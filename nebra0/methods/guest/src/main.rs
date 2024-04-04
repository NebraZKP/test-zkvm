#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_ec::{
    pairing::{Pairing, PairingOutput},
    AffineRepr,
};
use ark_ff::One;
use risc0_zkvm::guest::env;
use shared::{HasRepr, Inputs};
// use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);

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

    // let ab1: G1Affine = (a1 + b1).into();
    // env::commit(&ab1.to_repr());

    let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    let pairing_result = Bn254::final_exponentiation(multi_miller_result);

    // Check pairing result
    if let Some(target_field_value) = pairing_result {
        if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
            env::exit(0)
        }
    }
    env::exit(1);

    // env::log("f1_repr:");
    // env::write(f1.fmt());
    // env::write("\n");
    // env::log(format!("f1: {f1:?}"));

    // // write public output to the journal
    // env::commit(&(result0, result1.to_repr()));
}
