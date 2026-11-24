//! Wire primitives for indexed IEEE-695 and TIOF modules.
//!
//! These primitives decode the indexed envelope and record fields. They do not
//! select an ISA, map a section, or evaluate and apply a relocation.

use kuna_base::error::{KunaError, KunaResult};

/// A section, symbol, or other module-local record index.
pub type Id = u32;

/// Add the file offset to a format diagnostic.
pub fn err(offset: usize, detail: impl std::fmt::Display) -> KunaError {
    KunaError::Parse { explain: format!("IEEE-695 at file offset {offset:#x}: {detail}") }
}

/// A bounded record cursor and its original file coordinate.
#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
    /// Unconsumed bytes, within this part's bounds.
    pub rem: &'a [u8],
    /// File offset of the first unconsumed byte.
    pub offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn peek(&self) -> Option<u8> {
        self.rem.first().copied()
    }

    pub fn take(&mut self, n: usize) -> KunaResult<&'a [u8]> {
        let (taken, rem) = self.rem.split_at_checked(n)
            .ok_or_else(|| err(self.offset, "truncated record"))?;
        self.rem = rem;
        self.offset += n;
        Ok(taken)
    }

    pub fn byte(&mut self) -> KunaResult<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn expect(&mut self, expected: &[u8]) -> KunaResult<()> {
        let at = self.offset;
        if self.take(expected.len())? != expected {
            return Err(err(at, format_args!("expected record bytes {expected:02x?}")));
        }
        Ok(())
    }

    /// Read an immediate or a length-prefixed unsigned field.
    pub fn num(&mut self) -> KunaResult<u64> {
        let at = self.offset;
        match self.byte()? {
            n @ 0..=0x7f => Ok(n.into()),
            tag @ 0x81..=0x88 => Ok(self.take(usize::from(tag - 0x80))?.iter()
                .fold(0, |n, &byte| (n << 8) | u64::from(byte))),
            tag => Err(err(at, format_args!("expected number, got {tag:#04x}"))),
        }
    }

    pub fn id(&mut self) -> KunaResult<Id> {
        let at = self.offset;
        self.num()?.try_into().map_err(|_| err(at, "record index overflow"))
    }

    /// Read a field describing a byte count or file offset.
    pub fn read_length(&mut self) -> KunaResult<usize> {
        let at = self.offset;
        self.num()?.try_into().map_err(|_| err(at, "record length overflow"))
    }

    pub fn name(&mut self) -> KunaResult<&'a [u8]> {
        let n = self.read_length()?;
        self.take(n)
    }

    pub fn end(self) -> KunaResult<()> {
        if self.rem.is_empty() { Ok(()) } else { Err(err(self.offset, "unexpected trailing record")) }
    }
}

/// Address descriptor and bounded parts of an indexed binary module.
#[derive(Debug)]
pub struct Module<'a> {
    pub processor: &'a [u8],
    /// Bits per minimum addressable unit (MAU).
    pub bits_per_mau: u64,
    pub maus_per_address: Option<u64>,
    /// Optional raw byte-order tag from the address descriptor.
    pub order: Option<u8>,
    /// W0–W7, with an empty cursor for an absent part.
    pub parts: [Cursor<'a>; 8],
}

/// Open a binary module with ordered W0–W7 offsets and an ME trailer.
/// This validates the envelope, not the records inside the parts.
pub fn module(bytes: &[u8]) -> KunaResult<Module<'_>> {
    let mut header = Cursor { rem: bytes, offset: 0 };
    header.expect(&[0xe0])?;
    let processor = header.name()?;
    if header.peek() != Some(0xec) { header.name()?; }
    header.expect(&[0xec])?;
    let bits_per_mau = header.num()?;
    let maus_per_address = match header.peek() {
        Some(0x80) => { header.byte()?; None }
        Some(0x00..=0x88) => Some(header.num()?),
        _ => None,
    };
    let order = match header.peek() {
        Some(0xcc | 0xcd) => Some(header.byte()?),
        _ => None,
    };
    let mut starts = [0; 8];
    for (i, start) in starts.iter_mut().enumerate() {
        header.expect(&[0xe2, 0xd7, i as u8])?;
        *start = header.read_length()?;
    }
    if starts[0] != header.offset || starts[7] == 0 {
        return Err(err(header.offset, "invalid W-part table"));
    }
    let mut end = bytes.len();
    let mut parts = [Cursor { rem: &[], offset: 0 }; 8];
    for (i, &start) in starts.iter().enumerate().rev() {
        if start == 0 { continue; }
        if start >= end || start < header.offset {
            return Err(err(header.offset, format_args!("invalid W{i} part offset {start:#x}")));
        }
        parts[i] = Cursor { rem: &bytes[start..end], offset: start };
        end = start;
    }
    let mut trailer = parts[7];
    trailer.expect(&[0xe1])?;
    trailer.end()?;
    Ok(Module { processor, bits_per_mau, maus_per_address, order, parts })
}

/// A wire token from a relocation expression, before resolving its symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Atom {
    Number(i64),
    Letter(u8),
    Function(u8),
}

/// The punctuation separating commands and embedded expressions is consumed
/// by the caller; the length-prefixed integers can themselves contain it.
pub fn expression_atom(input: &mut Cursor<'_>, parenthesized: bool) -> KunaResult<Option<Atom>> {
    let at = input.offset;
    Ok(Some(match input.peek() {
        Some(0x00..=0x88) => Atom::Number(input.num()? as i64),
        Some(0xc1..=0xda) => Atom::Letter(input.byte()?),
        Some(0xbf) if parenthesized => return Ok(None),
        Some(0x90..=0xbe) => Atom::Function(input.byte()?),
        Some(0xe0..=0xff) | None if !parenthesized => return Ok(None),
        Some(n) => return Err(err(at, format_args!("unsupported expression token {n:#04x}"))),
        None => return Err(err(at, "truncated parenthesized expression")),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image() -> Vec<u8> {
        let mut bytes = b"\xe0\x06m68000\x05testo\xec\x08\x01\xcd".to_vec();
        let w0 = bytes.len() as u8 + 32;
        for (part, pos) in [w0, 0, 0, 0, 0, w0 + 1, 0, w0 + 4].into_iter().enumerate() {
            bytes.extend([0xe2, 0xd7, part as u8, pos]);
        }
        bytes.extend([0x00, 0x82, 0xbf, 0xe8, 0xe1]);
        bytes
    }

    #[test]
    fn opens_an_indexed_module_and_bounds_its_parts() {
        let bytes = image();
        let module = module(&bytes).unwrap();
        assert_eq!(module.processor, b"m68000");
        assert_eq!((module.bits_per_mau, module.maus_per_address, module.order), (8, Some(1), Some(0xcd)));
        assert!(module.parts[4].rem.is_empty());
        let mut data = module.parts[5];
        assert_eq!(expression_atom(&mut data, false).unwrap(), Some(Atom::Number(0xbfe8)));
        assert!(data.byte().is_err()); // must not read ME from W7
    }

    #[test]
    fn rejects_overlapping_parts_and_missing_module_end() {
        let mut bytes = image();
        let part_offset = |part: usize| 18 + 4 * part + 3;
        bytes[part_offset(2)] = bytes[part_offset(5)];
        assert!(module(&bytes).unwrap_err().explain().contains("invalid W2 part offset"));
        let mut bytes = image();
        bytes.pop();
        assert!(module(&bytes).unwrap_err().explain().contains("invalid W7 part offset"));
    }
}
