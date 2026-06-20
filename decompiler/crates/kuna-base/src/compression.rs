//! Port of `decompiler/cpp/compression.{hh,cc}` (W1): the `Compress` and
//! `Decompress` classes wrapping the deflate and inflate algorithms, plus the
//! `CompressBuffer` output filter.
//!
//! The C++ wraps zlib's `z_stream` directly.  Per LOSS-004
//! (`docs/rust-port/losses.md`) the Rust port goes through the `flate2`
//! crate (miniz_oxide backend) speaking the identical zlib wire format; the
//! chunked API semantics of compression.hh are preserved:
//!
//! - `input()` provides the next sequence of bytes, then `deflate()` /
//!   `inflate()` is called in a loop, each call filling one output buffer and
//!   returning the number of output bytes *still available* (C++ returns
//!   `compStream.avail_out`).
//! - C++ `z_stream` keeps a raw `next_in` pointer into the caller's buffer;
//!   the Rust port copies the input into an internal buffer with a consumed
//!   cursor instead (invisible to callers, avoids a self-referential
//!   lifetime).
//! - zlib return codes become `Result`: the codes C++ converts to
//!   `LowlevelError` map to `Err(KunaError::Lowlevel)` with the same explain
//!   strings; `Z_STREAM_END` on inflate sets the `streamFinished` flag
//!   queried via [`Decompress::is_finished`].
//!
//! The compression level pinned by the C++ tree: `slgh_compile.cc:3805`
//! constructs `sla::FormatEncode encoder(s,-1)`, i.e. level -1 =
//! `Z_DEFAULT_COMPRESSION` (level 6, zlib header bytes `0x78 0x9C`).

use std::io::Write;

use flate2::{FlushCompress, FlushDecompress, Status};

use crate::error::{KunaError, KunaResult};

/// Wrapper for the deflate algorithm (C++ `Compress`).
///
/// Initialize/free algorithm resources.  Provide successive arrays of bytes
/// to compress via the [`Compress::input`] method.  Compute successive arrays
/// of compressed bytes via the [`Compress::deflate`] method.
pub struct Compress {
    /// The zlib deflate algorithm state (C++ `z_stream compStream`)
    comp_stream: flate2::Compress,
    /// Pending input bytes (C++ keeps `next_in`/`avail_in` pointing into the
    /// caller's buffer; the port copies — see module docs)
    in_buf: Vec<u8>,
    /// Number of bytes of `in_buf` already consumed by the compressor
    in_pos: usize,
}

impl Compress {
    /// Initialize the deflate algorithm state.
    ///
    /// The compression `level` ranges from 1-9 from faster/least compression
    /// to slower/most compression.  Use a `level` of 0 for no compression and
    /// -1 for the *default* compression level.
    ///
    /// C++ calls `deflateInit(&compStream, level)` and throws
    /// `LowlevelError` if it does not return `Z_OK`; `deflateInit` rejects
    /// levels outside -1..=9 with `Z_STREAM_ERROR`, which is the only failure
    /// mode reachable here.
    pub fn new(level: i32) -> KunaResult<Self> {
        let compression = match level {
            // Z_DEFAULT_COMPRESSION == -1, internally level 6
            -1 => flate2::Compression::default(),
            // mixed signed/unsigned: zlib takes the level as int; valid
            // nonnegative levels fit u32 exactly
            0..=9 => flate2::Compression::new(level as u32),
            _ => return Err(KunaError::lowlevel("Could not initialize deflate stream state")),
        };
        Ok(Compress {
            // true: emit the zlib header/trailer, as C++ deflateInit does
            comp_stream: flate2::Compress::new(compression, true),
            in_buf: Vec::new(),
            in_pos: 0,
        })
    }

    /// Provide the next sequence of bytes to be compressed.
    ///
    /// `buffer` holds the bytes to compress (C++ passes `uint1 *buffer` and
    /// `int4 sz`; the slice carries its own length).
    pub fn input(&mut self, buffer: &[u8]) {
        self.in_buf.clear();
        self.in_buf.extend_from_slice(buffer);
        self.in_pos = 0;
    }

