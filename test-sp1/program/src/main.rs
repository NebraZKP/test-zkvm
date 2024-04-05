//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::{Bn254, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use shared::{HasRepr, Inputs};

fn sum_refs(a: &G1Projective, b: &G1Projective) -> G1Projective {
    a + b
}

fn sum_vals(a: G1Projective, b: G1Projective) -> G1Projective {
    a + b
}

pub fn main() {
    // // NOTE: values of n larger than 186 will overflow the u128 type,
    // // resulting in output that doesn't match fibonacci sequence.
    // // However, the resulting proof will still be valid!
    // let n = sp1_zkvm::io::read::<u32>();
    // let mut a: u128 = 0;
    // let mut b: u128 = 1;
    // let mut sum: u128;
    // for _ in 1..n {
    //     sum = a + b;
    //     a = b;
    //     b = sum;
    // }

    // sp1_zkvm::io::commit(&a);
    // sp1_zkvm::io::commit(&b);

    let inputs: Inputs = sp1_zkvm::io::read();
    let a1 = G1Affine::from_repr(&inputs.0).into_group();
    let a2 = G2Affine::from_repr(&inputs.1);
    let b1 = G1Affine::from_repr(&inputs.2).into_group();
    let b2 = G2Affine::from_repr(&inputs.3);

    // Sum G1 points
    // {
    //     let ab1_refs = sum_refs(&a1, &b1);
    //     let ab1: G1Affine = ab1_refs.into();

    //     sp1_zkvm::io::commit(&ab1.to_repr());
    // }

    // 2-pairing
    {
        let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
        let pairing_result = Bn254::final_exponentiation(multi_miller_result);

        // Check pairing result
        if let Some(target_field_value) = pairing_result {
            sp1_zkvm::io::commit(&target_field_value.0.c0.c0.c0.to_repr());

            // if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
            //     env::exit(0)
            // }
        }
        // env::exit(1);
    }
}
