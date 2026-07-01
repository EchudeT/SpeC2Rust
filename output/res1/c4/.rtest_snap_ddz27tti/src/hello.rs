use std::process::ExitCode;

pub struct Hello;

impl Hello {
    pub fn main() -> ExitCode {
        println!("hello, world");
        ExitCode::SUCCESS
    }
}
