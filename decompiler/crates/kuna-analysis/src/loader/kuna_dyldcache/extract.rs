//! Rebuild one cache image as a standalone thin Mach-O.
//!
//! Every segment keeps its cache virtual address, so an address kuna prints is
//! the address in the cache. The segments' bytes are copied with slide info
//! applied, and the image gets a private `__LINKEDIT`: the cache shares one
//! `__LINKEDIT` across all images, and the load commands point into it.
//!
//! The private `__LINKEDIT` holds the image's own nlists, with the local symbols
//! the cache moved to its local-symbols file in place of the `<redacted>`
//! placeholders left behind. It also holds a compact string table, the
//! indirect-symbol table renumbered to match, copies of the export trie,
//! function starts and data-in-code, and a `kuna.dyld.xrefs` `LC_NOTE` naming
//! the image's references into other images ([`super::xrefs`]). Linkedit data
//! the rebuild does not carry (code signature, split-seg info, rebase and bind
//! opcodes the cache builder already applied) is zeroed in its command.

use std::rc::Rc;

use super::{u32_at, u64_at, DyldCache};

const MH_MAGIC_64: u32 = 0xfeed_facf;
const CPU_TYPE_ARM64: u32 = 0x0100_000c;
const CPU_TYPE_X86_64: u32 = 0x0100_0007;
const LC_SEGMENT_64: u32 = 0x19;
const LC_SYMTAB: u32 = 0x2;
const LC_DYSYMTAB: u32 = 0xb;
const LC_DYLD_INFO: u32 = 0x22;
const LC_DYLD_INFO_ONLY: u32 = 0x8000_0022;
const LC_FUNCTION_STARTS: u32 = 0x26;
const LC_DATA_IN_CODE: u32 = 0x29;
const LC_DYLD_EXPORTS_TRIE: u32 = 0x8000_0033;
pub(crate) const LC_NOTE: u32 = 0x31;
const LINKEDIT_DATA_COMMANDS: &[u32] =
    &[0x1d, 0x1e, LC_FUNCTION_STARTS, LC_DATA_IN_CODE, 0x2b, 0x2e, LC_DYLD_EXPORTS_TRIE, 0x8000_0034, 0x36, 0x37, 0x38];
const INDIRECT_SYMBOL_LOCAL: u32 = 0x8000_0000;
const INDIRECT_SYMBOL_ABS: u32 = 0x4000_0000;
const N_STAB: u8 = 0xe0;
const N_TYPE: u8 = 0x0e;
const N_SECT: u8 = 0x0e;
const N_EXT: u8 = 0x01;
const PAGE: u64 = 0x4000;

/// A section header inside a segment command.
#[derive(Clone, Debug)]
pub struct Section {
    pub sectname: String,
    pub addr: u64,
    pub size: u64,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    /// Byte offset of this section header within the load commands.
    cmd_offset: usize,
}

impl Section {
    pub fn section_type(&self) -> u32 {
        self.flags & 0xff
    }

    pub fn is_code(&self) -> bool {
        self.flags & 0x8000_0400 != 0
    }

    fn is_zerofill(&self) -> bool {
        matches!(self.section_type(), 0x1 | 0xc | 0x12)
    }
}

/// A segment command of a cache image.
#[derive(Clone, Debug)]
pub struct Segment {
    pub name: String,
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub sections: Vec<Section>,
    cmd_offset: usize,
}

impl Segment {
    pub fn contains(&self, va: u64) -> bool {
        va >= self.vmaddr && va < self.vmaddr + self.vmsize
    }
}

/// An image's Mach-O header and load commands, read from the cache.
#[derive(Debug)]
pub struct ImageLayout {
    pub address: u64,
    header: Vec<u8>,
    cmds: Vec<u8>,
    pub segments: Vec<Segment>,
    symtab: Option<usize>,
    dysymtab: Option<usize>,
}

impl ImageLayout {
    fn linkedit(&self) -> Option<&Segment> {
        self.segments.iter().find(|s| s.name == "__LINKEDIT")
    }

    /// The address a `__LINKEDIT` file offset of this image names.
    fn linkedit_va(&self, offset: u32) -> Option<u64> {
        let le = self.linkedit()?;
        (offset as u64).checked_sub(le.fileoff).map(|d| le.vmaddr + d)
    }

