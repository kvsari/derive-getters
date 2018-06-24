# Derive Getters

Simple boilerplate library for getters.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them and weren't going to change once created. Of course one could just use `pub` everywhere.

Getters will be generated according to [convention](https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md#gettersetter-apis). This means that the generated methods will reside within the struct namespace.

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.0.6"
```

Then put this in your rust project root.
```rust
#[macro_use]
extern crate derive_getters;
```
or
```rust
#[macro_use] extern crate derive_getters;
```
PS. This way is better. It makes the code run faster.

## Usage

When you have a struct you want to automatically derive getters for... Just add the derive at the top like so;
```rust
#[derive(Getters)]
struct MyCheesyStruct {
    x: i64,
    y: i64,
}
```

A new impl will be produced for `MyCheesyStruct`.
```rust
impl MyCheesyStruct {
    fn get_x(&self) -> i64 {
        &self.x
    }

    fn get_y(&self) -> i64 {
        &self.y
    }
}
```

## Caveats

This crate will not yet create getters for unit structs, tuples or enums. If you try to derive over them, the code will just ignore you and do nothing. (Please let me know by filing an issue if this isn't the right way to handle things. Should this crate panic if attempting to derive on an unsupported value?)

Getters for `String` fields should return a `&str`. Right now you have to do an awkward `&my_struct.name()[..]` to shoehorn it into some semblance of a `&str`.
