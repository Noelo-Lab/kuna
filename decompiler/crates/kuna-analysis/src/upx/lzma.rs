//! The LZMA1 decompressor -- UPX's `M_LZMA` (method 14) back-end, the one every
//! `--lzma` / `--best` image is packed with.
//!
//! UPX does not emit a `.lzma` container: a block is a *raw* LZMA1 stream with
//! no 5-byte properties/size preamble and no end-of-stream marker, prefixed by
//! two UPX bytes that carry the coder parameters. `pb` is the low three bits of
//! the first byte; `lc` and `lp` are the low and high nibbles of the second
//! (`src/compress_lzma.cpp`, and the same reading in RetDec's
//! `decompressor_lzma.cpp`). The uncompressed length comes from the block's
//! `b_info` header, which is why nothing here needs an end marker -- though one
//! is still accepted, because the encoder is free to write it.
//!
//! The decoder itself is the reference LZMA algorithm (Igor Pavlov's LZMA SDK,
//! `C/LzmaDec.c` and the older `LzmaDecode.c` UPX actually links; public
//! domain), written out rather than pulled in as a dependency for the same
//! reason [`super::nrv`] is: the input is a hostile file, so every array index,
//! match distance and output write is bounds-checked and returns [`LzmaError`]
//! instead of panicking or emitting a partial image.
//!
//! An analyst who found a stream this decoder can read but no `b_info` to go
//! with it -- a stripped `PackHeader`, a private packer, a blob addressed by
//! hand -- has no `u_len`, so [`decompress_exhaustive`] runs the same decoder to
//! the end of the *input* instead, capping the output rather than the length it
//! was told to expect. `kuna unpack --raw-lzma` is that caller.

use std::fmt;

const NUM_STATES: usize = 12;
const NUM_POS_BITS_MAX: usize = 4;
const NUM_LEN_TO_POS_STATES: usize = 4;
const NUM_ALIGN_BITS: usize = 4;
const NUM_POS_SLOT_BITS: usize = 6;
const START_POS_MODEL_INDEX: u32 = 4;
const END_POS_MODEL_INDEX: u32 = 14;
const NUM_FULL_DISTANCES: usize = 1 << (END_POS_MODEL_INDEX >> 1);
const MATCH_MIN_LEN: usize = 2;
const LIT_SIZE: usize = 0x300;

/// Probability model scale: probabilities are 11-bit, adapted by a 1/32 step.
const PROB_INIT: u16 = 1024;
const NUM_MOVE_BITS: u16 = 5;
const TOP: u32 = 1 << 24;

/// The two UPX bytes in front of every LZMA block.
const HEADER_LEN: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LzmaError {
    /// The block is too short to hold the UPX property bytes plus a range-coder
    /// init word.
    TooShort,
    /// `lc`/`lp`/`pb` outside what LZMA1 defines, or a `lc + lp` wide enough to
    /// ask for an implausible literal table.
    BadProperties { lc: u8, lp: u8, pb: u8 },
    /// The range coder's first byte must be zero.
    BadRangeInit(u8),
    /// The decoder asked for a byte past the end of the block.
    InputOverrun,
    /// The stream produced more bytes than the block header promised.
    OutputOverrun,
    /// A match referenced bytes before the start of the output.
    LookbehindOverrun { dist: u32, have: usize },
    /// The stream ended -- on its end marker or on its input -- before `u_len`
    /// bytes were produced.
    Truncated { got: usize, want: usize },
    /// A distance-model index the LZMA constants make unreachable; a guard, not
    /// a stream a real encoder can produce.
    ModelIndex,
    /// A length-less decode reached the caller's output cap. Distinct from
    /// [`LzmaError::OutputOverrun`]: nothing declared this size, so it means
    /// "raise the cap or the range is wrong", not "the stream disagrees".
    OutputCap { max: usize },
}

