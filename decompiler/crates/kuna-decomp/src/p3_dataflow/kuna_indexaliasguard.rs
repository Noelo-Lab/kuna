//! P3 index-alias heritage guards — the `indexaliasguard` decision point.
//!
//! # The gap
//!
//! [`Heritage::guard`](crate::p3_dataflow::heritage::Heritage) ends with an arm
//! upstream Ghidra runs whenever a pointer can reach the range being heritaged
//! (`heritage.cc:1194`, gated on `Architecture::highPtrPossible`):
//! `Heritage::guardStores` (`heritage.cc:1538`) prepopulates data-flow across
//! every STORE that could alias the range, and `Heritage::guardLoads`
//! (`heritage.cc:1570`) puts an `addrforce` `CPUI_COPY` read of the range in
//! front of each indexed-stack LOAD whose discovered guard range covers it.
//! kuna shipped that arm behind a hard-coded `highPtrPossible == false`, so
//! neither ran.
//!
//! Without the LOAD guard a frame slot written by a direct `MOV [ESP+k],REG` and
//! read only through a `LEA`-derived pointer has no reader at all, so
//! `ActionDeadCode` deletes the store: the emitted C declares a stack array,
//! walks it with a pointer loop, and never initializes it.
//!
//! # The levels
//!
//! The halves have very different footprints, so the option is a level rather
//! than an on/off:
//!
//! * [`LEVEL_OFF`] — neither guard; byte-identical to what kuna shipped before
//!   this option.
//! * [`LEVEL_LOAD`] — the LOAD guard only.  It is what recovers the dropped
//!   initializing stores, and it moved 0/675 datatest and 0/714 stage
//!   assertions.
//! * [`LEVEL_GLOBAL`] — the LOAD guard, plus two guards on a global.  This is
//!   the shipped default.
//!
//!   The STORE guard puts an `indirect_store` `INDIRECT` on the global at every
//!   `STORE` into the global's own space ([`stores_into`]).  Without it a
//!   pointer `STORE` has no effect on a global in kuna's SSA, so a load of the
//!   global after `gi = a; *p = k;` reads the `COPY` of `a` and prints `a`, and
//!   a value read before the store prints as a read of the global after it.  A
//!   read-only range is skipped (a store cannot change it), and a function past
//!   [`GLOBAL_STORE_BUDGET`] guards no further range; the checks of this guard
//!   act only on a range heritage guarded
//!   ([`crate::heritage::Heritage::global_store_guarded`]).  `RulePropagateCopy`
//!   does not move a stored value into one of these `INDIRECT`s
//!   ([`keeps_store_guard_input`]), so the store stays where the binary makes
//!   it and the value does not join the global, nor past a `LOAD` that may
//!   read the global ([`keeps_store_before_load`]).
//!
//!   The LOAD guard puts an `addrforce` `COPY` read of the global in front of
//!   every `LOAD` from the global's space that may read it ([`loads_from`]).
//!   Without it a pointer `LOAD` reads no global in kuna's SSA, so
//!   `gi = a; x = *p; gi = b;` leaves the first store with no reader and
//!   `ActionDeadCode` deletes it.  Only a writable range with at least two
//!   writes other than `INDIRECT`s, none of them a smaller piece of the range,
//!   is guarded (a single write reaches the return guard), and a heritage pass
//!   stops guarding new ranges at [`GLOBAL_LOAD_BUDGET`] `COPY`s.
//!   [`keeps_forced_self_copy`] and [`keeps_forced_join`] keep the rules from
//!   printing such a kept store twice, and [`load_crosses_global_store`] keeps
//!   a `LOAD` explicit when its value is live across a store to a global, so
//!   it prints ahead of the store as the binary makes it.
//! * [`LEVEL_FULL`] — LOAD plus STORE guards on every range, stack included,
//!   i.e. upstream Ghidra's behavior, plus the global LOAD guard.  The STORE
//!   guard's `INDIRECT` chain on a frame slot survives into the output as
//!   write-backs of values a slot already held
//!   (`ttyperm._8_4_ = (unsigned int)v6;` beside `v6 = ttyperm._8_8_;`).
//!
//! The guards themselves are upstream `Heritage` methods and live with their
//! siblings in [`crate::p3_dataflow::heritage`]; this module owns the option's
//! parse surface, the level vocabulary the gate reads, and the global arm's
//! store and load selection.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// `option indexaliasguard off` — neither index-alias guard runs.
pub const LEVEL_OFF: int4 = 0;

