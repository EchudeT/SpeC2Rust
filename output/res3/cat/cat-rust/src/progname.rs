use std::path::Path;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

pub struct Progname;

impl Progname {
    pub fn set_program_name(argv0: Option<&str>) {
        let original = argv0.unwrap_or_else(|| {
            panic!("A NULL argv[0] was passed through an exec system call.");
        });

        let derived = if let Some(base) = original.rsplit('/').next() {
            if base.as_ptr() as usize >= 7 {
                if let Some(prefix_start) = original.rfind("/.libs/") {
                    if prefix_start + 7 == original.len() - base.len() {
                        if let Some(stripped) = base.strip_prefix("lt-") {
                            stripped.to_owned()
                        } else {
                            original.to_owned()
                        }
                    } else {
                        original.to_owned()
                    }
                } else {
                    original.to_owned()
                }
            } else {
                original.to_owned()
            }
        } else {
            original.to_owned()
        };

        let normalized = if derived.is_empty() {
            Path::new(original)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| original.to_owned())
        } else {
            derived
        };

        let _ = PROGRAM_NAME.set(normalized);
    }
}
