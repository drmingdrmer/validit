use std::error::Error;

/// Defines how to validate variable internal state.
///
/// See [crate level documentation](crate) for more details.
pub trait Validate {
    /// Validate the internal state.
    fn validate(&self) -> Result<(), Box<dyn Error>>;
}
