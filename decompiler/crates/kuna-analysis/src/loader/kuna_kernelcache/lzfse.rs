//! LZFSE decoding (Apple's `lzfse` reference format): the `bvx-` raw, `bvxn`
//! LZVN and `bvx2` FSE-coded blocks, ended by `bvx$`. Decode only, bounds-checked
//! throughout; any malformed field is an error rather than a guess.

const MAGIC_END: u32 = u32::from_le_bytes(*b"bvx$");
const MAGIC_RAW: u32 = u32::from_le_bytes(*b"bvx-");
const MAGIC_V1: u32 = u32::from_le_bytes(*b"bvx1");
const MAGIC_V2: u32 = u32::from_le_bytes(*b"bvx2");
const MAGIC_LZVN: u32 = u32::from_le_bytes(*b"bvxn");

const L_SYMBOLS: usize = 20;
const M_SYMBOLS: usize = 20;
const D_SYMBOLS: usize = 64;
const LITERAL_SYMBOLS: usize = 256;
const L_STATES: usize = 64;
const M_STATES: usize = 64;
const D_STATES: usize = 256;
const LITERAL_STATES: usize = 1024;
const MATCHES_PER_BLOCK: usize = 10_000;
const LITERALS_PER_BLOCK: usize = 4 * MATCHES_PER_BLOCK;
const V2_FIXED_HEADER: usize = 32;

const L_EXTRA_BITS: [u8; L_SYMBOLS] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 5, 8];
const L_BASE_VALUE: [u32; L_SYMBOLS] =
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 20, 28, 60];
const M_EXTRA_BITS: [u8; M_SYMBOLS] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 8, 11];
const M_BASE_VALUE: [u32; M_SYMBOLS] =
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 24, 56, 312];
const D_EXTRA_BITS: [u8; D_SYMBOLS] = [
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7,
    8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14,
    14, 14, 15, 15, 15, 15,
];
const D_BASE_VALUE: [u32; D_SYMBOLS] = [
    0, 1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24, 28, 36, 44, 52, 60, 76, 92, 108, 124, 156, 188, 220,
    252, 316, 380, 444, 508, 636, 764, 892, 1020, 1276, 1532, 1788, 2044, 2556, 3068, 3580, 4092,
    5116, 6140, 7164, 8188, 10236, 12284, 14332, 16380, 20476, 24572, 28668, 32764, 40956, 49148,
    57340, 65532, 81916, 98300, 114684, 131068, 163836, 196604, 229372,
];

pub(super) type Error = &'static str;

/// Whether `bytes` starts with an LZFSE block magic.
pub(super) fn is_lzfse(bytes: &[u8]) -> bool {
    rd_u32(bytes, 0).is_some_and(|m| {
        matches!(m, MAGIC_END | MAGIC_RAW | MAGIC_V1 | MAGIC_V2 | MAGIC_LZVN)
    })
}

/// Decode a whole LZFSE stream. `limit` caps the decoded size.
pub(super) fn decode(src: &[u8], size_hint: usize, limit: usize) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(size_hint.min(limit));
    let mut pos = 0usize;
    loop {
        let magic = rd_u32(src, pos).ok_or("LZFSE stream ends without an end-of-stream block")?;
        match magic {
            MAGIC_END => return Ok(out),
            MAGIC_RAW => {
                let n = rd_u32(src, pos + 4).ok_or(TRUNCATED)? as usize;
                reserve(&out, n, limit)?;
                let body = src.get(pos + 8..pos + 8 + n).ok_or(TRUNCATED)?;
                out.extend_from_slice(body);
                pos += 8 + n;
            }
            MAGIC_LZVN => {
                let n_raw = rd_u32(src, pos + 4).ok_or(TRUNCATED)? as usize;
                let n_payload = rd_u32(src, pos + 8).ok_or(TRUNCATED)? as usize;
                reserve(&out, n_raw, limit)?;
                let body = src.get(pos + 12..pos + 12 + n_payload).ok_or(TRUNCATED)?;
                lzvn_decode(body, &mut out, n_raw)?;
                pos += 12 + n_payload;
            }
            MAGIC_V2 => {
                let block = BlockV2::parse(src.get(pos..).ok_or(TRUNCATED)?)?;
                reserve(&out, block.n_raw_bytes, limit)?;
                let start = pos + block.header_size;
                let end = start + block.n_literal_payload_bytes + block.n_lmd_payload_bytes;
                let payload = src.get(start..end).ok_or(TRUNCATED)?;
                block.decode(payload, &mut out)?;
                pos = end;
            }
            MAGIC_V1 => return Err("LZFSE v1 (bvx1) blocks are not supported"),
            _ => return Err("bad LZFSE block magic"),
        }
    }
}

