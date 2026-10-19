//! (kuna `narrowargs callers`) What a 32-bit ARM image's bytes say about who
//! can reach a function, read once per binary.
//!
//! A function's callers can be listed only when nothing takes its address and
//! every branch to it is a call the decompiles saw. The decompiles themselves
//! catch every address their analysis resolves (`kuna_callerargs::record`);
//! this scan covers the bytes no decompile reads, conservatively: a value is
//! taken for an address wherever it could be one, and a branch is read at every
//! offset an instruction could start at, so a coincidence only leaves a
//! function open.

use std::collections::{BTreeMap, BTreeSet};

use object::{Object, ObjectSection, ObjectSymbol, ObjectSymbolTable};

/// `DT_ANDROID_REL`, `DT_ANDROID_RELA`: Android's packed relocation tables.
const PACKED_RELOCATION_TAGS: [u32; 2] = [0x6000_000f, 0x6000_0011];

/// Does a 32-bit dynamic section (its raw bytes) name a relocation table this
/// loader does not decode? Then an address may exist only as an encoded delta.
pub(crate) fn packs_relocations(dynamic: &[u8], little: bool) -> bool {
    let (entries, _) = dynamic.as_chunks::<8>();
    entries.iter().any(|e| {
        let tag = if little { u32::from_le_bytes([e[0], e[1], e[2], e[3]]) } else { u32::from_be_bytes([e[0], e[1], e[2], e[3]]) };
        PACKED_RELOCATION_TAGS.contains(&tag)
    })
}

/// Can the image's dynamic relocations be read? `false` for a shared object
/// with no `.dynamic` section to check, or one with Android packed relocations.
pub(crate) fn relocations_readable(file: &object::File) -> bool {
    match file.section_by_name(".dynamic") {
        Some(dynamic) => dynamic.data().is_ok_and(|d| !packs_relocations(d, file.is_little_endian())),
        None => file.kind() != object::ObjectKind::Dynamic,
    }
}

/// The scan's answer for a set of function entries.
pub(crate) struct ArmScan {
    /// Entries whose address the image can build or holds.
    pub(crate) open: BTreeSet<u64>,
    /// Entry -> every instruction address that decodes as a direct branch or
    /// call to it.
    pub(crate) branches: BTreeMap<u64, Vec<u64>>,
}

