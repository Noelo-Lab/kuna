//! (kuna) A value a function returns in a floating-point register is a float (P5).
//!
//! A calling convention that reserves registers for floating point (`xmm0` on
//! x86-64, `s0`/`d0` on hard-float ARM and AArch64, `ST0` on i386) makes the
//! register itself the declaration of what a function returns there: a float of
//! the register's width. The type fold hears nothing of it when the value is only
//! moved -- a constant, or a float callee's result handed back -- so
//! `float qnanf_(void) { return __builtin_nanf(""); }` printed as
//! `unsigned int qnanf_(void) { return 0x7fc00000; }`, and every caller that used
//! the result as a float printed `(float)qnanf_()`, a value conversion of the
//! bits.
//!
//! [`float_register_vote`] offers the float for a value a RETURN reads from a
//! float-class output register, where the fold says no more than an integer or
//! raw bytes. It is refused wherever `protoorder` refuses a callee's float vote:
//! an integer op computes with the value, it is stored, pieced or handed on
//! outside a float register, it is read from or written to a global, or it is
//! loaded through a pointer the function also moves integers through at that
//! width. It is refused where the value is one of the function's own inputs,
//! whose type its callers decide: `float pass(float x) { return x; }` called as
//! `pass(p[1])` of an `int *` would print a conversion by value where the binary
//! hands the bits on. It is refused where the value is handed to a call whose
//! parameter no declaration or recovery makes a float, or is the result of a call
//! whose return none does -- `f2u(a0)` beside `unsigned int f2u(unsigned int)`
//! converts. And it is refused for a NaN constant `NAN` does not spell exactly,
//! and for one half of an ARM register pair the function uses whole (a `double`
//! in `d0`). The function returns one type in the register, so a refusal for
//! any value a RETURN hands back there refuses them all: `if (k) return g.f;
//! return 0.0f;`, with a `ret` on each path, reads a global on one of them.
//!
//! [`float_input_vote`] is the same declaration on the way in: a parameter the
//! convention passes in a float register is a float of the register's width.
//! `void fs(int *p, double b, double *q) { *q = b; *p = 1; }` only stores `b`,
//! so the fold left it raw bytes and the prototype printed `fs(unsigned long
//! a0, ..)`, an integer the convention would pass in `rdi`. The vote speaks
//! only where the fold says nothing and every use of the value is one a float
//! has: it is copied, stored, loaded, computed with as a float, or handed to a
//! call in a float register. An integer op on its bits, a use as an address, a
//! call that takes it in a general register or as an integer, and a store
//! through a pointer the function also moves integers through each refuse it.
//! So does a return, and a global not declared a float of the value's width:
//! what a function hands back of its input is its callers' to type, and a
//! global has one declaration for every function, as for the return vote.
//!
//! [`argument_requirement`] and [`argument_vote`] keep the callers in step.  An
//! argument with integer evidence passed to a parameter that prints as a float
//! is cast, which prints as a reinterpretation of its bits; an untyped read of
//! read-only memory passed there takes the float type.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_class, type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::p4_calls::fspec::ParamListStandard;
use crate::varnode::Varnode;

/// The float `vn` is, when the function returns it in a float register and
/// nothing the function does with it says otherwise; `ct` is the fold's type
/// for it.
pub(crate) fn float_register_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    vote(data, vn, ct, None)
}

/// The float the function's input `vn` is, when it arrives in a float-class
/// register of the function's own model, the fold says nothing of it (`ct` is
/// raw bytes), and [`only_moved_as_a_float`] holds for its value.
pub(crate) fn float_input_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    if ct.get_metatype() != type_metatype::TYPE_UNKNOWN {
        return None;
    }
    let node = data.vbank().get(vn)?;
    let size = node.get_size();
    if !node.is_input() || !matches!(size, 4 | 8) || node.is_type_lock() {
        return None;
    }
    let proto = data.get_func_proto();
    if !proto.has_model() || proto.is_input_locked() || !float_class(data, proto.model().input_opt(), node) {
        return None;
    }
    if !only_moved_as_a_float(data, vn) || !spells_exactly(data, vn) {
        return None;
    }
    data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
}

