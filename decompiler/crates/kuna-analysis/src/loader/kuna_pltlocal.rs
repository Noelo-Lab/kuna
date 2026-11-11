//! (kuna) `pltlocal` — the PLT stubs a linked ELF's own relocations bind to a
//! function the same image defines.
//!
//! A shared library that calls one of its own default-visibility functions
//! calls it through `name@plt`, so the symbol can be interposed at run time.
//! [`crate::loader::elf_plt`] names that stub after the relocation's symbol and
//! registers it as an import, which tells the decompiler nothing about the
//! callee: it has no prototype, no body to read, and the definition the image
//! carries is a different address. A `-Bsymbolic` or hidden-visibility build of
//! the same code calls the definition directly.
//!
//! The join is the relocation, never a shared name: the slot the decoded stub
//! jumps through carries a symbol-bearing dynamic relocation, and that symbol is
//! a defined `STT_FUNC` (an `STT_GNU_IFUNC`'s value is its resolver, not the
//! function). This is the same binding
//! [`crate::listing::xrefs::XrefIndex::export_alias_class`] reads for `kuna xrefs`.

use std::collections::HashMap;

use object::read::{Object, ObjectSymbol, ObjectSymbolTable};
use object::RelocationTarget;

use super::elf_plt::PltSym;

/// `STT_FUNC`.
const STT_FUNC: u8 = 2;

/// `(stub, definition)` for every stub in `syms` whose slot's dynamic
/// relocation names a function this image defines, ascending by stub.
pub(crate) fn bind(file: &object::File, syms: &[PltSym]) -> Vec<(u64, u64)> {
    if !syms.iter().any(|s| s.slot.is_some()) {
        return Vec::new();
    }
    let (Some(relocs), Some(table)) = (file.dynamic_relocations(), file.dynamic_symbol_table()) else {
        return Vec::new();
    };
    let mut by_slot: HashMap<u64, usize> = HashMap::new();
    for (slot, reloc) in relocs {
        if let RelocationTarget::Symbol(index) = reloc.target() {
            by_slot.entry(slot).or_insert(index.0);
        }
    }
    let thumb = file.architecture() == object::Architecture::Arm;
    let mut out: Vec<(u64, u64)> = Vec::new();
    for sym in syms {
        let Some(&index) = sym.slot.as_ref().and_then(|slot| by_slot.get(slot)) else { continue };
        let Ok(def) = table.symbol_by_index(object::SymbolIndex(index)) else { continue };
        let object::SymbolFlags::Elf { st_info, .. } = def.flags() else { continue };
        if !def.is_definition() || st_info & 0xf != STT_FUNC || def.address() == 0 {
            continue;
        }
        let target = if thumb { def.address() & !1 } else { def.address() };
        let stub = if thumb { sym.addr & !1 } else { sym.addr };
        if target != stub {
            out.push((stub, target));
        }
    }
    out.sort_unstable();
    out.dedup_by_key(|(stub, _)| *stub);
    out
}

#[cfg(test)]
mod tests;
