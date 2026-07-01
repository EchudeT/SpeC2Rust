use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;

/// Lightweight proper-name selection helper.
///
/// This module provides the Rust-style equivalent of gnulib's
/// `proper_name_lite`, choosing among:
/// - a translated ASCII name, if one exists;
/// - otherwise the UTF-8 spelling in UTF-8 locales;
/// - otherwise the original ASCII spelling.
pub struct PropernameLite;

impl PropernameLite {
    /// Returns the best display form for a proper name.
    ///
    /// `name_ascii` is the untranslated ASCII identifier and fallback form.
    /// `name_utf8` is the preferred UTF-8 spelling used when no translation
    /// exists and the current locale charset is UTF-8.
    ///
    /// Since this Rust rewrite does not expose gettext integration from this
    /// module, callers may optionally provide a translation closure. If the
    /// closure returns `Some(translated)`, that translation is preferred.
    /// Otherwise the locale-based ASCII/UTF-8 choice is used.
    pub fn choose<'a>(
        name_ascii: &'a str,
        name_utf8: &'a str,
        translate: Option<impl FnOnce(&str) -> Option<&'a str>>,
    ) -> &'a str {
        if let Some(translated) = translate.and_then(|f| f(name_ascii)) {
            return translated;
        }

        if CStrcasecmp::compare(&Localcharset::locale_charset(), "UTF-8") == 0 {
            name_utf8
        } else {
            name_ascii
        }
    }
}
