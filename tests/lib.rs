//! Tests

#[macro_use] extern crate derive_getters;

#[derive(Getters)]
struct Number {
    num: u64,    
}

#[test]
fn number_num() {
    let number = Number { num: 655 };
    assert!(number.num() == &655);
}

#[derive(Getters)]
struct ManyNumbers {
    integer: u64,
    floating: f64,
    byte: u8,
}

#[test]
fn many_numbers() {
    let numbers = ManyNumbers {
        integer: 655,
        floating: 45.5,
        byte: 122,
    };

    assert!(numbers.integer() == &655);
    assert!(numbers.floating() == &45.5);
    assert!(numbers.byte() == &122);
}

/// This should do nothing
#[allow(dead_code)]
#[derive(Getters)]
struct UnitStruct;


/// Ditto
#[allow(dead_code)]
#[derive(Getters)]
enum AlgebraicDataType {
    X,
    Y,
    Z,
}

