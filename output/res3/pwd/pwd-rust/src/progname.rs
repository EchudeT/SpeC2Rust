use std::path::Path;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

pub struct Progname;

impl Progname {
    pub fn set_program_name(argv0: &str) {
        let normalized = if argv0.is_empty() {
            String::new()
        } else {
            Self::normalize_program_name(argv0)
        };

        let _ = PROGRAM_NAME.set(normalized);
    }

    fn normalize_program_name(argv0: &str) -> String {
        let path = Path::new(argv0);
        let slash_index = argv0.rfind('/');
        let base = slash_index.map_or(argv0, |idx| &argv0[idx + 1..]);

        if let Some(base_start) = slash_index.map(|idx| idx + 1) {
            if base_start >= 7 && &argv0[base_start - 7..base_start] == "/.libs/" {
                if let Some(stripped) = base.strip_prefix("lt-") {
                    return stripped.to_string();
                }
                return base.to_string();
            }
        }

        path.to_string_lossy().into_owned()
    }
}
