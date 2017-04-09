//! Tests

#[macro_use] extern crate derive_getters;

#[derive(Testing)]
struct TestStruct;

#[test]
fn test() {
    assert!(TestStruct::bongle());
}
