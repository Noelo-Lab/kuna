//! Name the references a cache image makes into other images.
//!
//! The cache builder routes an image's calls into other images past its own
//! stubs. On iOS 16 a `bl` lands on a stub island in a `__TEXT_STUBS` sub-cache
//! (`adrp x16; add x16; br x16` to the callee); on iOS 14 it lands on another
//! image's rewritten stub; within branch range it lands on the callee itself.
//! The builder also coalesces `__got` sections and drops their indirect-symbol
//! entries, so a load through one names nothing. None of these targets is in the
//! rebuilt image, so nothing there names them.
//!
//! [`collect`] finds every branch target outside the image and every coalesced
//! GOT slot holding an address outside it. It follows a stub to its final
//! target, then names that target from the symbol table of the image holding
//! it (exports, else the locals in the local-symbols file). The result rides in
//! the rebuilt image as an `LC_NOTE` owned by [`NOTE_OWNER`].
//! `macho_stubs` reads the note back as imports, under the `dyldcrossrefs`
//! option.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

use std::rc::Rc;

use super::extract::{ImageLayout, Nlist};
use super::{u32_at, u64_at, DyldCache};

/// The `LC_NOTE` `data_owner` of the cross-image reference table.
pub const NOTE_OWNER: &[u8; 16] = b"kuna.dyld.xrefs\0";

/// What an [`Xref`] address is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefKind {
    /// A branch target: calling it calls the named function.
    Call,
    /// A pointer slot holding the named symbol's address.
    Slot,
}

/// One named reference into another image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Xref {
    pub addr: u64,
    pub kind: XrefKind,
    /// The raw symbol name, leading underscore included.
    pub name: Vec<u8>,
}

/// Serialize: `u32 count`, then per entry `u64 addr, u32 kind, u32 name offset`,
/// then the NUL-terminated names.
pub fn encode(xrefs: &[Xref]) -> Vec<u8> {
    let mut table = (xrefs.len() as u32).to_le_bytes().to_vec();
    let mut names = Vec::new();
    for x in xrefs {
        table.extend(x.addr.to_le_bytes());
        table.extend((x.kind as u32).to_le_bytes());
        table.extend((names.len() as u32).to_le_bytes());
        names.extend(&x.name);
        names.push(0);
    }
    table.extend(names);
    table
}

/// The inverse of [`encode`]; a malformed blob yields the entries before the fault.
pub fn decode(blob: &[u8]) -> Vec<Xref> {
    let Some(count) = blob.get(..4).map(|b| u32_at(b, 0) as usize) else {
        return Vec::new();
    };
    let names_at = 4 + count.saturating_mul(16);
    let Some(names) = blob.get(names_at..) else {
        return Vec::new();
    };
    blob[4..names_at]
        .chunks_exact(16)
        .map_while(|e| {
            let kind = match u32_at(e, 8) {
                0 => XrefKind::Call,
                1 => XrefKind::Slot,
                _ => return None,
            };
            let name = names.get(u32_at(e, 12) as usize..)?;
            let name = &name[..name.iter().position(|&b| b == 0)?];
            Some(Xref { addr: u64_at(e, 0), kind, name: name.to_vec() })
        })
        .collect()
}

/// The cross-image references a thin 64-bit Mach-O carries in its
/// `kuna.dyld.xrefs` note; empty for any other input.
pub fn read_note(bytes: &[u8]) -> Vec<Xref> {
    const MH_MAGIC_64: u32 = 0xfeed_facf;
    if bytes.len() < 32 || u32_at(bytes, 0) != MH_MAGIC_64 {
        return Vec::new();
    }
    let (ncmds, sizeofcmds) = (u32_at(bytes, 16) as usize, u32_at(bytes, 20) as usize);
    let Some(cmds) = bytes.get(32..32 + sizeofcmds) else {
        return Vec::new();
    };
    let mut at = 0;
    for _ in 0..ncmds {
        let Some(c) = cmds.get(at..at + 8) else {
            break;
        };
        let (cmd, size) = (u32_at(c, 0), u32_at(c, 4) as usize);
        if cmd == super::extract::LC_NOTE && size >= 40 && cmds.get(at + 8..at + 24) == Some(NOTE_OWNER) {
            let (off, len) = (u64_at(cmds, at + 24) as usize, u64_at(cmds, at + 32) as usize);
            return bytes.get(off..off.saturating_add(len)).map(decode).unwrap_or_default();
        }
        if size < 8 {
            break;
        }
        at += size;
    }
    Vec::new()
}

