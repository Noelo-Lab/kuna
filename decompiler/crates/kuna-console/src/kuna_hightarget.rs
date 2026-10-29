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
//! Native storage uses a definition-scoped Symbol. Renumbered temporaries,
//! JOIN storage and definitions sharing one native address use the existing
//! DynamicHash machinery, checked against the selected high before binding.
//! Both kinds survive the next IR rebuild; no analysis-local id is a replay key.
//! A `name` with no type maps an unlocked Symbol only so later directives in
//! the batch can find the variable; the rebuild carries the rename as a name
//! instead (`kuna_stablenames`), since a Symbol over one definition of a merged
//! variable split it and its type lock retyped it. A `type` that keeps the
//! variable's width and kind travels the same way, as the type of the whole
//! variable.
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
//! keyed by the identifier the pass printed for it, which gives `resolve_local`
//! two readings of an identifier:
//!
//! 1. the variable the pass printed under it, whatever an earlier directive in
//!    the batch renamed it to -- so directives on different variables do not
//!    depend on their order, and `name a b` with `name b a` is a swap;
//! 2. failing that, the variable an earlier directive in the batch gave that
//!    name, so `name v1 rc` followed by `type rc unsigned int` retypes `rc`;
//! 3. when neither gives one variable, the one a print of the function
//!    declares under it, with the batch's renames given first: a name the
//!    printer suffixed (`score_1` beside a function `score`) reaches the local
//!    shown under it ([`resolve_shown`]).
//!
//! Where both readings exist and name different variables -- `name v1 v2`, then
//! `type v2 ...` -- nothing says which was meant, and the directive is rejected
//! naming both.  The one exception is a `name` whose new name the pass also
//! printed, on a variable that still carries the identifier (`name v1 v2`,
//! `name v2 v1`): the caller is permuting the names it was shown, and the first
//! reading is what makes swaps and rotations work. Different-width register
//! locals may share physical storage when unique definition witnesses select
//! separate highs. Required merges and isolation use those exact identities.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_CONSTANT, IPTR_INTERNAL, IPTR_JOIN, IPTR_SPACEBASE};
use kuna_base::types::int4;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::context::{HighVariableId, VarnodeId};
use kuna_decomp::database::{symbol_category, SymbolId};
use kuna_decomp::dtype::Datatype;
use kuna_decomp::funcdata::{DirectiveSymbol, Funcdata};
use kuna_decomp::varnode::varnode_flags;

/// The storage a printed local occupies, in the shape `Scope::addSymbol` takes.
pub struct PrintedLocal {
    /// Analysis-local identity, used only within this assertion batch.
    pub high: kuna_decomp::context::HighVariableId,
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
    /// Dataflow identity for storage that cannot be replayed by address alone.
    pub dynamic_hash: Option<u64>,
}

/// What a `name`/`type` identifier resolved to.
pub enum LocalTarget {
    /// A Symbol: a stack slot, a parameter, or a register local an earlier
    /// directive in the batch already mapped.
    Symbol(SymbolId),
    /// A printed local no Symbol backs yet.
    Printed(PrintedLocal),
}

