//! Tests

#[macro_use] extern crate derive_getters;

#[derive(Getters)]
struct Number {
    num: u64,    
}

#[test]
fn number_num() {
    let number = Number { num: 655 };
    assert!(number.get_num() == &655);
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

    assert!(numbers.get_integer() == &655);
    assert!(numbers.get_floating() == &45.5);
    assert!(numbers.get_byte() == &122);
}

/*
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
*/

#[derive(Getters)]
struct Textual {
    author: Option<String>,
    heading: String,    
    lines: Vec<String>,    
}

#[test]
fn textual_struct() {
    let article = Textual {
        author: None,
        heading: "abcdefg".to_string(),
        lines: vec!["hijk".to_string(),
                    "lmno".to_string(),
                    "pqrs".to_string(),
                    "tuvw".to_string(),],
    };

    assert!(article.get_author() == &None);
    assert!(article.get_heading() == "abcdefg");
    assert!(article.get_lines().len() == 4);
    assert!(article.get_lines()[0] == "hijk");
    assert!(article.get_lines()[1] == "lmno");
    assert!(article.get_lines()[2] == "pqrs");
    assert!(article.get_lines()[3] == "tuvw");

    let book = Textual {
        author: Some("name".to_string()),
        heading: "1234".to_string(),
        lines: vec!["2345".to_string(),
                    "3456".to_string(),
                    "4567".to_string(),
                    "5678".to_string(),],        
    };

    assert!(book.get_author() == &Some("name".to_string()));
}

/// There shouldn't be any dead code warnings
#[derive(Getters)]
struct DeadCode {
    x: u64,
    y: u64,
    z: u64,
}

#[test]
fn dead_code_struct() {
    let dc = DeadCode {
        x: 1,
        y: 2,
        z: 3,
    };

    assert!(*dc.get_x() == 1);
}
