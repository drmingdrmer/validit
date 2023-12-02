//! Defines macros for validation check, such as `less!(smaller, greater)`.

use std::fmt::Arguments;

pub fn make_err(fmt: Arguments) -> anyerror::AnyError {
    anyerror::AnyError::error(format!("{}", fmt))
}

/// Assert that `a` is less than `b`, otherwise it return an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::less;
/// fn expect_less(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     less!(a,b);
///     Ok(())
/// }
/// assert!(expect_less(1,2).is_ok());
/// assert_eq!("expect: a(2) < b(2)", expect_less(2,2).unwrap_err().to_string());
/// ```
#[macro_export]
macro_rules! less {
    ($a: expr, $b: expr) => {{
        let a = $a;
        let b = $b;
        if (a < b) {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect: {}({:?}) {} {}({:?})",
                stringify!($a),
                a,
                "<",
                stringify!($b),
                b,
            )))?;
        }
    }};
}

/// Assert that `a` is greater than `b`, otherwise it return an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::greater;
/// fn expect_greater(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     greater!(a,b);
///     Ok(())
/// }
/// assert!(expect_greater(2,1).is_ok());
/// assert_eq!("expect: a(2) > b(2)", expect_greater(2,2).unwrap_err().to_string());
/// ```
#[macro_export]
macro_rules! greater {
    ($a: expr, $b: expr) => {{
        let a = $a;
        let b = $b;
        if (a > b) {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect: {}({:?}) {} {}({:?})",
                stringify!($a),
                a,
                ">",
                stringify!($b),
                b,
            )))?;
        }
    }};
}

/// Assert that `a` is less than or equal to `b`, otherwise it return an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::less_equal;
/// fn expect_less_equal(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     less_equal!(a,b);
///     Ok(())
/// }
/// assert!(expect_less_equal(2,2).is_ok());
/// assert_eq!("expect: a(3) <= b(2)", expect_less_equal(3,2).unwrap_err().to_string());
/// ```
#[macro_export]
macro_rules! less_equal {
    ($a: expr, $b: expr) => {{
        let a = $a;
        let b = $b;
        if (a <= b) {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect: {}({:?}) {} {}({:?})",
                stringify!($a),
                a,
                "<=",
                stringify!($b),
                b,
            )))?;
        }
    }};
}

/// Assert that `a` is greater than or equal to `b`, otherwise it return an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::greater_equal;
/// fn expect_greater_equal(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     greater_equal!(a,b);
///     Ok(())
/// }
/// assert!(expect_greater_equal(2,2).is_ok());
/// assert_eq!("expect: a(2) >= b(3)", expect_greater_equal(2,3).unwrap_err().to_string());
/// ```
#[macro_export]
macro_rules! greater_equal {
    ($a: expr, $b: expr) => {{
        let a = $a;
        let b = $b;
        if (a >= b) {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect: {}({:?}) {} {}({:?})",
                stringify!($a),
                a,
                ">=",
                stringify!($b),
                b,
            )))?;
        }
    }};
}

/// Assert that `a` equal to `b`, otherwise it return an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::equal;
/// fn expect_equal(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     equal!(a,b);
///     Ok(())
/// }
/// assert!(expect_equal(2,2).is_ok());
/// assert_eq!("expect: a(3) == b(2)", expect_equal(3,2).unwrap_err().to_string());
/// ```
#[macro_export]
macro_rules! equal {
    ($a: expr, $b: expr) => {{
        let a = $a;
        let b = $b;
        if (a == b) {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect: {}({:?}) {} {}({:?})",
                stringify!($a),
                a,
                "==",
                stringify!($b),
                b,
            )))?;
        }
    }};
}
