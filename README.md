# Derive Getters

Simple boilerplate library for getters.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them and weren't going to change once created. Of course one could just use `pub` everywhere.

Getters will be generated according to [convention](https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md#gettersetter-apis). This means that the generated methods will reside within the struct namespace.

There is no mutability in getters and it isn't planned. There are no setters either nor will there ever be.

## Rust Docs
[Documentation is here.](https://docs.rs/derive-getters/0.0.8)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.0.8"
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

This crate can also handle structs with simple generic parameters. Check [docs](https://docs.rs/derive-getters/0.0.8) for further details.
```rust
#[derive(Getters)]
pub struct StructWithGeneric<T> {
    concrete: f64,
    generic: T,
}
```

## Caveats
1. This crate will not create getters for unit structs, tuples or enums. If you try to derive over them the code will chuck a wobbly.
2. All getter methods return a `&` immutable reference to their field. This means for some types it can get awkward.
3. Cannot handle lifetimes.

## Alternatives
[getset](https://github.com/Hoverbear/getset).
