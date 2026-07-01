use std::env;
use std::io::{self, Write};
use std::process::Command;

pub struct Whoami;

impl Whoami {
    pub fn who_am_i() -> Result<String, String> {
        if let Ok(output) = Command::new("id").arg("-un").output() {
            if output.status.success() {
                let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !name.is_empty() {
                    return Ok(name);
                }
            }
        }

        if let Ok(user) = env::var("USER") {
            if !user.is_empty() {
                return Ok(user);
            }
        }

        Err("I don't know!".to_string())
    }

    pub fn main(_args: &[String]) -> i32 {
        match Self::who_am_i() {
            Ok(user) => {
                println!("{user}");
                0
            }
            Err(message) => {
                let _ = writeln!(io::stderr(), "{message}");
                1
            }
        }
    }
}
