use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SeekOptimizationState {
    optimization_enabled: bool,
    optimization_prevented: bool,
}

pub trait FlushStream {
    fn flush_output(&mut self) -> io::Result<()>;
    fn stream_position(&mut self) -> io::Result<u64>;
    fn seek_absolute(&mut self, position: u64) -> io::Result<u64>;
    fn is_reading(&self) -> bool;
    fn has_ungetc_buffer(&self) -> bool;
    fn clear_ungetc_buffer_preserving_position(&mut self) -> io::Result<()>;
    fn clear_ungetc_buffer(&mut self) -> io::Result<()>;
    fn purge_read_buffer(&mut self) -> io::Result<()>;
    fn set_seek_optimization(&mut self, enabled: bool, prevented: bool);
    fn seek_optimization(&self) -> (bool, bool);
    fn update_cached_position(&mut self, position: Option<u64>);
}

impl<T> FlushStream for T
where
    T: Read + Write + Seek,
{
    fn flush_output(&mut self) -> io::Result<()> {
        self.flush()
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        self.stream_position()
    }

    fn seek_absolute(&mut self, position: u64) -> io::Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    fn is_reading(&self) -> bool {
        false
    }

    fn has_ungetc_buffer(&self) -> bool {
        false
    }

    fn clear_ungetc_buffer_preserving_position(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn clear_ungetc_buffer(&mut self) -> io::Result<()> {
        let _ = self.seek(SeekFrom::Current(0))?;
        Ok(())
    }

    fn purge_read_buffer(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn set_seek_optimization(&mut self, _enabled: bool, _prevented: bool) {}

    fn seek_optimization(&self) -> (bool, bool) {
        (false, false)
    }

    fn update_cached_position(&mut self, _position: Option<u64>) {}
}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S: FlushStream>(
        stream: &mut S,
    ) -> io::Result<()> {
        if stream.has_ungetc_buffer() {
            stream.clear_ungetc_buffer_preserving_position()?;
        }
        Ok(())
    }

    pub fn clear_ungetc_buffer<S: FlushStream>(stream: &mut S) -> io::Result<()> {
        stream.clear_ungetc_buffer()
    }

    pub fn disable_seek_optimization<S: FlushStream>(stream: &mut S) -> SeekOptimizationState {
        let (optimization_enabled, optimization_prevented) = stream.seek_optimization();
        stream.set_seek_optimization(false, true);
        SeekOptimizationState {
            optimization_enabled,
            optimization_prevented,
        }
    }

    pub fn restore_seek_optimization<S: FlushStream>(
        stream: &mut S,
        saved_flags: SeekOptimizationState,
    ) {
        stream.set_seek_optimization(
            saved_flags.optimization_enabled,
            saved_flags.optimization_prevented,
        );
    }

    pub fn update_fpos_cache<S: FlushStream>(stream: &mut S, position: Option<u64>) {
        stream.update_cached_position(position);
    }

    pub fn rpl_fflush<S: FlushStream>(stream: Option<&mut S>) -> io::Result<()> {
        let Some(stream) = stream else {
            return Ok(());
        };

        if !stream.is_reading() {
            return stream.flush_output();
        }

        Self::clear_ungetc_buffer_preserving_position(stream)?;

        let position = match stream.stream_position() {
            Ok(pos) => pos,
            Err(_) => {
                return Err(io::Error::from(io::ErrorKind::InvalidInput));
            }
        };

        Self::clear_ungetc_buffer(stream)?;
        stream.purge_read_buffer()?;

        let saved = Self::disable_seek_optimization(stream);
        let seek_result = stream.seek_absolute(position);
        Self::restore_seek_optimization(stream, saved);

        match seek_result {
            Ok(new_pos) => {
                Self::update_fpos_cache(stream, Some(new_pos));
                Ok(())
            }
            Err(err) => {
                Self::update_fpos_cache(stream, None);
                Err(err)
            }
        }
    }
}
