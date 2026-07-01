mod sds;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        crate::sds::Sds::main_root()
    }
}

fn main() {
    std::process::exit(Main::run());
}
