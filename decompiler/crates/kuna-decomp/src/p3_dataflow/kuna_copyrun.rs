//! (kuna `copyrun`) An unrolled run of element copies becomes `memcpy`.
//!
//! A compiler expands a fixed-size copy (a struct assignment, `memcpy` with a
//! constant length) into loads and stores of the widest registers it has:
//!
//! ```text
//!     movdqu xmm0, [rsi]        ; 16 bytes at a time with SSE
//!     movups [rdi], xmm0
//!     mov    rax, [rsi+0x10]    ; or 8 at a time
//!     mov    [rdi+0x10], rax
//! ```
//!
//! which prints as one assignment per element (`*a0 = *a1; a0[1] = a1[1]; ...`).
//!
//! # What is proved
//!
//! After the main loop has settled, the pass groups the STOREs of each basic
//! block whose value is a LOAD of the same width in the same block, through
//! copies, read by nothing else. A pointer is read as a base plus a constant
//! offset, through COPY, CAST, `INT_ADD` of a constant, `PTRSUB` and `PTRADD` of a
//! constant index. Stores group when their destinations share a base, their
//! sources share a base, and each pair's destination offset minus its source
//! offset is the same. A group becomes `memcpy(dst + lo, src + lo', n)` only
//! when:
//!
//! * the bytes its stores write form one contiguous range of `n` bytes (stores
//!   may overlap: gcc copies 31 bytes with four 8-byte moves at 0, 8, 15 and
//!   23), and there are at least three stores, or two of 32 bytes together
//!   (two SSE moves), so a two-word assignment keeps its two statements;
//! * between the group's first and last op no other op reads or writes memory
//!   or calls, so moving every element to one point changes nothing the block
//!   can observe;
//! * when both bases are the same pointer, the source and destination ranges
//!   do not overlap;
//! * the destination is not in the function's own frame (a
//!   builtin placed after heritage would leave the frame locations it writes
//!   without a definition).
//!
//! # The rewrite
//!
//! The builtin is placed at the group's last store, the stores and loads (and
//! the copies between them) are destroyed. The INDIRECTs heritage hung on the
//! stores (an address-taken local or a global the destination may alias) form
//! one chain per location, a link per store; each chain is folded to one
//! INDIRECT on the builtin reading the chain's first input, since several
//! INDIRECTs of one location on one op cannot merge and print as
//! self-assignments. A run whose chain has another reader is left alone.
//!
//! The run copies element by element in program order, which `memcpy` does not
//! promise when the two ranges overlap through different pointers; the rewrite
//! cannot see that, which is what the option is for.

use std::collections::BTreeMap;

use kuna_base::error::KunaResult;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode::{self, *};

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;
use crate::userop::BUILTIN_MEMCPY;

/// A run of two stores is printed as `memcpy` from this many bytes (two SSE
/// moves); three or more stores always are.
const MIN_BYTES: i64 = 32;

/// How far a pointer is followed to its base.
const BASE_DEPTH: usize = 8;

/// (kuna) `ActionCopyRun` — collapse an unrolled element-copy run into `memcpy`
/// (option `copyrun`).
pub struct ActionCopyRun {
    base: ActionBase,
}

impl ActionCopyRun {
    pub fn new(g: impl Into<String>) -> Self {
        Self { base: ActionBase::new(0, "copyrun", g) }
    }
}

