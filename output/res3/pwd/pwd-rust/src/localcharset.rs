use std::env;

pub struct Localcharset;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableEntry {
    pub key: &'static str,
    pub canonical: &'static str,
}

impl Localcharset {
    pub fn table_entry(key: &'static str, canonical: &'static str) -> TableEntry {
        TableEntry { key, canonical }
    }

    pub fn locale_charset() -> String {
        let mut codeset = Self::detect_codeset();

        if let Some(mapped) = Self::lookup_alias(&codeset) {
            codeset = mapped.to_string();
        } else if cfg!(any(target_os = "macos", target_os = "ios")) {
            codeset = "UTF-8".to_string();
        } else if codeset.is_empty() {
            codeset = "ASCII".to_string();
        }

        if cfg!(any(target_os = "macos", target_os = "ios"))
            && codeset == "UTF-8"
            && Self::is_posix_locale()
        {
            return "ASCII".to_string();
        }

        codeset
    }

    fn detect_codeset() -> String {
        if let Some(explicit) = Self::codeset_from_locale_env() {
            return explicit;
        }

        if cfg!(windows) {
            return Self::windows_fallback_codeset();
        }

        String::new()
    }

    fn codeset_from_locale_env() -> Option<String> {
        let locale = Self::locale_from_env()?;
        Self::extract_codeset(&locale)
    }

    fn locale_from_env() -> Option<String> {
        for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
            let value = env::var_os(key)?;
            let text = value.to_string_lossy();
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    fn extract_codeset(locale: &str) -> Option<String> {
        let dot = locale.find('.')?;
        let suffix = &locale[dot + 1..];
        let end = suffix.find('@').unwrap_or(suffix.len());
        let codeset = suffix[..end].trim();
        if codeset.is_empty() {
            None
        } else if codeset.eq_ignore_ascii_case("utf8") || codeset == "65001" {
            Some("UTF-8".to_string())
        } else {
            Some(codeset.to_string())
        }
    }

    fn windows_fallback_codeset() -> String {
        if let Ok(lang) = env::var("LANG") {
            if let Some(codeset) = Self::extract_codeset(&lang) {
                return codeset;
            }
        }
        "CP1252".to_string()
    }

    fn is_posix_locale() -> bool {
        match Self::locale_from_env() {
            Some(locale) => locale == "C" || locale == "POSIX",
            None => false,
        }
    }

    fn lookup_alias(name: &str) -> Option<&'static str> {
        const ALIAS_TABLE: &[TableEntry] = &[
            TableEntry {
                key: "ANSI_X3.4-1968",
                canonical: "ASCII",
            },
            TableEntry {
                key: "ASCII",
                canonical: "ASCII",
            },
            TableEntry {
                key: "CP1252",
                canonical: "CP1252",
            },
            TableEntry {
                key: "UTF-8",
                canonical: "UTF-8",
            },
            TableEntry {
                key: "US-ASCII",
                canonical: "ASCII",
            },
            TableEntry {
                key: "UTF8",
                canonical: "UTF-8",
            },
        ];

        ALIAS_TABLE
            .binary_search_by(|entry| entry.key.cmp(name))
            .ok()
            .map(|index| ALIAS_TABLE[index].canonical)
    }
}
