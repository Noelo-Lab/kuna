//! NEOLite static unpacking -- the `.NEOpack` loader stub and its payload.
//!
//! NEOLite (NeoWorx, ~1999-2000) packs a PE by compressing each original
//! section in place and appending two sections of its own: `.NEOpack`, which
//! holds the loader stub and takes the entry point, and `.NEOdata`, a zero-sized
//! scratch section. The original section table survives untouched, so every
//! section's virtual address and virtual size still describe the original
//! image; only the bytes behind them are a compressed stream. That is what
//! makes the format statically recoverable: the answer to "where does this
//! decompress to" is already in the file, and nothing has to be inferred from
//! executing the stub.
//!
//! The codec is an LZX derivative. It carries LZX's 58 position slots and its
//! three repeated offsets, and it merges match length and position slot into
//! one main-tree symbol the way LZX does (`256 + slot * 8 + len_slot`), but its
//! length extension uses DEFLATE's length base/extra tables through a separate
//! 28-symbol tree rather than LZX's 249-symbol length tree. Huffman code
//! lengths arrive DEFLATE-style through a 19-symbol code-length tree, delta-coded
//! modulo 16 against the previous block's lengths.
//!
//! What this module does NOT recover is anything the stub synthesizes at run
//! time that the file does not also still hold. In practice that is nothing:
//! the original import directory is inside the compressed `.rdata` and is found
//! by scanning the recovered image, and the original entry point is the operand
//! of the stub's own `push OEP; ret` hand-over. A section the stub stores
//! uncompressed (resources typically are) is copied through as it stands.

use std::fmt;

/// DEFLATE's length base table, indexed by the 28-symbol length tree.
const LEN_BASE: [u16; 28] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 32, 40, 48, 56, 64, 80, 96, 112, 128,
    160, 192, 224,
];

/// Extra bits read after each length-tree symbol.
const LEN_EXTRA: [u8; 28] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

/// LZX's position-slot extra bits. The slot bases are the running sums of
/// `1 << extra`, exactly as the stub computes them at startup.
const DIST_EXTRA: [u8; 58] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13, 14, 14, 15, 15, 16, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 18, 18,
    18, 18, 18, 18, 18,
];

const NUM_MAIN: usize = 721;
const NUM_LEN: usize = 28;
const NUM_ALIGN: usize = 8;
const NUM_LENS: usize = NUM_MAIN + NUM_LEN + NUM_ALIGN;
/// The main-tree symbol that ends a block and introduces the next one's trees.
const SYM_END: u32 = 720;
const MAX_CODE_BITS: usize = 15;
/// The bit reader's window, in bits -- codes are compared left-justified in it.
const VALUE_BITS: u32 = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeoliteError {
    /// No `.NEOpack` section owning the entry point: not a NEOLite image.
    NotPacked,
    /// Recognized as NEOLite, but this build cannot unpack it -- and says why.
    Unsupported(String),
    /// The headers are NEOLite's but do not describe a consistent file.
    Corrupt(String),
}

impl fmt::Display for NeoliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeoliteError::NotPacked => write!(f, "no NEOLite loader section found"),
            NeoliteError::Unsupported(what) => write!(f, "unsupported NEOLite image: {what}"),
            NeoliteError::Corrupt(what) => write!(f, "corrupt NEOLite image: {what}"),
        }
    }
}

impl std::error::Error for NeoliteError {}

/// What became of one section of the packed image.
#[derive(Debug, Clone)]
pub struct SectionOutcome {
    pub name: String,
    pub va: u32,
    /// Bytes the packed file stored for it.
    pub packed: u32,
    /// Bytes the rebuilt file stores for it.
    pub unpacked: u32,
    /// The stored bytes were a compressed stream and were decoded. False for a
    /// section the packer left alone (and for the stub's own sections).
    pub compressed: bool,
}

/// A successful unpack.
#[derive(Debug)]
pub struct Unpacked {
    /// The reconstructed original file.
    pub bytes: Vec<u8>,
    /// RVA the rebuilt image enters at -- the stub's `push OEP; ret` operand.
    pub entry: u32,
    /// RVA the packed image entered at, inside the stub.
    pub stub_entry: u32,
    /// RVA of the original import directory, when it was found in the recovered
    /// image and re-pointed at.
    pub imports: Option<u32>,
    pub sections: Vec<SectionOutcome>,
}

impl Unpacked {
    /// Bytes of compressed stream decoded, across every section.
    pub fn compressed_bytes(&self) -> u64 {
        self.sections.iter().filter(|s| s.compressed).map(|s| u64::from(s.packed)).sum()
    }

