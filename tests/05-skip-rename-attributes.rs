use derive_getters::Getters;

#[derive(Getters)]
struct SkipAField {
    keep: u64,
    
    #[getter(skip)]
    skip: String,
}

impl SkipAField {
    pub fn new<T: Into<String>>(keep: u64, skip: T) -> Self {
        SkipAField { keep, skip: skip.into() }
    }
}

fn main() {
    let s1 = SkipAField::new(45, "You can't get me.");

    assert!(*s1.keep() == 45);
}
