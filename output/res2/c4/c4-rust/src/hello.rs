pub struct Hello;

impl Hello {
    pub fn main() -> Result<(), String> {
        Self::module_test();
        Ok(())
    }

    pub fn module_test() {
        println!("hello, world");
    }
}
