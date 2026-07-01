use std::env;

pub struct SetlocaleNull;

#[derive(Clone, Debug, Eq, PartialEq)]
enum QueryError {
    InvalidCategory,
    Range,
}

impl SetlocaleNull {
    pub fn r_with_lock(category: i32, buf: &mut String, bufsize: usize) -> Result<(), i32> {
        Self::copy_locale_into_buffer(category, buf, bufsize).map_err(Self::query_error_code)
    }

    pub fn r(category: i32, buf: &mut String, bufsize: usize) -> Result<(), i32> {
        Self::r_with_lock(category, buf, bufsize)
    }

    pub fn setlocale_null(category: i32) -> Option<String> {
        let mut buf = String::new();
        match Self::r(category, &mut buf, usize::MAX) {
            Ok(()) => Some(Self::store_result(category, &buf)),
            Err(code) if code == 22 => None,
            Err(_) => Some("C".to_string()),
        }
    }

    fn query_error_code(error: QueryError) -> i32 {
        match error {
            QueryError::InvalidCategory => 22,
            QueryError::Range => 34,
        }
    }

    fn copy_locale_into_buffer(
        category: i32,
        buf: &mut String,
        bufsize: usize,
    ) -> Result<(), QueryError> {
        match Self::query_locale(category) {
            None => {
                buf.clear();
                Err(QueryError::InvalidCategory)
            }
            Some(result) => {
                if result.len() < bufsize {
                    buf.clear();
                    buf.push_str(&result);
                    Ok(())
                } else {
                    buf.clear();
                    if bufsize > 0 {
                        let truncated_len = bufsize.saturating_sub(1);
                        if truncated_len > 0 {
                            let end = Self::truncate_to_char_boundary(&result, truncated_len);
                            buf.push_str(&result[..end]);
                        }
                    }
                    Err(QueryError::Range)
                }
            }
        }
    }

    fn query_locale(category: i32) -> Option<String> {
        match category {
            0 => Some(Self::locale_all()),
            1 => Some(Self::locale_from_env("LC_COLLATE")),
            2 => Some(Self::locale_from_env("LC_CTYPE")),
            3 => Some(Self::locale_from_env("LC_MONETARY")),
            4 => Some(Self::locale_from_env("LC_NUMERIC")),
            5 => Some(Self::locale_from_env("LC_TIME")),
            6 => Some(Self::locale_messages()),
            7 => Some(Self::locale_from_env("LC_PAPER")),
            8 => Some(Self::locale_from_env("LC_NAME")),
            9 => Some(Self::locale_from_env("LC_ADDRESS")),
            10 => Some(Self::locale_from_env("LC_TELEPHONE")),
            11 => Some(Self::locale_from_env("LC_MEASUREMENT")),
            12 => Some(Self::locale_from_env("LC_IDENTIFICATION")),
            _ => None,
        }
    }

    fn locale_all() -> String {
        env::var("LC_ALL")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| "C".to_string())
    }

    fn locale_messages() -> String {
        env::var("LC_MESSAGES")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("LANGUAGE").ok().filter(|s| !s.is_empty()))
            .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| "C".to_string())
    }

    fn locale_from_env(key: &str) -> String {
        env::var("LC_ALL")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env::var(key).ok().filter(|s| !s.is_empty()))
            .or_else(|| env::var("LANG").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| "C".to_string())
    }

    fn truncate_to_char_boundary(value: &str, limit: usize) -> usize {
        if limit >= value.len() {
            value.len()
        } else {
            let mut end = limit;
            while end > 0 && !value.is_char_boundary(end) {
                end -= 1;
            }
            end
        }
    }

    fn store_result(_category: i32, value: &str) -> String {
        value.to_string()
    }
}