    /// Deflate as much as possible into the given buffer.
    ///
    /// Return the number of bytes of output space still available.  Output
    /// may be limited by the amount of space in the output buffer or the
    /// amount of data available in the current input buffer.
    ///
    /// * `buffer` is where compressed bytes are stored (C++ `uint1 *buffer`
    ///   plus `int4 sz`)
    /// * `finish` is set to true if this is the final buffer to add to the
    ///   stream
    ///
    /// C++ throws `LowlevelError` only on `Z_STREAM_ERROR`; `Z_BUF_ERROR`
    /// (no progress possible) is not an error there and maps to
    /// `Status::BufError` here, also not an error.
    pub fn deflate(&mut self, buffer: &mut [u8], finish: bool) -> KunaResult<i32> {
        let flush = if finish { FlushCompress::Finish } else { FlushCompress::None };
        let before_in = self.comp_stream.total_in();
        let before_out = self.comp_stream.total_out();
        if self.comp_stream.compress(&self.in_buf[self.in_pos..], buffer, flush).is_err() {
            return Err(KunaError::lowlevel("Error compressing stream"));
        }
        self.in_pos += (self.comp_stream.total_in() - before_in) as usize;
        let produced = (self.comp_stream.total_out() - before_out) as usize;
        // C++ returns compStream.avail_out (an int4); buffer sizes in the
        // engine are small (4096), so the cast cannot truncate
        Ok((buffer.len() - produced) as i32)
    }
}

/// Wrapper for the inflate algorithm (C++ `Decompress`).
///
/// Initialize/free algorithm resources.  Provide successive arrays of
/// compressed bytes via the [`Decompress::input`] method.  Compute successive
/// arrays of uncompressed bytes via the [`Decompress::inflate`] method.
pub struct Decompress {
    /// The zlib inflate algorithm state (C++ `z_stream compStream`)
    comp_stream: flate2::Decompress,
    /// Set to true if the end of the compressed stream has been reached
    stream_finished: bool,
    /// Pending input bytes (see [`Compress::input`])
    in_buf: Vec<u8>,
    /// Number of bytes of `in_buf` already consumed by the decompressor
    in_pos: usize,
}

impl Decompress {
    /// Initialize the inflate algorithm state.
    ///
    /// C++ calls `inflateInit` and throws `LowlevelError` on failure; the
    /// only failure mode there is memory exhaustion, which Rust treats as an
    /// allocation abort, so construction is infallible here.
    pub fn new() -> Self {
        Decompress {
            // true: expect the zlib header/trailer, as C++ inflateInit does
            comp_stream: flate2::Decompress::new(true),
            stream_finished: false,
            in_buf: Vec::new(),
            in_pos: 0,
        }
    }

    /// Provide the next sequence of compressed bytes.
    ///
    /// `buffer` holds the compressed bytes (C++ passes `uint1 *buffer` and
    /// `int4 sz`).
    pub fn input(&mut self, buffer: &[u8]) {
        self.in_buf.clear();
        self.in_buf.extend_from_slice(buffer);
        self.in_pos = 0;
    }

    /// Return true if the end of the compressed stream has been reached
    /// (C++ `isFinished`).
    pub fn is_finished(&self) -> bool {
        self.stream_finished
    }

    /// Inflate as much as possible into the given buffer.
    ///
    /// Return the number of bytes of output space still available.  Output
    /// may be limited by the amount of space in the output buffer or the
    /// amount of data available in the current input buffer.
    ///
    /// * `buffer` is where uncompressed bytes are stored
    ///
    /// C++ throws `LowlevelError` on `Z_NEED_DICT`, `Z_DATA_ERROR`,
    /// `Z_MEM_ERROR` and `Z_STREAM_ERROR` — exactly the conditions flate2
    /// reports as `Err` — and sets `streamFinished` on `Z_STREAM_END`.
    pub fn inflate(&mut self, buffer: &mut [u8]) -> KunaResult<i32> {
        let before_in = self.comp_stream.total_in();
        let before_out = self.comp_stream.total_out();
        // C++ passes Z_NO_FLUSH
        match self.comp_stream.decompress(&self.in_buf[self.in_pos..], buffer, FlushDecompress::None)
        {
            Err(_) => Err(KunaError::lowlevel("Error decompressing stream")),
            Ok(status) => {
                self.in_pos += (self.comp_stream.total_in() - before_in) as usize;
                if matches!(status, Status::StreamEnd) {
                    self.stream_finished = true;
                }
                let produced = (self.comp_stream.total_out() - before_out) as usize;
                Ok((buffer.len() - produced) as i32)
            }
        }
    }
}

impl Default for Decompress {
    fn default() -> Self {
        Decompress::new()
    }
}

/// Number of bytes in the *input* buffer (C++ `static const int4`, value 4096)
const IN_BUFFER_SIZE: usize = 4096;
/// Number of bytes in the *output* buffer (C++ `static const int4`, value 4096)
const OUT_BUFFER_SIZE: usize = 4096;

/// Stream buffer that performs compression (C++ `CompressBuffer`).
///
/// The C++ class is a `std::streambuf` filter; the Rust port implements
/// [`std::io::Write`] over a backing writer that is the ultimate destination
/// of the compressed bytes.  After writing the full sequence of bytes to be
/// compressed, make sure to call [`Write::flush`] (the port of the C++
/// `sync()`) to emit the final compressed bytes to the backing stream — it
/// finishes the deflate stream, so call it exactly once, after all input has
/// been written (the same contract as the C++ class, whose destructor does
/// not flush either).
pub struct CompressBuffer<W: Write> {
    /// The backing stream receiving compressed bytes (C++ `ostream &outStream`)
    out_stream: W,
    /// The *input* buffer; its length is the C++ put-area fill `pptr()-pbase()`
    in_buffer: Vec<u8>,
    /// The *output* buffer
    out_buffer: Vec<u8>,
    /// Compressor state
    compressor: Compress,
}

