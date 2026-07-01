mod bash;
mod getopt;
mod getopt1;
mod shell;
mod tilde;
mod which;

use crate::which::Which;

pub struct Main;

impl Main {
    pub fn run() -> std::process::ExitCode {
        let args: Vec<String> = std::env::args().collect();
        let mut app = Which::default();
        let code = app.run(&args);
        std::process::ExitCode::from(code.clamp(0, 255) as u8)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = Which::default();
    let code = app.run(&args);
    std::process::exit(code.clamp(0, 255));
}
