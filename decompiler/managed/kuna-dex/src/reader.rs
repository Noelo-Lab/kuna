//! Bounds-checked little-endian reads over untrusted DEX bytes: every read
//! returns an error instead of panicking past the end.

use crate::{err, Result};

#[derive(Clone)]
pub(crate) struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn at(data: &'a [u8], pos: usize) -> Result<Reader<'a>> {
        if pos > data.len() {
            return Err(err(format!("offset {pos:#x} past the end of the file")));
        }
        Ok(Reader { data, pos })
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        if n > self.remaining() {
            return Err(err(format!(
                "truncated: {n} bytes wanted at {:#x}, {} left",
                self.pos,
                self.remaining()
            )));
        }
        let out = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }

    pub fn u16(&mut self) -> Result<u16> {
        let b = self.bytes(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub fn u32(&mut self) -> Result<u32> {
        let b = self.bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn uleb(&mut self) -> Result<u32> {
        let mut value = 0u32;
        for i in 0..5 {
            let b = self.u8()?;
            if i == 4 && b > 0x0f {
                return Err(err(format!("bad uleb128 at {:#x}", self.pos - 1)));
            }
            value |= u32::from(b & 0x7f) << (7 * i);
            if b & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(err(format!("bad uleb128 at {:#x}", self.pos)))
    }

    /// `uleb128p1`: the value minus one, so `NO_INDEX` (`u32::MAX`) encodes as 0.
    pub fn uleb_p1(&mut self) -> Result<u32> {
        Ok(self.uleb()?.wrapping_sub(1))
    }

    pub fn sleb(&mut self) -> Result<i32> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let b = self.u8()?;
            if shift == 28 && b & 0x80 != 0 {
                return Err(err(format!("bad sleb128 at {:#x}", self.pos - 1)));
            }
            value |= u32::from(b & 0x7f) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                if shift < 32 && b & 0x40 != 0 {
                    value |= u32::MAX << shift;
                }
                return Ok(value as i32);
            }
        }
    }
}

/// A `string_data_item`'s MUTF-8 bytes (without the terminating NUL).
pub(crate) fn mutf8_at(data: &[u8], off: usize) -> Result<&[u8]> {
    let mut r = Reader::at(data, off)?;
    r.uleb()?;
    let start = r.pos();
    let len = data[start..]
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| err(format!("unterminated string at {off:#x}")))?;
    Ok(&data[start..start + len])
}

/// Decodes MUTF-8 (CESU-8 surrogates, `C0 80` for NUL); invalid bytes become U+FFFD.
pub fn mutf8_to_string(bytes: &[u8]) -> String {
    if bytes.is_ascii() {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    let mut units: Vec<u16> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let cont = |j: usize| bytes.get(j).copied().filter(|c| c & 0xc0 == 0x80);
        if b < 0x80 {
            units.push(u16::from(b));
            i += 1;
        } else if b & 0xe0 == 0xc0 {
            match cont(i + 1) {
                Some(c) => {
                    units.push((u16::from(b & 0x1f) << 6) | u16::from(c & 0x3f));
                    i += 2;
                }
                None => {
                    units.push(0xfffd);
                    i += 1;
                }
            }
        } else if b & 0xf0 == 0xe0 {
            match (cont(i + 1), cont(i + 2)) {
                (Some(c), Some(d)) => {
                    units.push(
                        (u16::from(b & 0x0f) << 12)
                            | (u16::from(c & 0x3f) << 6)
                            | u16::from(d & 0x3f),
                    );
                    i += 3;
                }
                _ => {
                    units.push(0xfffd);
                    i += 1;
                }
            }
        } else {
            units.push(0xfffd);
            i += 1;
        }
    }
    String::from_utf16_lossy(&units)
}
