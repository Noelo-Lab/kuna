//! The CLI's stdout boundary.
//!
//! Every command renders its output and hands it here instead of `println!`,
//! which panics (exit 101) the moment a downstream reader closes the pipe --
//! `kuna catalog --markdown | head` was a panic on every subcommand that writes
//! stdout.  A closed pipe is a normal terminal condition: no panic, no
//! diagnostic.
//!
//! It suppresses the *diagnostic*, never the *verdict*.  The status a command
//! computed says whether its work succeeded and is orthogonal to whether anyone
//! was still reading, so it survives the broken pipe (`kuna test | head` on a
//! REGRESSED baseline still exits 1 -- a gate that cannot go green by losing its
//! audience).  Any other write failure is a real error: it is reported and
//! forces exit 1.

use std::io::{self, BufWriter, Write};

pub fn emit(text: &str) -> io::Result<()> {
    emit_bytes(text.as_bytes())
}

pub fn emit_bytes(bytes: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    emit_to(&mut writer, bytes)
}

/// Emit `text` and exit with `status` -- the command's own verdict, which a
/// closed pipe does not overwrite.
pub fn emit_with_status(text: &str, status: i32) -> i32 {
    match emit(text) {
        Ok(()) => status,
        Err(err) => status_after(err, status),
    }
}

/// Fold a stdout write failure into the verdict `status` the command reached.
pub fn status_after(err: io::Error, status: i32) -> i32 {
    if err.kind() == io::ErrorKind::BrokenPipe {
        status
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
    use super::{emit_to, status_after};
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

    #[test]
    fn a_broken_pipe_keeps_the_verdict() {
        for status in [0, 1, 2, 137] {
            let err = io::Error::new(io::ErrorKind::BrokenPipe, "closed");
            assert_eq!(status_after(err, status), status);
        }
    }

    #[test]
    fn any_other_write_failure_forces_one() {
        let err = io::Error::new(io::ErrorKind::Other, "disk full");
        assert_eq!(status_after(err, 0), 1);
    }
}