impl fmt::Display for LzmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LzmaError::TooShort => write!(f, "LZMA block is too short to carry a header"),
            LzmaError::BadProperties { lc, lp, pb } => {
                write!(f, "LZMA properties lc={lc} lp={lp} pb={pb} are out of range")
            }
            LzmaError::BadRangeInit(b) => {
                write!(f, "LZMA range coder starts with {b:#04x}, not zero")
            }
            LzmaError::InputOverrun => write!(f, "compressed block ended mid-symbol"),
            LzmaError::OutputOverrun => write!(f, "block expanded past its declared size"),
            LzmaError::LookbehindOverrun { dist, have } => {
                write!(f, "match distance {dist} points before the block ({have} bytes decoded)")
            }
            LzmaError::Truncated { got, want } => {
                write!(f, "block decoded to {got} bytes, header declared {want}")
            }
            LzmaError::ModelIndex => write!(f, "LZMA distance model index out of range"),
            LzmaError::OutputCap { max } => {
                write!(f, "stream kept expanding past the {max}-byte output cap")
            }
        }
    }
}

/// Decompress one UPX LZMA block, header bytes included. `u_len` is the size
/// its `b_info` declares; a stream that does not reach it exactly is an error,
/// never a short read.
///
/// Unlike [`super::nrv::decompress`] this does not require the input to be
/// consumed exactly: an LZMA range coder holds up to four bytes of lookahead it
/// never uses, so trailing slack is a property of a *valid* stream. The
/// end-to-end integrity check is the packer's own adler32 over the decoded
/// bytes, which the walk verifies.
pub fn decompress(src: &[u8], u_len: usize) -> Result<Vec<u8>, LzmaError> {
    let (props, body) = Properties::split(src)?;
    let mut dec = Decoder::new(props, body, Some(u_len), u_len)?;
    dec.run()?;
    Ok(dec.out)
}

/// What a length-less decode recovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exhausted {
    pub bytes: Vec<u8>,
    /// Whether the stream stopped on its own end-of-stream marker. UPX writes
    /// none, so `false` is the normal answer and not a defect -- it only says
    /// the tail is bounded by the input rather than by the encoder.
    pub end_marker: bool,
    /// Compressed bytes the range coder actually read, header bytes included.
    /// Up to four fewer than `src.len()` on a well-formed stream, since the
    /// coder holds lookahead it never uses.
    pub consumed: usize,
}

/// Decompress a raw LZMA1 stream of *unknown* uncompressed length, header bytes
/// included, stopping on the end marker or on the end of `src` -- whichever
/// comes first -- and never producing more than `max` bytes.
///
/// This is the entry point for a stream nothing declares the size of: a UPX
/// image whose `PackHeader` was stripped, or a payload an analyst located in the
/// decompiler and passed in by address. Running out of input is the expected
/// ending here, so it returns what was decoded instead of [`LzmaError::Truncated`];
/// a corrupt stream still fails, because a bad distance or model index is a
/// different fact from a short one.
pub fn decompress_exhaustive(src: &[u8], max: usize) -> Result<Exhausted, LzmaError> {
    let (props, body) = Properties::split(src)?;
    let mut dec = Decoder::new(props, body, None, max)?;
    dec.run()?;
    let consumed = HEADER_LEN + dec.rc.pos;
    Ok(Exhausted { bytes: std::mem::take(&mut dec.out), end_marker: dec.end_marker, consumed })
}

/// `lc`/`lp`/`pb` as the two UPX header bytes spell them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Properties {
    pub lc: u8,
    pub lp: u8,
    pub pb: u8,
}

impl Properties {
    fn split(src: &[u8]) -> Result<(Properties, &[u8]), LzmaError> {
        let head = src.get(..HEADER_LEN).ok_or(LzmaError::TooShort)?;
        let props = Properties {
            lc: head[1] & 0x0f,
            lp: head[1] >> 4,
            pb: head[0] & 0x07,
        };
        // LZMA1 allows lc <= 8, lp <= 4, pb <= 4; the literal table is
        // 0x300 << (lc + lp) entries, so an out-of-range pair is refused here
        // rather than turned into a multi-gigabyte allocation.
        if props.lc > 8 || props.lp > 4 || props.pb > 4 || props.lc + props.lp > 8 {
            return Err(LzmaError::BadProperties { lc: props.lc, lp: props.lp, pb: props.pb });
        }
        Ok((props, &src[HEADER_LEN..]))
    }
}

