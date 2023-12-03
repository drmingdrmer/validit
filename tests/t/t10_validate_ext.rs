use std::error::Error;
use std::ops::Deref;

use validit::Validate;
use validit::ValidateExt;

struct LessThan5(u64);

impl Validate for LessThan5 {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        validit::less!(self.0, 5);
        Ok(())
    }
}

#[test]
fn test_validate_ext() {
    let _a = LessThan5(1).0; // Good
    let _a = LessThan5(1).valid().deref(); // Good

    let res = std::panic::catch_unwind(|| {
        let _a = LessThan5(10).valid().deref();
    });

    assert!(res.is_err());
}
