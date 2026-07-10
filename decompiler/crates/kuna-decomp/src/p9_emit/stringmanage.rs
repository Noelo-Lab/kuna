//! Port of `decompiler/cpp/stringmanage.{cc,hh}` — string-literal recovery and
//! UTF-8 re-encoding for C output.
//!
//! `StringManager` looks at data in the loadimage to decide whether it
//! represents a "string", decodes it (UTF-8/UTF-16/UTF-32 -> UTF-8), caches the
//! decoded bytes keyed by [`Address`], and hands them back for presentation.
//! `StringManagerUnicode` is the concrete implementation that understands
//! terminated unicode strings.
//!
//! # Fidelity notes
//!
//! * The encoding-detection and re-encoding rules ([`StringManager::get_codepoint`],
//!   [`StringManager::check_characters`], [`StringManager::write_utf8`],
//!   [`StringManager::write_unicode`]) are transcribed branch-for-branch from the
//!   C++ — every surrogate / continuation-byte / codepoint-range check is
//!   replicated exactly because they decide whether a literal is emitted at all.
//! * `write_utf8` encodes a codepoint as 1/2/3/4 UTF-8 bytes with the same
//!   `mostsigbit_set`-driven width selection and the same `^`-based byte
//!   construction; the bytes are pushed onto a `Vec<u8>` (the C++ writes them to
//!   an `ostream`).
//! * `encode`/`decode` reproduce the `<stringmanage>` / `<string>` / `<bytes>`
//!   marshalling, including the exact hex-content layout (a leading `\n`, two
//!   lowercase hex digits per byte, a `\n  ` break every 20 bytes, a trailing
//!   `\n`).  The `BTreeMap` keying replicates the C++ `map<Address,StringData>`
//!   iteration order (ascending by `Address`).
//!
//! # Boundaries
//!
//! * [`StringManager::register_internal_string_data`] reaches the constant space
//!   via `addr.getSpace()->getManager()->getConstant(hash)` in C++.  `AddrSpace`
//!   in the Rust tree holds no back-pointer to its manager, so the manager is
//!   threaded in as an explicit `&AddrSpaceManager` parameter.
//! * [`StringManagerUnicode::get_string_data`] needs `glb->loader` (a
//!   [`LoadImage`]) and `glb->translate->isBigEndian()`.  Rather than depend on
//!   the (W4) `Architecture`, those are threaded in as a `&mut dyn LoadImage` and
//!   a `bool bigend` — the same data the C++ reads off `glb`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use kuna_base::address::{mostsigbit_set, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{AttributeId, Decoder, ElementId, Encoder, ATTRIB_CONTENT};
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::{int4, uint1, uint8};
use kuna_sleigh::loadimage::LoadImage;

use crate::dtype::Datatype;

// kuna-base `crc_update`: the same 32-bit CRC the C++ `crc32.hh` provides.
use kuna_base::crc32::crc_update;

/// Marshaling attribute `trunc` (C++ `ATTRIB_TRUNC = AttributeId("trunc",69)`).
pub const ATTRIB_TRUNC: AttributeId = AttributeId::new("trunc", 69);

/// Marshaling element `<bytes>` (C++ `ELEM_BYTES = ElementId("bytes",83)`).
pub const ELEM_BYTES: ElementId = ElementId::new("bytes", 83);
/// Marshaling element `<string>` (C++ `ELEM_STRING = ElementId("string",84)`).
pub const ELEM_STRING: ElementId = ElementId::new("string", 84);
/// Marshaling element `<stringmanage>` (C++
/// `ELEM_STRINGMANAGE = ElementId("stringmanage",85)`).
pub const ELEM_STRINGMANAGE: ElementId = ElementId::new("stringmanage", 85);

/// String data (a sequence of bytes) stored by [`StringManager`]
/// (C++ `StringManager::StringData`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringData {
    /// `true` if the string is truncated (C++ `isTruncated`).
    pub is_truncated: bool,
    /// UTF-8 encoded string data (C++ `byteData`, a `vector<uint1>`).
    pub byte_data: Vec<uint1>,
}

