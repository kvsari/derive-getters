# Derive Getters

[![Build Status](https://travis-ci.org/kvsari/derive-getters.svg?branch=master)](https://travis-ci.org/kvsari/derive-getters)

Simple procedural macro for generating field getter methods on a named struct.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them to access and they weren't going to change once created. One could use `pub` everywhere but that would enable mutating the fields which is what this procedural macro aims to avoid.

Getters will be generated according to [convention](https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md#gettersetter-apis). This means that the generated methods will reside within the struct namespace.

## What this crate won't do
There are no mutable getters and it's not planned. There are no setters either nor will there ever be.

## Rust Docs
[Documentation is here.](https://docs.rs/derive-getters/0.1.0)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.1.0"
```

Then import the `Getters` macro in whichever module it's needed (assuming 2018 edition).
```rust
use derive_getters::Getters;

```
Otherwise just import at crate root.
```rust
#[macro_use]
extern crate derive_getters;
```

## Usage

When you have a struct you want to automatically derive getters for... Just add the derive at the top like so;
```rust
#[derive(Getters)]
pub struct MyCheesyStruct {
    x: i64,
    y: i64,
}
```

A new impl will be produced for `MyCheesyStruct`.
```rust
impl MyCheesyStruct {
    pub fn x(&self) -> &i64 {
        &self.x
    }

    pub fn y(&self) -> &i64 {
        &self.y
    }
}
```

This crate can also handle structs with simple generic parameters and lifetime annotations. Check [docs](https://docs.rs/derive-getters/0.1.0) for further details.
```rust
#[derive(Getters)]
pub struct StructWithGeneric<'a, T> {
    concrete: f64,
    generic: T,
    text: &'a str,
}
```

### Attributes
This macro comes with two optional field attributes.
* `#[getter(skip)]` to skip generating getters for a field.
* `#[getter(rename = "name")]` to change the getter name to "name".

## Caveats
1. Will not work on unit structs, tuples or enums. Derive `Getters` over them and the macro will chuck a wobbly.
2. All getter methods return an immutable reference, `&`, to their field. This means for some types it can get awkward.

## Alternatives
[getset](https://github.com/Hoverbear/getset).
