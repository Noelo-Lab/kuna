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
//! * [`LEVEL_GLOBAL`] — the LOAD guard, plus the STORE guard on a global: an
//!   `indirect_store` `INDIRECT` on the global at every `STORE` into the
//!   global's own space ([`stores_into`]).  This is the shipped default.
//!   Without it a pointer `STORE` has no effect on a global in kuna's SSA, so a
//!   load of the global after `gi = a; *p = k;` reads the `COPY` of `a` and
//!   prints `a`, and a value read before the store prints as a read of the
//!   global after it.  A read-only range is skipped (a store cannot change it),
//!   and a function past [`GLOBAL_STORE_BUDGET`] guards no further range; the
//!   checks of this level act only on a range heritage guarded
//!   ([`crate::heritage::Heritage::global_store_guarded`]), so the others print
//!   as at [`LEVEL_LOAD`].  `RulePropagateCopy` does not move a stored value
//!   into one of these `INDIRECT`s ([`keeps_store_guard_input`]), so the store
//!   stays where the binary makes it and the value does not join the global,
//!   nor past a `LOAD` that may read the global ([`keeps_store_before_load`]).
//! * [`LEVEL_FULL`] — LOAD plus STORE guards on every range, stack included,
//!   i.e. upstream Ghidra's behavior.  The STORE guard's `INDIRECT` chain on a
//!   frame slot survives into the output as write-backs of values a slot
//!   already held (`ttyperm._8_4_ = (unsigned int)v6;` beside
//!   `v6 = ttyperm._8_8_;`).
//!
//! The guards themselves are upstream `Heritage` methods and live with their
//! siblings in [`crate::p3_dataflow::heritage`]; this module owns the option's
//! parse surface, the level vocabulary the gate reads, and the global arm's
//! store selection.

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
                "Index-alias heritage guards turned on, LOAD side plus STORE side on globals"
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
    let sp = fd
        .get_arch()
        .manage()
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .and_then(|st| st.space.map(|space| (space, st.offset)));
    fd.obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter(|&op| {
            fd.obank().get(op).is_some_and(|o| {
                !o.is_dead()
                    && o.get_in(0)
                        .and_then(|v| store_space(fd, v))
                        .is_some_and(|s| Rc::ptr_eq(&s, spc))
                    && !o.get_in(1).is_some_and(|p| sp.as_ref().is_some_and(|sp| off_stack_pointer(fd, p, sp)))
            })
        })
        .collect()
}

/// Is `ptr` the stack pointer, through `COPY`s and constant offsets?
fn off_stack_pointer(fd: &Funcdata, mut ptr: VarnodeId, sp: &(Rc<AddrSpace>, u64)) -> bool {
    for _ in 0..16 {
        let Some(v) = fd.vbank().get(ptr) else {
            return false;
        };
        if Rc::ptr_eq(v.get_space(), &sp.0) && v.get_offset() == sp.1 {
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
