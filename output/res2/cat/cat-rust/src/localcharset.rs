use std::borrow::Cow;
use std::env;

pub struct Localcharset;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableEntry {
    pub alias: &'static str,
    pub canonical: &'static str,
}

impl Localcharset {
    pub fn table_entry(alias: &'static str, canonical: &'static str) -> TableEntry {
        TableEntry { alias, canonical }
    }

    pub fn locale_charset() -> String {
        let codeset = Self::detect_codeset();
        let canonical = Self::resolve_alias(codeset.as_ref());

        if canonical.is_empty() {
            "ASCII".to_string()
        } else {
            canonical.to_string()
        }
    }

    fn detect_codeset() -> Cow<'static, str> {
        if let Some(locale) = Self::active_locale() {
            if let Some(encoding) = Self::extract_encoding(&locale) {
                return Cow::Owned(encoding);
            }

            if locale == "C" || locale == "POSIX" {
                return Cow::Borrowed("");
            }

            if let Some(mapped) = Self::lookup_locale(locale.as_str()) {
                return Cow::Borrowed(mapped);
            }
        }

        if cfg!(target_vendor = "apple")
            || cfg!(target_os = "macos")
            || cfg!(target_os = "ios")
            || cfg!(target_os = "haiku")
        {
            Cow::Borrowed("UTF-8")
        } else {
            Cow::Borrowed("ASCII")
        }
    }

    fn active_locale() -> Option<String> {
        for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
            if let Some(value) = env::var_os(key) {
                let text = value.to_string_lossy();
                if !text.is_empty() {
                    return Some(text.into_owned());
                }
            }
        }
        None
    }

    fn extract_encoding(locale: &str) -> Option<String> {
        let (_, after_dot) = locale.split_once('.')?;
        let encoding = after_dot.split('@').next().unwrap_or(after_dot);
        if encoding.is_empty() {
            None
        } else {
            Some(encoding.to_string())
        }
    }

    fn resolve_alias(codeset: &str) -> &str {
        if codeset == "UTF-8" {
            return "UTF-8";
        }

        match Self::lookup_alias(codeset) {
            Some(canonical) => canonical,
            None => {
                if cfg!(target_vendor = "apple")
                    || cfg!(target_os = "macos")
                    || cfg!(target_os = "ios")
                    || cfg!(target_os = "haiku")
                {
                    "UTF-8"
                } else {
                    codeset
                }
            }
        }
    }

    fn lookup_alias(alias: &str) -> Option<&'static str> {
        ALIAS_TABLE
            .binary_search_by(|entry| entry.alias.cmp(alias))
            .ok()
            .map(|idx| ALIAS_TABLE[idx].canonical)
    }

    fn lookup_locale(locale: &str) -> Option<&'static str> {
        LOCALE_TABLE
            .binary_search_by(|entry| entry.alias.cmp(locale))
            .ok()
            .map(|idx| LOCALE_TABLE[idx].canonical)
    }
}

const ALIAS_TABLE: &[TableEntry] = &[
    TableEntry {
        alias: "ANSI_X3.4-1968",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "CP1252",
        canonical: "CP1252",
    },
    TableEntry {
        alias: "CPOSIX",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "IBM819",
        canonical: "ISO-8859-1",
    },
    TableEntry {
        alias: "ISO8859-1",
        canonical: "ISO-8859-1",
    },
    TableEntry {
        alias: "ISO8859-15",
        canonical: "ISO-8859-15",
    },
    TableEntry {
        alias: "ISO_8859-1",
        canonical: "ISO-8859-1",
    },
    TableEntry {
        alias: "ISO_8859-15",
        canonical: "ISO-8859-15",
    },
    TableEntry {
        alias: "US-ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "UTF8",
        canonical: "UTF-8",
    },
    TableEntry {
        alias: "utf8",
        canonical: "UTF-8",
    },
];

const LOCALE_TABLE: &[TableEntry] = &[
    TableEntry {
        alias: "C",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "POSIX",
        canonical: "ASCII",
    },
];
