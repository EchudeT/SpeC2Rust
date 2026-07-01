pub struct Foo;

impl Foo {
    pub fn main() -> i32 {
        let _ = Self::run();
        let _ = Self::run();
        0
    }

    pub fn run() -> i32 {
        0
    }
}
