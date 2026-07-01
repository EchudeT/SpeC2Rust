mod yank;

use crate::yank::Yank;

pub struct Main;

impl Main {
    pub fn run() -> std::process::ExitCode {
        Yank::main()
    }
}

fn main() -> std::process::ExitCode {
    Main::run()
}