/// Resolve the identifier a `name`/`type` directive names, by the two readings
/// in the module docs.  `renaming_to` is the new name of a `name` directive.
///
/// `Err` is the message the caller reports verbatim; a name nothing answers to
/// keeps the legacy `No symbol named:` wording, the one an agent has already
/// been taught to read.
fn resolve_local(
    fd: &mut Funcdata,
    name: &str,
    renaming_to: Option<&str>,
    maxduplicates: u32,
) -> Result<LocalTarget, String> {
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
    if printed.len() > 1 {
        return Err(format!("More than one symbol named: {name} ({})", printed.len()));
    }
    // The name a Symbol carries now; `None` for one a bare `type` left for the
    // naming pass to number, which the caller has not renamed.
    let current = |sym: SymbolId| {
        fd.get_scope_local()
            .map(|lm| lm.database().symbol(sym))
            .filter(|symbol| !symbol.is_name_undefined())
            .map(|symbol| symbol.name.clone())
    };
    let given: Vec<&DirectiveSymbol> = touched
        .iter()
        .filter(|d| d.printed != name && current(d.symbol).as_deref() == Some(name))
        .collect();
    let printed = printed.first().copied();
    let printed_high = printed.is_none() && !printed_highs(fd, name).is_empty();
    if let Some(other) = given.first().filter(|_| printed.is_some() || printed_high) {
        let moved = printed.and_then(current).filter(|now| now != name);
        let permutes = moved.is_none()
            && renaming_to.is_some_and(|new| new != name && printed_by_pass(fd, new, &touched));
        if !permutes {
            let moved = moved.map(|now| format!(" (now {now})")).unwrap_or_default();
            return Err(format!(
                "Ambiguous name: {name} is both the local printed as {name}{moved} \
                 and the local printed as {} (now {name})",
                other.printed
            ));
        }
    }
    if let Some(sym) = printed {
        return Ok(LocalTarget::Symbol(sym));
    }
    if let Some(target) = resolve_printed_local(fd, name, &touched, maxduplicates)? {
        return Ok(LocalTarget::Printed(target));
    }
    match given.len() {
        1 if !printed_high => Ok(LocalTarget::Symbol(given[0].symbol)),
        0 | 1 => Err(format!("No symbol named: {name}")),
        n => Err(format!("More than one symbol named: {name} ({n})")),
    }
}

/// Did the pass print a local as `name`: a Symbol the batch touched under it, an
/// untouched Symbol still holding it, or a HighVariable the printer declared.
fn printed_by_pass(fd: &Funcdata, name: &str, touched: &[DirectiveSymbol]) -> bool {
    touched.iter().any(|d| d.printed == name)
        || fd.get_scope_local().is_some_and(|lm| {
            lm.query_by_name(name).iter().any(|sym| !touched.iter().any(|d| d.symbol == *sym))
        })
        || printed_highs(fd, name)
            .into_iter()
            .any(|id| fd.high_bank().get(id).is_some_and(|h| !h.kuna_global()))
}

/// The HighVariables the printer declared as `name`, a local printed under its
/// library name answering to the number it is numbered under
/// (`kuna_decomp::kuna_libcnames`).
fn printed_highs(fd: &Funcdata, name: &str) -> Vec<HighVariableId> {
    fd.high_bank()
        .iter()
        .filter(|(id, h)| kuna_decomp::kuna_libcnames::shown_number(fd, *id).or(h.kuna_name()) == Some(name))
        // A CONCAT piece shares its root's name and carries the root's in-symbol
        // offset; the root (offset -1) is the declaration and the only nameable
        // target, exactly as the printer's decl loop decides it.
        .filter(|(_, h)| h.kuna_symbol_offset() < 0)
        // A high all of whose instances are constants is a `&symbol` reference
        // the printer renders inline, not storage anything can be mapped over.
        .filter(|(_, h)| {
            (0..h.num_instances())
                .any(|i| fd.vbank().get(h.get_instance(i)).is_some_and(|v| !v.is_constant()))
        })
        .map(|(id, _)| id)
        .collect()
}

/// The number a library name (`kuna_decomp::kuna_libcnames`) stands for, so a
/// directive reaches the local by either of its names and a replay records the
/// number, whether or not this run prints library names. `None` when no local
/// has the library name `name`, or when an earlier directive in the batch gave
/// `name` to another local: that rename took the library name, which is then
/// that local's.
fn library_number(fd: &Funcdata, name: &str) -> Option<String> {
    let number = kuna_decomp::kuna_libcnames::number_for(fd, name)?;
    let lm = fd.get_scope_local()?;
    let given = fd
        .kuna_directive_symbols()
        .iter()
        .any(|d| d.printed != name && lm.database().symbol(d.symbol).name == name);
    (!given).then_some(number)
}

