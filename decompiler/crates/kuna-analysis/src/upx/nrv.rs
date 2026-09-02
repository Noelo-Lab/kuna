//! The UCL NRV2B / NRV2D / NRV2E decompressors -- the LZ back-ends UPX uses for
//! every compression method below LZMA.
//!
//! Ported from UCL 1.03 (`src/n2b_d.c`, `n2d_d.c`, `n2e_d.c` and the `getbit_*`
//! macros of `src/getbit.h`; GPL-2.0-or-later, Markus F.X.J. Oberhumer). The
//! three variants share one loop and differ only in how the match offset and
//! match length are coded, so they are expressed here as one decoder
//! parameterized by [`Variant`] rather than three near-copies.
//!
//! Every decode is bounds-checked and returns [`NrvError`] instead of trusting
//! the block header: the input is attacker-controlled by construction. The
//! success contract is deliberately strict -- the output must reach *exactly*
//! `u_len` bytes and the input must be consumed *exactly* -- because that pair
//! is what tells a caller a block really decoded, and a UPX unpacker that
//! silently emits a half-decoded image is worse than one that refuses.

use std::fmt;

/// The bit-buffer layout: UPX's method id encodes both the LZ variant and how
/// the bit stream is packed (`_LE32` reads 32-bit little-endian words MSB-first,
/// `_LE16` 16-bit words, `_8` one byte at a time).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOrder {
    Byte,
    Le16,
    Le32,
}

/// Which of the three NRV codings the stream uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Nrv2b,
    Nrv2d,
    Nrv2e,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NrvError {
    /// The decoder asked for a byte or bit past the end of the block.
    InputOverrun,
    /// The block produced more bytes than its header promised.
    OutputOverrun,
    /// A match referenced bytes before the start of the output.
    LookbehindOverrun,
    /// The stream ended before `u_len` bytes were produced.
    Truncated { got: usize, want: usize },
    /// The stream ended with input left over (a wrong method, usually).
    InputNotConsumed { got: usize, want: usize },
}

impl fmt::Display for NrvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NrvError::InputOverrun => write!(f, "compressed block ended mid-symbol"),
            NrvError::OutputOverrun => write!(f, "block expanded past its declared size"),
            NrvError::LookbehindOverrun => write!(f, "match offset points before the block"),
            NrvError::Truncated { got, want } => {
                write!(f, "block decoded to {got} bytes, header declared {want}")
            }
            NrvError::InputNotConsumed { got, want } => {
                write!(f, "block left {} of {want} compressed bytes unread", want.saturating_sub(*got))
            }
        }
    }
}

/// Decompress one NRV block. `u_len` is the size its `b_info` header declares;
/// a stream that does not hit it exactly is an error, never a short read.
pub fn decompress(
    variant: Variant,
    order: BitOrder,
    src: &[u8],
    u_len: usize,
) -> Result<Vec<u8>, NrvError> {
    let mut bits = BitReader::new(src, order);
    let mut dst: Vec<u8> = Vec::with_capacity(u_len);
    let mut last_m_off: usize = 1;

    loop {
        while bits.get()? != 0 {
            if dst.len() >= u_len {
                return Err(NrvError::OutputOverrun);
            }
            dst.push(bits.byte()?);
        }

        // Match offset. NRV2B codes it as a plain Elias-gamma-ish run; NRV2D
        // and NRV2E interleave a second bit per level.
        let mut m_off: u64 = 1;
        loop {
            m_off = m_off * 2 + u64::from(bits.get()?);
            if m_off > 0x00ff_ffff + 3 {
                return Err(NrvError::LookbehindOverrun);
            }
            if bits.get()? != 0 {
                break;
            }
            if variant != Variant::Nrv2b {
                m_off = (m_off - 1) * 2 + u64::from(bits.get()?);
            }
        }

        let mut m_len: u64;
        if m_off == 2 {
            m_off = last_m_off as u64;
            m_len = match variant {
                Variant::Nrv2b => 0,
                _ => u64::from(bits.get()?),
            };
        } else {
            m_off = (m_off - 3) * 256 + u64::from(bits.byte()?);
            if m_off == 0xffff_ffff {
                break;
            }
            match variant {
                Variant::Nrv2b => m_len = 0,
                _ => {
                    m_len = (m_off ^ 0xffff_ffff) & 1;
                    m_off >>= 1;
                }
            }
            m_off += 1;
            last_m_off = m_off as usize;
        }

        // Match length, and the variant-specific "long offsets cost one more
        // byte, so spend it on length" bonus (UCL's M2_MAX_OFFSET).
        let bonus = match variant {
            Variant::Nrv2b => u64::from(m_off > 0xd00),
            _ => u64::from(m_off > 0x500),
        };
        if variant == Variant::Nrv2e {
            if m_len != 0 {
                m_len = 1 + u64::from(bits.get()?);
            } else if bits.get()? != 0 {
                m_len = 3 + u64::from(bits.get()?);
            } else {
                m_len = 1;
                loop {
                    m_len = m_len * 2 + u64::from(bits.get()?);
                    if m_len >= u_len as u64 {
                        return Err(NrvError::OutputOverrun);
                    }
                    if bits.get()? != 0 {
                        break;
                    }
                }
                m_len += 3;
            }
        } else {
            if variant == Variant::Nrv2b {
                m_len = u64::from(bits.get()?);
            }
            m_len = m_len * 2 + u64::from(bits.get()?);
            if m_len == 0 {
                m_len = 1;
                loop {
                    m_len = m_len * 2 + u64::from(bits.get()?);
                    if m_len >= u_len as u64 {
                        return Err(NrvError::OutputOverrun);
                    }
                    if bits.get()? != 0 {
                        break;
                    }
                }
                m_len += 2;
            }
        }
        m_len += bonus;

        let m_off = m_off as usize;
        let copy = m_len as usize + 1;
        if m_off > dst.len() {
            return Err(NrvError::LookbehindOverrun);
        }
        if dst.len() + copy > u_len {
            return Err(NrvError::OutputOverrun);
        }
        let mut pos = dst.len() - m_off;
        for _ in 0..copy {
            let b = dst[pos];
            dst.push(b);
            pos += 1;
        }
    }

    if dst.len() != u_len {
        return Err(NrvError::Truncated { got: dst.len(), want: u_len });
    }
    if bits.consumed() != src.len() {
        return Err(NrvError::InputNotConsumed { got: bits.consumed(), want: src.len() });
    }
    Ok(dst)
}

