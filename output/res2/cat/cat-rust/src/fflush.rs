use std::io::{self, BufRead, Error, ErrorKind, Seek, SeekFrom, Write};

pub struct Fflush;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SeekOptimizationState {
    Enabled,
    Disabled,
}

pub trait FlushStream {
    fn flush_output(&mut self) -> io::Result<()>;
    fn is_reading(&self) -> bool;
    fn position(&mut self) -> io::Result<u64>;
    fn purge_read_buffer(&mut self) -> io::Result<()>;
    fn seek_to(&mut self, position: u64) -> io::Result<()>;
    fn clear_pushback_preserving_position(&mut self) -> io::Result<()>;
    fn clear_pushback(&mut self) -> io::Result<()>;
    fn seek_optimization_state(&self) -> Option<SeekOptimizationState>;
    fn set_seek_optimization_state(&mut self, state: SeekOptimizationState);
    fn set_cached_position(&mut self, position: u64);
}

impl<T: ReadWriteSeek> FlushStream for io::BufReader<T> {
    fn flush_output(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn is_reading(&self) -> bool {
        true
    }

    fn position(&mut self) -> io::Result<u64> {
        let raw = self.get_mut().stream_position()?;
        let unread = self.fill_buf()?.len() as u64;
        Ok(raw.saturating_sub(unread))
    }

    fn purge_read_buffer(&mut self) -> io::Result<()> {
        let pos = self.position()?;
        self.seek(SeekFrom::Start(pos)).map(|_| ())
    }

    fn seek_to(&mut self, position: u64) -> io::Result<()> {
        self.seek(SeekFrom::Start(position)).map(|_| ())
    }

    fn clear_pushback_preserving_position(&mut self) -> io::Result<()> {
        self.seek(SeekFrom::Current(0)).map(|_| ())
    }

    fn clear_pushback(&mut self) -> io::Result<()> {
        self.seek(SeekFrom::Current(0)).map(|_| ())
    }

    fn seek_optimization_state(&self) -> Option<SeekOptimizationState> {
        Some(SeekOptimizationState::Enabled)
    }

    fn set_seek_optimization_state(&mut self, _state: SeekOptimizationState) {}

    fn set_cached_position(&mut self, _position: u64) {}
}

impl FlushStream for std::fs::File {
    fn flush_output(&mut self) -> io::Result<()> {
        self.flush()
    }

    fn is_reading(&self) -> bool {
        false
    }

    fn position(&mut self) -> io::Result<u64> {
        self.stream_position()
    }

    fn purge_read_buffer(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn seek_to(&mut self, position: u64) -> io::Result<()> {
        self.seek(SeekFrom::Start(position)).map(|_| ())
    }

    fn clear_pushback_preserving_position(&mut self) -> io::Result<()> {
        self.seek(SeekFrom::Current(0)).map(|_| ())
    }

    fn clear_pushback(&mut self) -> io::Result<()> {
        self.seek(SeekFrom::Current(0)).map(|_| ())
    }

    fn seek_optimization_state(&self) -> Option<SeekOptimizationState> {
        None
    }

    fn set_seek_optimization_state(&mut self, _state: SeekOptimizationState) {}

    fn set_cached_position(&mut self, _position: u64) {}
}

pub trait ReadWriteSeek: io::Read + Write + Seek {}
impl<T: io::Read + Write + Seek> ReadWriteSeek for T {}

impl Fflush {
    pub fn clear_ungetc_buffer_preserving_position<S: FlushStream>(
        stream: &mut S,
    ) -> io::Result<()> {
        stream.clear_pushback_preserving_position()
    }

    pub fn clear_ungetc_buffer<S: FlushStream>(stream: &mut S) -> io::Result<()> {
        stream.clear_pushback()
    }

    pub fn disable_seek_optimization<S: FlushStream>(
        stream: &mut S,
    ) -> Option<SeekOptimizationState> {
        let saved = stream.seek_optimization_state();
        if saved.is_some() {
            stream.set_seek_optimization_state(SeekOptimizationState::Disabled);
        }
        saved
    }

    pub fn restore_seek_optimization<S: FlushStream>(
        stream: &mut S,
        saved_flags: Option<SeekOptimizationState>,
    ) {
        if let Some(state) = saved_flags {
            stream.set_seek_optimization_state(state);
        }
    }

    pub fn update_fpos_cache<S: FlushStream>(stream: &mut S, pos: u64) {
        stream.set_cached_position(pos);
    }

    pub fn rpl_fflush<S: FlushStream>(stream: Option<&mut S>) -> io::Result<()> {
        let Some(stream) = stream else {
            return io::stdout().flush();
        };

        if !stream.is_reading() {
            return stream.flush_output();
        }

        let pos = stream
            .position()
            .map_err(|_| Error::new(ErrorKind::Unsupported, "stream is not seekable"))?;

        Self::clear_ungetc_buffer(stream)?;
        stream.purge_read_buffer()?;

        let saved_flags = Self::disable_seek_optimization(stream);
        let seek_result = stream.seek_to(pos);
        Self::restore_seek_optimization(stream, saved_flags);
        seek_result?;

        Self::update_fpos_cache(stream, pos);
        Ok(())
    }

    pub fn main_root_clear_ungetc_08<S: FlushStream>(stream: &mut S) -> io::Result<()> {
        Self::clear_ungetc_buffer_preserving_position(stream)?;
        Self::clear_ungetc_buffer(stream)
    }
}
