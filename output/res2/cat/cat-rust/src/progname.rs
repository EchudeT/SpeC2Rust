#[derive(Debug, Clone, Default)]
pub struct Progname {
    pub program_name: String,
}


impl Progname {
    pub fn set_program_name(argv0: Option<&str>) -> Self {
        let original = match argv0 {
            Some(value) => value,
            None => panic!("A NULL argv[0] was passed through an exec system call."),
        };

        let mut stored = original;

        if let Some(base_start) = original.rfind('/').map(|index| index + 1) {
            let base = &original[base_start..];
            if base_start >= 7 && &original[base_start - 7..base_start] == "/.libs/" {
                stored = if let Some(stripped) = base.strip_prefix("lt-") {
                    stripped
                } else {
                    base
                };
            }
        }


        Self {
            program_name: stored.to_owned(),
        }
    }
}
