//! (kuna `cppproto`) The C++ arm of the DWARF pass: resolve a subprogram
//! DEFINITION through its declaration link, qualify it by its namespace/class
//! ancestry, and bind the recovered prototype by ENTRY ADDRESS.
//!
//! ## What the name-only walk misses
//!
//! `DwarfPass` keys everything off a subprogram DIE's own `DW_AT_name`. That is
//! sufficient for C, where every definition carries its name, and wrong for C++,
//! where the compiler splits a definition from its declaration:
//!
//! * an **out-of-line member or namespace definition** (`int Account::deposit(int)`,
//!   `db::inner::scaled_add`) is emitted at CU top level with only
//!   `DW_AT_specification` pointing at the in-class / in-namespace declaration —
//!   no `DW_AT_name`, no `DW_AT_type` of its own;
//! * a **concrete out-of-line instance** of an inlined function (C too, at `-O2
//!   -g`) is emitted with only `DW_AT_abstract_origin`, and its parameters
//!   likewise carry `DW_AT_abstract_origin` instead of a name and type.
//!
//! Both were dropped whole by the `snap.name.is_empty()` guard, so on a `-g` C++
//! binary kuna lost the name, the typed signature AND the named stack locals of
//! every member function. Chasing the link is a **single hop** — the declaration
//! a definition points at is itself a declaration, never another indirection.
//!
//! ## Why only `DW_AT_specification` is followed (a measured exclusion)
//!
//! A GCC IPA clone (`put_word.isra.1`, `foo.constprop.0`, `bar.part.0`) is ALSO
//! emitted as a nameless `DW_TAG_subprogram` — with `DW_AT_abstract_origin`
//! pointing at the original — but its signature is *not* the original's: that is
//! the whole point of the clone. In coreutils `fmt`, `put_word.isra.1` at
//! `0x23d0` has TWO formal parameters, both `DW_AT_abstract_origin`-linked to the
//! SAME source parameter `Word *w`, because IPA-SRA split the aggregate into two
//! scalars. Following that link produced `put_word(Word *w,Word *w)` — a
//! duplicate name and a struct-pointer type on what is really a length — and the
//! wrong callee signature propagated into `put_line`'s call sites.
//!
//! A `DW_AT_specification` link carries no such hazard: it is a definition paired
//! with its own declaration, one signature described twice. So the subprogram
//! chase follows `DW_AT_specification` only. The cost is real and accepted — a
//! concrete out-of-line instance (a destructor body, a `.cold` part) still
//! recovers nothing — and the payoff, every out-of-line member and namespace
//! definition, is untouched by the restriction. Parameter DIEs are still resolved
//! through their own `DW_AT_abstract_origin` (harmless: within a
//! specification-linked definition every parameter carries its own name).
//!
//! ## Why the prototype binds by address
//!
//! `Architecture::set_function_prototype_pieces` resolves a NAME in the global
//! scope, while the parked prototype is read back by ADDRESS
//! (`Database::function_proto_pieces`). C++ breaks the round trip twice over: a
//! demangled template name is normalized (kuna's symbol for `maxof<int>` is
//! `maxof`), and a qualified name lives in a nested scope
//! (`find_create_scope_from_symbol_name` puts `Account::deposit` in scope
//! `Account`) that the global by-name query never reaches. `DW_AT_low_pc` is the
//! key both sides already agree on, and it is what Ghidra keys `DWARFFunction`
//! by (`getCodeAddress(dwarfBody.getFirstAddress())`).
//!
//! ## Origin (upstream Ghidra)
//!
//! `DIEAggregate` fuses a definition DIE with its specification/abstract-origin
//! DIEs and answers every attribute query across the fused set — this module is
//! the reduction of that fusion to the attributes the prototype needs.
//! `DWARFName`/`DWARFProgram.getName` walks the DIE parents for the namespace
//! path ([`qualified_name`]); `DWARFDataTypeImporter` maps `DW_TAG_class_type`
//! like a structure and a reference like a pointer (both handled in
//! [`super::build_datatype`]'s `cpp` arms).
//!
//! ## Out of scope (the sibling seam)
//!
//! A `DW_TAG_class_type`/`structure_type` still maps to a NAMED OPAQUE struct —
//! `DW_TAG_member`/`DW_AT_data_member_location` field population is deliberately
//! left alone, so `this->balance` still prints as `a0[1]`. Filling those fields
//! is a separate change that plugs into [`super::build_datatype`]'s struct arms
//! without touching anything here.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::uint4;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use super::kuna_typedepth::TypeWalk;
use super::{build_datatype, DieSnap};

/// A subprogram definition fused with the declaration it points at — the reduced
/// `DIEAggregate` view the prototype/local builders read.
pub(super) struct ResolvedSub {
    /// The namespace/class-qualified source name (`Account::deposit`).
    pub name: String,
    /// True when the name came from the linked declaration rather than the
    /// definition's own `DW_AT_name` (i.e. the definition would otherwise have
    /// been dropped entirely).
    pub chased: bool,
    /// The return-type DIE offset (`None` => `void`).
    pub type_ref: Option<usize>,
    /// The `DW_TAG_formal_parameter` DIE offsets, in declaration order.
    pub params: Vec<usize>,
    /// Fixed-parameter count at which `...` begins, or `-1` when not variadic.
    pub first_var_arg_slot: i32,
}