/// `option indexaliasguard load` — `Heritage::guardLoads` only.
pub const LEVEL_LOAD: int4 = 1;

/// `option indexaliasguard global` — `guardLoads` + the STORE guard on a
/// global range.
pub const LEVEL_GLOBAL: int4 = 2;

/// `option indexaliasguard full` — `guardLoads` + `guardStores`, i.e. upstream
/// Ghidra's behavior.
pub const LEVEL_FULL: int4 = 3;

/// How many global STORE guards one function's heritage builds before it stops
/// guarding further ranges.  Every later pass visits each guard: a run of data
/// decoded as code, with hundreds of stores and of addresses it "reads",
/// reaches 100k guards and triples a whole firmware decompile, and at 2048 the
/// guards still cost that firmware 6% of its time.
pub const GLOBAL_STORE_BUDGET: usize = 1024;

/// How many global LOAD guard `COPY`s one heritage pass builds before it stops
/// guarding further ranges.
pub const GLOBAL_LOAD_BUDGET: usize = 4096;

/// `option indexaliasguard off|load|global|full` (C++ has no equivalent —
/// upstream is unconditionally `full`).
pub struct OptionIndexAliasGuard;

impl OptionIndexAliasGuard {
    /// The option name.
    pub const NAME: &'static str = "indexaliasguard";

    /// Parse `off`/`load`/`global`/`full` into the guard level + a confirmation
    /// message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((LEVEL_OFF, "Index-alias heritage guards turned off".to_string())),
            "load" => Ok((
                LEVEL_LOAD,
                "Index-alias heritage guards turned on, LOAD side only (guardLoads)".to_string(),
            )),
            // Empty parameter reads as the shipped default, matching `options::on_or_off`.
            "global" | "on" | "" => Ok((
                LEVEL_GLOBAL,
                "Index-alias heritage guards turned on, LOAD side plus STORE and LOAD guards on globals"
                    .to_string(),
            )),
            "full" => Ok((
                LEVEL_FULL,
                "Index-alias heritage guards turned on, LOAD + STORE \
                 (guardLoads + guardStores)"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of off, load, global, full",
            )),
        }
    }
}

/// The live `STORE`s whose address space is `spc` and whose address is not the
/// stack pointer plus a constant: the ops that can write a global in `spc`.
pub fn stores_into(fd: &Funcdata, spc: &Rc<AddrSpace>, store_space: impl Fn(&Funcdata, VarnodeId) -> Option<Rc<AddrSpace>>) -> Vec<OpId> {
    let sp: Vec<(Rc<AddrSpace>, u64)> = fd
        .get_arch()
        .manage()
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .and_then(|st| st.space.map(|space| (space, st.offset)))
        .into_iter()
        .collect();
    fd.obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter(|&op| {
            fd.obank().get(op).is_some_and(|o| {
                !o.is_dead()
                    && o.get_in(0)
                        .and_then(|v| store_space(fd, v))
                        .is_some_and(|s| Rc::ptr_eq(&s, spc))
                    && !o.get_in(1).is_some_and(|p| based_on(fd, p, &sp))
            })
        })
        .collect()
}

