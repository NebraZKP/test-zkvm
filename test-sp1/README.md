
# INSTALL

```console
curl -L https://sp1.succinct.xyz | bash
```
restart the shell.

install the toolchain
```console
sp1up
```

# BUILD AND RUN

Build zkvm program
```console
cd program
cargo prove build
cd ..
```

Build host prover
```console
cd scripts
[SP1_DEV_MODE=1] cargo run --release
cd ..
```
