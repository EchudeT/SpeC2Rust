use std::env;
use std::io::{self, Write};

pub struct Whoami;

impl Whoami {
    pub fn who_am_i() -> Result<String, io::Error> {
        if let Some(user) = env::var_os("USER") {
            let user = user.to_string_lossy().into_owned();
            if !user.is_empty() {
                return Ok(user);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "current user could not be determined",
        ))
    }

    pub fn main(_args: &[String]) -> i32 {
        match Self::who_am_i() {
            Ok(user) => {
                let mut stdout = io::stdout().lock();
                if writeln!(stdout, "{user}").is_ok() {
                    0
                } else {
                    1
                }
            }
            Err(_) => {
                let _ = writeln!(io::stderr().lock(), "I don't know!");
                1
            }
        }
    }
}
