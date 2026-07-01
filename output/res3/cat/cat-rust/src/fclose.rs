use std::fs::File;
use std::io::{self, Write};

pub struct Fclose;

impl Fclose {
    pub fn nothrow(mut file: File) -> io::Result<()> {
        file.flush()?;
        file.sync_all()?;
        drop(file);
        Ok(())
    }

    pub fn rpl_fclose(mut file: File) -> io::Result<()> {
        let mut flush_error = None;

        if let Err(err) = file.flush() {
            flush_error = Some(err);
        }

        match Self::nothrow(file) {
            Ok(()) => match flush_error {
                Some(err) => Err(err),
                None => Ok(()),
            },
            Err(err) => match flush_error {
                Some(saved) => Err(saved),
                None => Err(err),
            },
        }
    }
}