/// The 11-bit adaptive binary range decoder.
struct RangeDecoder<'a> {
    src: &'a [u8],
    pos: usize,
    range: u32,
    code: u32,
}

impl<'a> RangeDecoder<'a> {
    fn new(src: &'a [u8]) -> Result<Self, LzmaError> {
        let init = src.get(..5).ok_or(LzmaError::TooShort)?;
        if init[0] != 0 {
            return Err(LzmaError::BadRangeInit(init[0]));
        }
        Ok(RangeDecoder {
            src,
            pos: 5,
            range: u32::MAX,
            code: u32::from_be_bytes([init[1], init[2], init[3], init[4]]),
        })
    }

    fn normalize(&mut self) -> Result<(), LzmaError> {
        if self.range < TOP {
            let b = *self.src.get(self.pos).ok_or(LzmaError::InputOverrun)?;
            self.pos += 1;
            self.range <<= 8;
            self.code = (self.code << 8) | u32::from(b);
        }
        Ok(())
    }

    fn bit(&mut self, prob: &mut u16) -> Result<u32, LzmaError> {
        let bound = (self.range >> 11) * u32::from(*prob);
        let bit = if self.code < bound {
            self.range = bound;
            *prob += (2048 - *prob) >> NUM_MOVE_BITS;
            0
        } else {
            self.range -= bound;
            self.code -= bound;
            *prob -= *prob >> NUM_MOVE_BITS;
            1
        };
        self.normalize()?;
        Ok(bit)
    }

    /// `count` equiprobable bits, MSB first.
    fn direct(&mut self, count: u32) -> Result<u32, LzmaError> {
        let mut result = 0u32;
        for _ in 0..count {
            self.range >>= 1;
            self.code = self.code.wrapping_sub(self.range);
            let t = 0u32.wrapping_sub(self.code >> 31);
            self.code = self.code.wrapping_add(self.range & t);
            self.normalize()?;
            result = (result << 1).wrapping_add(t.wrapping_add(1));
        }
        Ok(result)
    }

    fn bittree(&mut self, probs: &mut [u16], num_bits: usize) -> Result<u32, LzmaError> {
        let mut m = 1usize;
        for _ in 0..num_bits {
            m = (m << 1) | self.bit(&mut probs[m])? as usize;
        }
        Ok(m as u32 - (1 << num_bits))
    }

    /// The same tree read LSB-first, as the distance and align coders want it.
    /// `base` is the index the SDK's pointer arithmetic lands on; it is added to
    /// the walking index rather than to the slice so a `base` of `-1` (the
    /// `posSlot == 4` case) stays in bounds.
    fn bittree_rev(
        &mut self,
        probs: &mut [u16],
        base: isize,
        num_bits: usize,
    ) -> Result<u32, LzmaError> {
        let mut m = 1usize;
        let mut sym = 0u32;
        for i in 0..num_bits {
            let at = base + m as isize;
            let idx = usize::try_from(at).map_err(|_| LzmaError::ModelIndex)?;
            let p = probs.get_mut(idx).ok_or(LzmaError::ModelIndex)?;
            let b = self.bit(p)?;
            m = (m << 1) | b as usize;
            sym |= b << i;
        }
        Ok(sym)
    }
}

/// One of the two match-length coders (`len` and `rep_len`).
struct LenCoder {
    choice: u16,
    choice2: u16,
    low: Vec<u16>,
    mid: Vec<u16>,
    high: Vec<u16>,
}

impl LenCoder {
    fn new(pos_states: usize) -> Self {
        LenCoder {
            choice: PROB_INIT,
            choice2: PROB_INIT,
            low: vec![PROB_INIT; pos_states * 8],
            mid: vec![PROB_INIT; pos_states * 8],
            high: vec![PROB_INIT; 256],
        }
    }

