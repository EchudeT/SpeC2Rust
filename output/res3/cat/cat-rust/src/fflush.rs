use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SeekOptimizationState {
    pub enabled: bool,
    pub no_position_optimization: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FlushState {
    pub reading: bool,
    pub has_ungetc_data: bool,
    pub seek_optimization: SeekOptimizationState,
    pub cached_position: Option<u64>,
}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S>(
        stream: &mut S,
        state: &mut FlushState,
    ) -> io::Result<()>
    where
        S: Seek,
    {
        if state.has_ungetc_data {
            let _ = stream.stream_position()?;
            state.has_ungetc_data = false;
        }
        Ok(())
    }

    pub fn clear_ungetc_buffer<S>(stream: &mut S, state: &mut FlushState) -> io::Result<()>
    where
        S: Seek,
    {
        if state.has_ungetc_data {
            let _ = stream.stream_position()?;
            state.has_ungetc_data = false;
        }
        Ok(())
    }

    pub fn disable_seek_optimization(state: &mut FlushState) -> SeekOptimizationState {
        let saved = state.seek_optimization;
        state.seek_optimization.enabled = false;
        state.seek_optimization.no_position_optimization = true;
        saved
    }

    pub fn restore_seek_optimization(
        state: &mut FlushState,
        saved_flags: SeekOptimizationState,
    ) {
        state.seek_optimization = saved_flags;
    }

    pub fn update_fpos_cache(state: &mut FlushState, pos: u64) {
        state.cached_position = Some(pos);
    }

    pub fn rpl_fflush<S>(stream: Option<(&mut S, &mut FlushState)>) -> io::Result<()>
    where
        S: Read + Write + Seek,
    {
        match stream {
            None => Ok(()),
            Some((stream, state)) => {
                if !state.reading {
                    return stream.flush();
                }

                Self::clear_ungetc_buffer_preserving_position(stream, state)?;
                let pos = stream.stream_position().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "cannot flush a read-buffered non-seekable stream",
                    )
                })?;

                Self::clear_ungetc_buffer(stream, state)?;
                stream.seek(SeekFrom::Start(pos))?;
                Self::update_fpos_cache(state, pos);

                Ok(())
            }
        }
    }

    pub fn main_root_clear_ungetc_08<S>(stream: &mut S, state: &mut FlushState) -> io::Result<()>
    where
        S: Seek,
    {
        Self::clear_ungetc_buffer_preserving_position(stream, state)?;
        Self::clear_ungetc_buffer(stream, state)
    }
}
