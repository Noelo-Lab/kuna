//! (kuna `lazyfuncs`) Materialize the `Function` objects one decompile reads.
//!
//! The commit installed the discovered entries as NAMES only, so after a lift this
//! gives a `Function` to every target the p-code actually read -- a transfer (a tail
//! call is a `BRANCH`, not a call spec) and a code address used as a VALUE, which
//! `ActionConstantPtr` only types and names when the global Symbol exists -- and the
//! caller re-lifts while it makes progress.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// Give every target `lifted` read that lacks a `Function` one; 0 means nothing was
/// missing, so the caller must not re-run.
pub(crate) fn materialize_callees(arch: &mut Architecture, lifted: &KunaResult<Funcdata>) -> usize {
    let Ok(fd) = lifted else { return 0 };
    let Some(space) = fd.get_address().get_space().cloned() else { return 0 };
    let mut targets = transfers(fd);
    targets.extend(taken_addresses(arch, fd));
    targets.sort_unstable();
    targets.dedup();
    targets
        .into_iter()
        .filter(|&vma| ensure_function(arch, &Address::new(std::rc::Rc::clone(&space), vma)))
        .count()
}

/// Constant `CALL`/`CALLIND`/`BRANCH`/`BRANCHIND` destinations outside the body: the
/// callees and tail calls this lift referenced (an internal edge is not a callee).
fn transfers(fd: &Funcdata) -> Vec<u64> {
    let entry = fd.get_address().get_offset();
    let end = entry.saturating_add(fd.get_size().max(0) as u64);
    let mut out = Vec::new();
    for (_, id) in fd.obank().iter_all() {
        let Some(op) = fd.obank().get(id) else { continue };
        if !matches!(
            op.code(),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND
        ) {
            continue;
        }
        let Some(dest) = op.get_in(0).and_then(|v| fd.vbank().get(v)) else { continue };
        if dest.is_constant() && !(entry..end).contains(&dest.get_offset()) {
            out.push(dest.get_offset());
        }
    }
    out
}

/// The lazily-committed entries the body names by value: a stored function pointer, a
/// passed callback, a handler table.
fn taken_addresses(arch: &Architecture, fd: &Funcdata) -> Vec<u64> {
    fd.vbank()
        .iter_loc()
        .filter_map(|id| fd.vbank().get(id))
        .filter(|vn| vn.is_constant() && arch.is_lazy_entry(vn.get_offset()))
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
