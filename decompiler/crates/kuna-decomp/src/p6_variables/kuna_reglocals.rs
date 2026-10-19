//! (kuna `reglocals`) The locals the C declares outside the stack frame, on the
//! `decompile-all --json` `variables` surface (P6, JSON only).
//!
//! `extract_variables` reports the parameters and the stack frame.  A local the
//! C body declares whose storage is a register, a temporary, or nothing at all
//! (the `vN` of an optimized function) is in neither, so the surface reported no
//! row for it while the C declared it.  [`register_locals`] reports exactly the
//! declarations the printer just emitted that no parameter or frame row covers
//! and that are not stack storage, under the printed name, so a consumer can
//! join each row to its declaration line, and the declared type spelled as the
//! other rows spell theirs.  [`DeclarationHighs`] gives each row the uses of every
//! HighVariable its declaration covers, the same markup-backed evidence the
//! other rows carry.
//!
//! Read from the printer after the `print_c` of the same function: nothing here
//! feeds the IR or the emitted C.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;

use crate::context::HighVariableId;
use crate::decompile_drive::VarInfo;
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::printc::{DeclaredLocal, PrintC};

/// Is `high` a frame object: bound to a stack Symbol, stack storage only (or
/// none but the address constants of a frame object), or a stack instance
/// inside a reported frame row?
fn frame_object(
    fd: &Funcdata,
    high: HighVariableId,
    stack_space: Option<&Rc<AddrSpace>>,
    rows: &[(i64, i64)],
) -> bool {
    let Some(h) = fd.high_bank().get(high) else { return true };
    let in_stack = |address: &kuna_base::address::Address| {
        stack_space.is_some_and(|stack| address.get_space().map(|s| s.get_index()) == Some(stack.get_index()))
    };
    if let (Some(scope), Some(symbol)) = (fd.get_scope_local(), h.kuna_link_symbol()) {
        if scope.symbol_storage(symbol).iter().any(|(address, _)| in_stack(address)) {
            return true;
        }
    }
    let mut off_stack = false;
    for i in 0..h.num_instances() {
        let Some(v) = fd.vbank().get(h.get_instance(i)) else { continue };
        if v.is_constant() {
            continue;
        }
        let Some(stack) = stack_space.filter(|_| in_stack(v.get_addr())) else {
            off_stack = true;
            continue;
        };
        let lo = crate::decompile_drive::signed_space_offset(stack, v.get_offset());
        let hi = lo + i64::from(v.get_size());
        if rows.iter().any(|&(a, b)| a < hi && lo < b) {
            return true;
        }
    }
    !off_stack
}

/// Does every storage home the declaration's comment lists (`highs` are the
/// declared high and the highs declared through it) lie on the stack?
fn stack_homes_only(
    fd: &Funcdata,
    arch: &Architecture,
    highs: &[HighVariableId],
    stack_space: Option<&Rc<AddrSpace>>,
) -> bool {
    let Some(stack) = stack_space else { return false };
    let mut homes = highs
        .iter()
        .filter_map(|&high| crate::kuna_varsources::high_homes(fd, arch, high))
        .flat_map(|(homes, _)| homes.into_values())
        .peekable();
    homes.peek().is_some() && homes.all(|(space, _)| space.get_index() == stack.get_index())
}

fn widest_instance(fd: &Funcdata, high: HighVariableId) -> i64 {
    let Some(h) = fd.high_bank().get(high) else { return 0 };
    (0..h.num_instances())
        .filter_map(|i| fd.vbank().get(h.get_instance(i)))
        .map(|v| i64::from(v.get_size()))
        .max()
        .unwrap_or(0)
}

/// The declarations of the last document `print` printed, when it printed `fd`.
pub(crate) fn declarations_of<'a>(print: &'a PrintC, fd: &Funcdata) -> &'a [DeclaredLocal] {
    if print.declared_entry() != Some(fd.get_address()) {
        return &[];
    }
    print.declared_locals()
}

/// Does `local` still name a high of `fd` the naming pass gave its base name?
pub(crate) fn declares_high_of(fd: &Funcdata, local: &DeclaredLocal) -> bool {
    fd.high_bank().get(local.high).and_then(|h| h.kuna_name()) == Some(local.base.as_str())
}

/// The rows for the locals `arch`'s printer declared for `fd` that are neither
/// frame objects nor already in `reported` by name, in declaration order.
/// Empty unless the printer's last document was `fd`.
pub fn register_locals(arch: &Architecture, fd: &Funcdata, reported: &[VarInfo]) -> Vec<VarInfo> {
    let taken: HashSet<&str> = reported.iter().map(|v| v.name.as_str()).collect();
    let rows: Vec<(i64, i64)> = reported
        .iter()
        .filter_map(|v| v.stack_offset.map(|off| (off, off + v.size.max(1))))
        .collect();
    let stack_space = fd.get_scope_local().map(|scope| scope.get_space_id());
    let through = arch.print().declared_through();
    declarations_of(arch.print(), fd)
        .iter()
        .filter(|local| declares_high_of(fd, local))
        .filter(|local| !taken.contains(local.name.as_str()))
        .filter(|local| !frame_object(fd, local.high, stack_space, &rows))
        .filter(|local| {
            let mut highs = vec![local.high];
            highs.extend(through.get(&local.high).into_iter().flatten());
            !stack_homes_only(fd, arch, &highs, stack_space)
        })
        .map(|local| {
            let type_name = match &local.ty {
                Some(ty) => crate::kuna_bytehonest::exported_type_name(arch, ty),
                None => local.decl.clone(),
            };
            let size = local
                .ty
                .as_ref()
                .map(|ty| i64::from(ty.get_size()))
                .unwrap_or_else(|| widest_instance(fd, local.high));
            VarInfo {
                name: local.name.clone(),
                type_name,
                stack_offset: None,
                size,
                is_param: false,
                arg_index: None,
                line_numbers: Vec::new(),
                addresses: Vec::new(),
            }
        })
        .collect()
}

/// Each printed declaration's name and the highs it covers: the declared high
/// and every high the printer declared through it.
pub struct DeclarationHighs(BTreeMap<String, Vec<HighVariableId>>);

impl DeclarationHighs {
    /// The declarations of `print`'s last document; empty unless it printed `fd`.
    pub fn of(print: &PrintC, fd: &Funcdata) -> Self {
        let mut through = print.declared_through();
        DeclarationHighs(
            declarations_of(print, fd)
                .iter()
                .map(|local| {
                    let mut highs = vec![local.high];
                    highs.extend(through.remove(&local.high).unwrap_or_default());
                    (local.name.clone(), highs)
                })
                .collect(),
        )
    }

    /// The varrefs of every instance of the highs the declaration `name` covers.
    pub fn varrefs(&self, fd: &Funcdata, name: &str) -> BTreeSet<u64> {
        let mut out = BTreeSet::new();
        for &high in self.0.get(name).into_iter().flatten() {
            let Some(h) = fd.high_bank().get(high) else { continue };
            for i in 0..h.num_instances() {
                if let Some(v) = fd.vbank().get(h.get_instance(i)) {
                    out.insert(v.get_create_index() as u64);
                }
            }
        }
        out
    }
}