/// The local a print of `fd` declares as `shown` where no single local answers
/// to that name: the printer suffixes a name another identifier already holds
/// (`score_1` beside a function `score`, the second of two locals named
/// `dup`). The print first gives each local the batch renamed its new name,
/// so a name an earlier directive gave is read as the printer shows it.
/// `None` when the print declares no single local as `shown`, or declares
/// two locals the batch renamed under one line (their storage overlaps), so
/// it cannot say which the next decompile shows as `shown`; with the
/// target, the name to record the directive under.
fn resolve_shown(
    fd: &mut Funcdata,
    arch: &Architecture,
    shown: &str,
    maxduplicates: u32,
) -> Option<Result<(LocalTarget, String), String>> {
    let renamed = batch_renames(fd);
    let saved: Vec<(HighVariableId, String)> = renamed
        .iter()
        .filter_map(|(_, high, _)| Some((*high, fd.high_bank().get(*high)?.kuna_name()?.to_string())))
        .collect();
    for (_, high, name) in &renamed {
        if let Some(h) = fd.high_bank_mut().get_mut(*high).filter(|h| h.kuna_name().is_some()) {
            h.set_kuna_name(name.clone());
        }
    }
    let print = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        kuna_decomp::printc::PrintC::declarations_of(arch, fd)
    }))
    .ok();
    for (high, name) in saved.into_iter().rev() {
        if let Some(h) = fd.high_bank_mut().get_mut(high) {
            h.set_kuna_name(name);
        }
    }
    let print = print?;
    let mut found = print.declared_locals().iter().filter(|local| local.name == shown);
    let local = found.next()?.clone();
    if found.next().is_some() {
        return None;
    }
    let owner = |high: HighVariableId| print.declared_local(high).map(|local| local.high);
    let mut owners = renamed.iter().filter(|(_, high, _)| owner(*high) == Some(local.high)).map(|(sym, ..)| *sym);
    if let Some(sym) = owners.next() {
        if owners.any(|other| other != sym) {
            return None;
        }
        return Some(Ok((LocalTarget::Symbol(sym), shown.to_string())));
    }
    let h = fd.high_bank().get(local.high)?;
    let linked = h.kuna_link_symbol().or(h.kuna_ref_symbol()).filter(|sym| {
        fd.get_scope_local().is_some_and(|lm| lm.database().try_symbol(*sym).is_some_and(|s| !s.is_name_undefined()))
    });
    if let Some(sym) = linked {
        return Some(Ok((LocalTarget::Symbol(sym), shown.to_string())));
    }
    let key = fd
        .kuna_library_aliases()
        .iter()
        .find(|alias| alias.shown && alias.alias == local.base)
        .map_or_else(|| local.base.clone(), |alias| alias.number.clone());
    let shared = print.declared_locals().iter().filter(|other| other.base == local.base).count() > 1;
    let target = match printed_local(fd, local.high, shown, maxduplicates) {
        Ok(Some(target)) => target,
        Ok(None) => return None,
        Err(error) => return Some(Err(error)),
    };
    if !shared {
        return Some(Ok((LocalTarget::Printed(target), key)));
    }
    kuna_decomp::kuna_stablenames::relabel(fd, local.high, shown);
    Some(Ok((LocalTarget::Printed(target), shown.to_string())))
}

/// Each local the batch so far renamed, by the Symbol the directive recorded,
/// with its new name: the high a directive bound, or else the highs that
/// print the Symbol (its storage, or a reference to it, as `&v2` or a stack
/// array) or the identifier the pass printed for it.
fn batch_renames(fd: &Funcdata) -> Vec<(SymbolId, HighVariableId, String)> {
    let Some(lm) = fd.get_scope_local() else { return Vec::new() };
    let mut out = Vec::new();
    for d in fd.kuna_directive_symbols() {
        let Some(symbol) = lm.database().try_symbol(d.symbol) else { continue };
        if symbol.is_name_undefined() || symbol.name.is_empty() || symbol.name == d.printed {
            continue;
        }
        let highs: Vec<HighVariableId> = match &d.bound {
            Some((_, _, high)) => vec![*high],
            None => {
                let linked: Vec<HighVariableId> = fd
                    .high_bank()
                    .iter()
                    .filter(|(_, h)| h.kuna_link_symbol() == Some(d.symbol) || h.kuna_ref_symbol() == Some(d.symbol))
                    .map(|(id, _)| id)
                    .collect();
                if linked.is_empty() { printed_highs(fd, &d.printed) } else { linked }
            }
        };
        out.extend(highs.into_iter().map(|high| (d.symbol, high, symbol.name.clone())));
    }
    out
}

