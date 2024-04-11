use std::env;

fn main() {
    for (key, value) in env::vars() {
        if key.starts_with("CARGO_CFG_") {
            println!("{}: {:?}", key, value);
        }
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    // if target_arch != "x86_64" {
    //     panic!("stop and dump stdout");
    // }

    // CARGO_CFG_TARGET_ARCH: "x86_64"
    // panic!("stop and dump stdout");
}