    /// The raw symbol; the caller adds [`MATCH_MIN_LEN`].
    fn decode(&mut self, rc: &mut RangeDecoder<'_>, pos_state: usize) -> Result<usize, LzmaError> {
        if rc.bit(&mut self.choice)? == 0 {
            let v = rc.bittree(&mut self.low[pos_state * 8..(pos_state + 1) * 8], 3)?;
            return Ok(v as usize);
        }
        if rc.bit(&mut self.choice2)? == 0 {
            let v = rc.bittree(&mut self.mid[pos_state * 8..(pos_state + 1) * 8], 3)?;
            return Ok(8 + v as usize);
        }
        let v = rc.bittree(&mut self.high, 8)?;
        Ok(16 + v as usize)
    }
}

struct Decoder<'a> {
    rc: RangeDecoder<'a>,
    props: Properties,
    out: Vec<u8>,
    /// The declared uncompressed length, or `None` when the caller has none and
    /// the stream's own input is the only bound.
    u_len: Option<usize>,
    /// The hard ceiling on `out`: `u_len` when it is known, otherwise the cap
    /// [`decompress_exhaustive`] was given.
    max_out: usize,
    end_marker: bool,

    literal: Vec<u16>,
    is_match: Vec<u16>,
    is_rep: Vec<u16>,
    is_rep_g0: Vec<u16>,
    is_rep_g1: Vec<u16>,
    is_rep_g2: Vec<u16>,
    is_rep0_long: Vec<u16>,
    pos_slot: Vec<u16>,
    spec_pos: Vec<u16>,
    align: Vec<u16>,
    len_coder: LenCoder,
    rep_len_coder: LenCoder,
}

impl<'a> Decoder<'a> {
    fn new(
        props: Properties,
        body: &'a [u8],
        u_len: Option<usize>,
        max_out: usize,
    ) -> Result<Self, LzmaError> {
        let pos_states = 1usize << props.pb;
        Ok(Decoder {
            rc: RangeDecoder::new(body)?,
            props,
            out: Vec::with_capacity(u_len.unwrap_or(0)),
            u_len,
            max_out,
            end_marker: false,
            literal: vec![PROB_INIT; LIT_SIZE << (props.lc + props.lp)],
            is_match: vec![PROB_INIT; NUM_STATES << NUM_POS_BITS_MAX],
            is_rep: vec![PROB_INIT; NUM_STATES],
            is_rep_g0: vec![PROB_INIT; NUM_STATES],
            is_rep_g1: vec![PROB_INIT; NUM_STATES],
            is_rep_g2: vec![PROB_INIT; NUM_STATES],
            is_rep0_long: vec![PROB_INIT; NUM_STATES << NUM_POS_BITS_MAX],
            pos_slot: vec![PROB_INIT; NUM_LEN_TO_POS_STATES << NUM_POS_SLOT_BITS],
            spec_pos: vec![PROB_INIT; NUM_FULL_DISTANCES - END_POS_MODEL_INDEX as usize],
            align: vec![PROB_INIT; 1 << NUM_ALIGN_BITS],
            len_coder: LenCoder::new(pos_states),
            rep_len_coder: LenCoder::new(pos_states),
        })
    }

    /// The decode, plus the two endings only a *declared* length can judge: a
    /// stream that stops short of `u_len` is truncated, while one that stops
    /// short of the input with no length to reach is simply over.
    fn run(&mut self) -> Result<(), LzmaError> {
        match self.decode() {
            Ok(()) => {}
            // Running out of input is this mode's normal ending, not a defect.
            Err(LzmaError::InputOverrun) if self.u_len.is_none() => return Ok(()),
            Err(e) => return Err(e),
        }
        match self.u_len {
            Some(want) if self.out.len() != want => {
                Err(LzmaError::Truncated { got: self.out.len(), want })
            }
            _ => Ok(()),
        }
    }