impl Action for ActionCopyRun {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(Self { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().copy_run {
            return 0;
        }
        let blocks: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
        let mut changed = 0;
        for block in blocks {
            for run in runs(data, block) {
                if rewrite(data, &run).is_err() {
                    return (changed != 0) as ApplyResult;
                }
                changed += 1;
            }
        }
        self.base.count += changed;
        (changed != 0) as ApplyResult
    }
}

/// One element copy: the store, the load, the copies between, and the offsets.
struct Pair {
    store: OpId,
    load: OpId,
    copies: Vec<OpId>,
    dst_off: i64,
    src_off: i64,
    width: i64,
    at: usize,
    first: usize,
}

/// A group to rewrite: the bases, the lowest offsets, the length, the pairs.
struct Run {
    dst: VarnodeId,
    src: VarnodeId,
    dst_lo: i64,
    src_lo: i64,
    bytes: i64,
    pairs: Vec<Pair>,
}

fn op(data: &Funcdata, id: OpId) -> Option<&PcodeOp> {
    data.obank().get(id)
}

fn signed_constant(data: &Funcdata, v: VarnodeId) -> Option<i64> {
    let n = data.vbank().get(v)?;
    if !n.is_constant() {
        return None;
    }
    let bits = 8 * n.get_size().min(8) as u32;
    let k = n.get_offset();
    Some(if bits >= 64 { k as i64 } else { ((k << (64 - bits)) as i64) >> (64 - bits) })
}

/// `v` as a base pointer plus a constant byte offset.
fn base_offset(data: &Funcdata, mut v: VarnodeId) -> Option<(VarnodeId, i64)> {
    let mut off = 0i64;
    for _ in 0..BASE_DEPTH {
        let Some(d) = data.vbank().get(v)?.get_def().and_then(|d| op(data, d)) else { break };
        let (Some(a), b) = (d.get_in(0), d.get_in(1)) else { break };
        match d.code() {
            CPUI_COPY | CPUI_CAST => v = a,
            CPUI_INT_ADD | CPUI_PTRSUB => match b.and_then(|b| signed_constant(data, b)) {
                Some(k) => {
                    off = off.wrapping_add(k);
                    v = a;
                }
                None => break,
            },
            CPUI_PTRADD => {
                let (Some(i), Some(e)) = (b.and_then(|b| signed_constant(data, b)), d.get_in(2).and_then(|e| signed_constant(data, e))) else { break };
                off = off.wrapping_add(i.wrapping_mul(e));
                v = a;
            }
            _ => break,
        }
    }
    Some((v, off))
}

/// Does `v` point into the function's own frame?
fn in_frame(data: &Funcdata, v: VarnodeId) -> bool {
    data.vbank().get(v).is_some_and(|n| n.is_spacebase())
}

/// The LOAD a stored value comes from through single-use copies, and those copies.
fn loaded(data: &Funcdata, mut v: VarnodeId) -> Option<(OpId, Vec<OpId>)> {
    let mut copies = Vec::new();
    for _ in 0..4 {
        let n = data.vbank().get(v)?;
        if n.descend_iter().count() != 1 {
            return None;
        }
        let d = n.get_def()?;
        let p = op(data, d)?;
        match p.code() {
            CPUI_LOAD => return Some((d, copies)),
            CPUI_COPY => {
                copies.push(d);
                v = p.get_in(0)?;
            }
            _ => return None,
        }
    }
    None
}

fn touches_memory(code: OpCode) -> bool {
    matches!(code, CPUI_LOAD | CPUI_STORE | CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER | CPUI_NEW | CPUI_INDIRECT)
}

/// The INDIRECTs heritage hung on `store`.
fn indirects_on(data: &Funcdata, ops: &[OpId], store: OpId) -> Vec<OpId> {
    ops.iter()
        .copied()
        .filter(|&o| {
            op(data, o).is_some_and(|p| {
                p.code() == CPUI_INDIRECT
                    && p.get_in(1).and_then(|v| data.vbank().get(v)).is_some_and(|v| {
                        v.get_space().get_type() == kuna_base::space::spacetype::IPTR_IOP
                            && crate::funcdata_varnode::op_iop_decode(v.get_offset()) == store
                    })
            })
        })
        .collect()
}

/// The INDIRECTs hung on the run's stores, one chain per storage location in
/// program order: a location the stores may alias carries one INDIRECT per
/// store, each reading the one before. `None` when a chain is broken or an
/// intermediate value is read by anything but the next INDIRECT, so the chain
/// cannot be folded onto the one builtin.
fn indirect_chains(data: &Funcdata, ops: &[OpId], pairs: &[Pair]) -> Option<Vec<Vec<OpId>>> {
    let mut stores: Vec<&Pair> = pairs.iter().collect();
    stores.sort_by_key(|p| p.at);
    let mut chains: BTreeMap<(i32, u64, i32), Vec<OpId>> = BTreeMap::new();
    for p in stores {
        for ind in indirects_on(data, ops, p.store) {
            let out = data.vbank().get(op(data, ind)?.get_out()?)?;
            let key = (out.get_space().get_index(), out.get_offset(), out.get_size());
            chains.entry(key).or_default().push(ind);
        }
    }
    for chain in chains.values() {
        for w in chain.windows(2) {
            let prev = op(data, w[0])?.get_out()?;
            if op(data, w[1])?.get_in(0) != Some(prev) || data.vbank().get(prev)?.descend_iter().count() != 1 {
                return None;
            }
        }
    }
    Some(chains.into_values().collect())
}

fn runs(data: &Funcdata, block: BlockId) -> Vec<Run> {
    let ops = data.bb_ops(block);
    let index: BTreeMap<OpId, usize> = ops.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let mut groups: BTreeMap<(VarnodeId, VarnodeId, i64), Vec<Pair>> = BTreeMap::new();
    for (at, &s) in ops.iter().enumerate() {
        let Some(st) = op(data, s).filter(|p| p.code() == CPUI_STORE) else { continue };
        let (Some(sp), Some(dptr), Some(val)) = (st.get_in(0), st.get_in(1), st.get_in(2)) else { continue };
        let Some((l, copies)) = loaded(data, val) else { continue };
        let Some(ld) = op(data, l) else { continue };
        let Some(&first) = index.get(&l) else { continue };
        let (Some(lsp), Some(sptr)) = (ld.get_in(0), ld.get_in(1)) else { continue };
        let same_space = data.vbank().get(sp).map(|v| v.get_offset()) == data.vbank().get(lsp).map(|v| v.get_offset());
        let width = data.vbank().get(val).map_or(0, |v| v.get_size()) as i64;
        if !same_space || width == 0 {
            continue;
        }
        let (Some((db, doff)), Some((sb, soff))) = (base_offset(data, dptr), base_offset(data, sptr)) else { continue };
        if in_frame(data, db) {
            continue;
        }
        groups.entry((db, sb, doff.wrapping_sub(soff))).or_default().push(Pair {
            store: s,
            load: l,
            copies,
            dst_off: doff,
            src_off: soff,
            width,
            at,
            first,
        });
    }
    let mut out = Vec::new();
    for ((dst, src, _), mut pairs) in groups {
        if pairs.len() < 2 {
            continue;
        }
        pairs.sort_by_key(|p| p.dst_off);
        let lo = pairs[0].dst_off;
        let mut end = lo;
        let mut contiguous = true;
        for p in &pairs {
            if p.dst_off > end {
                contiguous = false;
                break;
            }
            end = end.max(p.dst_off + p.width);
        }
        let bytes = end - lo;
        if !contiguous || (bytes < MIN_BYTES && pairs.len() < 3) {
            continue;
        }
        let src_lo = pairs[0].src_off;
        if dst == src && src_lo < end && lo < src_lo + bytes {
            continue;
        }
        let begin = pairs.iter().map(|p| p.first).min().unwrap_or(0);
        let last = pairs.iter().map(|p| p.at).max().unwrap_or(0);
        let mut mine: Vec<OpId> = Vec::new();
        for p in &pairs {
            mine.push(p.store);
            mine.push(p.load);
            mine.extend(indirects_on(data, &ops, p.store));
        }
        let clean = ops[begin..=last]
            .iter()
            .all(|o| mine.contains(o) || op(data, *o).is_some_and(|p| !touches_memory(p.code())));
        if clean && indirect_chains(data, &ops, &pairs).is_some() {
            out.push(Run { dst, src, dst_lo: lo, src_lo, bytes, pairs });
        }
    }
    out
}

/// `base + off` as a new op before `before`, or `base` itself when `off` is 0.
fn offset(data: &mut Funcdata, base: VarnodeId, off: i64, before: OpId) -> KunaResult<VarnodeId> {
    if off == 0 {
        return Ok(base);
    }
    let size = data.vbank().get(base).map_or(8, |v| v.get_size());
    let at = op(data, before).map(|p| p.get_addr().clone()).unwrap_or_else(kuna_base::address::Address::new_invalid);
    let add = data.new_op(2, at);
    data.op_set_opcode_code(add, CPUI_INT_ADD);
    data.op_set_input(add, base, 0)?;
    let k = data.new_constant(size, off as uintb);
    data.op_set_input(add, k, 1)?;
    let out = data.new_unique_out(size, add)?;
    data.op_insert_before(add, before);
    Ok(out)
}

fn rewrite(data: &mut Funcdata, run: &Run) -> KunaResult<()> {
    let last = run.pairs.iter().max_by_key(|p| p.at).map(|p| p.store).ok_or_else(|| kuna_base::error::KunaError::lowlevel("copyrun: empty run"))?;
    let dst = offset(data, run.dst, run.dst_lo, last)?;
    let src = offset(data, run.src, run.src_lo, last)?;
    let at = op(data, last).map(|p| p.get_addr().clone()).ok_or_else(|| kuna_base::error::KunaError::lowlevel("copyrun: stale store"))?;
    let call = data.new_op(4, at);
    data.op_set_opcode_code(call, CPUI_CALLOTHER);
    let id = data.new_constant(4, BUILTIN_MEMCPY as uintb);
    data.op_set_input(call, id, 0)?;
    data.op_set_input(call, dst, 1)?;
    data.op_set_input(call, src, 2)?;
    let size = data.vbank().get(run.dst).map_or(8, |v| v.get_size());
    let n = data.new_constant(size, run.bytes as uintb);
    data.op_set_input(call, n, 3)?;
    data.op_insert_before(call, last);
    let block = op(data, call).and_then(|p| p.get_parent());
    let ops = block.map(|b| data.bb_ops(b)).unwrap_or_default();
    let chains = indirect_chains(data, &ops, &run.pairs).ok_or_else(|| kuna_base::error::KunaError::lowlevel("copyrun: broken INDIRECT chain"))?;
    for chain in chains {
        let (Some(&first), Some(&ind)) = (chain.first(), chain.last()) else { continue };
        if first != ind {
            let head = op(data, first).and_then(|p| p.get_in(0)).ok_or_else(|| kuna_base::error::KunaError::lowlevel("copyrun: INDIRECT without input"))?;
            data.op_set_input(ind, head, 0)?;
            for &dead in chain[..chain.len() - 1].iter().rev() {
                data.op_destroy(dead);
            }
        }
        let iop = data.new_varnode_iop(call);
        data.op_set_input(ind, iop, 1)?;
        data.op_uninsert(ind);
        data.op_insert_before(ind, call);
    }
    let mut pairs: Vec<&Pair> = run.pairs.iter().collect();
    pairs.sort_by_key(|p| p.at);
    for p in pairs {
        data.op_destroy(p.store);
        for &c in &p.copies {
            data.op_destroy(c);
        }
        data.op_destroy(p.load);
    }
    Ok(())
}
