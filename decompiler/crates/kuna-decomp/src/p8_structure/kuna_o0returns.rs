//! (kuna `returndup auto`) Print the `return` statements an unoptimized
//! function was written with.
//!
//! An unoptimizing compiler gives a function one epilogue and compiles every
//! `return` into code of its own: the value is put in the return register and
//! control jumps to the epilogue, or falls into it.  Paths that leave through
//! the same `return` share that code and paths through different ones do not,
//! so the raw block graph tells the source forms apart:
//!
//! ```c
//! if (c) { S; } return K;           // the epilogue block sets K itself
//! if (!c) return K; S; return K;    // each predecessor sets K, then jumps
//! ```
//!
//! Simplification empties the value-setting blocks and removes them, after
//! which both forms are one RETURN block with two predecessors and
//! `returndup`, `earlyreturn`, `rettriangle` and `tailform` guess the form.
//! [`record`] reads the raw graph at the first heritage pass of a function
//! that [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized and tags
//! each edge into the epilogue, and each edge into a predecessor that is only
//! a return statement, with the statement it belongs to.  A removed block's
//! in-edges keep their labels when they are re-pointed at its successor, so
//! when `returndup` runs the in-edges of the RETURN block still say which of
//! them share a statement.  [`split`] gives every statement its own copy of
//! the epilogue and no more, and the passes that place returns by rule leave
//! such a block, or such a function, alone.
//!
//! One statement is:
//! * a predecessor that only computes a value (no store, no call) on its way
//!   to the epilogue, whatever reaches it;
//! * in a function that returns a value, the epilogue itself when it computes
//!   the returned register (`mov eax, K` before `leave`); otherwise each
//!   predecessor ends in a `return` of its own;
//! * in a void function, where `return;` is a `nop` and a jump, a block that
//!   only jumps and is entered on its `nop`; otherwise the instruction a path
//!   lands on, paths that land on the same address sharing it.
//!
//! The compiler emits `return c ? a : b;` exactly like `if (c) return a;
//! return b;`, and this prints the second.

use std::collections::HashMap;

use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Edge-label bits that carry a statement tag (1-based index into
/// [`Record::tags`]); the ported edge flags use the low bits.
const TAG_SHIFT: u32 = 16;
const TAG_MASK: uint4 = 0x7fff << TAG_SHIFT;
/// The most epilogue in-edges a function is recorded for.
const MAX_EDGES: int4 = 64;
/// The most ops a return-statement block holds.
const MAX_STATEMENT_OPS: usize = 32;

/// The return statements of one function, read before simplification.
#[derive(Debug, Clone, Default)]
pub struct Record {
    /// Registers the epilogue block computes itself and passes on, as (space
    /// index, offset, size).
    held: Vec<(i32, u64, int4)>,
    /// Per tag: the statement it belongs to if the function returns a value,
    /// and if it returns void.
    tags: Vec<(u16, u16)>,
    /// [`split`] left every RETURN block one recorded statement.
    decided: bool,
}

/// Read and tag the return statements; once, at the first heritage pass.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0returns.is_some() {
        return;
    }
    let arch = fd.get_arch();
    if !arch.returndup_record || !arch.duplicate_shared_returns {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let Some(j) = epilogue(fd) else { return };
    let n = fd.bblocks_ref().block(j).size_in();
    if !(2..=MAX_EDGES).contains(&n) {
        return;
    }
    let preds: Vec<BlockId> = (0..n).map(|i| fd.bblocks_ref().block(j).get_in(i)).collect();
    let j_start = fd.bb_op_head(j).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
    let entries: Vec<Option<Entry>> = preds.iter().map(|&x| entry(fd, x, j)).collect();
    let mut lands = landings(&entries, j_start);
    chain_nops(fd, &mut lands, j_start);
    let mut valued: Vec<Key> = Vec::new();
    let mut void: Vec<Key> = Vec::new();
    let mut pure: Vec<bool> = Vec::new();
    for (i, &x) in preds.iter().enumerate() {
        let i = i as int4;
        let stmt = statement_only(fd, x);
        valued.push(if stmt { Key::Block(x) } else { Key::Edge(i) });
        void.push(if stmt && behind_nop(fd, x) {
            Key::Block(x)
        } else {
            lands[i as usize].map_or(Key::Edge(i), Key::Landing)
        });
        pure.push(stmt);
    }
    let (gv, gd) = (numbered(&valued), numbered(&void));
    let rec = Record { held: held(fd, j), tags: gv.into_iter().zip(gd).collect(), decided: false };
    for i in 0..n {
        let lab = ((i + 1) as uint4) << TAG_SHIFT;
        let x = fd.bblocks_ref().block(j).get_in(i);
        let slot = fd.bblocks_ref().block(j).get_in_rev_index(i);
        fd.bblocks_mut().set_out_edge_flag(x, slot, lab);
        if pure[i as usize] {
            for k in 0..fd.bblocks_ref().block(x).size_in() {
                let p = fd.bblocks_ref().block(x).get_in(k);
                let ps = fd.bblocks_ref().block(x).get_in_rev_index(k);
                fd.bblocks_mut().clear_out_edge_flag(p, ps, TAG_MASK);
                fd.bblocks_mut().set_out_edge_flag(p, ps, lab);
            }
        }
    }
    fd.kuna_o0returns = Some(Box::new(rec));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Block(BlockId),
    Edge(int4),
    Landing(u64),
}

