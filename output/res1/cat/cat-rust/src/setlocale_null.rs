const LC_ALL: i32 = 6;
const LC_CTYPE: i32 = 0;
const LC_NUMERIC: i32 = 1;
const LC_TIME: i32 = 2;
const LC_COLLATE: i32 = 3;
const LC_MONETARY: i32 = 4;
const LC_MESSAGES: i32 = 5;

pub struct SetlocaleNull;

impl SetlocaleNull {

    fn query_locale(category: i32) -> Option<String> {
        match category {
            LC_ALL => Some(Self::compose_all_locale()),
            LC_CTYPE => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_CTYPE", "LANG"])),
            LC_NUMERIC => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_NUMERIC", "LANG"])),
            LC_TIME => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_TIME", "LANG"])),
            LC_COLLATE => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_COLLATE", "LANG"])),
            LC_MONETARY => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_MONETARY", "LANG"])),
            LC_MESSAGES => Some(Self::locale_from_env_chain(&["LC_ALL", "LC_MESSAGES", "LANG"]))
            _ => None,
        }
    }

    fn locale_from_env_chain(keys: &[&str]) -> String {
        keys.iter()
            .find_map(|key| std::env::var(key).ok().filter(|value| !value.is_empty()))
            .unwrap_or_else(|| "C".to_string())
    }

    fn compose_all_locale() -> String {
        let all = std::env::var("LC_ALL").ok().filter(|value| !value.is_empty());
        if let Some(value) = all {
            return value;
        }

        let lang = std::env::var("LANG").ok().filter(|value| !value.is_empty());
        let fallback = lang.unwrap_or_else(|| "C".to_string());

        let categories = [
            ("LC_CTYPE", Self::locale_from_env_chain(&["LC_CTYPE", "LANG"])),
            ("LC_NUMERIC", Self::locale_from_env_chain(&["LC_NUMERIC", "LANG"])),
            ("LC_TIME", Self::locale_from_env_chain(&["LC_TIME", "LANG"])),
            ("LC_COLLATE", Self::locale_from_env_chain(&["LC_COLLATE", "LANG"])),
            ("LC_MONETARY", Self::locale_from_env_chain(&["LC_MONETARY", "LANG"])),
            ("LC_MESSAGES", Self::locale_from_env_chain(&["LC_MESSAGES", "LANG"])),
        ];

        if categories.iter().all(|(_, value)| value == &fallback) {
            fallback
        } else {
            categories
                .into_iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join(";")
        }
    }

    pub fn r_with_lock(category: i32, buf: &mut [u8]) -> bool {
        Self::copy_locale_into_buffer(category, buf)
    }

    pub fn r(category: i32, buf: &mut [u8]) -> bool {
        Self::r_with_lock(category, buf)
    }

    pub fn setlocale_null(category: i32) -> Option<String> {
        Self::query_locale(category)
    }

    fn copy_locale_into_buffer(category: i32, buf: &mut [u8]) -> bool {
        let Some(locale) = Self::query_locale(category) else {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return false;
        };

        let bytes = locale.as_bytes();
        if bytes.len() < buf.len() {
            if !bytes.is_empty() {
                buf[..bytes.len()].copy_from_slice(bytes);
            }
            if !buf.is_empty() {
                buf[bytes.len()] = 0;
            }
            true
        } else {
            if !buf.is_empty() {
                let copy_len = buf.len().saturating_sub(1).min(bytes.len());
                if copy_len > 0 {
                    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
                }
                buf[buf.len() - 1] = 0;
            }
            false
        }
    }
}
