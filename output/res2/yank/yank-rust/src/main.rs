mod yank;

use crate::yank::Yank;
use std::process::exit;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        let mut app = Yank::default();
        app.main(&args)
    }
}

fn main() {
    exit(Main::run());
}
