# validit


[![Discord Chat](https://img.shields.io/discord/1180545690976391251?logo=discord&style=flat-square)](https://discord.gg/fFPsTqYqUg)
[![Crates.io](https://img.shields.io/crates/v/validit.svg)](https://crates.io/crates/validit)
[![docs.rs](https://docs.rs/validit/badge.svg)](https://docs.rs/validit)
![Crates.io](https://img.shields.io/crates/d/validit.svg)
![Crates.io](https://img.shields.io/crates/dv/validit.svg)

Validate variable internal state when the variable is accessed.

- Implement trait `Validate` for a type `T` to define how to validate internal state of `T`.
- Wrapper struct `Valid<T: Validate>` implements `Deref` and `DerefMut` traits, and validates
  the internal state when the variable is accessed.

For example, If in your program you have a struct `Foo { a: u64 }` and you want to make sure
that `a` is always less than to `10`, you can implement `Validate` trait for `Foo` and use
`less!` macro to validate `a`.

```rust
use std::error::Error;
use validit::Validate;
use validit::ValidateExt;

struct LessThan5 { v: u64 }

impl Validate for LessThan5 {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        validit::less!(self.v, 5);
        Ok(())
    }
}

fn main() {
  let v1 = LessThan5 { v: 1 }.valid();
  let _x = v1.v; // Good

  let v6 = LessThan5 { v: 6 }.valid();
  let res = std::panic::catch_unwind(|| {
      let _x = v6.v; // panic: panicked at 'invalid state: expect: self.v(6) < 5(5) ...
  });
  assert!(res.is_err());
}
```

## Contribution

- 🙌 Questions? Join the [Discord channel](https://discord.gg/fFPsTqYqUg) or start a [discussion](https://github.com/drmingdrmer/validit/discussions/new).
