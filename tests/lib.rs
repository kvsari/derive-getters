#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-legacy.rs");
    t.pass("tests/02-simple-single-generic.rs");
}
