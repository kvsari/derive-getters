# Derive Getters

[![Build Status](https://travis-ci.org/kvsari/derive-getters.svg?branch=master)](https://travis-ci.org/kvsari/derive-getters)

Simple boilerplate library for getters.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them and weren't going to change once created. Of course one could just use `pub` everywhere but that would enable mutating the fields breaking program semantics.

Getters will be generated according to [convention](https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md#gettersetter-apis). This means that the generated methods will reside within the struct namespace.

## What this crate won't do
There is no mutability in getters and it isn't planned. There are no setters either nor will there ever be.

## Rust Docs
[Documentation is here.](https://docs.rs/derive-getters/0.0.8)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.0.8"
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
    pub fn get_x(&self) -> &i64 {
        &self.x
    }

    pub fn get_y(&self) -> &i64 {
        &self.y
    }
}
```

This crate can also handle structs with simple generic parameters and lifetime annotations. Check [docs](https://docs.rs/derive-getters/0.0.8) for further details.
```rust
#[derive(Getters)]
pub struct StructWithGeneric<'a, T> {
    concrete: f64,
    generic: T,
    text: &'a str,
}
```

## Caveats
1. This crate will not create getters for unit structs, tuples or enums. Derive `Getters` over them and the code will chuck a wobbly.
2. All getter methods return a `&` immutable reference to their field. This means for some types it can get awkward.

## Alternatives
[getset](https://github.com/Hoverbear/getset).
