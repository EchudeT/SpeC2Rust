use std::sync::OnceLock;

/// Rust-style compatibility API for determining the process descriptor table size.
pub struct Getdtablesize;

impl Getdtablesize {
    /// Non-throwing compatibility helper corresponding to the C module's
    /// `_setmaxstdio_nothrow` behavior.
    ///
    /// In this Rust port, there is no direct safe standard-library equivalent
    /// for mutating the process stdio table bound, so this helper models the
    /// observable contract needed by the C logic:
    /// it reports whether the requested bound is plausibly accepted.
    ///
    /// The original Windows-oriented code searched downward from 65536 until
    /// this helper stopped failing. Modeling that behavior safely is sufficient
    /// for the Rust-facing API here.
    pub fn setmaxstdio_nothrow(newmax: usize) -> Option<usize> {
        if newmax == 0 {
            return None;
        }

        const MAX_ACCEPTED: usize = 0x800;

        if newmax <= MAX_ACCEPTED {
            Some(newmax)
        } else {
            None
        }
    }

    /// Returns a descriptor-table-style capacity for the current
    /// process.
    ///
    /// On Unix platforms this uses the runtime soft limit when available.
    /// On other platforms it falls back to the compatibility logic evidenced
    /// by the source C module.
    pub fn descriptor_table_size() -> usize {
        #[cfg(unix)]
        {
            use std::process::Command;

            fn parse_limit(text: &str) -> Option<usize> {
                let trimmed = text.trim();
                if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("unlimited") {
                    return None;
                }
                trimmed.parse::<usize>().ok()
            }

            let output = Command::new("sh").arg("-c").arg("ulimit -n").output();

            if let Ok(output) = output {
                if output.status.success() {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        if let Some(limit) = parse_limit(&stdout) {
                            return limit;
                        }
                    }
                }
            }

            i32::MAX as usize
        }

        #[cfg(not(unix))]
        {
            static DTABLESIZE: OnceLock<usize> = OnceLock::new();

            *DTABLESIZE.get_or_init(|| {
                let orig_max_stdio = 512usize;
                let mut bound = 0x10000usize;

                while Self::setmaxstdio_nothrow(bound).is_none() && bound > 1 {
                    bound /= 2;
                }

                let _ = Self::setmaxstdio_nothrow(orig_max_stdio);
                bound
            })
        }
    }
}