const TRUNCATED: Error = "truncated LZFSE block";

fn reserve(out: &[u8], n: usize, limit: usize) -> Result<(), Error> {
    match out.len().checked_add(n) {
        Some(total) if total <= limit => Ok(()),
        _ => Err("LZFSE stream decodes past its size limit"),
    }
}

struct BlockV2 {
    n_raw_bytes: usize,
    header_size: usize,
    n_literals: usize,
    n_matches: usize,
    n_literal_payload_bytes: usize,
    n_lmd_payload_bytes: usize,
    literal_bits: i32,
    lmd_bits: i32,
    literal_state: [u16; 4],
    l_state: u16,
    m_state: u16,
    d_state: u16,
    l_freq: [u16; L_SYMBOLS],
    m_freq: [u16; M_SYMBOLS],
    d_freq: [u16; D_SYMBOLS],
    literal_freq: [u16; LITERAL_SYMBOLS],
}

fn field(v: u64, offset: u32, nbits: u32) -> u64 {
    (v >> offset) & ((1u64 << nbits) - 1)
}

impl BlockV2 {
    fn parse(src: &[u8]) -> Result<Self, Error> {
        let n_raw_bytes = rd_u32(src, 4).ok_or(TRUNCATED)? as usize;
        let f0 = rd_u64(src, 8).ok_or(TRUNCATED)?;
        let f1 = rd_u64(src, 16).ok_or(TRUNCATED)?;
        let f2 = rd_u64(src, 24).ok_or(TRUNCATED)?;
        let header_size = field(f2, 0, 32) as usize;
        let mut block = BlockV2 {
            n_raw_bytes,
            header_size,
            n_literals: field(f0, 0, 20) as usize,
            n_literal_payload_bytes: field(f0, 20, 20) as usize,
            n_matches: field(f0, 40, 20) as usize,
            literal_bits: field(f0, 60, 3) as i32 - 7,
            literal_state: [
                field(f1, 0, 10) as u16,
                field(f1, 10, 10) as u16,
                field(f1, 20, 10) as u16,
                field(f1, 30, 10) as u16,
            ],
            n_lmd_payload_bytes: field(f1, 40, 20) as usize,
            lmd_bits: field(f1, 60, 3) as i32 - 7,
            l_state: field(f2, 32, 10) as u16,
            m_state: field(f2, 42, 10) as u16,
            d_state: field(f2, 52, 10) as u16,
            l_freq: [0; L_SYMBOLS],
            m_freq: [0; M_SYMBOLS],
            d_freq: [0; D_SYMBOLS],
            literal_freq: [0; LITERAL_SYMBOLS],
        };
        if header_size < V2_FIXED_HEADER {
            return Err("bad LZFSE v2 header size");
        }
        let packed = src.get(V2_FIXED_HEADER..header_size).ok_or(TRUNCATED)?;
        block.read_freqs(packed)?;
        if block.n_literals > LITERALS_PER_BLOCK
            || block.n_matches > MATCHES_PER_BLOCK
            || block.literal_state.iter().any(|&s| s as usize >= LITERAL_STATES)
            || block.l_state as usize >= L_STATES
            || block.m_state as usize >= M_STATES
            || block.d_state as usize >= D_STATES
        {
            return Err("LZFSE v2 header field out of range");
        }
        Ok(block)
    }

