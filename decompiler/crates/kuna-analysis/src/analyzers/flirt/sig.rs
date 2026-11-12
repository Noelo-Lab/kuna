//! Reader for IDA FLIRT `.sig` files, versions 5 to 11.
//!
//! Layout as IDA 9.5 reads it (`libida.so` header 0x8b60e0, tree 0x8b4880,
//! names 0x8b4110 / 0x8b3c10, masks 0x8b4530): a little-endian header, then a
//! body (raw, deflate, zlib or zstd) holding a byte trie whose leaves list the
//! modules that share a prefix.

use std::io::Read;

/// One public name a module defines, at `off` bytes from the module start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Public {
    pub off: i64,
    pub flags: u8,
    pub name: String,
    /// Expected instruction bits at `off` (`(word ^ value) & mask == 0`).
    pub bits: Option<(u32, u32)>,
}

/// A v11 reference / relocation site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub off: u32,
    pub name: String,
    pub flags: u8,
    pub bits: Option<(u32, u32)>,
}

/// One library module: the CRC over the bytes after the trie prefix, the tail
/// bytes, the names it defines and the name it references.
#[derive(Debug, Clone)]
pub struct Module {
    pub crc_len: u8,
    pub crc: u16,
    pub length: u32,
    pub publics: Vec<Public>,
    /// `(offset after the CRC region, byte)`.
    pub tails: Vec<(u32, u8)>,
    /// `(offset from the module start, name)`.
    pub reference: Option<(i64, String)>,
    pub sites: Vec<Site>,
}

/// A trie node: `pattern` bytes (`None` = any byte) under the parent's, then
/// either children or the modules of a leaf.
#[derive(Debug, Clone, Default)]
pub struct Node {
    pub pattern: Vec<Option<u8>>,
    pub children: Vec<Node>,
    pub modules: Vec<Module>,
}

/// A parsed signature file.
#[derive(Debug, Clone)]
pub struct Sig {
    pub version: u8,
    pub library: String,
    /// The IDA file types (`FT_*` bits) the library applies to; 0 when unset.
    pub file_formats: u32,
    /// A startup signature (`LS_STARTUP`): its module names are IDA scripts
    /// that pick runtime signatures, not function names.
    pub startup: bool,
    pub pattern_len: usize,
    pub root: Node,
}

impl Sig {
    /// Every module in the file.
    pub fn modules(&self) -> Vec<&Module> {
        fn walk<'a>(n: &'a Node, out: &mut Vec<&'a Module>) {
            out.extend(n.modules.iter());
            n.children.iter().for_each(|c| walk(c, out));
        }
        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }
}

const LS_STARTUP: u16 = 0x01;
const LS_ZIP: u16 = 0x10;
const LS_ZSTD: u16 = 0x40;

/// Whether a `.sig` file's header admits a binary of IDA file type `ft`
/// (`FT_*` bit index): not a startup signature, and the file-type mask is unset
/// or names `ft`.
pub fn applies_to(data: &[u8], ft: Option<u32>) -> bool {
    if data.len() < 18 || &data[..6] != b"IDASGN" {
        return false;
    }
    let formats = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let flags = u16::from_le_bytes([data[16], data[17]]);
    flags & LS_STARTUP == 0 && (formats == 0 || ft.is_none_or(|b| formats >> b & 1 == 1))
}

