//! Extension trait to ease the use of `Validate` trait.

use crate::Valid;
use crate::Validate;

pub trait ValidateExt: Validate {
    /// Return an always valid instance `Valid<Self>` that enables internal state validation.
    fn valid(self) -> Valid<Self>
    where Self: Sized;
}

impl<T: Validate> ValidateExt for T {
    fn valid(self) -> Valid<Self>
    where Self: Sized {
        Valid::new(self)
    }
}