/// Scan `file` for the entries in `entries`:
///
/// - a 32-bit word of a loaded section, at any byte offset (a packed record
///   leaves a pointer unaligned), equal to the entry or to it with the Thumb
///   bit, or self-relative to it (its own address plus its value); a dynamic
///   relocation's target, a dynamic symbol, an export or the entry point;
/// - a `movw` and a `movt` (Thumb-2 or A32, anywhere in code) whose immediates
///   could build it, the halves not matched to each other;
/// - a PC-relative build: an `adr` (Thumb, Thumb-2, A32), or a literal
///   `ldr rX, [pc, #]` followed within 64 bytes by `add rX, pc` (the PIC
///   idiom, Thumb and A32), the literal plus that `pc` landing on it;
/// - a direct `bl`/`blx`/`b.w` (Thumb-2) or `bl`/`b` (A32) to it.
pub(crate) fn scan(file: &object::File, entries: &[u64]) -> ArmScan {
    #[expect(clippy::disallowed_types, reason = "Membership only; the BTreeSet orders the results.")]
    let mut wanted: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    for &e in entries {
        wanted.insert(e, e);
        wanted.insert(e | 1, e);
    }
    let mut out = ArmScan { open: BTreeSet::new(), branches: BTreeMap::new() };
    let little = file.is_little_endian();
    let half = |b: &[u8]| if little { u16::from_le_bytes([b[0], b[1]]) } else { u16::from_be_bytes([b[0], b[1]]) };
    let word = |b: &[u8]| {
        if little {
            u32::from_le_bytes([b[0], b[1], b[2], b[3]])
        } else {
            u32::from_be_bytes([b[0], b[1], b[2], b[3]])
        }
    };
    #[expect(clippy::disallowed_types, reason = "Membership only.")]
    let (mut movw, mut movt) = (std::collections::HashSet::new(), std::collections::HashSet::new());
    let mut loaded: Vec<(u64, Vec<u8>)> = Vec::new();
    for section in file.sections() {
        if !super::image_loads(&section) {
            continue;
        }
        let Ok(data) = section.data() else { continue };
        let addr = section.address();
        for i in 0..data.len().saturating_sub(3) {
            let w = u64::from(word(&data[i..]));
            for v in [w, (addr + i as u64).wrapping_add(w) & 0xffff_ffff] {
                if let Some(&e) = wanted.get(&v) {
                    out.open.insert(e);
                }
            }
        }
        if section.kind() == object::SectionKind::Text {
            loaded.push((addr, data.to_vec()));
        }
    }
    let read = |va: u64| -> Option<u32> {
        loaded.iter().find_map(|(a, d)| {
            let off = va.checked_sub(*a)? as usize;
            d.get(off..off + 4).map(word)
        })
    };
    for (addr, data) in &loaded {
        let mut pc_literal: Vec<(u64, u32, u32)> = Vec::new();
        for i in (0..data.len().saturating_sub(3)).step_by(2) {
            let a = addr + i as u64;
            let (h1, h2) = (u32::from(half(&data[i..])), u32::from(half(&data[i + 2..])));
            let thumb_pc = (a + 4) & !3;
            if matches!(h1 & 0xfbf0, 0xf240 | 0xf2c0) && h2 & 0x8000 == 0 {
                let imm = ((h1 & 0xf) << 12) | (((h1 >> 10) & 1) << 11) | (((h2 >> 12) & 7) << 8) | (h2 & 0xff);
                if h1 & 0xfbf0 == 0xf240 { movw.insert(imm) } else { movt.insert(imm) };
            }
            let mut built: Vec<u64> = Vec::new();
            if h1 & 0xf800 == 0xa000 {
                built.push(thumb_pc + u64::from(h1 & 0xff) * 4);
            }
            if matches!(h1 & 0xfbff, 0xf20f | 0xf2af) && h2 & 0x8000 == 0 {
                let imm = u64::from((((h1 >> 10) & 1) << 11) | (((h2 >> 12) & 7) << 8) | (h2 & 0xff));
                built.push(if h1 & 0xfbff == 0xf20f { thumb_pc + imm } else { thumb_pc.wrapping_sub(imm) });
            }
            if h1 & 0xf800 == 0x4800 {
                if let Some(v) = read(thumb_pc + u64::from(h1 & 0xff) * 4) {
                    pc_literal.push((a, (h1 >> 8) & 7, v));
                }
            }
            if matches!(h1, 0xf8df | 0xf85f) {
                let off = u64::from(h2 & 0xfff);
                let lit = if h1 == 0xf8df { thumb_pc + off } else { thumb_pc.wrapping_sub(off) };
                if let Some(v) = read(lit) {
                    pc_literal.push((a, h2 >> 12, v));
                }
            }
            if h1 & 0xff78 == 0x4478 {
                let rd = (h1 & 7) | ((h1 >> 4) & 8);
                if let Some(&(_, _, v)) = pc_literal.iter().rev().find(|&&(at, r, _)| r == rd && a - at <= 64) {
                    built.push(u64::from(v).wrapping_add(a + 4) & 0xffff_ffff);
                }
            }
            let mut branch: Vec<u64> = Vec::new();
            let rel = |imm: u32, bits: u32| ((a + 4) as i64 + ((imm << (32 - bits)) as i32 >> (32 - bits)) as i64) as u64;
            if h1 & 0xf800 == 0xf000 && h2 & 0xd000 >= 0x9000 {
                let s = (h1 >> 10) & 1;
                let (j1, j2) = ((h2 >> 13) & 1, (h2 >> 11) & 1);
                let (i1, i2) = (1 ^ (j1 ^ s), 1 ^ (j2 ^ s));
                let imm = (s << 24) | (i1 << 23) | (i2 << 22) | ((h1 & 0x3ff) << 12) | ((h2 & 0x7ff) << 1);
                let target = rel(imm, 25);
                branch.push(if h2 & 0x1000 == 0 { target & !3 } else { target });
            }
            if h1 & 0xf800 == 0xf000 && h2 & 0xd000 == 0x8000 && (h1 >> 6) & 0xe != 0xe {
                let (s, j1, j2) = ((h1 >> 10) & 1, (h2 >> 13) & 1, (h2 >> 11) & 1);
                branch.push(rel((s << 20) | (j2 << 19) | (j1 << 18) | ((h1 & 0x3f) << 12) | ((h2 & 0x7ff) << 1), 21));
            }
            if h1 & 0xf800 == 0xe000 {
                branch.push(rel((h1 & 0x7ff) << 1, 12));
            }
            if h1 & 0xf000 == 0xd000 && (h1 >> 8) & 0xe != 0xe {
                branch.push(rel((h1 & 0xff) << 1, 9));
            }
            if h1 & 0xf500 == 0xb100 {
                branch.push(a + 4 + u64::from((((h1 >> 9) & 1) << 6) | (((h1 >> 3) & 0x1f) << 1)));
            }
            if a.is_multiple_of(4) {
                let w = word(&data[i..]);
                let arm_pc = a + 8;
                if matches!(w & 0x0ff0_0000, 0x0300_0000 | 0x0340_0000) {
                    let imm = ((w >> 4) & 0xf000) | (w & 0xfff);
                    if w & 0x0ff0_0000 == 0x0300_0000 { movw.insert(imm) } else { movt.insert(imm) };
                }
                if matches!(w & 0x0fff_0000, 0x028f_0000 | 0x024f_0000) {
                    let imm = (w & 0xff).rotate_right(2 * ((w >> 8) & 0xf));
                    built.push(if w & 0x0fff_0000 == 0x028f_0000 { arm_pc + u64::from(imm) } else { arm_pc.wrapping_sub(u64::from(imm)) });
                }
                if w & 0x0f7f_0000 == 0x051f_0000 {
                    let off = u64::from(w & 0xfff);
                    let lit = if w & 0x0080_0000 != 0 { arm_pc + off } else { arm_pc.wrapping_sub(off) };
                    if let Some(v) = read(lit) {
                        pc_literal.push((a, (w >> 12) & 0xf, v));
                    }
                }
                if w & 0x0fff_0ff0 == 0x008f_0000 {
                    let rm = w & 0xf;
                    if let Some(&(_, _, v)) = pc_literal.iter().rev().find(|&&(at, r, _)| r == rm && a - at <= 64) {
                        built.push(u64::from(v).wrapping_add(arm_pc) & 0xffff_ffff);
                    }
                }
                if w & 0x0e00_0000 == 0x0a00_0000 {
                    let off = (((w & 0x00ff_ffff) << 8) as i32 >> 6) as i64;
                    let h = if w >> 28 == 0xf { u64::from((w >> 24) & 1) << 1 } else { 0 };
                    branch.push((((arm_pc as i64) + off) as u64) + h);
                }
            }
            for t in branch {
                if let Some(&e) = wanted.get(&t) {
                    out.branches.entry(e).or_default().push(a);
                }
            }
            for v in built {
                if let Some(&e) = wanted.get(&v) {
                    out.open.insert(e);
                }
            }
            pc_literal.retain(|&(at, _, _)| a - at <= 64);
        }
    }
    for &e in entries {
        for v in [e, e | 1] {
            if movw.contains(&((v & 0xffff) as u32)) && movt.contains(&((v >> 16) as u32)) {
                out.open.insert(e);
            }
        }
    }
    let mut note = |v: u64| {
        if let Some(&e) = wanted.get(&v) {
            out.open.insert(e);
        }
    };
    for sym in file.dynamic_symbols() {
        if sym.is_definition() {
            note(sym.address());
        }
    }
    if let Some(relocs) = file.dynamic_relocations() {
        for (_, r) in relocs {
            let base = match r.target() {
                object::RelocationTarget::Symbol(i) => {
                    file.dynamic_symbol_table().and_then(|t| t.symbol_by_index(i).ok()).map(|s| s.address())
                }
                object::RelocationTarget::Absolute => Some(0),
                _ => None,
            };
            if let Some(b) = base {
                note(b.wrapping_add(r.addend() as u64) & 0xffff_ffff);
            }
        }
    }
    for export in file.exports().unwrap_or_default() {
        note(export.address());
    }
    note(file.entry());
    out
}

#[cfg(test)]
mod tests {
    use super::packs_relocations;

    fn dynamic(tags: &[u32]) -> Vec<u8> {
        tags.iter().flat_map(|t| t.to_le_bytes().into_iter().chain(0u32.to_le_bytes())).collect()
    }

    #[test]
    fn android_packed_relocations_are_unreadable() {
        assert!(packs_relocations(&dynamic(&[0x6000_0011, 0x6000_0012, 0x17, 0]), true));
        assert!(packs_relocations(&dynamic(&[0x6000_000f, 0]), true));
    }

    #[test]
    fn rel_rela_and_relr_tables_are_readable() {
        assert!(!packs_relocations(&dynamic(&[0x11, 0x12, 0x13, 0x7, 0x8, 0x9, 0x24, 0x23, 0x25, 0]), true));
    }
}
