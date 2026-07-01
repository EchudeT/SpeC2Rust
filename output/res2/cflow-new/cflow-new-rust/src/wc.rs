use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process;

pub struct Wc;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Counts {
    chars: u64,
    words: u64,
    lines: u64,
}

impl Counts {
    fn add_assign(&mut self, other: Self) {
        self.chars += other.chars;
        self.words += other.words;
        self.lines += other.lines;
    }

    fn count_byte(&mut self, byte: u8) {
        self.chars += 1;
        if byte == b'\n' {
            self.lines += 1;
        }
    }
}

impl Wc {
    pub fn error_print(include_os_error: bool, message: &str, source: Option<&io::Error>) -> ! {
        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{message}");
        if include_os_error {
            match source {
                Some(err) => {
                    let _ = writeln!(stderr, ": {err}");
                }
                None => {
                    let fallback = io::Error::last_os_error();
                    let _ = writeln!(stderr, ": {fallback}");
                }
            }
        } else {
            let _ = writeln!(stderr);
        }
        process::exit(1);
    }

    pub fn fail(message: &str) -> ! {
        Self::error_print(false, message, None)
    }

    pub fn fail_with_error(message: &str, error: &io::Error) -> ! {
        Self::error_print(true, message, Some(error))
    }

    pub fn print_report(file: &str, ccount: u64, wcount: u64, lcount: u64) {
        println!("{lcount:>6} {wcount:>6} {ccount:>6} {file}");
    }

    pub fn is_word(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    pub fn getword<R: Read>(reader: &mut R, counts: &mut Counts) -> io::Result<bool> {
        let mut one = [0_u8; 1];

        loop {
            let n = reader.read(&mut one)?;
            if n == 0 {
                return Ok(false);
            }

            let byte = one[0];
            if Self::is_word(byte) {
                counts.words += 1;
                counts.count_byte(byte);
                break;
            }
            counts.count_byte(byte);
        }

        loop {
            let n = reader.read(&mut one)?;
            if n == 0 {
                return Ok(false);
            }

            let byte = one[0];
            counts.count_byte(byte);
            if !Self::is_word(byte) {
                return Ok(true);
            }
        }
    }

    pub fn count_file(file: &str) {
        let opened = File::open(file);
        let file_handle = match opened {
            Ok(fp) => fp,
            Err(err) => Self::fail_with_error(&format!("cannot open file `{file}`"), &err),
        };

        let mut reader = BufReader::new(file_handle);
        let mut counts = Counts::default();

        loop {
            match Self::getword(&mut reader, &mut counts) {
                Ok(true) => {}
                Ok(false) => break,
                Err(err) => Self::fail_with_error(&format!("cannot open file `{file}`"), &err),
            }
        }

        Self::print_report(file, counts.chars, counts.words, counts.lines);
    }

    pub fn main(args: &[String]) -> i32 {
        if args.len() < 2 {
            Self::fail("usage: wc FILE [FILE...]");
        }

        let mut total = Counts::default();

        for file in &args[1..] {
            let opened = File::open(file);
            let file_handle = match opened {
                Ok(fp) => fp,
                Err(err) => Self::fail_with_error(&format!("cannot open file `{file}`"), &err),
            };

            let mut reader = BufReader::new(file_handle);
            let mut counts = Counts::default();

            loop {
                match Self::getword(&mut reader, &mut counts) {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(err) => Self::fail_with_error(&format!("cannot open file `{file}`"), &err),
                }
            }

            Self::print_report(file, counts.chars, counts.words, counts.lines);
            total.add_assign(counts);
        }

        if args.len() > 2 {
            Self::print_report("total", total.chars, total.words, total.lines);
        }

        0
    }
}