    /// How many sections held a compressed stream.
    pub fn compressed_sections(&self) -> usize {
        self.sections.iter().filter(|s| s.compressed).count()
    }
}

/// Is this image NEOLite-packed? Cheap: section names and the entry point.
pub fn detect(image: &[u8]) -> bool {
    Pe::parse(image).map(|pe| pe.stub_index().is_some()).unwrap_or(false)
}

/// Unpack `image`, returning the reconstructed original file.
pub fn unpack(image: &[u8]) -> Result<Unpacked, NeoliteError> {
    let pe = Pe::parse(image).ok_or(NeoliteError::NotPacked)?;
    let stub = pe.stub_index().ok_or(NeoliteError::NotPacked)?;
    if pe.machine != 0x14c || pe.magic != 0x10b {
        return Err(NeoliteError::Unsupported(format!(
            "machine {:#06x} / optional header magic {:#06x} -- only 32-bit x86 NEOLite images \
             have been seen, so the layout this would rebuild is unverified",
            pe.machine, pe.magic
        )));
    }
    pe.rebuild(image, stub)
}

// ---------------------------------------------------------------------------
// The bit reader.
//
// A 32-bit accumulator refilled a byte at a time, whose most significant bits
// are the next code. `bit_pos` counts bits consumed out of the low byte, so the
// live window is `value >> (8 - bit_pos)` once normalization has brought
// `bit_pos` back under 8. Starting it at 32 is what makes the first read fill
// the accumulator.
// ---------------------------------------------------------------------------

struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    value: u32,
    bit_pos: u32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitReader { data, pos: 0, value: 0, bit_pos: 32 }
    }

    /// True once the reader has run off the end and is being fed zeros.
    fn exhausted(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn normalize(&mut self) {
        while self.bit_pos >= 8 {
            let b = self.data.get(self.pos).copied().unwrap_or(0);
            self.pos += 1;
            self.value = (self.value << 8) | u32::from(b);
            self.bit_pos -= 8;
        }
    }

    fn peek(&mut self) -> u32 {
        self.normalize();
        (self.value >> (8 - self.bit_pos)) & 0x00ff_ffff
    }

    fn take(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let v = self.peek() >> (VALUE_BITS - n);
        self.bit_pos += n;
        v
    }
}

// ---------------------------------------------------------------------------
// Canonical Huffman decoding, left-justified in the 24-bit window.
//
// `limits[k]` is the first window value above every code of length `k`, so a
// code of length `k` satisfies `limits[k - 1] <= val < limits[k]`, and its rank
// inside that length falls straight out of the subtraction.
// ---------------------------------------------------------------------------

struct Huffman {
    limits: [u32; MAX_CODE_BITS + 1],
    poses: [u32; MAX_CODE_BITS + 1],
    syms: Vec<u16>,
}

impl Huffman {
    /// Build from code lengths, refusing anything that is not a complete
    /// canonical code. Completeness is the whole validity test a stream gets
    /// before it is decoded, so it has to be exact: an over- or under-subscribed
    /// table is how a section the packer stored uncompressed announces itself.
    fn build(lens: &[u8]) -> Result<Huffman, String> {
        let mut counts = [0u32; MAX_CODE_BITS + 1];
        for &l in lens {
            if l as usize > MAX_CODE_BITS {
                return Err(format!("code length {l} exceeds {MAX_CODE_BITS}"));
            }
            if l != 0 {
                counts[l as usize] += 1;
            }
        }
        let used: u32 = counts.iter().sum();
        if used == 0 {
            return Err("no symbol carries a code".into());
        }
        let mut limits = [0u32; MAX_CODE_BITS + 1];
        let mut poses = [0u32; MAX_CODE_BITS + 1];
        let mut start: u64 = 0;
        let mut sum = 0u32;
        for k in 1..=MAX_CODE_BITS {
            start += u64::from(counts[k]) << (VALUE_BITS as usize - k);
            if start > 1u64 << VALUE_BITS {
                return Err(format!("over-subscribed code table at length {k}"));
            }
            limits[k] = start as u32;
            poses[k] = sum;
            sum += counts[k];
        }
        // One symbol cannot fill the code space and is still well defined; any
        // other short table means these bytes are not a stream.
        if start != 1u64 << VALUE_BITS && used != 1 {
            return Err("incomplete code table".into());
        }
        let mut syms = Vec::with_capacity(used as usize);
        for k in 1..=MAX_CODE_BITS {
            for (i, &l) in lens.iter().enumerate() {
                if l as usize == k {
                    syms.push(i as u16);
                }
            }
        }
        Ok(Huffman { limits, poses, syms })
    }

