use std::fs::File;
use std::io::{self, Seek};

pub struct Fclose;

impl Fclose {
    pub fn nothrow(file: File) -> io::Result<()> {
        file.sync_all()
    }

    pub fn rpl_fclose(file: File, had_prior_error: bool) -> io::Result<()> {
        let close_result = if !had_prior_error {
            let mut file = file;
            let seekable = file.stream_position().is_ok();
            if seekable && let Err(err) = io::Write::flush(&mut file) {
                let _ = Self::nothrow(file);
                return Err(err);
            }
            Self::nothrow(file)
        } else {
            Self::nothrow(file)
        };

        match (had_prior_error, close_result) {
            (_, Err(err)) => Err(err),
            (true, Ok(())) => Err(io::Error::other("stream had a prior error before close")),
            (false, Ok(())) => Ok(()),
        }
    }
}
