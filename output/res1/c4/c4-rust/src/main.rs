use std::env;
use std::process;

mod c4;
mod hello;

pub struct Main;

impl Main {
    pub fn run() -> Result<(), String> {
        let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            println!("{}", hello::Hello::main());
            return Ok(());
        }

        c4::C4::main(&args).map(|_| ())
    }
}

fn main() {
    if let Err(error) = Main::run() {
        eprintln!("{error}");
        process::exit(1);
    }
}
