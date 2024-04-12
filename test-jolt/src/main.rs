use std::{env, time::Instant};

use ark_bn254::{Bn254, Fq, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{Field, One};
use jolt::{host::Program, JoltPreprocessing};
use shared::{HasRepr, Inputs};

pub fn main() {
    println!("HERE");

    let f1 = Fr::from(13);
    let f2 = Fr::from(27);

    let fqa = -Fq::from(13);
    let fqb = Fq::from(27);

    let a1: G1Affine = (G1Affine::generator() * f1).into();
    let a2: G2Affine = (G2Affine::generator() * f2).into();
    let b1: G1Affine = (G1Affine::generator() * -f2).into();
    let b2: G2Affine = (G2Affine::generator() * f1).into();
    let x2_minus_x1: Fq = b1.x - a1.x;
    let x2_minus_x1_inv: Fq = x2_minus_x1.inverse().unwrap();
    let inputs: Inputs = (
        a1.to_repr(),
        a2.to_repr(),
        b1.to_repr(),
        b2.to_repr(),
        x2_minus_x1_inv.to_repr(),
    );

    let output = {
        if env::var("JOLT_DEV_MODE") == Ok("1".to_string()) {
            println!("DEV MODE.  Execute only.");
            guest::execute_ec_ops(inputs)
        } else {
            let (prove_ec_ops, verify_ec_ops) = guest::build_ec_ops();
            println!("built");

            let now = Instant::now();
            let (output, proof) = prove_ec_ops(inputs);
            println!("prove took {:?}", now.elapsed());
            let is_valid = verify_ec_ops(proof);
            println!("valid: {}", is_valid);
            output
        }
    };
    println!("output: {:?}", output);

    // G1 Sum
    {
        let out = G1Affine::from_repr(&output);
        let expect: G1Affine = (a1 + b1 * Fr::from(3)).into();
        // let expect: G1Affine = (a1 + b1).into();
        println!("out={out:?}");
        println!("expect={expect:?}");
        assert_eq!(expect, out);
    }

    // 2-pairing
    // {
    //     let result_0 = Fq::from_repr(&output);
    //     println!("result_0: {result_0:?}");
    //     assert_eq!(result_0, Fq::one());
    // }
}