    fn cmd_u32(&self, at: usize) -> u32 {
        u32_at(&self.cmds, at)
    }

    /// The commands, as `(cmd, offset)` pairs.
    fn commands(&self) -> Vec<(u32, usize)> {
        let mut out = Vec::new();
        let mut at = 0;
        while at + 8 <= self.cmds.len() {
            let size = u32_at(&self.cmds, at + 4) as usize;
            out.push((u32_at(&self.cmds, at), at));
            if size < 8 {
                break;
            }
            at += size;
        }
        out
    }

    /// Does any segment other than `__LINKEDIT` hold `va`?
    pub fn holds(&self, va: u64) -> bool {
        self.segments.iter().any(|s| s.name != "__LINKEDIT" && s.contains(va))
    }
}

fn parse_layout(address: u64, header: Vec<u8>, cmds: Vec<u8>) -> ImageLayout {
    let mut layout = ImageLayout { address, header, cmds, segments: Vec::new(), symtab: None, dysymtab: None };
    for (cmd, at) in layout.commands() {
        let c = &layout.cmds[at..];
        match cmd {
            LC_SEGMENT_64 if c.len() >= 72 => {
                let nsects = u32_at(c, 64) as usize;
                let sections = (0..nsects)
                    .map(|i| 72 + i * 80)
                    .take_while(|&s| s + 80 <= c.len())
                    .map(|s| Section {
                        sectname: fixed_name(&c[s..s + 16]),
                        addr: u64_at(c, s + 32),
                        size: u64_at(c, s + 40),
                        flags: u32_at(c, s + 64),
                        reserved1: u32_at(c, s + 68),
                        reserved2: u32_at(c, s + 72),
                        cmd_offset: at + s,
                    })
                    .collect();
                layout.segments.push(Segment {
                    name: fixed_name(&c[8..24]),
                    vmaddr: u64_at(c, 24),
                    vmsize: u64_at(c, 32),
                    fileoff: u64_at(c, 40),
                    filesize: u64_at(c, 48),
                    sections,
                    cmd_offset: at,
                });
            }
            LC_SYMTAB => layout.symtab = Some(at),
            LC_DYSYMTAB => layout.dysymtab = Some(at),
            _ => {}
        }
    }
    layout
}

fn fixed_name(raw: &[u8]) -> String {
    String::from_utf8_lossy(&raw[..raw.iter().position(|&b| b == 0).unwrap_or(raw.len())]).into_owned()
}

/// One symbol-table entry with its name resolved.
#[derive(Clone, Debug)]
pub struct Nlist {
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
    pub name: Vec<u8>,
}

impl Nlist {
    pub(crate) fn parse_all(raw: &[u8], pool: &super::StringPool) -> Vec<Nlist> {
        raw.chunks_exact(16)
            .map(|n| Nlist {
                n_type: n[4],
                n_sect: n[5],
                n_desc: u16::from_le_bytes([n[6], n[7]]),
                n_value: u64_at(n, 8),
                name: pool.get(u32_at(n, 0)).to_vec(),
            })
            .collect()
    }

    /// A defined, named, non-debug symbol: one that names an address.
    pub fn names_address(&self) -> bool {
        self.n_type & N_STAB == 0
            && self.n_type & N_TYPE == N_SECT
            && !self.name.is_empty()
            && self.name != b"<redacted>"
    }

    pub fn is_external(&self) -> bool {
        self.n_type & N_EXT != 0
    }
}

impl DyldCache {
    /// The header and load commands of image `index`, read once.
    pub fn image_layout(&self, index: usize) -> Result<Rc<ImageLayout>, String> {
        if let Some(cached) = self.layouts.borrow().get(&index) {
            return cached.clone().ok_or_else(|| format!("image {index} has no readable Mach-O header"));
        }
        let read = || -> Result<ImageLayout, String> {
            let address = self.images.get(index).ok_or("image index out of range")?.address;
            let header = self.read_va(address, 32)?;
            if u32_at(&header, 0) != MH_MAGIC_64 {
                return Err(format!("image at {address:#x} is not a 64-bit Mach-O"));
            }
            let cmds = self.read_va(address + 32, u32_at(&header, 20) as usize)?;
            Ok(parse_layout(address, header, cmds))
        };
        let layout = read().map(Rc::new);
        self.layouts.borrow_mut().insert(index, layout.as_ref().ok().cloned());
        layout
    }

