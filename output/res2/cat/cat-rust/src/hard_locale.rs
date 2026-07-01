use crate::setlocale_null::SetlocaleNull;

pub struct HardLocale;

impl HardLocale {
    pub fn is_hard(category: i32) -> bool {
        let Some(locale) = SetlocaleNull::setlocale_null(category) else {
            return false;
        };

        if locale != "C" && locale != "POSIX" {
            return true;
        }

        #[cfg(target_os = "android")]
        {
            if std::env::var_os("LC_ALL").is_some()
                || std::env::var_os("LC_CTYPE").is_some()
                || std::env::var_os("LANG").is_some()
            {
                return true;
            }
        }

        false
    }
}