fn numbered(keys: &[Key]) -> Vec<u16> {
    let mut ids: HashMap<Key, u16> = HashMap::new();
    keys.iter()
        .map(|k| {
            let next = ids.len() as u16;
            *ids.entry(*k).or_insert(next)
        })
        .collect()
}

/// The block every return goes through: the one holding the function's only
/// RETURN, or the stack-protector check in front of it.
fn epilogue(fd: &Funcdata) -> Option<BlockId> {
    let mut rets = fd.obank().iter_code(OpCode::CPUI_RETURN).filter(|&o| {
        fd.obank().get(o).is_some_and(|op| !op.is_dead() && op.get_halt_type() == 0)
    });
    let r = rets.next()?;
    if rets.next().is_some() {
        return None;
    }
    let mut j = fd.obank().get(r)?.get_parent()?;
    for _ in 0..4 {
        if fd.bblocks_ref().block(j).size_in() != 1 {
            break;
        }
        let p = fd.bblocks_ref().block(j).get_in(0);
        if !guard_check(fd, p, j) {
            break;
        }
        j = p;
    }
    Some(j)
}

/// Whether `p` branches either to `j` or to a dead end that makes a call (a
/// stack-protector check).
fn guard_check(fd: &Funcdata, p: BlockId, j: BlockId) -> bool {
    let b = fd.bblocks_ref().block(p);
    if b.size_out() != 2 || tail_code(fd, p) != Some(OpCode::CPUI_CBRANCH) {
        return false;
    }
    let other = if b.get_out(0) == j { b.get_out(1) } else { b.get_out(0) };
    let o = fd.bblocks_ref().block(other);
    other != j
        && o.size_out() == 0
        && o.size_in() == 1
        && fd.bb_ops(other).iter().any(|&op| fd.obank().get(op).is_some_and(|op| op.is_call()))
}

fn tail_code(fd: &Funcdata, b: BlockId) -> Option<OpCode> {
    fd.bb_op_tail(b).and_then(|t| fd.obank().get(t)).map(|o| o.code())
}

/// A block that only computes a value on its way to its one successor: no
/// store, call or decision.
fn statement_only(fd: &Funcdata, x: BlockId) -> bool {
    let b = fd.bblocks_ref().block(x);
    if b.size_out() != 1 || b.size_in() == 0 || b.is_entry_point() {
        return false;
    }
    let ops = fd.bb_ops(x);
    ops.len() <= MAX_STATEMENT_OPS
        && ops.iter().all(|&o| {
            fd.obank().get(o).is_some_and(|op| {
                !matches!(
                    op.code(),
                    OpCode::CPUI_STORE
                        | OpCode::CPUI_CALL
                        | OpCode::CPUI_CALLIND
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_CBRANCH
                        | OpCode::CPUI_BRANCHIND
                        | OpCode::CPUI_RETURN
                )
            })
        })
}