/// Parse a `.sig` file.
pub fn parse(data: &[u8]) -> Result<Sig, String> {
    if data.len() < 41 || &data[..6] != b"IDASGN" {
        return Err("not an IDA signature file".into());
    }
    let version = data[6];
    if !(5..=11).contains(&version) {
        return Err(format!("unsupported signature version {version}"));
    }
    let le16 = |p: usize| u16::from_le_bytes([data[p], data[p + 1]]);
    let flags = le16(16);
    let file_formats = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let name_len = data[34] as usize;
    let mut p = 37;
    if version >= 6 {
        p += 4;
    }
    let mut pattern_len = 32;
    if version >= 8 {
        pattern_len = le16(p) as usize;
        p += 2;
    }
    if version >= 10 {
        p += 2;
    }
    if version >= 11 {
        p += 1;
    }
    let library = String::from_utf8_lossy(data.get(p..p + name_len).ok_or("truncated header")?).into_owned();
    let rest = &data[p + name_len..];
    let body = if flags & LS_ZSTD != 0 {
        let mut out = Vec::new();
        ruzstd::decoding::StreamingDecoder::new(rest)
            .map_err(|e| e.to_string())?
            .read_to_end(&mut out)
            .map_err(|e| e.to_string())?;
        out
    } else if flags & LS_ZIP != 0 {
        let mut out = Vec::new();
        if version >= 7 {
            flate2::read::ZlibDecoder::new(rest).read_to_end(&mut out).map_err(|e| e.to_string())?;
        } else {
            flate2::read::DeflateDecoder::new(rest).read_to_end(&mut out).map_err(|e| e.to_string())?;
        }
        out
    } else {
        rest.to_vec()
    };
    let mut r = Reader { d: &body, p: 0, version };
    let root = r.tree(Vec::new())?;
    if r.p != body.len() {
        return Err(format!("trailing bytes at {} of {}", r.p, body.len()));
    }
    Ok(Sig { version, library, file_formats, startup: flags & LS_STARTUP != 0, pattern_len, root })
}

struct Reader<'a> {
    d: &'a [u8],
    p: usize,
    version: u8,
}

