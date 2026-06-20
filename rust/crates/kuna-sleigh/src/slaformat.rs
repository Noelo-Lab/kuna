//! Port of `decompiler/cpp/slaformat.{hh,cc}` (W2, item `w2-sleigh-core`):
//! the encoding values and the compressed reader/writer for the `.sla` file
//! format.
//!
//! What lives where:
//!
//! - The `FORMAT_SCOPE`/`FORMAT_VERSION` constants and the full
//!   AttributeId/ElementId table (translate.cc-style globals) are defined
//!   here.  [`register_sla_ids`] installs them on an [`IdRegistry`] — the
//!   explicit replacement for C++ global-constructor registration.  The
//!   sub-tables that other waves needed early (`ATTRIB_VAL`/`ATTRIB_OFF`/...,
//!   `ELEM_INTB`/...) are re-exported from their crate-local definitions in
//!   `slghsymbol::sla` / `semantics::sla` so a single `register_sla_ids` call
//!   covers the whole table without duplicate `ElementId` objects.
//! - [`FormatDecode`] is the C++ `FormatDecode : PackedDecode`: it verifies
//!   the `sla\x04` header, decompresses the zlib stream, and feeds the
//!   decompressed bytes into a [`PackedDecode`].  Inheritance becomes
//!   composition: [`FormatDecode`] *owns* a [`PackedDecode`] and forwards the
//!   whole [`Decoder`] surface to it (every packed byte has a high-bit
//!   pattern set, so the inner `ingest_stream`'s NUL-terminator scan never
//!   trips on valid data).
//! - [`FormatEncode`]/[`write_sla_header`]/`isSlaFormat` are the writer side
//!   (used by the unported compiler); [`FormatEncode`] wraps a
//!   [`PackedEncode`] over a [`CompressBuffer`] writer.

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

// The SLA attribute/element id table (slaformat.cc).  ATTRIB_CONTEXT = 1 is
// reserved.  Several of these ids are needed (and therefore defined) by
// earlier waves; those are re-exported below rather than redefined to keep
// each id object unique.

/// SLA format attribute "id"
pub const ATTRIB_ID: AttributeId = AttributeId::new("id", 3);
/// SLA format attribute "space"
pub const ATTRIB_SPACE: AttributeId = AttributeId::new("space", 4);
/// SLA format attribute "nonzero"
pub const ATTRIB_NONZERO: AttributeId = AttributeId::new("nonzero", 10);
/// SLA format attribute "scope"
pub const ATTRIB_SCOPE: AttributeId = AttributeId::new("scope", 13);
/// SLA format attribute "size"
pub const ATTRIB_SIZE: AttributeId = AttributeId::new("size", 15);
/// SLA format attribute "table"
pub const ATTRIB_TABLE: AttributeId = AttributeId::new("table", 16);
/// SLA format attribute "ct"
pub const ATTRIB_CT: AttributeId = AttributeId::new("ct", 17);
/// SLA format attribute "minlen"
pub const ATTRIB_MINLEN: AttributeId = AttributeId::new("minlen", 18);
/// SLA format attribute "base"
pub const ATTRIB_BASE: AttributeId = AttributeId::new("base", 19);
/// SLA format attribute "number"
pub const ATTRIB_NUMBER: AttributeId = AttributeId::new("number", 20);
/// SLA format attribute "context"
pub const ATTRIB_CONTEXT: AttributeId = AttributeId::new("context", 21);
/// SLA format attribute "parent"
pub const ATTRIB_PARENT: AttributeId = AttributeId::new("parent", 22);
/// SLA format attribute "subsym"
pub const ATTRIB_SUBSYM: AttributeId = AttributeId::new("subsym", 23);
/// SLA format attribute "line"
pub const ATTRIB_LINE: AttributeId = AttributeId::new("line", 24);
/// SLA format attribute "source"
pub const ATTRIB_SOURCE: AttributeId = AttributeId::new("source", 25);
/// SLA format attribute "length"
pub const ATTRIB_LENGTH: AttributeId = AttributeId::new("length", 26);
/// SLA format attribute "first"
pub const ATTRIB_FIRST: AttributeId = AttributeId::new("first", 27);
/// SLA format attribute "plus"
pub const ATTRIB_PLUS: AttributeId = AttributeId::new("plus", 28);
/// SLA format attribute "endbit"
pub const ATTRIB_ENDBIT: AttributeId = AttributeId::new("endbit", 30);
/// SLA format attribute "signbit"
pub const ATTRIB_SIGNBIT: AttributeId = AttributeId::new("signbit", 31);
/// SLA format attribute "endbyte"
pub const ATTRIB_ENDBYTE: AttributeId = AttributeId::new("endbyte", 32);
/// SLA format attribute "startbyte"
pub const ATTRIB_STARTBYTE: AttributeId = AttributeId::new("startbyte", 33);