/// Whether a jump into `x` lands on an instruction without p-code in front
/// of it: the `nop` a `return;` compiles to, where a `break` or `goto` is a
/// bare jump.
fn behind_nop(fd: &Funcdata, x: BlockId) -> bool {
    let Some(start) = fd.bb_op_head(x).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset()) else {
        return false;
    };
    let b = fd.bblocks_ref().block(x);
    (0..b.size_in()).any(|k| matches!(entry(fd, b.get_in(k), x), Some(Entry::Jump(d)) if d < start))
}

/// How `x` passes to `j`: by a jump to an address, or by falling into the
/// instructions after its last one.
#[derive(Debug, Clone, Copy)]
enum Entry {
    Jump(u64),
    Fall(u64),
}

fn entry(fd: &Funcdata, x: BlockId, j: BlockId) -> Option<Entry> {
    let tail = fd.bb_op_tail(x)?;
    let op = fd.obank().get(tail)?;
    let target = || {
        let v = fd.vbank().get(op.get_in(0)?)?;
        (v.get_space().get_type() != spacetype::IPTR_CONSTANT).then(|| Entry::Jump(v.get_offset()))
    };
    let fall = Entry::Fall(op.get_addr().get_offset());
    match op.code() {
        OpCode::CPUI_BRANCH => target(),
        OpCode::CPUI_CBRANCH => {
            let b = fd.bblocks_ref().block(x);
            match (b.get_out(0) == j, b.get_out(1) == j) {
                (true, false) => Some(fall),
                (false, true) => target(),
                _ => None,
            }
        }
        _ => Some(fall),
    }
}

/// The address each entry lands on.  A fall-through runs every instruction
/// between its block and `j` that has no p-code (a `nop`), so it lands on
/// the first jump target in that gap, or on `j` itself.
fn landings(entries: &[Option<Entry>], j_start: u64) -> Vec<Option<u64>> {
    let jumps: Vec<u64> = entries
        .iter()
        .filter_map(|e| match e {
            Some(Entry::Jump(d)) => Some(*d),
            _ => None,
        })
        .collect();
    entries
        .iter()
        .map(|e| match *e {
            Some(Entry::Jump(d)) => Some(d),
            Some(Entry::Fall(last)) => {
                Some(jumps.iter().copied().filter(|&d| d > last && d <= j_start).min().unwrap_or(j_start))
            }
            None => None,
        })
        .collect()
}

/// A landing on a `nop` that only runs into a later landing's `nop` (no
/// p-code between them) lands there too.
fn chain_nops(fd: &Funcdata, lands: &mut [Option<u64>], j_start: u64) {
    let mut nops: Vec<u64> = lands.iter().flatten().copied().filter(|&l| l < j_start).collect();
    nops.sort_unstable();
    nops.dedup();
    if nops.len() < 2 {
        return;
    }
    let addrs: Vec<u64> =
        fd.obank().iter_alive().filter_map(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset()).collect();
    let mut to = HashMap::new();
    for w in nops.windows(2).rev() {
        let (a, b) = (w[0], w[1]);
        if !addrs.iter().any(|&x| x >= a && x < b) {
            let end = *to.get(&b).unwrap_or(&b);
            to.insert(a, end);
        }
    }
    for l in lands.iter_mut().flatten() {
        if let Some(&e) = to.get(l) {
            *l = e;
        }
    }
}

/// The registers block `j` computes itself: values it derives rather than
/// copies in from a predecessor.
fn held(fd: &Funcdata, j: BlockId) -> Vec<(i32, u64, int4)> {
    let mut out = Vec::new();
    for o in fd.bb_ops(j) {
        let Some(vn) = fd.obank().get(o).and_then(|op| op.get_out()) else { continue };
        let Some(v) = fd.vbank().get(vn) else { continue };
        if v.get_space().get_type() == spacetype::IPTR_PROCESSOR && local(fd, vn, j, 8) {
            out.push((v.get_space().get_index(), v.get_offset(), v.get_size()));
        }
    }
    out
}

