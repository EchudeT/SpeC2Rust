pub struct HardLocale;

impl HardLocale {
    pub fn is_hard(category: i32) -> bool {
        let locale = std::env::var("LC_ALL")
            .ok()
            .filter(|value| !value.is_empty())
            .or_else(|| {
                category_name(category).and_then(|name| {
                    std::env::var(name)
                        .ok()
                        .filter(|value| !value.is_empty())
                })
            })
            .or_else(|| {
                std::env::var("LANG")
                    .ok()
                    .filter(|value| !value.is_empty())
            })
            .unwrap_or_else(|| "C".to_string());

        if locale != "C" && locale != "POSIX" {
            return true;
        }

        #[cfg(target_os = "android")]
        {
            if std::env::var("LC_CTYPE")
                .ok()
                .filter(|value| !value.is_empty())
                .map(|value| value != "C" && value != "POSIX")
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }
}

fn category_name(category: i32) -> Option<&'static str> {
    match category {
        0 => Some("LC_CTYPE"),
        1 => Some("LC_NUMERIC"),
        2 => Some("LC_TIME"),
        3 => Some("LC_COLLATE"),
        4 => Some("LC_MONETARY"),
        5 => Some("LC_MESSAGES"),
        6 => Some("LC_ALL"),
        _ => None,
    }
}
