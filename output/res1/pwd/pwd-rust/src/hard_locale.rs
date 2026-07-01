use std::env;

pub struct HardLocale;

impl HardLocale {
    pub fn is_hard(category: i32) -> bool {
        let locale = Self::locale_for_category(category);

        if !(locale == "C" || locale == "POSIX") {
            return true;
        }

        #[cfg(target_os = "android")]
        {
            if Self::android_c_locale_uses_multibyte() {
                return true;
            }
        }

        false
    }

    fn locale_for_category(category: i32) -> String {
        match category {
            0 => env::var("LC_ALL")
                .ok()
                .filter(|s| !s.is_empty())
                .or_else(|| env::var("LC_CTYPE").ok().filter(|s| !s.is_empty()))
                .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
                .unwrap_or_else(|| "C".to_string()),
            _ => env::var("LC_ALL")
                .ok()
                .filter(|s| !s.is_empty())
                .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
                .unwrap_or_else(|| "C".to_string()),
        }
    }

    #[cfg(target_os = "android")]
    fn android_c_locale_uses_multibyte() -> bool {
        env::var("LC_ALL")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("LC_CTYPE").ok().filter(|s| !s.is_empty()))
            .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
            .map(|locale| {
                let lowered = locale.to_ascii_lowercase();
                lowered.contains("utf-8") || lowered.contains("utf8")
            })
            .unwrap_or(false)
    }
}
