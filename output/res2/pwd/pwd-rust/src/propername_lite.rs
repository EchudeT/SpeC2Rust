use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;

pub struct PropernameLite;

impl PropernameLite {
    pub fn translate<'a>(name_ascii: &'a str, name_utf8: &'a str) -> &'a str {
        if let Some(translation) = Self::gettext(name_ascii) {
            return translation;
        }

        if CStrcasecmp::eq(&Localcharset::locale_charset(), "UTF-8") {
            name_utf8
        } else {
            name_ascii
        }
    }

    fn gettext(_message: &str) -> Option<&str> {
        None
    }
}