impl Reader<'_> {
    fn b(&mut self) -> Result<u8, String> {
        let v = *self.d.get(self.p).ok_or("truncated body")?;
        self.p += 1;
        Ok(v)
    }
    fn be16(&mut self) -> Result<u16, String> {
        Ok(u16::from(self.b()?) << 8 | u16::from(self.b()?))
    }
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let s = self.d.get(self.p..self.p + n).ok_or("truncated body")?;
        self.p += n;
        Ok(s)
    }
    fn m2(&mut self) -> Result<u32, String> {
        let b = self.b()?;
        Ok(if b & 0x80 != 0 { u32::from(b & 0x7f) << 8 | u32::from(self.b()?) } else { u32::from(b) })
    }
    fn mb(&mut self) -> Result<u32, String> {
        let b = self.b()?;
        if b & 0x80 == 0 {
            return Ok(u32::from(b));
        }
        if b & 0xc0 != 0xc0 {
            return Ok(u32::from(b & 0x7f) << 8 | u32::from(self.b()?));
        }
        if b & 0xe0 != 0xe0 {
            let hi = u32::from(b & 0x3f) << 24 | u32::from(self.b()?) << 16;
            return Ok(hi | u32::from(self.be16()?));
        }
        let s = self.take(4)?;
        Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn offset(&mut self) -> Result<u32, String> {
        if self.version >= 9 { self.mb() } else { self.m2() }
    }
    fn mask(&mut self, n: usize) -> Result<Vec<bool>, String> {
        let bits: u128 = if n < 0x10 {
            u128::from(self.m2()?)
        } else if n <= 0x20 {
            u128::from(self.mb()?)
        } else if n <= 0x40 {
            u128::from(self.mb()?) << 32 | u128::from(self.mb()?)
        } else {
            let nb = n >> 3;
            let mut v: Vec<bool> = Vec::with_capacity(n);
            let raw = self.take(nb)?.to_vec();
            let extra = if n & 7 != 0 { Some(self.b()?) } else { None };
            let bit = |i: usize| -> bool {
                if i < nb * 8 {
                    raw[i / 8] >> (i % 8) & 1 == 1
                } else {
                    extra.is_some_and(|e| e >> (i - nb * 8) & 1 == 1)
                }
            };
            for i in 0..n {
                v.push(bit(n - 1 - i));
            }
            return Ok(v);
        };
        Ok((0..n).map(|i| bits >> (n - 1 - i) & 1 == 1).collect())
    }
    fn tree(&mut self, pattern: Vec<Option<u8>>) -> Result<Node, String> {
        let n = self.m2()?;
        let mut node = Node { pattern, ..Node::default() };
        if n == 0 {
            node.modules = self.leaf()?;
            return Ok(node);
        }
        for _ in 0..n {
            let len = if self.version >= 10 { self.m2()? } else { u32::from(self.b()?) } as usize;
            let wild = self.mask(len)?;
            let mut pat = Vec::with_capacity(len);
            for w in wild {
                pat.push(if w { None } else { Some(self.b()?) });
            }
            node.children.push(self.tree(pat)?);
        }
        Ok(node)
    }
    fn public(&mut self) -> Result<(u8, String, Option<(u32, u32)>, u8), String> {
        if self.version >= 11 {
            let n = self.be16()? as usize;
            let fl = self.b()?;
            let name = String::from_utf8_lossy(self.take(n)?).into_owned();
            let bits = if fl & 0x80 != 0 { Some((self.mb()?, self.mb()?)) } else { None };
            if fl & 0x40 != 0 {
                self.mb()?;
                self.be16()?;
                self.b()?;
            }
            return Ok((fl & 0xbf, name, bits, self.b()?));
        }
        let mut c = self.b()?;
        let mut fl = 0;
        if c < 0x20 {
            fl = c;
            c = self.b()?;
        }
        let mut name = Vec::new();
        while c >= 0x20 {
            name.push(c);
            c = self.b()?;
        }
        Ok((fl, String::from_utf8_lossy(&name).into_owned(), None, c))
    }
    fn site(&mut self) -> Result<Site, String> {
        let off = self.mb()?;
        let n = self.be16()? as usize;
        let name = String::from_utf8_lossy(self.take(n)?).into_owned();
        let flags = self.b()?;
        let bits = if flags & 0x80 != 0 { Some((self.mb()?, self.mb()?)) } else { None };
        if flags & 0x40 != 0 {
            self.mb()?;
            self.be16()?;
            self.b()?;
        }
        if flags & 0x08 != 0 {
            self.mb()?;
        }
        Ok(Site { off, name, flags, bits })
    }
    fn leaf(&mut self) -> Result<Vec<Module>, String> {
        let mut out = Vec::new();
        loop {
            let crc_len = self.b()?;
            let crc = self.be16()?;
            let more_groups = loop {
                let length = self.offset()?;
                let mut publics = Vec::new();
                let mut off: i64 = 0;
                let pf = loop {
                    let mut d = i64::from(self.offset()?);
                    let (fl, name, bits, pf) = self.public()?;
                    if self.version >= 11 && fl & 0x10 != 0 {
                        d = -d;
                    }
                    off += d;
                    publics.push(Public { off, flags: fl, name, bits });
                    if pf & 1 == 0 {
                        break pf;
                    }
                };
                let mut tails = Vec::new();
                if pf & 2 != 0 {
                    let n = if self.version >= 8 { self.b()? } else { 1 };
                    for _ in 0..n {
                        let o = self.offset()?;
                        tails.push((o, self.b()?));
                    }
                }
                let mut reference = None;
                if pf & 4 != 0 {
                    if self.version >= 8 {
                        self.b()?;
                    }
                    let o = if self.version >= 9 { self.mb()? } else { self.m2()? };
                    let mut n = u32::from(self.b()?);
                    if n == 0 {
                        n = self.m2()?;
                    }
                    let raw = self.take(n as usize)?;
                    let negative = raw.last() == Some(&0);
                    let name = String::from_utf8_lossy(raw).trim_end_matches('\0').to_string();
                    reference = Some((if negative { -i64::from(o) } else { i64::from(o) }, name));
                }
                let mut sites = Vec::new();
                if self.version >= 11 && pf & 0x20 != 0 {
                    for _ in 0..self.mb()? {
                        sites.push(self.site()?);
                    }
                }
                if self.version >= 11 && pf & 0x40 != 0 {
                    for _ in 0..self.mb()? {
                        for _ in 0..self.mb()? {
                            self.site()?;
                        }
                    }
                }
                out.push(Module { crc_len, crc, length, publics, tails, reference, sites });
                if pf & 8 == 0 {
                    break pf & 0x10 != 0;
                }
            };
            if !more_groups {
                break;
            }
        }
        Ok(out)
    }
}

/// CRC-16/X.25 with the result byte-swapped, as FLIRT stores it.
pub fn crc16(data: &[u8]) -> u16 {
    let mut c: u16 = 0xffff;
    for &b in data {
        c ^= u16::from(b);
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0x8408 } else { c >> 1 };
        }
    }
    (!c).swap_bytes()
}