impl<W: Write> CompressBuffer<W> {
    /// Constructor.
    ///
    /// * `s` is the backing output stream
    /// * `level` is the level of compression (see [`Compress::new`])
    pub fn new(s: W, level: i32) -> KunaResult<Self> {
        Ok(CompressBuffer {
            out_stream: s,
            in_buffer: Vec::with_capacity(IN_BUFFER_SIZE),
            out_buffer: vec![0; OUT_BUFFER_SIZE],
            compressor: Compress::new(level)?,
        })
    }

    /// Compress the current set of bytes in the *input* buffer (C++
    /// `flushInput`).
    ///
    /// The compressor is called repeatedly and its output is written to the
    /// backing stream until the compressor can no longer fill the *output*
    /// buffer.
    ///
    /// `last_buffer` is true if this is the final set of bytes to add to the
    /// compressed stream.
    ///
    /// NOTE: C++ writes to the backing `ostream` without checking its state
    /// (failures silently set badbit); the Rust `Write` contract propagates
    /// them instead.
    fn flush_input(&mut self, last_buffer: bool) -> std::io::Result<()> {
        // int len = pptr() - pbase(); compressor.input((uint1 *)pbase(),len);
        self.compressor.input(&self.in_buffer);
        // do { outAvail = OUT_BUFFER_SIZE; outAvail = deflate(...); write(...);
        // } while(outAvail == 0);
        loop {
            let out_avail = self
                .compressor
                .deflate(&mut self.out_buffer, last_buffer)
                .map_err(std::io::Error::other)? as usize;
            self.out_stream.write_all(&self.out_buffer[..OUT_BUFFER_SIZE - out_avail])?;
            if out_avail != 0 {
                break;
            }
        }
        // pbump(-len): reset the put area
        self.in_buffer.clear();
        Ok(())
    }
}