    fn decode(&self, bits: &mut BitReader) -> Result<u32, String> {
        let val = bits.peek() & 0x00ff_fe00;
        let mut k = 1usize;
        while k <= MAX_CODE_BITS && val >= self.limits[k] {
            k += 1;
        }
        if k > MAX_CODE_BITS {
            return Err("no code matches the next 15 bits".into());
        }
        let idx = self.poses[k] + ((val - self.limits[k - 1]) >> (VALUE_BITS as usize - k));
        let sym = *self
            .syms
            .get(idx as usize)
            .ok_or_else(|| format!("code index {idx} past the {}-symbol table", self.syms.len()))?;
        bits.bit_pos += k as u32;
        Ok(u32::from(sym))
    }
}

/// The three Huffman trees a block carries, plus the block's distance mode.
struct Trees {
    main: Huffman,
    len: Huffman,
    align: Huffman,
    /// The aligned-offset mode: the low three bits of a wide distance come from
    /// the aligned tree instead of the raw stream. A tree whose eight codes are
    /// all three bits long decodes identically either way, and that is exactly
    /// how the packer spells "verbatim".
    aligned: bool,
}

/// Read one block header: the delta-coding reset flag, the code-length tree,
/// then the 757 code lengths the block's three trees share.
fn read_trees(bits: &mut BitReader, prev: &mut [u8; NUM_LENS]) -> Result<Trees, String> {
    if bits.take(1) == 0 {
        prev.fill(0);
    }
    let mut cl = [0u8; 19];
    for slot in cl.iter_mut() {
        *slot = bits.take(4) as u8;
    }
    let cl_tree = Huffman::build(&cl).map_err(|e| format!("code-length tree: {e}"))?;

    let mut lens = [0u8; NUM_LENS];
    let mut i = 0usize;
    while i < NUM_LENS {
        if bits.exhausted() {
            return Err("stream ended inside the code-length table".into());
        }
        let sym = cl_tree.decode(bits)?;
        match sym {
            0..=15 => {
                lens[i] = (prev[i].wrapping_add(sym as u8)) & 15;
                i += 1;
            }
            16 => {
                if i == 0 {
                    return Err("a repeat-previous code opens the code-length table".into());
                }
                let mut n = bits.take(2) + 3;
                while n > 0 && i < NUM_LENS {
                    lens[i] = lens[i - 1];
                    i += 1;
                    n -= 1;
                }
            }
            17 => {
                let mut n = bits.take(3) + 3;
                while n > 0 && i < NUM_LENS {
                    lens[i] = 0;
                    i += 1;
                    n -= 1;
                }
            }
            _ => {
                let mut n = bits.take(7) + 11;
                while n > 0 && i < NUM_LENS {
                    lens[i] = 0;
                    i += 1;
                    n -= 1;
                }
            }
        }
    }

    let align_lens = &lens[NUM_MAIN + NUM_LEN..];
    let aligned = !align_lens.iter().all(|&l| l == 3);
    let trees = Trees {
        main: Huffman::build(&lens[..NUM_MAIN]).map_err(|e| format!("main tree: {e}"))?,
        len: Huffman::build(&lens[NUM_MAIN..NUM_MAIN + NUM_LEN])
            .map_err(|e| format!("length tree: {e}"))?,
        align: Huffman::build(align_lens).map_err(|e| format!("aligned tree: {e}"))?,
        aligned,
    };
    *prev = lens;
    Ok(trees)
}

