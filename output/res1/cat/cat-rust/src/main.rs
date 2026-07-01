mod cat;

use crate::cat::Cat;
use std::process::ExitCode;

pub struct Main;

impl Main {
    pub fn run() -> ExitCode {
        let args: Vec<String> = std::env::args().collect();
        Cat::main(&args)
    }
}

fn main() -> ExitCode {
    Main::run()
}