/// The stack storage a local's Symbol covers, and the widths the local is
/// printed at: its storage's, its type's and, for a scalar, each local the
/// Symbol names (a slot written whole and read in part prints at the whole
/// width). `None` for a parameter or a Symbol off the stack.
fn stack_storage(fd: &Funcdata, sym: SymbolId) -> Option<(Address, Vec<int4>)> {
    use kuna_decomp::dtype::type_metatype::{TYPE_ARRAY, TYPE_STRUCT, TYPE_UNION};
    let lm = fd.get_scope_local()?;
    if lm.symbol_category(sym) == symbol_category::FUNCTION_PARAMETER {
        return None;
    }
    let (addr, size) = lm
        .named_symbol_storage()
        .into_iter()
        .filter(|(s, addr, ..)| *s == sym && addr.get_space().is_some_and(|space| space.get_type() == IPTR_SPACEBASE))
        .map(|(_, addr, size, _)| (addr, size))
        .max_by_key(|(_, size)| *size)?;
    let mut widths = vec![size];
    let dtype = lm.database().symbol(sym).dtype.clone();
    if let Some(dtype) = &dtype {
        widths.push(dtype.get_size());
        if matches!(dtype.get_metatype(), TYPE_ARRAY | TYPE_STRUCT | TYPE_UNION) {
            return Some((addr, widths));
        }
    }
    for (_, h) in fd.high_bank().iter().filter(|(_, h)| h.kuna_link_symbol() == Some(sym)) {
        widths.extend((0..h.num_instances()).filter_map(|i| fd.vbank().get(h.get_instance(i)).map(|v| v.get_size())));
    }
    Some((addr, widths))
}

/// Refuse a type a stack local printed at `widths` and stored from `addr`
/// cannot take whole: a scalar of none of those widths, which leaves the local
/// declared as before or not at all, with its neighbours re-laid around the
/// new width, or a type that runs past the end of the stack. A structure,
/// union or array states the extent of the object at that address, so it may
/// grow the local over the frame (a buffer the analysis saw only the start
/// of) or narrow it.
fn check_stack_width(addr: &Address, widths: &[int4], ct: &Datatype) -> Result<(), String> {
    use kuna_decomp::dtype::type_metatype::{TYPE_ARRAY, TYPE_STRUCT, TYPE_UNION};
    let size = widths.iter().copied().max().unwrap_or(0);
    let composite = matches!(ct.get_metatype(), TYPE_ARRAY | TYPE_STRUCT | TYPE_UNION);
    if !composite && !widths.contains(&ct.get_size()) {
        return Err(format!("Storage is {size} bytes, the stated type is {}", ct.get_size()));
    }
    let end = (ct.get_size() as u64).checked_sub(1).and_then(|last| addr.get_offset().checked_add(last));
    if ct.get_size() > size && addr.get_space().is_some_and(|space| end.is_none_or(|end| end > space.get_highest())) {
        return Err(format!(
            "Storage is {size} bytes, the stated type is {} and runs past the end of the stack",
            ct.get_size()
        ));
    }
    Ok(())
}

