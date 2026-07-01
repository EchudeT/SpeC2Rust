pub struct SetlocaleNull;

impl SetlocaleNull {
    pub fn r_with_lock(category: i32, buf: &mut [u8]) -> Result<(), i32> {
        Self::write_locale_to_buffer(category, buf)
    }

    pub fn r(category: i32, buf: &mut [u8]) -> Result<(), i32> {
        Self::r_with_lock(category, buf)
    }

    pub fn setlocale_null(category: i32) -> Option<String> {
        match Self::r(category, &mut []) {
            Ok(()) => Some(String::new()),
            Err(34) => Some("C".to_string()),
            Err(22) => None,
            Err(_) => Some("C".to_string()),
        }
        .and_then(|initial| {
            if !initial.is_empty() {
                return Some(initial);
            }

            let mut size = 16usize;
            loop {
                let mut buf = vec![0u8; size];
                match Self::r(category, &mut buf) {
                    Ok(()) => {
                        let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                        return String::from_utf8(buf[..len].to_vec()).ok();
                    }
                    Err(22) => return None,
                    Err(34) => {
                        if size >= 4096 {
                            return Some("C".to_string());
                        }
                        size *= 2;
                    }
                    Err(_) => return Some("C".to_string()),
                }
            }
        })
    }

    fn write_locale_to_buffer(category: i32, buf: &mut [u8]) -> Result<(), i32> {
        let locale = match std::env::var("LC_ALL")
            .ok()
            .filter(|v| !v.is_empty())
            .or_else(|| {
                Self::category_env_var(category)
                    .and_then(|name| std::env::var(name).ok())
                    .filter(|v| !v.is_empty())
            })
            .or_else(|| std::env::var("LANG").ok().filter(|v| !v.is_empty()))
        {
            Some(value) => value,
            None => "C".to_string(),
        };

        let bytes = locale.as_bytes();

        if buf.is_empty() {
            return if bytes.is_empty() { Ok(()) } else { Err(34) };
        }

        if bytes.len() < buf.len() {
            buf[..bytes.len()].copy_from_slice(bytes);
            buf[bytes.len()] = 0;
            Ok(())
        } else {
            let copy_len = buf.len().saturating_sub(1);
            if copy_len > 0 {
                buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            }
            buf[buf.len() - 1] = 0;
            Err(34)
        }
    }

    fn category_env_var(category: i32) -> Option<&'static str> {
        if category == libc_lc_all() {
            Some("LC_ALL")
        } else if category == libc_lc_ctype() {
            Some("LC_CTYPE")
        } else if category == libc_lc_numeric() {
            Some("LC_NUMERIC")
        } else if category == libc_lc_time() {
            Some("LC_TIME")
        } else if category == libc_lc_collate() {
            Some("LC_COLLATE")
        } else if category == libc_lc_monetary() {
            Some("LC_MONETARY")
        } else if category == libc_lc_messages() {
            Some("LC_MESSAGES")
        } else {
            None
        }
    }
}

const fn libc_lc_all() -> i32 {
    6
}

const fn libc_lc_collate() -> i32 {
    3
}

const fn libc_lc_ctype() -> i32 {
    0
}

const fn libc_lc_monetary() -> i32 {
    4
}

const fn libc_lc_numeric() -> i32 {
    1
}

const fn libc_lc_time() -> i32 {
    2
}

const fn libc_lc_messages() -> i32 {
    5
}