/// Storage for decoding and storing strings associated with an address
/// (C++ `StringManager`).
///
/// Looks at data in the loadimage to determine if it represents a "string",
/// decodes it for presentation in the output, and caches the decoded string
/// until it is needed.  Strings are associated with their starting address in
/// memory.  An *internal* string (not in the loadimage) can be registered and is
/// associated with a constant.
#[derive(Debug, Clone, Default)]
pub struct StringManager {
    /// Map from address to string data (C++ `map<Address,StringData> stringMap`).
    ///
    /// A `BTreeMap` to replicate the C++ `map`'s ascending-`Address` iteration
    /// order used by [`StringManager::encode`].
    pub string_map: BTreeMap<Address, StringData>,
    /// Maximum characters in a string before truncating (C++ `maximumChars`).
    pub maximum_chars: int4,
}

impl StringManager {
    /// C++ `StringManager::StringManager(int4 max)`.
    pub fn new(max: int4) -> StringManager {
        StringManager {
            string_map: BTreeMap::new(),
            maximum_chars: max,
        }
    }

    /// C++ `StringManager::clear` — clear out any cached strings.
    pub fn clear(&mut self) {
        self.string_map.clear();
    }

    /// C++ `StringManager::writeUnicode` (stringmanage.cc:36-53).
    ///
    /// Assume the buffer contains a null-terminated unicode-encoded string.
    /// Write the characters out (as UTF-8) into `out`.  Returns `true` if the
    /// byte array contains valid unicode.
    fn write_unicode(
        &self,
        out: &mut Vec<uint1>,
        buffer: &[uint1],
        size: int4,
        charsize: int4,
        bigend: bool,
    ) -> bool {
        let mut i: int4 = 0;
        let mut count: int4 = 0;
        let mut skip: int4 = charsize;
        while i < size {
            let codepoint =
                StringManager::get_codepoint(&buffer[i as usize..], charsize, bigend, &mut skip);
            if codepoint < 0 {
                return false;
            }
            if codepoint == 0 {
                break; // Terminator
            }
            StringManager::write_utf8(out, codepoint);
            i += skip;
            count += 1;
            if count >= self.maximum_chars {
                break;
            }
        }
        true
    }

    /// C++ `StringManager::assignStringData` (stringmanage.cc:66-86).
    ///
    /// Translate and assign raw string data to a [`StringData`].  The data is
    /// translated to UTF-8 and truncated to `maximum_chars`.  The encoding must
    /// be legal unicode (as performed by [`StringManager::check_characters`]).
    fn assign_string_data(
        &self,
        data: &mut StringData,
        buf: &[uint1],
        size: int4,
        charsize: int4,
        num_chars: int4,
        bigend: bool,
    ) {
        if charsize == 1 && num_chars < self.maximum_chars {
            data.byte_data.clear();
            data.byte_data.reserve(size as usize);
            data.byte_data.extend_from_slice(&buf[..size as usize]);
        } else {
            // We need to translate to UTF8 and/or truncate
            let mut s: Vec<uint1> = Vec::new();
            if !self.write_unicode(&mut s, buf, size, charsize, bigend) {
                return;
            }
            let new_size = s.len();
            data.byte_data.clear();
            data.byte_data.reserve(new_size + 1);
            data.byte_data.extend_from_slice(&s[..new_size]);
            data.byte_data.push(0); // Make sure there is a null terminator
        }
        data.is_truncated = num_chars >= self.maximum_chars;
    }

    /// C++ `StringManager::calcInternalHash` (stringmanage.cc:95-105).
    ///
    /// Calculate a 32-bit CRC of the bytes and XOR it into the upper part of the
    /// Address offset.
    fn calc_internal_hash(addr: &Address, buf: &[uint1], size: int4) -> uint8 {
        let mut reg: u32 = 0x7b7c66a9;
        for i in 0..size {
            reg = crc_update(reg, buf[i as usize] as u32);
        }
        let mut res: uint8 = addr.get_offset();
        res ^= (reg as uint8) << 32;
        res
    }

