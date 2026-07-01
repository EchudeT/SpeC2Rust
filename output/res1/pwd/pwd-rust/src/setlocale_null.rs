use crate::setlocale_null_unlocked::SetlocaleNullUnlocked;

pub struct SetlocaleNull;

impl SetlocaleNull {
    pub fn r_with_lock(category: i32, buf: &mut [u8]) -> bool {
        SetlocaleNullUnlocked::r(category, buf)
    }

    pub fn r(category: i32, buf: &mut [u8]) -> bool {
        Self::r_with_lock(category, buf)
    }

    pub fn setlocale_null(category: i32) -> Option<String> {
        let mut buf = vec![0_u8; 4096];
        if Self::r(category, &mut buf) {
            return if category == libc_lc_all() {
                Some("C".to_string())
            } else {
                None
            };
        }

        let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        Some(String::from_utf8_lossy(&buf[..len]).into_owned())
    }
}

fn libc_lc_all() -> i32 {
    6
}