/// The live `LOAD`s from `spc` that may read a global there, each with the
/// bytes it reads when its address is a constant.  A `LOAD` through the stack
/// pointer plus a constant reads the frame, and one through an x86 segment base
/// (`FS_OFFSET`, `GS_OFFSET`, how the stack canary is read) reads thread
/// storage, not a global.
pub fn loads_from(
    fd: &Funcdata,
    spc: &Rc<AddrSpace>,
    load_space: impl Fn(&Funcdata, VarnodeId) -> Option<Rc<AddrSpace>>,
) -> Vec<(OpId, Option<(u64, u64)>)> {
    let manage = fd.get_arch().manage();
    let mut bases: Vec<(Rc<AddrSpace>, u64)> = Vec::new();
    if let Some(st) = manage.get_stack_space().and_then(|s| s.get_spacebase(0).ok()) {
        if let Some(space) = st.space {
            bases.push((space, st.offset));
        }
    }
    if let Some(lookup) = manage.register_lookup() {
        for name in ["FS_OFFSET", "GS_OFFSET"] {
            if let Some(st) = lookup.probe_register(name) {
                if let Some(space) = st.space.clone() {
                    bases.push((space, st.offset));
                }
            }
        }
    }
    let mut loads = Vec::new();
    for op in fd.obank().iter_code(OpCode::CPUI_LOAD) {
        let Some(o) = fd.obank().get(op) else {
            continue;
        };
        if o.is_dead() || o.uses_spacebase_ptr() {
            continue;
        }
        if !o.get_in(0).and_then(|v| load_space(fd, v)).is_some_and(|s| Rc::ptr_eq(&s, spc)) {
            continue;
        }
        let Some(ptr) = o.get_in(1) else {
            continue;
        };
        if based_on(fd, ptr, &bases) {
            continue;
        }
        let bytes = fd.vbank().get(ptr).filter(|p| p.is_constant()).map(|p| {
            let size = o.get_out().and_then(|w| fd.vbank().get(w)).map_or(1, |w| w.get_size() as u64);
            (p.get_offset(), p.get_offset().wrapping_add(size))
        });
        loads.push((op, bytes));
    }
    loads
}

