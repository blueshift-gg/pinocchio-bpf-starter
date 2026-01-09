# Pinocchio BPF Starter

A template for creating Solana BPF programs in Pinocchio using upstream BPF.

## Prerequisites

Install the required tools:

```bash
# Install sbpf-linker
cargo install sbpf-linker

# Install cargo-generate
cargo install cargo-generate
```

## Usage

Create a new project from this template:

```bash
cargo generate --git https://github.com/blueshift-gg/pinocchio-bpf-starter.git
```

## Building

Build your BPF program:

```bash
cargo +nightly build-bpf
```

The compiled program will be at:
```
target/bpfel-unknown-none/release/libyour_program_name.so
```

## Testing

Run tests:

```bash
cargo test
```

## License

MIT