fn local(fd: &Funcdata, vn: VarnodeId, j: BlockId, depth: u32) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else { return !v.is_input() };
    if op.get_parent() != Some(j) {
        return false;
    }
    match op.code() {
        OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => false,
        OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            depth > 0 && op.get_in(0).is_some_and(|i| local(fd, i, j, depth - 1))
        }
        _ => true,
    }
}

/// The recorded statements of RETURN block `b`: its in-edge indices grouped
/// by the return statement each came through.  `None` when the function has
/// no record or an in-edge carries no tag.
pub(crate) fn groups(fd: &Funcdata, b: BlockId) -> Option<Vec<Vec<int4>>> {
    let rec = fd.kuna_o0returns.as_deref()?;
    let op = fd.bb_op_tail(b).and_then(|t| fd.obank().get(t))?;
    if op.code() != OpCode::CPUI_RETURN || op.get_halt_type() != 0 {
        return None;
    }
    let valued = op.num_input() >= 2;
    let held = valued && {
        let proto = fd.get_func_proto();
        let (a, sz) = if proto.has_store() {
            let out = proto.get_output();
            (out.get_address(), out.get_size())
        } else {
            let v = fd.vbank().get(op.get_in(1)?)?;
            (v.get_addr().clone(), v.get_size())
        };
        let s = a.get_space().map_or(-1, |s| s.get_index());
        let (off, sz) = (a.get_offset(), sz as u64);
        rec.held.iter().any(|&(hs, ho, hz)| hs == s && ho < off + sz && off < ho + hz as u64)
    };
    let blk = fd.bblocks_ref().block(b);
    let mut parts: Vec<(u16, Vec<int4>)> = Vec::new();
    for i in 0..blk.size_in() {
        let tag = ((blk.get_in_label(i) & TAG_MASK) >> TAG_SHIFT) as usize;
        let &(gv, gd) = rec.tags.get(tag.checked_sub(1)?)?;
        let gid = if held { 0 } else if valued { gv } else { gd };
        match parts.iter_mut().find(|(g, _)| *g == gid) {
            Some((_, p)) => p.push(i),
            None => parts.push((gid, vec![i])),
        }
    }
    Some(parts.into_iter().map(|(_, p)| p).collect())
}

/// Whether the record decides RETURN block `b`: the function's returns are
/// all placed, or `b`'s in-edges are all recorded.
pub(crate) fn recorded(fd: &Funcdata, b: BlockId) -> bool {
    decided(fd) || groups(fd, b).is_some()
}

/// Whether [`split`] placed every return of the function.
pub(crate) fn decided(fd: &Funcdata) -> bool {
    fd.kuna_o0returns.as_deref().is_some_and(|r| r.decided)
}

fn returns(fd: &Funcdata) -> Vec<BlockId> {
    let mut out: Vec<BlockId> = Vec::new();
    let ops: Vec<OpId> = fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
    for o in ops {
        let Some(op) = fd.obank().get(o) else { continue };
        if op.is_dead() || op.get_halt_type() != 0 {
            continue;
        }
        if let Some(p) = op.get_parent() {
            if !out.contains(&p) {
                out.push(p);
            }
        }
    }
    out
}

/// Give every recorded return statement its own copy of the epilogue; the
/// largest group keeps the original.  Returns the number of splits.
pub(crate) fn split(fd: &mut Funcdata, max_splits: int4, max_inedges: int4) -> int4 {
    if fd.kuna_o0returns.is_none() || decided(fd) {
        return 0;
    }
    let mut count = 0;
    for b in returns(fd) {
        let b = joined(fd, b);
        loop {
            let Some(parts) = groups(fd, b) else { break };
            if parts.len() < 2
                || count >= max_splits
                || fd.bblocks_ref().block(b).size_in() > max_inedges
                || !(fd.return_split_is_splittable(b) || pure_epilogue(fd, b))
            {
                break;
            }
            let keep = (0..parts.len()).max_by_key(|&k| (parts[k].len(), parts[k].contains(&0))).unwrap_or(0);
            let Some(part) = parts.iter().enumerate().rev().find(|&(k, _)| k != keep).map(|(_, p)| p.clone()) else {
                break;
            };
            if part.len() > 1 && has_multiequal(fd, b) {
                break;
            }
            let blk = fd.bblocks_ref().block(b);
            let preds: Vec<BlockId> = part.iter().map(|&i| blk.get_in(i)).collect();
            let (p0, slot) = (preds[0], blk.get_in_rev_index(part[0]));
            if fd.node_split(b, part[0]).is_err() {
                break;
            }
            count += 1;
            let nb = fd.bblocks_ref().block(p0).get_out(slot);
            for &q in &preds[1..] {
                fd.switch_edge(q, b, nb);
            }
        }
    }
    let all = returns(fd).into_iter().all(|b| groups(fd, b).is_some_and(|g| g.len() <= 1));
    if let Some(rec) = fd.kuna_o0returns.as_deref_mut() {
        rec.decided = all;
    }
    count
}