    /// The variable-length frequency tables: L, M, D, then literal frequencies.
    fn read_freqs(&mut self, packed: &[u8]) -> Result<(), Error> {
        if packed.is_empty() {
            return Ok(());
        }
        let mut accum = 0u32;
        let mut accum_nbits = 0u32;
        let mut src = packed.iter();
        let tables: [&mut [u16]; 4] =
            [&mut self.l_freq, &mut self.m_freq, &mut self.d_freq, &mut self.literal_freq];
        for table in tables {
            for slot in table.iter_mut() {
                while accum_nbits + 8 <= 32 {
                    let Some(&b) = src.next() else { break };
                    accum |= (b as u32) << accum_nbits;
                    accum_nbits += 8;
                }
                let (value, nbits) = freq_value(accum);
                if nbits > accum_nbits {
                    return Err("truncated LZFSE frequency table");
                }
                *slot = value;
                accum >>= nbits;
                accum_nbits -= nbits;
            }
        }
        if accum_nbits >= 8 || src.next().is_some() {
            return Err("LZFSE frequency table size mismatch");
        }
        Ok(())
    }

    fn decode(&self, payload: &[u8], out: &mut Vec<u8>) -> Result<(), Error> {
        let start = out.len();
        let literals = self.decode_literals(&payload[..self.n_literal_payload_bytes])?;
        let lmd = &payload[self.n_literal_payload_bytes..];
        let l_table = ValueTable::new(L_STATES, &self.l_freq, &L_EXTRA_BITS, &L_BASE_VALUE)?;
        let m_table = ValueTable::new(M_STATES, &self.m_freq, &M_EXTRA_BITS, &M_BASE_VALUE)?;
        let d_table = ValueTable::new(D_STATES, &self.d_freq, &D_EXTRA_BITS, &D_BASE_VALUE)?;
        let mut bits = BitIn::new(lmd, self.lmd_bits)?;
        let (mut l_state, mut m_state, mut d_state) =
            (self.l_state as usize, self.m_state as usize, self.d_state as usize);
        let mut lit = 0usize;
        let mut d = 0usize;
        for _ in 0..self.n_matches {
            bits.flush();
            let l = l_table.decode(&mut l_state, &mut bits)? as usize;
            let m = m_table.decode(&mut m_state, &mut bits)? as usize;
            let new_d = d_table.decode(&mut d_state, &mut bits)? as usize;
            if new_d != 0 {
                d = new_d;
            }
            let run = literals.get(lit..lit + l).ok_or("LZFSE literal run past its block")?;
            out.extend_from_slice(run);
            lit += l;
            copy_match(out, d, m)?;
            if out.len() - start > self.n_raw_bytes {
                return Err("LZFSE block decodes past its declared size");
            }
        }
        if out.len() - start != self.n_raw_bytes {
            return Err("LZFSE block size mismatch");
        }
        Ok(())
    }

    fn decode_literals(&self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let table = SymbolTable::new(LITERAL_STATES, &self.literal_freq)?;
        let mut bits = BitIn::new(payload, self.literal_bits)?;
        let mut states = self.literal_state.map(|s| s as usize);
        let mut literals = Vec::with_capacity(self.n_literals + 4);
        while literals.len() < self.n_literals {
            bits.flush();
            for state in states.iter_mut() {
                literals.push(table.decode(state, &mut bits)?);
            }
        }
        Ok(literals)
    }
}

fn copy_match(out: &mut Vec<u8>, d: usize, m: usize) -> Result<(), Error> {
    if m == 0 {
        return Ok(());
    }
    if d == 0 || d > out.len() {
        return Err("LZ match distance outside the decoded data");
    }
    let from = out.len() - d;
    if d >= m {
        out.extend_from_within(from..from + m);
    } else {
        for k in 0..m {
            let b = out[from + k];
            out.push(b);
        }
    }
    Ok(())
}

