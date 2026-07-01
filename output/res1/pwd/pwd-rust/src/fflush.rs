use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SeekOptimizationState {
    disabled: bool,
}

pub trait FlushStream: Read + Write + Seek {
    fn is_reading(&self) -> bool {
        true
    }

    fn has_ungetc_buffer(&self) -> bool {
        false
    }

    fn clear_ungetc_state(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn purge_read_buffer(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn update_cached_position(&mut self, _pos: u64) {}

    fn seek_optimization_enabled(&self) -> bool {
        false
    }

    fn set_seek_optimization_enabled(&mut self, _enabled: bool) {}

    fn flush_output(&mut self) -> io::Result<()> {
        self.flush()
    }
}

pub enum FlushTarget<'a, S: FlushStream + ?Sized> {
    All,
    Stream(&'a mut S),
}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S: FlushStream + ?Sized>(
        stream: &mut S,
    ) -> io::Result<()> {
        if stream.has_ungetc_buffer() {
            let pos = stream.stream_position()?;
            stream.clear_ungetc_state()?;
            stream.seek(SeekFrom::Start(pos))?;
        }
        Ok(())
    }

    pub fn clear_ungetc_buffer<S: FlushStream + ?Sized>(stream: &mut S) -> io::Result<()> {
        if stream.has_ungetc_buffer() {
            stream.clear_ungetc_state()?;
        } else {
            let pos = stream.stream_position()?;
            stream.seek(SeekFrom::Start(pos))?;
        }
        Ok(())
    }

    pub fn disable_seek_optimization<S: FlushStream + ?Sized>(
        stream: &mut S,
    ) -> SeekOptimizationState {
        let was_enabled = stream.seek_optimization_enabled();
        stream.set_seek_optimization_enabled(false);
        SeekOptimizationState {
            disabled: was_enabled,
        }
    }

    pub fn restore_seek_optimization<S: FlushStream + ?Sized>(
        stream: &mut S,
        saved: SeekOptimizationState,
    ) {
        stream.set_seek_optimization_enabled(saved.disabled);
    }

    pub fn update_fpos_cache<S: FlushStream + ?Sized>(stream: &mut S, pos: u64) {
        stream.update_cached_position(pos);
    }

    pub fn rpl_fflush<S: FlushStream + ?Sized>(target: FlushTarget<'_, S>) -> io::Result<()> {
        match target {
            FlushTarget::All => Ok(()),
            FlushTarget::Stream(stream) => {
                if !stream.is_reading() {
                    return stream.flush_output();
                }

                let pos = stream.stream_position().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "stream is not seekable")
                })?;

                Self::clear_ungetc_buffer(stream)?;
                stream.purge_read_buffer()?;

                let saved = Self::disable_seek_optimization(stream);
                let seek_result = stream.seek(SeekFrom::Start(pos)).map(|_| ());
                Self::restore_seek_optimization(stream, saved);
                seek_result?;
                Self::update_fpos_cache(stream, pos);
                Ok(())
            }
        }
    }

    pub fn main_root_clear_ungetc_09<S: FlushStream + ?Sized>(
        stream: &mut S,
        preserve_position: bool,
    ) -> io::Result<()> {
        if preserve_position {
            Self::clear_ungetc_buffer_preserving_position(stream)
        } else {
            Self::clear_ungetc_buffer(stream)
        }
    }
}
