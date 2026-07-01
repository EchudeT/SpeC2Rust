mod bash;
mod getopt;
mod getopt1;
mod shell;
mod tilde;
mod which;

use crate::which::Which;
use std::env;
use std::process;

pub struct Main;

impl Main {
    pub fn run(args: Vec<String>) -> i32 {
        let mut app = Which::default();
        app.run(&args)
    }
}

fn main() {
    let code = Main::run(env::args().collect());
    process::exit(code);
}
