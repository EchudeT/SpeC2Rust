use std::sync::OnceLock;

pub struct Progname;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

impl Progname {
    pub fn set_program_name(argv0: &str) {
        let normalized = if let Some(slash) = argv0.rfind('/') {
            let base_start = slash + 1;
            if slash >= 6 && &argv0[slash - 6..base_start] == "/.libs/" {
                let base = &argv0[base_start..];
                if let Some(stripped) = base.strip_prefix("lt-") {
                    stripped.to_owned()
                } else {
                    base.to_owned()
                }
            } else {
                argv0.to_owned()
            }
        } else {
            argv0.to_owned()
        };

        let _ = PROGRAM_NAME.set(normalized);
    }
}
