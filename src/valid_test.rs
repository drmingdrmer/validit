use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::less_equal;
use crate::valid::Valid;
use crate::validate::Validate;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Foo {
    le_10: u64,
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

    let a = Valid::new(Foo { le_10: 3 });
    let b = Valid::new(Foo { le_10: 4 });

    // PartialEq
    assert_eq!(false, PartialEq::eq(&a, &b));

    // Eq
    fn assert_eq<T: Eq>(_t: &T) {}
    assert_eq(&a);

    // PartialOrd
    assert_eq!(Some(Ordering::Less), PartialOrd::partial_cmp(&a, &b));

    // Ord
    assert_eq!(Ordering::Less, Ord::cmp(&a, &b));

    // Copy
    let c = &a;
    let d = *c;
    assert_eq!(a, d);

    // Debug
    fn assert_debug<T: Debug>(_t: &T) {}
    assert_debug(&a);

    // Hash
    fn assert_hash<T: std::hash::Hash>(_t: &T) {}
    assert_hash(&a);
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
}

#[test]
fn test_valid_into_inner() {
    let f = Foo { le_10: 20 };
    let f = Valid::new(f);
    let got = f.into_inner();
    assert_eq!(Foo { le_10: 20 }, got);
}
