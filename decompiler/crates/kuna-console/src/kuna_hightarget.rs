//! Resolving a symbol-scoped directive against the name the EMITTER invented.
//!
//! `rename`/`retype` — and the `--assert name`/`type` directives that lower onto
//! them — resolve their target with `ScopeLocal::query_by_name`, so they reach
//! exactly the locals a `Symbol` backs: the stack slots and the parameters.
//! Every register-resident local kuna prints (`v6 // rax`, `v2 // al`) is a
//! HighVariable the naming pass named directly, with nothing in the scope behind
//! it, so the one plane an agent states facts through cannot name it — while
//! `kuna decompile --help` teaches `type v2 char[16]` on exactly such a name.
//!
//! The console already owns the machinery this needs.  `type varnode %RAX(pc)
//! <type>` maps an isolated, locked Symbol over a register at a use address, and
//! `linkSymbol`'s `query_container_for_link(addr, vn->getUsePoint())` binds it to
//! the high that reads that storage there.  What was missing is the translation
//! from the identifier the printer chose to that `(storage, usepoint)` pair.
//! This module is that translation and nothing else: it adds no decision to the
//! pipeline, so it carries no option.
//!
//! # The usepoint is load-bearing
//!
//! Mapping the Symbol with an INVALID usepoint — the whole-scope mapping an
//! ordinary stack local gets — is not a harmless simplification.  Its
//! `SymbolEntry::inUse` then matches every read of the register in the function,
//! the naming pass binds it to more than one high, and the printer emits the
//! storage twice: `uint8 *v6; // rax` next to a bare `uint8 v6;`, i.e. a
//! redeclaration, in a body that goes on to use both.  Measured on the witness
//! (`sub_1005350` of the `graphy` VM).  So a target carries the representative's
//! own use point, which is the address `linkSymbol` queries with.
//!
//! # A batch reads the names it was shown
//!
//! Every `name`/`type` applied between two decompiles is one batch, and each
//! one's identifier is read against the output that batch started from, not
//! against what the directives before it did.  Mapping a Symbol used to clear
//! the analysis, which emptied the HighVariables the NEXT directive resolves
//! against, so of two independent renames the second always answered `No symbol
//! named:` -- in either order.  The pass is now left intact and
//! [`Funcdata::kuna_directive_symbols`] keeps every Symbol the batch touched
//! keyed by the identifier the pass printed for it, which gives [`resolve_local`]
//! two readings of an identifier:
//!
//! 1. the variable the pass printed under it, whatever an earlier directive in
//!    the batch renamed it to -- so directives on different variables do not
//!    depend on their order, and `name a b` with `name b a` is a swap;
//! 2. failing that, the variable an earlier directive in the batch gave that
//!    name, so `name v1 rc` followed by `type rc unsigned int` retypes `rc`.
//!
//! The first reading wins where both exist, because it is the one the caller
//! could see.  Two register locals whose storage overlaps (`char *s // rax`
//! then `uint4 v1 // eax`) cannot both be given a Symbol in one batch -- the
//! second pass merges them into one variable -- so the later of the two is
//! rejected, naming the earlier.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_CONSTANT, IPTR_INTERNAL, IPTR_JOIN};
use kuna_base::types::int4;
use kuna_decomp::database::{symbol_category, SymbolId};
use kuna_decomp::dtype::Datatype;
use kuna_decomp::funcdata::{DirectiveSymbol, Funcdata};
use kuna_decomp::varnode::varnode_flags;

/// The storage a printed local occupies, in the shape `Scope::addSymbol` takes.
pub struct PrintedLocal {
    /// The name representative's storage address (`%RAX`, a unique-space temp).
    pub addr: Address,
    /// The width of that storage, which a stated type has to match.
    pub size: int4,
    /// `Varnode::getUsePoint` of that representative — the def-op's address, or
    /// the entry address minus one for a function input.
    pub usepoint: Address,
    /// The type the representative carries today, so a `name` directive can
    /// rename without also restating the type.
    pub dtype: Rc<Datatype>,
}