    /// The nlists of image `index`'s own symbol table, names resolved.
    pub(crate) fn image_symbols(&self, index: usize) -> Result<Vec<Nlist>, String> {
        let layout = self.image_layout(index)?;
        let Some(at) = layout.symtab else {
            return Ok(Vec::new());
        };
        let (symoff, nsyms) = (layout.cmd_u32(at + 8), layout.cmd_u32(at + 12) as usize);
        let (stroff, strsize) = (layout.cmd_u32(at + 16), layout.cmd_u32(at + 20) as u64);
        if nsyms == 0 {
            return Ok(Vec::new());
        }
        let sym_va = layout.linkedit_va(symoff).ok_or("symbol table lies outside __LINKEDIT")?;
        let str_va = layout.linkedit_va(stroff).ok_or("string table lies outside __LINKEDIT")?;
        let raw = self.read_va(sym_va, nsyms * 16)?;
        let pool = super::StringPool::read(&|off, len| self.read_va(str_va + off, len), strsize, &raw)?;
        Ok(Nlist::parse_all(&raw, &pool))
    }

    /// Image `index`'s indirect-symbol table, as read from the cache.
    pub(crate) fn image_indirect_symbols(&self, index: usize) -> Result<Vec<u32>, String> {
        let layout = self.image_layout(index)?;
        let Some(at) = layout.dysymtab else {
            return Ok(Vec::new());
        };
        let count = layout.cmd_u32(at + 60) as usize;
        if count == 0 {
            return Ok(Vec::new());
        }
        let va = layout
            .linkedit_va(layout.cmd_u32(at + 56))
            .ok_or("indirect-symbol table lies outside __LINKEDIT")?;
        Ok(self.read_va(va, count * 4)?.chunks_exact(4).map(|c| u32_at(c, 0)).collect())
    }

