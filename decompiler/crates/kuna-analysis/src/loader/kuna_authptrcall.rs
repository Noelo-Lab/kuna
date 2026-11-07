//! (kuna `authptrcall`) Bind a call through an arm64e `__auth_ptr` slot to the
//! import its chained bind names.
//!
//! An arm64e link keeps two pointer tables for imported functions. `__auth_got`
//! backs the `__auth_stubs` a direct `bl` reaches, and is listed in the
//! `LC_DYSYMTAB` indirect-symbol table like `__got`, so the stub and the slot
//! are named and `peimportcall` binds a call through the slot. `__auth_ptr`
//! holds the signed pointers code loads itself, for instance the
//! `adrp/ldr/blraaz` sequence clang emits for `___chkstk_darwin`. It is a plain
//! `S_REGULAR` section with no indirect-symbol entries, so its slots are named
//! only by their `LC_DYLD_CHAINED_FIXUPS` bind, and such a call rendered
//! `(*dat_1000716c0)(...)`.
//!
//! This pass names every bind slot of a `__auth_ptr` section after its import
//! (one leading `_` stripped, as the indirect-symbol walk does) and paints it
//! `externref`, so `ActionDeindirect` resolves the call exactly as it does
//! through `__auth_got`. A bind with an addend points inside the import rather
//! than at it and is skipped. Inert on any image without a `__auth_ptr` bind.

use object::read::{Object, ObjectSection};

use crate::loader::format::macho::resolve_chained_fixups;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, SymFact, SymKind};

/// The arm64e section of signed pointers to imported functions.
const AUTH_PTR_SECTION: &str = "__auth_ptr";

/// The `__auth_ptr` call binding pass (gate id `authptrcall`).
#[derive(Clone, Copy, Default)]
pub struct AuthPtrCallPass;

impl AnalysisPass for AuthPtrCallPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "authptrcall"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        auth_ptr_binds(ctx.file, ctx.bytes)
    }
}

/// The import symbol and `externref` range of every addend-free bind slot in a
/// `__auth_ptr` section of `file`.
fn auth_ptr_binds(file: &object::File, bytes: &[u8]) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    if file.format() != object::BinaryFormat::MachO {
        return out;
    }
    let ranges: Vec<(u64, u64)> = file
        .sections()
        .filter(|s| s.name() == Ok(AUTH_PTR_SECTION))
        .map(|s| (s.address(), s.address().saturating_add(s.size())))
        .collect();
    if ranges.is_empty() {
        return out;
    }
    let fixups = resolve_chained_fixups(file, bytes);
    let width: u64 = if file.is_64() { 8 } else { 4 };
    let mut slots: Vec<(u64, &str)> = fixups
        .binds()
        .filter(|&(va, _)| ranges.iter().any(|&(lo, hi)| va >= lo && va < hi))
        .filter(|(_, bind)| bind.addend == 0)
        .filter_map(|(va, bind)| std::str::from_utf8(&bind.name).ok().map(|name| (va, name)))
        .collect();
    slots.sort_unstable();
    for (va, name) in slots {
        let name = name.strip_prefix('_').unwrap_or(name);
        let Some(end) = va.checked_add(width).filter(|_| !name.is_empty()) else { continue };
        out.symbols.push(SymFact { addr: va, name: name.to_string(), kind: SymKind::Function });
        out.externref.push((va, end));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `macho_arm64e_auth`'s one `__auth_ptr` slot binds `_ext_func`; the
    /// `__const` binds beside it are not this pass's.
    #[test]
    fn names_the_auth_ptr_bind() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_arm64e_auth");
        let bytes = std::fs::read(path).expect("read fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        let out = auth_ptr_binds(&file, &bytes);
        let symbols: Vec<(u64, &str, SymKind)> =
            out.symbols.iter().map(|s| (s.addr, s.name.as_str(), s.kind)).collect();
        assert_eq!(symbols, vec![(0x100004018, "ext_func", SymKind::Function)]);
        assert_eq!(out.externref, vec![(0x100004018, 0x100004020)]);
    }

    /// A `__auth_ptr` header mapped at the top of the address space bounds its
    /// range instead of overflowing it.
    #[test]
    fn auth_ptr_section_at_the_top_of_the_address_space() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_arm64e_auth");
        let mut bytes = std::fs::read(path).expect("read fixture");
        let u32_at = |b: &[u8], o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap()) as usize;
        let mut off = 32;
        for _ in 0..u32_at(&bytes, 16) {
            if u32_at(&bytes, off) == 0x19 {
                for j in 0..u32_at(&bytes, off + 64) {
                    let s = off + 72 + 80 * j;
                    if bytes[s..s + 16].starts_with(AUTH_PTR_SECTION.as_bytes()) {
                        bytes[s + 32..s + 40].copy_from_slice(&(u64::MAX - 3).to_le_bytes());
                    }
                }
            }
            off += u32_at(&bytes, off + 4);
        }
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert!(auth_ptr_binds(&file, &bytes).externref.is_empty());
    }
}
