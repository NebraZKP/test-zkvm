// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use ark_bn254::{Bn254, Fq, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{One, UniformRand};
use methods::{NEBRA0_GUEST_ELF, NEBRA0_GUEST_ID};
use rand::rngs::OsRng;
use risc0_zkvm::{default_prover, ExecutorEnv};
use shared::{HasRepr, Inputs};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // let f1 = Fr::rand(&mut OsRng);
    // let f2 = Fr::rand(&mut OsRng);
    let f1 = Fr::from(13);
    let f2 = Fr::from(27);

    let fqa = -Fr::from(13);
    let fqb = Fr::from(27);

    let a1: G1Affine = (G1Affine::generator() * f1).into();
    let a2: G2Affine = (G2Affine::generator() * f2).into();
    let b1: G1Affine = (G1Affine::generator() * -f2).into();
    let b2: G2Affine = (G2Affine::generator() * f1).into();

    // {
    //     let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    //     let pairing_result = Bn254::final_exponentiation(multi_miller_result);
    //     if let Some(target_field_value) = pairing_result {
    //         let fq12_val = target_field_value.0;
    //         let fq12_val_c0 = fq12_val.c0;
    //         let fq12_val_c0_c0 = fq12_val_c0.c0;
    //         let fq12_val_c0_c0_c0 = fq12_val_c0_c0.c0;
    //         assert_eq!(
    //             target_field_value.0,
    //             <<Bn254 as Pairing>::TargetField as One>::one()
    //         );
    //     } else {
    //         panic!("pairing failed");
    //     }
    // }

    // println!("a1: {a1:?}");
    // println!("a2: {a2:?}");
    // println!("b1: {b1:?}");
    // println!("b2: {b2:?}");

    let inputs: Inputs = (a1.to_repr(), a2.to_repr(), b1.to_repr(), b2.to_repr());
    let env = ExecutorEnv::builder()
        // .write(&inputs)
        .write(&[fqa.to_repr(), fqb.to_repr()])
        .unwrap()
        .build()
        .unwrap();

    // println!("e(a1,a2).e(b1,b2): {pairing_result:?}");

    // let f1 = Fr::from(1);
    // let f1_repr = f1.to_repr();
    // println!("f1_repr: {f1_repr:?}");
    // println!("f1: {f1:?}");

    // For example:
    // let input: (u32, u64, <Fr as HasRepr>::Repr) =
    //     (15 * u32::pow(2, 27) + 1, 0xf * u64::pow(2, 60) + 1, f1_repr);

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, NEBRA0_GUEST_ELF).unwrap();

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    // let output: (u32, <Fr as HasRepr>::Repr) = receipt.journal.decode().unwrap();
    // let output_repr: <G1Affine as HasRepr>::Repr = receipt.journal.decode().unwrap();
    // let output = G1Affine::from_repr(&output_repr);
    // println!("Inputs: {inputs:?}, output: {output:?}");

    // Sum Fq values
    {
        // let expect = a1.x + a1.y;
        let expect = fqa * fqb;
        let output_repr: [u32; 8] = receipt.journal.decode().unwrap();
        println!("expect: {expect:?}, actual: {output_repr:?}");
    }

    // Sum G1 points
    // {
    //     let output_repr: <G1Affine as HasRepr>::Repr = receipt.journal.decode().unwrap();
    //     let output = G1Affine::from_repr(&output_repr);
    //     println!("output: {output:?}");

    //     let ab1: G1Affine = (a1 + b1).into();
    //     println!("ab1: {ab1:?}");
    //     assert_eq!(ab1, output);
    // }

    // 2-pairing
    // {
    //     let output: <Fq as HasRepr>::Repr = receipt.journal.decode().unwrap();
    //     let result_0 = Fq::from_repr(&output);
    //     println!("result_0: {result_0:?}");
    //     assert_eq!(result_0, Fq::one());
    // }

    // The receipt was verified at the end of proving, but the below code is 2an
    // example of how someone else could verify this receipt.
    // receipt.verify(NEBRA0_GUEST_ID).unwrap();
}
