use std::path::PathBuf;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

fn program_name_cell() -> &'static OnceLock<String> {
    &PROGRAM_NAME
}

fn normalize_program_name(argv0: &str) -> String {
    let path = PathBuf::from(argv0);

    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        if let Some(parent) = path.parent() {
            let parent_text = parent.to_string_lossy();
            if parent_text.ends_with("/.libs") || parent_text == ".libs" {
                if let Some(stripped) = file_name.strip_prefix("lt-") {
                    return stripped.to_string();
                }
            }
        }
    }

    argv0.to_string()
}

pub struct Progname;

impl Progname {
    pub fn set_program_name(argv0: Option<&str>) {
        let argv0 =
            argv0.unwrap_or_else(|| panic!("A NULL argv[0] was passed through an exec system call."));

        let normalized = normalize_program_name(argv0);
        let _ = program_name_cell().set(normalized);
    }
}
