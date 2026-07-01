use std::env;
use std::path::Path;
use std::sync::OnceLock;

pub struct Getprogname;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PstStatus {
    pub pst_ucomm: String,
    pub pst_cmd: String,
}

impl Getprogname {
    pub fn program_name() -> Option<&'static str> {
        static PROGRAM_NAME: OnceLock<Option<String>> = OnceLock::new();

        PROGRAM_NAME
            .get_or_init(Self::resolve_program_name)
            .as_deref()
    }

    pub fn pst_status() -> PstStatus {
        let argv0 = env::args_os().next();
        let cmd = argv0
            .as_ref()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        let basename = argv0
            .as_ref()
            .and_then(|s| Path::new(s).file_name())
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        PstStatus {
            pst_ucomm: basename,
            pst_cmd: cmd,
        }
    }

    fn resolve_program_name() -> Option<String> {
        let status = Self::pst_status();
        Self::select_from_pst_status(&status).or_else(Self::fallback_from_args)
    }

    fn select_from_pst_status(status: &PstStatus) -> Option<String> {
        const PST_UCOMMLEN: usize = 15;

        if !status.pst_ucomm.is_empty() && status.pst_ucomm.chars().count() < PST_UCOMMLEN {
            return Some(status.pst_ucomm.clone());
        }

        if !status.pst_cmd.is_empty() {
            let cmd_head = status.pst_cmd.split(' ').next().unwrap_or_default();
            let cmd_name = Path::new(cmd_head)
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| cmd_head.to_string());

            if cmd_name.chars().count() > PST_UCOMMLEN - 1
                && status.pst_ucomm.chars().take(PST_UCOMMLEN - 1).collect::<String>()
                    == cmd_name.chars().take(PST_UCOMMLEN - 1).collect::<String>()
            {
                return Some(cmd_name);
            }
        }

        if !status.pst_ucomm.is_empty() {
            Some(status.pst_ucomm.clone())
        } else {
            None
        }
    }

    fn fallback_from_args() -> Option<String> {
        env::args_os()
            .next()
            .and_then(|arg| {
                Path::new(&arg)
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .or_else(|| {
                        let text = arg.to_string_lossy().into_owned();
                        (!text.is_empty()).then_some(text)
                    })
            })
            .or_else(|| Some("?".to_string()))
    }
}
