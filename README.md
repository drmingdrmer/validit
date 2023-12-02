# validit


[![Discord Chat](https://img.shields.io/discord/1180545690976391251?logo=discord&style=flat-square)](https://discord.gg/fFPsTqYqUg)
[![Crates.io](https://img.shields.io/crates/v/validit.svg)](https://crates.io/crates/validit)
[![docs.rs](https://docs.rs/validit/badge.svg)](https://docs.rs/validit)

Validate variable internal state when the variable is accessed.

- Implement trait `Validate` for a type `T` to define how to validate internal state of `T`.
- Wrapper struct `Valid<T: Validate>` implements `Deref` and `DerefMut` traits, and validates
  the internal state when the variable is accessed.

For example, If in your program you have a struct `Foo { a: u64 }` and you want to make sure
that `a` is always less than to `10`, you can implement `Validate` trait for `Foo` and use
`less!` macro to validate `a`.

```rust
use std::error::Error;
use validit::Valid;
use validit::Validate;

struct Foo { a: u64 }

impl Validate for Foo {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        validit::less!(self.a, 10);
        Ok(())
    }
}

fn main() {
    let foo = Valid::new(Foo { a: 1 });
    let _x = foo.a; // Good

    let invalid = Valid::new(Foo { a: 10 });
    let _x = foo.a; // Panic
}
```

## Contribution

- 🙌 Questions? Join the [Discord channel](https://discord.gg/fFPsTqYqUg) or start a [discussion](https://github.com/drmingdrmer/validit/discussions/new).
