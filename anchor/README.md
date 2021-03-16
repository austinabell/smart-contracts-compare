# Anchor Plutocratic Hosting

## QuickStart

```bash
# Install Solana (needed for compiling to BPF)
sh -c "$(curl -sSfL https://release.solana.com/v1.5.5/install)"

# Install anchor CLI
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

# On ubuntu, install these dependencies
sudo apt-get install -y pkg-config build-essential libudev-dev

# Install dev dependencies to test
yarn

# Build contract code
anchor build

# Run integration tests
anchor test
```