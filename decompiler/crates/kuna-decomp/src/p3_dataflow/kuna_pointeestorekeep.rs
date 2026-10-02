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
//! optional join, so the value keeps its own variable.  This runs before types
//! exist, when `q + 1` is still an integer addition, so every `+` leading to an
//! address counts.
//!
//! Any other reader of the `COPY` is a load of the global the binary makes.
//! kuna's SSA gives a pointer `STORE` no effect on a global, so a load after
//! `gi = q; *pp = p;` still reads the `COPY`, and rewritten to read `q` it would
//! print the register where the binary reads memory `*pp` may have changed.
//! [`declines`] keeps such a load on the global ([`written_between`]), and keeps
//! a register copy of it from being moved past a later `STORE`.  A load that
//! uses what it reads as an address keeps reading the global too when the
//! stored value is not otherwise one, so the dereference prints through the
//! global the binary reads.

use std::collections::BTreeSet;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::block::DominatesMemo;
use crate::context::{BlockId, OpId, VarnodeId};
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
/// of `op`?
///
/// When `vn` is a register and `invn` a global holding a stored value, `vn` is
/// a load of that global ([`loads_stored_global`]), and stays in `op` when a
/// write lies between the load and `op`.  Otherwise only when `vn` is a global
/// and `invn` a value the function computes (a parameter never merges with a
/// global) that is not just the global read back.  A marker, or a `COPY` into
/// the same global, keeps `vn` when the stored value is used as an address and
/// no earlier value of the global is read after the store.  Any other reader is
/// a load of the global the binary makes.  When the stored value is used as an
/// address, the load keeps `vn` when a write lies between the store and the
/// load, unless the global's own marker carries an earlier value past the store
/// ([`marks_after`]).  When it is not, the load keeps `vn` when it uses what it
/// loads as an address: the value then still joins the global, and the
/// dereference prints through the global the binary reads.
pub fn declines(data: &Funcdata, op: OpId, vn: VarnodeId, invn: VarnodeId) -> bool {
    let (Some(v), Some(iv), Some(reader)) = (
        data.vbank().get(vn),
        data.vbank().get(invn),
        data.obank().get(op),
    ) else {
        return false;
    };
    if !v.is_persist() {
        return v.get_def().is_some_and(|d| loads_stored_global(data, d))
            && written_between(data, vn, op);
    }
    if !computed(iv) || holds_global(data, invn, v) {
        return false;
    }
    let same = |o: Option<VarnodeId>| {
        o.and_then(|o| data.vbank().get(o))
            .is_some_and(|out| out.get_addr() == v.get_addr() && out.get_size() == v.get_size())
    };
    if reader.is_marker() || (reader.code() == OpCode::CPUI_COPY && same(reader.get_out())) {
        used_as_address(data, invn) && !old_value_read_after(data, vn, true)
    } else {
        if !used_as_address(data, invn) {
            return reads_address(data, op, vn);
        }
        written_between(data, vn, op) && !old_value_read_after(data, vn, false)
    }
}

/// How many operations [`written_between`] reads before it answers yes.
const OP_BOUND: usize = 2048;

/// Can an operation on some path from `vn`'s definition to `reader` change the
/// global without an `INDIRECT` that says so: a `STORE` through a pointer, or a
/// call heritage gave no `INDIRECT` on the global?  `vn` is the global's `COPY`
/// output, or a register copy of the global.  A load after such an operation
/// reads memory that operation may have written, so it has to stay a load.  The
/// paths are walked backward from `reader` to the definition; past [`OP_BOUND`]
/// operations, or at a block no path from the definition reaches, the answer is
/// yes.
pub fn written_between(data: &Funcdata, vn: VarnodeId, reader: OpId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return true;
    };
    let Some(def) = v.get_def() else {
        return true;
    };
    let global = if v.is_persist() {
        Some(vn)
    } else {
        data.obank().get(def).and_then(|o| o.get_in(0))
    };
    let Some(g) = global.and_then(|g| data.vbank().get(g)) else {
        return true;
    };
    let (gaddr, gsize) = (g.get_addr(), g.get_size());
    let guarded = |mut cur: Option<OpId>| {
        while let Some(o) = cur.and_then(|c| data.obank().get(c)) {
            if o.code() != OpCode::CPUI_INDIRECT {
                return false;
            }
            if o.get_out()
                .and_then(|out| data.vbank().get(out))
                .is_some_and(|out| out.get_addr() == gaddr && out.get_size() == gsize)
            {
                return true;
            }
            cur = o.basic_neighbours().0;
        }
        false
    };
    let Some(start) = data.obank().get(reader) else {
        return true;
    };
    let Some(block) = start.get_parent() else {
        return true;
    };
    let mut work = vec![(block, start.basic_neighbours().0)];
    let mut scanned = BTreeSet::new();
    let mut budget = OP_BOUND;
    while let Some((bl, mut cur)) = work.pop() {
        let mut reached = false;
        while let Some(o) = cur {
            if o == def {
                reached = true;
                break;
            }
            let Some(op) = data.obank().get(o) else {
                return true;
            };
            let writes = op.code() == OpCode::CPUI_STORE
                || matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND);
            if writes && !guarded(op.basic_neighbours().0) {
                return true;
            }
            budget = match budget.checked_sub(1) {
                Some(b) => b,
                None => return true,
            };
            cur = op.basic_neighbours().0;
        }
        if reached {
            continue;
        }
        let b = data.bblocks_ref().block(bl);
        if b.size_in() == 0 {
            return true;
        }
        for i in 0..b.size_in() {
            let pred = b.get_in(i);
            if scanned.insert(pred) {
                work.push((pred, data.bb_op_tail(pred)));
            }
        }
    }
    false
}