fn freq_value(bits: u32) -> (u16, u32) {
    const NBITS: [u8; 32] = [
        2, 3, 2, 5, 2, 3, 2, 8, 2, 3, 2, 5, 2, 3, 2, 14, 2, 3, 2, 5, 2, 3, 2, 8, 2, 3, 2, 5, 2, 3,
        2, 14,
    ];
    const VALUE: [u8; 32] = [
        0, 2, 1, 4, 0, 3, 1, 0, 0, 2, 1, 5, 0, 3, 1, 0, 0, 2, 1, 6, 0, 3, 1, 0, 0, 2, 1, 7, 0, 3, 1,
        0,
    ];
    let b = (bits & 31) as usize;
    let n = NBITS[b] as u32;
    let value = match n {
        8 => 8 + ((bits >> 4) & 0xf) as u16,
        14 => 24 + ((bits >> 4) & 0x3ff) as u16,
        _ => VALUE[b] as u16,
    };
    (value, n)
}

/// One `(symbol, k, delta)` decoder entry per state, laid out the way the
/// encoder spreads states over symbols in frequency order.
fn spread(nstates: usize, freq: &[u16], mut f_entry: impl FnMut(usize, u32, i32)) -> Result<(), Error> {
    let n_clz = (nstates as u32).leading_zeros();
    let mut total = 0usize;
    for (symbol, &f) in freq.iter().enumerate() {
        if f == 0 {
            continue;
        }
        let f = f as u32;
        total += f as usize;
        if total > nstates {
            return Err("LZFSE frequencies exceed the state count");
        }
        let k = f.leading_zeros() - n_clz;
        let j0 = ((2 * nstates as u32) >> k) - f;
        for j in 0..f {
            if j < j0 {
                f_entry(symbol, k, (((f + j) << k) as i32) - nstates as i32);
            } else {
                f_entry(symbol, k - 1, ((j - j0) << (k - 1)) as i32);
            }
        }
    }
    Ok(())
}

struct SymbolTable(Vec<(u8, u8, i32)>);

impl SymbolTable {
    fn new(nstates: usize, freq: &[u16]) -> Result<Self, Error> {
        let mut entries = Vec::with_capacity(nstates);
        spread(nstates, freq, |symbol, k, delta| entries.push((symbol as u8, k as u8, delta)))?;
        entries.resize(nstates, (0, 0, 0));
        Ok(Self(entries))
    }

    fn decode(&self, state: &mut usize, bits: &mut BitIn) -> Result<u8, Error> {
        let &(symbol, k, delta) = self.0.get(*state).ok_or(BAD_STATE)?;
        *state = (delta + bits.pull(k as u32)? as i32) as usize;
        Ok(symbol)
    }
}

const BAD_STATE: Error = "LZFSE decoder state out of range";

struct ValueEntry {
    total_bits: u8,
    value_bits: u8,
    delta: i32,
    vbase: u32,
}

struct ValueTable(Vec<ValueEntry>);

impl ValueTable {
    fn new(nstates: usize, freq: &[u16], vbits: &[u8], vbase: &[u32]) -> Result<Self, Error> {
        let mut entries = Vec::with_capacity(nstates);
        spread(nstates, freq, |symbol, k, delta| {
            entries.push(ValueEntry {
                total_bits: k as u8 + vbits[symbol],
                value_bits: vbits[symbol],
                delta,
                vbase: vbase[symbol],
            })
        })?;
        entries.resize_with(nstates, || ValueEntry { total_bits: 0, value_bits: 0, delta: 0, vbase: 0 });
        Ok(Self(entries))
    }

    fn decode(&self, state: &mut usize, bits: &mut BitIn) -> Result<u32, Error> {
        let e = self.0.get(*state).ok_or(BAD_STATE)?;
        let raw = bits.pull(e.total_bits as u32)?;
        *state = (e.delta + (raw >> e.value_bits) as i32) as usize;
        Ok(e.vbase + (raw & ((1u64 << e.value_bits) - 1)) as u32)
    }
}

/// The FSE bit stream, read backwards from the end of its payload.
struct BitIn<'a> {
    buf: &'a [u8],
    pos: usize,
    accum: u64,
    nbits: u32,
}