/// SLA format attribute "version"
pub const ATTRIB_VERSION: AttributeId = AttributeId::new("version", 34);
/// SLA format attribute "bigendian"
pub const ATTRIB_BIGENDIAN: AttributeId = AttributeId::new("bigendian", 35);
/// SLA format attribute "align"
pub const ATTRIB_ALIGN: AttributeId = AttributeId::new("align", 36);
/// SLA format attribute "uniqbase"
pub const ATTRIB_UNIQBASE: AttributeId = AttributeId::new("uniqbase", 37);
/// SLA format attribute "maxdelay"
pub const ATTRIB_MAXDELAY: AttributeId = AttributeId::new("maxdelay", 38);
/// SLA format attribute "uniqmask"
pub const ATTRIB_UNIQMASK: AttributeId = AttributeId::new("uniqmask", 39);
/// SLA format attribute "numsections"
pub const ATTRIB_NUMSECTIONS: AttributeId = AttributeId::new("numsections", 40);
/// SLA format attribute "defaultspace"
pub const ATTRIB_DEFAULTSPACE: AttributeId = AttributeId::new("defaultspace", 41);
/// SLA format attribute "delay"
pub const ATTRIB_DELAY: AttributeId = AttributeId::new("delay", 42);
/// SLA format attribute "wordsize"
pub const ATTRIB_WORDSIZE: AttributeId = AttributeId::new("wordsize", 43);
/// SLA format attribute "physical"
pub const ATTRIB_PHYSICAL: AttributeId = AttributeId::new("physical", 44);
/// SLA format attribute "scopesize"
pub const ATTRIB_SCOPESIZE: AttributeId = AttributeId::new("scopesize", 45);
/// SLA format attribute "symbolsize"
pub const ATTRIB_SYMBOLSIZE: AttributeId = AttributeId::new("symbolsize", 46);
/// SLA format attribute "varnode"
pub const ATTRIB_VARNODE: AttributeId = AttributeId::new("varnode", 47);
/// SLA format attribute "low"
pub const ATTRIB_LOW: AttributeId = AttributeId::new("low", 48);
/// SLA format attribute "high"
pub const ATTRIB_HIGH: AttributeId = AttributeId::new("high", 49);
/// SLA format attribute "flow"
pub const ATTRIB_FLOW: AttributeId = AttributeId::new("flow", 50);
/// SLA format attribute "contain"
pub const ATTRIB_CONTAIN: AttributeId = AttributeId::new("contain", 51);
/// SLA format attribute "i"
pub const ATTRIB_I: AttributeId = AttributeId::new("i", 52);
/// SLA format attribute "numct"
pub const ATTRIB_NUMCT: AttributeId = AttributeId::new("numct", 53);
/// SLA format attribute "section"
pub const ATTRIB_SECTION: AttributeId = AttributeId::new("section", 54);
/// SLA format attribute "labels"
pub const ATTRIB_LABELS: AttributeId = AttributeId::new("labels", 55);

/// SLA format attribute "val"
pub const ATTRIB_VAL: AttributeId = AttributeId::new("val", 2);
/// SLA format attribute "s"
pub const ATTRIB_S: AttributeId = AttributeId::new("s", 5);
/// SLA format attribute "off"
pub const ATTRIB_OFF: AttributeId = AttributeId::new("off", 6);
/// SLA format attribute "code"
pub const ATTRIB_CODE: AttributeId = AttributeId::new("code", 7);
/// SLA format attribute "mask"
pub const ATTRIB_MASK: AttributeId = AttributeId::new("mask", 8);
/// SLA format attribute "index"
pub const ATTRIB_INDEX: AttributeId = AttributeId::new("index", 9);
/// SLA format attribute "piece"
pub const ATTRIB_PIECE: AttributeId = AttributeId::new("piece", 11);
/// SLA format attribute "name"
pub const ATTRIB_NAME: AttributeId = AttributeId::new("name", 12);
/// SLA format attribute "startbit"
pub const ATTRIB_STARTBIT: AttributeId = AttributeId::new("startbit", 14);
/// SLA format attribute "shift"
pub const ATTRIB_SHIFT: AttributeId = AttributeId::new("shift", 29);