/// Collect image `layout`'s references into other images. `view(va, len)`
/// reads the image's slid bytes. Branches are decoded only for an arm64 image
/// (`a64`); an x86_64 image gets its GOT slots alone.
pub(super) fn collect<'v>(
    cache: &DyldCache,
    layout: &ImageLayout,
    a64: bool,
    view: &dyn Fn(u64, usize) -> Option<&'v [u8]>,
) -> Vec<Xref> {
    let mut targets = BTreeSet::new();
    let mut slots = Vec::new();
    for seg in &layout.segments {
        for sec in &seg.sections {
            let Some(bytes) = view(sec.addr, sec.size as usize) else {
                continue;
            };
            if sec.is_code() {
                if !a64 {
                    continue;
                }
                for (i, w) in bytes.chunks_exact(4).enumerate() {
                    let w = u32_at(w, 0);
                    if w & 0x7C00_0000 == 0x1400_0000 {
                        let pc = sec.addr + i as u64 * 4;
                        let target = pc.wrapping_add_signed(sign_extend(w & 0x03FF_FFFF, 26) << 2);
                        if !layout.holds(target) {
                            targets.insert(target);
                        }
                    }
                }
            } else if (sec.sectname == "__got" || sec.sectname == "__auth_got") && sec.section_type() == 0 {
                for (i, p) in bytes.chunks_exact(8).enumerate() {
                    let value = u64_at(p, 0);
                    if value != 0 && !layout.holds(value) {
                        slots.push((sec.addr + i as u64 * 8, value));
                    }
                }
            }
        }
    }
    if targets.is_empty() && slots.is_empty() {
        return Vec::new();
    }
    let names = Names::new(cache);
    let mut out = Vec::new();
    for target in targets {
        if let Some(name) = names.call_target(target) {
            out.push(Xref { addr: target, kind: XrefKind::Call, name });
        }
    }
    for (addr, value) in slots {
        if let Some(name) = names.at(value) {
            out.push(Xref { addr, kind: XrefKind::Slot, name });
        }
    }
    out
}

fn sign_extend(v: u32, bits: u32) -> i64 {
    let shift = 64 - bits;
    ((v as i64) << shift) >> shift
}

/// Follow the stub at `addr` to the address it jumps to: up to four
/// instructions of `adrp`, `add`, `ldr` (through slid data) ending in `br`,
/// `braa`/`brab` or `braaz`/`brabz`. `None` for anything else.
fn follow_stub(cache: &DyldCache, addr: u64) -> Option<u64> {
    let code = cache.read_va(addr, 16).ok()?;
    let mut regs = [None::<u64>; 32];
    for (i, w) in code.chunks_exact(4).enumerate() {
        let w = u32_at(w, 0);
        let pc = addr + i as u64 * 4;
        let rd = (w & 31) as usize;
        let rn = ((w >> 5) & 31) as usize;
        if w & 0x9F00_0000 == 0x9000_0000 {
            let imm = ((w >> 3) & 0x001F_FFFC) | ((w >> 29) & 3);
            regs[rd] = Some((pc & !0xFFF).wrapping_add_signed(sign_extend(imm, 21) << 12));
        } else if w & 0xFF80_0000 == 0x9100_0000 {
            let imm = ((w >> 10) & 0xFFF) as u64;
            let shift = if w & (1 << 22) != 0 { 12 } else { 0 };
            regs[rd] = Some(regs[rn]?.wrapping_add(imm << shift));
        } else if w & 0xFFC0_0000 == 0xF940_0000 {
            let slot = regs[rn]?.wrapping_add((((w >> 10) & 0xFFF) as u64) * 8);
            regs[rd] = Some(u64_at(&cache.read_slid(slot, 8).ok()?, 0));
        } else if w & 0xFFFF_FC1F == 0xD61F_0000 || w & 0xFFFF_F800 == 0xD71F_0800 || w & 0xFFFF_F81F == 0xD61F_081F {
            return regs[rn];
        } else {
            return None;
        }
    }
    None
}

/// The most stubs a call target is followed through before it is left unnamed.
const MAX_STUB_HOPS: usize = 4;

/// One image's names: by address, and the symbol table and indirect-symbol
/// table its stubs index.
#[derive(Default)]
struct ImageNames {
    by_addr: HashMap<u64, Vec<u8>>,
    symbols: Vec<Nlist>,
    indirect: Vec<u32>,
}

/// Symbol names by address across the whole cache, built per image on demand.
struct Names<'a> {
    cache: &'a DyldCache,
    ranges: Vec<(u64, u64, usize)>,
    images: RefCell<HashMap<usize, Rc<ImageNames>>>,
}

