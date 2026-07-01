use std::io::{self, BufRead, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SeekOptimizationState {
    enabled: bool,
    no_pushback_seek_optimization: bool,
}

pub trait StreamPosition {
    fn stream_position(&mut self) -> io::Result<u64>;
}

impl<T: Seek + ?Sized> StreamPosition for T {
    fn stream_position(&mut self) -> io::Result<u64> {
        Seek::stream_position(self)
    }
}

pub trait FlushStream: Write + Seek {
    fn is_reading(&self) -> bool {
        false
    }

    fn has_ungetc_buffer(&self) -> bool {
        false
    }

    fn clear_ungetc_buffer(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn purge_read_buffer(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn seek_optimization_state(&self) -> SeekOptimizationState {
        SeekOptimizationState::default()
    }

    fn set_seek_optimization_state(&mut self, _state: SeekOptimizationState) {}

    fn update_cached_position(&mut self, _pos: u64) {}
}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S>(stream: &mut S) -> io::Result<()>
    where
        S: FlushStream,
    {
        if stream.has_ungetc_buffer() {
            stream.seek(SeekFrom::Current(0))?;
        }
        Ok(())
    }

    pub fn clear_ungetc_buffer<S>(stream: &mut S) -> io::Result<()>
    where
        S: FlushStream,
    {
        if stream.has_ungetc_buffer() {
            stream.clear_ungetc_buffer()?;
        } else {
            stream.seek(SeekFrom::Current(0))?;
        }
        Ok(())
    }

    pub fn disable_seek_optimization<S>(stream: &mut S) -> bool
    where
        S: FlushStream,
    {
        let saved = stream.seek_optimization_state();
        let next = SeekOptimizationState {
            enabled: false,
            no_pushback_seek_optimization: true,
        };
        stream.set_seek_optimization_state(next);
        saved.enabled
    }

    pub fn restore_seek_optimization<S>(stream: &mut S, saved_flags: bool)
    where
        S: FlushStream,
    {
        let mut state = stream.seek_optimization_state();
        state.enabled = saved_flags;
        if saved_flags {
            state.no_pushback_seek_optimization = false;
        }
        stream.set_seek_optimization_state(state);
    }

    pub fn update_fpos_cache<S>(stream: &mut S, pos: u64)
    where
        S: FlushStream,
    {
        stream.update_cached_position(pos);
    }

    pub fn stream_position<S>(stream: &mut S) -> io::Result<u64>
    where
        S: StreamPosition + ?Sized,
    {
        stream.stream_position()
    }

    pub fn rpl_fflush<S>(stream: Option<&mut S>) -> io::Result<()>
    where
        S: FlushStream,
    {
        match stream {
            None => Ok(()),
            Some(stream) => {
                if !stream.is_reading() {
                    return stream.flush();
                }

                Self::clear_ungetc_buffer_preserving_position(stream)?;

                let pos = Self::stream_position(stream)?;

                Self::clear_ungetc_buffer(stream)?;
                stream.purge_read_buffer()?;

                let saved_flags = Self::disable_seek_optimization(stream);
                let seek_result = stream.seek(SeekFrom::Start(pos));
                Self::restore_seek_optimization(stream, saved_flags);
                seek_result?;

                Self::update_fpos_cache(stream, pos);
                Ok(())
            }
        }
    }

    pub fn main_root_clear_ungetc_09<R>(reader: &mut R) -> io::Result<usize>
    where
        R: BufRead,
    {
        let buffered = reader.fill_buf()?.len();
        reader.consume(buffered);
        Ok(buffered)
    }
}
