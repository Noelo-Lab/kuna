//! The burst framing of the Ghidra client <-> decompiler protocol — a
//! line-faithful port of the static stream helpers on `ArchitectureGhidra`
//! (`decompiler/cpp/ghidra_arch.cc:50-248`).
//!
//! Every protocol marker ("burst") is written as the four bytes
//! `{0x00,0x00,0x01,code}` and read tolerantly as *one or more zero bytes,
//! then 0x01, then the code byte* (`readToAnyBurst`, ghidra_arch.cc:79-98).
//! Code bytes pair open (even) / close (odd):
//!
//! | pair    | meaning                                 |
//! |---------|-----------------------------------------|
//! | 2 / 3   | Command (Ghidra -> decompiler)          |
//! | 4 / 5   | Query (decompiler -> Ghidra)            |
//! | 6 / 7   | Command response (decompiler -> Ghidra) |
//! | 8 / 9   | Query response (Ghidra -> decompiler)   |
//! | 10 / 11 | Exception (either direction)            |
//! | 12 / 13 | Byte stream                             |
//! | 14 / 15 | String stream                           |
//! | 16 / 17 | Native message (warnings/errors)        |
//!
//! Payload bytes never contain 0x00 (packed documents are all-nonzero by
//! construction, marshal.hh:456), which is what lets the leading zero of a
//! close burst terminate payload accumulation.
//!
//! Everything is generic over [`std::io::Read`] / [`std::io::Write`] so the
//! tests drive it with in-memory buffers.  The C++ helpers call `exit(1)`
//! when the pipe closes (ghidra_arch.cc:95-96 and copies); library code
//! must not exit, so EOF surfaces as the distinct [`WireError::PipeClosed`]
//! which the binary maps to `exit(1)`.
//!
//! Phase-1 scope: the framing layer is complete (it is identical in every
//! later phase); see `docs/ghidra-integration.md`.

use std::io::{Read, Write};

use kuna_base::error::KunaError;
use kuna_base::marshal::Decoder;

// ---------------------------------------------------------------------------
// Burst codes (ghidra_arch.cc:50-77; the 16/17 message pair is emitted by
// GhidraCommand::sendResult, ghidra_process.cc:112-114)
// ---------------------------------------------------------------------------

/// Command open (Ghidra -> decompiler).
pub const BURST_COMMAND_OPEN: u8 = 2;
/// Command close.
pub const BURST_COMMAND_CLOSE: u8 = 3;
/// Query open (decompiler -> Ghidra).
pub const BURST_QUERY_OPEN: u8 = 4;
/// Query close.
pub const BURST_QUERY_CLOSE: u8 = 5;
/// Command response open (decompiler -> Ghidra).
pub const BURST_RESPONSE_OPEN: u8 = 6;
/// Command response close.
pub const BURST_RESPONSE_CLOSE: u8 = 7;
/// Query response open (Ghidra -> decompiler).
pub const BURST_QUERY_RESPONSE_OPEN: u8 = 8;
/// Query response close.
pub const BURST_QUERY_RESPONSE_CLOSE: u8 = 9;
/// Exception open (either direction).
pub const BURST_EXCEPTION_OPEN: u8 = 10;
/// Exception close.
pub const BURST_EXCEPTION_CLOSE: u8 = 11;
/// Byte stream open.
pub const BURST_BYTES_OPEN: u8 = 12;
/// Byte stream close.
pub const BURST_BYTES_CLOSE: u8 = 13;
/// String stream open.
pub const BURST_STRING_OPEN: u8 = 14;
/// String stream close.
pub const BURST_STRING_CLOSE: u8 = 15;
/// Native message open (decompiler -> Ghidra).
pub const BURST_MESSAGE_OPEN: u8 = 16;
/// Native message close.
pub const BURST_MESSAGE_CLOSE: u8 = 17;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// The wire-level error type of the ghidra-mode front-end.
///
/// The C++ helpers have three failure modes: pipe closure (`exit(1)`),
/// protocol misalignment / Java exceptions (`throw JavaError`), and engine
/// errors (`LowlevelError` etc.).  The latter two are [`KunaError`]s; pipe
/// closure gets its own variant so the process loop can map it to a clean
/// process exit instead of a warning.
#[derive(Debug)]
pub enum WireError {
    /// stdin/stdout reached EOF or broke — the parent (Java) process is
    /// probably dead (the C++ `exit(1)` sites, ghidra_arch.cc:95-96).
    PipeClosed,
    /// Any other I/O failure on the streams.
    Io(std::io::Error),
    /// A protocol/engine error ([`KunaError::Java`] carries both real Java
    /// exceptions and the C++-generated "alignment" errors).
    Kuna(KunaError),
}