/// Fuse the subprogram definition `sub` with the declaration reached through its
/// one-hop `DW_AT_specification` link (see the clone note in the module header
/// for why `DW_AT_abstract_origin` is deliberately not followed here).
///
/// `None` when no name can be recovered from either DIE — a nameless subprogram
/// is nothing this pass can install.
pub(super) fn resolve_subprogram(
    sub: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
) -> Option<ResolvedSub> {
    let decl =
        if sub.origin_is_spec { sub.origin_ref.and_then(|o| dies.get(&o)) } else { None };

    // The DIE that owns the source name also owns the namespace/class ancestry.
    let (named, chased) = if !sub.name.is_empty() {
        (sub, false)
    } else {
        match decl {
            Some(d) if !d.name.is_empty() => (d, true),
            _ => return None,
        }
    };

    let mut params: Vec<usize> = formal_parameters(sub, dies);
    let mut first_var_arg_slot = var_arg_slot(sub, dies, params.len());
    if params.is_empty() && first_var_arg_slot < 0 {
        // A definition that lists no parameters of its own (gcc does this for a
        // `DW_AT_declaration`-linked definition of a parameterless-looking body):
        // fall back to the declaration's list, which carries the real types.
        if let Some(d) = decl {
            params = formal_parameters(d, dies);
            first_var_arg_slot = var_arg_slot(d, dies, params.len());
        }
    }

    Some(ResolvedSub {
        name: qualified_name(named, dies),
        chased,
        type_ref: sub.type_ref.or_else(|| decl.and_then(|d| d.type_ref)),
        params,
        first_var_arg_slot,
    })
}