/// Independently locked locals cannot both own the same frame bytes. A single
/// aggregate or union may describe multiple views, but two C declarations would
/// lose the alias between their writes and reads.
fn check_stack_retype_overlap(
    fd: &Funcdata,
    target: SymbolId,
    address: &Address,
    size: int4,
) -> Result<(), String> {
    if !address.get_space().is_some_and(|space| space.get_type() == IPTR_SPACEBASE) {
        return Ok(());
    }
    let Some(scope) = fd.get_scope_local() else { return Ok(()) };
    // Definition-scoped bindings may describe distinct incarnations of a slot.
    // This guard concerns independent whole-frame declarations only.
    if scope.database().symbol(target).mapentry.iter().any(|&entry| {
        !scope.database().entry(scope.scope_id(), entry).uselimit.empty()
    }) {
        return Ok(());
    }
    let start = address.get_offset() as u128;
    let end = start + size.max(0) as u128;
    for directive in fd.kuna_directive_symbols() {
        if !directive.retyped || directive.symbol == target {
            continue;
        }
        for &mapping in &scope.database().symbol(directive.symbol).mapentry {
            let entry = scope.database().entry(scope.scope_id(), mapping);
            if !entry.uselimit.empty() || entry.get_offset() != 0 || entry.get_addr().is_invalid() {
                continue;
            }
            let other = entry.get_addr();
            let width = entry.get_size();
            if other.get_space().map(|space| space.get_index())
                != address.get_space().map(|space| space.get_index())
            {
                continue;
            }
            let other_start = other.get_offset() as u128;
            let other_end = other_start + width.max(0) as u128;
            if start < other_end && other_start < end {
                return Err(format!(
                    "Stack storage overlaps the independently retyped local `{}`; declare one aggregate or union covering the shared storage",
                    directive.printed
                ));
            }
        }
    }
    Ok(())
}

