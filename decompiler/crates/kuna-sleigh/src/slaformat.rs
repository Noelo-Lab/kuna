//! Binary SLA headers, compression and the shared format-ID vocabulary.
//!
//! [`FormatDecode`] decompresses a `.sla` image into a packed decoder.
//! [`FormatEncode`] buffers a packed element stream and compresses it on flush.
//! XML callers use the same IDs through [`register_sla_ids`].

use kuna_base::compression::{CompressBuffer, Decompress};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, IdRegistry, PackedDecode, PackedEncode,
};
use kuna_base::space::AddrSpaceManager;
use std::io::Write;
use std::rc::Rc;

/// C++ `sla::FORMAT_SCOPE` — grouping scope for SLA format elements/attributes.
pub const FORMAT_SCOPE: i32 = 1;
/// C++ `sla::FORMAT_VERSION` — current version of the `.sla` file.
pub const FORMAT_VERSION: i32 = 4;

pub mod ids;
pub use ids::*;

/// Register the complete SLA vocabulary for an XML decoder's dedicated registry.
pub fn register_sla_ids(registry: &mut IdRegistry) {
    for attribute in ids::ATTRIBUTE_IDS {
        registry.register_attribute(attribute);
    }
    for element in ids::ELEMENT_IDS {
        registry.register_element(element);
    }
}

/// The bytes of the header are read from the stream and verified against the
/// required form and current version.  Returns `true` if a valid header is
/// present.  No additional bytes are consumed beyond the four header bytes.
/// (C++ reads from an `istream`; the Rust takes the leading bytes of a slice
/// and returns the remaining tail alongside the verdict.)
pub fn is_sla_format(s: &[u8]) -> (bool, &[u8]) {
    if s.len() < 4 {
        return (false, s);
    }
    let header = &s[..4];
    let rest = &s[4..];
    if header[0] != b's' || header[1] != b'l' || header[2] != b'a' {
        return (false, rest);
    }
    // C++ `header[3] != FORMAT_VERSION`: the version byte is one raw byte.
    if i32::from(header[3]) != FORMAT_VERSION {
        return (false, rest);
    }
    (true, rest)
}

/// Write a valid header, including the format version number, to the writer.
pub fn write_sla_header<W: Write>(s: &mut W) -> std::io::Result<()> {
    // C++ `header[3] = FORMAT_VERSION`: int4 -> char truncation (version 4).
    let header = [b's', b'l', b'a', FORMAT_VERSION as u8];
    s.write_all(&header)
}

/// The encoder for the `.sla` file format (C++ `FormatEncode : PackedEncode`).
///
/// This provides the format header, does compression, and encodes the raw
/// data elements/attributes.  The C++ class layers a `PackedEncode` over a
/// `CompressBuffer` `streambuf`; the Rust composes a [`PackedEncode`] that
/// writes into a `Vec<u8>`, then compresses that buffer through a
/// [`CompressBuffer`] on `flush`.
pub struct FormatEncode<W: Write> {
    /// The uncompressed packed bytes accumulated so far.
    buffer: Vec<u8>,
    /// The backing writer that receives the header + compressed bytes.
    out: W,
    /// Compression level (`-1` = zlib default, matching the C++ tree).
    level: i32,
    /// Whether the header has been written to `out` yet.
    header_written: bool,
}

impl<W: Write> FormatEncode<W> {
    /// Initialize an encoder at a specific compression level (C++ writes the
    /// header to the backing stream in the constructor).
    pub fn new(out: W, level: i32) -> FormatEncode<W> {
        FormatEncode { buffer: Vec::new(), out, level, header_written: false }
    }

    /// Borrow the accumulating packed buffer as a [`PackedEncode`] for the
    /// duration of one encode pass.
    pub fn packed(&mut self) -> PackedEncode<'_> {
        PackedEncode::new(&mut self.buffer)
    }

    /// Flush any buffered bytes in the encoder to the backing stream
    /// (C++ `FormatEncode::flush` -> `compStream.flush()`).
    pub fn flush(&mut self) -> KunaResult<()> {
        if !self.header_written {
            write_sla_header(&mut self.out)
                .map_err(|e| KunaError::lowlevel(format!("Could not write sla header: {e}")))?;
            self.header_written = true;
        }
        let mut comp = CompressBuffer::new(&mut self.out, self.level)?;
        comp.write_all(&self.buffer)
            .map_err(|e| KunaError::lowlevel(format!("Error compressing sla stream: {e}")))?;
        comp.flush()
            .map_err(|e| KunaError::lowlevel(format!("Error flushing sla stream: {e}")))?;
        self.buffer.clear();
        Ok(())
    }
}

