use anyerror::AnyError;

use crate::be_true;

/// Test `be_true!()` macro accepts from 1 to 8 arguments function call.
#[test]
#[allow(clippy::too_many_arguments)]
fn test_macro_be_true() {
    fn arg1(r: bool) -> bool {
        r
    }

    fn arg2(r: bool, _a: u64) -> bool {
        r
    }

    fn arg3(r: bool, _a: u64, _b: u64) -> bool {
        r
    }

    fn arg4(r: bool, _a: u64, _b: u64, _c: u64) -> bool {
        r
    }

    fn arg5(r: bool, _a: u64, _b: u64, _c: u64, _d: u64) -> bool {
        r
    }

    fn arg6(r: bool, _a: u64, _b: u64, _c: u64, _d: u64, _e: u64) -> bool {
        r
    }

    fn arg7(r: bool, _a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64) -> bool {
        r
    }

    fn arg8(r: bool, _a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64) -> bool {
        r
    }

    assert!((|| {
        be_true!(arg1(true));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg2(true, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg3(true, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg4(true, 1, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg5(true, 1, 1, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg6(true, 1, 1, 1, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg7(true, 1, 1, 1, 1, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());
    assert!((|| {
        be_true!(arg8(true, 1, 1, 1, 1, 1, 1, 1));
        Ok::<(), AnyError>(())
    })()
    .is_ok());

    assert_eq!(
        "expect to be true: arg1(false(false))",
        (|| {
            be_true!(arg1(false));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg2(false(false), 1(1))",
        (|| {
            be_true!(arg2(false, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg3(false(false), 1(1), 1(1))",
        (|| {
            be_true!(arg3(false, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg4(false(false), 1(1), 1(1), 1(1))",
        (|| {
            be_true!(arg4(false, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg5(false(false), 1(1), 1(1), 1(1), 1(1))",
        (|| {
            be_true!(arg5(false, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg6(false(false), 1(1), 1(1), 1(1), 1(1), 1(1))",
        (|| {
            be_true!(arg6(false, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg7(false(false), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1))",
        (|| {
            be_true!(arg7(false, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
    assert_eq!(
        "expect to be true: arg8(false(false), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1))",
        (|| {
            be_true!(arg8(false, 1, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
    );
}
