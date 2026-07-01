use std::env;

pub struct Localcharset;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableEntry {
    pub key: &'static str,
    pub canonical: &'static str,
}

const ALIAS_TABLE: &[TableEntry] = &[
    TableEntry {
        key: "ANSI_X3.4-1968",
        canonical: "ASCII",
    },
    TableEntry {
        key: "ANSI_X3.4-1986",
        canonical: "ASCII",
    },
    TableEntry {
        key: "ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        key: "CP65001",
        canonical: "UTF-8",
    },
    TableEntry {
        key: "ISO646-US",
        canonical: "ASCII",
    },
    TableEntry {
        key: "US-ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        key: "UTF8",
        canonical: "UTF-8",
    },
    TableEntry {
        key: "UTF-8",
        canonical: "UTF-8",
    },
];

const LOCALE_TABLE: &[TableEntry] = &[
    TableEntry {
        key: "C",
        canonical: "ASCII",
    },
    TableEntry {
        key: "POSIX",
        canonical: "ASCII",
    },
];

impl Localcharset {
    pub fn locale_charset() -> &'static str {
        let locale = Self::effective_locale();

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            if Self::is_posix_locale(locale) {
                return "ASCII";
            }
            return "UTF-8";
        }

        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            if let Some(codeset) = Self::extract_codeset(locale) {
                return Self::canonicalize_codeset(codeset);
            }

            if let Some(mapped) = Self::lookup_locale(locale) {
                return mapped;
            }

            "ASCII"
        }
    }

    pub fn table_entry(key: &'static str, canonical: &'static str) -> TableEntry {
        TableEntry { key, canonical }
    }

    fn effective_locale() -> &'static str {
        for name in ["LC_ALL", "LC_CTYPE", "LANG"] {
            if let Ok(value) = env::var(name) {
                if !value.is_empty() {
                    return Self::leak_string(value);
                }
            }
        }
        ""
    }

    fn extract_codeset(locale: &str) -> Option<&str> {
        let dot = locale.find('.')?;
        let after_dot = &locale[dot + 1..];
        if after_dot.is_empty() {
            return None;
        }
        let end = after_dot.find('@').unwrap_or(after_dot.len());
        let codeset = &after_dot[..end];
        if codeset.is_empty() {
            None
        } else {
            Some(codeset)
        }
    }

    fn canonicalize_codeset(codeset: &str) -> &'static str {
        if let Some(found) = Self::lookup_alias(codeset) {
            return found;
        }

        let upper = Self::ascii_upper(codeset);

        match upper.as_str() {
            "" => "ASCII",
            "UTF-8" | "UTF8" | "CP65001" => "UTF-8",
            "US-ASCII" | "ASCII" | "ANSI_X3.4-1968" | "ANSI_X3.4-1986" | "ISO646-US" => "ASCII",
            _ => Self::leak_string(codeset.to_owned()),
        }
    }

    fn lookup_alias(codeset: &str) -> Option<&'static str> {
        let folded = Self::ascii_upper(codeset);
        Self::alias_table()
            .binary_search_by(|entry| entry.key.cmp(folded.as_str()))
            .ok()
            .map(|idx| Self::alias_table()[idx].canonical)
    }

    fn lookup_locale(locale: &str) -> Option<&'static str> {
        let key = Self::ascii_upper(locale);
        Self::locale_table()
            .binary_search_by(|entry| entry.key.cmp(key.as_str()))
            .ok()
            .map(|idx| Self::locale_table()[idx].canonical)
    }

    fn is_posix_locale(locale: &str) -> bool {
        locale == "C" || locale == "POSIX"
    }

    fn ascii_upper(input: &str) -> String {
        input
            .bytes()
            .map(|b| {
                if b.is_ascii_lowercase() {
                    (b - b'a' + b'A') as char
                } else {
                    b as char
                }
            })
            .collect()
    }

    fn leak_string(value: String) -> &'static str {
        Box::leak(value.into_boxed_str())
    }

    fn alias_table() -> &'static [TableEntry] {
        ALIAS_TABLE
    }

    fn locale_table() -> &'static [TableEntry] {
        LOCALE_TABLE
    }
}