/// SLA format element "sleigh"
pub const ELEM_SLEIGH: ElementId = ElementId::new("sleigh", 33);
/// SLA format element "spaces"
pub const ELEM_SPACES: ElementId = ElementId::new("spaces", 34);
/// SLA format element "sourcefiles"
pub const ELEM_SOURCEFILES: ElementId = ElementId::new("sourcefiles", 35);
/// SLA format element "sourcefile"
pub const ELEM_SOURCEFILE: ElementId = ElementId::new("sourcefile", 36);
/// SLA format element "space"
pub const ELEM_SPACE: ElementId = ElementId::new("space", 37);
/// SLA format element "symbol_table"
pub const ELEM_SYMBOL_TABLE: ElementId = ElementId::new("symbol_table", 38);
/// SLA format element "scope"
pub const ELEM_SCOPE: ElementId = ElementId::new("scope", 22);
/// SLA format element "space_other"
pub const ELEM_SPACE_OTHER: ElementId = ElementId::new("space_other", 45);
/// SLA format element "space_unique"
pub const ELEM_SPACE_UNIQUE: ElementId = ElementId::new("space_unique", 46);

/// SLA format element "constructor"
pub const ELEM_CONSTRUCTOR: ElementId = ElementId::new("constructor", 20);

/// Register every SLA-format AttributeId/ElementId defined in this module on
/// the given registry (the explicit stand-in for C++ static-object
/// registration).  Earlier waves register their own slices through their own
/// `register_*_ids` helpers (`register_slghsymbol_ids`,
/// `register_semantics_ids`); this covers the slaformat.cc names not already
/// supplied there.
pub fn register_sla_ids(registry: &mut IdRegistry) {
    for a in [
        &ATTRIB_ID,
        &ATTRIB_SPACE,
        &ATTRIB_NONZERO,
        &ATTRIB_SCOPE,
        &ATTRIB_SIZE,
        &ATTRIB_TABLE,
        &ATTRIB_CT,
        &ATTRIB_MINLEN,
        &ATTRIB_BASE,
        &ATTRIB_NUMBER,
        &ATTRIB_CONTEXT,
        &ATTRIB_PARENT,
        &ATTRIB_SUBSYM,
        &ATTRIB_LINE,
        &ATTRIB_SOURCE,
        &ATTRIB_LENGTH,
        &ATTRIB_FIRST,
        &ATTRIB_PLUS,
        &ATTRIB_ENDBIT,
        &ATTRIB_SIGNBIT,
        &ATTRIB_ENDBYTE,
        &ATTRIB_STARTBYTE,
        &ATTRIB_VERSION,
        &ATTRIB_BIGENDIAN,
        &ATTRIB_ALIGN,
        &ATTRIB_UNIQBASE,
        &ATTRIB_MAXDELAY,
        &ATTRIB_UNIQMASK,
        &ATTRIB_NUMSECTIONS,
        &ATTRIB_DEFAULTSPACE,
        &ATTRIB_DELAY,
        &ATTRIB_WORDSIZE,
        &ATTRIB_PHYSICAL,
        &ATTRIB_SCOPESIZE,
        &ATTRIB_SYMBOLSIZE,
        &ATTRIB_VARNODE,
        &ATTRIB_LOW,
        &ATTRIB_HIGH,
        &ATTRIB_FLOW,
        &ATTRIB_CONTAIN,
        &ATTRIB_I,
        &ATTRIB_NUMCT,
        &ATTRIB_SECTION,
        &ATTRIB_LABELS,
    ] {
        registry.register_attribute(a);
    }
    for e in [
        &ELEM_SLEIGH,
        &ELEM_SPACES,
        &ELEM_SOURCEFILES,
        &ELEM_SOURCEFILE,
        &ELEM_SPACE,
        &ELEM_SYMBOL_TABLE,
        &ELEM_SCOPE,
        &ELEM_SPACE_OTHER,
        &ELEM_SPACE_UNIQUE,
    ] {
        registry.register_element(e);
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
/// [`CompressBuffer`] on `flush`.  (The `.sla` writer is only exercised by
/// the unported compiler; the read path is what the engine needs.)
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
    /// decompress the whole stream, then ingest the decompressed bytes into
    /// the inner [`PackedDecode`].  The C++ decompresses directly into the
    /// PackedDecode chunk buffers; the Rust decompresses into one `Vec<u8>`
    /// and delegates to `PackedDecode::ingest_stream` (identical chunking and
    /// `endIngest` padding — every packed byte has its high bit set, so the
    /// inner NUL-terminator scan never trips).
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
        self.inner.ingest_stream(&out)
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