    /// Rebuild image `index` as a standalone thin Mach-O.
    pub fn extract(&self, index: usize) -> Result<Vec<u8>, String> {
        let layout = self.image_layout(index)?;
        let cputype = u32_at(&layout.header, 4);
        if cputype != CPU_TYPE_ARM64 && cputype != CPU_TYPE_X86_64 {
            return Err(format!("dyld cache image cputype {cputype:#x} is not arm64 or x86_64"));
        }
        layout.linkedit().ok_or("image has no __LINKEDIT segment")?;

        let mut out = Vec::new();
        let mut placed: Vec<(usize, u64)> = Vec::new();
        let text = layout.segments.iter().position(|s| s.vmaddr == layout.address).unwrap_or(0);
        let order = std::iter::once(text).chain((0..layout.segments.len()).filter(|&i| i != text));
        for i in order {
            let seg = &layout.segments[i];
            if seg.name == "__LINKEDIT" {
                continue;
            }
            let offset = if i == text { 0 } else { align(out.len() as u64, PAGE) };
            out.resize(offset as usize, 0);
            if seg.filesize > 0 {
                out.extend(self.read_slid(seg.vmaddr, seg.filesize as usize)?);
            }
            placed.push((i, offset));
        }
        let file_offset_of = |i: usize| placed.iter().find(|p| p.0 == i).map(|p| p.1);

        let xrefs = {
            let view = |va: u64, len: usize| -> Option<&[u8]> {
                let (i, off) = placed.iter().find(|(i, _)| {
                    let s = &layout.segments[*i];
                    va >= s.vmaddr && va + len as u64 <= s.vmaddr + s.filesize
                })?;
                let start = (off + (va - layout.segments[*i].vmaddr)) as usize;
                out.get(start..start + len)
            };
            super::xrefs::collect(self, &layout, &view)
        };

        let linkedit = self.build_linkedit(index, &layout, &xrefs)?;
        let le_offset = align(out.len() as u64, PAGE);
        out.resize(le_offset as usize, 0);
        out.extend(&linkedit.bytes);

        let mut cmds = layout.cmds.clone();
        for (seg_index, seg) in layout.segments.iter().enumerate() {
            let at = seg.cmd_offset;
            if seg.name == "__LINKEDIT" {
                put_u64(&mut cmds, at + 32, align(linkedit.bytes.len() as u64, PAGE));
                put_u64(&mut cmds, at + 40, le_offset);
                put_u64(&mut cmds, at + 48, linkedit.bytes.len() as u64);
                continue;
            }
            let base = file_offset_of(seg_index).unwrap_or(0);
            put_u64(&mut cmds, at + 40, base);
            for sec in &seg.sections {
                let offset = if sec.is_zerofill() || sec.size == 0 { 0 } else { base + (sec.addr - seg.vmaddr) };
                put_u32(&mut cmds, sec.cmd_offset + 48, offset as u32);
                put_u32(&mut cmds, sec.cmd_offset + 56, 0);
                put_u32(&mut cmds, sec.cmd_offset + 60, 0);
            }
        }
        let rel = |off: u64| (le_offset + off) as u32;
        for (cmd, at) in layout.commands() {
            match cmd {
                LC_SYMTAB => {
                    put_u32(&mut cmds, at + 8, rel(linkedit.symoff));
                    put_u32(&mut cmds, at + 12, linkedit.nsyms);
                    put_u32(&mut cmds, at + 16, rel(linkedit.stroff));
                    put_u32(&mut cmds, at + 20, linkedit.strsize);
                }
                LC_DYSYMTAB => {
                    let [ilocal, nlocal, iextdef, nextdef, iundef, nundef] = linkedit.dysymtab;
                    for (field, value) in [ilocal, nlocal, iextdef, nextdef, iundef, nundef].into_iter().enumerate() {
                        put_u32(&mut cmds, at + 8 + field * 4, value);
                    }
                    for field in (32..80).step_by(4) {
                        put_u32(&mut cmds, at + field, 0);
                    }
                    if linkedit.nindirect > 0 {
                        put_u32(&mut cmds, at + 56, rel(linkedit.indirectoff));
                        put_u32(&mut cmds, at + 60, linkedit.nindirect);
                    }
                }
                LC_DYLD_INFO | LC_DYLD_INFO_ONLY => {
                    for field in (8..48).step_by(4) {
                        put_u32(&mut cmds, at + field, 0);
                    }
                    if let Some((off, size)) = linkedit.copied(LC_DYLD_EXPORTS_TRIE) {
                        put_u32(&mut cmds, at + 40, rel(off));
                        put_u32(&mut cmds, at + 44, size);
                    }
                }
                c if LINKEDIT_DATA_COMMANDS.contains(&c) => {
                    let (off, size) = linkedit.copied(c).map(|(o, s)| (rel(o), s)).unwrap_or((0, 0));
                    put_u32(&mut cmds, at + 8, off);
                    put_u32(&mut cmds, at + 12, size);
                }
                _ => {}
            }
        }
        let mut ncmds = u32_at(&layout.header, 16);
        if let Some((off, size)) = linkedit.note {
            let mut note = Vec::with_capacity(40);
            note.extend(LC_NOTE.to_le_bytes());
            note.extend(40u32.to_le_bytes());
            note.extend(super::xrefs::NOTE_OWNER);
            note.extend((le_offset + off).to_le_bytes());
            note.extend(size.to_le_bytes());
            cmds.extend(note);
            ncmds += 1;
        }

        let room = layout.segments[text]
            .sections
            .iter()
            .filter(|s| s.size > 0 && !s.is_zerofill())
            .map(|s| s.addr - layout.segments[text].vmaddr)
            .min()
            .unwrap_or(layout.segments[text].filesize);
        if 32 + cmds.len() as u64 > room {
            return Err(format!(
                "no room for {} bytes of load commands before the first section of {}",
                cmds.len(),
                self.images[index].path
            ));
        }
        let mut header = layout.header.clone();
        put_u32(&mut header, 16, ncmds);
        put_u32(&mut header, 20, cmds.len() as u32);
        out[..32].copy_from_slice(&header);
        out[32..32 + cmds.len()].copy_from_slice(&cmds);
        Ok(out)
    }

