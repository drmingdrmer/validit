//! Validate variable internal state when the variable is accessed.
//!
//! - Implement trait `Validate` for a type `T` to define how to validate internal state of `T`.
//! - Wrapper struct `Valid<T: Validate>` implements `Deref` and `DerefMut` traits, and validates
//!   the internal state when the variable is accessed.
//!
//! For example, If in your program you have a struct `Foo { a: u64 }` and you want to make sure
//! that `a` is always less than to `10`, you can implement `Validate` trait for `Foo` and use
//! `less!` macro to validate `a`.
//! ```
//! # use std::error::Error;
//! # use validit::Valid;
//! # use validit::Validate;
//! struct Foo { a: u64 }
//!
//! impl Validate for Foo {
//!     fn validate(&self) -> Result<(), Box<dyn Error>> {
//!         validit::less!(self.a, 10);
//!         Ok(())
//!     }
//! }
//!
//! # fn main() {
//! let foo = Valid::new(Foo { a: 1 });
//! let _x = foo.a; // Good
//!
//! let invalid = Valid::new(Foo { a: 10 });
//! let _x = foo.a; // Panic
//! # }
//! ```

#[cfg(feature = "macros")]
pub mod macros;

mod valid;
#[cfg(test)]
mod valid_test;
mod validate;
mod validate_impl;

pub use valid::Valid;
pub use validate::Validate;