/// Is every varnode of `vn`'s value family made and used only the way a float
/// is: copied and joined, stored, loaded, computed with by float ops, returned
/// by a call that returns a float, handed to a call in a float register, or
/// another input in one?  Anything else -- an integer op on the bits, an
/// address, a call that takes it in a general register or as an integer, a
/// return, a store through a pointer the function also moves integers through,
/// a global that is not declared a float of its width -- is evidence the vote
/// loses to.
fn only_moved_as_a_float(data: &Funcdata, vn: VarnodeId) -> bool {
    let family = crate::kuna_protoorder::value_family(data, vn);
    if family.len() >= 128 {
        return false;
    }
    let proto = data.get_func_proto();
    let data_space = data.get_arch().manage().get_default_data_space().map(Rc::clone);
    let globals: std::cell::RefCell<Vec<(Address, int4)>> = std::cell::RefCell::new(Vec::new());
    let through_a_global = |op: crate::context::OpId, p: VarnodeId, size: int4| {
        if !addresses_a_global(data, p) {
            return true;
        }
        let Some(at) = constant_global(data, op, p) else { return false };
        globals.borrow_mut().push((at, size));
        true
    };
    let moved = family.iter().all(|&v| {
        let Some(node) = data.vbank().get(v) else { return false };
        let declared_float = node.is_type_lock() && node.get_type().get_metatype() == type_metatype::TYPE_FLOAT;
        if node.is_type_lock() && !declared_float {
            return false;
        }
        let global = node.is_persist()
            || data_space.as_ref().is_some_and(|d| node.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, d)));
        if global && !declared_float {
            globals.borrow_mut().push((node.get_addr().clone(), node.get_size()));
        }
        let made = match node.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o))) {
            None => v == vn || (node.is_input() && float_class(data, proto.model().input_opt(), node)),
            Some((d, o)) => match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => true,
                OpCode::CPUI_INDIRECT => !o.is_indirect_creation(),
                OpCode::CPUI_LOAD => {
                    !crate::kuna_protoorder::moves_integers_beside(data, d, node.get_size())
                        && o.get_in(1).is_some_and(|p| through_a_global(d, p, node.get_size()))
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => call_returns_a_float(data, d),
                code => makes_a_float(code),
            },
        };
        made && node.descend_iter().all(|r| {
            let Some(o) = data.obank().get(r) else { return true };
            match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => true,
                OpCode::CPUI_STORE => {
                    o.get_in(1) != Some(v)
                        && o.get_in(0) != Some(v)
                        && !crate::kuna_protoorder::moves_integers_beside(data, r, node.get_size())
                        && o.get_in(1).is_some_and(|p| through_a_global(r, p, node.get_size()))
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    o.get_in(0) != Some(v)
                        && (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).all(|s| {
                            crate::kuna_protoorder::float_read_width(data, r, s) == Some(node.get_size())
                                || (!crate::kuna_protoorder::reads_a_float(data, r, s)
                                    && !crate::kuna_protoorder::reads_other_than_a_float(data, r, s)
                                    && passed_in_a_float_register(data, r, s))
                        })
                }
                code => reads_a_float(code),
            }
        })
    });
    moved && globals.into_inner().iter().all(|(at, size)| crate::kuna_floatglobals::float_only(data, at, *size))
}

