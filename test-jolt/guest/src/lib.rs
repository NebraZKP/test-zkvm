#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use ark_bn254::{Bn254, Fq, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use shared::{g1_add_refs_affine, HasRepr, Inputs};

#[jolt::provable(stack_size = 2048000, memory_size = 204800000)]
fn ec_ops(inputs: Inputs) -> <G1Affine as HasRepr>::Repr {
    // Sum G1 points
    {
        // Must be consistent with host-side main.rs
        const NUM_ITERATIONS: u32 = 3;

        let a1 = G1Affine::from_repr(&inputs.0);
        let b1 = G1Affine::from_repr(&inputs.2);

        // ark_bn256 (affine)
        {
            let mut ab_sum = a1;
            for _ in 0..NUM_ITERATIONS {
                ab_sum = g1_add_refs_affine(&ab_sum, &b1);
            }
            ab_sum.to_repr()
        }
    }
}

// TO RUN THIS EXPERIMENT INSTEAD:
// - comment the above
// - uncomment this
// - see host-side main.rs and make the equivalent changes

// #[jolt::provable(stack_size = 2048000, memory_size = 204800000)]
// fn ec_ops(inputs: Inputs) -> <Fq as HasRepr>::Repr {
//     // 2-pairing
//     {
//         let a1 = G1Affine::from_repr(&inputs.0);
//         let a2 = G2Affine::from_repr(&inputs.1);
//         let b1 = G1Affine::from_repr(&inputs.2);
//         let b2 = G2Affine::from_repr(&inputs.3);

//         let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
//         let pairing_result = Bn254::final_exponentiation(multi_miller_result);

//         // Check pairing result
//         if let Some(target_field_value) = pairing_result {
//             if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
//                 return target_field_value.0.c0.c0.c0.to_repr();
//             }
//         }

//         panic!("pairing check failed");
//     }
// }