    /// C++ `StringManager::writeUtf8` (stringmanage.cc:124-158).
    ///
    /// Encode the given unicode `codepoint` as UTF-8 (1, 2, 3, or 4 bytes) and
    /// append the bytes to `out`.  `panic`s with the same `LowlevelError` text the
    /// C++ throws for a negative or out-of-range codepoint.
    pub fn write_utf8(out: &mut Vec<uint1>, codepoint: int4) {
        let mut bytes = [0u8; 4];
        let size: int4;

        if codepoint < 0 {
            panic!("Negative unicode codepoint");
        }
        if codepoint < 128 {
            out.push(codepoint as uint1);
            return;
        }
        let bits = mostsigbit_set(codepoint as u64) + 1;
        if bits > 21 {
            panic!("Bad unicode codepoint");
        }
        if bits < 12 {
            // Encode with two bytes
            bytes[0] = 0xc0 ^ ((codepoint >> 6) & 0x1f) as uint1;
            bytes[1] = 0x80 ^ (codepoint & 0x3f) as uint1;
            size = 2;
        } else if bits < 17 {
            bytes[0] = 0xe0 ^ ((codepoint >> 12) & 0xf) as uint1;
            bytes[1] = 0x80 ^ ((codepoint >> 6) & 0x3f) as uint1;
            bytes[2] = 0x80 ^ (codepoint & 0x3f) as uint1;
            size = 3;
        } else {
            bytes[0] = 0xf0 ^ ((codepoint >> 18) & 7) as uint1;
            bytes[1] = 0x80 ^ ((codepoint >> 12) & 0x3f) as uint1;
            bytes[2] = 0x80 ^ ((codepoint >> 6) & 0x3f) as uint1;
            bytes[3] = 0x80 ^ (codepoint & 0x3f) as uint1;
            size = 4;
        }
        out.extend_from_slice(&bytes[..size as usize]);
    }

    /// C++ `StringManager::registerInternalStringData` (stringmanage.cc:185-199).
    ///
    /// Associate string data at a code address (or other location that doesn't
    /// hold string data normally).  Returns a non-zero hash if the bytes form a
    /// legal string (retrievable via [`StringManager::get_string_data_cached`]
    /// keyed on the constant address built from this hash), 0 otherwise.
    ///
    /// `manager` is the [`AddrSpaceManager`] reached in C++ via
    /// `addr.getSpace()->getManager()` (boundary: see module docs).
    pub fn register_internal_string_data(
        &mut self,
        addr: &Address,
        buf: &[uint1],
        size: int4,
        char_type: &Datatype,
        manager: &AddrSpaceManager,
    ) -> uint8 {
        let charsize = char_type.get_size();
        let num_chars = StringManager::check_characters(buf, size, charsize, addr.is_big_endian());
        if num_chars < 0 {
            return 0; // Not a legal encoding
        }
        let hash = StringManager::calc_internal_hash(addr, buf, size);
        let const_addr = manager.get_constant(hash);
        let mut data = StringData::default();
        // assignStringData borrows `self` immutably while mutating `data`; build
        // the StringData detached, then insert it (one `stringMap[constAddr]`
        // slot, matching the C++ which mutates the single map entry in place).
        self.assign_string_data(&mut data, buf, size, charsize, num_chars, addr.is_big_endian());
        self.string_map.insert(const_addr, data);
        hash
    }

    /// C++ `StringManager::isString` (stringmanage.cc:166-172).
    ///
    /// Returns `true` if the data is some kind of complete string.  Uses
    /// [`StringManagerUnicode::get_string_data`] (a `virtual` in C++), so this is
    /// implemented on [`StringManagerUnicode`].
    pub fn is_string_cached(&self, addr: &Address) -> bool {
        match self.string_map.get(addr) {
            Some(d) => !d.byte_data.is_empty(),
            None => false,
        }
    }

