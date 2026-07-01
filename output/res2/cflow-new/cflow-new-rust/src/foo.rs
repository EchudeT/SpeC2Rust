pub struct Foo;

impl Foo {
    pub fn value() -> i32 {
        0
    }

    pub fn main() {
        let _ = Self::value();
        let _ = Self::value();
    }
}
