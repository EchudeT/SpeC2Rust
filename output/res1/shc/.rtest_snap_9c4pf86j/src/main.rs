mod shc;

use crate::shc::Shc;
use std::process::ExitCode;

pub struct Main;

impl Main {
    pub fn run() -> ExitCode {
        let args: Vec<String> = std::env::args().collect();
        let mut shc = Shc::default();
        shc.main(&args)
    }
}

fn main() -> ExitCode {
    Main::run()
}
