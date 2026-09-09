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

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_CONSTANT, IPTR_INTERNAL, IPTR_JOIN};
use kuna_base::types::int4;
use kuna_decomp::dtype::Datatype;
use kuna_decomp::funcdata::Funcdata;
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

/// Find the HighVariable the printer declared as `name`.
///
/// `Err` is the message the caller reports verbatim; a name no high answers to
/// keeps the legacy `No symbol named:` wording, because at that point the
/// directive has missed both namespaces and the scope's answer is the one an
/// agent has already been taught to read.
pub fn resolve_printed_local(fd: &mut Funcdata, name: &str) -> Result<PrintedLocal, String> {
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
        0 => return Err(format!("No symbol named: {name}")),
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
    // The scope already owns this storage, so the high is a stack local or a
    // parameter that the by-name query missed for some OTHER reason -- most often
    // because an earlier directive in the same batch renamed its Symbol while the
    // high still reports the name the first pass printed.  Mapping a second
    // Symbol over storage a Symbol already covers would put two entries on one
    // stack slot; the caller's `No symbol named:` is the right answer there.
    if fd
        .get_scope_local()
        .is_some_and(|lm| lm.containing_symbol_for_storage(&addr).is_some())
    {
        return Err(format!("No symbol named: {name}"));
    }
    // C++ `Varnode::getUsePoint` (varnode.cc:715), which `Funcdata::linkSymbol`
    // passes to the local-scope container query.
    let usepoint = match def.filter(|_| written).and_then(|op| fd.obank().get(op)) {
        Some(op) => op.get_addr().clone(),
        None => &fd.get_address().clone() + -1,
    };
    Ok(PrintedLocal { addr, size, usepoint, dtype })
}

/// Map an isolated, locked Symbol over a printed local's storage — the mapping
/// `type varnode %REG(pc)` makes, keyed by the emitter's identifier instead of a
/// hand-written varnode specifier.
///
/// The analysis is cleared first (C++ `IfcTypeVarnode`'s `clearAnalysis`) so the
/// caller's next `decompile` reads the new Symbol; the Symbol is locked, so the
/// `clearUnlocked` that clear performs does not take it back.
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
pub fn bind_printed_local(
    fd: &mut Funcdata,
    target: &PrintedLocal,
    name: &str,
    ct: Rc<Datatype>,
) -> Result<(), String> {
    // A Symbol covers `ct.get_size()` bytes from the storage address, so a type
    // wider than the target is a statement about the NEXT variable along: `type
    // v1 char *` on a 4-byte `v1 // eax` maps 8 bytes at EAX's address, i.e. RAX,
    // and comes back `applied` with `v1` untouched.  The width is the one thing
    // the caller cannot see from the C, so say it.
    if ct.get_size() != target.size {
        return Err(format!(
            "Storage is {} bytes, the stated type is {}",
            target.size,
            ct.get_size()
        ));
    }
    fd.clear();
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
    Ok(())
}