/// Is the pointer `ptr` the address of a global, or of an element or field of
/// one: a constant in the data space, or such a constant added to an index?
/// A global has one declaration for every function, and another function may
/// read it as an integer.
fn addresses_a_global(data: &Funcdata, ptr: VarnodeId) -> bool {
    let mut work = vec![(ptr, 0)];
    while let Some((v, depth)) = work.pop() {
        let Some(node) = data.vbank().get(v) else { continue };
        if node.is_constant() {
            if node.get_offset() != 0 {
                return true;
            }
            continue;
        }
        let Some((d, o)) = node.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o))) else { continue };
        if depth >= 6 {
            continue;
        }
        match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => {
                work.extend(o.get_in(0).map(|i| (i, depth + 1)));
            }
            OpCode::CPUI_INT_ADD => {
                for i in (0..2).filter_map(|k| o.get_in(k)) {
                    if data.vbank().get(i).is_some_and(|n| n.is_constant()) {
                        if crate::kuna_ptrfromuse::constant_may_be_global_base(data, d, i) {
                            return true;
                        }
                    } else {
                        work.push((i, depth + 1));
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// The constant address the pointer `ptr` of the LOAD or STORE `op` holds,
/// through copies and casts.
fn constant_global(data: &Funcdata, op: crate::context::OpId, ptr: VarnodeId) -> Option<Address> {
    let manage = data.get_arch().manage();
    let space = data
        .obank()
        .get(op)
        .and_then(|o| o.get_in(0))
        .and_then(|s| data.vbank().get(s))
        .map(|s| s.get_offset())
        .filter(|&i| i < manage.num_spaces() as u64)
        .and_then(|i| manage.get_space(i as i32).cloned())?;
    let mut cur = ptr;
    for _ in 0..8 {
        let node = data.vbank().get(cur)?;
        if node.is_constant() {
            return Some(Address::new(Rc::clone(&space), node.get_offset()));
        }
        let def = data.obank().get(node.get_def()?)?;
        if !matches!(def.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        cur = def.get_in(0)?;
    }
    None
}

/// The global `vn` reads, through copies: the data-space varnode at the root of
/// its copy chain.
fn global_read(data: &Funcdata, vn: VarnodeId) -> Option<(Address, int4)> {
    let data_space = data.get_arch().manage().get_default_data_space().map(Rc::clone)?;
    let mut cur = vn;
    for _ in 0..8 {
        let node = data.vbank().get(cur)?;
        if node.get_addr().get_space().is_some_and(|s| Rc::ptr_eq(s, &data_space)) {
            return Some((node.get_addr().clone(), node.get_size()));
        }
        let def = data.obank().get(node.get_def()?)?;
        if def.code() != OpCode::CPUI_COPY {
            return None;
        }
        cur = def.get_in(0)?;
    }
    None
}

/// The most Varnodes the walk over one global's values in one function visits:
/// every call the function makes carries the global across it in an INDIRECT.
const MAX_GLOBAL_FAMILY: usize = 4096;

/// Does this function move the `size`-byte global at `addr` only as a float:
/// every access of it is a whole one of that width, every value written there
/// comes from a float operation, a float register, memory or a constant that
/// spells, and every value read there reaches only copies, float operations,
/// stores of its bits and float registers of calls and returns?  The function's
/// own code is the one view the whole-program scan cannot miss, and a float
/// taken against it prints a conversion where the machine moves the bits
/// (`v1 = (long)gd` for a case body that adds to them).
pub(crate) fn moved_as_a_float_here(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let key = (addr.get_offset(), size);
    if let Some(known) = MOVED.with(|m| m.borrow().as_ref().and_then(|c| c.get(&key).copied())) {
        return known;
    }
    let moved = walk_the_global(data, addr, size);
    MOVED.with(|m| {
        if let Some(c) = m.borrow_mut().as_mut() {
            c.insert(key, moved);
        }
    });
    moved
}

type MovedMemo = std::collections::HashMap<(u64, int4), bool>;

thread_local! {
    static MOVED: std::cell::RefCell<Option<MovedMemo>> = const { std::cell::RefCell::new(None) };
}

/// Run `f` with [`moved_as_a_float_here`]'s answers kept by global: the walk
/// reads the function's ops, constants, prototypes and the global scope, which
/// one type-inference pass only types and never changes.
pub(crate) fn with_moved_memo<T>(f: impl FnOnce() -> T) -> T {
    struct Restore(Option<MovedMemo>);
    impl Drop for Restore {
        fn drop(&mut self) {
            MOVED.with(|m| *m.borrow_mut() = self.0.take());
        }
    }
    let _restore = Restore(MOVED.with(|m| m.borrow_mut().replace(MovedMemo::new())));
    f()
}

fn walk_the_global(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(space) = addr.get_space() else { return false };
    let (lo, hi) = (addr.get_offset(), addr.get_offset() + size as u64);
    let start = Address::new(Rc::clone(space), lo.saturating_sub(16));
    let end = Address::new(Rc::clone(space), hi);
    for id in data.vbank().iter_loc_addr_range(&start, &end) {
        let Some(node) = data.vbank().get(id) else { continue };
        let (off, end) = (node.get_offset(), node.get_offset() + node.get_size() as u64);
        if end <= lo || off >= hi || (node.get_def().is_none() && !node.is_input() && node.descend_iter().next().is_none()) {
            continue;
        }
        if off != lo || node.get_size() != size || !written_as_a_float(data, id) || !read_as_a_float(data, id) {
            return false;
        }
    }
    let Some(cspace) = data.get_arch().manage().get_constant_space().map(Rc::clone) else { return false };
    let Some(ptr) = data.get_arch().types().map(|t| t.get_size_of_pointer()) else { return false };
    let symbol = data.get_arch().global_symbol_extent(addr).map(|(first, whole)| (first, first.saturating_add(whole as u64)));
    let aggregate = symbol.filter(|&(first, last)| first <= lo && hi <= last && last - first > size as u64);
    let indexable = symbol.is_none() || aggregate.is_some();
    let (from, to) = aggregate.unwrap_or((lo, hi));
    let first = Address::new(Rc::clone(&cspace), from);
    let last = Address::new(cspace, to);
    for c in data.vbank().iter_loc_addr_range(&first, &last) {
        let Some(node) = data.vbank().get(c).filter(|n| n.is_constant() && n.get_size() == ptr) else { continue };
        let own = (lo..hi).contains(&node.get_offset());
        if !used_as_an_address(data, c, (lo, hi), indexable, own) {
            return false;
        }
    }
    true
}

/// How the address `v` reaches the output of `o`: `Some(Some(k))` moved by the
/// constant `k`, `Some(None)` by an index, `None` when `o` does not form an
/// address from it.
fn moved_by(data: &Funcdata, o: &crate::op::PcodeOp, v: VarnodeId) -> Option<Option<u64>> {
    let constant = |k: int4| o.get_in(k).and_then(|i| data.vbank().get(i)).filter(|n| n.is_constant()).map(|n| n.get_offset());
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => Some(Some(0)),
        OpCode::CPUI_INT_ADD => Some(constant(if o.get_in(0) == Some(v) { 1 } else { 0 })),
        OpCode::CPUI_INT_SUB if o.get_in(0) == Some(v) => Some(constant(1).map(u64::wrapping_neg)),
        OpCode::CPUI_PTRADD if o.get_in(0) == Some(v) => Some(constant(1).zip(constant(2)).map(|(i, e)| i.wrapping_mul(e))),
        OpCode::CPUI_PTRSUB if o.get_in(0) == Some(v) => Some(constant(1)),
        OpCode::CPUI_PTRSUB if constant(0) == Some(0) => Some(Some(0)),
        _ => None,
    }
}

/// Does every load and store through the constant address `vn`, and through
/// every pointer formed from it, leave the global `[lo, hi)` alone or move it
/// whole as a float?  A pointer at a known address must be the global's own
/// where it overlaps it; one indexed from, or joined from two addresses, may
/// reach any element, so each load and store through it must move a float of
/// the global's width.  Only an `indexable` global -- an element of a larger
/// Symbol, or one no Symbol sizes -- may be indexed from at all: from a
/// declared scalar the index reads past it.  The global's `own` address, or
/// one a constant moves it by, handed to a call, stored or returned can read
/// it as anything; a pointer only an index reaches, or anything formed from a
/// constant elsewhere in the aggregate, handed on is not the function's own
/// read or write of the global.  Any other use of a pointer the walk cannot
/// follow refuses.
fn used_as_an_address(data: &Funcdata, vn: VarnodeId, (lo, hi): (u64, u64), indexable: bool, own: bool) -> bool {
    let start = data.vbank().get(vn).map(|n| n.get_offset());
    let mut work = vec![(vn, start, own)];
    let mut seen: std::collections::HashMap<VarnodeId, (Option<u64>, bool)> = std::collections::HashMap::new();
    while let Some((v, at, kept)) = work.pop() {
        let (at, kept) = match seen.get(&v) {
            None => (at, kept),
            Some(&(prev, was)) => {
                let joined = (if prev == at { prev } else { None }, was || kept);
                if joined == (prev, was) {
                    continue;
                }
                if joined.0.is_none() && !indexable {
                    return false;
                }
                joined
            }
        };
        seen.insert(v, (at, kept));
        if seen.len() > MAX_GLOBAL_FAMILY {
            return false;
        }
        let Some(node) = data.vbank().get(v) else { return false };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
            let access = match o.code() {
                OpCode::CPUI_LOAD if o.get_in(1) == Some(v) => o.get_out().map(|x| (x, true)),
                OpCode::CPUI_STORE if o.get_in(1) == Some(v) && o.get_in(2) != Some(v) => o.get_in(2).map(|x| (x, false)),
                _ => None,
            };
            if let Some((value, load)) = access {
                let width = data.vbank().get(value).map_or(0, |n| n.get_size() as u64);
                let touches = at.is_none_or(|a| a < hi && a.saturating_add(width) > lo);
                let whole = at.is_none_or(|a| a == lo) && width == hi - lo;
                let moved = if load { read_as_a_float(data, value) } else { written_as_a_float(data, value) };
                if touches && !(whole && moved) {
                    return false;
                }
                continue;
            }
            let handed_on = match o.code() {
                OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => continue,
                OpCode::CPUI_INT_SUB if o.get_in(1) == Some(v) && o.get_in(0) != Some(v) => continue,
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => o.get_in(0) != Some(v),
                OpCode::CPUI_STORE => o.get_in(2) == Some(v) && o.get_in(1) != Some(v),
                OpCode::CPUI_RETURN => true,
                _ => false,
            };
            match (moved_by(data, o, v), o.get_out()) {
                (Some(Some(k)), Some(out)) => work.push((out, at.map(|a| a.wrapping_add(k)), kept)),
                (Some(None), Some(out)) if indexable => work.push((out, None, false)),
                _ if handed_on && !kept => {}
                _ => return false,
            }
        }
    }
    true
}

/// Is every value `vn` takes, through copies and joins, made the way a float
/// is: by a float operation, in a float register, out of memory, or as a
/// constant that spells?
fn written_as_a_float(data: &Funcdata, vn: VarnodeId) -> bool {
    let proto = data.get_func_proto();
    let mut work = vec![vn];
    let mut seen: std::collections::HashSet<VarnodeId> = std::collections::HashSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > MAX_GLOBAL_FAMILY {
            return false;
        }
        let Some(node) = data.vbank().get(v) else { return false };
        if node.is_constant() {
            if !spells(data, v) {
                return false;
            }
            continue;
        }
        let Some((d, o)) = node.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o))) else {
            let register = node.get_addr().get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
                && !node.is_persist();
            if register && !(proto.has_model() && in_a_float_entry(proto.model().input_opt(), node)) {
                return false;
            }
            continue;
        };
        match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => work.extend(o.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => work.extend((0..o.num_input()).filter_map(|k| o.get_in(k))),
            OpCode::CPUI_INDIRECT => work.extend(o.get_in(0)),
            OpCode::CPUI_LOAD => {}
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let model = data.get_call_specs_index(d).map(|i| data.get_call_specs(i)).filter(|fc| fc.proto().has_model());
                if !model.is_some_and(|fc| in_a_float_entry(fc.proto().model().output_list(), node)) {
                    return false;
                }
            }
            code if makes_a_float(code) => {}
            _ => return false,
        }
    }
    true
}