impl<'a> Names<'a> {
    fn new(cache: &'a DyldCache) -> Names<'a> {
        let mut ranges: Vec<(u64, u64, usize)> = (0..cache.images().len())
            .filter_map(|i| cache.image_layout(i).ok().map(|l| (i, l)))
            .flat_map(|(i, layout)| {
                layout
                    .segments
                    .iter()
                    .filter(|s| s.name != "__LINKEDIT")
                    .map(|s| (s.vmaddr, s.vmaddr + s.vmsize, i))
                    .collect::<Vec<_>>()
            })
            .collect();
        ranges.sort_unstable();
        Names { cache, ranges, images: RefCell::default() }
    }

    fn image_at(&self, addr: u64) -> Option<usize> {
        let i = self.ranges.partition_point(|r| r.0 <= addr).checked_sub(1)?;
        let (lo, hi, image) = self.ranges[i];
        (addr >= lo && addr < hi).then_some(image)
    }

    fn image(&self, index: usize) -> Rc<ImageNames> {
        self.images.borrow_mut().entry(index).or_insert_with(|| Rc::new(self.load(index))).clone()
    }

    /// Image `index`'s names: locals first, so an export of the same address
    /// replaces them.
    fn load(&self, index: usize) -> ImageNames {
        let locals = self.cache.local_symbols(index).unwrap_or_default();
        let symbols = self.cache.image_symbols(index).unwrap_or_default();
        let mut by_addr = HashMap::new();
        for sym in locals.iter().chain(symbols.iter().filter(|s| !s.is_external())) {
            if sym.names_address() {
                by_addr.entry(sym.n_value).or_insert_with(|| sym.name.clone());
            }
        }
        for sym in symbols.iter().filter(|s| s.is_external() && s.names_address()) {
            by_addr.insert(sym.n_value, sym.name.clone());
        }
        let indirect = self.cache.image_indirect_symbols(index).unwrap_or_default();
        ImageNames { by_addr, symbols, indirect }
    }

    fn at(&self, addr: u64) -> Option<Vec<u8>> {
        self.image(self.image_at(addr)?).by_addr.get(&addr).cloned()
    }

    /// The function a branch to `addr` calls: a symbol at `addr`, else the
    /// import a stub at `addr` is for, else whatever the stub jumps to.
    fn call_target(&self, mut addr: u64) -> Option<Vec<u8>> {
        for _ in 0..MAX_STUB_HOPS {
            if let Some(name) = self.at(addr).or_else(|| self.stub_import(addr)) {
                return Some(name);
            }
            addr = follow_stub(self.cache, addr)?;
        }
        None
    }

    /// The symbol an image's own `S_SYMBOL_STUBS` entry at `addr` stands for,
    /// from that image's indirect-symbol table.
    fn stub_import(&self, addr: u64) -> Option<Vec<u8>> {
        const S_SYMBOL_STUBS: u32 = 0x8;
        let index = self.image_at(addr)?;
        let layout = self.cache.image_layout(index).ok()?;
        let sec = layout
            .segments
            .iter()
            .flat_map(|s| &s.sections)
            .find(|s| s.section_type() == S_SYMBOL_STUBS && addr >= s.addr && addr < s.addr + s.size)?;
        let stride = sec.reserved2 as u64;
        if stride == 0 || (addr - sec.addr) % stride != 0 {
            return None;
        }
        let image = self.image(index);
        let slot = sec.reserved1 as usize + ((addr - sec.addr) / stride) as usize;
        let symbol = *image.indirect.get(slot)?;
        if symbol & 0xC000_0000 != 0 {
            return None;
        }
        let name = &image.symbols.get(symbol as usize)?.name;
        (!name.is_empty()).then(|| name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_round_trips() {
        let xrefs = vec![
            Xref { addr: 0x1_8c99_b570, kind: XrefKind::Call, name: b"___stack_chk_fail".to_vec() },
            Xref { addr: 0x1_dc07_f118, kind: XrefKind::Slot, name: b"_objc_debug_isa_class_mask".to_vec() },
        ];
        assert_eq!(decode(&encode(&xrefs)), xrefs);
        assert!(decode(&[1, 0]).is_empty());
        assert!(decode(&encode(&xrefs)[..20]).is_empty());
    }

    #[test]
    fn adrp_immediates_decode_with_sign() {
        assert_eq!(sign_extend(0x1F_FFFF, 21), -1);
        assert_eq!(sign_extend(0x03FF_FFFF, 26) << 2, -4);
    }
}