/// Apply one `name`/`type` directive to the local `name` resolves to: rename it
/// to `newname` (empty keeps the name) and, for a `type`, retype it to
/// `retype`. Returns the Symbol the directive is recorded on, if any (a stack
/// object's view has none).
pub fn apply_local(
    fd: &mut Funcdata,
    name: &str,
    newname: &str,
    retype: Option<Rc<Datatype>>,
    maxduplicates: u32,
    arch: Option<&Architecture>,
) -> Result<Option<SymbolId>, String> {
    if newname != name {
        check_local_name(fd, newname)?;
    }
    let number = library_number(fd, name);
    let name = number.as_deref().unwrap_or(name);
    if kuna_decomp::kuna_stackobjectasserts::apply(fd, name, newname, retype.clone())? {
        return Ok(None);
    }
    let (target, name) = match resolve_local(fd, name, retype.is_none().then_some(newname), maxduplicates) {
        Ok(target) => (target, name.to_string()),
        Err(error) if error.starts_with("Ambiguous name") => return Err(error),
        Err(error) => match arch.and_then(|arch| resolve_shown(fd, arch, name, maxduplicates)) {
            Some(found) => found?,
            None => return Err(error),
        },
    };
    let name = name.as_str();
    let sym = match target {
        LocalTarget::Printed(target) => {
            let lock_type = retype.is_some();
            let unit_type =
                retype.as_ref().and_then(|ct| kuna_decomp::kuna_stablenames::unit_retype(&target.dtype, ct));
            let ct = retype.unwrap_or_else(|| target.dtype.clone());
            let symbol = bind_printed_local(fd, &target, newname, ct, lock_type)?;
            fd.kuna_record_directive_symbol(DirectiveSymbol {
                symbol,
                printed: name.to_string(),
                bound: Some((target.addr.clone(), target.size, target.high)),
                retyped: lock_type,
                unit_type,
            });
            return Ok(Some(symbol));
        }
        LocalTarget::Symbol(sym) => sym,
    };
    let bound_size = fd
        .kuna_directive_symbols()
        .iter()
        .find(|d| d.symbol == sym)
        .and_then(|d| d.bound.as_ref().map(|(_, size, _)| *size));
    let has_instances = bound_size.is_some() || symbol_storage_has_varnodes(fd, sym);
    let unit_type = fd
        .get_scope_local()
        .filter(|lm| has_instances && lm.symbol_category(sym) != symbol_category::FUNCTION_PARAMETER)
        .and_then(|lm| lm.database().symbol(sym).dtype.clone())
        .zip(retype.as_ref())
        .and_then(|(from, to)| kuna_decomp::kuna_stablenames::unit_retype(&from, to));
    if let (Some(size), Some(ct)) = (bound_size, retype.as_ref()) {
        check_width(size, ct)?;
    }
    if let (None, Some(ct)) = (bound_size, retype.as_ref()) {
        if let Some((addr, widths)) = stack_storage(fd, sym) {
            check_stack_width(&addr, &widths, ct)?;
            check_stack_retype_overlap(fd, sym, &addr, ct.get_size())?;
        }
    }
    let retype_given = retype.is_some();
    let parameter = crate::kuna_paramasserts::prepare(fd, sym, retype.as_ref())?;
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
            let symbol = lm.database().symbol(sym);
            let dynamic = !symbol.mapentry.is_empty()
                && symbol.mapentry.iter().all(|entry| {
                    matches!(entry, kuna_decomp::database::EntryRef::Dynamic(_))
                });
            lm.rename_symbol(sym, newname).map_err(|e| e.explain().to_string())?;
            if bound_size.is_some() || (!dynamic && !lm.database().symbol(sym).is_type_locked()) {
                lm.set_attribute(sym, varnode_flags::namelock);
            } else if dynamic {
                lm.set_attribute(sym, varnode_flags::namelock);
                if !lm.database().symbol(sym).is_type_locked() {
                    lm.set_symbol_identity_isolated(sym);
                }
            } else {
                lm.set_attribute(sym, varnode_flags::namelock | varnode_flags::typelock);
            }
        }
        Some(ct) => {
            lm.retype_symbol(sym, ct).map_err(|e| e.explain().to_string())?;
            lm.set_attribute(sym, varnode_flags::typelock);
            if bound_size.is_some() {
                lm.set_symbol_isolated(sym, true);
            }
            if !newname.is_empty() && newname != current {
                lm.rename_symbol(sym, newname).map_err(|e| e.explain().to_string())?;
                lm.set_attribute(sym, varnode_flags::namelock);
            }
        }
    }
    if let Some(parameter) = parameter {
        crate::kuna_paramasserts::apply(fd, sym, parameter);
    }
    fd.kuna_record_directive_symbol(DirectiveSymbol { symbol: sym, printed: current, bound: None, retyped: false, unit_type: None });
    if retype_given {
        fd.kuna_set_directive_retype(sym, unit_type);
    }
    Ok(Some(sym))
}

/// Does any varnode live in `sym`'s storage? A local only ever reached
/// through its address has none, so there is no instance to carry a unit
/// retype to and its Symbol takes the type, as its storage is the variable's
/// whole identity.
fn symbol_storage_has_varnodes(fd: &Funcdata, sym: SymbolId) -> bool {
    let Some(lm) = fd.get_scope_local() else { return false };
    let storage: Vec<(Address, int4)> = lm
        .named_symbol_storage()
        .into_iter()
        .filter(|(s, ..)| *s == sym)
        .map(|(_, addr, size, _)| (addr, size))
        .collect();
    fd.vbank().iter_loc().any(|vn| {
        fd.vbank().get(vn).is_some_and(|v| {
            !v.is_constant()
                && !v.is_annotation()
                && storage.iter().any(|(addr, size)| {
                    v.get_addr().get_space().map(|s| s.get_index()) == addr.get_space().map(|s| s.get_index())
                        && v.get_addr().get_offset() < addr.get_offset().wrapping_add(*size as u64)
                        && addr.get_offset() < v.get_addr().get_offset().wrapping_add(v.get_size() as u64)
                })
        })
    })
}

