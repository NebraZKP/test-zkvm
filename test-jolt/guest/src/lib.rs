#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use ark_bn254::{Bn254, Fq, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use shared::{HasRepr, Inputs};

fn sum_proj(a: &G1Projective, b: &G1Projective) -> G1Projective {
    a + b
}

fn sum_affine(a: &G1Affine, b: &G1Affine) -> G1Projective {
    a.clone() + b
}

// #[jolt::provable()]
#[jolt::provable(stack_size = 2048000, memory_size = 204800000)]
fn ec_ops(inputs: Inputs) -> <G1Affine as HasRepr>::Repr {
    // Sum G1 points
    {
        let a1 = G1Affine::from_repr(&inputs.0); /* .into_group() */
        let b1 = G1Affine::from_repr(&inputs.2);

        // let ab1_refs = sum_affine(&a1, &b1);
        // let ab1: G1Affine = ab1_refs.into();
        // ab1.to_repr()

        // G1 Sum (affine)
        // let mut acc: G1Affine = a1.into();
        // let b1: G1Affine = b1.into();
        // for _ in 0..100 {
        //     acc = sum_affine(acc, &b1).into();
        // }
        // acc.to_repr()

        // G1 Sum (proj)
        let mut acc: G1Projective = a1.into();
        let b1: G1Projective = b1.into();
        for _ in 0..3 {
            acc = sum_proj(&acc, &b1);
        }
        let acc_affine: G1Affine = acc.into();
        acc_affine.to_repr()
    }

    // 2-pairing
    // {
    //     let a1 = G1Affine::from_repr(&inputs.0); /* .into_group() */
    //     let a2 = G2Affine::from_repr(&inputs.1); /* .into_group() */
    //     let b1 = G1Affine::from_repr(&inputs.2);
    //     let b2 = G2Affine::from_repr(&inputs.3); /* .into_group() */

    //     let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    //     let pairing_result = Bn254::final_exponentiation(multi_miller_result);

    //     // Check pairing result
    //     if let Some(target_field_value) = pairing_result {
    //         if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
    //             return target_field_value.0.c0.c0.c0.to_repr();
    //         }
    //     }

    //     panic!("pairing check failed");
    // }
}
