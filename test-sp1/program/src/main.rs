//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use ark_bn254::{Bn254, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::One;
use shared::{g1_add_refs_affine, HasRepr, Inputs};
use sp1_zkvm::syscalls::syscall_bn254_add;

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
    let a1 = G1Affine::from_repr(&inputs.0);
    let a2 = G2Affine::from_repr(&inputs.1);
    let b1 = G1Affine::from_repr(&inputs.2);
    let b2 = G2Affine::from_repr(&inputs.3);

    // Sum G1 points
    // {
    //     // let a1_proj: G1Projective = a1.into();
    //     // let b1_proj: G1Projective = b1.into();

    //     // println!("cycle-tracker-start: G1 add proj (ark_bn254)");
    //     // let ab1_refs = sum_refs(&a1, &b1);
    //     // println!("cycle-tracker-end: G1 add proj (ark_bn254)");
    //     // let ab1: G1Affine = ab1_refs.into();
    //     // sp1_zkvm::io::commit(&ab1.to_repr());

    //     println!("cycle-tracker-start: G1 add affine (ark_bn254)");
    //     let ab1_refs = add_g1_refs_affine(&a1, &b1);
    //     let ab1: G1Affine = ab1_refs.into();
    //     println!("cycle-tracker-end: G1 add affine (ark_bn254)");
    //     sp1_zkvm::io::commit(&ab1.to_repr());

    //     // let mut ab1: G1Affine = a1.clone();
    //     // let mut b1_copy: G1Affine = b1.clone();

    //     // let mut a1_data = inputs.0;
    //     // let mut b1_data = inputs.2;

    //     // println!("cycle-tracker-start: G1 add (syscall)");
    //     // let mut a1_ptr: *mut u64 = &mut a1_data[0][0];
    //     // let mut b1_ptr: *mut u64 = &mut b1_data[0][0];
    //     // unsafe {
    //     //     syscall_bn254_add(a1_ptr as *mut u32, b1_ptr as *mut u32);
    //     // }
    //     // println!("cycle-tracker-end: G1 add (syscall)");

    //     // sp1_zkvm::io::commit(&a1_data);
    // }

    // G1 (affine) add * 10
    {
        const NUM_ITERATIONS: u32 = 10;

        // ark_bn256 (affine)
        {
            let mut ab_sum = a1;
            println!(
                "cycle-tracker-start: G1 add {} pts affine (ark_bn254)",
                NUM_ITERATIONS
            );
            for _ in 0..NUM_ITERATIONS {
                ab_sum = g1_add_refs_affine(&ab_sum, &b1);
            }
            println!(
                "cycle-tracker-end: G1 add {} pts affine (ark_bn254)",
                NUM_ITERATIONS
            );

            sp1_zkvm::io::commit(&ab_sum.to_repr());
        }

        // syscall
        // {
        //     let mut a1_data = inputs.0;
        //     let mut b1_data = inputs.2;
        //     let mut a1_ptr: *mut u64 = &mut a1_data[0][0];
        //     let mut b1_ptr: *mut u64 = &mut b1_data[0][0];

        //     println!(
        //         "cycle-tracker-start: G1 add {} pts affine (syscall)",
        //         NUM_ITERATIONS
        //     );
        //     for _ in 0..NUM_ITERATIONS {
        //         unsafe {
        //             syscall_bn254_add(a1_ptr as *mut u32, b1_ptr as *mut u32);
        //         }
        //     }

        //     println!(
        //         "cycle-tracker-end: G1 add {} pts affine (syscall)",
        //         NUM_ITERATIONS
        //     );
        //     sp1_zkvm::io::commit(&a1_data);
        // }
    }

    // 2-pairing
    // {
    //     println!("cycle-tracker-start: 2-pairing (ark_bn)");
    //     let multi_miller_result = Bn254::multi_miller_loop(&[a1, b1], &[a2, b2]);
    //     let pairing_result = Bn254::final_exponentiation(multi_miller_result);
    //     println!("cycle-tracker-end: 2-pairing (ark_bn)");

    //     // Check pairing result
    //     if let Some(target_field_value) = pairing_result {
    //         sp1_zkvm::io::commit(&target_field_value.0.c0.c0.c0.to_repr());

    //         // if target_field_value.0 == <<Bn254 as Pairing>::TargetField as One>::one() {
    //         //     env::exit(0)
    //         // }
    //     }
    //     // env::exit(1);
    // }
}
