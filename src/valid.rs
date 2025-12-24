use core::panic;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::Validate;

/// A wrapper of T that validate the state of T every time accessing it.
///
/// - It validates the state before accessing it, i.e., if when a invalid state is written to it, it
///   won't panic until next time accessing it.
/// - The validation is turned on only when `debug_assertions` is enabled.
///
/// An example of defining field `a` whose value must not exceed `10`.
/// ```ignore
/// # use std::error::Error;
/// # use openraft::less_equal;
/// struct Foo { a: u64 }
/// impl Validate for Foo {
///     fn validate(&self) -> Result<(), Box<dyn Error>> {
///         less_equal!(self.a, 10);
///         Ok(())
///     }
/// }
///
/// let f = Valid::new(Foo { a: 20 });
/// let _x = f.a; // panic: panicked at 'invalid state: expect: self.a(20) <= 10(10) ...
/// ```
///
/// Validation is triggered when:
/// - `Deref` or `DerefMut` is called. This is the major use case.
/// - `Clone` is called.
/// - `PartialEq` or `Eq` is called.
/// - `PartialOrd` or `Ord` is called.
/// - `Hash` is called.
///
/// Validation is not triggered when:
/// - `Copy`: Because it is just a byte copy.
/// - `Debug` and `Display`: for being able to examine the value for debugging.
pub struct Valid<T>
where T: Validate
{
    pub(crate) enabled: bool,
    inner: T,
}

impl<T: Validate> Valid<T> {
    /// Create a new `Valid<T>`.
    pub fn new(inner: T) -> Self {
        Self {
            enabled: true,
            inner,
        }
    }

    /// Set whether to validate the state when accessing it.
    pub fn enable_validation(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get whether to validate the state when accessing it.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Return a reference to the wrapped value: `Valid<&T>`.
    pub fn as_ref(&self) -> Valid<&T> {
        Valid {
            enabled: self.enabled,
            inner: self.deref(),
        }
    }

    /// Consume self and return the wrapped value.
    ///
    /// This does NOT validate the state.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Valid<T>
where T: Validate
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        if self.enabled
            && let Err(e) = self.inner.validate()
        {
            panic!("invalid state: {}", e);
        }

        &self.inner
    }
}

impl<T> DerefMut for Valid<T>
where T: Validate
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(debug_assertions)]
        if self.enabled
            && let Err(e) = self.inner.validate()
        {
            panic!("invalid state: {}", e);
        }

        &mut self.inner
    }
}

impl<T: PartialEq> PartialEq for Valid<T>
where T: Validate
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.deref(), other.deref())
    }
}

impl<T: Eq> Eq for Valid<T> where T: Validate {}

impl<T: PartialOrd> PartialOrd for Valid<T>
where T: Validate
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.deref(), other.deref())
    }
}

impl<T: Ord> Ord for Valid<T>
where T: Validate
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self.deref(), other.deref())
    }
}

impl<T: Debug> Debug for Valid<T>
where T: Validate
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Do not use `deref()`, skip validation so that the state can be displayed.
        self.inner.fmt(f)
    }
}

impl<T: Display> Display for Valid<T>
where T: Validate
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Do not use `deref()`, skip validation so that the state can be displayed.
        self.inner.fmt(f)
    }
}

impl<T: Clone> Clone for Valid<T>
where T: Validate
{
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            inner: self.deref().clone(),
        }
    }
}

impl<T: Copy> Copy for Valid<T> where T: Validate {}

impl<T: Default> Default for Valid<T>
where T: Validate
{
    fn default() -> Self {
        Self {
            enabled: true,
            inner: T::default(),
        }
    }
}

impl<T: Hash> Hash for Valid<T>
where T: Validate
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}
