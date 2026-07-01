mod shc;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = std::env::args().collect();
        crate::shc::Shc::main(&args)
    }
}

fn main() {
    std::process::exit(Main::run());
}
