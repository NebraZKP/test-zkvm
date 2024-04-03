#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: (u32, u64) = env::read();

    // TODO: do something with the input
    let result: u32 = input.0 ^ ((input.1 >> 32) as u32) ^ (input.1 as u32);

    // write public output to the journal
    env::commit(&result);
}
