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
//! # use std::panic::catch_unwind;
//! # use validit::Validate;
//! # use validit::ValidateExt;
//! struct Lt5 { v: u64 }
//!
//! impl Validate for Lt5 {
//!     fn validate(&self) -> Result<(), Box<dyn Error>> {
//!         validit::less!(self.v, 5);
//!         Ok(())
//!     }
//! }
//!
//! # fn main() {
//! let v1 = Lt5 { v: 1 }.valid();
//! let _x = v1.v; // Good
//!
//! let v6 = Lt5 { v: 6 }.valid();
//! let res = catch_unwind(|| {
//!     let _x = v6.v; // panic: panicked at 'invalid state: expect: self.v(6) < 5(5) ...
//! });
//! assert!(res.is_err());
//! # }
//! ```

#[cfg(feature = "macros")]
pub mod macros;

mod valid;
#[cfg(test)]
mod valid_test;
mod validate;
mod validate_ext;
mod validate_impl;

pub use valid::Valid;
pub use validate::Validate;
pub use validate_ext::ValidateExt;
