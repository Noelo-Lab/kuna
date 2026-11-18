//! **Exception tables** — the call-site records of every function's
//! language-specific data area (LSDA), for the flow's `ehcatch` edges
//! (`kuna_decomp::kuna_ehcatch`). Linked ELF images only; inert on an image
//! without an LSDA.
//!
//! # Where an LSDA comes from
//!
//! * `.eh_frame`: an FDE whose CIE augmentation carries `L` points at its LSDA
//!   in `.gcc_except_table` ([`super::scan_eh_frame_lsdas`]).
//! * `.ARM.exidx` (ARM32): a generic-model `.ARM.extab` entry holds the LSDA
//!   after its unwind opcodes ([`super::kuna_armexidx`]).
//!
//! # The layout
//!
//! ```text
//!   u8      lpStart encoding      (omit: landing pads count from the function start)
//!   ...     lpStart
//!   u8      type-table encoding   (omit: no type table)
//!   uleb    type-table offset     (from the byte after this field to the table's END)
//!   u8      call-site encoding
//!   uleb    call-site table length
//!   call-site records:  start, length, landing pad (call-site encoding), action (uleb)
//!   action records:     type filter (sleb), offset to the next record (sleb, self-relative)
//!   type table:         one pointer per catch type, indexed BACKWARDS from its end
//! ```
//!
//! A record's `start` counts from the function start. A landing pad of 0 means
//! a throw leaves the function. An action of 0 is a cleanup-only pad; otherwise
//! `action - 1` is the offset of the first action record. A positive filter `n`
//! selects the `n`th type-table entry counted back from the table's end, and a
//! null entry is `catch (...)`. A zero filter is a cleanup and a negative one
//! an exception specification; neither is a handler.
//!
//! # Naming a caught type
//!
//! A type-table entry leads to a `std::type_info` object, usually through a
//! slot the dynamic linker fills. The name comes from, in order: the symbol of
//! a dynamic relocation on the slot; an `_ZTI` symbol at the object; the
//! object's own name string (its second word), which survives stripping. An
//! entry that resolves to none of these prints as `...`.
//!
//! On ARM32 the entries are `R_ARM_TARGET2` words behind an `absptr` encoding
//! byte. Linux resolves that relocation as a GOT-relative displacement, so the
//! word is read as a self-relative offset to a slot; when nothing names the
//! slot it is read as a self-relative offset to the object itself (the
//! bare-metal convention).

use std::collections::HashMap;

use object::read::{Object, ObjectSection, ObjectSymbol, ObjectSymbolTable};
use object::{ObjectKind, RelocationTarget, SectionKind};

use kuna_decomp::kuna_ehcatch::{EhCallSite, CATCH_ALL};

use super::{
    decode_eh_pointer_ordered, read_sleb128, read_u32, read_u64, read_uleb128, skip_eh_pointer, DW_EH_PE_ABSPTR,
    DW_EH_PE_OMIT,
};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// The longest action chain followed for one call site.
const MAX_ACTIONS: usize = 64;
/// The longest guarded range accepted.
const MAX_RANGE: u64 = 1 << 24;

/// The allocated sections of the image, for reads by address.
struct Image<'a> {
    sections: Vec<(u64, &'a [u8])>,
    ptr_size: usize,
    le: bool,
}

impl<'a> Image<'a> {
    fn new(file: &'a object::File<'a>) -> Self {
        let mut sections: Vec<(u64, &'a [u8])> = file
            .sections()
            .filter(|s| !matches!(s.kind(), SectionKind::Metadata | SectionKind::Other | SectionKind::Debug))
            .filter_map(|s| Some((s.address(), s.data().ok()?)))
            .filter(|(vma, data)| *vma != 0 && !data.is_empty())
            .collect();
        sections.sort_by_key(|s| s.0);
        Image { sections, ptr_size: if file.is_64() { 8 } else { 4 }, le: file.is_little_endian() }
    }

    /// The section holding `vma`, as `(section address, bytes, offset of vma)`.
    fn locate(&self, vma: u64) -> Option<(u64, &'a [u8], usize)> {
        let i = self.sections.partition_point(|s| s.0 <= vma).checked_sub(1)?;
        let (base, data) = self.sections[i];
        let off = usize::try_from(vma - base).ok()?;
        (off < data.len()).then_some((base, data, off))
    }

    fn pointer(&self, vma: u64) -> Option<u64> {
        let (_, data, off) = self.locate(vma)?;
        let bytes = data.get(off..off.checked_add(self.ptr_size)?)?;
        Some(if self.ptr_size == 8 { read_u64(bytes, self.le) } else { read_u32(bytes, self.le) as u64 })
    }

    fn c_string(&self, vma: u64) -> Option<&'a str> {
        let (_, data, off) = self.locate(vma)?;
        let rest = data.get(off..)?;
        let len = rest.iter().take(4096).position(|&b| b == 0)?;
        std::str::from_utf8(&rest[..len]).ok()
    }
}

