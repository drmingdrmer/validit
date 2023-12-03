use std::error::Error;

use validit::Valid;
use validit::Validate;

struct Foo(u64);

impl Validate for Foo {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        validit::less!(self.0, 10);
        Ok(())
    }
}

#[test]
fn test_valid() {
    let _a = Valid::new(Foo(1)).0;

    let res = std::panic::catch_unwind(|| {
        let _a = Valid::new(Foo(10)).0;
    });
    assert!(res.is_err());
}
