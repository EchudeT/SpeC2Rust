use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Localcharset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableEntry {
    pub alias: &'static str,
    pub canonical: &'static str,
}

impl Localcharset {
    pub fn table_entry(alias: &'static str, canonical: &'static str) -> TableEntry {
        TableEntry { alias, canonical }
    }

    pub fn locale_charset() -> String {
        let codeset = Self::codeset_from_locale_name()
            .unwrap_or_else(|| Self::default_codeset().to_string());

        if let Some(mapped) = Self::resolve_alias(&codeset) {
            return mapped.to_string();
        }

        if cfg!(any(target_os = "macos", target_os = "ios", target_os = "haiku")) {
            return "UTF-8".to_string();
        }

        if codeset.is_empty() {
            "ASCII".to_string()
        } else {
            codeset
        }
    }

    fn locale_from_environment() -> Option<String> {
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

    fn codeset_from_locale_name() -> Option<String> {
        let locale = Self::locale_from_environment()?;
        let dot = locale.find('.')?;
        let suffix = &locale[dot + 1..];
        let end = suffix.find('@').unwrap_or(suffix.len());
        let codeset = &suffix[..end];
        if codeset.is_empty() {
            None
        } else {
            Some(codeset.to_string())
        }
    }

    fn default_codeset() -> &'static str {
        if cfg!(windows) {
            "CP1252"
        } else {
            ""
        }
    }

    fn resolve_alias(codeset: &str) -> Option<&'static str> {
        let table = Self::alias_table();
        table.binary_search_by(|entry| entry.alias.cmp(codeset))
            .ok()
            .map(|idx| table[idx].canonical)
    }

    fn alias_table() -> &'static [TableEntry] {
        const TABLE: &[TableEntry] = &[
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
                alias: "CP1252",
                canonical: "CP1252",
            },
            TableEntry {
                alias: "ISO-8859-1",
                canonical: "ISO-8859-1",
            },
            TableEntry {
                alias: "ISO8859-1",
                canonical: "ISO-8859-1",
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
        TABLE
    }
}
