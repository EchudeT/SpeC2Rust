use std::io::{self, Seek};

pub struct Fclose;

impl Fclose {
    pub fn nothrow<T>(stream: T) -> io::Result<()>
    where
        T: io::Write,
    {
        let mut stream = stream;
        stream.flush()
    }

    pub fn rpl_fclose<T>(stream: T) -> io::Result<()>
    where
        T: io::Write,
    {
        Self::nothrow(stream)
    }
}

#[allow(dead_code)]
fn _seekability_probe<T>(stream: &mut T) -> bool
where
    T: Seek,
{
    stream.stream_position().is_ok()
}
