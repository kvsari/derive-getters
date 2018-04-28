# Derive Getters

Simple boilerplate library for getters.

The need for this library came about when I was making various data structures for JSON to deserialize into. These data structures had many fields in them and weren't going to change once created. Of course one could just use `pub` everywhere.

Prior versions of `derive-getters` would add impl's to the struct in question. This could cause conflicts in the naming if there was a need to create a method of the same name as one of the struct fields. In order to not pollute the struct namespace with this libraries autegenerated code, it will instead produce a trait which will contain the getters. Therefore to make use of the getters one must import the trait. Within the trait, the getter methods will be the struct field name with `get_` prepended.

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
derive-getters = "0.0.3"
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

A new trait will be produced called `MyCheesyStructGetters`. This trait will exist in the same module as `MyCheesyStruct`.
```rust
pub trait MyCheesyStructGetters {
		fn get_x(&self) -> i64;
		fn get_y(&self) -> i64;
}
```
And an impl for the trait will be done for `MyCheesyStruct`.

Then, when you want to use the getters for `MyCheesyStruct`, add a `use` for the trait `use module::path::to::MyCheesyStructGetters`.

## Caveats

This crate will not yet create getters for unit structs, tuples or enums. If you try to derive over them, the code will just ignore you and do nothing. (Please let me know by filing an issue if this isn't the right way to handle things. Should this crate panic if attempting to derive on an unsupported value?)