impl std::fmt::Display for WireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WireError::PipeClosed => write!(f, "pipe closed"),
            WireError::Io(e) => write!(f, "i/o error: {e}"),
            WireError::Kuna(e) => write!(f, "{e}"),
        }
    }
}

impl From<KunaError> for WireError {
    fn from(e: KunaError) -> WireError {
        WireError::Kuna(e)
    }
}

/// Convenience alias for the wire helpers.
pub type WireResult<T> = Result<T, WireError>;

/// `throw JavaError("alignment", msg)` — the C++-generated protocol error.
pub fn java_alignment(msg: &str) -> WireError {
    WireError::Kuna(KunaError::java("alignment", msg))
}

/// Map a write/flush error: a broken pipe means the parent died.
fn write_err(e: std::io::Error) -> WireError {
    if e.kind() == std::io::ErrorKind::BrokenPipe {
        WireError::PipeClosed
    } else {
        WireError::Io(e)
    }
}

/// One byte from the stream — the C++ `istream::get()`, with EOF (`c < 0`)
/// mapped to [`WireError::PipeClosed`].
fn get_byte<R: Read>(s: &mut R) -> WireResult<u8> {
    let mut buf = [0u8; 1];
    loop {
        match s.read(&mut buf) {
            Ok(0) => return Err(WireError::PipeClosed),
            Ok(_) => return Ok(buf[0]),
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err(WireError::PipeClosed)
            }
            Err(e) => return Err(WireError::Io(e)),
        }
    }
}

// ---------------------------------------------------------------------------
// The ten static helpers (ghidra_arch.cc:79-248)
// ---------------------------------------------------------------------------

/// Scan the stream to the next burst marker and return its code byte
/// (C++ `ArchitectureGhidra::readToAnyBurst`, ghidra_arch.cc:79-98).
///
/// Tolerant: arbitrary non-zero garbage before the marker is skipped, and
/// one *or more* zero bytes are accepted before the 0x01.
pub fn read_to_any_burst<R: Read>(s: &mut R) -> WireResult<u8> {
    loop {
        // skip non-zero garbage
        let mut c = get_byte(s)?;
        while c > 0 {
            c = get_byte(s)?;
        }
        // skip the zero run
        while c == 0 {
            c = get_byte(s)?;
        }
        if c == 1 {
            return get_byte(s); // the code byte
        }
        // (c < 0 is PipeClosed inside get_byte; other values re-scan)
    }
}

/// Accumulate payload bytes up to the next burst and return
/// `(payload, code)`.
///
/// This is the shared tail of the two `readStringStream` variants
/// (ghidra_arch.cc:133-155, 162-176): bytes accumulate while positive; the
/// first zero starts the close burst.  With `tolerant` set, garbage after
/// the zero run falls back to the full [`read_to_any_burst`] scan (the
/// decoder variant resumes with `readToAnyBurst`); otherwise anything but
/// `0x01 code` is a protocol error (the plain-string variant's strict
/// terminator check).
fn read_payload_to_burst<R: Read>(s: &mut R, tolerant: bool) -> WireResult<(Vec<u8>, u8)> {
    let mut payload: Vec<u8> = Vec::new();
    let mut c = get_byte(s)?;
    while c > 0 {
        payload.push(c);
        c = get_byte(s)?;
    }
    while c == 0 {
        c = get_byte(s)?;
    }
    if c == 1 {
        let code = get_byte(s)?;
        return Ok((payload, code));
    }
    if tolerant {
        let code = read_to_any_burst(s)?;
        return Ok((payload, code));
    }
    Err(java_alignment("Expecting string terminator"))
}

/// Accumulate an already-opened string payload up to the next burst,
/// tolerantly, returning `(payload, close_code)` — the base
/// `GhidraCommand::loadParameters` arch-id read (`sin >> dec >> id` then
/// `readToAnyBurst`, ghidra_process.cc:86-103), which scans to the close
/// burst with full [`read_to_any_burst`] tolerance.  The caller has already
/// consumed the opening burst and checks the close code itself (the C++
/// messages differ per caller).
pub fn read_id_payload<R: Read>(s: &mut R) -> WireResult<(Vec<u8>, u8)> {
    read_payload_to_burst(s, true)
}

