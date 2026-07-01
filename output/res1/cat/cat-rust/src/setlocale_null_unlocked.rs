use std::env;

pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn query(category: i32) -> Option<String> {
        let result = Self::locale_for_category(category);

        if result.is_none() && cfg!(target_os = "android") && Self::android_fallback_category(category)
        {
            return Some("C".to_string());
        }

        result
    }

    fn locale_for_category(category: i32) -> Option<String> {
        crate::setlocale_null::SetlocaleNull::setlocale_null(category)
    }

    fn android_fallback_category(category: i32) -> bool {
        const LC_CTYPE: i32 = 0;
        const LC_NUMERIC: i32 = 1;
        const LC_TIME: i32 = 2;
        const LC_COLLATE: i32 = 3;
        const LC_MONETARY: i32 = 4;
        const LC_MESSAGES: i32 = 5;
        const LC_ALL: i32 = 6;
        const LC_PAPER: i32 = 7;
        const LC_NAME: i32 = 8;
        const LC_ADDRESS: i32 = 9;
        const LC_TELEPHONE: i32 = 10;
        const LC_MEASUREMENT: i32 = 11;

        matches!(
            category,
            LC_CTYPE
                | LC_NUMERIC
                | LC_TIME
                | LC_COLLATE
                | LC_MONETARY
                | LC_MESSAGES
                | LC_ALL
                | LC_PAPER
                | LC_NAME
                | LC_ADDRESS
                | LC_TELEPHONE
                | LC_MEASUREMENT
        )
    }

    pub fn current_from_env() -> Option<String> {
        env::var("LC_ALL")
            .ok()
            .filter(|v| !v.is_empty())
            .or_else(|| env::var("LANG").ok().filter(|v| !v.is_empty()))
    }
}