/// Does every value read out of `vn` reach, through copies and joins, only
/// float operations, stores of its bits, and float registers of calls and
/// returns?
fn read_as_a_float(data: &Funcdata, vn: VarnodeId) -> bool {
    let proto = data.get_func_proto();
    let mut work = vec![vn];
    let mut seen: std::collections::HashSet<VarnodeId> = std::collections::HashSet::new();
    while let Some(v) = work.pop() {
        if !seen.insert(v) {
            continue;
        }
        if seen.len() > MAX_GLOBAL_FAMILY {
            return false;
        }
        let Some(node) = data.vbank().get(v) else { return false };
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
                    if o.code() == OpCode::CPUI_INDIRECT && o.get_in(0) != Some(v) {
                        continue;
                    }
                    work.extend(o.get_out());
                }
                OpCode::CPUI_STORE => {
                    if o.get_in(2) != Some(v) || o.get_in(1) == Some(v) {
                        return false;
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    if o.get_in(0) == Some(v)
                        || !(1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).all(|s| passed_in_a_float_register(data, r, s))
                    {
                        return false;
                    }
                }
                OpCode::CPUI_RETURN => {
                    if !(proto.has_model() && in_a_float_entry(proto.model().output_list(), node)) {
                        return false;
                    }
                }
                code if reads_a_float(code) => {}
                _ => return false,
            }
        }
    }
    true
}