/// Find the HighVariable the printer declared as `name`; `Ok(None)` when no
/// high answers to it.  `touched` is the batch so far.
fn resolve_printed_local(
    fd: &mut Funcdata,
    name: &str,
    _touched: &[DirectiveSymbol],
    maxduplicates: u32,
) -> Result<Option<PrintedLocal>, String> {
    let ids = printed_highs(fd, name);
    match ids.len() {
        0 => Ok(None),
        1 => printed_local(fd, ids[0], name, maxduplicates),
        n => Err(format!("More than one variable named: {name} ({n})")),
    }
}

/// The storage a directive binds for the printed local `high`, shown as
/// `name`; `Ok(None)` when a Symbol already owns that storage.
fn printed_local(
    fd: &mut Funcdata,
    high: HighVariableId,
    name: &str,
    maxduplicates: u32,
) -> Result<Option<PrintedLocal>, String> {
    let ids = [high];
    let rep = fd
        .high_name_representative(ids[0])
        .ok_or_else(|| format!("No storage for: {name}"))?;
    let (addr, size, dtype, written, def) = {
        let v = fd.vbank().get(rep).ok_or_else(|| format!("No storage for: {name}"))?;
        (v.get_addr().clone(), v.get_size(), v.get_type().clone(), v.is_written(), v.get_def())
    };
    let stable = addr.get_space().is_some_and(|space|
        !matches!(space.get_type(), IPTR_INTERNAL | IPTR_JOIN | IPTR_CONSTANT));
    if !stable {
        return dynamic_target(fd, ids[0], rep, addr, size, dtype, maxduplicates).map(Some);
    }
    // The replay key is storage + a native definition address, not just a
    // register. Prove this anchor names only the selected high; several p-code
    // definitions can share an instruction address, so PC alone is not always
    // sufficient. Keep those cases rejected until a richer witness exists.
    let usepoint = match def.filter(|_| written).and_then(|op| fd.obank().get(op)) {
        Some(op) => op.get_addr().clone(),
        None => &fd.get_address().clone() + -1,
    };
    let ambiguous = fd.vbank().iter_loc().any(|vn| {
        let Some(v) = fd.vbank().get(vn) else { return false };
        if v.get_addr() != &addr || v.get_size() != size || v.get_high() == Some(ids[0]) {
            return false;
        }
        let at = match v.get_def().and_then(|op| fd.obank().get(op)) {
            Some(op) if v.is_written() => op.get_addr().clone(),
            _ => &fd.get_address().clone() + -1,
        };
        at == usepoint
    });
    if ambiguous {
        return dynamic_target(fd, ids[0], rep, addr, size, dtype, maxduplicates).map(Some);
    }
    // These are already separate HighVariables; selecting them does not request
    // a merge or a split. Their logical covers can overlap after COPY
    // propagation (the earlier register value was saved elsewhere), even though
    // their native definitions select different register incarnations. The
    // unique definition anchor above, not cover interference, identifies the
    // Symbol on replay. bind_printed_local makes that Symbol isolated so later
    // merging cannot erase its identity.
    // The scope already owns this storage, so the high is a stack local or a
    // parameter the by-name query missed.  Mapping a second Symbol over storage
    // a Symbol already covers would put two entries on one stack slot, so this is
    // a miss.
    if fd
        .get_scope_local()
        .is_some_and(|lm| lm.query_container_for_link(&addr, &usepoint).is_some())
    {
        return Ok(None);
    }
    Ok(Some(PrintedLocal {
        high: ids[0],
        addr,
        size,
        usepoint,
        dtype,
        dynamic_hash: None,
    }))
}

/// A native operation plus its dataflow hash anchors storage that is renumbered.
fn dynamic_target(
    fd: &mut Funcdata,
    high: HighVariableId,
    rep: VarnodeId,
    addr: Address,
    size: int4,
    dtype: Rc<Datatype>,
    maxduplicates: u32,
) -> Result<PrintedLocal, String> {
    let (hash, usepoint) = kuna_decomp::dynamic::dynamic_unique_hash(rep, maxduplicates, fd)
        .map_err(|error| error.explain().to_string())?;
    let found = (hash != 0).then(|| {
        kuna_decomp::dynamic::DynamicHash::new().find_varnode(fd, &usepoint, hash)
    }).flatten();
    if !found.and_then(|id| fd.vbank().get(id))
        .is_some_and(|v| v.get_high() == Some(high) && v.get_size() == size)
    {
        return Err("Unable to identify the selected variable uniquely for replay".to_string());
    }
    Ok(PrintedLocal { high, addr, size, usepoint, dtype, dynamic_hash: Some(hash) })
}

