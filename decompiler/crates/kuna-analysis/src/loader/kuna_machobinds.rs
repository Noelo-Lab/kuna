//! Mach-O pointer slots a bind fills, named for their import.
//!
//! The indirect-symbol table names the stubs and the typed `__got` /
//! `__la_symbol_ptr` slots. Any other pointer slot dyld fills from an import - a
//! C function or data pointer initialized to an import, a vtable or Swift
//! witness entry - is recorded only by a bind: a chained-fixup bind or a classic
//! `LC_DYLD_INFO` bind opcode. This pass names each such slot for its import and
//! paints it `externref`, the channel `elfgotimports` uses for ELF GOT slots: a
//! symbol-table name `query_function` resolves for a call or a read through the
//! slot, never a function-inventory entry. A bind with an addend is skipped (the
//! slot holds `&sym + addend`), and so are the symbol-pointer sections the
//! indirect-symbol table names and `__auth_ptr`, `__objc_*` and `__cfstring`,
//! whose own passes name their slots.

use object::read::{Object, ObjectSection};

use crate::loader::format::macho::{resolve_chained_fixups, resolve_dyld_info_bind_slots};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// `(slot, import name)` for every bound slot outside the sections another
/// reader names, leading underscore stripped, sorted by slot.
pub(crate) fn bound_slots(file: &object::File, bytes: &[u8]) -> Vec<(u64, String)> {
    if file.format() != object::BinaryFormat::MachO {
        return Vec::new();
    }
    let chained = resolve_chained_fixups(file, bytes);
    let classic = resolve_dyld_info_bind_slots(file, bytes);
    let mut binds: Vec<(u64, String)> = chained
        .binds()
        .filter(|(_, bind)| bind.addend == 0)
        .map(|(slot, bind)| (slot, String::from_utf8_lossy(&bind.name).into_owned()))
        .chain(classic.iter().filter(|(_, (_, addend))| *addend == 0).map(|(&slot, (name, _))| (slot, name.to_string())))
        .collect();
    if binds.is_empty() {
        return binds;
    }
    let owned: Vec<(u64, u64)> = file
        .sections()
        .filter(|s| is_symbol_pointers(s) || s.name().is_ok_and(|n| n.starts_with("__objc_") || n == "__cfstring" || n == "__auth_ptr"))
        .map(|s| (s.address(), s.address().saturating_add(s.size())))
        .collect();
    binds.retain(|&(slot, _)| !owned.iter().any(|&(lo, hi)| slot >= lo && slot < hi));
    binds.sort_unstable();
    binds.dedup_by_key(|b| b.0);
    for (_, name) in &mut binds {
        if name.starts_with('_') {
            name.remove(0);
        }
    }
    binds
}

/// A section the indirect-symbol table names slot by slot
/// (`S_NON_LAZY_SYMBOL_POINTERS`, `S_LAZY_SYMBOL_POINTERS`).
fn is_symbol_pointers(section: &object::Section) -> bool {
    matches!(section.flags(), object::SectionFlags::MachO { flags } if matches!(flags & 0xff, 0x6 | 0x7))
}

/// Names each bound slot for its import and paints it `externref`.
pub struct MachoBindSlotsPass;

impl AnalysisPass for MachoBindSlotsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "machobinds"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let width = if ctx.file.is_64() { 8 } else { 4 };
        for (slot, name) in bound_slots(ctx.file, ctx.bytes) {
            let Some(end) = slot.checked_add(width) else { continue };
            out.externref.push((slot, end));
            out.import_slot_names.push((slot, name));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixture_binds_name_their_data_pointers() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_bindslots_arm64");
        let bytes = std::fs::read(path).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert_eq!(
            bound_slots(&file, &bytes),
            [(0x1_0000_4000, "ext_handler".to_string()), (0x1_0000_4008, "ext_counter".to_string())]
        );
    }
}