/// The decoder for the `.sla` file format (C++ `FormatDecode : PackedDecode`).
///
/// This verifies the `.sla` file header, does decompression, and decodes the
/// raw data elements/attributes.  C++ inherits `PackedDecode`; the Rust owns
/// one and forwards the [`Decoder`] surface to it.
pub struct FormatDecode<'a> {
    inner: PackedDecode<'a>,
}

impl<'a> FormatDecode<'a> {
    /// C++ `FormatDecode::FormatDecode(const AddrSpaceManager *)`.
    pub fn new(spc_manager: &'a AddrSpaceManager) -> FormatDecode<'a> {
        FormatDecode { inner: PackedDecode::new(spc_manager) }
    }

    /// C++ `FormatDecode::ingestStream(istream &)`: verify the header,
    /// decompress the whole stream, then hand the decompressed buffer to the
    /// inner [`PackedDecode`].  The C++ decompresses directly into the
    /// PackedDecode chunk buffers; the Rust decompresses into one `Vec<u8>`
    /// and transfers it with `ingest_owned`, which copies only the trailing
    /// partial chunk so `endIngest` has one to pad — every packed byte has its
    /// high bit set, so the NUL-terminator scan never trips.
    pub fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()> {
        let (ok, compressed) = is_sla_format(s);
        if !ok {
            return Err(KunaError::lowlevel("Missing SLA format header"));
        }
        let mut decompressor = Decompress::new();
        let mut out: Vec<u8> = Vec::new();
        // The C++ `Decompress` keeps a `z_stream next_in` pointer INTO the
        // caller's buffer, so unconsumed input survives across `inflate`
        // calls.  The kuna `Decompress::input` instead REPLACES its internal
        // buffer (dropping any unconsumed tail), so feeding the compressed
        // stream in IN_BUFFER_SIZE chunks like the C++ would discard the
        // bytes inflate had not yet consumed when an output buffer filled.
        // Since the whole `.sla` is already in memory, feed it all at once
        // (the decompressor advances its own `in_pos` across inflate calls,
        // exactly matching the C++ next_in cursor) and inflate fixed-size
        // output chunks until the stream ends.
        decompressor.input(compressed);
        while !decompressor.is_finished() {
            let mut buf = vec![0u8; OUT_BUFFER_SIZE];
            let avail = decompressor.inflate(&mut buf)?;
            // avail = bytes of space still available in buf; produced =
            // OUT_BUFFER_SIZE - avail.
            let produced = OUT_BUFFER_SIZE - avail as usize; // avail in [0,OUT_BUFFER_SIZE]
            out.extend_from_slice(&buf[..produced]);
            // No progress and not finished => truncated/corrupt stream.
            if produced == 0 && !decompressor.is_finished() {
                return Err(KunaError::lowlevel("Truncated SLA compressed stream"));
            }
        }
        self.inner.ingest_owned(out)
    }

    /// Borrow the inner [`PackedDecode`] as a `&mut dyn Decoder`.
    pub fn as_decoder(&mut self) -> &mut dyn Decoder {
        &mut self.inner
    }
}

/// The inflate output chunk size (C++ uses `PackedDecode::BUFFER_SIZE`).
const OUT_BUFFER_SIZE: usize = 4096;

// Forward the entire Decoder surface from FormatDecode to the inner
// PackedDecode (C++ inheritance).
impl Decoder for FormatDecode<'_> {
    fn get_addr_space_manager(&self) -> &AddrSpaceManager {
        self.inner.get_addr_space_manager()
    }
    fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()> {
        FormatDecode::ingest_stream(self, s)
    }
    fn peek_element(&mut self) -> KunaResult<u32> {
        self.inner.peek_element()
    }
    fn open_element(&mut self) -> KunaResult<u32> {
        self.inner.open_element()
    }
    fn open_element_id(&mut self, elem_id: &ElementId) -> KunaResult<u32> {
        self.inner.open_element_id(elem_id)
    }
    fn close_element(&mut self, id: u32) -> KunaResult<()> {
        self.inner.close_element(id)
    }
    fn close_element_skipping(&mut self, id: u32) -> KunaResult<()> {
        self.inner.close_element_skipping(id)
    }
    fn get_next_attribute_id(&mut self) -> KunaResult<u32> {
        self.inner.get_next_attribute_id()
    }
    fn get_indexed_attribute_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u32> {
        self.inner.get_indexed_attribute_id(attrib_id)
    }
    fn rewind_attributes(&mut self) {
        self.inner.rewind_attributes()
    }
    fn read_bool(&mut self) -> KunaResult<bool> {
        self.inner.read_bool()
    }
    fn read_bool_id(&mut self, attrib_id: &AttributeId) -> KunaResult<bool> {
        self.inner.read_bool_id(attrib_id)
    }
    fn read_signed_integer(&mut self) -> KunaResult<i64> {
        self.inner.read_signed_integer()
    }
    fn read_signed_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<i64> {
        self.inner.read_signed_integer_id(attrib_id)
    }
    fn read_signed_integer_expect_string(
        &mut self,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        self.inner.read_signed_integer_expect_string(expect, expectval)
    }
    fn read_signed_integer_expect_string_id(
        &mut self,
        attrib_id: &AttributeId,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        self.inner.read_signed_integer_expect_string_id(attrib_id, expect, expectval)
    }
    fn read_unsigned_integer(&mut self) -> KunaResult<u64> {
        self.inner.read_unsigned_integer()
    }
    fn read_unsigned_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u64> {
        self.inner.read_unsigned_integer_id(attrib_id)
    }
    fn read_string(&mut self) -> KunaResult<Vec<u8>> {
        self.inner.read_string()
    }
    fn read_string_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Vec<u8>> {
        self.inner.read_string_id(attrib_id)
    }
    fn read_space(&mut self) -> KunaResult<Rc<kuna_base::space::AddrSpace>> {
        self.inner.read_space()
    }
    fn read_space_id(
        &mut self,
        attrib_id: &AttributeId,
    ) -> KunaResult<Rc<kuna_base::space::AddrSpace>> {
        self.inner.read_space_id(attrib_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::marshal::{Encoder, ATTRIB_CONTENT, ELEM_DATA};

    fn compressed(payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_sla_header(&mut bytes).unwrap();
        let mut compressor = CompressBuffer::new(&mut bytes, -1).unwrap();
        compressor.write_all(payload).unwrap();
        compressor.flush().unwrap();
        bytes
    }

    #[test]
    fn owned_sla_buffer_decodes_across_inflate_and_packed_boundaries() {
        let manager = AddrSpaceManager::new();
        for length in [1018, 1019, 1020, 4090, 4091, 4092, 8192, 20000] {
            let payload: Vec<_> = (0..length).map(|i| (i % 255 + 1) as u8).collect();
            let mut packed = Vec::new();
            let mut enc = PackedEncode::new(&mut packed);
            enc.open_element(&ELEM_DATA);
            enc.write_string(&ATTRIB_CONTENT, &payload);
            enc.close_element(&ELEM_DATA);
            let bytes = compressed(&packed);
            let mut dec = FormatDecode::new(&manager);
            dec.ingest_stream(&bytes).unwrap();
            let el = dec.open_element_id(&ELEM_DATA).unwrap();
            assert_eq!(dec.read_string_id(&ATTRIB_CONTENT).unwrap(), payload);
            dec.close_element(el).unwrap();
            assert_eq!(dec.peek_element().unwrap(), 0);
        }
    }

    #[test]
    fn owned_sla_buffer_keeps_header_truncation_and_checksum_checks() {
        let manager = AddrSpaceManager::new();
        let bytes = compressed(&[0x41, 0x81]);
        for end in 0..bytes.len() {
            let mut dec = FormatDecode::new(&manager);
            assert!(dec.ingest_stream(&bytes[..end]).is_err(), "truncation at {end}");
        }
        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        assert!(FormatDecode::new(&manager).ingest_stream(&corrupt).is_err());
        let mut wrong_header = bytes;
        wrong_header[3] = 0;
        assert!(FormatDecode::new(&manager).ingest_stream(&wrong_header).is_err());
        assert!(FormatDecode::new(&manager).ingest_stream(&compressed(&[])).is_err());
    }

    #[test]
    fn nul_termination_does_not_skip_the_compressed_checksum() {
        let manager = AddrSpaceManager::new();
        let mut bytes = compressed(&[0x41, 0x81, 0, 0x41, 0x81]);
        let mut dec = FormatDecode::new(&manager);
        dec.ingest_stream(&bytes).unwrap();
        let el = dec.open_element().unwrap();
        dec.close_element(el).unwrap();
        assert_eq!(dec.peek_element().unwrap(), 0);
        *bytes.last_mut().unwrap() ^= 1;
        assert!(FormatDecode::new(&manager).ingest_stream(&bytes).is_err());
    }
}

// The opcode-reading extension (kuna-num) — needed because ConstructTpl /
// OpTpl decode through the OpcodeDecoder surface; forward to the inner
// PackedDecode (C++ inheritance).
impl kuna_num::opcodes::OpcodeDecoder for FormatDecode<'_> {
    fn read_opcode(&mut self) -> KunaResult<kuna_num::opcodes::OpCode> {
        self.inner.read_opcode()
    }
    fn read_opcode_id(
        &mut self,
        attrib_id: &AttributeId,
    ) -> KunaResult<kuna_num::opcodes::OpCode> {
        self.inner.read_opcode_id(attrib_id)
    }
}
