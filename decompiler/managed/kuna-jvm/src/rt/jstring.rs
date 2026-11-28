//! Java string semantics over Rust `str`.
//!
//! A Java `String` is a UTF-16 sequence and may hold unpaired surrogates, which
//! a Rust `String` cannot. Class-file text is decoded so that an unpaired
//! surrogate `0xD800..=0xDFFF` becomes the private-use scalar
//! `SURROGATE_BASE + unit`; [`JavaStr::utf16`] maps it back, so every UTF-16
//! view of the string is exactly what the JVM sees.

/// First scalar of the private-use block that stands in for unpaired surrogates.
pub const SURROGATE_BASE: u32 = 0xF_0000;

fn unit_to_scalar(unit: u16) -> char {
    char::from_u32(SURROGATE_BASE + unit as u32).expect("plane-15 private use scalar")
}

fn push_units(out: &mut String, units: &[u16]) {
    let mut i = 0;
    while i < units.len() {
        let u = units[i];
        if (0xD800..0xDC00).contains(&u)
            && i + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[i + 1])
        {
            let c = 0x1_0000 + (((u as u32) - 0xD800) << 10) + (units[i + 1] as u32 - 0xDC00);
            out.push(char::from_u32(c).expect("valid surrogate pair"));
            i += 2;
        } else if (0xD800..0xE000).contains(&u) {
            out.push(unit_to_scalar(u));
            i += 1;
        } else {
            out.push(char::from_u32(u as u32).expect("BMP scalar"));
            i += 1;
        }
    }
}

/// Build a string from UTF-16 units, keeping unpaired surrogates recoverable.
pub fn from_utf16(units: &[u16]) -> String {
    let mut out = String::with_capacity(units.len());
    push_units(&mut out, units);
    out
}

/// Decode a class-file `CONSTANT_Utf8` payload (modified UTF-8).
pub fn decode_modified_utf8(bytes: &[u8]) -> String {
    if bytes.is_ascii() {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    let mut units: Vec<u16> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i] as u32;
        if b < 0x80 {
            units.push(b as u16);
            i += 1;
        } else if b >> 5 == 0b110 && i + 1 < bytes.len() {
            units.push((((b & 0x1F) << 6) | (bytes[i + 1] as u32 & 0x3F)) as u16);
            i += 2;
        } else if b >> 4 == 0b1110 && i + 2 < bytes.len() {
            units.push(
                (((b & 0x0F) << 12)
                    | ((bytes[i + 1] as u32 & 0x3F) << 6)
                    | (bytes[i + 2] as u32 & 0x3F)) as u16,
            );
            i += 3;
        } else {
            units.push(0xFFFD);
            i += 1;
        }
    }
    from_utf16(&units)
}

/// `String.hashCode()`.
pub fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    if s.is_ascii() {
        for b in s.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as i32);
        }
        return h;
    }
    for u in s.utf16() {
        h = h.wrapping_mul(31).wrapping_add(u as i32);
    }
    h
}

/// Java `String` methods, indexed in UTF-16 units like the originals.
///
/// Byte-indexed `str` methods are equivalent only for ASCII text; a parser that
/// slices at ASCII delimiters (descriptors, signatures) may use them directly.
pub trait JavaStr {
    /// The exact UTF-16 units the JVM holds.
    fn utf16(&self) -> Vec<u16>;
    /// `length()`.
    fn jlen(&self) -> usize;
    /// `charAt(i)`.
    fn jchar_at(&self, index: usize) -> u16;
    /// `substring(begin, end)`.
    fn jsubstring(&self, begin: usize, end: usize) -> String;
    /// `substring(begin)`.
    fn jsubstring_from(&self, begin: usize) -> String;
}

impl JavaStr for str {
    fn utf16(&self) -> Vec<u16> {
        let mut out = Vec::with_capacity(self.len());
        for c in self.chars() {
            let v = c as u32;
            if (SURROGATE_BASE + 0xD800..SURROGATE_BASE + 0xE000).contains(&v) {
                out.push((v - SURROGATE_BASE) as u16);
            } else {
                let mut buf = [0u16; 2];
                out.extend_from_slice(c.encode_utf16(&mut buf));
            }
        }
        out
    }

    fn jlen(&self) -> usize {
        if self.is_ascii() {
            self.len()
        } else {
            self.utf16().len()
        }
    }

    fn jchar_at(&self, index: usize) -> u16 {
        if self.is_ascii() {
            self.as_bytes()[index] as u16
        } else {
            self.utf16()[index]
        }
    }

    fn jsubstring(&self, begin: usize, end: usize) -> String {
        if self.is_ascii() {
            self[begin..end].to_string()
        } else {
            from_utf16(&self.utf16()[begin..end])
        }
    }

    fn jsubstring_from(&self, begin: usize) -> String {
        if self.is_ascii() {
            self[begin..].to_string()
        } else {
            from_utf16(&self.utf16()[begin..])
        }
    }
}
