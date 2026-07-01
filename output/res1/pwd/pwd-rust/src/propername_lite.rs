use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;

pub struct PropernameLite;

impl PropernameLite {
    pub fn choose<'a>(name_ascii: &'a str, name_utf8: &'a str) -> &'a str {
        let translation = name_ascii;
        if translation != name_ascii {
            translation
        } else if CStrcasecmp::eq(Localcharset::locale_charset(), "UTF-8") {
            name_utf8
        } else {
            name_ascii
        }
    }
}