/// Is `node` in a float-class entry of `list`, whole or a half of one?
fn in_a_float_entry(list: Option<&ParamListStandard>, node: &Varnode) -> bool {
    list.and_then(|l| l.find_entry(node.get_addr(), node.get_size(), true).map(|i| l.get_entry()[i].get_type()))
        == Some(type_class::TYPECLASS_FLOAT)
}

/// The float a call argument read straight out of read-only memory takes from
/// the parameter it is passed to: the callee's unlocked parameter there prints
/// as a float, the call passes it in a float register, and every other use of
/// the read is one a float has.  A literal-pool `movsd dat_2010(%rip),%xmm0`
/// before `call fy` is the `2.5` the source passed, and an untyped read there
/// printed as an integer the parameter converts.
pub(crate) fn argument_vote(
    data: &Funcdata,
    fc: &crate::fspec::FuncCallSpecs,
    op: crate::context::OpId,
    slot: int4,
) -> Option<Rc<Datatype>> {
    if fc.proto().get_param(slot - 1).is_some_and(|p| p.is_type_locked()) {
        return None;
    }
    let vn = data.obank().get(op)?.get_in(slot)?;
    let node = data.vbank().get(vn)?;
    let size = node.get_size();
    if !matches!(size, 4 | 8)
        || node.is_constant()
        || node.is_type_lock()
        || !matches!(node.get_type().get_metatype(), type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_FLOAT)
    {
        return None;
    }
    let global = if node.is_read_only() { None } else { Some(global_read(data, vn).filter(|&(_, w)| w == size)?.0) };
    let float_global = global.is_some();
    let float_argument = |call: crate::context::OpId, s: int4| {
        passed_in_a_float_register(data, call, s)
            && (crate::kuna_protoorder::float_read_width(data, call, s) == Some(size)
                || (float_global
                    && !crate::kuna_protoorder::reads_a_float(data, call, s)
                    && !crate::kuna_protoorder::reads_other_than_a_float(data, call, s)))
    };
    let family = crate::kuna_protoorder::value_family(data, vn);
    if family.len() >= 128 {
        return None;
    }
    let floats = family.iter().all(|&v| {
        let Some(member) = data.vbank().get(v) else { return false };
        let made = member.get_def().and_then(|d| data.obank().get(d)).is_none_or(|o| {
            matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL)
                || (o.code() == OpCode::CPUI_INDIRECT && !o.is_indirect_creation())
        });
        made && !member.is_type_lock()
            && member.descend_iter().all(|r| {
                data.obank().get(r).is_some_and(|o| match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => true,
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        o.get_in(0) != Some(v)
                            && (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).all(|s| float_argument(r, s))
                    }
                    code => reads_a_float(code),
                })
            })
    });
    if !floats || global.is_some_and(|at| !crate::kuna_floatglobals::float_only(data, &at, size)) {
        return None;
    }
    data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
}

