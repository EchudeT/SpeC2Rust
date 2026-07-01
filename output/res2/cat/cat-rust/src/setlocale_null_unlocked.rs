pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn get(category: i32) -> Option<String> {
        let locale = std::env::var_os("LC_ALL")
            .or_else(|| category_env_var(category).and_then(std::env::var_os))
            .or_else(|| std::env::var_os("LANG"));

        let value = locale?;
        let text = value.into_string().ok()?;

        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

fn category_env_var(category: i32) -> Option<&'static str> {
    match category {
        0 => Some("LC_CTYPE"),
        1 => Some("LC_NUMERIC"),
        2 => Some("LC_TIME"),
        3 => Some("LC_COLLATE"),
        4 => Some("LC_MONETARY"),
        5 => Some("LC_MESSAGES"),
        6 => Some("LC_ALL"),
        7 => Some("LC_PAPER"),
        8 => Some("LC_NAME"),
        9 => Some("LC_ADDRESS"),
        10 => Some("LC_TELEPHONE"),
        11 => Some("LC_MEASUREMENT"),
        _ => None,
    }
}
