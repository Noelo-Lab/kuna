//! Walk the global + module symbol streams of an opened `.pdb` and recover
//! function names + addresses.
//!
//! The kuna analog of Ghidra's `PdbUniversalAnalyzer` symbol-application loop
//! (`ApplySymbols` over the `SymbolGroup`s). For the NAME-level MVP (PR-P1) we read
//! the two function-bearing symbol kinds — `S_PUB32` (Public) and `S_GPROC32`
//! (Procedure) — resolve each to a VMA, and return `(vma, name)` pairs the
//! [`crate::pdb::PdbPass`] turns into `out.fid_names` renames + `out.symbols`.
//!
//! Types/locals/lines are the deferred PR-P2/P3 (design §5): this is the NAME tier
//! — stripped `FUN_*` → the real source name.
//!
//! ## Address resolution
//!
//! A PDB stores a function's location as an internal `segment:offset` pair. The
//! `pdb` crate's `AddressMap` turns that into an **RVA** (`offset.to_rva(&map)`);
//! adding the PE's `ImageBase` gives the absolute **VMA** the engine's symbol table
//! is keyed by. We take the image base from the PE (the `rtti` `pe_image_base`
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

/// The `S_LPROC32`/`S_GPROC32` record kinds (plain, `_ST`, `_ID` and DPC forms),
/// checked before a record is parsed; the `pdb` crate does not export its kind
/// constants, and a module stream is mostly locals and ranges.
const PROCEDURE_KINDS: [u16; 8] = [0x100a, 0x100b, 0x110f, 0x1110, 0x1146, 0x1147, 0x1155, 0x1156];

/// One module-stream procedure record, global or local, at absolute VMAs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PdbProcedure {
    /// The procedure's first address.
    pub start: u64,
    /// One past its last code byte, or `None` when the record gives no usable
    /// extent: a zero code length, a length that overflows the address space, or
    /// code the PDB's address map does not place as one contiguous range.
    pub end: Option<u64>,
}

/// Walk every module stream of an opened `.pdb` and return its procedure
/// records, sorted and deduplicated.
///
/// The global stream holds only references to procedures; the records that carry a
/// code length live in the per-module streams, so this is the only place a
/// procedure's end is recorded. A record whose start does not resolve to an RVA is
/// skipped; a module whose stream fails to parse contributes whatever it yielded
/// before the failure.
pub fn walk_procedures(
    pdb: &mut pdb::PDB<'_, std::fs::File>,
    image_base: u64,
) -> Vec<PdbProcedure> {
    let mut out: Vec<PdbProcedure> = Vec::new();
    let (Ok(address_map), Ok(dbi)) = (pdb.address_map(), pdb.debug_information()) else {
        return out;
    };
    let Ok(mut modules) = dbi.modules() else {
        return out;
    };
    while let Ok(Some(module)) = modules.next() {
        let Ok(Some(info)) = pdb.module_info(&module) else {
            continue;
        };
        let Ok(mut symbols) = info.symbols() else {
            continue;
        };
        while let Ok(Some(symbol)) = symbols.next() {
            if !PROCEDURE_KINDS.contains(&symbol.raw_kind()) {
                continue;
            }
            let Ok(pdb::SymbolData::Procedure(proc)) = symbol.parse() else {
                continue;
            };
            let Some(internal) = proc.offset.to_internal_rva(&address_map) else {
                continue;
            };
            let Some(rva) = internal.to_rva(&address_map) else {
                continue;
            };
            let Some(start) = image_base.checked_add(u64::from(rva.0)) else {
                continue;
            };
            let end = contiguous_length(&address_map, internal, rva, proc.len)
                .and_then(|len| start.checked_add(len));
            out.push(PdbProcedure { start, end });
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// `len` when the `len` code bytes at PDB-internal `internal` map to exactly one
/// range of the image beginning at `rva`. Without an OMAP that is every non-empty,
/// non-overflowing record; with one, a procedure the post-link optimizer split or
/// moved piecewise has no single extent.
fn contiguous_length(
    map: &pdb::AddressMap<'_>,
    internal: pdb::PdbInternalRva,
    rva: pdb::Rva,
    len: u32,
) -> Option<u64> {
    let internal_end = pdb::PdbInternalRva(internal.0.checked_add(len)?);
    let mut ranges = map.rva_ranges(internal..internal_end);
    let only = ranges.next()?;
    let whole = ranges.next().is_none()
        && only.start == rva
        && only.end.0.checked_sub(only.start.0) == Some(len);
    (len != 0 && whole).then_some(u64::from(len))
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

    /// Global and local procedure records both come back, and a code length that
    /// overflows the RVA space yields a start with no end.
    #[test]
    fn procedures_carry_their_extent_or_none() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pe_pdbinterior_x86_64.pdb");
        let file = std::fs::File::open(path).expect("open the fixture pdb");
        let mut pdb = pdb::PDB::open(file).expect("parse the fixture pdb");
        let procs = walk_procedures(&mut pdb, 0x140000000);
        assert_eq!(procs.len(), 34, "{procs:#x?}");
        for (start, end) in [
            (0x1400011c0, Some(0x1400011f8)),
            (0x140001340, Some(0x140001378)),
            (0x140001540, None),
            (0x140001548, Some(0x140001568)),
            (0x140002008, Some(0x140002010)),
        ] {
            assert!(procs.contains(&PdbProcedure { start, end }), "{start:#x}: {procs:#x?}");
        }
    }
}
