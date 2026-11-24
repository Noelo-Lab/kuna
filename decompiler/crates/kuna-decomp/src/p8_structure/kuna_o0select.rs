//! (kuna `iteregion auto`) Print a conditional value the way an unoptimized
//! function wrote it.
//!
//! An unoptimizing compiler finishes every statement with a store to the
//! variable it assigns, and evaluates an expression in registers.  The two
//! spellings of a conditional value therefore compile apart:
//!
//! ```c
//! x = c ? a : b;              // each arm loads a register; one store after the join
//! if (c) x = a; else x = b;   // each arm stores to x
//! ```
//!
//! Once the stack is in SSA form both are one merge of two values, which
//! `iteregion` prints as `?:` or not by the shape of the arms.  [`record`]
//! reads the raw block graph at the first heritage pass of a function that
//! [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized: a branch whose
//! arms each define a temporary register that the join merges and reads is a
//! conditional value (whatever else the arms do, as in `c ? i++ : j`), and a
//! branch whose arms each write memory and join nothing is a statement.  The
//! record keys each decided branch by the address of its `CBRANCH`, which
//! every later copy of that branch keeps.  A value that is only returned is
//! left to [`crate::p8_structure::kuna_o0returns`].

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;

/// The most ops an arm of a recorded branch holds.
const MAX_ARM_OPS: usize = 64;
/// How many copies of a joined value are followed to find its use.
const USE_DEPTH: u32 = 8;

/// How the source wrote a recorded branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// The value joins in a register: a `?:`.
    Value,
    /// Each arm stores: an `if`/`else` statement.
    Statement,
}

/// The decided branches of one function, read before simplification.
#[derive(Debug, Clone, Default)]
pub struct Record {
    /// `(CBRANCH address, form)`, sorted by address.
    sites: Vec<(u64, Form)>,
}

impl Record {
    fn new(mut sites: Vec<(u64, Form)>) -> Self {
        sites.sort_unstable_by_key(|s| s.0);
        sites.dedup_by_key(|s| s.0);
        Record { sites }
    }

    fn form(&self, addr: u64) -> Option<Form> {
        self.sites.binary_search_by_key(&addr, |s| s.0).ok().map(|i| self.sites[i].1)
    }
}

/// Read and record the conditional values; once, at the first heritage pass.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0select.is_some() {
        return;
    }
    let arch = fd.get_arch();
    if !arch.iteregion || !arch.iteregion_record {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let sites = (0..fd.bblocks_get_size()).filter_map(|i| classify(fd, fd.bblocks_get_block(i))).collect();
    fd.kuna_o0select = Some(Box::new(Record::new(sites)));
}

fn classify(fd: &Funcdata, c: BlockId) -> Option<(u64, Form)> {
    let g = fd.bblocks_ref();
    let b = g.block(c);
    if b.size_out() != 2 {
        return None;
    }
    let br = fd.bb_op_tail(c).and_then(|t| fd.obank().get(t))?;
    if br.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let at = br.get_addr().get_offset();
    let (t, e) = (b.get_out(0), b.get_out(1));
    if t == e || t == c || e == c {
        return None;
    }
    let (tb, eb) = (g.block(t), g.block(e));
    if tb.size_out() != 1 || eb.size_out() != 1 {
        return None;
    }
    let j = tb.get_out(0);
    if eb.get_out(0) != j || j == c || j == t || j == e {
        return None;
    }
    let (st, se) = (stores(fd, t)?, stores(fd, e)?);
    if tb.size_in() == 1 && eb.size_in() == 1 && joined(fd, t, e, j, !st && !se) {
        return Some((at, Form::Value));
    }
    (st && se).then_some((at, Form::Statement))
}

/// Whether `j` merges a temporary that arms `t` and `e` each define and that
/// is read after the join.  A call reads it directly only when the arms set
/// it and do nothing else (`quiet`): an argument evaluated in place; after a
/// statement in each arm the register holds a leftover that an unprototyped
/// call reads as well.
fn joined(fd: &Funcdata, t: BlockId, e: BlockId, j: BlockId, quiet: bool) -> bool {
    let (Some(st), Some(se)) = (slot(fd, j, t), slot(fd, j, e)) else { return false };
    fd.bb_ops(j).into_iter().any(|o| {
        let Some(op) = fd.obank().get(o) else { return false };
        if op.code() != OpCode::CPUI_MULTIEQUAL || !op.get_out().is_some_and(|v| temporary(fd, v)) {
            return false;
        }
        let def = |s: usize, x: BlockId| {
            op.get_in(s as i32)
                .and_then(|v| fd.vbank().get(v))
                .and_then(|v| v.get_def())
                .and_then(|d| fd.obank().get(d))
                .filter(|d| d.get_parent() == Some(x))
                .map(|d| d.code() != OpCode::CPUI_INDIRECT)
        };
        let (Some(dt), Some(de)) = (def(st, t), def(se, e)) else { return false };
        op.get_out().is_some_and(|v| used(fd, v, USE_DEPTH, quiet && dt && de))
    })
}

