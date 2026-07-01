use std::io::{self, Seek, SeekFrom};

use crate::fflush::{Fflush, FlushState};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S>(
        stream: &mut S,
        state: &mut FlushState,
        offset: i64,
        whence: SeekFrom,
    ) -> io::Result<u64>
    where
        S: Seek,
    {
        let no_pending_buffers = !state.has_ungetc_data && !state.reading;

        if no_pending_buffers {
            let target = match whence {
                SeekFrom::Start(_) => {
                    let absolute = u64::try_from(offset).map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "negative offset is invalid for SeekFrom::Start",
                        )
                    })?;
                    SeekFrom::Start(absolute)
                }
                SeekFrom::Current(_) => SeekFrom::Current(offset),
                SeekFrom::End(_) => SeekFrom::End(offset),
            };

            let pos = stream.seek(target)?;
            state.cached_position = Some(pos);
            return Ok(pos);
        }

        Fflush::clear_ungetc_buffer_preserving_position(stream, state)?;

        let target = match whence {
            SeekFrom::Start(_) => {
                let absolute = u64::try_from(offset).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "negative offset is invalid for SeekFrom::Start",
                    )
                })?;
                SeekFrom::Start(absolute)
            }
            SeekFrom::Current(_) => SeekFrom::Current(offset),
            SeekFrom::End(_) => SeekFrom::End(offset),
        };

        let pos = stream.seek(target)?;
        state.reading = false;
        state.has_ungetc_data = false;
        state.cached_position = Some(pos);
        Ok(pos)
    }

    pub fn seek_from_start<S>(
        stream: &mut S,
        state: &mut FlushState,
        offset: u64,
    ) -> io::Result<u64>
    where
        S: Seek,
    {
        Self::seek(stream, state, offset as i64, SeekFrom::Start(0))
    }

    pub fn seek_from_current<S>(
        stream: &mut S,
        state: &mut FlushState,
        offset: i64,
    ) -> io::Result<u64>
    where
        S: Seek,
    {
        Self::seek(stream, state, offset, SeekFrom::Current(0))
    }

    pub fn seek_from_end<S>(
        stream: &mut S,
        state: &mut FlushState,
        offset: i64,
    ) -> io::Result<u64>
    where
        S: Seek,
    {
        Self::seek(stream, state, offset, SeekFrom::End(0))
    }
}
