use std::env;

pub struct HardLocale;

impl HardLocale {
    pub fn is_hard(category: i32) -> bool {
        let locale = match Self::locale_for_category(category) {
            Some(locale) => locale,
            None => return false,
        };

        if locale != "C" && locale != "POSIX" {
            return true;
        }

        #[cfg(target_os = "android")]
        {
            if Self::android_mb_cur_max() > 1 {
                return true;
            }
        }

        false
    }

    fn locale_for_category(category: i32) -> Option<String> {
        let value = match category {
            0 => Self::first_present(&["LC_ALL", "LANG"]),
            1 => Self::from_env("LC_COLLATE"),
            2 => Self::from_env("LC_CTYPE"),
            3 => Self::from_env("LC_MONETARY"),
            4 => Self::from_env("LC_NUMERIC"),
            5 => Self::from_env("LC_TIME"),
            6 => Self::from_env("LC_MESSAGES"),
            _ => None,
        };

        value.and_then(Self::normalize_locale)
    }

    fn first_present(keys: &[&str]) -> Option<String> {
        for key in keys {
            if let Some(value) = Self::from_env(key) {
                return Some(value);
            }
        }
        None
    }

    fn from_env(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    fn normalize_locale(locale: String) -> Option<String> {
        if locale.is_empty() {
            None
        } else {
            Some(locale)
        }
    }

    #[cfg(target_os = "android")]
    fn android_mb_cur_max() -> usize {
        match Self::from_env("LC_ALL")
            .or_else(|| Self::from_env("LC_CTYPE"))
            .or_else(|| Self::from_env("LANG"))
        {
            Some(locale) if Self::looks_multibyte(&locale) => 4,
            _ => 1,
        }
    }

    #[cfg(target_os = "android")]
    fn looks_multibyte(locale: &str) -> bool {
        let lower = locale.to_ascii_lowercase();
        lower.contains("utf-8") || lower.contains("utf8")
    }
}
