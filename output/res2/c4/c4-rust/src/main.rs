mod c4;
mod hello;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        match crate::c4::C4::run_c_4(&args) {
            Ok(status) => status,
            Err(message) => {
                eprintln!("{message}");
                1
            }
        }
    }
}

fn main() {
    std::process::exit(Main::run());
}