/// What the image says about its `std::type_info` objects.
struct TypeNames {
    /// Slot address -> the symbol a dynamic relocation binds there.
    slot_symbols: HashMap<u64, String>,
    /// Slot address -> the address a symbol-less dynamic relocation writes.
    slot_targets: HashMap<u64, u64>,
    /// Address -> `_ZTI` symbol defined there.
    objects: HashMap<u64, String>,
}

impl TypeNames {
    fn new(file: &object::File) -> Self {
        let mut names =
            TypeNames { slot_symbols: HashMap::new(), slot_targets: HashMap::new(), objects: HashMap::new() };
        if let Some(relocs) = file.dynamic_relocations() {
            let dynsyms = file.dynamic_symbol_table();
            for (slot, rel) in relocs {
                match rel.target() {
                    RelocationTarget::Symbol(idx) => {
                        let name = dynsyms.as_ref().and_then(|t| t.symbol_by_index(idx).ok());
                        if let Some(name) = name.and_then(|s| s.name().ok().map(str::to_string)) {
                            names.slot_symbols.insert(slot, name);
                        }
                    }
                    _ if !rel.has_implicit_addend() => {
                        names.slot_targets.insert(slot, rel.addend() as u64);
                    }
                    _ => {}
                }
            }
        }
        for sym in file.symbols().chain(file.dynamic_symbols()) {
            if let Ok(name) = sym.name() {
                if sym.is_definition() && name.starts_with("_ZTI") {
                    names.objects.insert(sym.address(), name.to_string());
                }
            }
        }
        names
    }

    /// The pointer the loader leaves in `slot`.
    fn slot_value(&self, image: &Image, slot: u64) -> Option<u64> {
        self.slot_targets.get(&slot).copied().or_else(|| image.pointer(slot)).filter(|&v| v != 0)
    }

    /// The type a `type_info` object at `object` describes.
    fn object_type(&self, image: &Image, object: u64) -> Option<String> {
        if let Some(sym) = self.objects.get(&object) {
            return crate::demangle::demangle_typeinfo_name(sym.strip_prefix("_ZTI")?);
        }
        let name = self.slot_value(image, object + image.ptr_size as u64)?;
        crate::demangle::demangle_typeinfo_name(image.c_string(name)?)
    }

    /// The type behind a slot that holds a `type_info` pointer.
    fn slot_type(&self, image: &Image, slot: u64) -> Option<String> {
        if let Some(sym) = self.slot_symbols.get(&slot) {
            return crate::demangle::demangle_typeinfo_name(sym.strip_prefix("_ZTI")?);
        }
        self.object_type(image, self.slot_value(image, slot)?)
    }
}

/// The type table of one LSDA.
struct TypeTable {
    /// Address one past the table's last entry.
    end: u64,
    encoding: u8,
    /// ARM32 `R_ARM_TARGET2` entries.
    target2: bool,
}

impl TypeTable {
    /// The clause text for the positive type filter `filter`.
    fn catch(&self, image: &Image, names: &TypeNames, filter: u64) -> String {
        self.resolve(image, names, filter).unwrap_or_else(|| CATCH_ALL.to_string())
    }

    fn resolve(&self, image: &Image, names: &TypeNames, filter: u64) -> Option<String> {
        let size = match self.encoding & 0x0f {
            DW_EH_PE_ABSPTR => image.ptr_size,
            0x02 | 0x0a => 2,
            0x03 | 0x0b => 4,
            0x04 | 0x0c => 8,
            _ => return None,
        };
        let entry = self.end.checked_sub(filter.checked_mul(size as u64)?)?;
        if names.slot_symbols.contains_key(&entry) {
            return names.slot_type(image, entry);
        }
        let (base, data, off) = image.locate(entry)?;
        let window = data.get(..off.checked_add(size)?)?;
        if window[off..].iter().all(|&b| b == 0) && !names.slot_targets.contains_key(&entry) {
            return Some(CATCH_ALL.to_string());
        }
        if self.target2 {
            let target = entry.wrapping_add(read_u32(&window[off..], image.le) as i32 as i64 as u64) & 0xffff_ffff;
            return names.slot_type(image, target).or_else(|| names.object_type(image, target));
        }
        let direct = self.encoding & 0x7f;
        let target = match names.slot_targets.get(&entry) {
            Some(&target) => target,
            None => decode_eh_pointer_ordered(direct, entry, base, window, off, image.ptr_size, image.le)?,
        };
        if self.encoding & 0x80 != 0 {
            names.slot_type(image, target)
        } else {
            names.object_type(image, target)
        }
    }
}

