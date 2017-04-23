# Derive Getters

Simple boilerplate library for getters.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them and weren't going to change once created. Of course one could just use `pub` everywhere.

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.0.1"
```

Then put this in your rust project root.
```rust
#[macro_use] extern crate derive_getters;
```

## Usage

When you have a struct you want to automatically derive getters for... Just add the derive at the top like so;
```rust
#[derive(Getters)]
struct MyCheesyStruct {
    x: i64,
    y: i64,
}
```

The getters will have the same name as the fields and return references.


## Caveats

This crate will not yet create getters for unit structs, tuples or enums. If you try to derive over them, the code will just ignore you and do nothing. (Please let me know by filing an issue if this isn't the right way to handle things. Should this crate panic if attempting to derive on an unsupported value?)