/// Is an earlier value of the global that `vn`'s `COPY` stores to carried past
/// the store by the global's own marker ([`marks_after`]), or, when
/// `by_register`, still read after the store, directly or through a copy into a
/// register?  The global's forced merge then keeps the stored value apart
/// anyway, and keeping the store or a load of it in place would only make
/// chapter 06 copy that earlier value far from where the binary reads it.
fn old_value_read_after(data: &Funcdata, vn: VarnodeId, by_register: bool) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let Some(mut store) = v.get_def().and_then(|d| Store::new(data, d)) else {
        return false;
    };
    for w in data.vbank().iter_loc_size_addr(v.get_size(), v.get_addr()) {
        let Some(wv) = data.vbank().get(w) else {
            continue;
        };
        if w == vn || wv.get_def().is_some_and(|d| d == store.op || store.before(data, d)) {
            continue;
        }
        if wv.descend_iter().any(|r| marks_after(data, &mut store, r, w)) {
            return true;
        }
        if !by_register {
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
            if store.before(data, r) {
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

/// The store [`old_value_read_after`] asks about, with the dominator-tree
/// answers for its block kept across the global's instances.
struct Store {
    op: OpId,
    order: u32,
    block: Option<(BlockId, DominatesMemo)>,
}

impl Store {
    fn new(data: &Funcdata, op: OpId) -> Option<Store> {
        let o = data.obank().get(op)?;
        let block = o.get_parent().map(|b| (b, DominatesMemo::new(b)));
        Some(Store { op, order: o.get_seq_num().get_order(), block })
    }

    /// Does the store come before `op`: earlier in its block, or in a block
    /// that dominates `op`'s?
    fn before(&mut self, data: &Funcdata, op: OpId) -> bool {
        let Some((o, q)) = data.obank().get(op).and_then(|o| Some((o, o.get_parent()?))) else {
            return false;
        };
        match &self.block {
            Some((b, _)) if *b == q => self.order < o.get_seq_num().get_order(),
            Some(_) => self.dominates(data, q),
            None => false,
        }
    }

    /// Does the store's block dominate `bl`?
    fn dominates(&mut self, data: &Funcdata, bl: BlockId) -> bool {
        self.block
            .as_mut()
            .is_some_and(|(_, memo)| data.bblocks_ref().dominates_memo(memo, Some(bl)))
    }
}

/// Does `r`, a marker of the global, carry `w`, a value of the global from
/// before `store`, past `store`: a `MULTIEQUAL` reading it on an edge from a
/// block that `store` dominates, or an `INDIRECT` after `store`?  Heritage never
/// builds that; it is left behind when a pointer `STORE` turns into the
/// global's `COPY` after the global's heritage, and the joins still read the
/// value from before it.
fn marks_after(data: &Funcdata, store: &mut Store, r: OpId, w: VarnodeId) -> bool {
    let Some(rop) = data.obank().get(r) else {
        return false;
    };
    let (Some(rb), Some(sb)) = (rop.get_parent(), store.block.as_ref().map(|(b, _)| *b)) else {
        return false;
    };
    if rop.is_dead() {
        return false;
    }
    match rop.code() {
        OpCode::CPUI_MULTIEQUAL => {
            let b = data.bblocks_ref().block(rb);
            (0..rop.num_input().min(b.size_in()))
                .any(|i| rop.get_in(i) == Some(w) && store.dominates(data, b.get_in(i)))
        }
        OpCode::CPUI_INDIRECT if rop.get_in(0) == Some(w) => {
            if rb == sb {
                store.order < rop.get_seq_num().get_order()
            } else {
                store.dominates(data, rb)
            }
        }
        _ => false,
    }
}

/// Must `RulePushMulti` leave `op`, a `MULTIEQUAL` of a global, in place rather
/// than replace it with a join of the values its stores copy?  It must when one
/// of its inputs is a store [`declines`] keeps for the global's markers: a load
/// of the global after a pointer store would then read the replacement, a
/// frame variable or register that the store does not change.
pub fn keeps_join(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if !o
        .get_out()
        .and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.is_persist())
    {
        return false;
    }
    (0..o.num_input()).filter_map(|i| o.get_in(i)).any(|g| {
        holds_stored_value(data, g)
            && data
                .vbank()
                .get(g)
                .and_then(|gv| gv.get_def())
                .and_then(|d| data.obank().get(d))
                .and_then(|d| d.get_in(0))
                .is_some_and(|x| used_as_address(data, x))
    })
}

/// Is `op` a `COPY` that reads a global holding a stored value
/// ([`holds_stored_value`]) into a register: a load of the global the binary
/// makes after the store, which [`declines`] keeps when a write lies between it
/// and its reader?
pub fn loads_stored_global(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    o.code() == OpCode::CPUI_COPY
        && o.get_out()
            .and_then(|v| data.vbank().get(v))
            .is_some_and(|v| !v.is_persist())
        && o.get_in(0).is_some_and(|g| {
            data.vbank().get(g).is_some_and(|gv| gv.is_persist()) && holds_stored_value(data, g)
        })
}

/// Is `g`, a global's varnode, written by a `COPY` of a value the function
/// computes that is not the global read back: the store this module keeps?
fn holds_stored_value(data: &Funcdata, g: VarnodeId) -> bool {
    let Some(gv) = data.vbank().get(g) else {
        return false;
    };
    let Some(def) = gv.get_def().and_then(|d| data.obank().get(d)) else {
        return false;
    };
    if def.code() != OpCode::CPUI_COPY {
        return false;
    }
    def.get_in(0).is_some_and(|x| {
        data.vbank().get(x).is_some_and(|xv| computed(xv)) && !holds_global(data, x, gv)
    })
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
