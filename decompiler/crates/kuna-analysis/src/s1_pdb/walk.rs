//! Walk the global + module symbol streams of an opened `.pdb` and recover
//! function names + addresses.
//!
//! The kuna analog of Ghidra's `PdbUniversalAnalyzer` symbol-application loop
//! (`ApplySymbols` over the `SymbolGroup`s). For the NAME-level MVP (PR-P1) we read
//! the two function-bearing symbol kinds — `S_PUB32` (Public) and `S_GPROC32`
//! (Procedure) — resolve each to a VMA, and return `(vma, name)` pairs the
//! [`crate::s1_pdb::PdbPass`] turns into `out.fid_names` renames + `out.symbols`.
//!
//! Types/locals/lines are the deferred PR-P2/P3 (design §5): this is the NAME tier
//! — stripped `FUN_*` → the real source name.
//!
//! ## Address resolution
//!
//! A PDB stores a function's location as an internal `segment:offset` pair. The
//! `pdb` crate's `AddressMap` turns that into an **RVA** (`offset.to_rva(&map)`);
//! adding the PE's `ImageBase` gives the absolute **VMA** the engine's symbol table
//! is keyed by. We take the image base from the PE (the `s1_rtti` `pe_image_base`
//! shape) so the VMAs line up with the loaded image exactly.

use pdb::FallibleIterator;

/// One recovered function name at its absolute VMA.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdbFunc {
    /// The absolute virtual address of the function entry (`ImageBase + RVA`).
    pub vma: u64,
    /// The function name (already the real source name; PDB stores it un-mangled
    /// for `S_GPROC32`, decorated for some `S_PUB32` — kept verbatim, the printer /
    /// commit seam owns any demangling).
    pub name: String,
}

/// Walk an opened `.pdb`'s global symbol stream, returning the function
/// `(vma, name)` pairs (`S_PUB32` functions + `S_GPROC32` procedures).
///
/// `image_base` is the PE `ImageBase` (the RVA→VMA addend). Everything is total:
/// any `pdb` parse error on a single symbol is skipped (best-effort, faithful to
/// the never-fail analysis-pass contract); a stream-level error yields whatever was
/// collected so far. Procedures are preferred over publics at the same VMA (a
/// procedure carries the clean un-decorated name), deduped by the caller.
pub fn walk_functions(pdb: &mut pdb::PDB<'_, std::fs::File>, image_base: u64) -> Vec<PdbFunc> {
    let mut out: Vec<PdbFunc> = Vec::new();

    // The address map turns an internal segment:offset into an RVA. Without it we
    // cannot resolve any VMA, so its absence is the inert path.
    let Ok(address_map) = pdb.address_map() else {
        return out;
    };

    // (1) Global symbol stream: the program-wide S_PUB32 / S_GPROC32 publics +
    // procedures (the bulk of a stripped binary's recoverable names).
    if let Ok(symbols) = pdb.global_symbols() {
        collect_symbols(&symbols, &address_map, image_base, &mut out);
    }

    out
}

/// Collect the function symbols from one `SymbolTable` into `out`.
fn collect_symbols(
    symbols: &pdb::SymbolTable<'_>,
    address_map: &pdb::AddressMap<'_>,
    image_base: u64,
    out: &mut Vec<PdbFunc>,
) {
    let mut iter = symbols.iter();
    // `FallibleIterator::next` yields `Result<Option<Symbol>>`; a per-symbol parse
    // error stops THIS stream (the stream is corrupt past this point) but keeps what
    // we have — never panics, never fails the pass.
    while let Ok(Some(symbol)) = iter.next() {
        let Ok(data) = symbol.parse() else {
            continue;
        };
        match data {
            // S_PUB32: a public symbol. Only the `function` ones name code (the
            // others are data — deferred to the type/data tier).
            pdb::SymbolData::Public(pubsym) if pubsym.function => {
                if let Some(rva) = pubsym.offset.to_rva(address_map) {
                    out.push(PdbFunc {
                        vma: image_base.wrapping_add(u64::from(rva.0)),
                        name: pubsym.name.to_string().into_owned(),
                    });
                }
            }
            // S_GPROC32: a procedure (a defined function with a body). Its name is
            // the clean source name.
            pdb::SymbolData::Procedure(proc) => {
                if let Some(rva) = proc.offset.to_rva(address_map) {
                    out.push(PdbFunc {
                        vma: image_base.wrapping_add(u64::from(rva.0)),
                        name: proc.name.to_string().into_owned(),
                    });
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdb_func_eq() {
        let a = PdbFunc { vma: 0x401000, name: "main".into() };
        let b = PdbFunc { vma: 0x401000, name: "main".into() };
        assert_eq!(a, b);
    }
}
