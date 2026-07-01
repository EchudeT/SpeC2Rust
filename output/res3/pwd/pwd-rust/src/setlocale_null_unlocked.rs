pub struct SetlocaleNullUnlocked;

impl SetlocaleNullUnlocked {
    pub fn query(category: i32) -> Option<String> {
        let result = crate::setlocale_null::SetlocaleNull::setlocale_null(category);

        #[cfg(target_os = "android")]
        {
            if result.is_none() && Self::is_android_supported_category(category) {
                return Some(String::from("C"));
            }
        }

        result
    }

    #[cfg(target_os = "android")]
    fn is_android_supported_category(category: i32) -> bool {
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
}