/// The float C requires of argument `slot` of the call `op`: the callee's
/// unlocked parameter there prints as a float of the argument's width, the call
/// passes it in a float register, and the value is an integer or the result of
/// a call that does not return a float.  No integer travels in a float
/// register, so its bits got there by a reinterpretation, and the cast this
/// requirement adds prints as one: `vmov s0,r3` hands the bits of an
/// `unsigned int` to a `float` parameter, which C would convert by value.  A
/// NaN constant no literal spells gets none.
pub(crate) fn argument_requirement(
    data: &Funcdata,
    fc: &crate::fspec::FuncCallSpecs,
    op: crate::context::OpId,
    slot: int4,
) -> Option<Rc<Datatype>> {
    if fc.proto().get_param(slot - 1).is_some_and(|p| p.is_type_locked()) {
        return None;
    }
    let vn = data.obank().get(op)?.get_in(slot)?;
    let size = data.vbank().get(vn)?.get_size();
    if !matches!(size, 4 | 8)
        || !passed_in_a_float_register(data, op, slot)
        || crate::kuna_protoorder::float_read_width(data, op, slot) != Some(size)
        || !(crate::kuna_varargfloat::integer_bits(data, op, vn) || result_of_a_non_float_call(data, vn))
        || !spells(data, vn)
    {
        return None;
    }
    data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
}

/// Is `vn`, through copies, casts and truncations, the result of a call whose
/// callee no declaration, statement or vote makes return a float?  Its printed
/// declaration returns an integer (`unsigned int` for raw bytes), so the
/// call's value is the integer the printer spells.
fn result_of_a_non_float_call(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut cur = vn;
    for _ in 0..16 {
        let Some((d, o)) = data.vbank().get(cur).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d).map(|o| (d, o)))
        else {
            return false;
        };
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => return !call_returns_a_float(data, d),
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_SUBPIECE => {
                let Some(i) = o.get_in(0) else { return false };
                cur = i;
            }
            _ => return false,
        }
    }
    false
}

/// Does the call `call` pass its argument `slot` in a float-class register of
/// its model?  Asked of where the argument lives, not of the value handed in:
/// copy propagation leaves `vmov r0,s0; bl put3` reading `s0`.
fn passed_in_a_float_register(data: &Funcdata, call: crate::context::OpId, slot: int4) -> bool {
    let Some(fc) = data.get_call_specs_index(call).map(|i| data.get_call_specs(i)) else { return false };
    let proto = fc.proto();
    let storage = fc
        .final_input_storage()
        .get((slot - 1) as usize)
        .cloned()
        .or_else(|| proto.get_param(slot - 1).map(|p| (p.get_address(), p.get_size())));
    let Some((addr, size)) = storage.filter(|_| proto.has_model()) else { return false };
    proto
        .model()
        .input_opt()
        .and_then(|l| l.find_entry(&addr, size, true).map(|i| l.get_entry()[i].get_type()))
        == Some(type_class::TYPECLASS_FLOAT)
}

/// Does the op `code` produce a float from its inputs?
pub(crate) fn makes_a_float(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_INT2FLOAT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
    )
}

/// Does the op `code` read its inputs as floats?
pub(crate) fn reads_a_float(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
            | OpCode::CPUI_FLOAT_TRUNC
    )
}

/// [`float_register_vote`], taking the result of the call `jump` for whatever
/// the function returns: an import stub's jump through its slot.
fn vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>, jump: Option<crate::context::OpId>) -> Option<Rc<Datatype>> {
    if !matches!(ct.get_metatype(), type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_INT | type_metatype::TYPE_UINT) {
        return None;
    }
    let node = data.vbank().get(vn)?;
    let size = node.get_size();
    if !matches!(size, 4 | 8 | 10) || node.is_type_lock() || !returned_in_a_float_register(data, vn, node) {
        return None;
    }
    let float = data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()?;
    if returned_beside(data, vn).into_iter().any(|r| refuses(data, r, &float, jump)) {
        return None;
    }
    Some(float)
}

/// Does anything about the value `vn`, returned in a float register, refuse
/// the float `float` for it?
fn refuses(data: &Funcdata, vn: VarnodeId, float: &Rc<Datatype>, jump: Option<crate::context::OpId>) -> bool {
    if data.vbank().get(vn).is_some_and(|n| n.is_constant()) {
        return !spells(data, vn);
    }
    let family = crate::kuna_protoorder::value_family(data, vn);
    family.iter().any(|&v| data.vbank().get(v).is_some_and(|n| n.is_input()))
        || crate::kuna_protoorder::input_refuses(data, vn, float)
        || family.iter().any(|&v| crosses_a_call_as_other_than_a_float(data, v, jump))
        || family.iter().any(|&v| loaded_beside_integers(data, v))
        || !spells_exactly(data, vn)
}

