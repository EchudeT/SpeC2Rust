use std::sync::OnceLock;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ProgramNameState {
    full_name: String,
    short_name: String,
}

fn state() -> &'static OnceLock<ProgramNameState> {
    static STATE: OnceLock<ProgramNameState> = OnceLock::new();
    &STATE
}

pub struct Progname;

impl Progname {
    pub fn set_program_name(argv0: Option<&str>) {
        let argv0 = match argv0 {
            Some(value) if !value.is_empty() => value,
            _ => panic!("A NULL argv[0] was passed through an exec system call."),
        };

        let slash_pos = argv0.rfind('/');
        let base = slash_pos.map_or(argv0, |idx| &argv0[idx + 1..]);

        let effective = if base.as_ptr() as usize >= argv0.as_ptr() as usize + 7 {
            let base_start = base.as_ptr() as usize - argv0.as_ptr() as usize;
            if base_start >= 7 && &argv0[base_start - 7..base_start] == "/.libs/" {
                base.strip_prefix("lt-").unwrap_or(base)
            } else {
                argv0
            }
        } else {
            argv0
        };

        let short_name = base.strip_prefix("lt-").unwrap_or(base).to_string();
        let full_name = effective.to_string();

        let _ = state().set(ProgramNameState {
            full_name,
            short_name,
        });
    }
}
