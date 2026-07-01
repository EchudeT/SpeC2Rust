mod yank;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        crate::yank::Yank::main()
    }
}

fn main() {
    std::process::exit(Main::run());
}
