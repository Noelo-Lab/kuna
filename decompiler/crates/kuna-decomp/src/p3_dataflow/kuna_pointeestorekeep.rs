//! A store of a register value into a global stays where the binary makes it
//! when the value is also used as an address, and a read of the global the
//! binary makes stays a read of the global.
//!
//! `RulePropagateCopy` rewrites a marker (`MULTIEQUAL`, `INDIRECT`) that reads a
//! global's `COPY` to read the `COPY`'s input.  The `COPY` then loses its last
//! reader and dies, and chapter 06's forced marker merge joins the value with the
//! global: every later use of the value prints as a read of the global.  A
//! dereference through it then takes the global's pointee type, which the output
//! never states (`gc = &a0[a1]; v1 = gc[2]; touch();` reads bytes once `gc` is
//! declared `char *`).
//!
//! [`declines`] keeps that `COPY` when the value reaches the address of a `LOAD`
//! or `STORE`, or the base of a `PTRADD` or `PTRSUB`, directly or through the
//! additions that offset it; chapter 06's `kuna_pointeevalue` then refuses the
//! optional join, so the value keeps its own variable.  It also keeps an
//! ordinary reader of the global's `COPY` reading the global when that reader
//! uses it as an address and the stored value is not otherwise one: such a
//! reader is a load of the global the binary makes (`-O0` reloads every
//! global), and it keeps printing as one.  This runs
//! before types exist, when `q + 1` is still an integer addition, so every `+`
//! leading to an address counts.

use std::collections::BTreeSet;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varnode::Varnode;

/// Does an operation with opcode `code` read its operand in `slot` through the
/// operand's pointee type?  A load's or store's address does, and the base of a
/// `PTRADD` or `PTRSUB`, which C spells `p[i]`, `&p[i]` or `p->f`.
pub fn reads_pointee(code: OpCode, slot: int4) -> bool {
    match code {
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE => slot == 1,
        OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => slot == 0,
        _ => false,
    }
}

/// Does an operation with opcode `code` offset its operand in `slot`, so that
/// its result is an address whenever the operand is?  Either side of `+` and
/// the left side of `-`.
pub fn offsets_address(code: OpCode, slot: int4) -> bool {
    match code {
        OpCode::CPUI_INT_ADD => true,
        OpCode::CPUI_INT_SUB => slot == 0,
        _ => false,
    }
}

/// How many varnodes a walk visits before it answers "used as an address"
/// anyway: keeping a value apart from a global is always correct.
pub const WALK_BOUND: usize = 256;

/// Must `RulePropagateCopy` leave `vn`, the output of `COPY invn`, as the input
/// of `op`?  Only when `vn` is a global and `invn` a value the function
/// computes (a parameter never merges with a global) that is not just the
/// global read back.  A marker, or a `COPY` into the same global, keeps `vn`
/// when the stored value is used as an address and no earlier value of the
/// global is read after the store.  Any other reader is a load of the global
/// the binary makes, and keeps `vn` when it uses what it loads as an address
/// while the stored value itself is not one: the value then still joins the
/// global, and the dereference prints through the global the binary reads.
pub fn declines(data: &Funcdata, op: OpId, vn: VarnodeId, invn: VarnodeId) -> bool {
    let (Some(v), Some(iv), Some(reader)) = (
        data.vbank().get(vn),
        data.vbank().get(invn),
        data.obank().get(op),
    ) else {
        return false;
    };
    if !v.is_persist() || !computed(iv) || holds_global(data, invn, v) {
        return false;
    }
    let same = |o: Option<VarnodeId>| {
        o.and_then(|o| data.vbank().get(o))
            .is_some_and(|out| out.get_addr() == v.get_addr() && out.get_size() == v.get_size())
    };
    if reader.is_marker() || (reader.code() == OpCode::CPUI_COPY && same(reader.get_out())) {
        used_as_address(data, invn) && !old_value_read_after(data, vn)
    } else {
        reads_address(data, op, vn) && !used_as_address(data, invn)
    }
}

/// Is an earlier value of the global that `vn`'s `COPY` stores to still read
/// after the store, directly or through a copy into a register?  The global's
/// forced merge then keeps the stored value apart anyway, and keeping the store
/// in place would only make chapter 06 copy that earlier value far from where
/// the binary reads it.
fn old_value_read_after(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let Some(store) = v.get_def() else {
        return false;
    };
    let after = |a: OpId, b: OpId| {
        let (Some(x), Some(y)) = (data.obank().get(a), data.obank().get(b)) else {
            return false;
        };
        match (x.get_parent(), y.get_parent()) {
            (Some(p), Some(q)) if p == q => {
                x.get_seq_num().get_order() < y.get_seq_num().get_order()
            }
            (Some(p), Some(q)) => data.bblocks_ref().dominates(p, Some(q)),
            _ => false,
        }
    };
    for w in data.vbank().iter_loc_size_addr(v.get_size(), v.get_addr()) {
        let Some(wv) = data.vbank().get(w) else {
            continue;
        };
        if w == vn || wv.get_def().is_some_and(|d| d == store || after(store, d)) {
            continue;
        }
        let mut readers: Vec<OpId> = wv.descend_iter().collect();
        let mut seen = BTreeSet::new();
        while let Some(r) = readers.pop() {
            if !seen.insert(r) || seen.len() > WALK_BOUND {
                continue;
            }
            let Some(rop) = data.obank().get(r) else {
                continue;
            };
            if rop.is_dead() || rop.is_marker() {
                continue;
            }
            if after(store, r) {
                return true;
            }
            if rop.code() == OpCode::CPUI_COPY {
                if let Some(out) = rop
                    .get_out()
                    .and_then(|o| data.vbank().get(o))
                    .filter(|o| !o.is_persist())
                {
                    readers.extend(out.descend_iter());
                }
            }
        }
    }
    false
}

