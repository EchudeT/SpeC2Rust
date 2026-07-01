use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::sync::OnceLock;

fn program_name_cell() -> &'static OnceLock<OsString> {
    static PROGRAM_NAME: OnceLock<OsString> = OnceLock::new();
    &PROGRAM_NAME
}

fn normalize_program_name(argv0: &OsStr) -> OsString {
    let path = Path::new(argv0);

    let slash_stripped = path
        .file_name()
        .map(OsStr::to_os_string)
        .unwrap_or_else(|| argv0.to_os_string());

    let full_lossy = argv0.to_string_lossy();
    if full_lossy.contains("/.libs/") {
        let short_lossy = slash_stripped.to_string_lossy();
        if let Some(stripped) = short_lossy.strip_prefix("lt-") {
            return OsString::from(stripped);
        }
        return slash_stripped;
    }

    argv0.to_os_string()
}

pub struct Progname;

impl Progname {
    pub fn set_program_name(argv0: &OsStr) {
        let normalized = normalize_program_name(argv0);
        let _ = program_name_cell().set(normalized);
    }

    pub fn set_program_name_from_env() {
        if let Some(argv0) = env::args_os().next() {
            Self::set_program_name(&argv0);
        } else {
            eprintln!("A NULL argv[0] was passed through an exec system call.");
            panic!("missing argv[0]");
        }
    }
}
