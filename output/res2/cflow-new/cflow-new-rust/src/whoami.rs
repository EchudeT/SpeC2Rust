use std::env;

pub struct Whoami;

impl Whoami {
    pub fn who_am_i() -> Result<String, ()> {
        if let Ok(user) = env::var("USER") {
            if !user.is_empty() {
                println!("{user}");
                return Ok(user);
            }
        }

        eprintln!("I don't know!");
        Err(())
    }

    pub fn main() -> i32 {
        match Self::who_am_i() {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}
