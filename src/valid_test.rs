use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::panic::UnwindSafe;

use crate::less_equal;
use crate::valid::Valid;
use crate::validate::Validate;
use crate::ValidateExt;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Foo {
    le_10: u64,
}

impl Foo {
    fn new_valid(le_10: u64) -> Valid<Self> {
        Self { le_10 }.valid()
    }
}

impl Validate for Foo {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        less_equal!(self.le_10, 10);
        Ok(())
    }
}

impl Display for Foo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo:{}", self.le_10)
    }
}

#[test]
#[allow(clippy::redundant_clone)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::bool_assert_comparison)]
fn test_valid_derived_trait() {
    let a = Valid::new(Foo { le_10: 3 });
    let b = Valid::new(Foo { le_10: 4 });

    // Debug
    fn assert_debug<T: Debug>(_t: &T) {}
    assert_debug(&a);

    // Display
    println!("Display: {}", Valid::new(Foo { le_10: 3 }));

    // Default
    assert_eq!(
        Valid::new(Foo { le_10: 0 }),
        Valid::<Foo>::default(),
        "impl Default"
    );

    // Clone
    let _clone = Valid::new(Foo { le_10: 3 }).clone();

    // Copy
    let c = &a;
    let d = *c;
    assert_eq!(a, d);

    // PartialEq
    assert_eq!(false, PartialEq::eq(&a, &b));

    // Eq
    fn assert_eq<T: Eq>(_t: &T) {}
    assert_eq(&a);

    // PartialOrd
    assert_eq!(Some(Ordering::Less), PartialOrd::partial_cmp(&a, &b));

    // Ord
    assert_eq!(Ordering::Less, Ord::cmp(&a, &b));

    // Hash
    fn assert_hash<T: std::hash::Hash>(_t: &T) {}
    assert_hash(&a);
}

#[test]
#[allow(clippy::redundant_clone)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::bool_assert_comparison)]
#[allow(clippy::deref_addrof)]
fn test_valid_trigger_validation() {
    fn assert_panic<F: FnOnce() + UnwindSafe>(func: F) {
        let res = std::panic::catch_unwind(func);
        assert!(res.is_err());
    }

    fn assert_no_panic<F: FnOnce() + UnwindSafe>(func: F) {
        let res = std::panic::catch_unwind(func);
        assert!(res.is_ok());
    }

    // Debug does not trigger panic
    assert_no_panic(|| {
        format!("{:?}", Foo::new_valid(20));
    });

    // Display does not trigger panic
    assert_no_panic(|| {
        format!("{}", Foo::new_valid(20));
    });

    // Clone trigger panic
    assert_panic(|| {
        let _clone = Foo::new_valid(20).clone();
    });

    // Copy does not trigger panic
    assert_no_panic(|| {
        let _clone = *&Foo::new_valid(20);
    });

    // PartialEq trigger panic
    assert_panic(|| {
        let _ = PartialEq::eq(&Foo::new_valid(20), &Foo::new_valid(1));
    });
    assert_panic(|| {
        let _ = PartialEq::eq(&Foo::new_valid(1), &Foo::new_valid(20));
    });

    // PartialOrd trigger panic
    assert_panic(|| {
        let _ = PartialOrd::partial_cmp(&Foo::new_valid(20), &Foo::new_valid(1));
    });
    assert_panic(|| {
        let _ = PartialOrd::partial_cmp(&Foo::new_valid(1), &Foo::new_valid(20));
    });

    // Ord trigger panic
    assert_panic(|| {
        let _ = Ord::cmp(&Foo::new_valid(20), &Foo::new_valid(1));
    });
    assert_panic(|| {
        let _ = Ord::cmp(&Foo::new_valid(1), &Foo::new_valid(20));
    });

    // Hash trigger panic
    assert_panic(|| {
        let mut map = HashSet::new();
        map.insert(Foo::new_valid(20));
    });

    // ---

    // `as_ref()` trigger panic
    assert_panic(|| {
        let f = Foo::new_valid(20);
        let _r = f.as_ref();
    });
}

#[test]
fn test_valid_deref() {
    // panic when reading an invalid state
    let res = std::panic::catch_unwind(|| {
        let f = Valid::new(Foo { le_10: 20 });
        let _x = f.le_10;
    });
    assert!(res.is_err());

    // Disable validation
    let res = std::panic::catch_unwind(|| {
        let mut f = Valid::new(Foo { le_10: 20 });
        f.enabled = false;
        let _x = f.le_10;
    });
    assert!(res.is_ok());

    // valid state
    let res = std::panic::catch_unwind(|| {
        let f = Valid::new(Foo { le_10: 10 });
        let _x = f.le_10;
    });
    assert!(res.is_ok());

    // no panic when just becoming invalid
    let res = std::panic::catch_unwind(|| {
        let mut f = Valid::new(Foo { le_10: 10 });
        f.le_10 += 3;
    });
    assert!(res.is_ok());

    // panic on next write access
    let res = std::panic::catch_unwind(|| {
        let mut f = Valid::new(Foo { le_10: 10 });
        f.le_10 += 3;
        f.le_10 += 1;
    });
    assert!(res.is_err());
}

#[test]
fn test_valid_impl_for_ref() {
    // Valid
    let res = std::panic::catch_unwind(|| {
        let f = Foo { le_10: 5 };
        let f = Valid::new(&f);
        let _x = f.le_10;
    });
    assert!(res.is_ok());

    // Invalid
    let res = std::panic::catch_unwind(|| {
        let f = Foo { le_10: 20 };
        let f = Valid::new(&f);
        let _x = f.le_10;
    });
    assert!(res.is_err());
}

#[test]
fn test_valid_as_ref() {
    // Invalid
    let res = std::panic::catch_unwind(|| {
        let f = Foo { le_10: 20 };
        let f = Valid::new(f);
        let ref_f = f.as_ref();
        let _x = ref_f.le_10;
    });
    assert!(res.is_err());

    // Check `enabled` is inherited
    let mut f = Foo::new_valid(10);

    let r = f.as_ref();
    assert!(r.enabled);
    assert!(r.is_enabled());

    f.enable_validation(false);
    let r = f.as_ref();
    assert!(!r.enabled);
    assert!(!r.is_enabled());
}

#[test]
fn test_set_validation() {
    let mut f = Foo::new_valid(10);
    assert!(f.enabled);

    f.enable_validation(false);
    assert!(!f.enabled);

    f.enable_validation(true);
    assert!(f.enabled);
}

#[test]
fn test_valid_into_inner() {
    let f = Foo { le_10: 20 };
    let f = Valid::new(f);
    let got = f.into_inner();
    assert_eq!(Foo { le_10: 20 }, got);
}