/// The catch clauses of the action chain that starts at `action` (1-based
/// offset into the action table at `actions`), in handler order.
fn catch_clauses(
    data: &[u8],
    actions: usize,
    action: u64,
    types: Option<&TypeTable>,
    image: &Image,
    names: &TypeNames,
) -> Vec<String> {
    let mut out = Vec::new();
    let Some(mut pos) = action.checked_sub(1).and_then(|a| actions.checked_add(usize::try_from(a).ok()?)) else {
        return out;
    };
    for _ in 0..MAX_ACTIONS {
        let Some((filter, link)) = read_sleb128(data, pos) else {
            break;
        };
        let Some((next, _)) = read_sleb128(data, link) else {
            break;
        };
        if filter > 0 {
            out.push(types.map_or_else(|| CATCH_ALL.to_string(), |t| t.catch(image, names, filter as u64)));
        }
        if next == 0 {
            break;
        }
        match (link as i64).checked_add(next).and_then(|p| usize::try_from(p).ok()) {
            Some(p) => pos = p,
            None => break,
        }
    }
    out
}

/// Decode the LSDA at `lsda` for the function starting at `func`, appending one
/// record per call site that has a landing pad.
fn decode_lsda(image: &Image, names: &TypeNames, arm: bool, func: u64, lsda: u64, out: &mut Vec<EhCallSite>) -> Option<()> {
    let (base, data, mut p) = image.locate(lsda)?;
    let ptr = image.ptr_size;
    let field = |enc: u8, p: usize| decode_eh_pointer_ordered(enc, base + p as u64, base, data, p, ptr, image.le);

    let lp_enc = *data.get(p)?;
    p += 1;
    let lp_start = if lp_enc == DW_EH_PE_OMIT {
        func
    } else {
        let v = field(lp_enc, p)?;
        p = skip_eh_pointer(lp_enc, data, p, ptr)?;
        v
    };
    let tt_enc = *data.get(p)?;
    p += 1;
    let types = if tt_enc == DW_EH_PE_OMIT {
        None
    } else {
        let (offset, np) = read_uleb128(data, p)?;
        p = np;
        Some(TypeTable {
            end: base + (p as u64).checked_add(offset)?,
            encoding: tt_enc,
            target2: arm && tt_enc & 0x7f == DW_EH_PE_ABSPTR,
        })
    };
    let cs_enc = *data.get(p)?;
    p += 1;
    let (cs_len, np) = read_uleb128(data, p)?;
    p = np;
    let actions = p.checked_add(usize::try_from(cs_len).ok()?).filter(|&e| e <= data.len())?;
    let table = &data[..actions];

    while p < actions {
        let start = decode_eh_pointer_ordered(cs_enc & 0x0f, 0, 0, table, p, ptr, image.le)?;
        p = skip_eh_pointer(cs_enc, table, p, ptr)?;
        let len = decode_eh_pointer_ordered(cs_enc & 0x0f, 0, 0, table, p, ptr, image.le)?;
        p = skip_eh_pointer(cs_enc, table, p, ptr)?;
        let pad = decode_eh_pointer_ordered(cs_enc & 0x0f, 0, 0, table, p, ptr, image.le)?;
        p = skip_eh_pointer(cs_enc, table, p, ptr)?;
        let (action, np) = read_uleb128(data, p)?;
        p = np;
        if pad == 0 || len == 0 || len > MAX_RANGE {
            continue;
        }
        let start = func.checked_add(start)?;
        out.push(EhCallSite {
            start,
            end: start.checked_add(len)?,
            landing_pad: lp_start.checked_add(pad)?,
            catches: catch_clauses(data, actions, action, types.as_ref(), image, names),
        });
    }
    Some(())
}

/// Every call-site record of a linked ELF image that has a landing pad inside
/// executable bytes, sorted by start with overlapping records dropped.
pub fn scan(file: &object::File) -> Vec<EhCallSite> {
    let mut out = Vec::new();
    if file.format() != object::BinaryFormat::Elf || file.kind() == ObjectKind::Relocatable {
        return out;
    }
    let arm = file.architecture() == object::Architecture::Arm;
    let mut lsdas = super::scan_eh_frame_lsdas(file, if file.is_64() { 8 } else { 4 });
    lsdas.extend(super::kuna_armexidx::scan(file).into_iter().filter_map(|e| Some((e.start, e.lsda?))));
    if lsdas.is_empty() {
        return out;
    }
    let image = Image::new(file);
    let names = TypeNames::new(file);
    for (func, lsda) in lsdas {
        let func = if arm { func & !1 } else { func };
        let kept = out.len();
        if decode_lsda(&image, &names, arm, func, lsda, &mut out).is_none() {
            out.truncate(kept);
        }
    }
    let execs = super::executable_sections(file);
    out.retain(|s| super::in_executable_section(&execs, s.start) && super::in_executable_section(&execs, s.landing_pad));
    out.sort_by_key(|s| (s.start, s.end));
    let mut end = 0;
    out.retain(|s| {
        let keep = s.start >= end;
        if keep {
            end = s.end;
        }
        keep
    });
    out
}

/// (kuna `ehcatch`) Park the image's exception-table call sites for the flow.
pub struct EhTablePass;

impl AnalysisPass for EhTablePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "ehtable"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        out.eh_call_sites = scan(ctx.file);
        out
    }
}

#[cfg(test)]
#[path = "kuna_ehtable/tests.rs"]
mod tests;
