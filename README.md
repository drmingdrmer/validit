# validit


<a href="https://discord.gg/fFPsTqYqUg">
<img src="https://img.shields.io/badge/discord-validit-0abd59" alt="discord" />
</a>

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