/// Read a string-stream burst containing a single `'t'`/`'f'` character
/// (C++ `readBoolStream`, ghidra_arch.cc:104-126).
pub fn read_bool_stream<R: Read>(s: &mut R) -> WireResult<bool> {
    let t = read_to_any_burst(s)?;
    if t != BURST_STRING_OPEN {
        return Err(java_alignment("Expecting string"));
    }
    let c = get_byte(s)?;
    let res = c == b't';
    let mut c2 = if c == 0 { 0 } else { get_byte(s)? };
    while c2 == 0 {
        c2 = get_byte(s)?;
    }
    if c2 == 1 && get_byte(s)? == BURST_STRING_CLOSE {
        return Ok(res);
    }
    Err(java_alignment("Expecting string terminator"))
}

/// Read a full string stream into a byte buffer (C++
/// `readStringStream(istream&,string&)`, ghidra_arch.cc:133-155).
pub fn read_string_stream<R: Read>(s: &mut R) -> WireResult<Vec<u8>> {
    let t = read_to_any_burst(s)?;
    if t != BURST_STRING_OPEN {
        return Err(java_alignment("Expecting string"));
    }
    let (payload, code) = read_payload_to_burst(s, false)?;
    if code != BURST_STRING_CLOSE {
        return Err(java_alignment("Expecting string terminator"));
    }
    Ok(payload)
}

/// Read an *optional* string stream: `Some(payload)` if a string burst is
/// present, `None` if the next burst is any close (odd) code — the framing
/// of C++ `readStringStream(istream&,Decoder&)` (ghidra_arch.cc:162-176)
/// minus the decoder ingest, which [`read_string_stream_decoder`] layers on.
pub fn read_string_stream_optional<R: Read>(s: &mut R) -> WireResult<Option<Vec<u8>>> {
    let t = read_to_any_burst(s)?;
    if t == BURST_STRING_OPEN {
        let (payload, code) = read_payload_to_burst(s, true)?;
        if code != BURST_STRING_CLOSE {
            return Err(java_alignment("Expecting XML string end"));
        }
        return Ok(Some(payload));
    }
    if (t & 1) == 1 {
        return Ok(None); // empty response
    }
    Err(java_alignment("Expecting string or end of query response"))
}

/// Read a string stream into a stream decoder (C++
/// `readStringStream(istream&,Decoder&)`, ghidra_arch.cc:162-176).
/// Returns `false` on an empty (close-burst) response.
pub fn read_string_stream_decoder<R: Read>(
    s: &mut R,
    decoder: &mut dyn Decoder,
) -> WireResult<bool> {
    // (The C++ ingests straight off the istream; kuna's Decoder::ingest_stream
    // takes a slice, so the payload is accumulated first — same bytes, the
    // packed encoding is NUL-free so the accumulation boundary is identical.)
    let t = read_to_any_burst(s)?;
    if t == BURST_STRING_OPEN {
        let (payload, code) = read_payload_to_burst(s, true)?;
        decoder.ingest_stream(&payload).map_err(WireError::Kuna)?;
        if code != BURST_STRING_CLOSE {
            return Err(java_alignment("Expecting XML string end"));
        }
        return Ok(true);
    }
    if (t & 1) == 1 {
        return Ok(false);
    }
    Err(java_alignment("Expecting string or end of query response"))
}

/// Write a burst marker `{0x00,0x00,0x01,code}`.
pub fn write_burst<W: Write>(s: &mut W, code: u8) -> WireResult<()> {
    s.write_all(&[0, 0, 1, code]).map_err(write_err)
}

/// Write a string stream with its protocol markers (C++
/// `writeStringStream`, ghidra_arch.cc:181-187).
pub fn write_string_stream<W: Write>(s: &mut W, msg: &[u8]) -> WireResult<()> {
    write_burst(s, BURST_STRING_OPEN)?;
    s.write_all(msg).map_err(write_err)?;
    write_burst(s, BURST_STRING_CLOSE)
}

/// Consume the query-response header; an exception frame is decoded and
/// surfaced as [`KunaError::Java`] (C++ `readToResponse`,
/// ghidra_arch.cc:192-205).
pub fn read_to_response<R: Read>(s: &mut R) -> WireResult<()> {
    let t = read_to_any_burst(s)?;
    if t == BURST_QUERY_RESPONSE_OPEN {
        return Ok(());
    }
    if t == BURST_EXCEPTION_OPEN {
        let extype = read_string_stream(s)?;
        let message = read_string_stream(s)?;
        let _ = read_to_any_burst(s)?; // the exception terminator
        return Err(WireError::Kuna(KunaError::java(
            String::from_utf8_lossy(&extype).into_owned(),
            String::from_utf8_lossy(&message).into_owned(),
        )));
    }
    Err(java_alignment("Expecting query response"))
}

