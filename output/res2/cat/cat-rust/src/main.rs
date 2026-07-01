mod c_ctype;
mod alignalloc;
mod binary_io;
mod c32isprint;
mod c_strcasecmp;
mod cat;
mod close_stream;
mod closeout;
mod copy_file_range;
mod exitfail;
mod fadvise;
mod fclose;
mod fcntl;
mod fflush;
mod fpurge;
mod fseeko;
mod full_write;
mod hard_locale;
mod ialloc;
mod localcharset;
mod mbrtoc32;
mod mbszero;
mod progname;
mod propername_lite;
mod quotearg;
mod safe_read;
mod safe_write;
mod setlocale_null;
mod setlocale_null_unlocked;
mod stdbit;
mod stdc_leading_zeros;
mod version;
mod version_etc;
mod version_etc_fsf;
mod xalignalloc;
mod xalloc_die;
mod xbinary_io;
mod xmalloc;

use crate::cat::Cat;

pub struct Main;

impl Main {
    pub fn run() -> i32 {
        Cat::main()
    }
}

fn main() {
    std::process::exit(Main::run());
}