/// Is `v` a value the function computes: not a global, a frame location, a
/// constant or a parameter?
fn computed(v: &Varnode) -> bool {
    !v.is_persist() && !v.is_addr_tied() && !v.is_constant() && !v.is_input()
}

/// Is `value` only the global `global` read back, through `COPY`s, `INDIRECT`s
/// and `MULTIEQUAL`s?  Merging such a value with the global is upstream's
/// business: it prints as the global because it is the global.
fn holds_global(data: &Funcdata, value: VarnodeId, global: &Varnode) -> bool {
    let mut stack = vec![value];
    let mut seen = BTreeSet::new();
    while let Some(x) = stack.pop() {
        if !seen.insert(x) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return false;
        }
        let Some(xv) = data.vbank().get(x) else {
            return false;
        };
        if xv.get_addr() == global.get_addr() && xv.get_size() == global.get_size() {
            continue;
        }
        let Some(def) = xv.get_def().and_then(|d| data.obank().get(d)) else {
            return false;
        };
        match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => stack.extend(def.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => {
                stack.extend((0..def.num_input()).filter_map(|i| def.get_in(i)))
            }
            _ => return false,
        }
    }
    true
}

/// Does `op` use what it reads from `vn` as an address: [`reads_pointee`] in
/// the slot `vn` fills, or a result that is copied or offset on to such a use?
fn reads_address(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    let code = o.code();
    let mut passes = false;
    for slot in 0..o.num_input() {
        if o.get_in(slot) != Some(vn) {
            continue;
        }
        if reads_pointee(code, slot) {
            return true;
        }
        passes |= offsets_address(code, slot) || code == OpCode::CPUI_COPY;
    }
    let out = o
        .get_out()
        .filter(|&out| data.vbank().get(out).is_some_and(|v| !v.is_persist()));
    passes && out.is_some_and(|out| walk(data, vec![(out, false)]))
}

/// Does the value `start` reach an operation that [`reads_pointee`], directly or
/// through additions that [`offsets_address`]?
///
/// The value is every varnode `Merge` could join with it: the `COPY`s,
/// `INDIRECT`s and `MULTIEQUAL`s that carry it unchanged, either way.  A sum
/// computed from it is followed forward only.  Globals and constants end the
/// walk.
fn used_as_address(data: &Funcdata, start: VarnodeId) -> bool {
    walk(data, vec![(start, true)])
}

/// The walk behind [`used_as_address`] from `stack`, whose entries are a
/// varnode and whether the value's copies are followed backward from it too.
fn walk(data: &Funcdata, mut stack: Vec<(VarnodeId, bool)>) -> bool {
    let mut seen = BTreeSet::new();
    let carries = |x: VarnodeId| {
        data.vbank()
            .get(x)
            .is_some_and(|v| !v.is_persist() && !v.is_constant())
    };
    while let Some((x, member)) = stack.pop() {
        if !seen.insert((x, member)) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return true;
        }
        let Some(xv) = data.vbank().get(x) else {
            continue;
        };
        if member {
            if let Some(def) = xv.get_def().and_then(|d| data.obank().get(d)) {
                let joined = match def.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => 1,
                    OpCode::CPUI_MULTIEQUAL => def.num_input(),
                    _ => 0,
                };
                stack.extend(
                    (0..joined)
                        .filter_map(|i| def.get_in(i))
                        .filter(|&i| carries(i))
                        .map(|i| (i, true)),
                );
            }
        }
        for d in xv.descend_iter() {
            let Some(dop) = data.obank().get(d) else {
                continue;
            };
            if dop.is_dead() {
                continue;
            }
            let code = dop.code();
            let out = dop.get_out().filter(|&o| carries(o));
            match code {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
                    stack.extend(out.map(|o| (o, member)))
                }
                OpCode::CPUI_INDIRECT => {
                    if dop.get_in(0) == Some(x) {
                        stack.extend(out.map(|o| (o, member)));
                    }
                }
                _ => {
                    for slot in 0..dop.num_input() {
                        if dop.get_in(slot) != Some(x) {
                            continue;
                        }
                        if reads_pointee(code, slot) {
                            return true;
                        }
                        if offsets_address(code, slot) {
                            stack.extend(out.map(|o| (o, false)));
                        }
                    }
                }
            }
        }
    }
    false
}