/// `vn` and every other value a live RETURN hands back in a slot `vn` is
/// returned in: the function returns one type there, so a float is refused for
/// all of them when it is refused for one. `if (ready) return packed.f; return
/// 0.0f;` returns a global on one path and a constant on the other, and the
/// constant alone would make the global a float.
pub(crate) fn returned_beside(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let Some(node) = data.vbank().get(vn) else { return vec![vn] };
    let slots: Vec<int4> = node
        .descend_iter()
        .filter_map(|r| data.obank().get(r).filter(|o| o.code() == OpCode::CPUI_RETURN))
        .flat_map(|o| (1..o.num_input()).filter(|&s| o.get_in(s) == Some(vn)).collect::<Vec<_>>())
        .collect();
    let mut out = vec![vn];
    for r in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let Some(o) = data.obank().get(r).filter(|o| !o.is_dead()) else { continue };
        for &s in &slots {
            if let Some(v) = o.get_in(s).filter(|v| !out.contains(v)) {
                out.push(v);
            }
        }
    }
    out
}

/// Does the value `v` cross a call as something no declaration or recovery
/// makes a float: handed to a parameter that is not one, or produced by a call
/// (its output, or the register a call leaves behind) whose return is not one?
fn crosses_a_call_as_other_than_a_float(data: &Funcdata, v: VarnodeId, jump: Option<crate::context::OpId>) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    let handed = node.descend_iter().any(|r| {
        data.obank().get(r).is_some_and(|o| {
            matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                && (1..o.num_input())
                    .filter(|&s| o.get_in(s) == Some(v))
                    .any(|s| !crate::kuna_protoorder::reads_a_float(data, r, s))
        })
    });
    if handed {
        return true;
    }
    let Some(def) = node.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o))) else { return false };
    match def.1.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => Some(def.0) != jump && !call_returns_a_float(data, def.0),
        OpCode::CPUI_INDIRECT if def.1.is_indirect_creation() => {
            let call = def
                .1
                .get_in(1)
                .and_then(|i| data.vbank().get(i))
                .map(|i| crate::context::OpId::from(slotmap::KeyData::from_ffi(i.get_offset())));
            call.is_none_or(|c| data.obank().get(c).is_none() || !call_returns_a_float(data, c))
        }
        _ => false,
    }
}

/// Is `v` loaded through a pointer the function also moves integers through at
/// its width?  The float would type the pointer, and those accesses with it.
fn loaded_beside_integers(data: &Funcdata, v: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(v) else { return false };
    node.get_def().is_some_and(|d| {
        data.obank().get(d).is_some_and(|o| o.code() == OpCode::CPUI_LOAD)
            && crate::kuna_protoorder::moves_integers_beside(data, d, node.get_size())
    })
}

/// Does the call `op`'s callee return a float: a locked output of one, a
/// recovered return its own decompile stated, or one the float-register vote
/// gave it?
fn call_returns_a_float(data: &Funcdata, op: crate::context::OpId) -> bool {
    let Some(fc) = data.get_call_specs_index(op).map(|i| data.get_call_specs(i)) else { return false };
    let proto = fc.proto();
    let float = |t: &Datatype| t.get_metatype() == type_metatype::TYPE_FLOAT;
    if proto.is_output_locked() {
        return proto.get_output_type().is_some_and(|t| float(t));
    }
    let entry = fc.get_entry_address();
    let Some(k) = entry.get_space().map(|s| (s.get_index(), entry.get_offset())) else { return false };
    match data.kuna_callret_stated(k) {
        Some(stated) => float(&stated.ct),
        None => data.kuna_callee_returns(k) == Some(crate::kuna_voidret::Returns::Float),
    }
}

/// The type an import stub's jump through its slot hands back: the stub's own
/// return, when that is a float -- declared (`double strtod(..)`) or the float
/// its register makes it -- and the stub returns the jump's result.  Otherwise
/// the result is untyped and the stub prints `v1 = (float)(*dat_4018)()`,
/// converting what the target handed back.
pub(crate) fn jump_result_type(data: &Funcdata, op: crate::context::OpId, size: int4) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let out = o.get_out()?;
    let at = o.get_addr().clone();
    let returned = data.vbank().get(out)?.descend_iter().any(|r| {
        data.obank().get(r).is_some_and(|ret| {
            ret.code() == OpCode::CPUI_RETURN && !ret.is_dead() && *ret.get_addr() == at && (1..ret.num_input()).any(|s| ret.get_in(s) == Some(out))
        })
    });
    if !returned {
        return None;
    }
    let proto = data.get_func_proto();
    if proto.is_output_locked() {
        let t = proto.get_output_type()?;
        return (t.get_metatype() == type_metatype::TYPE_FLOAT && t.get_size() == size).then(|| Rc::clone(t));
    }
    let unknown = data.get_arch().types()?.get_base(size, type_metatype::TYPE_UNKNOWN).ok()?;
    vote(data, out, &unknown, Some(op))
}

