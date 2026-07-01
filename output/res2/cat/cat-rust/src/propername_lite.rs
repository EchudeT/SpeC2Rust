use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;

pub struct PropernameLite;

impl PropernameLite {
    pub fn choose(name_ascii: &str, name_utf8: &str) -> String {
        let translation = Self::gettext(name_ascii);

        if translation != name_ascii {
            translation
        } else if CStrcasecmp::eq_ignore_case(&Localcharset::locale_charset(), "UTF-8") {
            name_utf8.to_owned()
        } else {
            name_ascii.to_owned()
        }
    }

    fn gettext(message: &str) -> String {
        message.to_owned()
    }
}
