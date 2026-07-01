mod sds;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        crate::sds::Sds::test()
    }
}

fn main() {
    std::process::exit(Main::run());
}