/// RETURN block `b`, or, when its one untagged in-edge comes from a block that
/// only runs into it (the join left by a stripped stack-protector check), the
/// two spliced into that block.
fn joined(fd: &mut Funcdata, b: BlockId) -> BlockId {
    let g = fd.bblocks_ref();
    if g.block(b).size_in() != 1 || g.block(b).get_in_label(0) & TAG_MASK != 0 {
        return b;
    }
    let p = g.block(b).get_in(0);
    let pb = g.block(p);
    let tagged = pb.size_in() > 0 && (0..pb.size_in()).all(|i| pb.get_in_label(i) & TAG_MASK != 0);
    if pb.size_out() != 1 || !tagged || fd.splice_block_basic(p).is_err() {
        return b;
    }
    p
}

/// An epilogue that only computes the returned value: arithmetic over
/// settled values, no memory access, call or branch.
fn pure_epilogue(fd: &Funcdata, b: BlockId) -> bool {
    let ops = fd.bb_ops(b);
    ops.len() <= MAX_STATEMENT_OPS
        && ops.iter().all(|&o| {
            let Some(op) = fd.obank().get(o) else { return false };
            let quiet = !matches!(
                op.code(),
                OpCode::CPUI_STORE
                    | OpCode::CPUI_LOAD
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_CALLOTHER
                    | OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_INDIRECT
                    | OpCode::CPUI_NEW
            );
            let settled = (0..op.num_input())
                .filter_map(|i| op.get_in(i))
                .all(|v| fd.vbank().get(v).is_some_and(|v| v.is_constant() || v.is_annotation() || !v.is_free()));
            let local = op.get_out().and_then(|v| fd.vbank().get(v)).is_none_or(|v| !v.is_persist());
            quiet && settled && local
        })
}

fn has_multiequal(fd: &Funcdata, b: BlockId) -> bool {
    fd.bb_ops(b).iter().any(|&o| fd.obank().get(o).is_some_and(|op| op.code() == OpCode::CPUI_MULTIEQUAL))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fall_through_lands_on_the_jump_target_in_its_gap() {
        let e = [Some(Entry::Jump(0xc8)), Some(Entry::Fall(0xc3))];
        assert_eq!(landings(&e, 0xc9), vec![Some(0xc8), Some(0xc8)]);
        let e = [Some(Entry::Jump(0xa6)), Some(Entry::Fall(0xa7))];
        assert_eq!(landings(&e, 0xa8), vec![Some(0xa6), Some(0xa8)]);
    }

    #[test]
    fn returndup_auto_keeps_duplication_on_and_adds_the_record() {
        let opt = crate::p8_structure::kuna_returndup::OptionReturnDup;
        assert!(matches!(opt.apply("auto"), Ok((true, false, true, true, _))));
        assert!(matches!(opt.apply("record"), Ok((true, false, true, false, _))));
        assert!(matches!(opt.apply("on"), Ok((true, false, false, false, _))));
        assert!(matches!(opt.apply("off"), Ok((false, false, false, false, _))));
        assert!(matches!(opt.apply("calls"), Ok((true, true, true, true, _))));
        assert!(opt.apply("sometimes").is_err());
    }

    #[test]
    fn numbered_shares_ids_between_equal_keys() {
        let keys = [Key::Landing(0x10), Key::Edge(1), Key::Landing(0x10), Key::Edge(3)];
        assert_eq!(numbered(&keys), vec![0, 1, 0, 2]);
    }
}
