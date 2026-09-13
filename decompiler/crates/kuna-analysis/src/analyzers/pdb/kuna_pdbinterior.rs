//! (kuna) `pdbinterior` — never start a function inside a procedure the PDB
//! describes (P1 code/data partition).
//!
//! The PDB half of [`crate::entry::kuna_pdatainterior`], for the functions the
//! exception table cannot vouch for. A kuna `FunctionSymbol` is an entry address
//! with no extent, so the commit boundary cannot answer "is this candidate already
//! inside a known function?". `.pdata` answers it for every function that needs
//! unwind data, but a frameless leaf has no `RUNTIME_FUNCTION` at all. When MSVC
//! also inlined that leaf into its only caller, the out-of-line copy has no caller
//! either, so recursive descent never decodes it and the PDB is the only record of
//! where it starts and ends.
//!
//! ## What goes wrong without it
//!
//! The reporter's MSVC `/O2` testbed (`ab_o2.exe`) has `cascade_switch` at
//! `0x1400010e0`, a 246-byte leaf of exactly that kind. The aggressive gap walk
//! (`aif`) probes its undecoded bytes and accepts `sub_140001178` (the middle of a
//! 6-byte `cmp`) and `sub_1400011bc` (a real instruction, but a fall-through inside
//! the switch). `funcboundflow` then truncates `cascade_switch` at the first of
//! them, and the `0xf043` case disappears from the emitted C. Across that image
//! and its `/Od` twin, 136 and 134 discovered starts sit strictly inside a PDB
//! procedure, almost all in CRT leaves and thunks with no `.pdata` record.
//!
//! ## What makes a range eligible
//!
//! Every `S_GPROC32`/`S_LPROC32` record carries its code length, which is the
//! extent the symbol table never had. A procedure's `[start, start + len)` is used
//! under the same three-part test `.pdata` records pass: no named function start
//! (image symbol or PDB public) strictly inside it, no other procedure's start
//! strictly inside it, and no overlap with the range kept before it. A `.pdata`
//! `BeginAddress` inside a procedure deliberately does not disqualify it: MSVC
//! splits one function's unwind data across several chained records, and the PDB
//! procedure is the authority on where that function ends.
//!
//! ## Scope
//!
//! A PE whose fingerprint-matched `.pdb` is found (the `pdb` pass's search and
//! GUID/age gate, reused), on x86, x64 and ARM64. 32-bit ARM abstains: a Thumb
//! entry can carry the interworking bit, which would place an entry that IS a
//! procedure start one byte inside it. The commit applies the bodies only while
//! `pdb` is also on, so switching the PDB off removes every trace of it.

use object::read::Object;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{codeview, locate, open_fingerprint_matched, pe_image_base, walk};

/// (kuna) The PDB-procedure-interior entry-suppression pass (`pdbinterior`).
///
/// Reports the eligible procedure extents on [`AnalysisOutput::fde_bodies`], the
/// channel `fdeinterior` and `pdatainterior` share; the commit applies them with
/// [`crate::entry::kuna_fdeinterior::suppress_interior_entries`].
pub struct PdbInteriorPass;

impl AnalysisPass for PdbInteriorPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "pdbinterior"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput {
            fde_bodies: eligible_pdb_bodies(ctx.file, ctx.bytes, ctx.image_path),
            ..AnalysisOutput::default()
        }
    }
}

/// The PDB procedure extents that describe exactly one function, sorted and
/// disjoint. Empty unless `file` is an x86/x64/ARM64 PE whose fingerprint-matched
/// `.pdb` can be located from `image_path`.
pub fn eligible_pdb_bodies(
    file: &object::File,
    bytes: &[u8],
    image_path: Option<&std::path::Path>,
) -> Vec<(u64, u64)> {
    if file.format() != object::BinaryFormat::Pe
        || !matches!(
            file.architecture(),
            object::Architecture::I386 | object::Architecture::X86_64 | object::Architecture::Aarch64
        )
    {
        return Vec::new();
    }
    let Some(cv) = codeview::extract_codeview(bytes) else {
        return Vec::new();
    };
    let candidates = locate::pdb_candidates(image_path, &cv);
    let (Some(image_base), false) = (pe_image_base(bytes), candidates.is_empty()) else {
        return Vec::new();
    };
    let Some(mut pdb) = open_fingerprint_matched(&candidates, &cv) else {
        return Vec::new();
    };
    let procs = walk::walk_procedure_extents(&mut pdb, image_base);
    if procs.is_empty() {
        return procs;
    }
    let mut named = crate::entry::existing_function_addrs(file, bytes);
    named.extend(walk::walk_functions(&mut pdb, image_base).into_iter().map(|f| f.vma));
    named.sort_unstable();
    named.dedup();
    crate::entry::kuna_pdatainterior::eligible(&procs, &named)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::kuna_fdeinterior::suppress_interior_entries;

    const LEAF: u64 = 0x1400011c0;
    const LEAF_END: u64 = 0x1400011f8;
    const PHANTOM: u64 = 0x1400011d0;
    const PICK_09: u64 = 0x1400011a0;

    fn fixture_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    fn bodies_for(exe: &std::path::Path) -> Vec<(u64, u64)> {
        let bytes = std::fs::read(exe).unwrap_or_else(|e| panic!("read {exe:?}: {e}"));
        let file = object::File::parse(&*bytes).expect("the fixture parses");
        eligible_pdb_bodies(&file, &bytes, Some(exe))
    }

    /// The sidecar's `S_GPROC32` length bounds the leaf, and the gap walk's start
    /// inside it is exactly what the suppression removes.
    #[test]
    fn the_leaf_extent_comes_from_the_pdb() {
        let bodies = bodies_for(&fixture_dir().join("pe_pdbinterior_x86_64.exe"));
        assert_eq!(bodies.len(), 22, "one body per procedure, got {bodies:#x?}");
        assert!(bodies.contains(&(LEAF, LEAF_END)), "leaf extent missing: {bodies:#x?}");
        let mut entries = vec![PICK_09, LEAF, PHANTOM, LEAF_END];
        assert_eq!(suppress_interior_entries(&mut entries, &bodies), vec![PHANTOM]);
        assert_eq!(entries, vec![PICK_09, LEAF, LEAF_END]);
    }

    /// With no `.pdb` beside the image there is nothing to vouch for a body.
    #[test]
    fn no_sidecar_abstains() {
        let dir = std::env::temp_dir().join(format!("kuna-pdbinterior-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let exe = dir.join("pe_pdbinterior_x86_64.exe");
        std::fs::copy(fixture_dir().join("pe_pdbinterior_x86_64.exe"), &exe).expect("copy");
        let bodies = bodies_for(&exe);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(bodies.is_empty(), "no sidecar, yet {bodies:#x?}");
    }

    /// A 32-bit ARM image abstains before the PDB is opened.
    #[test]
    fn arm32_abstains() {
        let exe = fixture_dir().join("pe_pdbinterior_x86_64.exe");
        let mut bytes = std::fs::read(&exe).expect("read fixture");
        let lfanew = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        bytes[lfanew + 4..lfanew + 6]
            .copy_from_slice(&object::pe::IMAGE_FILE_MACHINE_ARMNT.to_le_bytes());
        let file = object::File::parse(&*bytes).expect("an ARMNT PE still parses");
        assert!(eligible_pdb_bodies(&file, &bytes, Some(&exe)).is_empty());
    }
}