/// Consume the query-response terminator (C++ `readResponseEnd`,
/// ghidra_arch.cc:210-216).
pub fn read_response_end<R: Read>(s: &mut R) -> WireResult<()> {
    let t = read_to_any_burst(s)?;
    if t != BURST_QUERY_RESPONSE_CLOSE {
        return Err(java_alignment("Expecting end of query response"));
    }
    Ok(())
}

/// Read a whole query response into a decoder (C++ `readAll`,
/// ghidra_arch.cc:223-232).  Returns `false` if the client answered with an
/// empty response.
pub fn read_all<R: Read>(s: &mut R, decoder: &mut dyn Decoder) -> WireResult<bool> {
    read_to_response(s)?;
    if read_string_stream_decoder(s, decoder)? {
        read_response_end(s)?;
        return Ok(true);
    }
    Ok(false)
}

/// Send an exception frame to the Ghidra client (C++ `passJavaException`,
/// ghidra_arch.cc:241-248).
pub fn pass_java_exception<W: Write>(s: &mut W, tp: &str, msg: &str) -> WireResult<()> {
    write_burst(s, BURST_EXCEPTION_OPEN)?;
    write_string_stream(s, tp.as_bytes())?;
    write_string_stream(s, msg.as_bytes())?;
    write_burst(s, BURST_EXCEPTION_CLOSE)
}

// ---------------------------------------------------------------------------
// Byte-stream payload codecs (getBytes / getStringData formats)
// ---------------------------------------------------------------------------

/// Condense a nibble-doubled byte-stream payload: each source byte `b`
/// travels as the two bytes `'A'+(b>>4)` and `'A'+(b&0xF)`
/// (getBytes decode loop, ghidra_arch.cc:743-745; Java encoder
/// `DecompileProcess.java:867-885`).  `pairs.len()` must be even.
pub fn nibble_condense(pairs: &[u8]) -> Vec<u8> {
    debug_assert!(pairs.len().is_multiple_of(2));
    let mut out = Vec::with_capacity(pairs.len() / 2);
    for chunk in pairs.chunks_exact(2) {
        out.push((chunk[0].wrapping_sub(b'A') << 4) | (chunk[1].wrapping_sub(b'A') & 0xF));
    }
    out
}

/// Expand bytes into the nibble-doubled wire form (the Java side of
/// [`nibble_condense`]; used by the tests' MockJava and by any future
/// server-side byte responses).
pub fn nibble_expand(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(((b >> 4) & 0xF) + b'A');
        out.push((b & 0xF) + b'A');
    }
    out
}

/// Decode the getStringData two-byte size header: `size = (b1-0x20) ^
/// ((b2-0x20)<<6)` (ghidra_arch.cc:793-797; the Java encoder biases each
/// 6-bit chunk of `len+1` by 0x20, `DecompileProcess.java:894-914`).
pub fn string_data_size(b1: u8, b2: u8) -> u32 {
    (b1.wrapping_sub(0x20) as u32) ^ ((b2.wrapping_sub(0x20) as u32) << 6)
}

