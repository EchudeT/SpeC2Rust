mod yank;

use crate::yank::Yank;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        Yank::main(&args)
    }
}

fn main() {
    std::process::exit(Main::run());
}