impl<W: Write> Write for CompressBuffer<W> {
    /// Buffer the bytes, compressing each time the *input* buffer fills.
    ///
    /// Mirrors the C++ streambuf behavior: bytes accumulate in the put area
    /// and `overflow()` hands a full `IN_BUFFER_SIZE` chunk to the compressor
    /// (the deflate output is identical regardless of `Z_NO_FLUSH` chunk
    /// boundaries, so only the chunk *size* is structural).
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut written = 0;
        while written < buf.len() {
            if self.in_buffer.len() == IN_BUFFER_SIZE {
                // overflow(c): pass the filled input buffer to the compressor
                self.flush_input(false)?;
            }
            let take = (IN_BUFFER_SIZE - self.in_buffer.len()).min(buf.len() - written);
            self.in_buffer.extend_from_slice(&buf[written..written + take]);
            written += take;
        }
        Ok(buf.len())
    }

    /// Pass remaining bytes in the input buffer to the compressor and finish
    /// the compressed stream (C++ `sync()`; returns 0 for success there).
    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_input(true)?;
        self.out_stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Patterned but not trivially-constant input, larger than the 4096-byte
    /// buffers so CompressBuffer exercises its overflow path.
    fn sample_data(len: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(len);
        let phrase = b"kuna sleigh spec payload ";
        let mut i = 0usize;
        while v.len() < len {
            v.push(phrase[i % phrase.len()] ^ ((i / 251) as u8));
            i += 1;
        }
        v
    }

    /// Compress `data` through the chunked Compress API in one input() call,
    /// finishing the stream.
    fn deflate_all(data: &[u8], level: i32) -> Vec<u8> {
        let mut compressor = Compress::new(level).unwrap();
        compressor.input(data);
        let mut out = Vec::new();
        let mut buf = [0u8; 256];
        loop {
            let avail = compressor.deflate(&mut buf, true).unwrap() as usize;
            out.extend_from_slice(&buf[..256 - avail]);
            if avail != 0 {
                break;
            }
        }
        out
    }

    /// Decompress through the chunked Decompress API, feeding small input
    /// slices and a small output buffer, following the consumption loop of
    /// slaformat.cc:235-251.
    fn inflate_all(comp: &[u8]) -> Vec<u8> {
        let mut decompressor = Decompress::new();
        let mut out = Vec::new();
        let mut buf = [0u8; 64];
        for chunk in comp.chunks(7) {
            decompressor.input(chunk);
            loop {
                let avail = decompressor.inflate(&mut buf).unwrap() as usize;
                out.extend_from_slice(&buf[..64 - avail]);
                // input chunk exhausted once output space is left over
                if avail != 0 || decompressor.is_finished() {
                    break;
                }
            }
            if decompressor.is_finished() {
                break;
            }
        }
        assert!(decompressor.is_finished());
        out
    }

    /// The level the C++ tree pins (slgh_compile.cc passes -1 =
    /// Z_DEFAULT_COMPRESSION = level 6) must produce the zlib header
    /// `0x78 0x9C` exactly as C zlib does.
    #[test]
    fn test_compression_zlib_header_default_level() {
        let comp = deflate_all(b"kuna", -1);
        assert_eq!(&comp[..2], &[0x78, 0x9c]);
        // Level 0 (stored blocks) is unused in-tree; miniz_oxide emits the
        // header 0x08 0x1D there (CINFO=0, a smaller declared window) where
        // C zlib emits 0x78 0x01 — a valid, decodable stream either way, so
        // only the round trip is asserted (deviation noted in LOSS-004 terms).
        let comp0 = deflate_all(b"kuna", 0);
        assert_eq!(inflate_all(&comp0), b"kuna");
    }

    /// Round trip a payload larger than both 4096-byte buffers through
    /// CompressBuffer (write/flush) and Decompress (chunked inflate loop).
    #[test]
    fn test_compression_round_trip_chunked() {
        let data = sample_data(20000);
        let mut compressed: Vec<u8> = Vec::new();
        let mut filter = CompressBuffer::new(&mut compressed, -1).unwrap();
        // write in awkward chunk sizes to exercise the put-area refill logic
        for chunk in data.chunks(1000) {
            filter.write_all(chunk).unwrap();
        }
        filter.flush().unwrap();
        drop(filter);
        assert_eq!(&compressed[..2], &[0x78, 0x9c]);
        assert!(compressed.len() < data.len());
        assert_eq!(inflate_all(&compressed), data);
    }

    /// CompressBuffer's 4096-byte chunking must not change the produced
    /// stream relative to a single-shot Compress of the same bytes
    /// (Z_NO_FLUSH boundaries are not observable in deflate output).
    #[test]
    fn test_compression_buffer_matches_direct_compress() {
        let data = sample_data(10000);
        let direct = deflate_all(&data, -1);
        let mut via_buffer: Vec<u8> = Vec::new();
        let mut filter = CompressBuffer::new(&mut via_buffer, -1).unwrap();
        filter.write_all(&data).unwrap();
        filter.flush().unwrap();
        drop(filter);
        assert_eq!(via_buffer, direct);
    }

    /// Decode a stream produced by C zlib itself (python3 `zlib.compress` at
    /// the default level — the same library the C++ oracle links), proving
    /// wire-format interoperability per LOSS-004.
    #[test]
    fn test_compression_decompress_c_zlib_oracle_stream() {
        let expect = b"kuna .sla payload: kuna .sla payload: kuna .sla payload!\n";
        let comp: [u8; 32] = [
            0x78, 0x9c, 0xcb, 0x2e, 0xcd, 0x4b, 0x54, 0xd0, 0x2b, 0xce, 0x49, 0x54, 0x28, 0x48,
            0xac, 0xcc, 0xc9, 0x4f, 0x4c, 0xb1, 0x52, 0xc8, 0x26, 0x42, 0x48, 0x91, 0x0b, 0x00,
            0x47, 0x5f, 0x13, 0xb5,
        ];
        assert_eq!(inflate_all(&comp), expect);
    }

    /// C++ deflateInit returns Z_STREAM_ERROR for levels outside -1..=9 and
    /// the constructor throws LowlevelError.
    #[test]
    fn test_compression_invalid_level_rejected() {
        for level in [-2, 10, 100] {
            match Compress::new(level) {
                Err(KunaError::Lowlevel { explain }) => {
                    assert_eq!(explain, "Could not initialize deflate stream state");
                }
                _ => panic!("level {} should be rejected", level),
            }
        }
        // valid boundary levels construct fine
        assert!(Compress::new(-1).is_ok());
        assert!(Compress::new(0).is_ok());
        assert!(Compress::new(9).is_ok());
    }

    /// Corrupt input makes inflate fail the way C++ converts Z_DATA_ERROR.
    #[test]
    fn test_compression_corrupt_stream_errors() {
        let mut decompressor = Decompress::new();
        // invalid zlib header bytes
        decompressor.input(&[0xde, 0xad, 0xbe, 0xef, 0x00, 0x01, 0x02, 0x03]);
        let mut buf = [0u8; 64];
        match decompressor.inflate(&mut buf) {
            Err(KunaError::Lowlevel { explain }) => {
                assert_eq!(explain, "Error decompressing stream");
            }
            other => panic!("expected decompression error, got {:?}", other.map(|_| ())),
        }
        assert!(!decompressor.is_finished());
    }
}