    /// Retrieve already-cached string data (for internal/registered strings).
    /// Returns the cached UTF-8 bytes, or an empty slice if nothing is cached.
    pub fn get_string_data_cached(&self, addr: &Address) -> &[uint1] {
        match self.string_map.get(addr) {
            Some(d) => &d.byte_data,
            None => &[],
        }
    }

    /// C++ `StringManager::encode` (stringmanage.cc:203-227).
    ///
    /// Encode the `<stringmanage>` element with `<string>` children.  The hex
    /// content layout is reproduced byte-for-byte: a leading `\n`, two lowercase
    /// hex digits per byte (`setfill('0') setw(2) hex`), a `\n  ` break after
    /// every 20 bytes, and a trailing `\n`.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_STRINGMANAGE);

        for (addr, string_data) in self.string_map.iter() {
            encoder.open_element(&ELEM_STRING);
            addr.encode(encoder)?;
            encoder.open_element(&ELEM_BYTES);
            encoder.write_bool(&ATTRIB_TRUNC, string_data.is_truncated);
            let mut s = String::new();
            s.push('\n');
            for (i, b) in string_data.byte_data.iter().enumerate() {
                let _ = write!(s, "{:02x}", *b as int4);
                if i % 20 == 19 {
                    s.push_str("\n  ");
                }
            }
            s.push('\n');
            encoder.write_string(&ATTRIB_CONTENT, s.as_bytes());
            encoder.close_element(&ELEM_BYTES);
            encoder.close_element(&ELEM_STRING);
        }
        encoder.close_element(&ELEM_STRINGMANAGE);
        Ok(())
    }

    /// C++ `StringManager::decode` (stringmanage.cc:231-271).
    ///
    /// Parse a `<stringmanage>` element with `<string>` children.  The hex-content
    /// scan replicates the C++ `istringstream` loop: skip whitespace, read two
    /// hex chars, decode `c1*16 + c2` until a non-positive char is read.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_STRINGMANAGE)?;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id != ELEM_STRING.get_id() {
                break;
            }
            let addr = Address::decode(decoder)?;
            let mut string_data = StringData::default();
            let sub_id2 = decoder.open_element_id(&ELEM_BYTES)?;
            string_data.is_truncated = decoder.read_bool_id(&ATTRIB_TRUNC)?;
            let content = decoder.read_string_id(&ATTRIB_CONTENT)?;
            decode_hex_bytes(&content, &mut string_data.byte_data);
            decoder.close_element(sub_id2)?;
            decoder.close_element(sub_id)?;
            self.string_map.insert(addr, string_data);
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// C++ `StringManager::hasCharTerminator` (stringmanage.cc:277-291).
    ///
    /// Returns `true` if a string terminator (a full-width zero character) is
    /// found in the first `size` bytes.
    pub fn has_char_terminator(buffer: &[uint1], size: int4, charsize: int4) -> bool {
        let mut i: int4 = 0;
        while i < size {
            let mut is_terminator = true;
            for j in 0..charsize {
                if buffer[(i + j) as usize] != 0 {
                    // Non-zero byte means this character can't be a null terminator
                    is_terminator = false;
                    break;
                }
            }
            if is_terminator {
                return true;
            }
            i += charsize;
        }
        false
    }

    /// C++ `StringManager::readUtf16` (stringmanage.cc:297-312).
    ///
    /// Pull the first two bytes from `buf` and combine them in the requested
    /// endian order.
    pub fn read_utf16(buf: &[uint1], bigend: bool) -> int4 {
        if bigend {
            let mut cp = buf[0] as int4;
            cp <<= 8;
            cp += buf[1] as int4;
            cp
        } else {
            let mut cp = buf[1] as int4;
            cp <<= 8;
            cp += buf[0] as int4;
            cp
        }
    }

    /// C++ `StringManager::checkCharacters` (stringmanage.cc:324-339).
    ///
    /// Make sure `buf` contains a valid bounded set of unicode.  Returns the
    /// number of characters, or -1 if there is an invalid encoding.
    pub fn check_characters(buf: &[uint1], size: int4, charsize: int4, bigend: bool) -> int4 {
        // An empty slice models the C++ null-buffer check here.
        if buf.is_empty() {
            return -1;
        }
        let mut i: int4 = 0;
        let mut count: int4 = 0;
        let mut skip: int4 = charsize;
        while i < size {
            let codepoint =
                StringManager::get_codepoint(&buf[i as usize..], charsize, bigend, &mut skip);
            if codepoint < 0 {
                return -1;
            }
            if codepoint == 0 {
                break;
            }
            count += 1;
            i += skip;
        }
        count
    }

    /// C++ `StringManager::getCodepoint` (stringmanage.cc:347-410).
    ///
    /// Extract the next unicode codepoint from `buf`.  One or more bytes are
    /// consumed; the number consumed is passed back via `skip`.  Returns the
    /// codepoint or -1 if the encoding is invalid.
    pub fn get_codepoint(buf: &[uint1], charsize: int4, bigend: bool, skip: &mut int4) -> int4 {
        let codepoint: int4;
        let mut sk: int4 = 0;
        if charsize == 2 {
            // UTF-16
            let mut cp = StringManager::read_utf16(buf, bigend);
            sk += 2;
            if (0xD800..=0xDBFF).contains(&cp) {
                // high surrogate
                let trail = StringManager::read_utf16(&buf[2..], bigend);
                sk += 2;
                if !(0xDC00..=0xDFFF).contains(&trail) {
                    return -1; // Bad trail
                }
                cp = (cp << 10) + trail + (0x10000 - (0xD800 << 10) - 0xDC00);
            } else if (0xDC00..=0xDFFF).contains(&cp) {
                return -1; // trail before high
            }
            codepoint = cp;
        } else if charsize == 1 {
            // UTF-8
            let val = buf[0] as int4;
            if (val & 0x80) == 0 {
                codepoint = val;
                sk = 1;
            } else if (val & 0xe0) == 0xc0 {
                let val2 = buf[1] as int4;
                sk = 2;
                if (val2 & 0xc0) != 0x80 {
                    return -1; // Not a valid UTF8-encoding
                }
                codepoint = ((val & 0x1f) << 6) | (val2 & 0x3f);
            } else if (val & 0xf0) == 0xe0 {
                let val2 = buf[1] as int4;
                let val3 = buf[2] as int4;
                sk = 3;
                if ((val2 & 0xc0) != 0x80) || ((val3 & 0xc0) != 0x80) {
                    return -1; // invalid encoding
                }
                codepoint = ((val & 0xf) << 12) | ((val2 & 0x3f) << 6) | (val3 & 0x3f);
            } else if (val & 0xf8) == 0xf0 {
                let val2 = buf[1] as int4;
                let val3 = buf[2] as int4;
                let val4 = buf[3] as int4;
                sk = 4;
                if ((val2 & 0xc0) != 0x80) || ((val3 & 0xc0) != 0x80) || ((val4 & 0xc0) != 0x80) {
                    return -1; // invalid encoding
                }
                codepoint =
                    ((val & 7) << 18) | ((val2 & 0x3f) << 12) | ((val3 & 0x3f) << 6) | (val4 & 0x3f);
            } else {
                return -1;
            }
        } else if charsize == 4 {
            // UTF-32
            sk = 4;
            if bigend {
                codepoint = ((buf[0] as int4) << 24)
                    + ((buf[1] as int4) << 16)
                    + ((buf[2] as int4) << 8)
                    + buf[3] as int4;
            } else {
                codepoint = ((buf[3] as int4) << 24)
                    + ((buf[2] as int4) << 16)
                    + ((buf[1] as int4) << 8)
                    + buf[0] as int4;
            }
        } else {
            return -1;
        }
        if codepoint >= 0xd800 {
            if codepoint > 0x10ffff {
                // Bigger than maximum codepoint
                return -1;
            }
            if codepoint <= 0xdfff {
                return -1; // Reserved for surrogates, invalid codepoints
            }
        }
        *skip = sk;
        codepoint
    }
}

