//! PE import-call binding (`peimportcall`) — bind a `call dword ptr [IAT slot]`
//! to the import symbol the loader already resolved at that slot.
//!
//! ## The gap
//!
//! [`crate::loader::pe_iat`] names every IAT slot, so `kuna functions` on a PE
//! lists `0x4112c4 ExitThread`. The decompiler never asks: a
//! `call dword ptr [0x4112c4]` lifts to a `CALLIND` whose target Varnode is the
//! *contents* of the global at the slot, and the one place that resolves such a
//! target — `ActionDeindirect` (`coreaction.cc:1235`) — gates its
//! `queryExternalRefFunction` arm on `Varnode::isPersist() &&
//! Varnode::isExternalRef()`. `persist` is already painted; `externref` is set
//! nowhere in the workspace, because Ghidra sets it from an `ExternRefSymbol`
//! (`Scope::addExternalRef`) that kuna's loader never creates. So every
//! `call [IAT]` in a PE stays an unnamed `(*dat_4112c4)(0)` — no name, no
//! prototype, and (the damaging part) no no-return flow effect, so a function
//! ending in `ExitThread(0)` runs on and absorbs the whole next function.
//!
//! ## What this pass does
//!
//! Two facts, both PE-only and both additive:
//!
//! 1. **`externref` over the IAT slots.** One `[slot, slot+ptr)` range per
//!    Import-Directory slot, committed into the symbol table's property map
//!    (`Database::setPropertyRange(Varnode::externref, …)`) exactly as the
//!    loader's read-only section ranges are. `Scope::queryProperties` ORs the
//!    property map into a global Varnode's flags, so the slot read now carries
//!    `persist|externref` and `ActionDeindirect` resolves it through the
//!    FunctionSymbol `pe_iat` already registered at the slot VA. (kuna's
//!    `query_function` keys on the Varnode's own address rather than upstream's
//!    `ExternRefSymbol::refaddr` indirection, so the slot-VA registration lines
//!    up without a second symbol.)
//! 2. **The Win32 no-return API names.** kuna's shipped
//!    `PeMacFunctionsThatDoNotReturn` carries only the shared CRT names; the
//!    Windows API exits (`ExitProcess`, `ExitThread`, `FreeLibraryAndExitThread`,
//!    `KeBugCheck`, `longjmp`, …) live in upstream's separate
//!    `PEFunctionsThatDoNotReturn`, vendored beside this module and matched here
//!    against a PE/COFF object only — a Mach-O never sees a Windows name set.
//!
//! Binding alone renames the call; the no-return names alone cannot reach a
//! CALLIND. Both are needed for the boundary, so both sit behind the one gate.

use object::read::{Object, ObjectSymbol};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Phase};

/// Upstream Ghidra's PE-only no-return name list, vendored at
/// `decompiler/crates/kuna-analysis/data/PEFunctionsThatDoNotReturn`.
const PE_WIN32_NORETURN_LIST: &str = include_str!("../../data/PEFunctionsThatDoNotReturn");

/// The PE import-call binding pass (gate id `peimportcall`).
#[derive(Clone, Copy, Default)]
pub struct PeImportCallPass;

impl AnalysisPass for PeImportCallPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "peimportcall"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if !matches!(ctx.file.format(), object::BinaryFormat::Pe | object::BinaryFormat::Coff) {
            return out;
        }
        out.externref = crate::loader::pe_iat::resolve_pe_import_slots(ctx.bytes);
        out.noreturn = scan_win32_noreturn(ctx.file, ctx.bytes);
        out
    }
}

/// Match the vendored Win32 no-return names over the same two symbol streams
/// `loader::noreturn::scan_noreturn` walks — the object's own defined text
/// symbols and the resolved imports (IAT slots + thunk veneers + exports) — so
/// the emitted address is the address the funcsym stream installed.
fn scan_win32_noreturn(file: &object::File, bytes: &[u8]) -> Vec<NoReturnFact> {
    let (exact, wildcard) = crate::loader::noreturn::parse_list(PE_WIN32_NORETURN_LIST);
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut emit = |out: &mut Vec<NoReturnFact>, addr: u64, n: String| {
        if crate::loader::noreturn::name_matches(&n, &exact, &wildcard)
            && seen.insert((addr, n.clone()))
        {
            out.push(NoReturnFact { addr, name: n });
        }
    };

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != object::SymbolKind::Text {
            continue;
        }
        let addr = sym.address();
        if addr == 0 {
            continue;
        }
        let Ok(n) = sym.name() else { continue };
        emit(&mut out, addr, n.to_string());
    }

    for p in crate::loader::format::resolve_imports(file, bytes) {
        let Ok(n) = String::from_utf8(p.name) else { continue };
        emit(&mut out, p.addr, n);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::noreturn::{name_matches, parse_list};

    /// The vendored upstream list carries the Win32 API exits kuna's merged
    /// PE/Mach-O list never named — the whole point of the widening.
    #[test]
    fn win32_list_names_the_api_exits() {
        let (exact, wildcard) = parse_list(PE_WIN32_NORETURN_LIST);
        for want in [
            "ExitProcess",
            "ExitThread",
            "FreeLibraryAndExitThread",
            "KeBugCheck",
            "KeBugCheckEx",
            "RpcRaiseException",
            "longjmp",
            "__crtExitProcess",
        ] {
            assert!(name_matches(want, &exact, &wildcard), "missing {want}");
        }
    }

    /// Windows API names that DO return must not be in the set (the list is
    /// upstream's, so this guards a bad hand-edit of the vendored file).
    #[test]
    fn win32_list_omits_returning_apis() {
        let (exact, wildcard) = parse_list(PE_WIN32_NORETURN_LIST);
        for nope in ["CreateThread", "GetTickCount", "HeapFree", "GetProcessHeap", "TerminateProcess"]
        {
            assert!(!name_matches(nope, &exact, &wildcard), "{nope} must not be no-return");
        }
    }
}