    fn decode(&mut self) -> Result<(), LzmaError> {
        let pb_mask = (1usize << self.props.pb) - 1;
        let lp_mask = (1usize << self.props.lp) - 1;
        let mut state = 0usize;
        let (mut rep0, mut rep1, mut rep2, mut rep3) = (0u32, 0u32, 0u32, 0u32);

        // `max_out` is `u_len` when one was declared, so this one condition
        // serves both modes: the declared length, or the caller's cap.
        while self.out.len() < self.max_out {
            let pos = self.out.len();
            let pos_state = pos & pb_mask;
            if self.rc.bit(&mut self.is_match[(state << NUM_POS_BITS_MAX) + pos_state])? == 0 {
                let byte = self.literal(state, pos, lp_mask, rep0)?;
                self.out.push(byte);
                state = if state < 4 {
                    0
                } else if state < 10 {
                    state - 3
                } else {
                    state - 6
                };
                continue;
            }

            let len;
            if self.rc.bit(&mut self.is_rep[state])? != 0 {
                if pos == 0 {
                    return Err(LzmaError::LookbehindOverrun { dist: rep0, have: 0 });
                }
                if self.rc.bit(&mut self.is_rep_g0[state])? == 0 {
                    let long =
                        self.rc.bit(&mut self.is_rep0_long[(state << NUM_POS_BITS_MAX) + pos_state])?;
                    if long == 0 {
                        state = if state < 7 { 9 } else { 11 };
                        let b = self.back(rep0)?;
                        self.out.push(b);
                        continue;
                    }
                } else {
                    let dist = if self.rc.bit(&mut self.is_rep_g1[state])? == 0 {
                        rep1
                    } else if self.rc.bit(&mut self.is_rep_g2[state])? == 0 {
                        let d = rep2;
                        rep2 = rep1;
                        d
                    } else {
                        let d = rep3;
                        rep3 = rep2;
                        rep2 = rep1;
                        d
                    };
                    rep1 = rep0;
                    rep0 = dist;
                }
                len = self.rep_len_coder.decode(&mut self.rc, pos_state)?;
                state = if state < 7 { 8 } else { 11 };
            } else {
                rep3 = rep2;
                rep2 = rep1;
                rep1 = rep0;
                len = self.len_coder.decode(&mut self.rc, pos_state)?;
                state = if state < 7 { 7 } else { 10 };
                match self.distance(len)? {
                    Some(d) => rep0 = d,
                    // The end marker: legal, but only where the block is done.
                    None => {
                        self.end_marker = true;
                        break;
                    }
                }
            }

            self.copy_match(rep0, len + MATCH_MIN_LEN)?;
        }
        // With no declared length the loop has exactly two honest endings, and
        // both leave it early: the end marker, or input the range coder ran
        // off. Falling out of the condition instead means the cap bound it.
        if self.u_len.is_none() && !self.end_marker {
            return Err(LzmaError::OutputCap { max: self.max_out });
        }
        Ok(())
    }

    /// One literal byte -- context-coded on the previous byte, and additionally
    /// on the byte at `rep0` once the state says a match came just before.
    fn literal(
        &mut self,
        state: usize,
        pos: usize,
        lp_mask: usize,
        rep0: u32,
    ) -> Result<u8, LzmaError> {
        let prev = if pos == 0 { 0u32 } else { u32::from(self.out[pos - 1]) };
        let ctx = ((pos & lp_mask) << self.props.lc) + (prev >> (8 - self.props.lc)) as usize;
        let mut match_byte = if state >= 7 { u32::from(self.back(rep0)?) } else { 0 };

        let Decoder { rc, literal, .. } = self;
        let probs = &mut literal[ctx * LIT_SIZE..(ctx + 1) * LIT_SIZE];
        let mut symbol = 1usize;
        if state >= 7 {
            let mut offs = 0x100usize;
            while symbol < 0x100 {
                match_byte <<= 1;
                let bit = (match_byte as usize) & offs;
                let b = rc.bit(&mut probs[offs + bit + symbol])? as usize;
                symbol = (symbol << 1) | b;
                if b == 1 {
                    offs &= bit;
                } else {
                    offs &= !bit;
                }
                if offs == 0 {
                    break;
                }
            }
        }
        while symbol < 0x100 {
            symbol = (symbol << 1) | rc.bit(&mut probs[symbol])? as usize;
        }
        Ok(symbol as u8)
    }

