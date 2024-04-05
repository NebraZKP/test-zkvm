//! A simple script to generate and verify the proof of a given program.

use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{One, UniformRand};
use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
// use methods::{NEBRA0_GUEST_ELF, NEBRA0_GUEST_ID};
// use rand::rngs::OsRng;
// use risc0_zkvm::{default_prover, ExecutorEnv};
use shared::{HasRepr, Inputs};
use std::env;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // let n = 186u32;
    // stdin.write(&n);
    // let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // // Read output.
    // let a = proof.public_values.read::<u128>();
    // let b = proof.public_values.read::<u128>();
    // println!("a: {}", a);
    // println!("b: {}", b);

    // // Verify proof.
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // // Save proof.
    // proof
    //     .save("proof-with-io.json")
    //     .expect("saving proof failed");

    // println!("successfully generated and verified proof for the program!")

    let f1 = Fr::from(13);
    let f2 = Fr::from(27);

    let a1: G1Affine = (G1Affine::generator() * f1).into();
    let a2: G2Affine = (G2Affine::generator() * f2).into();
    let b1: G1Affine = (G1Affine::generator() * -f2).into();
    let b2: G2Affine = (G2Affine::generator() * f1).into();
    let inputs: Inputs = (a1.to_repr(), a2.to_repr(), b1.to_repr(), b2.to_repr());

    stdin.write(&inputs);
    let mut public_values = {
        if env::var("SP1_DEV_MODE") == Ok("1".to_string()) {
            println!("SP1_DEV_MODE is set");
            SP1Prover::execute(ELF, stdin).expect("execution failed")
        } else {
            println!("FULL PROVER MODE");
            let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
            proof.public_values
        }
    };

    // Sum G1 points and check
    // {
    //     let output_repr: <G1Affine as HasRepr>::Repr = public_values.read();
    //     let output = G1Affine::from_repr(&output_repr);
    //     println!("output: {output:?}");

    //     let ab1: G1Affine = (a1 + b1).into();
    //     println!("ab1: {ab1:?}");
    //     assert_eq!(ab1, output);
    // }

    // 2-pairing
    {
        let output: <Fq as HasRepr>::Repr = public_values.read();
        let result_0 = Fq::from_repr(&output);
        println!("result_0: {result_0:?}");
        assert_eq!(result_0, Fq::one());
    }
}
