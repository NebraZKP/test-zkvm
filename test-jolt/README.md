## Setup

```console
rustup install nightly-x86_64-unknown-linux-gnu
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
jolt install-toolchain
```

Also needed
```console
sudo apt install libfontconfig
```

```console
rustup target add riscv32i-unknown-none-elf
```

New projects:
```console
jolt new <PROJECT_NAME>
cd <PROJECT_NAME>
```

## Project

```
[JOLT_DEV_MODE=1] cargo run --release
```