/// Must `RulePropagateCopy` leave `vn`, a global's store that a global LOAD
/// guard forced, as the input of `op`, a `COPY` of it into the global's own
/// storage (a `MULTIEQUAL` that splitting a shared block turned into a `COPY`)?
/// The store is kept anyway, so taking its value would print the store twice.
pub fn keeps_forced_self_copy(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    if data.get_arch().index_alias_guard < LEVEL_GLOBAL {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if o.code() != OpCode::CPUI_COPY {
        return false;
    }
    let (Some(v), Some(w)) = (data.vbank().get(vn), o.get_out().and_then(|w| data.vbank().get(w))) else {
        return false;
    };
    v.is_addr_force() && v.is_persist() && w.get_addr() == v.get_addr() && w.get_size() == v.get_size()
}

/// Must `RuleMultiCollapse` keep `out`, a `MULTIEQUAL` on a global, from
/// matching its inputs `a` and `b` by functional equality when one of them is
/// a store that a global LOAD guard forced?  The forced store is kept anyway,
/// so the collapse would print the store again where the paths meet.
pub fn keeps_forced_join(data: &Funcdata, out: VarnodeId, a: VarnodeId, b: VarnodeId) -> bool {
    let forced = |v: VarnodeId| data.vbank().get(v).is_some_and(|d| d.is_addr_force() && d.is_written());
    data.get_arch().index_alias_guard >= LEVEL_GLOBAL
        && data.vbank().get(out).is_some_and(|o| o.is_persist())
        && (forced(a) || forced(b))
}

/// Is `vn`, a `LOAD`'s output whose Cover is current, live across a store to a
/// global in the space the `LOAD` reads?  Such a load printed at its reader,
/// past the store, would read the stored value.  The global LOAD guard keeps
/// stores where the binary makes them, also ones a `LOAD` used to be merged
/// into and printed with ahead of the other stores (`x = *p; gm = 0; gk = x;`
/// printed `gk = *a0; gm = 0;` and now prints `gm = 0;` first).  A store is a
/// live, non-marker op writing a persistent varnode in that space, or a
/// varnode merged into a global's HighVariable there, other than a return
/// copy, a `COPY` of the global into its own storage, or a `COPY` into a
/// temporary of its input's HighVariable.  When the `LOAD`'s address is a
/// constant, only a global overlapping the loaded bytes counts.
pub fn load_crosses_global_store(data: &Funcdata, vn: VarnodeId) -> bool {
    if data.get_arch().index_alias_guard < LEVEL_GLOBAL {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let Some(load) = v.get_def().and_then(|d| data.obank().get(d)) else {
        return false;
    };
    if load.code() != OpCode::CPUI_LOAD {
        return false;
    }
    let Some(cover) = v.cover() else {
        return false;
    };
    let Some(index) = load.get_in(0).and_then(|s| data.vbank().get(s)).map(|s| s.get_offset()) else {
        return false;
    };
    let window = load
        .get_in(1)
        .and_then(|p| data.vbank().get(p))
        .filter(|p| p.is_constant())
        .map(|p| (p.get_offset(), p.get_offset().wrapping_add(v.get_size() as u64)));
    let global = |w: VarnodeId| {
        data.vbank().get(w).is_some_and(|w| {
            w.is_persist()
                && w.get_space().get_index() as u64 == index
                && window.is_none_or(|(a, b)| w.get_offset() < b && a < w.get_offset().wrapping_add(w.get_size() as u64))
        })
    };
    for (blk, _) in cover.iter() {
        if blk < 0 || blk >= data.bblocks_get_size() {
            continue;
        }
        for op in data.bb_ops(data.bblocks_get_block(blk)) {
            let Some(o) = data.obank().get(op) else {
                continue;
            };
            if o.is_dead() || o.is_marker() || o.is_return_copy() {
                continue;
            }
            let Some(wid) = o.get_out() else {
                continue;
            };
            let Some(w) = data.vbank().get(wid) else {
                continue;
            };
            if o.code() == OpCode::CPUI_COPY
                && o.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| {
                    (i.get_addr() == w.get_addr() && i.get_size() == w.get_size())
                        || (!w.is_persist() && i.get_high().is_some() && i.get_high() == w.get_high())
                })
            {
                continue;
            }
            let writes = global(wid)
                || w.get_high().and_then(|h| data.high_bank().get(h)).is_some_and(|h| {
                    (0..h.num_instances()).any(|i| global(h.get_instance(i)))
                });
            if writes && cover.contain(blk, data.op_cover_point_pub(op), 2) {
                return true;
            }
        }
    }
    false
}

/// Is `ptr` one of the registers `bases`, through `COPY`s and constant offsets?
fn based_on(fd: &Funcdata, mut ptr: VarnodeId, bases: &[(Rc<AddrSpace>, u64)]) -> bool {
    for _ in 0..16 {
        let Some(v) = fd.vbank().get(ptr) else {
            return false;
        };
        if bases.iter().any(|(space, off)| Rc::ptr_eq(v.get_space(), space) && v.get_offset() == *off) {
            return true;
        }
        let Some(def) = v.get_def().and_then(|d| fd.obank().get(d)) else {
            return false;
        };
        let constant = |i: int4| def.get_in(i).and_then(|c| fd.vbank().get(c)).is_some_and(|c| c.is_constant());
        let next = match def.code() {
            OpCode::CPUI_COPY => def.get_in(0),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB if constant(1) => def.get_in(0),
            OpCode::CPUI_PTRADD if constant(1) && constant(2) => def.get_in(0),
            _ => None,
        };
        let Some(n) = next else {
            return false;
        };
        ptr = n;
    }
    false
}

/// Must `RulePropagateCopy` leave `vn`, a global's stored value, as the input
/// of `op`, the `INDIRECT` a pointer `STORE` puts on that global?  Taking the
/// value would kill the global's `COPY`, and `Merge` would then rebuild the
/// store in front of the pointer `STORE`'s neighbours or join the value with
/// the global, whose type it would take.
pub fn keeps_store_guard_input(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    if data.get_arch().index_alias_guard != LEVEL_GLOBAL {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    o.code() == OpCode::CPUI_INDIRECT
        && o.is_indirect_store()
        && data.vbank().get(vn).is_some_and(|v| v.is_persist())
        && o.get_out().and_then(|out| data.vbank().get(out)).is_some_and(|out| out.is_persist())
}

/// Must `RulePropagateCopy` leave `vn`, a guarded global's stored value, as the
/// input of `op` because a `LOAD` that may read the global follows the global's
/// `COPY` before the next write of the global (a `MULTIEQUAL` on the global
/// only joins what reaches it, so the path continues through one)?  `op` is a
/// marker, or a `COPY` of the global into its own storage (a `MULTIEQUAL` that
/// splitting a shared return block turned into a `COPY`).  Taking the value
/// would kill the `COPY`, and the store would then print where the value is
/// computed, at the end of the block the marker reads it from, or at the
/// moved `COPY`, and that can be on the wrong side of the `LOAD`.  A `LOAD` may
/// read the global when it loads from the global's space and, at a constant
/// address, overlaps it.
pub fn keeps_store_before_load(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    if data.get_arch().index_alias_guard != LEVEL_GLOBAL {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    let self_copy = o.code() == OpCode::CPUI_COPY
        && o.get_out().and_then(|w| data.vbank().get(w)).zip(data.vbank().get(vn)).is_some_and(|(w, v)| {
            w.is_persist() && w.get_addr() == v.get_addr() && w.get_size() == v.get_size()
        });
    if !o.is_marker() && !self_copy {
        return false;
    }
    if data.obank().iter_code(OpCode::CPUI_LOAD).next().is_none() {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if !v.is_persist() || !data.global_store_guarded(v) {
        return false;
    }
    let Some(copy) = v.get_def() else {
        return false;
    };
    let Some(start) = data.obank().get(copy).and_then(|o| o.get_parent()) else {
        return false;
    };
    let (index, lo, hi) = (v.get_space().get_index() as u64, v.get_offset(), v.get_offset().wrapping_add(v.get_size() as u64));
    // Some(true): a LOAD that may read the global; Some(false): a write of it.
    let effect = |op: OpId| -> Option<bool> {
        let o = data.obank().get(op)?;
        if o.is_dead() {
            return None;
        }
        if o.code() == OpCode::CPUI_LOAD
            && o.get_in(0).and_then(|s| data.vbank().get(s)).is_some_and(|s| s.get_offset() == index)
        {
            let size = o.get_out().and_then(|w| data.vbank().get(w)).map_or(1, |w| w.get_size() as u64);
            let reads = match o.get_in(1).and_then(|p| data.vbank().get(p)).filter(|p| p.is_constant()) {
                Some(p) => p.get_offset() < hi && lo < p.get_offset().wrapping_add(size),
                None => true,
            };
            if reads {
                return Some(true);
            }
        }
        if o.code() == OpCode::CPUI_MULTIEQUAL {
            return None;
        }
        let w = data.vbank().get(o.get_out()?)?;
        let overlaps = w.get_space().get_index() as u64 == index
            && w.get_offset() < hi
            && lo < w.get_offset().wrapping_add(w.get_size() as u64);
        (w.is_persist() && overlaps).then_some(false)
    };
    let scan = |mut from: Option<OpId>| -> Option<bool> {
        while let Some(op) = from {
            if let Some(e) = effect(op) {
                return Some(e);
            }
            from = data.bb_op_next(op);
        }
        None
    };
    let push = |blk, work: &mut Vec<_>| {
        let b = data.bblocks_ref().block(blk);
        work.extend((0..b.size_out()).map(|i| b.get_out(i)));
    };
    let mut work = Vec::new();
    match scan(data.bb_op_next(copy)) {
        Some(true) => return true,
        Some(false) => {}
        None => push(start, &mut work),
    }
    let mut seen = std::collections::HashSet::new();
    while let Some(blk) = work.pop() {
        if !seen.insert(blk) {
            continue;
        }
        match scan(data.bb_op_head(blk)) {
            Some(true) => return true,
            Some(false) => {}
            None => push(blk, &mut work),
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_is_registered() {
        assert!(crate::options::KUNA_OPTION_NAMES.contains(&OptionIndexAliasGuard::NAME));
    }

    #[test]
    fn levels_parse_and_reject() {
        let o = OptionIndexAliasGuard;
        assert_eq!(o.apply("off").unwrap().0, LEVEL_OFF);
        assert_eq!(o.apply("load").unwrap().0, LEVEL_LOAD);
        assert_eq!(o.apply("global").unwrap().0, LEVEL_GLOBAL);
        assert_eq!(o.apply("on").unwrap().0, LEVEL_GLOBAL);
        assert_eq!(o.apply("").unwrap().0, LEVEL_GLOBAL);
        assert_eq!(o.apply("full").unwrap().0, LEVEL_FULL);
        assert!(o.apply("sometimes").is_err());
    }
}
