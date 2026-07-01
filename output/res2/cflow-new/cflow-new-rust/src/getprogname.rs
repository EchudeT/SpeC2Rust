use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::sync::OnceLock;

pub struct Getprogname;

impl Getprogname {
    pub fn program_name() -> Option<String> {
        static PROGRAM_NAME: OnceLock<Option<String>> = OnceLock::new();

        PROGRAM_NAME.get_or_init(Self::detect_program_name).clone()
    }

    pub fn pst_status() -> Option<String> {
        let arg0 = Self::args_os()?;
        let path = Path::new(&arg0);
        let name = path.file_name().unwrap_or(path.as_os_str());

        if name.is_empty() {
            None
        } else {
            Some(name.to_string_lossy().into_owned())
        }
    }

    pub fn args_os() -> Option<OsString> {
        env::args_os().next().filter(|arg| !arg.is_empty())
    }

    fn detect_program_name() -> Option<String> {
        Self::pst_status().or_else(|| Some("?".to_string()))
    }
}
