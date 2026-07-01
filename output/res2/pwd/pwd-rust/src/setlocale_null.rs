use std::env;

pub struct SetlocaleNull;

impl SetlocaleNull {
    pub fn r(category: i32, buf: &mut [u8]) -> bool {
        Self::r_unlocked(category, buf)
    }


    pub fn setlocale_null(category: i32) -> Option<String> {
        let mut buf = vec![0u8; 4096];
        if Self::r(category, &mut buf) {
            let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            Some(String::from_utf8_lossy(&buf[..len]).into_owned())
        } else {
            None
        }
    }

    fn r_unlocked(category: i32, buf: &mut [u8]) -> bool {
        let locale = match Self::locale_for_category(category) {
            Some(value) => value,
            None => {
                if let Some(first) = buf.first_mut() {
                    *first = 0;
                }
                return false;
            }
        };

        let bytes = locale.as_bytes();
        if bytes.len() < buf.len() {
            if !bytes.is_empty() {
                buf[..bytes.len()].copy_from_slice(bytes);
            }
            buf[bytes.len()] = 0;
            true
        } else {
            if !buf.is_empty() {
                let copy_len = buf.len().saturating_sub(1);
                if copy_len > 0 {
                    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
                }
                buf[buf.len() - 1] = 0;
            }
            false
        }
    }

    fn locale_for_category(category: i32) -> Option<String> {
        if category == 6 {
            return Some(Self::current_locale_all());
        }

        let candidates = match category {
            0 => &["LC_CTYPE", "LANG"][..],
            1 => &["LC_NUMERIC", "LANG"][..],
            2 => &["LC_TIME", "LANG"][..],
            3 => &["LC_COLLATE", "LANG"][..],
            4 => &["LC_MONETARY", "LANG"][..],
            5 => &["LC_MESSAGES", "LANG"][..],
            7 => &["LC_PAPER", "LANG"][..],
            8 => &["LC_NAME", "LANG"][..],
            9 => &["LC_ADDRESS", "LANG"][..],
            10 => &["LC_TELEPHONE", "LANG"][..],
            11 => &["LC_MEASUREMENT", "LANG"][..],
            12 => &["LC_IDENTIFICATION", "LANG"][..],
            _ => return None,
        };

        Some(
            candidates
                .iter()
                .find_map(|name| env::var(name).ok().filter(|v| !v.is_empty()))
                .unwrap_or_else(|| "C".to_string()),
        )
    }

    fn current_locale_all() -> String {
        for name in ["LC_ALL", "LANG"] {
            if let Ok(value) = env::var(name) {
                if !value.is_empty() {
                    return value;
                }
            }
        }
        "C".to_string()
    }
}
