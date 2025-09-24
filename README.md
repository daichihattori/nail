# Nail

[![Rust](https://github.com/daichihattori/nail/workflows/CI/badge.svg)](https://github.com/daichihattori/nail/actions)
[![Crates.io](https://img.shields.io/crates/v/nail.svg)](https://crates.io/crates/nail)
[![Documentation](https://docs.rs/nail/badge.svg)](https://docs.rs/nail)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/daichihattori/nail#license)

A high-performance **fixed-size** big integer library for Rust with configurable limb count. Unlike dynamic big integer libraries, `nail` uses compile-time sized arrays for predictable memory usage and optimal performance.

## ✨ Features

- **🔢 Compile-time sizing**: `Uint<N>` where N is the number of 64-bit limbs
- **🚀 Zero-allocation arithmetic**: All operations use stack-allocated arrays
- **🔄 Mixed-size operations**: Add/multiply integers with different limb counts
- **🔐 Modular arithmetic**: Built-in support for modular operations
- **⚡ High performance**: Optimized algorithms with predictable performance
- **🧪 Comprehensive testing**: Extensive test suite including edge cases
- **🔒 Cryptography-ready**: Constant-time operations suitable for cryptographic use
- **📦 Zero dependencies**: No runtime dependencies

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
nail = "0.1.0"
```

## 📝 Basic Usage

```rust
use bigint_rs::{U256, U512};

// Create 256-bit integers
let a = U256::from_u64(0xFFFFFFFFFFFFFFFF);
let b = U256::from_u64(0x1000);

// Addition
let sum = a.add(&b);

// Multiplication
let product = a.mul(&b);

// Mixed-size operations (larger on left)
let large = U512::from_u64(1000);  // 512-bit
let small = U256::from_u64(2000);  // 256-bit
let result = large.add(&small);    // Returns 512-bit result

// Modular arithmetic
let mod_result = a.addmod_bits(&b, 256);  // (a + b) mod 2^256
```

## 📚 Type Aliases

The library provides convenient type aliases for common bit sizes:

- `U64` - 64-bit unsigned integer (1 limb)
- `U128` - 128-bit unsigned integer (2 limbs)
- `U256` - 256-bit unsigned integer (4 limbs)
- `U512` - 512-bit unsigned integer (8 limbs)
- `U1024` - 1024-bit unsigned integer (16 limbs)

You can also create custom sizes: `Uint<N>` where N is the number of 64-bit limbs.

## 🔧 Examples

The `examples/` directory contains comprehensive demonstrations:

### Run Basic Usage Example
```bash
cargo run --example basic_usage
```
Shows fundamental operations including arithmetic, different bit sizes, and modular operations.

### Run Cryptography Example
```bash
cargo run --example cryptography
```
Demonstrates cryptographic use cases like RSA operations, elliptic curve simulations, and hash functions.

### Run Performance Example
```bash
cargo run --example performance
```
Performance analysis and benchmarking across different integer sizes.

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_addition
```

The library includes comprehensive tests covering:
- Basic arithmetic operations
- Edge cases and overflow handling
- Mixed-size operations
- Modular arithmetic
- Large number multiplication
- Carry propagation

## 📊 Benchmarking

### Run Local Benchmarks

```bash
# Install criterion (if not already installed)
cargo install cargo-criterion

# Run all benchmarks
cargo criterion

# Run specific benchmark group
cargo criterion --bench bigint_bench

# Generate benchmark report
cargo criterion --message-format=json > benchmark_results.json
```

### Benchmark Comparison

This library is benchmarked against other popular Rust big integer libraries:
- [num-bigint](https://crates.io/crates/num-bigint) - Dynamic big integers
- [rug](https://crates.io/crates/rug) - GMP bindings
- [malachite](https://crates.io/crates/malachite) - Pure Rust arbitrary precision


## 🏗️ Architecture

### Memory Layout
- **Fixed-size arrays**: `[u64; LIMBS]` stored on the stack
- **No heap allocation**: All operations use compile-time known sizes
- **Predictable memory usage**: `Uint<N>` uses exactly `N * 8` bytes

### Arithmetic Algorithms
- **Addition**: School-book addition with carry propagation - O(n)
- **Multiplication**: School-book multiplication - O(n²)
- **Modular operations**: Efficient bit-masked reductions

### Performance Characteristics
- **Constant-time operations**: No branching on secret data
- **Predictable performance**: Linear scaling for addition, quadratic for multiplication
- **Cache-friendly**: Contiguous memory layout

## 🔒 Security Considerations

This library is designed with cryptographic applications in mind:

- **Constant-time arithmetic**: Operations run in time dependent only on the bit size, not the values
- **No secret-dependent branching**: Eliminates timing side-channel vulnerabilities
- **Stack allocation**: Reduces memory-based side channels
- **Zero dependencies**: Minimizes attack surface

## 🌟 Use Cases

- **Cryptography**: RSA, elliptic curves, hash functions
- **Blockchain**: Big integer operations for cryptocurrencies
- **Scientific computing**: High-precision arithmetic
- **Protocol implementations**: Where predictable performance is critical

## 🔄 Mixed-Size Operations

The library supports operations between different-sized integers with the constraint that the left operand must have greater than or equal limbs than the right operand:

```rust
use bigint_rs::{U256, U512};

let big = U512::from_u64(1000);     // 512-bit
let small = U256::from_u64(2000);   // 256-bit

// ✅ This works: larger + smaller
let result = big.add(&small);       // Returns U512

// ❌ This panics: smaller + larger
// let result = small.add(&big);    // Panics!
```

## 📈 Performance Comparison

Typical performance on modern hardware (times are approximate):

| Operation | U64 | U128 | U256 | U512 | U1024 |
|-----------|-----|------|------|------|-------|
| Addition  | ~15ns | ~28ns | ~46ns | ~79ns | ~150ns |
| Multiplication | ~30ns | ~66ns | ~177ns | ~549ns | ~2.1μs |

## 🛠️ Development

### Building from Source

```bash
git clone https://github.com/daichihattori/nail.git
cd nail
cargo build --release
```

### Running CI Locally

```bash
# Install dependencies for GMP (rug crate)
sudo apt-get install -y libgmp-dev  # Ubuntu/Debian
# or
brew install gmp  # macOS

# Run tests and benchmarks
cargo test
cargo criterion
```

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📚 Documentation

- [API Documentation](https://docs.rs/nail)
- [Examples](./examples/)
- [Benchmark Results](./BENCHMARK_RESULTS.md)

## 🙏 Acknowledgments

- Inspired by various big integer implementations in the Rust ecosystem
- Performance comparisons with num-bigint, rug, and malachite
<!-- BENCHMARK_RESULTS_START -->
# Benchmark Results

*Generated automatically from criterion benchmark results*

## Addition Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 14 ns | 45 ns | 23 ns | - | - |
| **nail** | 1 ns | 2 ns | 3 ns | 7 ns | 19 ns |
| **num-bigint** | 46 ns | 46 ns | 23 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Addition Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (7 ns)
- **1024-bit**: Fastest is **nail** (19 ns)

## Multiplication Performance

| Library | 64-bit | 128-bit | 256-bit | 512-bit | 1024-bit |
|---------|---------|---------|---------|---------|---------|
| **malachite** | 15 ns | 38 ns | 51 ns | - | - |
| **nail** | 1 ns | 2 ns | 6 ns | 38 ns | 191 ns |
| **num-bigint** | 46 ns | 36 ns | 52 ns | - | - |
| **rug-gmp** | 2 ns | 2 ns | 2 ns | - | - |

### Multiplication Performance Summary

- **64-bit**: Fastest is **nail** (1 ns)
- **128-bit**: Fastest is **rug-gmp** (2 ns)
- **256-bit**: Fastest is **rug-gmp** (2 ns)
- **512-bit**: Fastest is **nail** (38 ns)
- **1024-bit**: Fastest is **nail** (191 ns)


<!-- BENCHMARK_RESULTS_END -->
