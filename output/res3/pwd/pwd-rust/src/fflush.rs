use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SeekOptimizationState {
    enabled: bool,
    no_position_tracking: bool,
}

pub trait FlushStream {
    fn flush_output(&mut self) -> io::Result<()>;
    fn stream_position(&mut self) -> io::Result<u64>;
    fn seek_to(&mut self, position: u64) -> io::Result<u64>;
    fn is_reading(&self) -> bool;
    fn has_ungetc_buffer(&self) -> bool;
    fn clear_ungetc_state(&mut self) -> io::Result<()>;
    fn purge_read_buffer(&mut self) -> io::Result<()>;
    fn seek_optimization_state(&self) -> SeekOptimizationState;
    fn set_seek_optimization_state(&mut self, state: SeekOptimizationState);
    fn update_cached_position(&mut self, position: u64) -> io::Result<()>;
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

    fn seek_to(&mut self, position: u64) -> io::Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    fn is_reading(&self) -> bool {
        false
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

    fn seek_optimization_state(&self) -> SeekOptimizationState {
        SeekOptimizationState::default()
    }

    fn set_seek_optimization_state(&mut self, _state: SeekOptimizationState) {}

    fn update_cached_position(&mut self, _position: u64) -> io::Result<()> {
        Ok(())
    }
}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S: FlushStream>(
        stream: &mut S,
    ) -> io::Result<()> {
        if stream.has_ungetc_buffer() {
            let pos = stream.stream_position()?;
            stream.seek_to(pos)?;
            stream.clear_ungetc_state()?;
        }
        Ok(())
    }

    pub fn clear_ungetc_buffer<S: FlushStream>(stream: &mut S) -> io::Result<()> {
        if stream.has_ungetc_buffer() {
            stream.clear_ungetc_state()?;
        }
        Ok(())
    }

    pub fn disable_seek_optimization<S: FlushStream>(stream: &mut S) -> SeekOptimizationState {
        let saved = stream.seek_optimization_state();
        let mut disabled = saved;
        disabled.enabled = false;
        disabled.no_position_tracking = true;
        stream.set_seek_optimization_state(disabled);
        saved
    }

    pub fn restore_seek_optimization<S: FlushStream>(
        stream: &mut S,
        saved_flags: SeekOptimizationState,
    ) {
        stream.set_seek_optimization_state(saved_flags);
    }

    pub fn update_fpos_cache<S: FlushStream>(stream: &mut S, pos: u64) -> io::Result<()> {
        stream.update_cached_position(pos)
    }

    pub fn rpl_fflush<S: FlushStream>(stream: Option<&mut S>) -> io::Result<()> {
        let Some(stream) = stream else {
            return Ok(());
        };

        if !stream.is_reading() {
            return stream.flush_output();
        }

        Self::clear_ungetc_buffer_preserving_position(stream)?;

        let pos = stream.stream_position().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot flush a non-seekable input stream",
            )
        })?;

        Self::clear_ungetc_buffer(stream)?;
        stream.purge_read_buffer()?;

        let saved_flags = Self::disable_seek_optimization(stream);
        let seek_result = stream.seek_to(pos);
        Self::restore_seek_optimization(stream, saved_flags);

        let pos = seek_result?;
        Self::update_fpos_cache(stream, pos)?;
        Ok(())
    }

    pub fn main_root_clear_ungetc_09() -> io::Result<()> {
        Ok(())
    }
}