/// What a `name`/`type` identifier resolved to.
pub enum LocalTarget {
    /// A Symbol: a stack slot, a parameter, or a register local an earlier
    /// directive in the batch already mapped.
    Symbol(SymbolId),
    /// A register-resident local no Symbol backs yet.
    Printed(PrintedLocal),
}

/// Resolve the identifier a `name`/`type` directive names, by the two readings
/// in the module docs.
///
/// `Err` is the message the caller reports verbatim; a name nothing answers to
/// keeps the legacy `No symbol named:` wording, the one an agent has already
/// been taught to read.
pub fn resolve_local(fd: &mut Funcdata, name: &str) -> Result<LocalTarget, String> {
    let touched = fd.kuna_directive_symbols().to_vec();
    let mut printed: Vec<SymbolId> =
        touched.iter().filter(|d| d.printed == name).map(|d| d.symbol).collect();
    if let Some(lm) = fd.get_scope_local() {
        printed.extend(
            lm.query_by_name(name)
                .into_iter()
                .filter(|sym| !touched.iter().any(|d| d.symbol == *sym)),
        );
    }
    match printed.len() {
        0 => {}
        1 => return Ok(LocalTarget::Symbol(printed[0])),
        n => return Err(format!("More than one symbol named: {name} ({n})")),
    }
    if let Some(target) = resolve_printed_local(fd, name, &touched)? {
        return Ok(LocalTarget::Printed(target));
    }
    let given: Vec<SymbolId> = match fd.get_scope_local() {
        Some(lm) => touched
            .iter()
            .filter(|d| d.printed != name && lm.database().symbol(d.symbol).name == name)
            .map(|d| d.symbol)
            .collect(),
        None => Vec::new(),
    };
    match given.len() {
        0 => Err(format!("No symbol named: {name}")),
        1 => Ok(LocalTarget::Symbol(given[0])),
        n => Err(format!("More than one symbol named: {name} ({n})")),
    }
}

/// Apply one `name`/`type` directive to the local `name` resolves to: rename it
/// to `newname` (empty keeps the name) and, for a `type`, retype it to `retype`.
pub fn apply_local(
    fd: &mut Funcdata,
    name: &str,
    newname: &str,
    retype: Option<Rc<Datatype>>,
) -> Result<(), String> {
    let sym = match resolve_local(fd, name)? {
        LocalTarget::Printed(target) => {
            let ct = retype.unwrap_or_else(|| target.dtype.clone());
            let symbol = bind_printed_local(fd, &target, newname, ct)?;
            fd.kuna_record_directive_symbol(DirectiveSymbol {
                symbol,
                printed: name.to_string(),
                bound: Some((target.addr.clone(), target.size)),
            });
            return Ok(());
        }
        LocalTarget::Symbol(sym) => sym,
    };
    let bound_size = fd
        .kuna_directive_symbols()
        .iter()
        .find(|d| d.symbol == sym)
        .and_then(|d| d.bound.as_ref().map(|(_, size)| *size));
    if let (Some(size), Some(ct)) = (bound_size, retype.as_ref()) {
        check_width(size, ct)?;
    }
    // A parameter's storage is model-derived; locking its name or type locks the
    // input side of the prototype too (C++ `IfcRename`/`IfcRetype`).
    let lm = fd.get_scope_local().ok_or_else(|| "Function has no local scope".to_string())?;
    let current = lm.database().symbol(sym).name.clone();
    if lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER {
        fd.get_func_proto_mut().set_input_lock(true);
    }
    let lm = fd
        .get_scope_local_mut()
        .ok_or_else(|| "Function has no local scope".to_string())?;
    match retype {
        None => {
            lm.rename_symbol(sym, newname).map_err(|e| e.explain().to_string())?;
            lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
        }
        Some(ct) => {
            lm.retype_symbol(sym, ct).map_err(|e| e.explain().to_string())?;
            lm.set_attribute(sym, varnode_flags::typelock);
            if !newname.is_empty() && newname != current {
                lm.rename_symbol(sym, newname).map_err(|e| e.explain().to_string())?;
                lm.set_attribute(sym, varnode_flags::namelock);
            }
        }
    }
    fd.kuna_record_directive_symbol(DirectiveSymbol { symbol: sym, printed: current, bound: None });
    Ok(())
}