/// The DIE's `DW_TAG_formal_parameter` children, in order, **flattening
/// `DW_TAG_GNU_formal_parameter_pack`**.
///
/// A variadic template's expanded pack arguments are not direct children: GCC
/// wraps them in a `DW_TAG_GNU_formal_parameter_pack` grouping DIE. Reading only
/// the direct children therefore under-counts the arity — `std::vector<int>
/// ::emplace_back<int>(this, int &&)` looked like a one-parameter function, and
/// locking that signature dropped the argument at every call site.
fn formal_parameters(die: &DieSnap, dies: &BTreeMap<usize, DieSnap>) -> Vec<usize> {
    let mut out = Vec::new();
    for &c in &die.children {
        let Some(child) = dies.get(&c) else { continue };
        match child.tag {
            gimli::DW_TAG_formal_parameter => out.push(c),
            gimli::DW_TAG_GNU_formal_parameter_pack => {
                for &p in &child.children {
                    if dies.get(&p).map(|d| d.tag) == Some(gimli::DW_TAG_formal_parameter) {
                        out.push(p);
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// The fixed-parameter count at which a trailing `DW_TAG_unspecified_parameters`
/// starts the `...`, or `-1` when the DIE has none.
fn var_arg_slot(die: &DieSnap, dies: &BTreeMap<usize, DieSnap>, nparams: usize) -> i32 {
    let variadic = die
        .children
        .iter()
        .filter_map(|c| dies.get(c))
        .any(|c| c.tag == gimli::DW_TAG_unspecified_parameters);
    if variadic {
        nparams as i32
    } else {
        -1
    }
}

/// Build the `A::B::name` source name by walking `die`'s namespace/class/struct
/// ancestry outermost-first (Ghidra's `DWARFName` / `DWARFProgram.getName`).
///
/// An anonymous ancestor (an unnamed namespace or class) contributes nothing —
/// Ghidra spells it `(anonymous namespace)`, but kuna resolves the prototype by
/// address, so the shorter name only has to be a legible label.
fn qualified_name(die: &DieSnap, dies: &BTreeMap<usize, DieSnap>) -> String {
    let mut path: Vec<&str> = Vec::new();
    let mut cur = die.parent;
    // The DIE tree is finite and acyclic, but bound the walk anyway: a corrupt
    // parent link must not hang the loader.
    for _ in 0..MAX_SCOPE_DEPTH {
        let Some(off) = cur else { break };
        let Some(p) = dies.get(&off) else { break };
        if matches!(
            p.tag,
            gimli::DW_TAG_namespace
                | gimli::DW_TAG_class_type
                | gimli::DW_TAG_structure_type
                | gimli::DW_TAG_union_type
        ) && !p.name.is_empty()
        {
            path.push(&p.name);
        }
        cur = p.parent;
    }
    if path.is_empty() {
        return die.name.clone();
    }
    path.reverse();
    path.push(&die.name);
    path.join("::")
}

/// Ancestry-walk bound (a C++ nesting depth no real program reaches).
const MAX_SCOPE_DEPTH: u32 = 32;

/// Build [`PrototypePieces`] for a resolved subprogram.
///
/// Unlike the name-only builder this NEVER drops the whole prototype for one
/// unmappable parameter: a type the DIE switch cannot build degrades to an
/// `undefined<n>` of the DIE's own byte size ([`degrade_datatype`]), so a single
/// exotic member type costs that one parameter's type rather than the function's
/// entire signature — names and every other type survive.
pub(super) fn build_pieces(
    res: &ResolvedSub,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> Option<PrototypePieces> {
    let mut walk = TypeWalk::new();
    let outtype = build_datatype(res.type_ref, dies, types, word_size, &mut walk, true);

    let mut intypes = Vec::with_capacity(res.params.len());
    let mut innames = Vec::with_capacity(res.params.len());
    for &poff in &res.params {
        let Some(p) = dies.get(&poff) else { continue };
        let origin = p.origin_ref.and_then(|o| dies.get(&o));
        let type_ref = p.type_ref.or_else(|| origin.and_then(|o| o.type_ref));
        let ty = build_param_type(type_ref, dies, types, word_size, &mut walk)?;
        let name = if p.name.is_empty() {
            origin.map(|o| o.name.clone()).unwrap_or_default()
        } else {
            p.name.clone()
        };
        intypes.push(ty);
        innames.push(name);
    }

    Some(PrototypePieces {
        name: res.name.clone(),
        outtype,
        intypes,
        innames,
        first_var_arg_slot: res.first_var_arg_slot,
        output_storage: None,
        input_storage: Vec::new(),
    })
}

/// Map one parameter's type, degrading rather than failing.
///
/// Two things force the degrade. A tag the switch cannot map at all is the
/// obvious one. The subtler one is an **aggregate passed by VALUE**: a
/// `DW_TAG_class_type`/`structure_type` maps to a NAMED OPAQUE type of size 0
/// (fields are the sibling increment), and handing storage assignment a
/// zero-width by-value parameter is not merely imprecise — it walked off the end
/// of the parameter storage model and failed `std::vector<int>
/// ::_M_realloc_insert` outright. Any built type of non-positive size is
/// therefore replaced by an `undefined<n>` at the DIE's own `DW_AT_byte_size`.
fn build_param_type(
    off: Option<usize>,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
    walk: &mut TypeWalk,
) -> Option<Rc<Datatype>> {
    match build_datatype(off, dies, types, word_size, walk, true) {
        Some(t) if t.get_size() > 0 => Some(t),
        _ => degrade_datatype(off, dies, types),
    }
}

/// The stand-in for a parameter type [`build_param_type`] rejected: an
/// `undefined<n>` at the type DIE's own `DW_AT_byte_size` when that is a width
/// the type factory builds, else pointer-width.
///
/// Getting the WIDTH right is what matters — parameter storage assignment reads
/// the size, so a plausible width keeps the rest of the parameter list on its
/// real storage while only this one parameter loses its name-level type. The
/// qualifier chain is stripped first, because the byte size lives on the
/// aggregate, not on the `const`/`typedef` in front of it.
fn degrade_datatype(
    off: Option<usize>,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
) -> Option<Rc<Datatype>> {
    let size = off
        .and_then(|o| dies.get(&o))
        .map(|d| super::strip_qualifiers(d, dies).0)
        .and_then(|d| d.byte_size)
        .filter(|&b| matches!(b, 1 | 2 | 4 | 8))
        .map(|b| b as i32)
        .unwrap_or_else(|| types.get_size_of_pointer());
    types.get_base(size, type_metatype::TYPE_UNKNOWN).ok()
}

/// Collect the named, typed `DW_OP_fbreg` stack locals of a resolved subprogram,
/// chasing each child's own `DW_AT_abstract_origin` for the name and type an
/// out-of-line concrete instance leaves on its abstract counterpart.
///
/// The same direct-children / single-`DW_OP_fbreg` scope as the name-only
/// collector; a child that cannot be fully grounded is skipped, never a failure.
pub(super) fn collect_fbreg_locals(
    func_addr: u64,
    sub: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
    cfa: i64,
    out: &mut Vec<crate::pass::LocalFact>,
) {
    let mut walk = TypeWalk::new();
    for &coff in &sub.children {
        let Some(child) = dies.get(&coff) else { continue };
        if !matches!(child.tag, gimli::DW_TAG_variable | gimli::DW_TAG_formal_parameter) {
            continue;
        }
        let Some(fbreg) = child.fbreg_location else { continue };
        let origin = child.origin_ref.and_then(|o| dies.get(&o));
        let name = if child.name.is_empty() {
            origin.map(|o| o.name.clone()).unwrap_or_default()
        } else {
            child.name.clone()
        };
        if name.is_empty() {
            continue;
        }
        let type_ref = child.type_ref.or_else(|| origin.and_then(|o| o.type_ref));
        // A zero-width type (the named-opaque aggregate) would map a zero-extent
        // stack symbol that covers no access; skip rather than shadow the slot.
        let Some(ty) = build_datatype(type_ref, dies, types, word_size, &mut walk, true)
            .filter(|t| t.get_size() > 0)
        else {
            continue;
        };
        out.push(crate::pass::LocalFact {
            func_addr,
            name,
            type_: ty,
            stack_offset: cfa.wrapping_add(fbreg),
        });
    }
}
