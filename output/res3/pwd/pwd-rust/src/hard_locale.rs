use std::env;

/// Locale hardness detection.
///
/// This is a Rust-style port of gnulib's `hard_locale` behavior:
/// it reports whether the active locale for a given category is
/// meaningfully different from the traditional `"C"` / `"POSIX"` locale.
pub struct HardLocale;

impl HardLocale {
    /// Returns `true` when the locale for `category` is "hard", meaning it is
    /// not the plain `"C"` or `"POSIX"` locale.
    ///
    /// The `category` parameter is accepted for API compatibility with the
    /// original behavior, but this Rust implementation relies on process
    /// environment variables rather than `setlocale(3)` category queries.
    pub fn is_hard(category: i32) -> bool {
        let _ = category;

        let locale = Self::current_locale_name();

        if let Some(locale) = locale {
            if locale != "C" && locale != "POSIX" {
                return true;
            }

            #[cfg(target_os = "android")]
            {
                if Self::multibyte_locale_hint() {
                    return true;
                }
            }
        }

        false
    }

    fn current_locale_name() -> Option<String> {
        for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
            if let Some(value) = Self::nonempty_env(key) {
                return Some(Self::normalize_locale_name(&value));
            }
        }
        None
    }

    fn nonempty_env(key: &str) -> Option<String> {
        match env::var(key) {
            Ok(value) if !value.is_empty() => Some(value),
            _ => None,
        }
    }

    fn normalize_locale_name(locale: &str) -> String {
        locale.split_once('@').map_or(locale, |(base, _)| base).to_string()
    }

    #[cfg(target_os = "android")]
    fn multibyte_locale_hint() -> bool {
        if let Some(locale) = Self::current_locale_name() {
            let upper = locale.to_ascii_uppercase();
            return upper.contains("UTF-8") || upper.contains("UTF8");
        }
        false
    }
}
