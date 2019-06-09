use derive_getters::Getters;

#[derive(Getters)]
struct SimpleSingleGeneric<T> {
    concrete: u16,
    generic: T,
}

fn main() {
    
}