    /// The match distance for a fresh (non-`rep`) match. `Ok(None)` is the
    /// end-of-stream marker.
    fn distance(&mut self, len: usize) -> Result<Option<u32>, LzmaError> {
        let len_state = len.min(NUM_LEN_TO_POS_STATES - 1);
        let slot = {
            let lo = len_state << NUM_POS_SLOT_BITS;
            let probs = &mut self.pos_slot[lo..lo + (1 << NUM_POS_SLOT_BITS)];
            self.rc.bittree(probs, NUM_POS_SLOT_BITS)?
        };
        if slot < START_POS_MODEL_INDEX {
            return Ok(Some(slot));
        }
        let num_direct = (slot >> 1) - 1;
        let mut dist = (2 | (slot & 1)) << num_direct;
        if slot < END_POS_MODEL_INDEX {
            let base = dist as isize - slot as isize - 1;
            dist += self.rc.bittree_rev(&mut self.spec_pos, base, num_direct as usize)?;
        } else {
            let direct = self.rc.direct(num_direct - NUM_ALIGN_BITS as u32)?;
            dist = dist.wrapping_add(direct << NUM_ALIGN_BITS);
            dist = dist.wrapping_add(self.rc.bittree_rev(&mut self.align, 0, NUM_ALIGN_BITS)?);
            if dist == u32::MAX {
                return Ok(None);
            }
        }
        Ok(Some(dist))
    }

    fn back(&self, dist: u32) -> Result<u8, LzmaError> {
        let have = self.out.len();
        let at = have
            .checked_sub(1)
            .and_then(|i| i.checked_sub(dist as usize))
            .ok_or(LzmaError::LookbehindOverrun { dist, have })?;
        Ok(self.out[at])
    }

