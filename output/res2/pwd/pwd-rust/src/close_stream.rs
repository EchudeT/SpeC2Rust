use std::io;

pub struct CloseStream;

impl CloseStream {
    pub fn close<W>(stream: W) -> io::Result<()>
    where
        W: PendingWriter,
    {
        let some_pending = stream.has_pending_output();
        let prev_fail = stream.has_error();
        let close_result = stream.close();
        let close_error_kind = close_result.as_ref().err().map(io::Error::kind);

        if prev_fail
            || matches!(
                close_error_kind,
                Some(kind) if some_pending || kind != io::ErrorKind::NotConnected
            )
        {
            return match close_result {
                Err(err) => Err(err),
                Ok(()) => Err(io::Error::other(
                    "stream reported a prior write or flush failure before close",
                )),
            };
        }

        Ok(())
    }
}

pub trait PendingWriter {
    fn has_pending_output(&self) -> bool;
    fn has_error(&self) -> bool;
    fn close(self) -> io::Result<()>;
}
