//! Defines macros for validation check, such as `less!(smaller, greater)`.

use std::fmt::Arguments;

pub fn make_err(fmt: Arguments) -> anyerror::AnyError {
    anyerror::AnyError::error(format!("{fmt}"))
}

/// Assert that function call `call(a,b,...)`(up to 8 arguments) to return true, otherwise it return
/// an error.
///
/// For example:
/// ```
/// # use std::error::Error;
/// # use validit::be_true;
/// fn expect_true(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     fn le(a: u64, b: u64) -> bool { a <= b }
///     be_true!(le(a,b));
///     Ok(())
/// }
/// assert!(expect_true(1,2).is_ok());
/// assert!(expect_true(3,2).unwrap_err().to_string().starts_with("expect to be true: le(a(3), b(2)) at "));
/// ```
///
/// Another example with 3 arguments:
/// ```
/// # use std::error::Error;
/// # use validit::be_true;
/// fn expect_true3(a: u64, b:u64) -> Result<(), Box<dyn Error + 'static>> {
///     fn mid(l: u64, x: u64, r: u64) -> bool { l <= x && x <= r }
///     be_true!(mid(a,5,b));
///     Ok(())
/// }
/// assert!(expect_true3(1,10).is_ok());
/// assert!(expect_true3(6,10).unwrap_err().to_string().starts_with("expect to be true: mid(a(6), 5(5), b(10)) at "));
/// ```
#[macro_export]
macro_rules! be_true {
    // 0 args
    ($($call: ident).+()) => {{
        let __result = $($call).+();
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}() at {}:{}",
                stringify!($($call).+),
                file!(), line!(),
            )))?;
        }
    }};

    // 1 args
    ($($call: ident).+($a: expr)) => {{
        let __a = $a;
        let __result = $($call).+(__a);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                file!(), line!(),
            )))?;
        }
    }};

    // 2 args
    ($($call: ident).+($a: expr, $b: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __result = $($call).+(__a, __b);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                file!(), line!(),
            )))?;
        }
    }};

    // 3 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __result = $($call).+(__a, __b, __c);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                file!(), line!(),
            )))?;
        }
    }};

    // 4 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr, $d: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __d = $d;
        let __result = $($call).+(__a, __b, __c, __d);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                stringify!($d),
                __d,
                file!(), line!(),
            )))?;
        }
    }};

    // 5 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr, $d: expr, $e: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __d = $d;
        let __e = $e;
        let __result = $($call).+(__a, __b, __c, __d, __e);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                stringify!($d),
                __d,
                stringify!($e),
                __e,
                file!(), line!(),
            )))?;
        }
    }};

    // 6 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr, $d: expr, $e: expr, $f: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __d = $d;
        let __e = $e;
        let __f = $f;
        let __result = $($call).+(__a, __b, __c, __d, __e, __f);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                stringify!($d),
                __d,
                stringify!($e),
                __e,
                stringify!($f),
                __f,
                file!(), line!(),
            )))?;
        }
    }};

    // 7 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr, $d: expr, $e: expr, $f: expr, $g: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __d = $d;
        let __e = $e;
        let __f = $f;
        let __g = $g;
        let __result = $($call).+(__a, __b, __c, __d, __e, __f, __g);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                stringify!($d),
                __d,
                stringify!($e),
                __e,
                stringify!($f),
                __f,
                stringify!($g),
                __g,
                file!(), line!(),
            )))?;
        }
    }};

    // 8 args
    ($($call: ident).+($a: expr, $b: expr, $c: expr, $d: expr, $e: expr, $f: expr, $g: expr, $h: expr)) => {{
        let __a = $a;
        let __b = $b;
        let __c = $c;
        let __d = $d;
        let __e = $e;
        let __f = $f;
        let __g = $g;
        let __h = $h;
        let __result = $($call).+(__a, __b, __c, __d, __e, __f, __g, __h);
        if __result {
            // Ok
        } else {
            Err($crate::macros::make_err(format_args!(
                "expect to be true: {}({}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?}), {}({:?})) at {}:{}",
                stringify!($($call).+),
                stringify!($a),
                __a,
                stringify!($b),
                __b,
                stringify!($c),
                __c,
                stringify!($d),
                __d,
                stringify!($e),
                __e,
                stringify!($f),
                __f,
                stringify!($g),
                __g,
                stringify!($h),
                __h,
                file!(), line!(),
            )))?;
        }
    }};
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
/// assert!(expect_less(2,2).unwrap_err().to_string().starts_with("expect: a(2) < b(2) at "));
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
                "expect: {}({:?}) < {}({:?}) at {}:{}",
                stringify!($a),
                a,
                stringify!($b),
                b,
                file!(),
                line!(),
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
/// assert!(expect_greater(2,2).unwrap_err().to_string().starts_with("expect: a(2) > b(2) at "));
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
                "expect: {}({:?}) > {}({:?}) at {}:{}",
                stringify!($a),
                a,
                stringify!($b),
                b,
                file!(),
                line!(),
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
/// assert!(expect_less_equal(3,2).unwrap_err().to_string().starts_with("expect: a(3) <= b(2) at "));
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
                "expect: {}({:?}) <= {}({:?}) at {}:{}",
                stringify!($a),
                a,
                stringify!($b),
                b,
                file!(),
                line!(),
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
/// assert!(expect_greater_equal(2,3).unwrap_err().to_string().starts_with("expect: a(2) >= b(3) at "));
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
                "expect: {}({:?}) >= {}({:?}) at {}:{}",
                stringify!($a),
                a,
                stringify!($b),
                b,
                file!(),
                line!(),
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
/// assert!(expect_equal(3,2).unwrap_err().to_string().starts_with("expect: a(3) == b(2) at "));
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
                "expect: {}({:?}) == {}({:?}) at {}:{}",
                stringify!($a),
                a,
                stringify!($b),
                b,
                file!(),
                line!(),
            )))?;
        }
    }};
}
