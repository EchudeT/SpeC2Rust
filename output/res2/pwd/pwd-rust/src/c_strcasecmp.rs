use crate::c_ctype::CCtype;
use std::cmp::Ordering;

pub struct CStrcasecmp;

impl CStrcasecmp {
    pub fn compare(s1: &str, s2: &str) -> Ordering {
        let mut it1 = s1.bytes();
        let mut it2 = s2.bytes();

        loop {
            let c1 = it1.next().map(|b| CCtype::to_lower(b as char) as u32).unwrap_or(0);
            let c2 = it2.next().map(|b| CCtype::to_lower(b as char) as u32).unwrap_or(0);

            if c1 == 0 {
                return c1.cmp(&c2);
            }

            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
    }

    pub fn eq(s1: &str, s2: &str) -> bool {
        Self::compare(s1, s2) == Ordering::Equal
    }
}
