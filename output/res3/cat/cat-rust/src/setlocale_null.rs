use std::env;

pub struct SetlocaleNull;

impl SetlocaleNull {
    pub fn r_with_lock(category: i32, buf: &mut String, bufsize: usize) -> bool {
        Self::r(category, buf, bufsize)
    }

    pub fn r(category: i32, buf: &mut String, bufsize: usize) -> bool {
        match current_locale_for_category(category) {
            Some(locale) => {
                if locale.len().saturating_add(1) > bufsize {
                    return false;
                }
                buf.clear();
                buf.push_str(&locale);
                true
            }
            None => false,
        }
    }

    pub fn setlocale_null(category: i32) -> Option<String> {
        current_locale_for_category(category)
    }
}

fn current_locale_for_category(category: i32) -> Option<String> {
    let var = locale_env_var_name(category)?;
    if let Some(value) = env::var_os(var) {
        let text = value.to_string_lossy().into_owned();
        if !text.is_empty() {
            return Some(text);
        }
    }

    if let Some(value) = env::var_os("LC_ALL") {
        let text = value.to_string_lossy().into_owned();
        if !text.is_empty() {
            return Some(text);
        }
    }

    if let Some(value) = env::var_os("LANG") {
        let text = value.to_string_lossy().into_owned();
        if !text.is_empty() {
            return Some(text);
        }
    }

    Some(String::from("C"))
}

fn locale_env_var_name(category: i32) -> Option<&'static str> {
    match category {
        0 => Some("LC_CTYPE"),
        1 => Some("LC_NUMERIC"),
        2 => Some("LC_TIME"),
        3 => Some("LC_COLLATE"),
        4 => Some("LC_MONETARY"),
        5 => Some("LC_MESSAGES"),
        6 => Some("LC_ALL"),
        7 => Some("LC_PAPER"),
        8 => Some("LC_NAME"),
        9 => Some("LC_ADDRESS"),
        10 => Some("LC_TELEPHONE"),
        11 => Some("LC_MEASUREMENT"),
        12 => Some("LC_IDENTIFICATION"),
        _ => None,
    }
}
