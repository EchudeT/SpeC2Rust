mod pwd;
mod progname;
mod same_inode;
mod xgetcwd;
mod xalloc_die;
use crate::pwd::Pwd;
use std::env;
use std::process::ExitCode;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        let args: Vec<String> = env::args().collect();
        Pwd::main(&args)
    }
}

fn main() -> ExitCode {
    ExitCode::from(Main::run().clamp(0, u8::MAX as i32) as u8)
}
