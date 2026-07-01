use std::process::ExitCode;

mod c4;
mod hello;

pub struct Main;

impl Main {
    pub fn run() -> ExitCode {
        let args: Vec<String> = std::env::args().collect();

        match c4::C4::main(&args) {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{output}");
                }
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(1)
            }
        }
    }
}

fn main() -> ExitCode {
    Main::run()
}
