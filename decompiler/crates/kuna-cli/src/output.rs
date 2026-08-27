use std::io::{self, BufWriter, Write};

pub fn emit(text: &str) -> io::Result<()> {
    emit_bytes(text.as_bytes())
}

pub fn emit_bytes(bytes: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    emit_to(&mut writer, bytes)
}

pub fn emit_with_status(text: &str, status: i32) -> i32 {
    match emit(text) {
        Ok(()) => status,
        Err(err) => error_status(err),
    }
}

pub fn error_status(err: io::Error) -> i32 {
    if err.kind() == io::ErrorKind::BrokenPipe {
        0
    } else {
        eprintln!("error: could not write stdout: {err}");
        1
    }
}

fn emit_to(writer: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
    writer.write_all(bytes)?;
    writer.flush()
}

#[cfg(test)]
mod tests {
    use super::emit_to;
    use std::io::{self, Write};

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "synthetic failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn propagates_non_broken_pipe_write_errors() {
        let err = emit_to(&mut FailingWriter, b"output").expect_err("write should fail");
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "synthetic failure");
    }
}
