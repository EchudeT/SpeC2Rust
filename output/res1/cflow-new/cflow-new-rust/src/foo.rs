pub struct Foo;

impl Foo {
    pub fn value() -> i32 {
        0
    }

    pub fn main() -> i32 {
        let _ = Self::value();
        let _ = Self::value();
        0
    }
}
