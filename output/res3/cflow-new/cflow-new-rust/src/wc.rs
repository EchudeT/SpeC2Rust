use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

pub struct Wc;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Counts {
    chars: u64,
    words: u64,
    lines: u64,
}

impl Wc {
    pub fn error_print(perr: bool, message: impl fmt::Display) -> ! {
        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{}", message);
        if perr {
            let err = io::Error::last_os_error();
            let _ = writeln!(stderr, ": {}", err);
        } else {
            let _ = writeln!(stderr);
        }
        process::exit(1);
    }

    pub fn error(message: impl fmt::Display) -> ! {
        Self::error_print(false, message)
    }

    pub fn perror(message: impl fmt::Display) -> ! {
        Self::error_print(true, message)
    }

    pub fn print_report(file: &str, ccount: u64, wcount: u64, lcount: u64) {
        println!("{:6} {:6} {:6} {}", lcount, wcount, ccount, file);
    }

    pub fn is_word(c: u8) -> bool {
        c.is_ascii_alphabetic()
    }

    pub fn getword<R: Read>(reader: &mut R, counts: &mut (u64, u64, u64)) -> io::Result<bool> {
        let mut saw_any = false;
        let mut buf = [0u8; 1];

        loop {
            match reader.read(&mut buf)? {
                0 => return Ok(false),
                _ => {
                    let c = buf[0];
                    saw_any = true;
                    counts.0 += 1;
                    if c == b'\n' {
                        counts.2 += 1;
                    }
                    if Self::is_word(c) {
                        counts.1 += 1;
                        break;
                    }
                }
            }
        }

        loop {
            match reader.read(&mut buf)? {
                0 => return Ok(saw_any),
                _ => {
                    let c = buf[0];
                    counts.0 += 1;
                    if c == b'\n' {
                        counts.2 += 1;
                    }
                    if !Self::is_word(c) {
                        return Ok(true);
                    }
                }
            }
        }
    }

    pub fn count_file(file: &str) -> io::Result<(u64, u64, u64)> {
        let path = Path::new(file);
        let mut fp = match File::open(path) {
            Ok(file_handle) => file_handle,
            Err(_) => Self::perror(format!("cannot open file `{}`", file)),
        };

        let mut counts = (0_u64, 0_u64, 0_u64);
        while Self::getword(&mut fp, &mut counts)? {}

        Self::print_report(file, counts.0, counts.1, counts.2);
        Ok(counts)
    }

    pub fn main(args: &[String]) -> i32 {
        if args.len() < 2 {
            Self::error("usage: wc FILE...");
        }

        let mut total = Counts::default();

        for file in &args[1..] {
            match Self::count_file(file) {
                Ok((chars, words, lines)) => {
                    total.chars += chars;
                    total.words += words;
                    total.lines += lines;
                }
                Err(err) => {
                    let _ = writeln!(io::stderr().lock(), "wc: {}: {}", file, err);
                    return 1;
                }
            }
        }

        if args.len() > 2 {
            Self::print_report("total", total.chars, total.words, total.lines);
        }

        0
    }
}
