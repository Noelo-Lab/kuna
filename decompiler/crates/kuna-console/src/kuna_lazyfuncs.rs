//! (kuna `lazyfuncs`) Materialize the `Function` objects one decompile reads.
//!
//! The commit installed the discovered entries as NAMES only, except the ones another
//! commit fact lands on, so after a lift this gives a `Function` to every parked entry
//! the p-code actually read -- a transfer (a tail call is a `BRANCH`, not a call spec)
//! and a code address used as a VALUE, which `ActionConstantPtr` only types and names
//! when the global Symbol exists -- and the caller re-lifts while it makes progress.

use std::collections::HashSet;

use kuna_analysis::pass::AnalysisOutput;
use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The entries the commit must still materialize eagerly: every start a later commit
/// step or a decompile-time reader attaches a fact to through the `Function` (a
/// committed name, a no-return flag, a FID rename, a parked prototype, locals,
/// comments, tracked registers, format sites).
pub(crate) fn fact_entries(out: &AnalysisOutput) -> HashSet<u64> {
    let mut eager: HashSet<u64> = HashSet::new();
    eager.extend(out.entry_names.iter().map(|(a, _)| *a));
    eager.extend(out.noreturn.iter().map(|f| f.addr));
    eager.extend(out.fid_names.iter().map(|f| f.addr));
    eager.extend(out.symbols.iter().map(|s| s.addr));
    eager.extend(out.prototypes_at.iter().map(|(a, _)| *a));
    eager.extend(out.main_decl_at.iter().map(|(a, _)| *a));
    eager.extend(out.arm_float_prototypes.iter().filter_map(|(a, _)| *a));
    eager.extend(out.locals.iter().map(|l| l.func_addr));
    eager.extend(out.comments.iter().map(|c| c.func_addr));
    eager.extend(out.tracked_regs.iter().map(|t| t.func_addr));
    eager.extend(out.format_sites.iter().map(|f| f.func));
    eager.extend(out.cpp_dwarf.symbols.iter().map(|s| s.addr));
    eager.extend(out.cpp_dwarf.locals.iter().map(|l| l.func_addr));
    eager.extend(out.cpp_dwarf.prototypes.iter().map(|(a, _)| *a));
    eager.extend(out.cpp_sig.proven.iter().map(|(a, _)| *a));
    eager.extend(out.cpp_sig.inferred.iter().map(|(a, _)| *a));
    eager.extend(out.cpp_sig.msvc.iter().map(|(a, _, _)| *a));
    eager.extend(out.cpp_sig.msvc_legacy.iter().map(|(a, _)| *a));
    eager.extend(out.cpp_sig.msvc_byval.iter().map(|(a, _)| *a));
    eager.extend(out.cpp_sig.msvc_builders.iter().map(|(a, _)| *a));
    eager
}

/// Give every lazy entry `lifted` read a `Function`; 0 means nothing was missing, so
/// the caller must not re-run.  Only parked entries are ever materialized, so the
/// symbol table never holds a `Function` the eager commit would not have built.
pub(crate) fn materialize_callees(arch: &mut Architecture, lifted: &KunaResult<Funcdata>) -> usize {
    let Ok(fd) = lifted else { return 0 };
    let Some(space) = fd.get_address().get_space().cloned() else { return 0 };
    let mut targets: Vec<u64> = transfers(fd)
        .into_iter()
        .chain(constants(fd))
        .filter_map(|vma| lazy_entry_for(arch, vma))
        .collect();
    targets.sort_unstable();
    targets.dedup();
    targets
        .into_iter()
        .filter(|&vma| ensure_function(arch, &Address::new(std::rc::Rc::clone(&space), vma)))
        .count()
}

/// The parked entry `vma` names: itself, or the even start of a Thumb code pointer.
fn lazy_entry_for(arch: &Architecture, vma: u64) -> Option<u64> {
    if arch.is_lazy_entry(vma) {
        Some(vma)
    } else if vma & 1 == 1 && arch.is_lazy_entry(vma & !1) {
        Some(vma & !1)
    } else {
        None
    }
}

/// `CALL`/`BRANCH` destinations and constant `CALLIND`/`BRANCHIND` targets outside the
/// body. A constant-space `BRANCH` operand is a p-code-relative offset, not a target.
fn transfers(fd: &Funcdata) -> Vec<u64> {
    let entry = fd.get_address().get_offset();
    let end = entry.saturating_add(fd.get_size().max(0) as u64);
    let mut out = Vec::new();
    for (_, id) in fd.obank().iter_all() {
        let Some(op) = fd.obank().get(id) else { continue };
        let direct = match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_BRANCH => true,
            OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND => false,
            _ => continue,
        };
        let Some(dest) = op.get_in(0).and_then(|v| fd.vbank().get(v)) else { continue };
        if dest.is_constant() != direct && !(entry..end).contains(&dest.get_offset()) {
            out.push(dest.get_offset());
        }
    }
    out
}

/// Every constant the body carries: a stored function pointer, a passed callback, a
/// handler table entry.
fn constants(fd: &Funcdata) -> Vec<u64> {
    fd.vbank()
        .iter_loc()
        .filter_map(|id| fd.vbank().get(id))
        .filter(|vn| vn.is_constant())
        .map(|vn| vn.get_offset())
        .collect()
}

/// Create the `Function` at `addr` when the symbol table has none -- the
/// `createFunctionIfMissing` step `read symbols` skipped for a lazy entry.
fn ensure_function(arch: &mut Architecture, addr: &Address) -> bool {
    if arch.symboltab.find_function_across_scopes(addr).is_some() {
        return false;
    }
    let name = arch.name_function(addr);
    let Ok(type_code) = arch.types().get_type_code() else { return false };
    let min_size = arch.min_funcsymbol_size;
    let num_spaces = arch.manage().num_spaces() as i32;
    let Ok((scope, base)) =
        arch.symboltab.find_create_scope_from_symbol_name(&name, "::", None, num_spaces)
    else {
        return false;
    };
    arch.symboltab.add_function(scope, addr, &base, min_size, type_code).is_ok()
}
