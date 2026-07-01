use std::fs::File;
use std::io::{self, Read, Seek, Write};

pub struct Fclose;

enum StreamKind {
    Stdin(io::Stdin),
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    File(File),
}

impl StreamKind {
    fn into_nothrow(self) -> io::Result<()> {
        match self {
            StreamKind::Stdin(_) => Ok(()),
            StreamKind::Stdout(mut stream) => stream.flush(),
            StreamKind::Stderr(mut stream) => stream.flush(),
            StreamKind::File(file) => file.sync_all(),
        }
    }
}

impl Fclose {
    pub fn nothrow(stream: StreamKind) -> bool {
        stream.into_nothrow().is_ok()
    }

    pub fn rpl_fclose(stream: StreamKind) -> io::Result<()> {
        match stream {
            StreamKind::Stdin(stdin) => {
                let result = Self::nothrow(StreamKind::Stdin(stdin));
                if result {
                    Ok(())
                } else {
                    Err(io::Error::other("failed to close standard input"))
                }
            }
            StreamKind::Stdout(mut stdout) => {
                let flush_result = stdout.flush();
                let close_result = Self::nothrow(StreamKind::Stdout(stdout));
                match (flush_result, close_result) {
                    (Err(err), _) => Err(err),
                    (Ok(()), true) => Ok(()),
                    (Ok(()), false) => Err(io::Error::other("failed to close standard output")),
                }
            }
            StreamKind::Stderr(mut stderr) => {
                let flush_result = stderr.flush();
                let close_result = Self::nothrow(StreamKind::Stderr(stderr));
                match (flush_result, close_result) {
                    (Err(err), _) => Err(err),
                    (Ok(()), true) => Ok(()),
                    (Ok(()), false) => Err(io::Error::other("failed to close standard error")),
                }
            }
            StreamKind::File(mut file) => {
                let mut saved_error = None;

                let seekable = file.stream_position().is_ok();
                if seekable {
                    if let Err(err) = file.flush() {
                        saved_error = Some(err);
                    }
                } else {
                    let mut probe = [0_u8; 1];
                    if file.read(&mut probe).is_err() {
                        if let Err(err) = file.flush() {
                            saved_error = Some(err);
                        }
                    }
                }

                let close_ok = Self::nothrow(StreamKind::File(file));
                if let Some(err) = saved_error {
                    Err(err)
                } else if close_ok {
                    Ok(())
                } else {
                    Err(io::Error::other("failed to close file stream"))
                }
            }
        }
    }
}
