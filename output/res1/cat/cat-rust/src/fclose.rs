use std::fs::File;
use std::io::{self, Seek, Write};

pub struct Fclose;

impl Fclose {
    pub fn nothrow(file: File) -> io::Result<()> {
        file.sync_all()
    }

    pub fn rpl_fclose(mut file: File) -> io::Result<()> {
        let flush_result = file.flush();
        let close_result = Self::nothrow(file);

        match (flush_result, close_result) {
            (Err(err), _) => Err(err),
            (Ok(()), Err(err)) => Err(err),
            (Ok(()), Ok(())) => Ok(()),
        }
    }
}