/// The words C reserves, which no declaration can use as a variable's name.
const C_KEYWORDS: &[&str] = &[
    "alignas", "alignof", "auto", "bool", "break", "case", "char", "const", "constexpr", "continue",
    "default", "do", "double", "else", "enum", "extern", "false", "float", "for", "goto", "if",
    "inline", "int", "long", "nullptr", "register", "restrict", "return", "short", "signed",
    "sizeof", "static", "static_assert", "struct", "switch", "thread_local", "true", "typedef",
    "typeof", "typeof_unqual", "union", "unsigned", "void", "volatile", "while", "_Alignas",
    "_Alignof", "_Atomic", "_BitInt", "_Bool", "_Complex", "_Decimal128", "_Decimal32",
    "_Decimal64", "_Generic", "_Imaginary", "_Noreturn", "_Static_assert", "_Thread_local",
];

/// Refuse a new local name the declaration could not print as a name: a C
/// keyword (`int int;`) or a type the program knows by that name.
fn check_local_name(fd: &Funcdata, newname: &str) -> Result<(), String> {
    if newname.is_empty() {
        return Ok(());
    }
    if C_KEYWORDS.contains(&newname) {
        return Err(format!("`{newname}` is a C keyword, not a usable local name"));
    }
    let is_type = fd
        .get_arch()
        .types()
        .is_some_and(|types| types.find_by_name(newname).ok().flatten().is_some());
    if is_type {
        return Err(format!("`{newname}` names a type, not a usable local name"));
    }
    Ok(())
}

/// A Symbol covers `ct.get_size()` bytes from the storage address, so a type
/// wider than the target is a statement about the NEXT variable along: `type
/// v1 char *` on a 4-byte `v1 // eax` maps 8 bytes at EAX's address, i.e. RAX,
/// and comes back `applied` with `v1` untouched.  The width is the one thing
/// the caller cannot see from the C, so say it.
fn check_width(size: int4, ct: &Datatype) -> Result<(), String> {
    if ct.get_size() != size {
        return Err(format!(
            "Storage is {size} bytes, the stated type is {}",
            ct.get_size()
        ));
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
/// better (the target keeps the name the caller typed) but produced INVALID C
/// when the `vN` allocator did not consult the scope: it handed the same `v5` to
/// an unrelated temporary and the printer declared `v5` twice in one body
/// (measured on the witness for `type v5 char *` and `type v7 short`).  The
/// naming pass gives it back the number its pieces had (`kuna_stablenames`),
/// and an explicit `type v6 <T> <newname>` pins a name outright.
fn bind_printed_local(
    fd: &mut Funcdata,
    target: &PrintedLocal,
    name: &str,
    ct: Rc<Datatype>,
    lock_type: bool,
) -> Result<SymbolId, String> {
    check_width(target.size, &ct)?;
    let scope = fd
        .get_scope_local_mut()
        .ok_or_else(|| "Function has no local scope".to_string())?;
    let sym = match target.dynamic_hash {
        Some(hash) => scope.add_dynamic_symbol(name, ct, &target.usepoint, hash),
        None => scope.add_symbol(name, ct, &target.addr, &target.usepoint),
    }.map_err(|error| error.explain().to_string())?;
    if lock_type {
        scope.set_attribute(sym, varnode_flags::typelock);
        scope.set_symbol_isolated(sym, true);
    }
    if !name.is_empty() {
        scope.set_attribute(sym, varnode_flags::namelock);
    }
    Ok(sym)
}
