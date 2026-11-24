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
//! * in a void function, where `return;` is only a jump or a `nop`, the
//!   instruction a path lands on: paths that land on the same address share
//!   it, and a block that jumps to the epilogue is a `return;` of its own.
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
    let lands = landings(&entries, j_start);
    let mut valued: Vec<Key> = Vec::new();
    let mut void: Vec<Key> = Vec::new();
    let mut pure: Vec<bool> = Vec::new();
    for (i, &x) in preds.iter().enumerate() {
        let i = i as int4;
        let stmt = statement_only(fd, x);
        valued.push(if stmt { Key::Block(x) } else { Key::Edge(i) });
        void.push(if stmt { Key::Block(x) } else { lands[i as usize].map_or(Key::Edge(i), Key::Landing) });
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
    dump_raw(fd, j, &valued, &void, &rec);
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
    while fd.bblocks_ref().block(j).size_in() == 1 {
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
    let jumps: Vec<u64> = entries.iter().filter_map(|e| match e {
        Some(Entry::Jump(d)) => Some(*d),
        _ => None,
    }).collect();
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
    let phi = if valued && std::env::var_os("KUNA_O0RET_VALCONST").is_some() {
        op.get_in(1).and_then(|v| fd.vbank().get(v)).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)).filter(|d| d.code() == OpCode::CPUI_MULTIEQUAL && d.get_parent() == Some(b))
    } else {
        None
    };
    let mut parts: Vec<(u16, Vec<int4>)> = Vec::new();
    for i in 0..blk.size_in() {
        let tag = ((blk.get_in_label(i) & TAG_MASK) >> TAG_SHIFT) as usize;
        let &(gv, gd) = rec.tags.get(tag.checked_sub(1)?)?;
        let mut gid = if held { 0 } else if valued { gv } else { gd };
        if let Some(m) = phi {
            let pure = statement_only(fd, blk.get_in(i));
            let konst = m.get_in(i).is_some_and(|v| const_value(fd, v));
            if !pure && !konst {
                gid = u16::MAX;
            }
        }
        match parts.iter_mut().find(|(g, _)| *g == gid) {
            Some((_, p)) => p.push(i),
            None => parts.push((gid, vec![i])),
        }
    }
    Some(parts.into_iter().map(|(_, p)| p).collect())
}

fn const_value(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    v.get_def().and_then(|d| fd.obank().get(d)).is_some_and(|d| {
        d.code() == OpCode::CPUI_COPY && d.get_in(0).and_then(|i| fd.vbank().get(i)).is_some_and(|i| i.is_constant())
    })
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
        loop {
            let Some(parts) = groups(fd, b) else { break };
            if parts.len() < 2
                || count >= max_splits
                || fd.bblocks_ref().block(b).size_in() > max_inedges
                || !fd.return_split_is_splittable(b)
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
        dump_late(fd, b);
    }
    let all = returns(fd).into_iter().all(|b| groups(fd, b).is_some_and(|g| g.len() <= 1));
    if let Some(rec) = fd.kuna_o0returns.as_deref_mut() {
        rec.decided = all;
    }
    if dump_dir().is_some() {
        dump_line(format!(
            "{{\"kind\":\"split\",\"file\":\"{}\",\"fn\":{},\"decided\":{all},\"count\":{count}}}",
            dump_file(fd),
            fd.get_address().get_offset()
        ));
    }
    count
}

fn has_multiequal(fd: &Funcdata, b: BlockId) -> bool {
    fd.bb_ops(b).iter().any(|&o| fd.obank().get(o).is_some_and(|op| op.code() == OpCode::CPUI_MULTIEQUAL))
}

fn dump_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("KUNA_O0RET_DUMP").map(std::path::PathBuf::from)
}

fn dump_file(fd: &Funcdata) -> String {
    fd.get_arch().loader.as_ref().map(|l| l.borrow().get_file_name().to_string()).unwrap_or_default()
}

fn dump_line(line: String) {
    use std::io::Write;
    let Some(dir) = dump_dir() else { return };
    let path = dir.join(format!("{}.jsonl", std::process::id()));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{line}");
    }
}

fn dump_raw(fd: &Funcdata, j: BlockId, valued: &[Key], void: &[Key], rec: &Record) {
    if dump_dir().is_none() {
        return;
    }
    let edges: Vec<String> = (0..valued.len())
        .map(|i| {
            let x = fd.bblocks_ref().block(j).get_in(i as int4);
            let start = fd.bb_op_head(x).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
            format!(
                "{{\"pred\":{start},\"valued\":\"{:?}\",\"void\":\"{:?}\",\"gv\":{},\"gd\":{}}}",
                valued[i], void[i], rec.tags[i].0, rec.tags[i].1
            )
        })
        .collect();
    let jstart = fd.bb_op_head(j).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
    dump_line(format!(
        "{{\"kind\":\"raw\",\"file\":\"{}\",\"fn\":{},\"name\":\"{}\",\"j\":{jstart},\"held\":{},\"edges\":[{}]}}",
        dump_file(fd),
        fd.get_address().get_offset(),
        fd.get_name(),
        rec.held.len(),
        edges.join(",")
    ));
}

fn dump_late(fd: &Funcdata, b: BlockId) {
    if dump_dir().is_none() {
        return;
    }
    let op = fd.bb_op_tail(b).and_then(|t| fd.obank().get(t));
    let valued = op.is_some_and(|o| o.num_input() >= 2);
    let parts = groups(fd, b).map(|g| g.iter().map(|p| p.len().to_string()).collect::<Vec<_>>().join(","));
    let blk = fd.bblocks_ref().block(b);
    let labs: Vec<String> = (0..blk.size_in())
        .map(|i| {
            let x = blk.get_in(i);
            let a = fd.bb_op_head(x).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
            format!("{a:x}:{:x}", blk.get_in_label(i))
        })
        .collect();
    dump_line(format!(
        "{{\"kind\":\"late\",\"file\":\"{}\",\"fn\":{},\"valued\":{valued},\"size_in\":{},\"parts\":\"{}\",\"labs\":\"{}\"}}",
        dump_file(fd),
        fd.get_address().get_offset(),
        blk.size_in(),
        parts.unwrap_or_else(|| "none".to_string()),
        labs.join(" ")
    ));
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
    fn numbered_shares_ids_between_equal_keys() {
        let keys = [Key::Landing(0x10), Key::Edge(1), Key::Landing(0x10), Key::Edge(3)];
        assert_eq!(numbered(&keys), vec![0, 1, 0, 2]);
    }
}
