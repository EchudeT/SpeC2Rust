use std::env;

pub struct Localcharset;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableEntry {
    pub source: &'static str,
    pub canonical: &'static str,
}

impl Localcharset {
    pub fn table_entry(source: &'static str, canonical: &'static str) -> TableEntry {
        TableEntry { source, canonical }
    }

    pub fn locale_charset() -> String {
        let locale = Self::current_locale();
        let mut codeset = Self::codeset_from_locale(&locale)
            .map(Self::normalize_codeset_name)
            .unwrap_or_else(|| {
                if cfg!(any(target_os = "macos", target_os = "ios")) {
                    "UTF-8".to_string()
                } else {
                    "ASCII".to_string()
                }
            });

        if codeset.is_empty() {
            codeset = if cfg!(any(target_os = "macos", target_os = "ios")) {
                "UTF-8".to_string()
            } else {
                "ASCII".to_string()
            };
        }

        if cfg!(any(target_os = "macos", target_os = "ios")) && codeset.eq_ignore_ascii_case("utf-8")
        {
            return "UTF-8".to_string();
        }

        codeset
    }

    fn current_locale() -> String {
        for key in ["LC_ALL", "LC_CTYPE", "LANG"] {
            if let Ok(value) = env::var(key) {
                if !value.is_empty() {
                    return value;
                }
            }
        }
        String::new()
    }

    fn codeset_from_locale(locale: &str) -> Option<String> {
        if locale.is_empty() {
            return None;
        }

        if locale == "C" || locale == "POSIX" {
            return Some(String::from("ASCII"));
        }

        if let Some(dot) = locale.rfind('.') {
            let suffix = &locale[dot + 1..];
            let encoding = suffix.split('@').next().unwrap_or("");
            if !encoding.is_empty() {
                return Some(encoding.to_string());
            }
        }

        Self::lookup_locale_table(locale).map(str::to_string)
    }

    fn normalize_codeset_name(codeset: String) -> String {
        if let Some(mapped) = Self::lookup_alias_table(&codeset) {
            return mapped.to_string();
        }

        if codeset.eq_ignore_ascii_case("utf8") || codeset.eq_ignore_ascii_case("65001") {
            return "UTF-8".to_string();
        }

        if codeset.is_empty() {
            "ASCII".to_string()
        } else {
            codeset
        }
    }

    fn lookup_alias_table(codeset: &str) -> Option<&'static str> {
        let normalized = codeset;
        Self::alias_table()
            .binary_search_by(|entry| entry.source.cmp(normalized))
            .ok()
            .map(|idx| Self::alias_table()[idx].canonical)
    }

    fn lookup_locale_table(locale: &str) -> Option<&'static str> {
        Self::locale_table()
            .binary_search_by(|entry| entry.source.cmp(locale))
            .ok()
            .map(|idx| Self::locale_table()[idx].canonical)
    }

    fn alias_table() -> &'static [TableEntry] {
        &[
            TableEntry {
                source: "ANSI_X3.4-1968",
                canonical: "ASCII",
            },
            TableEntry {
                source: "ANSI_X3.4-1986",
                canonical: "ASCII",
            },
            TableEntry {
                source: "ASCII",
                canonical: "ASCII",
            },
            TableEntry {
                source: "CP1250",
                canonical: "CP1250",
            },
            TableEntry {
                source: "CP1251",
                canonical: "CP1251",
            },
            TableEntry {
                source: "CP1252",
                canonical: "CP1252",
            },
            TableEntry {
                source: "CP1253",
                canonical: "CP1253",
            },
            TableEntry {
                source: "CP1254",
                canonical: "CP1254",
            },
            TableEntry {
                source: "CP1255",
                canonical: "CP1255",
            },
            TableEntry {
                source: "CP1256",
                canonical: "CP1256",
            },
            TableEntry {
                source: "CP1257",
                canonical: "CP1257",
            },
            TableEntry {
                source: "CP1258",
                canonical: "CP1258",
            },
            TableEntry {
                source: "EUC-JP",
                canonical: "EUC-JP",
            },
            TableEntry {
                source: "EUC-KR",
                canonical: "EUC-KR",
            },
            TableEntry {
                source: "GB18030",
                canonical: "GB18030",
            },
            TableEntry {
                source: "GB2312",
                canonical: "GB2312",
            },
            TableEntry {
                source: "ISO-8859-1",
                canonical: "ISO-8859-1",
            },
            TableEntry {
                source: "ISO-8859-15",
                canonical: "ISO-8859-15",
            },
            TableEntry {
                source: "ISO-8859-2",
                canonical: "ISO-8859-2",
            },
            TableEntry {
                source: "ISO-8859-5",
                canonical: "ISO-8859-5",
            },
            TableEntry {
                source: "ISO-8859-9",
                canonical: "ISO-8859-9",
            },
            TableEntry {
                source: "KOI8-R",
                canonical: "KOI8-R",
            },
            TableEntry {
                source: "KOI8-U",
                canonical: "KOI8-U",
            },
            TableEntry {
                source: "SHIFT_JIS",
                canonical: "SHIFT_JIS",
            },
            TableEntry {
                source: "US-ASCII",
                canonical: "ASCII",
            },
            TableEntry {
                source: "UTF-8",
                canonical: "UTF-8",
            },
            TableEntry {
                source: "UTF8",
                canonical: "UTF-8",
            },
        ]
    }

    fn locale_table() -> &'static [TableEntry] {
        &[
            TableEntry {
                source: "C",
                canonical: "ASCII",
            },
            TableEntry {
                source: "POSIX",
                canonical: "ASCII",
            },
        ]
    }
}
