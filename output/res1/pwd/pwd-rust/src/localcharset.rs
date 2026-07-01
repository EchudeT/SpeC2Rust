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

    pub fn locale_charset() -> &'static str {
        let locale = env_locale();
        let mut codeset = extract_codeset(&locale).unwrap_or_else(|| default_codeset(&locale));

        if is_utf8_name(codeset) {
            return "UTF-8";
        }

        if let Some(mapped) = lookup_alias(codeset) {
            codeset = mapped;
        } else if codeset.is_empty() {
            codeset = "ASCII";
        }

        codeset
    }
}

fn env_locale() -> String {
    for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Some(value) = env::var_os(key) {
            let value = value.to_string_lossy();
            if !value.is_empty() {
                return value.into_owned();
            }
        }
    }
    String::new()
}

fn extract_codeset(locale: &str) -> Option<&str> {
    let dot = locale.find('.')?;
    let tail = &locale[dot + 1..];
    if tail.is_empty() {
        return None;
    }
    let end = tail.find('@').unwrap_or(tail.len());
    let codeset = &tail[..end];
    if codeset.is_empty() {
        None
    } else {
        Some(codeset)
    }
}

fn default_codeset(locale: &str) -> &'static str {
    if locale.is_empty() || locale == "C" || locale == "POSIX" {
        "ASCII"
    } else if locale.to_ascii_uppercase().ends_with("UTF-8")
        || locale.to_ascii_uppercase().ends_with("UTF8")
    {
        "UTF-8"
    } else {
        "ASCII"
    }
}

fn is_utf8_name(name: &str) -> bool {
    let normalized: String = name
        .bytes()
        .filter(|b| *b != b'-' && *b != b'_')
        .map(|b| (b as char).to_ascii_uppercase())
        .collect();
    normalized == "UTF8"
}

fn lookup_alias(name: &str) -> Option<&'static str> {
    ALIAS_TABLE
        .binary_search_by(|entry| entry.alias.cmp(name))
        .ok()
        .map(|index| ALIAS_TABLE[index].canonical)
}

const ALIAS_TABLE: &[TableEntry] = &[
    TableEntry {
        alias: "ANSI_X3.4-1968",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "ANSI_X3.4-1986",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "CP65001",
        canonical: "UTF-8",
    },
    TableEntry {
        alias: "ISO-IR-6",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "US",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "US-ASCII",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "UTF-8",
        canonical: "UTF-8",
    },
    TableEntry {
        alias: "UTF8",
        canonical: "UTF-8",
    },
];
