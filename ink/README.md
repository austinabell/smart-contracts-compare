# Ink Plutocratic Hosting

## QuickStart

```bash
# Nightly toolchain is needed to compile these contracts
rustup toolchain install nightly

# Install wasm32 target
rustup target add wasm32-unknown-unknown
```

Install [binaryen](https://github.com/WebAssembly/binaryen) package (for wasm code optimization)

```bash
# CLI through cargo which allows for contract initialization and interaction
# Use `cargo contract --help` to see available commands
cargo install cargo-contract --vers 0.10.0 --force --locked

# Set directory toolchain default to nightly to compile and test
rustup override set nightly

cargo contract build
cargo contract test
```