/// The `getbit_8` / `getbit_le16` / `getbit_le32` bit buffers of UCL's
/// `getbit.h`, kept bit-exact (including the C `unsigned` wraparound the LE16
/// refill relies on) because the bit order is what the whole format hangs on.
struct BitReader<'a> {
    src: &'a [u8],
    ilen: usize,
    bb: u32,
    bc: u32,
    order: BitOrder,
}

impl<'a> BitReader<'a> {
    fn new(src: &'a [u8], order: BitOrder) -> Self {
        BitReader { src, ilen: 0, bb: 0, bc: 0, order }
    }

    fn consumed(&self) -> usize {
        self.ilen
    }

    fn byte(&mut self) -> Result<u8, NrvError> {
        let b = *self.src.get(self.ilen).ok_or(NrvError::InputOverrun)?;
        self.ilen += 1;
        Ok(b)
    }

    fn get(&mut self) -> Result<u32, NrvError> {
        match self.order {
            BitOrder::Byte => {
                if self.bb & 0x7f != 0 {
                    self.bb = self.bb.wrapping_mul(2);
                } else {
                    self.bb = u32::from(self.byte()?) * 2 + 1;
                }
                Ok((self.bb >> 8) & 1)
            }
            BitOrder::Le16 => {
                self.bb = self.bb.wrapping_mul(2);
                if self.bb & 0xffff != 0 {
                    Ok((self.bb >> 16) & 1)
                } else {
                    let lo = u32::from(self.byte()?);
                    let hi = u32::from(self.byte()?);
                    self.bb = (lo + hi * 0x100).wrapping_mul(2) + 1;
                    Ok((self.bb >> 16) & 1)
                }
            }
            BitOrder::Le32 => {
                if self.bc > 0 {
                    self.bc -= 1;
                    Ok((self.bb >> self.bc) & 1)
                } else {
                    if self.ilen + 4 > self.src.len() {
                        return Err(NrvError::InputOverrun);
                    }
                    let w = &self.src[self.ilen..self.ilen + 4];
                    self.bb = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
                    self.ilen += 4;
                    self.bc = 31;
                    Ok((self.bb >> 31) & 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vendored fixture's first block: 226 compressed bytes that decode to
    /// the original `Elf64_Ehdr` + 13 `Elf64_Phdr`s. Its opening bit word
    /// `0xff21fbf6` spends eight 1-bits on literals (`\x7fELF\x02\x01\x01\x00`)
    /// and then a length-8 match at offset 1, which lays down `e_ident`'s zero
    /// padding -- the shortest end-to-end proof that the bit order, the offset
    /// coding and the length coding are all right.
    #[test]
    fn nrv2b_le32_decodes_the_fixtures_elf_header_block() {
        let image = std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/upx_packed_x86_64"),
        )
        .expect("fixture");
        let out = decompress(Variant::Nrv2b, BitOrder::Le32, &image[0x10c..0x10c + 226], 792)
            .expect("first block decodes");
        assert_eq!(out.len(), 792);
        assert_eq!(&out[..8], b"\x7fELF\x02\x01\x01\x00");
        assert_eq!(&out[8..16], &[0u8; 8]);
        // e_phnum == 13, and 64 + 13*56 == 792 -- the block is exactly the
        // header table it claims to be.
        assert_eq!(u16::from_le_bytes([out[56], out[57]]), 13);
    }

    #[test]
    fn a_short_block_is_an_error_not_a_short_read() {
        let src = [0xf6, 0xfb, 0x21, 0xff, 0x7f, 0x45];
        assert_eq!(
            decompress(Variant::Nrv2b, BitOrder::Le32, &src, 16),
            Err(NrvError::InputOverrun)
        );
    }

    /// A block whose declared size is smaller than what the stream produces is
    /// an error: silently truncating would be a corrupt image.
    #[test]
    fn a_declared_length_that_is_too_small_is_rejected() {
        let src = [0xf6, 0xfb, 0x21, 0xff, 0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00];
        assert!(matches!(
            decompress(Variant::Nrv2b, BitOrder::Le32, &src, 4),
            Err(NrvError::OutputOverrun)
        ));
    }
}
