pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn query(category: i32) -> Option<String> {
        let result = std::env::var("LC_ALL")
            .ok()
            .filter(|value| !value.is_empty())
            .or_else(|| category_env_var(category).and_then(read_nonempty_env))
            .or_else(|| std::env::var("LANG").ok().filter(|value| !value.is_empty()))
            .or_else(|| Some(String::from("C")));

        #[cfg(target_os = "android")]
        {
            return result.or_else(|| android_fallback(category).map(str::to_owned));
        }

        #[cfg(not(target_os = "android"))]
        {
            result
        }
    }
}

fn read_nonempty_env(name: &'static str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn category_env_var(category: i32) -> Option<&'static str> {
    match category {
        0 => Some("LC_CTYPE"),
        1 => Some("LC_NUMERIC"),
        2 => Some("LC_TIME"),
        3 => Some("LC_COLLATE"),
        4 => Some("LC_MONETARY"),
        5 => Some("LC_MESSAGES"),
        _ => None,
    }
}

#[cfg(target_os = "android")]
fn android_fallback(category: i32) -> Option<&'static str> {
    match category {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => Some("C"),
        _ => None,
    }
}