/// Decode one section's stream to `out_len` bytes.
///
/// The packer sizes a stream by the section's virtual size, zero tail included,
/// so a well-formed stream ends exactly when the section is full. A stream that
/// runs out first is truncated rather than corrupt -- what is decoded is kept
/// and the rest is zero -- because the alternative is decoding the bytes past
/// the end as if they were symbols.
pub fn decompress(input: &[u8], out_len: usize) -> Result<Vec<u8>, String> {
    let dist_base = {
        let mut base = [0u32; 58];
        let mut acc = 0u32;
        for (i, &e) in DIST_EXTRA.iter().enumerate() {
            base[i] = acc;
            acc += 1 << e;
        }
        base
    };

    let mut bits = BitReader::new(input);
    let mut prev = [0u8; NUM_LENS];
    let mut trees = read_trees(&mut bits, &mut prev)?;
    // Grown into rather than reserved: `out_len` comes from a section header.
    let mut out: Vec<u8> = Vec::with_capacity(out_len.min(1 << 20));
    // LZX's three most recent distances, which slots 0..2 name instead of
    // encoding. They span blocks.
    let mut recent = [0u32; 3];

    while out.len() < out_len {
        if bits.exhausted() {
            break;
        }
        let sym = trees.main.decode(&mut bits)?;
        if sym < 256 {
            out.push(sym as u8);
            continue;
        }
        if sym >= SYM_END {
            trees = read_trees(&mut bits, &mut prev)?;
            continue;
        }
        let packed = sym - 256;
        let len_slot = packed & 7;
        let dist_slot = (packed >> 3) as usize;
        let mut length = len_slot + 2;
        if len_slot == 7 {
            let i = trees.len.decode(&mut bits)? as usize;
            let extra = bits.take(u32::from(LEN_EXTRA[i]));
            length += u32::from(LEN_BASE[i]) + extra;
        }

        let nbits = u32::from(DIST_EXTRA[dist_slot]);
        let base = dist_base[dist_slot];
        let formatted = if trees.aligned && nbits >= 3 {
            let high = bits.take(nbits - 3);
            let low = trees.align.decode(&mut bits)?;
            base + low + (high << 3)
        } else {
            base + bits.take(nbits)
        };

        let offset = if formatted < 3 {
            let d = recent[formatted as usize];
            if formatted != 0 {
                recent[formatted as usize] = recent[0];
                recent[0] = d;
            }
            d
        } else {
            let d = formatted - 3;
            recent[2] = recent[1];
            recent[1] = recent[0];
            recent[0] = d;
            d
        };

        let dist = offset as usize + 1;
        if dist > out.len() {
            return Err(format!(
                "match distance {dist} reaches back past the {} bytes decoded so far",
                out.len()
            ));
        }
        let take = (length as usize).min(out_len - out.len());
        for _ in 0..take {
            let b = out[out.len() - dist];
            out.push(b);
        }
    }

    out.resize(out_len, 0);
    Ok(out)
}

// ---------------------------------------------------------------------------
// The PE half: recognizing the stub, and rebuilding the image around what the
// decoder produced.
// ---------------------------------------------------------------------------

/// How far past the entry point the `push OEP; ret` hand-over is looked for.
const HANDOVER_WINDOW: usize = 64;
/// The ceiling on any one section's decompressed size. A section header is
/// attacker-controlled and a virtual size is what the decode allocates against,
/// so it is bounded before it is believed.
const MAX_SECTION_BYTES: u32 = 256 << 20;
/// The ceiling on the rebuilt file as a whole, for the same reason.
const MAX_IMAGE_BYTES: usize = 1 << 30;
/// `IMAGE_SCN_CNT_CODE | MEM_EXECUTE | MEM_READ`, restored on the section the
/// recovered entry point lands in. The packer rewrites every section to plain
/// read-write data, and a `.text` that does not say it is code is a section no
/// disassembler will walk.
const TEXT_CHARACTERISTICS: u32 = 0x6000_0020;

struct Section {
    name: String,
    virtual_size: u32,
    virtual_address: u32,
    raw_size: u32,
    raw_offset: u32,
    /// Offset of this section's 40-byte header in the file.
    header_at: usize,
}

impl Section {
    fn is_stub(&self) -> bool {
        self.name == ".NEOpack" || self.name == ".NEOdata"
    }
}

struct Pe {
    machine: u16,
    magic: u16,
    entry: u32,
    image_base: u32,
    file_alignment: u32,
    size_of_headers: u32,
    opt_at: usize,
    num_data_dirs: u32,
    sections: Vec<Section>,
}

