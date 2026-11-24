//! (kuna `iteregion auto`) Print a conditional value the way an unoptimized
//! function wrote it.
//!
//! An unoptimizing compiler finishes every statement with a store to the
//! variable it assigns, and computes an expression in registers.  The two
//! spellings of a conditional value therefore compile apart:
//!
//! ```c
//! x = c ? a : b;              // each arm loads into a register; one store after the join
//! if (c) x = a; else x = b;   // each arm stores to x
//! ```
//!
//! Once the stack is in SSA form both are one merge of two values, which
//! `iteregion` prints as `?:` or not by the shape of the arms.  [`record`]
//! reads the raw block graph at the first heritage pass of a function that
//! [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized: a branch whose
//! two arms only compute a register that the join reads is a conditional
//! value, and a branch whose two arms each store is a statement.  The record
//! keys each decided branch by the address of its `CBRANCH`, which every later
//! copy of that branch keeps.  A branch whose value is only returned is left
//! to [`crate::p8_structure::kuna_o0returns`].

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
    let mut sites = Vec::new();
    for i in 0..fd.bblocks_get_size() {
        let c = fd.bblocks_get_block(i);
        if let Some(site) = classify(fd, c) {
            sites.push(site);
        }
    }
    sites.sort_unstable_by_key(|s| s.0);
    sites.dedup_by_key(|s| s.0);
    fd.kuna_o0select = Some(Box::new(Record { sites }));
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
    let (at_t, at_e) = (arm(fd, t)?, arm(fd, e)?);
    if at_t.stores && at_e.stores {
        return Some((at, Form::Statement));
    }
    if at_t.stores || at_e.stores || tb.size_in() != 1 || eb.size_in() != 1 {
        return None;
    }
    let (st, se) = (slot(fd, j, t)?, slot(fd, j, e)?);
    let joined = fd.bb_ops(j).into_iter().any(|o| {
        let Some(op) = fd.obank().get(o) else { return false };
        if op.code() != OpCode::CPUI_MULTIEQUAL || !op.get_out().is_some_and(|v| scratch(fd, v)) {
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
        op.get_out().is_some_and(|v| used(fd, v, USE_DEPTH, dt && de))
    });
    joined.then_some((at, Form::Value))
}

/// Whether `vn` is a register a call does not preserve: where an unoptimizing
/// compiler evaluates an expression, not where it keeps a `register` variable.
fn scratch(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).is_some_and(|v| {
        !memory(fd, vn)
            && fd.get_func_proto().has_effect(&v.get_addr(), v.get_size()) != crate::fspec::effect_type::UNAFFECTED
    })
}

/// The in-edge index of `j` that comes from `x`.
fn slot(fd: &Funcdata, j: BlockId, x: BlockId) -> Option<usize> {
    let jb = fd.bblocks_ref().block(j);
    (0..jb.size_in()).find(|&k| jb.get_in(k) == x).map(|k| k as usize)
}

struct Arm {
    stores: bool,
}

/// What arm `x` does.  The return address a call instruction pushes is not a
/// store of the arm's.
fn arm(fd: &Funcdata, x: BlockId) -> Option<Arm> {
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
    let mut a = Arm { stores: false };
    for o in ops {
        let op = fd.obank().get(o)?;
        match op.code() {
            OpCode::CPUI_STORE => a.stores |= !calls.contains(&op.get_addr().get_offset()),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {}
            OpCode::CPUI_RETURN | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CBRANCH => return None,
            OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => {}
            _ => a.stores |= op.get_out().is_some_and(|v| memory(fd, v)),
        }
    }
    Some(a)
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
    rec.sites.binary_search_by_key(&addr, |s| s.0).ok().map(|i| rec.sites[i].1)
}

/// How the source wrote the branch of `CBRANCH` op `op`, if recorded.
pub(crate) fn form_of(fd: &Funcdata, op: crate::context::OpId) -> Option<Form> {
    form(fd, fd.obank().get(op)?.get_addr().get_offset())
}

/// TEMP debug: the recorded sites and the structured shape each ended in.
pub(crate) fn dump(fd: &Funcdata) {
    let Ok(dir) = std::env::var("KUNA_O0SEL_DUMP") else { return };
    let Some(rec) = fd.kuna_o0select.as_deref() else { return };
    if rec.sites.is_empty() {
        return;
    }
    let mut shapes: Vec<(u64, String)> = Vec::new();
    if fd.sblocks_get_size() > 0 {
        let mut stack = vec![fd.sblocks_root()];
        while let Some(id) = stack.pop() {
            let g = fd.sblocks_ref();
            let blk = g.block(id);
            for i in 0..blk.get_size() {
                stack.push(blk.get_block(i));
            }
            if blk.get_type() != crate::block::BlockType::If {
                continue;
            }
            let cond = blk.get_block(0);
            let tail = crate::p8_structure::kuna_itecondlist::cond_list_tail(fd, cond);
            let Some(bb) = last_leaf(fd, tail) else { continue };
            let Some(&cb) = crate::p8_structure::kuna_iteregion::printed_ops(fd, bb).last() else { continue };
            let Some(cbo) = fd.obank().get(cb) else { continue };
            if cbo.code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let at = cbo.get_addr().get_offset();
            let arm = |k: i32| -> String {
                if k >= blk.get_size() {
                    return "-".into();
                }
                let Some(bb) = crate::p8_structure::kuna_iteregion::leaf_bblock(fd, blk.get_block(k)) else {
                    return "X".into();
                };
                let ops = crate::p8_structure::kuna_iteregion::printed_ops(fd, bb);
                let names: Vec<String> = ops
                    .iter()
                    .filter_map(|&o| fd.obank().get(o))
                    .map(|o| format!("{:?}", o.code()).trim_start_matches("CPUI_").to_lowercase())
                    .collect();
                names.join("+")
            };
            let marked = crate::p8_structure::kuna_iteregion::match_ite_assignment(fd, id).is_some()
                || crate::p8_structure::kuna_iteboolean::match_ite_boolean(fd, id).is_some();
            let cond_kind = if tail != cond || g.block(tail).get_type() == crate::block::BlockType::Condition {
                "cc"
            } else {
                "c"
            };
            shapes.push((
                at,
                format!("{}:{}:{}|{}{}", blk.get_size(), cond_kind, arm(1), arm(2), if marked { ":M" } else { "" }),
            ));
        }
    }
    let mut out = String::new();
    for &(a, f) in &rec.sites {
        let s: Vec<&str> = shapes.iter().filter(|x| x.0 == a).map(|x| x.1.as_str()).collect();
        out.push_str(&format!(
            "{{\"fn\":\"{}\",\"entry\":{},\"at\":\"{:#x}\",\"form\":\"{:?}\",\"shape\":\"{}\"}}\n",
            fd.get_name(),
            fd.get_address().get_offset(),
            a,
            f,
            if s.is_empty() { "none".to_string() } else { s.join(";") }
        ));
    }
    let path = std::path::Path::new(&dir).join(format!("{}.jsonl", std::process::id()));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        use std::io::Write;
        let _ = f.write_all(out.as_bytes());
    }
}

fn last_leaf(fd: &Funcdata, id: BlockId) -> Option<BlockId> {
    let g = fd.sblocks_ref();
    let blk = g.block(id);
    match blk.get_type() {
        crate::block::BlockType::Condition => last_leaf(fd, blk.get_block(blk.get_size() - 1)),
        _ => crate::p8_structure::kuna_iteregion::leaf_bblock(fd, id),
    }
}

