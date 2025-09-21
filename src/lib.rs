//! # Nail
//! 
//! A high-performance fixed-size big integer library for Rust with configurable limb count.
//! 
//! This library provides compile-time sized unsigned integers with efficient arithmetic operations.
//! Unlike dynamic big integer libraries, `nail` uses fixed-size arrays for predictable
//! memory usage and optimal performance.
//! 
//! ## Features
//! 
//! - **Compile-time sizing**: `Uint<N>` where N is the number of 64-bit limbs
//! - **Zero-allocation arithmetic**: All operations use stack-allocated arrays
//! - **Mixed-size operations**: Add/multiply integers with different limb counts
//! - **Modular arithmetic**: Built-in support for modular operations
//! - **Extensive testing**: Comprehensive test suite including edge cases
//! 
//! ## Examples
//! 
//! ```rust
//! use nail::Uint;
//! 
//! // Create 256-bit integers (4 * 64 = 256 bits)
//! let a = Uint::<4>::from_u64(0xFFFFFFFFFFFFFFFF);
//! let b = Uint::<4>::from_u64(0x1000);
//! 
//! // Addition
//! let sum = a.add(&b);
//! 
//! // Multiplication
//! let product = a.mul(&b);
//! 
//! // Mixed-size operations (larger on left)
//! let large = Uint::<8>::from_u64(1000);  // 512-bit
//! let small = Uint::<2>::from_u64(2000);  // 128-bit
//! let result = large.add(&small);         // Returns 512-bit result
//! ```
//! 
//! ## Performance
//! 
//! This library is designed for high-performance applications where predictable
//! memory usage and minimal allocation overhead are critical. See the benchmark
//! results in the repository for detailed performance comparisons.

pub mod uint;

pub use uint::Uint;

/// Type alias for 64-bit unsigned integer (1 limb)
pub type U64 = Uint<1>;

/// Type alias for 128-bit unsigned integer (2 limbs)
pub type U128 = Uint<2>;

/// Type alias for 256-bit unsigned integer (4 limbs)
pub type U256 = Uint<4>;

/// Type alias for 512-bit unsigned integer (8 limbs)
pub type U512 = Uint<8>;

/// Type alias for 1024-bit unsigned integer (16 limbs)
pub type U1024 = Uint<16>;