fn rd16(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn rd32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn wr32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn align_up(v: u32, a: u32) -> Option<u32> {
    if a == 0 {
        return Some(v);
    }
    v.checked_add(a - 1).map(|x| x / a * a)
}

impl Pe {
    fn parse(image: &[u8]) -> Option<Pe> {
        if image.get(..2)? != b"MZ" {
            return None;
        }
        let pe_at = rd32(image, 0x3c)? as usize;
        if image.get(pe_at..pe_at + 4)? != b"PE\0\0" {
            return None;
        }
        let machine = rd16(image, pe_at + 4)?;
        let num_sections = rd16(image, pe_at + 6)? as usize;
        let opt_size = rd16(image, pe_at + 20)? as usize;
        let opt_at = pe_at + 24;
        let magic = rd16(image, opt_at)?;
        // Everything read before the magic check sits at the same offset in a
        // PE32 and a PE32+ optional header, which is what lets a 64-bit NEOLite
        // image be recognized and then declined by name rather than reported
        // "not packed".
        let entry = rd32(image, opt_at + 16)?;
        // These four are PE32-shaped; `unpack` refuses any other magic before
        // a single one of them is used to rebuild anything.
        let pe32 = magic == 0x10b;
        let image_base = if pe32 { rd32(image, opt_at + 28)? } else { 0 };
        let file_alignment = if pe32 { rd32(image, opt_at + 36)? } else { 0 };
        let size_of_headers = if pe32 { rd32(image, opt_at + 60)? } else { 0 };
        let num_data_dirs = if pe32 { rd32(image, opt_at + 92)? } else { 0 };
        let table_at = opt_at + opt_size;
        let mut sections = Vec::with_capacity(num_sections);
        for i in 0..num_sections {
            let at = table_at + i * 40;
            let raw = image.get(at..at + 40)?;
            let end = raw[..8].iter().position(|&c| c == 0).unwrap_or(8);
            sections.push(Section {
                name: String::from_utf8_lossy(&raw[..end]).into_owned(),
                virtual_size: rd32(raw, 8)?,
                virtual_address: rd32(raw, 12)?,
                raw_size: rd32(raw, 16)?,
                raw_offset: rd32(raw, 20)?,
                header_at: at,
            });
        }
        Some(Pe {
            machine,
            magic,
            entry,
            image_base,
            file_alignment,
            size_of_headers,
            opt_at,
            num_data_dirs,
            sections,
        })
    }

    /// The `.NEOpack` section, if it exists and owns the entry point. Both
    /// halves matter: the name alone is a string anyone can write into a section
    /// header, and it is the entry point landing in the stub that says the stub
    /// is what runs first.
    fn stub_index(&self) -> Option<usize> {
        let i = self.sections.iter().position(|s| s.name == ".NEOpack")?;
        let s = &self.sections[i];
        let span = s.virtual_size.max(s.raw_size);
        (self.entry >= s.virtual_address && self.entry - s.virtual_address < span).then_some(i)
    }

    fn section_of_rva(&self, rva: u32) -> Option<&Section> {
        self.sections.iter().find(|s| {
            let span = s.virtual_size.max(s.raw_size);
            span != 0 && rva >= s.virtual_address && rva - s.virtual_address < span
        })
    }

    /// The original entry point, read out of the stub's `push OEP; ret`.
    fn original_entry(&self, image: &[u8], stub: usize) -> Result<u32, NeoliteError> {
        let s = &self.sections[stub];
        let from = (self.entry - s.virtual_address) as usize + s.raw_offset as usize;
        let to = (from + HANDOVER_WINDOW).min(image.len());
        let win = image.get(from..to).ok_or_else(|| {
            NeoliteError::Corrupt(format!("entry point {:#x} is not backed by file bytes", self.entry))
        })?;
        for i in 0..win.len().saturating_sub(5) {
            if win[i] != 0x68 || win[i + 5] != 0xc3 {
                continue;
            }
            let va = u32::from_le_bytes(win[i + 1..i + 5].try_into().unwrap());
            let Some(rva) = va.checked_sub(self.image_base) else { continue };
            match self.section_of_rva(rva) {
                Some(sec) if !sec.is_stub() => return Ok(rva),
                _ => continue,
            }
        }
        Err(NeoliteError::Unsupported(
            "the stub does not hand over with `push OEP; ret` in its first instructions -- \
             this is a NEOLite variant whose entry point cannot be read statically"
                .into(),
        ))
    }

    fn rebuild(&self, image: &[u8], stub: usize) -> Result<Unpacked, NeoliteError> {
        let oep = self.original_entry(image, stub)?;
        let file_alignment = if self.file_alignment == 0 { 0x200 } else { self.file_alignment };

        let mut bodies: Vec<Vec<u8>> = Vec::with_capacity(self.sections.len());
        let mut outcomes: Vec<SectionOutcome> = Vec::with_capacity(self.sections.len());
        let mut decoded_any = false;
        for s in &self.sections {
            let from = s.raw_offset as usize;
            let to = from.saturating_add(s.raw_size as usize).min(image.len());
            let stored: &[u8] = if from < to { &image[from..to] } else { &[] };
            let mut compressed = false;
            let body = if s.is_stub()
                || stored.is_empty()
                || s.virtual_size <= s.raw_size
                || s.virtual_size > MAX_SECTION_BYTES
            {
                stored.to_vec()
            } else {
                match decompress(stored, s.virtual_size as usize) {
                    Ok(bytes) => {
                        compressed = true;
                        decoded_any = true;
                        bytes
                    }
                    // A section the packer left alone -- resources typically are
                    // -- fails the code-table validity test on its first block
                    // header and is carried through exactly as it stands.
                    Err(_) => stored.to_vec(),
                }
            };
            outcomes.push(SectionOutcome {
                name: s.name.clone(),
                va: s.virtual_address,
                packed: s.raw_size,
                unpacked: body.len() as u32,
                compressed,
            });
            bodies.push(body);
        }
        if !decoded_any {
            return Err(NeoliteError::Corrupt(
                "no section holds a decodable stream -- the payload is not where the section \
                 table says it is"
                    .into(),
            ));
        }

        let header_bytes = align_up(self.size_of_headers.max(1), file_alignment)
            .ok_or_else(|| NeoliteError::Corrupt("header size overflows".into()))?
            as usize;
        if header_bytes > image.len() {
            return Err(NeoliteError::Corrupt("headers run past the end of the file".into()));
        }
        // Every field the rebuild writes has to live inside the bytes the
        // rebuild copies, and a hostile SizeOfHeaders is how it would not.
        let last_header_end = self.sections.iter().map(|s| s.header_at + 40).max().unwrap_or(0);
        if last_header_end > header_bytes || self.opt_at + 96 + 16 > header_bytes {
            return Err(NeoliteError::Corrupt(
                "the section table lies outside the image's own declared headers".into(),
            ));
        }
        let mut out = image[..header_bytes].to_vec();
        let mut cursor = header_bytes as u32;
        for (i, s) in self.sections.iter().enumerate() {
            let body = &bodies[i];
            let padded = u32::try_from(body.len())
                .ok()
                .and_then(|n| align_up(n, file_alignment))
                .and_then(|n| cursor.checked_add(n).map(|_| n))
                .ok_or_else(|| NeoliteError::Corrupt("the rebuilt image does not fit".into()))?;
            let (offset, size) = if body.is_empty() { (0, 0) } else { (cursor, padded) };
            let h = s.header_at;
            wr32(&mut out, h + 16, size);
            wr32(&mut out, h + 20, offset);
            if body.is_empty() {
                continue;
            }
            if out.len() + padded as usize > MAX_IMAGE_BYTES {
                return Err(NeoliteError::Corrupt(
                    "the section table describes an image too large to rebuild".into(),
                ));
            }
            out.extend_from_slice(body);
            out.resize(out.len() + (padded as usize - body.len()), 0);
            cursor += padded;
        }

        wr32(&mut out, self.opt_at + 16, oep);
        if let Some(entry_sec) = self.section_of_rva(oep) {
            wr32(&mut out, entry_sec.header_at + 36, TEXT_CHARACTERISTICS);
        }

        let imports = self.find_import_directory(&bodies);
        if let Some((rva, size)) = imports {
            if self.num_data_dirs > 1 {
                wr32(&mut out, self.opt_at + 96 + 8, rva);
                wr32(&mut out, self.opt_at + 96 + 12, size);
            }
        }

        Ok(Unpacked {
            bytes: out,
            entry: oep,
            stub_entry: self.entry,
            imports: imports.map(|(rva, _)| rva),
            sections: outcomes,
        })
    }

    /// Find the original `IMAGE_IMPORT_DESCRIPTOR` array in the recovered
    /// image. The packer builds its own two-entry table in `.NEOpack` and points
    /// the data directory at it; the program's real one is still sitting in the
    /// `.rdata` that was just decompressed, and it identifies itself -- every
    /// descriptor names a `.dll` and the array ends in twenty zero bytes.
    fn find_import_directory(&self, bodies: &[Vec<u8>]) -> Option<(u32, u32)> {
        let mut best: Option<(u32, u32)> = None;
        for (i, s) in self.sections.iter().enumerate() {
            if s.is_stub() {
                continue;
            }
            let body = &bodies[i];
            let mut at = 0usize;
            while at + 40 <= body.len() {
                if let Some(n) = self.count_descriptors(body, at, bodies) {
                    let cand = (s.virtual_address + at as u32, ((n + 1) * 20) as u32);
                    if best.is_none_or(|(_, size)| cand.1 > size) {
                        best = Some(cand);
                    }
                }
                at += 4;
            }
        }
        best
    }

    /// How many valid descriptors start at `at`, if this is a terminated array
    /// of at least one. `None` for anything else.
    fn count_descriptors(&self, body: &[u8], at: usize, bodies: &[Vec<u8>]) -> Option<usize> {
        let mut n = 0usize;
        let mut off = at;
        loop {
            let d = body.get(off..off + 20)?;
            let name = rd32(d, 12)?;
            let first_thunk = rd32(d, 16)?;
            if d.iter().all(|&b| b == 0) {
                return (n > 0).then_some(n);
            }
            // TimeDateStamp and ForwarderChain are zero in an unbound image,
            // which is what a packed one always is.
            if rd32(d, 4)? != 0 || rd32(d, 8)? != 0 || name == 0 || first_thunk == 0 {
                return None;
            }
            let dll = self.cstring_at(name, bodies)?;
            let lower = dll.to_ascii_lowercase();
            if !lower.ends_with(".dll") || dll.len() < 5 {
                return None;
            }
            self.section_of_rva(first_thunk)?;
            n += 1;
            if n > 64 {
                return None;
            }
            off += 20;
        }
    }

    /// The NUL-terminated ASCII string at `rva` in the recovered image, if it is
    /// short, printable and inside a section.
    fn cstring_at(&self, rva: u32, bodies: &[Vec<u8>]) -> Option<String> {
        let (i, s) = self
            .sections
            .iter()
            .enumerate()
            .find(|(_, s)| {
                let span = s.virtual_size.max(s.raw_size);
                span != 0 && rva >= s.virtual_address && rva - s.virtual_address < span
            })?;
        let body = &bodies[i];
        let from = (rva - s.virtual_address) as usize;
        let win = body.get(from..(from + 64).min(body.len()))?;
        let end = win.iter().position(|&c| c == 0)?;
        let bytes = &win[..end];
        if bytes.is_empty() || !bytes.iter().all(|&c| (0x20..0x7f).contains(&c)) {
            return None;
        }
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The generated witness -- see `tests/fixtures/neolite_packed_pe_i386.py`,
    /// which carries a reference encoder for the format and rebuilds this file.
    fn fixture() -> Vec<u8> {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/neolite_packed_pe_i386.exe");
        std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
    }

    /// The 16-byte snippet `.text` is 256 copies of.
    const SNIPPET: [u8; 16] = [
        0x55, 0x8b, 0xec, 0x8b, 0x45, 0x08, 0x03, 0x4d, 0x0c, 0x5d, 0xc3, 0x90, 0x90, 0x90, 0x90,
        0x90,
    ];

    /// The rebuilt file's section bodies, by name.
    fn sections(image: &[u8]) -> Vec<(String, u32, Vec<u8>)> {
        let pe = Pe::parse(image).expect("rebuilt file parses");
        pe.sections
            .iter()
            .map(|s| {
                let from = s.raw_offset as usize;
                let to = (from + s.raw_size as usize).min(image.len());
                let body = if from < to { image[from..to].to_vec() } else { Vec::new() };
                (s.name.clone(), rd32(image, s.header_at + 36).unwrap(), body)
            })
            .collect()
    }

    #[test]
    fn a_file_with_no_loader_section_is_not_neolite() {
        assert!(!detect(b"\x7fELF not a PE at all"));
        assert_eq!(unpack(b"MZ short").err(), Some(NeoliteError::NotPacked));
    }

    #[test]
    fn the_loader_section_must_own_the_entry_point() {
        // The name alone proves nothing: move the entry point out of
        // `.NEOpack` and the image stops being a NEOLite image.
        let mut image = fixture();
        let pe = Pe::parse(&image).unwrap();
        assert!(detect(&image));
        wr32(&mut image, pe.opt_at + 16, 0x1000);
        assert!(!detect(&image));
    }

    #[test]
    fn the_fixture_unpacks_to_its_original_sections() {
        let u = unpack(&fixture()).expect("fixture unpacks");
        assert_eq!(u.entry, 0x1000, "original entry point, read from the stub");
        assert_eq!(u.stub_entry, 0x6001);
        assert_eq!(u.compressed_sections(), 3);

        let secs = sections(&u.bytes);
        let text = &secs.iter().find(|(n, _, _)| n == ".text").unwrap().2;
        assert_eq!(text.len(), 0x1000);
        assert!(
            text.chunks_exact(16).all(|c| c == SNIPPET),
            "every 16-byte row of .text decodes back to the snippet"
        );

        let data = &secs.iter().find(|(n, _, _)| n == ".data").unwrap().2;
        assert_eq!(data.len(), 0x2000);
        assert!(data.starts_with(b"kuna neolite fixture payload\0"));
        assert!(data[29..].iter().all(|&b| b == 0), "the zero tail is part of the stream");
    }

    #[test]
    fn a_section_the_packer_stored_is_carried_through_verbatim() {
        let u = unpack(&fixture()).expect("fixture unpacks");
        let rsrc = u.sections.iter().find(|s| s.name == ".rsrc").unwrap();
        assert!(!rsrc.compressed, "plaintext must not be run through the decoder");
        assert_eq!(rsrc.packed, rsrc.unpacked);
        let secs = sections(&u.bytes);
        let body = &secs.iter().find(|(n, _, _)| n == ".rsrc").unwrap().2;
        assert!(body.starts_with(b"stored plaintext the packer left alone"));
    }

    #[test]
    fn the_entry_section_is_restored_as_code() {
        let u = unpack(&fixture()).expect("fixture unpacks");
        let secs = sections(&u.bytes);
        let (_, ch, _) = secs.iter().find(|(n, _, _)| n == ".text").unwrap();
        assert_eq!(*ch, TEXT_CHARACTERISTICS, "the packer left .text as plain rw data");
        let (_, other, _) = secs.iter().find(|(n, _, _)| n == ".data").unwrap();
        assert_eq!(*other, 0xC000_0040, "no other section's flags are touched");
    }

    #[test]
    fn the_original_import_directory_is_found_and_pointed_at() {
        let u = unpack(&fixture()).expect("fixture unpacks");
        assert_eq!(u.imports, Some(0x2040));
        let pe = Pe::parse(&u.bytes).unwrap();
        assert_eq!(rd32(&u.bytes, pe.opt_at + 96 + 8), Some(0x2040));

        // Walk it the way a disassembler would: a table nothing can follow is
        // the failure this guards.
        let names: Vec<String> = {
            let mut out = Vec::new();
            let mut d = 0x2040u32;
            loop {
                let at = |rva: u32| -> usize {
                    let s = pe.section_of_rva(rva).unwrap();
                    (s.raw_offset + (rva - s.virtual_address)) as usize
                };
                let dll_rva = rd32(&u.bytes, at(d) + 12).unwrap();
                if dll_rva == 0 {
                    break;
                }
                let a = at(dll_rva);
                let n = u.bytes[a..].iter().position(|&c| c == 0).unwrap();
                let dll = String::from_utf8_lossy(&u.bytes[a..a + n]).into_owned();
                let mut thunk = rd32(&u.bytes, at(d) + 16).unwrap();
                loop {
                    let hint = rd32(&u.bytes, at(thunk)).unwrap();
                    if hint == 0 {
                        break;
                    }
                    let a = at(hint) + 2;
                    let n = u.bytes[a..].iter().position(|&c| c == 0).unwrap();
                    out.push(format!("{dll}!{}", String::from_utf8_lossy(&u.bytes[a..a + n])));
                    thunk += 4;
                }
                d += 20;
            }
            out
        };
        assert_eq!(
            names,
            vec![
                "KERNEL32.dll!ExitProcess",
                "KERNEL32.dll!GetTickCount",
                "USER32.dll!MessageBoxA"
            ]
        );
    }

    #[test]
    fn an_incomplete_code_table_is_refused() {
        // Completeness is the only thing standing between a section the packer
        // stored and a decode of it as though it were a stream.
        assert!(Huffman::build(&[0u8; 19]).is_err(), "no codes at all");
        assert!(Huffman::build(&[1, 3, 3]).is_err(), "under-subscribed");
        assert!(Huffman::build(&[1, 1, 1]).is_err(), "over-subscribed");
        assert!(Huffman::build(&[1, 2, 2]).is_ok());
        assert!(Huffman::build(&[0, 1, 0]).is_ok(), "a lone symbol is well defined");
    }

    #[test]
    fn plaintext_is_not_mistaken_for_a_stream() {
        let plain = b"\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\x04\0stored resource directory bytes";
        assert!(decompress(plain, 4096).is_err());
    }

    #[test]
    fn a_64_bit_image_is_declined_by_name() {
        let mut image = fixture();
        let pe = Pe::parse(&image).unwrap();
        image[pe.opt_at] = 0x0b;
        image[pe.opt_at + 1] = 0x02; // PE32+
        match unpack(&image) {
            Err(NeoliteError::Unsupported(what)) => assert!(what.contains("magic")),
            other => panic!("expected an Unsupported, got {other:?}"),
        }
    }
}