/// Encode the getStringData size header (the Java side of
/// [`string_data_size`]): `sz` is `len+1` and must fit in 12 bits.
pub fn string_data_size_header(sz: u32) -> [u8; 2] {
    [
        ((sz & 0x3f) + 0x20) as u8,
        (((sz >> 6) & 0x3f) + 0x20) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn burst(code: u8) -> Vec<u8> {
        vec![0, 0, 1, code]
    }

    #[test]
    fn test_read_to_any_burst_tolerance() {
        // garbage bytes before the marker are skipped
        let mut s = Cursor::new([b'x', b'y', 0, 0, 1, 7].to_vec());
        assert_eq!(read_to_any_burst(&mut s).unwrap(), 7);
        // one or many zeros are accepted
        let mut s = Cursor::new([0, 1, 9].to_vec());
        assert_eq!(read_to_any_burst(&mut s).unwrap(), 9);
        let mut s = Cursor::new([0, 0, 0, 0, 1, 3].to_vec());
        assert_eq!(read_to_any_burst(&mut s).unwrap(), 3);
        // a zero run NOT followed by 0x01 re-scans (0x05 is skipped as garbage)
        let mut s = Cursor::new([0, 0, 5, 0, 0, 1, 2].to_vec());
        assert_eq!(read_to_any_burst(&mut s).unwrap(), 2);
    }

    #[test]
    fn test_read_to_any_burst_eof_is_pipe_closed() {
        let mut s = Cursor::new(Vec::<u8>::new());
        assert!(matches!(
            read_to_any_burst(&mut s),
            Err(WireError::PipeClosed)
        ));
        // EOF mid-marker
        let mut s = Cursor::new([0, 0, 1].to_vec());
        assert!(matches!(
            read_to_any_burst(&mut s),
            Err(WireError::PipeClosed)
        ));
        // EOF inside a string payload
        let mut input = burst(BURST_STRING_OPEN);
        input.extend_from_slice(b"abc");
        let mut s = Cursor::new(input);
        assert!(matches!(
            read_string_stream(&mut s),
            Err(WireError::PipeClosed)
        ));
    }

    #[test]
    fn test_string_stream_round_trip() {
        let mut wire = Vec::new();
        write_string_stream(&mut wire, b"registerProgram").unwrap();
        let mut s = Cursor::new(wire);
        assert_eq!(read_string_stream(&mut s).unwrap(), b"registerProgram");
        // empty payload
        let mut wire = Vec::new();
        write_string_stream(&mut wire, b"").unwrap();
        let mut s = Cursor::new(wire);
        assert_eq!(read_string_stream(&mut s).unwrap(), b"");
    }

    #[test]
    fn test_string_stream_alignment_errors() {
        // wrong opening burst
        let mut s = Cursor::new(burst(BURST_BYTES_OPEN));
        match read_string_stream(&mut s) {
            Err(WireError::Kuna(KunaError::Java { type_name, explain })) => {
                assert_eq!(type_name, "alignment");
                assert_eq!(explain, "Expecting string");
            }
            other => panic!("expected alignment error, got {other:?}"),
        }
        // wrong close code (strict variant)
        let mut input = burst(BURST_STRING_OPEN);
        input.extend_from_slice(b"abc");
        input.extend_from_slice(&burst(BURST_BYTES_CLOSE));
        let mut s = Cursor::new(input);
        match read_string_stream(&mut s) {
            Err(WireError::Kuna(KunaError::Java { explain, .. })) => {
                assert_eq!(explain, "Expecting string terminator");
            }
            other => panic!("expected alignment error, got {other:?}"),
        }
    }

    #[test]
    fn test_bool_stream() {
        let mut wire = Vec::new();
        write_string_stream(&mut wire, b"t").unwrap();
        assert!(read_bool_stream(&mut Cursor::new(wire)).unwrap());
        let mut wire = Vec::new();
        write_string_stream(&mut wire, b"f").unwrap();
        assert!(!read_bool_stream(&mut Cursor::new(wire)).unwrap());
        // an empty payload reads as false (C++ takes the first burst zero as
        // the character)
        let mut wire = Vec::new();
        write_string_stream(&mut wire, b"").unwrap();
        assert!(!read_bool_stream(&mut Cursor::new(wire)).unwrap());
    }

    #[test]
    fn test_read_to_response_exception_frame() {
        let mut wire = burst(BURST_EXCEPTION_OPEN);
        write_string_stream(&mut wire, b"java.lang.Exception").unwrap();
        write_string_stream(&mut wire, b"boom").unwrap();
        wire.extend_from_slice(&burst(BURST_EXCEPTION_CLOSE));
        match read_to_response(&mut Cursor::new(wire)) {
            Err(WireError::Kuna(KunaError::Java { type_name, explain })) => {
                assert_eq!(type_name, "java.lang.Exception");
                assert_eq!(explain, "boom");
            }
            other => panic!("expected JavaError, got {other:?}"),
        }
    }

    #[test]
    fn test_nibble_codec() {
        let bytes = [0x00u8, 0x0f, 0xf0, 0xde, 0xad];
        let pairs = nibble_expand(&bytes);
        assert_eq!(pairs, b"AAAPPANOKN".to_vec());
        assert_eq!(nibble_condense(&pairs), bytes.to_vec());
    }

    #[test]
    fn test_string_data_size_header() {
        for sz in [0u32, 1, 63, 64, 100, 0xfff] {
            let [b1, b2] = string_data_size_header(sz);
            assert_eq!(string_data_size(b1, b2), sz, "sz={sz}");
        }
    }
}
