extern crate test;
use std::error::Error;

use test::Bencher;
use test::black_box;

use crate::Valid;
use crate::Validate;
use crate::less_equal;

struct Foo {
    a: u64,
}

impl Validate for Foo {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        less_equal!(self.a, 10);
        Ok(())
    }
}

#[bench]
fn valid_deref(b: &mut Bencher) {
    let f = Valid::new(Foo { a: 5 });

    b.iter(|| {
        let _x = black_box(f.a);
    })
}