/// Decode the hex byte-content of a `<bytes>` element into `out`
/// (the body of C++ `StringManager::decode`'s `istringstream` loop,
/// stringmanage.cc:242-266).
fn decode_hex_bytes(content: &[u8], out: &mut Vec<uint1>) {
    let mut pos = skip_ws(content, 0);
    let mut c1 = get_char(content, &mut pos);
    let mut c2 = get_char(content, &mut pos);
    while c1 > 0 && c2 > 0 {
        let mut d1 = c1;
        let mut d2 = c2;
        if d1 <= b'9' as i32 {
            d1 -= b'0' as i32;
        } else if d1 <= b'F' as i32 {
            d1 = d1 + 10 - b'A' as i32;
        } else {
            d1 = d1 + 10 - b'a' as i32;
        }
        if d2 <= b'9' as i32 {
            d2 -= b'0' as i32;
        } else if d2 <= b'F' as i32 {
            d2 = d2 + 10 - b'A' as i32;
        } else {
            d2 = d2 + 10 - b'a' as i32;
        }
        let val = d1 * 16 + d2;
        out.push(val as uint1);
        pos = skip_ws(content, pos);
        c1 = get_char(content, &mut pos);
        c2 = get_char(content, &mut pos);
    }
}

/// `is >> ws` — advance past ASCII whitespace.
fn skip_ws(content: &[u8], mut pos: usize) -> usize {
    while pos < content.len() && content[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// `c = is.get()` — read one char, returning the C++ EOF sentinel (-1) at the
/// end.  The C++ `char` is signed; the loop guard `c > 0` stops on EOF (and on a
/// NUL byte), which this matches by returning -1 past the end.
fn get_char(content: &[u8], pos: &mut usize) -> i32 {
    if *pos >= content.len() {
        -1
    } else {
        let c = content[*pos] as i8 as i32; // signed char, as in C++
        *pos += 1;
        c
    }
}

/// An implementation of [`StringManager`] that understands terminated unicode
/// strings (C++ `StringManagerUnicode`).
///
/// Understands UTF-8, UTF-16, and UTF-32 encodings.  It reports a string if it
/// sees a valid encoding that is null-terminated.
#[derive(Debug, Default)]
pub struct StringManagerUnicode {
    /// The shared cache + limits (C++ base `StringManager`).
    pub base: StringManager,
    /// Temporary buffer for pulling in loadimage bytes (C++ `testBuffer`, an
    /// array of `maximumChars` bytes).
    test_buffer: Vec<uint1>,
}

impl StringManagerUnicode {
    /// C++ `StringManagerUnicode::StringManagerUnicode(Architecture *g,int4 max)`.
    ///
    /// `glb`/`loader`/`translate` are threaded into [`Self::get_string_data`]
    /// rather than stored (boundary: see module docs).
    pub fn new(max: int4) -> StringManagerUnicode {
        StringManagerUnicode {
            base: StringManager::new(max),
            test_buffer: vec![0u8; max.max(0) as usize],
        }
    }

    /// C++ `StringManagerUnicode::getStringData` (stringmanage.cc:427-475).
    ///
    /// Retrieve string data at `addr` as a UTF-8 byte array.  Pulls bytes from
    /// `loader` 32 at a time until a terminator is found (or `maximumChars`
    /// bytes are read), validates the encoding, re-encodes to UTF-8, caches the
    /// result, and returns it.  An empty result means "not a string here".
    ///
    /// `is_trunc` passes back whether the cached string was truncated.  Endianness
    /// is read off `addr` (`addr.isBigEndian()`), exactly as the C++ does.
    pub fn get_string_data(
        &mut self,
        addr: &Address,
        char_type: &Datatype,
        loader: &mut dyn LoadImage,
        is_trunc: &mut bool,
    ) -> &[uint1] {
        if self.base.string_map.contains_key(addr) {
            let d = &self.base.string_map[addr];
            *is_trunc = d.is_truncated;
            return &self.base.string_map[addr].byte_data;
        }

        // Allocate the map entry (initially empty).
        {
            let entry = self.base.string_map.entry(addr.clone()).or_default();
            entry.is_truncated = false;
        }
        *is_trunc = false;

        if char_type.is_opaque_string() {
            // Cannot currently test for an opaque encoding — return empty buffer.
            return &self.base.string_map[addr].byte_data;
        }

        let mut cur_buffer_size: int4 = 0;
        let charsize = char_type.get_size();

        // A non-closure transcription of the C++ do/while: the `'fill` block
        // mirrors the `try` body, returning the load_fill Result so the
        // DataUnavail catch is handled below.
        let result: KunaResult<()> = 'fill: loop {
            let mut amount: int4 = 32; // Grab 32 bytes of image at a time
            let mut new_buffer_size: int4 = cur_buffer_size + amount;
            // C++ compares a uint4 newBufferSize against int4 maximumChars; both
            // are small non-negative here, so the comparison agrees.
            if new_buffer_size > self.base.maximum_chars {
                new_buffer_size = self.base.maximum_chars;
                amount = new_buffer_size - cur_buffer_size;
                if amount == 0 {
                    // Could not find terminator — C++ `return stringData.byteData`
                    // exits with the still-empty buffer BEFORE
                    // checkCharacters/assignStringData run. Return the empty cached
                    // buffer directly; do NOT fall through to the
                    // check_characters/assign_string_data path below.
                    return &self.base.string_map[addr].byte_data;
                }
            }
            let fill_addr = addr + cur_buffer_size as i64;
            let start = cur_buffer_size as usize;
            let end = start + amount as usize;
            if let Err(e) = loader.load_fill(&mut self.test_buffer[start..end], &fill_addr) {
                break 'fill Err(e);
            }
            let found_terminator =
                StringManager::has_char_terminator(&self.test_buffer[start..], amount, charsize);
            cur_buffer_size = new_buffer_size;
            if found_terminator {
                break 'fill Ok(());
            }
        };

        // Whether we stopped on a terminator, ran out of room, or hit
        // DataUnavail, the next check decides whether anything is emitted.  On
        // DataUnavail the C++ returns the empty buffer immediately.
        match result {
            Ok(()) => {}
            Err(KunaError::DataUnavail { .. }) => {
                return &self.base.string_map[addr].byte_data;
            }
            // No other error is expected from load_fill; propagate by treating
            // it as "no string" (empty buffer), matching the DataUnavail path.
            Err(_) => {
                return &self.base.string_map[addr].byte_data;
            }
        }

        let num_chars = StringManager::check_characters(
            &self.test_buffer,
            cur_buffer_size,
            charsize,
            addr.is_big_endian(),
        );
        if num_chars < 0 {
            // Return the empty buffer (invalid encoding)
            return &self.base.string_map[addr].byte_data;
        }
        let mut data = std::mem::take(self.base.string_map.get_mut(addr).unwrap());
        let buf = self.test_buffer.clone();
        self.base.assign_string_data(
            &mut data,
            &buf,
            cur_buffer_size,
            charsize,
            num_chars,
            addr.is_big_endian(),
        );
        *is_trunc = data.is_truncated;
        self.base.string_map.insert(addr.clone(), data);
        &self.base.string_map[addr].byte_data
    }
}

#[cfg(test)]
#[path = "stringmanage/tests.rs"]
mod tests;