/// Find the HighVariable the printer declared as `name`; `Ok(None)` when no
/// high answers to it.  `touched` is the batch so far.
fn resolve_printed_local(
    fd: &mut Funcdata,
    name: &str,
    touched: &[DirectiveSymbol],
) -> Result<Option<PrintedLocal>, String> {
    let mut ids: Vec<_> = fd
        .high_bank()
        .iter()
        .filter(|(_, h)| h.kuna_name() == Some(name))
        // A CONCAT piece shares its root's name and carries the root's in-symbol
        // offset; the root (offset -1) is the declaration and the only nameable
        // target, exactly as the printer's decl loop decides it.
        .filter(|(_, h)| h.kuna_symbol_offset() < 0)
        .map(|(id, _)| id)
        .collect();
    // A high all of whose instances are constants is a `&symbol` reference the
    // printer renders inline, not storage anything can be mapped over.
    ids.retain(|&id| {
        fd.high_bank().get(id).is_some_and(|h| {
            (0..h.num_instances())
                .any(|i| fd.vbank().get(h.get_instance(i)).is_some_and(|v| !v.is_constant()))
        })
    });
    match ids.len() {
        0 => return Ok(None),
        1 => {}
        n => return Err(format!("More than one variable named: {name} ({n})")),
    }
    let rep = fd
        .high_name_representative(ids[0])
        .ok_or_else(|| format!("No storage for: {name}"))?;
    let (addr, size, dtype, written, def) = {
        let v = fd.vbank().get(rep).ok_or_else(|| format!("No storage for: {name}"))?;
        (v.get_addr().clone(), v.get_size(), v.get_type().clone(), v.is_written(), v.get_def())
    };
    // A Symbol is keyed by its storage, and only storage the next IR rebuild
    // re-creates at the same address can be found again: a machine register, the
    // stack frame, ram.  A `unique`-space temporary is renumbered every rebuild
    // and a JOIN is a synthetic pair, so a Symbol mapped over one binds nothing
    // on the second pass -- it survives as a declared-but-unused local while the
    // variable the caller aimed at is unchanged.  Reporting that as `applied`
    // would be the failure this plane exists to end, so it is a rejection.
    let stable = addr
        .get_space()
        .is_some_and(|spc| !matches!(spc.get_type(), IPTR_INTERNAL | IPTR_JOIN | IPTR_CONSTANT));
    if !stable {
        return Err(format!(
            "Not addressable storage: {name} (a decompiler temporary has no stable location)"
        ));
    }
    // One Symbol, two variables is the other way this ends in invalid C.  The
    // naming pass has a usepoint-blind arm as well (`ScopeLocal::name_for_varnode`
    // is a bare `find_overlap`), so when a SECOND high holds the same register at
    // the same width -- a copy the merge did not coalesce -- both take the mapped
    // Symbol's name and the printer declares it twice.  Sub-register neighbours
    // (`al`, `eax` inside `rax`) are not this: they overlap at a different width,
    // resolve their own names, and are the ordinary case the witness exercises.
    // Only a high the printer would DECLARE counts: an unnamed high sharing the
    // storage emits no declaration, and fauxware's `int v1; // eax` has one.
    let rivals: Vec<_> = fd
        .high_bank()
        .iter()
        .filter(|(id, h)| *id != ids[0] && h.kuna_name().is_some_and(|n| n != name))
        .map(|(id, _)| id)
        .collect();
    for rival in rivals {
        let rep = match fd.high_name_representative(rival) {
            Some(r) => r,
            None => continue,
        };
        let shared = fd
            .vbank()
            .get(rep)
            .is_some_and(|v| v.get_addr() == &addr && v.get_size() == size);
        if shared {
            return Err(format!("Storage of {name} is shared by another variable"));
        }
    }
    // Two Symbols the batch mapped over overlapping registers (`eax` inside
    // `rax`) are the same trap at different widths: the second pass folds both
    // variables into one (`text._0_4_ = 0`).  One per register per batch.
    let clash = touched.iter().find(|d| {
        d.bound.as_ref().is_some_and(|(at, width)| {
            at.overlap(0, &addr, size) >= 0 || addr.overlap(0, at, *width) >= 0
        })
    });
    if let Some(earlier) = clash {
        return Err(format!(
            "Storage of {name} overlaps {}, which an earlier directive already changed",
            earlier.printed
        ));
    }
    // The scope already owns this storage, so the high is a stack local or a
    // parameter the by-name query missed.  Mapping a second Symbol over storage
    // a Symbol already covers would put two entries on one stack slot, so this is
    // a miss.
    if fd
        .get_scope_local()
        .is_some_and(|lm| lm.containing_symbol_for_storage(&addr).is_some())
    {
        return Ok(None);
    }
    // C++ `Varnode::getUsePoint` (varnode.cc:715), which `Funcdata::linkSymbol`
    // passes to the local-scope container query.
    let usepoint = match def.filter(|_| written).and_then(|op| fd.obank().get(op)) {
        Some(op) => op.get_addr().clone(),
        None => &fd.get_address().clone() + -1,
    };
    Ok(Some(PrintedLocal { addr, size, usepoint, dtype }))
}

