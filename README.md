## Getting Started

### Cargo

- **Rust**: Install Rust through [rustup.rs](https://rustup.rs/)
- **System dependencies**: 
  - `openssl`
  - `pkg-config`

### Using Nix (Alternative)

- **Nix**: Install [Nix package manager](https://nixos.org/download.html)
- **Nix Flakes**: Ensure flakes are enabled

## Building

### Using Cargo:

```bash
cargo build --release
```

### There is also a nix flake:

```bash
nix build
```
or
```bash
nix shell
```