/// Is `node` in a float-class entry of `list`, and a whole float of it?  An
/// entry narrower than a `double` (ARM hard-float's `s0`..`s15`) is also half
/// of one (`d0` over `s0` and `s1`), so a value there is a `float` only when
/// the function never uses the pair whole: `third()` loads `1.0 / 3.0` into
/// `d0`, and the `s0` left once the load is folded is not a float.
pub(crate) fn float_class(data: &Funcdata, list: Option<&ParamListStandard>, node: &Varnode) -> bool {
    let Some((l, i)) = list.and_then(|l| l.find_entry(node.get_addr(), node.get_size(), true).map(|i| (l, i))) else {
        return false;
    };
    let entry = &l.get_entry()[i];
    if entry.get_type() != type_class::TYPECLASS_FLOAT {
        return false;
    }
    if entry.get_size() == 10 {
        return node.get_size() == 10;
    }
    if entry.get_size() >= 8 {
        return true;
    }
    let Some(space) = node.get_addr().get_space() else { return false };
    !data.kuna_float_pair_half((space.get_index(), node.get_offset())) && !pair_used_whole(data, node.get_addr(), node.get_size())
}

/// Does an op of the function read or write one value spanning the whole pair
/// of registers `[addr, addr+size)` belongs to, a `double` in `d0` over `s0`
/// and `s1`?  The trials a call or a return carries for every register it may
/// pass or return in say nothing, and `s1` read on its own is a second float
/// (`clamp(float x, float lo, float hi)` receives `lo` there).
fn pair_used_whole(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(space) = addr.get_space() else { return false };
    let half = size as u64;
    let pair = addr.get_offset() & !(2 * half - 1);
    let lo = Address::new(Rc::clone(space), pair.saturating_sub(64));
    let hi = Address::new(Rc::clone(space), pair + 2 * half);
    let computes = |op: crate::context::OpId| {
        data.obank().get(op).is_some_and(|o| {
            !matches!(
                o.code(),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL
            )
        })
    };
    data.vbank().iter_loc_addr_range(&lo, &hi).any(|id| {
        data.vbank().get(id).is_some_and(|v| {
            let (off, end) = (v.get_offset(), v.get_offset().wrapping_add(v.get_size() as u64));
            off <= pair && pair + 2 * half <= end && (v.get_def().is_some_and(computes) || v.descend_iter().any(computes))
        })
    })
}

/// Note, before any fold can remove it, each narrow float-class register of
/// the function's model (ARM hard-float's `s0`..`s15`) whose pair the
/// function's code uses whole: `vldr d0, [pc]` loads a `double` whose low half
/// is all that is left in `s0` once the constant is folded.
pub(crate) fn note_float_pairs(data: &mut Funcdata) {
    let proto = data.get_func_proto();
    if !proto.has_model() {
        return;
    }
    let lists = [proto.model().input_opt(), proto.model().output_list()];
    let halves: Vec<(Address, int4)> = lists
        .into_iter()
        .flatten()
        .flat_map(|l| l.get_entry().iter())
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() < 8 && e.get_size() > 0)
        .map(|e| (Address::new(Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect();
    for (addr, size) in halves {
        if pair_used_whole(data, &addr, size) {
            let Some(space) = addr.get_space() else { continue };
            data.kuna_note_float_pair_half((space.get_index(), addr.get_offset()));
        }
    }
}

pub(crate) fn returned_in_a_float_register(data: &Funcdata, vn: VarnodeId, node: &Varnode) -> bool {
    let proto = data.get_func_proto();
    if !proto.has_model() || proto.is_output_locked() || data.kuna_float_return_withdrawn() {
        return false;
    }
    let returned = node.descend_iter().any(|r| {
        data.obank()
            .get(r)
            .is_some_and(|o| o.code() == OpCode::CPUI_RETURN && (1..o.num_input()).any(|s| o.get_in(s) == Some(vn)))
    });
    returned && float_class(data, proto.model().output_list(), node)
}

/// Does every constant in `vn`'s value family print as a float literal that
/// compiles back to the same bits?  The constants are the inputs of the copies
/// and joins the family is made of (a literal-pool load folds to `s0 =
/// COPY #0x7fc00123`).  `NAN` and `-NAN` are the canonical quiet NaNs; any
/// other payload, and a signalling NaN, has no spelling.
fn spells_exactly(data: &Funcdata, vn: VarnodeId) -> bool {
    crate::kuna_protoorder::value_family(data, vn).into_iter().all(|v| {
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) else {
            return true;
        };
        let inputs = match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => 1,
            OpCode::CPUI_MULTIEQUAL => def.num_input(),
            _ => 0,
        };
        (0..inputs).filter_map(|k| def.get_in(k)).all(|c| spells(data, c))
    })
}

/// Does `c`, when it is a constant, print as a float literal that compiles back
/// to the same bits?
pub(crate) fn spells(data: &Funcdata, c: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(c).filter(|n| n.is_constant()) else { return true };
    let Some(format) = data.get_arch().get_float_format(node.get_size()) else { return false };
    let bits = node.get_offset() as u64;
    if format.get_host_float(bits).1 != kuna_num::float::floatclass::nan {
        return true;
    }
    node.get_size() <= 8 && (bits == format.get_encoding(f64::NAN) || bits == format.get_encoding(-f64::NAN))
}
