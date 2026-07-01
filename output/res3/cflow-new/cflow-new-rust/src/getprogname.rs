use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;

pub struct Getprogname;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PstStatus {
    pub pst_ucomm: OsString,
    pub pst_cmd: OsString,
}

impl Getprogname {
    pub fn program_name() -> Option<String> {
        let arg0 = Self::args_os()?;
        let status = Self::pst_status(arg0);
        let chosen = Self::select_name(&status);
        let text = chosen.to_string_lossy().into_owned();
        if text.is_empty() { None } else { Some(text) }
    }

    pub fn pst_status(program: OsString) -> PstStatus {
        let basename = Self::basename_os(&program)
            .map(OsStr::to_os_string)
            .unwrap_or_else(|| program.clone());
        PstStatus {
            pst_ucomm: basename,
            pst_cmd: program,
        }
    }

    pub fn args_os() -> Option<OsString> {
        env::args_os().next()
    }

    pub fn set_program_name(argv0: &str) {
        let _ = argv0;
    }

    fn select_name(status: &PstStatus) -> &OsStr {
        const PST_UCOMMLEN: usize = 15;

        let ucomm = status.pst_ucomm.as_os_str();
        let cmd = status.pst_cmd.as_os_str();

        let ucomm_len = ucomm.to_string_lossy().chars().count();
        if ucomm_len < PST_UCOMMLEN - 1 {
            return ucomm;
        }

        let cmd_first_word = Self::first_word(cmd);
        let cmd_base = Self::basename_os(cmd_first_word).unwrap_or(cmd_first_word);

        let cmd_base_text = cmd_base.to_string_lossy();
        let ucomm_text = ucomm.to_string_lossy();

        if cmd_base_text.chars().count() > PST_UCOMMLEN - 1
            && cmd_base_text
                .chars()
                .take(PST_UCOMMLEN - 1)
                .eq(ucomm_text.chars().take(PST_UCOMMLEN - 1))
        {
            cmd_base
        } else {
            ucomm
        }
    }

    fn basename_os(value: &OsStr) -> Option<&OsStr> {
        Path::new(value).file_name()
    }

    fn first_word(value: &OsStr) -> &OsStr {
        let path = Path::new(value);
        if let Some(first) = path.iter().next() {
            first
        } else {
            value
        }
    }
}
