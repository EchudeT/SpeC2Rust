use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Progname;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

impl Progname {
    pub fn set_program_name(argv0: Option<&str>) {
        let raw = match argv0 {
            Some(value) => value,
            None => panic!("A NULL argv[0] was passed through an exec system call."),
        };

        let effective = Self::normalize_program_name(raw);

        let _ = PROGRAM_NAME.set(effective);
    }

    fn normalize_program_name(argv0: &str) -> String {
        let path = Path::new(argv0);
        let base = path
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .unwrap_or(argv0);

        let has_libs_prefix = argv0
            .rfind(base)
            .map(|base_start| {
                base_start >= 7 && &argv0[base_start - 7..base_start] == "/.libs/"
            })
            .unwrap_or(false);

        if has_libs_prefix {
            base.strip_prefix("lt-").unwrap_or(base).to_owned()
        } else {
            argv0.to_owned()
        }
    }
}