impl<'a> BitIn<'a> {
    fn new(buf: &'a [u8], n: i32) -> Result<Self, Error> {
        if !(-7..=0).contains(&n) {
            return Err("bad LZFSE bit-stream header");
        }
        let width = if n != 0 { 8 } else { 7 };
        let pos = buf.len().saturating_sub(width);
        let accum = le_bytes(&buf[pos..]) << (8 * (width - (buf.len() - pos)));
        let nbits = (n + 8 * width as i32) as u32;
        if nbits < 64 && accum >> nbits != 0 {
            return Err("bad LZFSE bit-stream header");
        }
        Ok(Self { buf, pos, accum, nbits })
    }

    /// Refill to at least 56 bits. Near the start of the payload the encoder's
    /// stream may refill from bytes before it, which are never consumed; they
    /// read as zero here.
    fn flush(&mut self) {
        let nbytes = ((63 - self.nbits) >> 3) as usize;
        if nbytes == 0 {
            return;
        }
        let from = self.pos.saturating_sub(nbytes);
        let missing = nbytes - (self.pos - from);
        let incoming = le_bytes(&self.buf[from..self.pos]) << (8 * missing);
        self.pos = from;
        self.accum = (self.accum << (8 * nbytes)) | incoming;
        self.nbits += 8 * nbytes as u32;
    }

    fn pull(&mut self, n: u32) -> Result<u64, Error> {
        if n > self.nbits {
            return Err(TRUNCATED);
        }
        self.nbits -= n;
        let result = self.accum >> self.nbits;
        self.accum &= (1u64 << self.nbits) - 1;
        Ok(result)
    }
}

fn le_bytes(b: &[u8]) -> u64 {
    b.iter().rev().fold(0u64, |acc, &x| (acc << 8) | x as u64)
}

/// Decode one LZVN block of `n_raw` bytes, appending to `out` (match distances
/// may reach into earlier blocks).
fn lzvn_decode(src: &[u8], out: &mut Vec<u8>, n_raw: usize) -> Result<(), Error> {
    const TRUNC: Error = "truncated LZVN block";
    let end = out.len() + n_raw;
    let byte = |i: usize| src.get(i).copied().map(usize::from).ok_or(TRUNC);
    let mut i = 0usize;
    let mut d = 0usize;
    loop {
        let op = byte(i)?;
        let (len, l, m) = match op {
            0x06 => break,
            0x0e | 0x16 => (1, 0, 0),
            0x1e | 0x26 | 0x2e | 0x36 | 0x3e | 0x70..=0x7f | 0xd0..=0xdf => {
                return Err("undefined LZVN opcode")
            }
            0xa0..=0xbf => {
                let (b1, b2) = (byte(i + 1)?, byte(i + 2)?);
                d = (b1 >> 2) | (b2 << 6);
                (3, (op >> 3) & 3, (((op & 7) << 2) | (b1 & 3)) + 3)
            }
            0xe0 => (2, byte(i + 1)? + 16, 0),
            0xe1..=0xef => (1, op & 0xf, 0),
            0xf0 => (2, 0, byte(i + 1)? + 16),
            0xf1..=0xff => (1, 0, op & 0xf),
            _ if op & 7 == 7 => {
                d = byte(i + 1)? | (byte(i + 2)? << 8);
                (3, op >> 6, ((op >> 3) & 7) + 3)
            }
            _ if op & 7 == 6 => (1, op >> 6, ((op >> 3) & 7) + 3),
            _ => {
                d = ((op & 7) << 8) | byte(i + 1)?;
                (2, op >> 6, ((op >> 3) & 7) + 3)
            }
        };
        i += len;
        let run = src.get(i..i + l).ok_or(TRUNC)?;
        out.extend_from_slice(run);
        i += l;
        copy_match(out, d, m)?;
        if out.len() > end {
            return Err("LZVN block decodes past its declared size");
        }
    }
    if out.len() != end {
        return Err("LZVN block size mismatch");
    }
    Ok(())
}

fn rd_u32(b: &[u8], off: usize) -> Option<u32> {
    b.get(off..off.checked_add(4)?).map(|s| u32::from_le_bytes(s.try_into().unwrap()))
}

fn rd_u64(b: &[u8], off: usize) -> Option<u64> {
    b.get(off..off.checked_add(8)?).map(|s| u64::from_le_bytes(s.try_into().unwrap()))
}