/// A Symbol covers `ct.get_size()` bytes from the storage address, so a type
/// wider than the target is a statement about the NEXT variable along: `type
/// v1 char *` on a 4-byte `v1 // eax` maps 8 bytes at EAX's address, i.e. RAX,
/// and comes back `applied` with `v1` untouched.  The width is the one thing
/// the caller cannot see from the C, so say it.
fn check_width(size: int4, ct: &Datatype) -> Result<(), String> {
    if ct.get_size() != size {
        return Err(format!("Storage is {size} bytes, the stated type is {}", ct.get_size()));
    }
    Ok(())
}

/// Map an isolated, locked Symbol over a printed local's storage — the mapping
/// `type varnode %REG(pc)` makes, keyed by the emitter's identifier instead of a
/// hand-written varnode specifier.
///
/// Unlike C++ `IfcTypeVarnode` this does not clear the analysis: every caller
/// decompiles again from the scope, and the pass's HighVariables are what the
/// rest of the batch resolves its identifiers against (see the module docs).
///
/// An EMPTY `name` -- what a bare `type v6 <T>` passes, since it states no
/// identifier -- leaves the Symbol for the naming pass to number, and that is
/// deliberate.  Binding the printed identifier back as a namelocked Symbol reads
/// better (the target keeps the name the caller typed) and produces INVALID C:
/// the `vN` allocator does not consult the scope, so it hands the same `v5` to
/// an unrelated temporary and the printer declares `v5` twice in one body
/// (measured on the witness for `type v5 char *` and `type v7 short`).  The cost
/// of the safe choice is that a retyped local can come back under a different
/// number; the storage comment (`// rax`) is what identifies it across the two
/// passes, and an explicit `type v6 <T> <newname>` pins a name outright.
fn bind_printed_local(
    fd: &mut Funcdata,
    target: &PrintedLocal,
    name: &str,
    ct: Rc<Datatype>,
) -> Result<SymbolId, String> {
    check_width(target.size, &ct)?;
    let scope = fd
        .get_scope_local_mut()
        .ok_or_else(|| "Function has no local scope".to_string())?;
    let sym = scope
        .add_symbol(name, ct, &target.addr, &target.usepoint)
        .map_err(|e| e.explain().to_string())?;
    scope.set_attribute(sym, varnode_flags::typelock);
    scope.set_symbol_isolated(sym, true);
    if !name.is_empty() {
        scope.set_attribute(sym, varnode_flags::namelock);
    }
    Ok(sym)
}
