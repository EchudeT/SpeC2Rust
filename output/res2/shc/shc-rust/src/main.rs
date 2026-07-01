mod shc;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        shc::Shc::main()
    }
}

fn main() {
    std::process::exit(Main::run());
}
