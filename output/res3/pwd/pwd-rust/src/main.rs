mod c_ctype;
mod same_inode;
mod c32isprint;
mod c_strcasecmp;
mod close_stream;
mod closeout;
mod exitfail;
mod fclose;
mod fflush;
mod fseeko;
mod hard_locale;
mod ialloc;
mod localcharset;
mod mbrtoc32;
mod mbszero;
mod progname;
mod propername_lite;
mod pwd;
mod quotearg;
mod root_dev_ino;
mod setlocale_null;
mod setlocale_null_unlocked;
mod version;
mod version_etc;
mod version_etc_fsf;
mod xalloc_die;
mod xgetcwd;
mod xmalloc;

use crate::closeout::Closeout;
use crate::pwd::Pwd;
use std::env;
use std::io::{self, Write};
use std::process;

pub struct Main;

impl Main {
    pub fn run(args: &[String]) -> i32 {
        match Pwd::main(args) {
            Ok(output) => {
                if !output.is_empty() {
                    let mut stdout = io::stdout().lock();
                    if writeln!(stdout, "{output}").is_err() {
                        return 1;
                    }
                }
                Closeout::close_stdout();
                0
            }
            Err(error) => {
                let _ = writeln!(io::stderr().lock(), "{error}");
                Closeout::close_stdout();
                1
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    process::exit(Main::run(&args));
}
