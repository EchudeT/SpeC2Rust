mod main_root;
pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        crate::main_root::main_entry(&args)
    }
}

fn main() {
    std::process::exit(Main::run());
}