    fn build_linkedit(
        &self,
        index: usize,
        layout: &ImageLayout,
        xrefs: &[super::xrefs::Xref],
    ) -> Result<Linkedit, String> {
        let mut symbols = self.image_symbols(index)?;
        let mut dysymtab = [0u32; 6];
        let mut indirect = Vec::new();
        if let Some(at) = layout.dysymtab {
            for (i, field) in dysymtab.iter_mut().enumerate() {
                *field = layout.cmd_u32(at + 8 + i * 4);
            }
            let [ilocal, nlocal, ..] = dysymtab;
            let locals = self.local_symbols(index)?;
            let local_end = (ilocal + nlocal) as usize;
            let replace = !locals.is_empty() && local_end <= symbols.len();
            let added = if replace { locals.len() as i64 - nlocal as i64 } else { 0 };
            let remap = |old: u32| -> u32 {
                if !replace || old < ilocal {
                    old
                } else if (old as usize) < local_end {
                    INDIRECT_SYMBOL_LOCAL
                } else {
                    (old as i64 + added) as u32
                }
            };
            indirect = self
                .image_indirect_symbols(index)?
                .into_iter()
                .map(|v| if v & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 { v } else { remap(v) })
                .collect();
            if replace {
                let count = locals.len() as u32;
                symbols.splice(ilocal as usize..local_end, locals);
                dysymtab[1] = count;
                dysymtab[2] = remap(dysymtab[2]);
                dysymtab[4] = remap(dysymtab[4]);
            }
        }

        let mut le = Linkedit::default();
        let mut strings = b" \0".to_vec();
        let mut nlists = Vec::with_capacity(symbols.len() * 16);
        for sym in &symbols {
            let strx = if sym.name.is_empty() {
                0
            } else {
                let at = strings.len() as u32;
                strings.extend(&sym.name);
                strings.push(0);
                at
            };
            nlists.extend(strx.to_le_bytes());
            nlists.extend([sym.n_type, sym.n_sect]);
            nlists.extend(sym.n_desc.to_le_bytes());
            nlists.extend(sym.n_value.to_le_bytes());
        }
        le.symoff = le.push(&nlists);
        le.nsyms = symbols.len() as u32;
        le.dysymtab = dysymtab;
        if !indirect.is_empty() {
            let raw: Vec<u8> = indirect.iter().flat_map(|v| v.to_le_bytes()).collect();
            le.indirectoff = le.push(&raw);
            le.nindirect = indirect.len() as u32;
        }
        for (cmd, at) in layout.commands() {
            let (kind, off, size) = match cmd {
                LC_DYLD_INFO | LC_DYLD_INFO_ONLY => {
                    (LC_DYLD_EXPORTS_TRIE, layout.cmd_u32(at + 40), layout.cmd_u32(at + 44))
                }
                LC_DYLD_EXPORTS_TRIE | LC_FUNCTION_STARTS | LC_DATA_IN_CODE => {
                    (cmd, layout.cmd_u32(at + 8), layout.cmd_u32(at + 12))
                }
                _ => continue,
            };
            if size == 0 || le.copied(kind).is_some() {
                continue;
            }
            let va = layout.linkedit_va(off).ok_or("linkedit data lies outside __LINKEDIT")?;
            let bytes = self.read_va(va, size as usize)?;
            let at = le.push(&bytes);
            le.copies.push((kind, at, size));
        }
        if !xrefs.is_empty() {
            let blob = super::xrefs::encode(xrefs);
            let at = le.push(&blob);
            le.note = Some((at, blob.len() as u64));
        }
        le.stroff = le.push(&strings);
        le.strsize = strings.len() as u32;
        Ok(le)
    }
}

/// The private `__LINKEDIT` being assembled; offsets are from its start.
#[derive(Default)]
struct Linkedit {
    bytes: Vec<u8>,
    symoff: u64,
    nsyms: u32,
    stroff: u64,
    strsize: u32,
    dysymtab: [u32; 6],
    indirectoff: u64,
    nindirect: u32,
    copies: Vec<(u32, u64, u32)>,
    note: Option<(u64, u64)>,
}

impl Linkedit {
    fn push(&mut self, data: &[u8]) -> u64 {
        let at = align(self.bytes.len() as u64, 8);
        self.bytes.resize(at as usize, 0);
        self.bytes.extend(data);
        at
    }

    fn copied(&self, cmd: u32) -> Option<(u64, u32)> {
        self.copies.iter().find(|c| c.0 == cmd).map(|c| (c.1, c.2))
    }
}

fn align(v: u64, to: u64) -> u64 {
    v.div_ceil(to) * to
}

fn put_u32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn put_u64(b: &mut [u8], at: usize, v: u64) {
    b[at..at + 8].copy_from_slice(&v.to_le_bytes());
}
