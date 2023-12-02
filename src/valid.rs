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

    /// Return a reference to the wrapped value: `Valid<&T>`.
    pub fn as_ref(&self) -> Valid<&T> {
        Valid {
            enabled: self.enabled,
            inner: &self.inner,
        }
    }

    /// Consume self and return the wrapped value.
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
        if self.enabled {
            if let Err(e) = self.inner.validate() {
                panic!("invalid state: {}", e);
            }
        }

        &self.inner
    }
}

impl<T> DerefMut for Valid<T>
where T: Validate
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[cfg(debug_assertions)]
        if self.enabled {
            if let Err(e) = self.inner.validate() {
                panic!("invalid state: {}", e);
            }
        }

        &mut self.inner
    }
}

impl<T: PartialEq> PartialEq for Valid<T>
where T: Validate
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Eq> Eq for Valid<T> where T: Validate {}

impl<T: PartialOrd> PartialOrd for Valid<T>
where T: Validate
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.inner, &other.inner)
    }
}

impl<T: Ord> Ord for Valid<T>
where T: Validate
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.inner, &other.inner)
    }
}

impl<T: Debug> Debug for Valid<T>
where T: Validate
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Display> Display for Valid<T>
where T: Validate
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Clone> Clone for Valid<T>
where T: Validate
{
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            inner: self.inner.clone(),
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
        self.inner.hash(state)
    }
}
