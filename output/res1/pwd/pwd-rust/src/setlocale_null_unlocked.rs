pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn query(category: i32) -> Option<String> {
        let result = std::panic::catch_unwind(|| std::env::var("LC_ALL").ok())
            .ok()
            .flatten()
            .filter(|value| !value.is_empty());

        if result.is_some() {
            return result;
        }

        let category_value = match category {
            0 => std::env::var("LC_CTYPE").ok(),
            1 => std::env::var("LC_NUMERIC").ok(),
            2 => std::env::var("LC_TIME").ok(),
            3 => std::env::var("LC_COLLATE").ok(),
            4 => std::env::var("LC_MONETARY").ok(),
            5 => std::env::var("LC_MESSAGES").ok(),
            _ => None,
        }
        .filter(|value| !value.is_empty());

        if category_value.is_some() {
            return category_value;
        }

        let lang = std::panic::catch_unwind(|| std::env::var("LANG").ok())
            .ok()
            .flatten()
            .filter(|value| !value.is_empty());

        if lang.is_some() {
            return lang;
        }

        if Self::android_fallback_category(category) {
            Some(String::from("C"))
        } else {
            None
        }
    }

    fn android_fallback_category(category: i32) -> bool {
        matches!(category, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11)
    }
}