    fn copy_match(&mut self, dist: u32, len: usize) -> Result<(), LzmaError> {
        if self.out.len() + len > self.max_out {
            return Err(match self.u_len {
                Some(_) => LzmaError::OutputOverrun,
                None => LzmaError::OutputCap { max: self.max_out },
            });
        }
        let mut at = self
            .out
            .len()
            .checked_sub(1)
            .and_then(|i| i.checked_sub(dist as usize))
            .ok_or(LzmaError::LookbehindOverrun { dist, have: self.out.len() })?;
        for _ in 0..len {
            let b = self.out[at];
            self.out.push(b);
            at += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The property bytes UPX writes for its default `lc=3 lp=0 pb=2` are
    /// `0x1a 0x03`; RetDec reads the same file the same way.
    #[test]
    fn upx_property_bytes_decode_to_the_default_triple() {
        let (p, body) = Properties::split(&[0x1a, 0x03, 0x00, 0, 0, 0, 0]).unwrap();
        assert_eq!(p, Properties { lc: 3, lp: 0, pb: 2 });
        assert_eq!(body.len(), 5);
    }

    #[test]
    fn out_of_range_properties_are_refused_not_allocated() {
        let err = Properties::split(&[0x00, 0xff, 0x00, 0, 0, 0, 0]).unwrap_err();
        assert!(matches!(err, LzmaError::BadProperties { lc: 15, lp: 15, pb: 0 }));
    }

    #[test]
    fn a_short_block_is_refused() {
        assert_eq!(decompress(&[0x1a], 4).unwrap_err(), LzmaError::TooShort);
        assert_eq!(decompress(&[0x1a, 0x03, 0, 0], 4).unwrap_err(), LzmaError::TooShort);
    }

    #[test]
    fn a_nonzero_range_init_byte_is_refused() {
        let err = decompress(&[0x1a, 0x03, 0x7f, 0, 0, 0, 0], 4).unwrap_err();
        assert_eq!(err, LzmaError::BadRangeInit(0x7f));
    }

    /// A stream of zero bytes decodes literals from an all-`PROB_INIT` model and
    /// runs out of input long before a real block would end: the decoder must
    /// say so rather than return what it managed.
    #[test]
    fn a_truncated_stream_is_an_error_not_a_short_read() {
        let mut src = vec![0x1a, 0x03];
        src.extend_from_slice(&[0u8; 16]);
        let err = decompress(&src, 1 << 20).unwrap_err();
        assert!(
            matches!(err, LzmaError::InputOverrun | LzmaError::Truncated { .. }),
            "unexpected {err:?}"
        );
    }

    /// Round-trip against a stream a different implementation produced: the
    /// first block of the vendored UPX 4.2.4 `--lzma` witness, which decodes to
    /// the original file's Ehdr + Phdrs.
    #[test]
    fn the_vendored_witness_first_block_decodes() {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/upx_packed_lzma_x86_64");
        let packed = std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
        // The first `b_info` sits right after `l_info` + `p_info`.
        let at = FIRST_BLOCK;
        let u_len = u32::from_le_bytes(packed[at..at + 4].try_into().unwrap()) as usize;
        let c_len = u32::from_le_bytes(packed[at + 4..at + 8].try_into().unwrap()) as usize;
        assert_eq!(packed[at + 8], 14, "the fixture's first block is LZMA");
        let out = decompress(&packed[at + 12..at + 12 + c_len], u_len).expect("first block");
        assert_eq!(&out[..4], b"\x7fELF");
        assert_eq!(out.len(), u_len);
    }

    /// The first `b_info` in the fixture, right after `l_info` + `p_info`.
    const FIRST_BLOCK: usize = 0x100;

    /// The same block again with its `u_len` withheld -- the situation
    /// `--raw-lzma` is for. UPX writes no end marker, so the decode has nothing
    /// to stop on but the input, and the byte count is what it recovers rather
    /// than what it was told.
    #[test]
    fn a_length_less_decode_ends_on_the_input_and_finds_the_same_bytes() {
        let (block, u_len) = witness_block();
        let got = decompress_exhaustive(&block, 1 << 20).expect("first block");
        assert_eq!(got.bytes.len(), u_len);
        assert_eq!(&got.bytes[..4], b"\x7fELF");
        assert!(!got.end_marker, "UPX writes no end-of-stream marker");
        assert_eq!(got.consumed, block.len(), "the whole block is read");
        assert_eq!(got.bytes, decompress(&block, u_len).unwrap());
    }

    /// The cap is the only bound a length-less decode has, so hitting it is
    /// reported rather than returned as a short answer.
    #[test]
    fn a_length_less_decode_that_hits_the_cap_says_so() {
        let (block, u_len) = witness_block();
        let err = decompress_exhaustive(&block, u_len / 2).unwrap_err();
        assert_eq!(err, LzmaError::OutputCap { max: u_len / 2 });
    }

    /// A declared length still judges the same stream the old way in both
    /// directions -- this is the path every `kuna unpack` of a real UPX image
    /// takes, and the cap that now serves the length-less mode must not have
    /// loosened it.
    #[test]
    fn a_declared_length_still_refuses_a_stream_that_misses_it() {
        let (block, u_len) = witness_block();
        assert_eq!(decompress(&block, u_len + 1).unwrap_err(), LzmaError::InputOverrun);
        assert_eq!(decompress(&block, u_len - 1).unwrap_err(), LzmaError::OutputOverrun);
    }

    /// The vendored witness's first block, and the `u_len` its `b_info` declares.
    fn witness_block() -> (Vec<u8>, usize) {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/upx_packed_lzma_x86_64");
        let packed = std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
        let at = FIRST_BLOCK;
        let u_len = u32::from_le_bytes(packed[at..at + 4].try_into().unwrap()) as usize;
        let c_len = u32::from_le_bytes(packed[at + 4..at + 8].try_into().unwrap()) as usize;
        (packed[at + 12..at + 12 + c_len].to_vec(), u_len)
    }
}
