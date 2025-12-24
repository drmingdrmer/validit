use anyerror::AnyError;

use crate::be_true;

/// Test `be_true!()` macro accepts from 1 to 8 arguments function call.
#[test]
#[allow(clippy::too_many_arguments)]
fn test_macro_be_true() {
    fn arg0_true() -> bool {
        true
    }

    fn arg0_false() -> bool {
        false
    }

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

    assert!(
        (|| {
            be_true!(arg0_true());
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg1(true));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg2(true, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg3(true, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg4(true, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg5(true, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg6(true, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg7(true, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );
    assert!(
        (|| {
            be_true!(arg8(true, 1, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .is_ok()
    );

    assert!(
        (|| {
            be_true!(arg0_false());
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg0_false() at ")
    );

    assert!(
        (|| {
            be_true!(arg1(false));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg1(false(false)) at ")
    );

    assert!(
        (|| {
            be_true!(arg2(false, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg2(false(false), 1(1)) at ")
    );

    assert!(
        (|| {
            be_true!(arg3(false, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg3(false(false), 1(1), 1(1)) at ")
    );

    assert!(
        (|| {
            be_true!(arg4(false, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg4(false(false), 1(1), 1(1), 1(1)) at ")
    );

    assert!(
        (|| {
            be_true!(arg5(false, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg5(false(false), 1(1), 1(1), 1(1), 1(1)) at ")
    );

    assert!(
        (|| {
            be_true!(arg6(false, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with("expect to be true: arg6(false(false), 1(1), 1(1), 1(1), 1(1), 1(1)) at ")
    );

    assert!(
        (|| {
            be_true!(arg7(false, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with(
            "expect to be true: arg7(false(false), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1)) at "
        )
    );

    assert!(
        (|| {
            be_true!(arg8(false, 1, 1, 1, 1, 1, 1, 1));
            Ok::<(), AnyError>(())
        })()
        .unwrap_err()
        .to_string()
        .starts_with(
            "expect to be true: arg8(false(false), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1), 1(1)) at "
        )
    );
}

#[test]
#[allow(clippy::too_many_arguments)]
fn test_call_method() {
    struct Bar {
        foo: Foo,
    }

    struct Foo {
        _v: u64,
    }

    impl Foo {
        fn get0(&self) -> bool {
            true
        }
        fn get1(&self, _v1: u64) -> bool {
            true
        }
        fn get2(&self, _v1: u64, _v2: u64) -> bool {
            true
        }
        fn get3(&self, _v1: u64, _v2: u64, _v3: u64) -> bool {
            true
        }
        fn get4(&self, _v1: u64, _v2: u64, _v3: u64, _v4: u64) -> bool {
            true
        }
        fn get5(&self, _v1: u64, _v2: u64, _v3: u64, _v4: u64, _v5: u64) -> bool {
            true
        }
        fn get6(&self, _v1: u64, _v2: u64, _v3: u64, _v4: u64, _v5: u64, _v6: u64) -> bool {
            true
        }
        fn get7(
            &self,
            _v1: u64,
            _v2: u64,
            _v3: u64,
            _v4: u64,
            _v5: u64,
            _v6: u64,
            _v7: u64,
        ) -> bool {
            true
        }
        fn get8(
            &self,
            _v1: u64,
            _v2: u64,
            _v3: u64,
            _v4: u64,
            _v5: u64,
            _v6: u64,
            _v7: u64,
            _v8: u64,
        ) -> bool {
            true
        }
    }

    let foo = Foo { _v: 5 };
    let bar = Bar { foo: Foo { _v: 5 } };

    let _got = (|| {
        be_true!(foo.get0());
        be_true!(foo.get1(1));
        be_true!(foo.get2(1, 2));
        be_true!(foo.get3(1, 2, 3));
        be_true!(foo.get4(1, 2, 3, 4));
        be_true!(foo.get5(1, 2, 3, 4, 5));
        be_true!(foo.get6(1, 2, 3, 4, 5, 6));
        be_true!(foo.get7(1, 2, 3, 4, 5, 6, 7));
        be_true!(foo.get8(1, 2, 3, 4, 5, 6, 7, 8));
        be_true!(bar.foo.get0());
        be_true!(bar.foo.get1(1));
        be_true!(bar.foo.get2(1, 2));
        be_true!(bar.foo.get3(1, 2, 3));
        be_true!(bar.foo.get4(1, 2, 3, 4));
        be_true!(bar.foo.get5(1, 2, 3, 4, 5));
        be_true!(bar.foo.get6(1, 2, 3, 4, 5, 6));
        be_true!(bar.foo.get7(1, 2, 3, 4, 5, 6, 7));
        be_true!(bar.foo.get8(1, 2, 3, 4, 5, 6, 7, 8));
        Ok::<(), AnyError>(())
    })();
}