/// Whether register `vn` holds an expression's value rather than a variable.
/// An unoptimizing compiler evaluates in registers a call does not preserve,
/// and keeps a value in a preserved one only to carry it across a call to
/// its one use, a copy into an argument register; a `register` variable lives
/// in a preserved register and is read wherever the source names it.
fn temporary(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if memory(fd, vn) {
        return false;
    }
    if fd.get_func_proto().has_effect(&v.get_addr(), v.get_size()) != crate::fspec::effect_type::UNAFFECTED {
        return true;
    }
    let mut reads = v.descend_iter().filter_map(|d| fd.obank().get(d)).filter(|d| {
        !matches!(d.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_RETURN)
    });
    matches!((reads.next(), reads.next()), (Some(r), None) if r.code() == OpCode::CPUI_COPY)
}

/// The in-edge index of `j` that comes from `x`.
fn slot(fd: &Funcdata, j: BlockId, x: BlockId) -> Option<usize> {
    let jb = fd.bblocks_ref().block(j);
    (0..jb.size_in()).find(|&k| jb.get_in(k) == x).map(|k| k as usize)
}

/// Whether arm `x` writes memory; `None` for a block that is no arm.  The
/// return address a call instruction pushes is not a store of the arm's.
fn stores(fd: &Funcdata, x: BlockId) -> Option<bool> {
    let ops = fd.bb_ops(x);
    if ops.len() > MAX_ARM_OPS {
        return None;
    }
    let calls: Vec<u64> = ops
        .iter()
        .filter_map(|&o| fd.obank().get(o))
        .filter(|op| op.is_call())
        .map(|op| op.get_addr().get_offset())
        .collect();
    let mut writes = false;
    for o in ops {
        let op = fd.obank().get(o)?;
        writes |= match op.code() {
            OpCode::CPUI_STORE => !calls.contains(&op.get_addr().get_offset()),
            OpCode::CPUI_RETURN | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CBRANCH => return None,
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_MULTIEQUAL => false,
            _ => op.get_out().is_some_and(|v| memory(fd, v)),
        };
    }
    Some(writes)
}

/// Whether `vn` is read by something other than a merge or a return, looking
/// through copies into another register.  A call reads it only when the arms
/// set it or a copy moved it into an argument register (`copied`): an
/// unprototyped call also reads whatever an earlier call left in a scratch
/// register.
fn used(fd: &Funcdata, vn: VarnodeId, depth: u32, copied: bool) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    v.descend_iter().any(|d| {
        let Some(op) = fd.obank().get(d) else { return false };
        let next = |copied: bool| {
            op.get_out().is_some_and(|o| memory(fd, o) || (depth > 0 && used(fd, o, depth - 1, copied)))
        };
        let low = |slot: i32| op.get_in(slot).and_then(|c| fd.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0);
        match op.code() {
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_RETURN => false,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => copied,
            OpCode::CPUI_COPY => next(true),
            OpCode::CPUI_SUBPIECE => low(1) && next(copied),
            OpCode::CPUI_PIECE => op.get_in(1) == Some(vn) && next(copied),
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => next(copied),
            _ => true,
        }
    })
}

/// Whether `vn` is memory: a variable in the frame or the data space rather
/// than a register or a temporary.
fn memory(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    let s = v.get_space();
    match s.get_type() {
        spacetype::IPTR_INTERNAL | spacetype::IPTR_CONSTANT => false,
        spacetype::IPTR_PROCESSOR => s.get_name() != "register",
        _ => true,
    }
}

/// How the source wrote the branch whose `CBRANCH` sits at `addr`, if recorded.
pub(crate) fn form(fd: &Funcdata, addr: u64) -> Option<Form> {
    if !fd.get_arch().iteregion_record {
        return None;
    }
    let rec = fd.kuna_o0select.as_deref()?;
    rec.form(addr)
}

/// How the source wrote the branch of `CBRANCH` op `op`, if recorded.
pub(crate) fn form_of(fd: &Funcdata, op: crate::context::OpId) -> Option<Form> {
    form(fd, fd.obank().get(op)?.get_addr().get_offset())
}

/// Whether the branch of `CBRANCH` op `op` is a recorded statement.
pub(crate) fn statement(fd: &Funcdata, op: crate::context::OpId) -> bool {
    form_of(fd, op) == Some(Form::Statement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_answers_by_branch_address() {
        let rec = Record::new(vec![(0x40, Form::Statement), (0x10, Form::Value), (0x40, Form::Statement)]);
        assert_eq!(rec.form(0x10), Some(Form::Value));
        assert_eq!(rec.form(0x40), Some(Form::Statement));
        assert_eq!(rec.form(0x20), None);
        assert_eq!(rec.sites.len(), 2);
    }

    #[test]
    fn iteregion_auto_keeps_folding_on_and_adds_the_record() {
        let opt = crate::p8_structure::kuna_iteregion::OptionIteRegion;
        assert!(matches!(opt.apply("auto"), Ok((true, true, _))));
        assert!(matches!(opt.apply("on"), Ok((true, false, _))));
        assert!(matches!(opt.apply("off"), Ok((false, false, _))));
        assert!(opt.apply("sometimes").is_err());
    }
}
