use std::env;

pub struct HardLocale;

impl HardLocale {
    pub fn is_hard(category: &str) -> bool {
        let locale = Self::locale_for_category(category);

        if locale == "C" || locale == "POSIX" {
            #[cfg(target_os = "android")]
            {
                return !Self::looks_single_byte_locale();
            }

            #[cfg(not(target_os = "android"))]
            {
                return false;
            }
        }

        true
    }

    fn locale_for_category(category: &str) -> String {
        let vars: &[&str] = match category {
            "LC_ALL" => &["LC_ALL"],
            "LC_COLLATE" => &["LC_ALL", "LC_COLLATE", "LANG"],
            "LC_CTYPE" => &["LC_ALL", "LC_CTYPE", "LANG"],
            "LC_MESSAGES" => &["LC_ALL", "LC_MESSAGES", "LANG"],
            "LC_MONETARY" => &["LC_ALL", "LC_MONETARY", "LANG"],
            "LC_NUMERIC" => &["LC_ALL", "LC_NUMERIC", "LANG"],
            "LC_TIME" => &["LC_ALL", "LC_TIME", "LANG"],
            _ => &["LC_ALL", "LANG"],
        };

        for key in vars {
            if let Some(value) = env::var_os(key) {
                let s = value.to_string_lossy().into_owned();
                if !s.is_empty() {
                    return s;
                }
            }
        }

        "C".to_string()
    }

    #[cfg(target_os = "android")]
    fn looks_single_byte_locale() -> bool {
        let ctype = Self::locale_for_category("LC_CTYPE");
        let upper = ctype.to_ascii_uppercase();

        !(upper.contains("UTF-8") || upper.contains("UTF8"))
    }
}
