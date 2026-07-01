mod c4;
mod hello;

use std::process::ExitCode;

pub struct Main;

impl Main {
    pub fn run() -> ExitCode {
        crate::c4::C4::main()
    }
}

fn main() -> ExitCode {
    Main::run()
}
