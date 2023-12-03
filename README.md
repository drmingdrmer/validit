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

For example, If in your program you have a struct `Foo(u64)` and you want to make sure
that `a` is always less than to `5`, you can implement `Validate` trait for `Foo` and use
`less!` macro to validate `a`.

```rust
struct Foo(u64);

impl validit::Validate for Foo {
  fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
    validit::less!(self.0, 5);
    Ok(())
  }
}

fn main() {
  let v1 = Valid::new(Foo(1));
  let _x = v1.0; // Good.

  let v6 = Foo(6);
  let _x = v6.0; // No panic without validation.
  
  let v6 = Valid::new(Foo(6));
  let _x = v6.0; // panic: panicked at 'invalid state: expect: self.0(6) < 5(5) ...
}
```

## Contribution

- 🙌 Questions? Join the [Discord channel](https://discord.gg/fFPsTqYqUg) or start a [discussion](https://github.com/drmingdrmer/validit/discussions/new).
