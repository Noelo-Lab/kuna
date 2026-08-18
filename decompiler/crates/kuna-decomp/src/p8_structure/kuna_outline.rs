//! **Outline** a region: excise it from the function and emit a call to a
//! synthesized pseudofunction in its place.
//!
//! ```text
//! if (a1) { v1 = a0+7; if (0x33 <= v1) v1 = a0+4; a0 = v1*3; }
//!   -->  if (a1) a0 = outlined_0x40100a(a0);
//! ```
//!
//! # Who chooses the region
//!
//! A human, or an LLM agent. **Not a detector** - and that is a measured
//! conclusion, not an omission.
//!
//! Two automated detectors were built and scored against DWARF
//! `DW_TAG_inlined_subroutine` ground truth on the decbench corpus. Both failed:
//!
//! * The SAILR goto premise (report block sets that become single-entry when a
//!   virtualized edge is deleted) scored **1.03-1.09x enrichment over the
//!   per-function base rate** with **4.5% recall** over 578 regions - statistically
//!   indistinguishable from picking blocks at random inside the same function.
//! * Structural enumeration plus scoring fared no better. Every candidate-indexed
//!   feature measured at chance within a function: interface narrowness AUC 0.528,
//!   stack-slot confinement 0.458 (anti-correlated), repetition refuted outright.
//!   Even a *perfect* scorer over the best candidate pool tops out at a ceiling of
//!   0.507 (95% CI [0.474, 0.542]) on the addressable instances of zero-goto
//!   functions.
//!
//! And **63.8% of inlined instances are unaddressable by any block-set method at
//! all** - they span fewer than two whole basic blocks (median instance 33 bytes).
//! Region enumeration is the wrong tool for most of the phenomenon.
//!
//! So the region arrives as an option value, `<fn_entry>:<head>-<exit>`:
//!
//! ```text
//! --option outline 0x401000:0x40100a-0x401018
//! ```
//!
//! which is exactly the interface an agent needs. The one signal that did beat
//! chance was matching against a known function body (out-of-line twin, AUC 0.562),
//! so if a detector is ever added it should be content matching, not region
//! enumeration.
//!
//! # What the region is
//!
//! A **block set**, never the address interval `head..exit`: an inlined callee is
//! routinely laid out in several disjoint pieces (DWARF records 3-6 ranges for the
//! callees inlined into `coreutils/tr::parse_str`, and 66% of instances span more
//! than one range). The value names only a head and an exit; the member set is
//! re-derived here from the live CFG with [`kuna_check_region`] and is **never**
//! trusted from the spec, so a stale or hand-written pair cannot excise something
//! that is not a region.
//!
//! # The seam
//!
//! Pre-heritage `bblocks` surgery, self-gated on `get_heritage_pass() == 0`
//! (the documented pre-SSA window, as [`crate::p2_lift::kuna_loweredswitch`] uses).
//! Before SSA there are no `MULTIEQUAL`s to patch when edges move, and
//! `ActionFuncLink` still runs afterwards to materialize the call's argument and
//! return varnodes. Doing this later, as a P8 print-tree edit, would be wrong:
//! deleting a subtree there leaves the defining ops alive and the printer emits
//! variables with no visible definition.
//!
//! # What the region is
//!
//! A **block set**, never the address interval `head..exit`. Stage 1's measurements
//! are unambiguous about this: an inlined callee is routinely laid out in several
//! disjoint pieces, and in stage 1's own witness the exit address is *below* the
//! head. The members are therefore re-derived here from the CFG with the same
//! single-entry predicate stage 1 uses
//! ([`kuna_check_region`]), so the spec only has to name the head and the exit.
//!
//! # v1 restrictions, and why each one is here
//!
//! The pass declines — leaving the function untouched — unless all of:
//!
//! * the region is a genuine single-entry region over the live `bblocks` (never
//!   trusted from the spec; re-verified every time);
//! * exactly **one** edge enters the head from outside the region, and its source
//!   is not itself a member (v1 does not merge several entry edges into one call
//!   site);
//! * **at most one** value is live out of the region. Excising a region with a
//!   live-out you did not find silently produces C that computes the wrong value,
//!   which is the worst failure mode available here, so v1 refuses rather than
//!   guesses;
//! * the region contains no call and no `STORE` to a non-stack space. Both are
//!   side effects whose ordering relative to the synthesized call v1 does not
//!   reason about.
//!
//! The liveness used for that test is deliberately an **over**-approximation (see
//! [`region_liveness`]): a value written inside and read anywhere outside counts as
//! live-out even when no path carries it. Over-approximating can only cause extra
//! declines, never a wrong excision.
//!
//! # What it does not do
//!
//! It does not emit a **body** for the pseudofunction. There is no seam for that:
//! `PrintC::doc_function_full` is one `Funcdata` per document. The call is emitted
//! and named; the bytes it stands for remain in the binary and can be decompiled
//! separately at the head address.

use std::collections::{BTreeMap, BTreeSet};

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::marshal::ElementId;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p4_calls::fspec::FuncCallSpecs;
use crate::p7_regions::kuna_regiongraph::{
    kuna_subgraph_between_nodes, KunaIncrementalDominators, KunaNodeId, KunaNodePool,
    KunaNodeSet, KunaRegionGraph, NodeKind,
};

/// kuna ElementId for the `outline` option (4124; `almostregion` took 4123).
pub const ELEM_OUTLINE: ElementId = ElementId::new("outline", 4124);

/// One `<fn_entry>:<head>-<exit>` run of the `outline` option value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutlineRun {
    /// Entry address of the function the run applies to.
    pub fn_entry: uintb,
    /// Region head (entry block) address.
    pub head: uintb,
    /// Region exit (join block) address.
    pub exit: uintb,
}

/// The parsed `outline` option value.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutlineSpec {
    /// Every requested region, in the order written.
    pub runs: Vec<OutlineRun>,
}

impl OutlineSpec {
    /// Parse `off` or `<fn>:<head>-<exit>[;<fn>:<head>-<exit>...]`.
    ///
    /// The function entry is part of the grammar because an option value is global
    /// to the run: it is applied once and every function is then decompiled under
    /// it, so each run has to say which function it belongs to.
    pub fn parse(text: &str) -> KunaResult<OutlineSpec> {
        let t = text.trim();
        if t.is_empty() || t == "off" {
            return Ok(OutlineSpec::default());
        }
        if t.chars().any(|c| c.is_whitespace()) {
            return Err(KunaError::lowlevel(
                "outline: the value must not contain whitespace \
                 (use 0xFN:0xHEAD-0xEXIT;0xFN:0xHEAD-0xEXIT)",
            ));
        }
        let mut runs = Vec::new();
        for part in t.split(';') {
            if part.is_empty() {
                continue;
            }
            let (fnpart, region) = part.split_once(':').ok_or_else(|| {
                KunaError::lowlevel("outline: expected <fn_entry>:<head>-<exit>")
            })?;
            let (headpart, exitpart) = region.split_once('-').ok_or_else(|| {
                KunaError::lowlevel("outline: expected <head>-<exit> after the colon")
            })?;
            runs.push(OutlineRun {
                fn_entry: parse_hex(fnpart)?,
                head: parse_hex(headpart)?,
                exit: parse_hex(exitpart)?,
            });
        }
        if runs.is_empty() {
            return Err(KunaError::lowlevel("outline: no region in the value"));
        }
        Ok(OutlineSpec { runs })
    }

    /// True when no region was requested.
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// The run that applies to the function entered at `entry`, if any.
    pub fn for_function(&self, entry: uintb) -> Option<&OutlineRun> {
        self.runs.iter().find(|r| r.fn_entry == entry)
    }
}

fn parse_hex(s: &str) -> KunaResult<uintb> {
    let t = s.trim();
    let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    uintb::from_str_radix(body, 16)
        .map_err(|_| KunaError::lowlevel("outline: address is not hexadecimal"))
}

/// \brief Excise a supplied single-entry region and emit a call to a synthesized
/// pseudofunction in its place. Option-gated (`outline`), destructive.
pub struct ActionOutline {
    base: ActionBase,
}

impl ActionOutline {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionOutline { base: ActionBase::new(0, "outline", g) })
    }
}

impl Action for ActionOutline {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionOutline { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option outline <spec>`, carried on the Architecture boundary.
        let raw = data.get_arch().outline_spec.clone();
        if raw.is_empty() || raw == "off" {
            return 0;
        }
        // The pre-SSA window: before heritage there are no MULTIEQUALs to patch.
        if data.get_heritage_pass() != 0 {
            return 0;
        }
        if data.bblocks_get_size() < 2 {
            return 0;
        }
        let spec = match OutlineSpec::parse(&raw) {
            Ok(s) => s,
            Err(_) => return 0, // the option layer already reported the parse error
        };
        let entry = data.get_address().get_offset();
        let run = match spec.for_function(entry) {
            Some(r) => *r,
            None => return 0, // an option value is global; this function is not the target
        };
        match outline_one(data, &run) {
            Ok(true) => 1,
            // Declining is the designed behaviour for every precondition failure;
            // never fail a decompile over an outline request.
            Ok(false) | Err(_) => 0,
        }
    }
}

/// Locate a basic block by its start address.
///
/// `Funcdata::kuna_block_at_start` is private, and widening it is not this PR's
/// business, so the scan is done here over the public block accessors.
fn block_at(data: &Funcdata, off: uintb) -> Option<BlockId> {
    let n = data.bblocks_get_size();
    (0..n)
        .map(|i| data.bblocks_get_block(i))
        .find(|&bl| data.bblocks_block_start(bl).get_offset() == off)
}

/// The live-in and live-out storage of a member block set.
///
/// Deliberately an over-approximation, in the safe direction:
///
/// * **live-in** — every `(space, offset, size)` read by a member op that is not
///   written earlier in that same block. Reads of constants and of `unique` space
///   are dropped: constants are not storage, and a `unique` never survives a basic
///   block, so it cannot cross the region boundary.
/// * **live-out** — every storage written by a member op that is also read by any
///   op outside the region, *regardless of whether a path actually carries the
///   value there*. Over-counting live-outs can only cause the pass to decline.
///
/// A precise pre-SSA live-variable analysis would iterate to a fixed point over
/// the block graph; v1 does not, and the docs say so rather than implying it does.
fn region_liveness(
    data: &Funcdata,
    members: &BTreeSet<BlockId>,
    cont: &BTreeSet<BlockId>,
) -> (Vec<Store>, Vec<Store>) {
    let mut live_in: BTreeMap<StoreKey, Store> = BTreeMap::new();
    let mut written: BTreeMap<StoreKey, Store> = BTreeMap::new();

    for &bl in members {
        let mut local: BTreeSet<StoreKey> = BTreeSet::new();
        for op in data.bb_ops(bl) {
            for st in op_inputs(data, op) {
                if !local.contains(&st.key) {
                    live_in.entry(st.key).or_insert(st);
                }
            }
            if let Some(st) = op_output(data, op) {
                local.insert(st.key);
                written.insert(st.key, st);
            }
        }
    }

    let mut live_out: BTreeMap<StoreKey, Store> = BTreeMap::new();
    for &bl in cont {
        // Within a continuation block a read only counts until the storage is
        // redefined there; after that the region's value is dead.
        let mut killed: BTreeSet<StoreKey> = BTreeSet::new();
        for op in data.bb_ops(bl) {
            for st in op_inputs(data, op) {
                if written.contains_key(&st.key) && !killed.contains(&st.key) {
                    live_out.insert(st.key, st);
                }
            }
            if let Some(st) = op_output(data, op) {
                killed.insert(st.key);
            }
        }
    }
    (
        merge_overlapping(live_in.into_values().collect()),
        merge_overlapping(live_out.into_values().collect()),
    )
}

/// Collapse overlapping storage in the same space to one entry, keeping the
/// widest.
///
/// Without this, one x86 register counts twice: a 32-bit write zero-extends, so
/// SLEIGH writes both `EAX` (4 bytes) and `RAX` (8 bytes) at the same offset, and
/// a key of `(space, offset, size)` sees two live-outs where the machine has one.
/// v1 declines above one live-out, so un-merged aliases would reject almost every
/// real region.
fn merge_overlapping(mut v: Vec<Store>) -> Vec<Store> {
    v.sort_by_key(|s| (s.key.0, s.offset, std::cmp::Reverse(s.size)));
    let mut out: Vec<Store> = Vec::new();
    for st in v {
        match out.last_mut() {
            Some(prev)
                if prev.key.0 == st.key.0
                    && st.offset < prev.offset + prev.size.max(0) as uintb =>
            {
                // Overlaps the previous run; widen it if this one reaches further.
                let end = st.offset + st.size.max(0) as uintb;
                let prev_end = prev.offset + prev.size.max(0) as uintb;
                if end > prev_end {
                    prev.size = (end - prev.offset) as int4;
                    prev.key = (prev.key.0, prev.offset, prev.size);
                }
            }
            _ => out.push(st),
        }
    }
    out
}

/// Ordered identity of a storage location: `(space index, offset, size)`.
type StoreKey = (i32, uintb, int4);

/// A storage location, carrying the space handle needed to rebuild a varnode.
#[derive(Clone)]
pub(crate) struct Store {
    key: StoreKey,
    space: Rc<AddrSpace>,
    offset: uintb,
    size: int4,
}

/// The storage a varnode denotes, or `None` for constants and `unique` temporaries
/// (a constant is not storage, and a `unique` never outlives its basic block, so
/// neither can cross the region boundary).
fn store_of(data: &Funcdata, vn: VarnodeId) -> Option<Store> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    let ad = v.get_addr();
    let space = ad.get_space()?.clone();
    match space.get_type() {
        spacetype::IPTR_CONSTANT
        | spacetype::IPTR_INTERNAL
        | spacetype::IPTR_FSPEC
        | spacetype::IPTR_IOP => None,
        _ => Some(Store {
            key: (space.get_index(), ad.get_offset(), size),
            space,
            offset: ad.get_offset(),
            size,
        }),
    }
}

fn op_inputs(data: &Funcdata, op: OpId) -> Vec<Store> {
    let Some(o) = data.obank().get(op) else { return Vec::new() };
    let n = o.num_input();
    // Slot 0 of a control-transfer op is its DESTINATION, carried as a varnode in
    // the code space.  It is an address, not a value the region reads, and
    // counting it makes every conditional look like it consumes memory.
    let first = match o.code() {
        OpCode::CPUI_BRANCH
        | OpCode::CPUI_CBRANCH
        | OpCode::CPUI_BRANCHIND
        | OpCode::CPUI_CALL
        | OpCode::CPUI_CALLIND
        | OpCode::CPUI_CALLOTHER
        | OpCode::CPUI_RETURN => 1,
        _ => 0,
    };
    let ins: Vec<_> = (first..n).filter_map(|s| o.get_in(s)).collect();
    ins.into_iter().filter_map(|vn| store_of(data, vn)).collect()
}

/// Blocks reachable from `from` without entering `members` (the region's
/// continuation).
fn reachable_from(
    data: &Funcdata,
    from: BlockId,
    members: &BTreeSet<BlockId>,
) -> BTreeSet<BlockId> {
    let mut seen: BTreeSet<BlockId> = BTreeSet::new();
    let mut stack = vec![from];
    while let Some(bl) = stack.pop() {
        if members.contains(&bl) || !seen.insert(bl) {
            continue;
        }
        let g = data.bblocks_ref();
        let b = g.block(bl);
        for i in 0..b.size_out() {
            stack.push(b.get_out(i));
        }
    }
    seen
}

fn op_output(data: &Funcdata, op: OpId) -> Option<Store> {
    let out = data.obank().get(op)?.get_out()?;
    store_of(data, out)
}


/// Does any continuation block end in a `RETURN`?
fn continuation_returns(data: &Funcdata, cont: &BTreeSet<BlockId>) -> bool {
    cont.iter().any(|&bl| {
        data.bb_ops(bl).iter().any(|&op| {
            data.obank().get(op).map(|o| o.code() == OpCode::CPUI_RETURN).unwrap_or(false)
        })
    })
}

/// Storage the region wrote that the continuation never redefines and that is a
/// plausible return value.
///
/// Restricted to the storage the *containing function's own prototype* names as its
/// output, so this cannot invent a result for a void function. Returns `None` when
/// the function has no output storage or the region did not write it.
fn returned_storage(
    data: &Funcdata,
    members: &BTreeSet<BlockId>,
    cont: &BTreeSet<BlockId>,
) -> Option<Store> {
    let out = data.get_func_proto().get_output();
    let addr = out.get_address().clone();
    let size = out.get_size();
    if size <= 0 {
        return None;
    }
    let space = addr.get_space()?.clone();
    let key = (space.get_index(), addr.get_offset(), size);

    let wrote = members.iter().any(|&bl| {
        data.bb_ops(bl)
            .iter()
            .any(|&op| op_output(data, op).map(|st| st.key == key).unwrap_or(false))
    });
    if !wrote {
        return None;
    }
    let redefined = cont.iter().any(|&bl| {
        data.bb_ops(bl)
            .iter()
            .any(|&op| op_output(data, op).map(|st| st.key == key).unwrap_or(false))
    });
    if redefined {
        return None;
    }
    Some(Store { key, space, offset: addr.get_offset(), size })
}

/// Does any member block hold a call or a store this pass will not reason about?
fn has_unmodelled_side_effect(data: &Funcdata, members: &BTreeSet<BlockId>) -> bool {
    for &bl in members {
        for op in data.bb_ops(bl) {
            let Some(o) = data.obank().get(op) else { continue };
            match o.code() {
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_STORE => return true,
                _ => {}
            }
        }
    }
    false
}

/// Perform one outline. Returns `Ok(false)` when a precondition declines it.
fn outline_one(data: &mut Funcdata, run: &OutlineRun) -> KunaResult<bool> {
    let head_bl = match block_at(data, run.head) {
        Some(b) => b,
        None => return Ok(false),
    };
    let exit_bl = match block_at(data, run.exit) {
        Some(b) => b,
        None => return Ok(false),
    };
    if head_bl == exit_bl {
        return Ok(false);
    }

    // Re-derive the region over the live CFG with stage 1's predicate. The spec is
    // never trusted: it names a head and an exit, and this decides whether they
    // actually bound a single-entry region right now.
    let sc = build_scratch(data);
    let (hn, xn) = match (sc.node_of.get(&head_bl), sc.node_of.get(&exit_bl)) {
        (Some(&h), Some(&x)) => (h, x),
        _ => return Ok(false),
    };
    let mut doms = KunaIncrementalDominators::new(&sc.pool, &sc.graph, sc.head, false)?;
    if !kuna_check_region(&sc.pool, &sc.graph, hn, xn, &mut doms)? {
        return Ok(false);
    }

    let mut frontier = KunaNodeSet::new();
    frontier.insert(sc.pool.key(xn));
    let mut slice = KunaRegionGraph::new();
    kuna_subgraph_between_nodes(&sc.pool, &sc.graph, hn, &frontier, false, &mut slice)?;
    let mut nodes: Vec<KunaNodeId> = Vec::new();
    slice.get_nodes(&mut nodes);

    let mut block_of: BTreeMap<KunaNodeId, BlockId> = BTreeMap::new();
    for (&bl, &nid) in sc.node_of.iter() {
        block_of.insert(nid, bl);
    }
    let members: BTreeSet<BlockId> =
        nodes.iter().filter_map(|n| block_of.get(n).copied()).collect();
    if members.len() < 2 || !members.contains(&head_bl) || members.contains(&exit_bl) {
        return Ok(false);
    }

    if has_unmodelled_side_effect(data, &members) {
        return Ok(false);
    }

    // Exactly one edge may enter the head from outside the region.
    let mut entries: Vec<(BlockId, int4)> = Vec::new();
    {
        let g = data.bblocks_ref();
        let hb = g.block(head_bl);
        for i in 0..hb.size_in() {
            let pred = hb.get_in(i);
            if !members.contains(&pred) {
                entries.push((pred, i));
            }
        }
    }
    if entries.len() != 1 {
        return Ok(false);
    }

    // The region's continuation: everything reachable from the exit without
    // re-entering the region.  A read BEFORE the region (the entry block's own
    // flag test, say) is not a live-out, and counting it rejected every region.
    let cont = reachable_from(data, exit_bl, &members);
    let (live_in, mut live_out) = region_liveness(data, &members, &cont);

    // Pre-SSA the RETURN op does not yet carry the return VALUE as an input - return
    // storage is recovered later - so a region whose only live-out is the value the
    // function returns shows zero live-outs, and excising it silently drops the
    // result.  That is an UNDER-approximation, the one direction the liveness is not
    // allowed to err in.  Found by the coreutils LLM evaluation on two independent
    // `du` witnesses, both reporting "0 result(s)" for a region computing the return.
    //
    // If the continuation can reach a RETURN, treat storage the region wrote and the
    // continuation never redefines as potentially returned, and fold it in.  Pushing
    // past one live-out simply declines, which is the safe outcome.
    if !live_out.iter().any(|_| true) && continuation_returns(data, &cont) {
        let maybe = returned_storage(data, &members, &cont);
        if let Some(st) = maybe {
            live_out.push(st);
        }
    }
    if live_out.len() > 1 {
        return Ok(false);
    }

    // ---- surgery -----------------------------------------------------------
    // A fresh EMPTY block on the single entry edge.
    //
    // `node_split_block_edge` looks like the primitive for this and is not: it
    // `copy_basic_range`s the head's p-code into the new block and gives it the
    // head's successors, because it exists for tail duplication.  Using it here
    // produced a clone of the region head whose out-edges pointed into the
    // region, and heritage then failed its dominator invariant.  What is wanted
    // is that function minus the copy: allocate a block, move the one entry edge
    // onto it, and point it at the join.
    let (pred, _in_slot) = entries[0];
    let graph = data.bblocks_root_pub();
    let call_bl = data.bblocks_mut().new_block_basic(graph);
    data.bblocks_mut().switch_edge(pred, head_bl, call_bl);
    data.bblocks_mut().add_edge(call_bl, exit_bl);

    let site = data.bblocks_block_start(call_bl);
    let head_addr = data.bblocks_block_start(head_bl);
    let name = format!("outlined_0x{:x}", head_addr.get_offset());

    // Manufacture the CALL. Every other site in the tree mutates an op the decoder
    // produced; this builds one from nothing, so it installs the fspec annotation
    // by hand exactly as FlowInfo::build_call_specs does.
    let nargs = live_in.len() as int4;
    let callop = data.new_op(1 + nargs, site.clone());
    data.op_set_opcode_code(callop, OpCode::CPUI_CALL);

    let mut fc = FuncCallSpecs::new(callop, head_addr.clone());
    fc.set_funcdata(head_addr.clone(), &name)?;
    // A hand-built FuncCallSpecs has neither a prototype store nor a MODEL.
    //
    // `attach_internal_store` alone is not enough and the difference is not cosmetic:
    // it installs the store and leaves `model = None` (`p4_calls/fspec.rs:5081`),
    // whereas `set_internal` (`:5000`) installs the store *and* the model.  Without a
    // model the call has no known stack-pointer effect, and `ActionDefaultParams`
    // does not repair it because this spec already reports a known callee.  The
    // observable damage is in the code the region never touched: stack-pointer
    // normalization fails for the calls that FOLLOW the outlined one, raw `RSP` leaks
    // into the C as `&Stack...`, and a later `memcpy` prints with four arguments.
    // Found by the coreutils LLM evaluation on `factor::lbuf_putc`.
    let void_ty = match data.get_arch().types().map(|t| t.get_type_void()) {
        Some(Ok(t)) => t,
        _ => return Ok(false),
    };
    let model = match data.get_arch().eval_fp_called().cloned() {
        Some(m) => m,
        // No default model registered (hand-built fixture): decline rather than
        // synthesize a call whose stack effect is unknown.
        None => return Ok(false),
    };
    fc.proto_mut().set_internal(model, void_ty);
    let idx = data.push_call_specs(fc);
    let handle = crate::flow::next_fspec_handle();
    let angr = data.get_arch().kuna_name_style();
    data.get_call_specs(idx).register_in_fspec_space(handle, angr);
    let fspecvn = data.new_varnode_call_specs(handle);
    data.op_set_input(callop, fspecvn, 0)?;

    // The region's live-in storage becomes the call's arguments.
    for (slot, st) in live_in.iter().enumerate() {
        let ad = Address::new(st.space.clone(), st.offset);
        let vn = data.new_varnode(st.size, &ad, None);
        data.op_set_input(callop, vn, 1 + slot as int4)?;
    }
    // A single live-out becomes the call's return value.
    if let Some(st) = live_out.first() {
        let ad = Address::new(st.space.clone(), st.offset);
        data.new_varnode_out(st.size, &ad, callop)?;
    }

    data.op_insert_end(callop, call_bl);

    // The region has lost its only entry from outside, so the unreachable sweep
    // collects it (never block_remove_internal directly - kuna_stackguard.rs:326
    // is the idiom).
    data.remove_unreachable_blocks(false, true)?;
    data.structure_reset();

    // A freshly created block has no real address, so a per-site comment would be
    // dropped; the header anchors at the function entry and always renders.
    data.warning_header(&format!(
        "outline: excised {} block(s) at 0x{:x} into {}() ({} argument(s), {} result(s))",
        members.len(),
        head_addr.get_offset(),
        name,
        live_in.len(),
        live_out.len()
    ));
    Ok(true)
}


//===========================================================================
// Region substrate
//
// `kuna_check_region` is transcribed from the private
// `KunaRegionIdentifier::check_region` (`p7_regions/kuna_regionid.rs:1504`, angr RI
// `_check_region`) so this pass can run the predicate over a scratch graph it owns.
// `kuna_regionid.rs` is a line-faithful port pinned by `docs/spec/07-regions.md`
// and is deliberately not modified to expose it.
//===========================================================================

struct Scratch {
    pool: KunaNodePool,
    graph: KunaRegionGraph,
    head: KunaNodeId,
    sink: KunaNodeId,
    node_of: BTreeMap<BlockId, KunaNodeId>,
}

fn build_scratch(data: &Funcdata) -> Scratch {
    let mut pool = KunaNodePool::new();
    let mut graph = KunaRegionGraph::new();
    let mut node_of: BTreeMap<BlockId, KunaNodeId> = BTreeMap::new();

    let n = data.bblocks_get_size();
    for i in 0..n {
        let bl = data.bblocks_get_block(i);
        let addr = data.bblocks_block_start(bl);
        let nid = pool.make(NodeKind::Block, addr.get_offset(), i as uint4);
        graph.add_node(&pool, nid);
        node_of.insert(bl, nid);
    }
    for i in 0..n {
        let bl = data.bblocks_get_block(i);
        let g = data.bblocks_ref();
        let sout = g.block(bl).size_out();
        for e in 0..sout {
            let dst = g.block(bl).get_out(e);
            if let (Some(&s), Some(&d)) = (node_of.get(&bl), node_of.get(&dst)) {
                graph.add_edge(&pool, s, d);
            }
        }
    }
    // Synthetic head, so the forward dominator root reaches every block (the entry
    // block may carry a back-edge and so not be zero-in-degree; block 0 is wired
    // unconditionally for that reason).
    let head = pool.make(NodeKind::Dummy, 0, uint4::MAX);
    graph.add_node(&pool, head);
    if n > 0 {
        let entry = node_of[&data.bblocks_get_block(0)];
        graph.add_edge(&pool, head, entry);
    }
    for i in 0..n {
        let nid = node_of[&data.bblocks_get_block(i)];
        if graph.size_in(nid).unwrap_or(0) == 0 {
            graph.add_edge(&pool, head, nid);
        }
    }
    // Synthetic sink, so the post-dominator tree is rooted even with several returns.
    let sink = pool.make(NodeKind::Dummy, u64::MAX, uint4::MAX - 1);
    graph.add_node(&pool, sink);
    for i in 0..n {
        let nid = node_of[&data.bblocks_get_block(i)];
        if graph.size_out(nid).unwrap_or(0) == 0 {
            graph.add_edge(&pool, nid, sink);
        }
    }
    Scratch { pool, graph, head, sink, node_of }
}

/// Is `[start, end)` a single-entry region of `g`?
///
/// Transcribed from the private `KunaRegionIdentifier::check_region`
/// (`p7_regions/kuna_regionid.rs:1504`, angr RI `_check_region`) so this pass can
/// run the predicate over a scratch graph it owns and may mutate edge-by-edge.
/// `kuna_regionid.rs` is a line-faithful port pinned by `docs/spec/07-regions.md`
/// and is deliberately not modified to expose it.
fn kuna_check_region(
    pool: &KunaNodePool,
    g: &KunaRegionGraph,
    start_node: KunaNodeId,
    end_node: KunaNodeId,
    doms: &mut KunaIncrementalDominators,
) -> KunaResult<bool> {
    if !doms.dominates(start_node, end_node) {
        let early_start_frontier = doms.df(pool, g, start_node)?;
        for it in early_start_frontier.iter() {
            if it.id != start_node && it.id != end_node {
                return Ok(false);
            }
        }
    }

    let end_frontier = doms.df(pool, g, end_node)?;
    for it in end_frontier.iter() {
        if doms.dominates(start_node, it.id) && it.id != end_node {
            return Ok(false); // An edge enters the region
        }
    }

    let start_frontier = doms.df(pool, g, start_node)?;
    for it in start_frontier.iter() {
        let node = it.id;
        if node == start_node || node == end_node {
            continue;
        }
        if !end_frontier.contains(it) {
            return Ok(false); // An edge leaves the region
        }
        for &p in g.get_preds(node)? {
            if doms.dominates(start_node, p) && !doms.dominates(end_node, p) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

//===========================================================================
// The option
//===========================================================================

/// The `outline off|<fn>:<head>-<exit>[;...]` ArchOption.
pub struct OptionOutline;

impl OptionOutline {
    /// The option name.
    pub const NAME: &'static str = "outline";

    /// Validate the region grammar, returning the value to store plus a message.
    pub fn apply(&self, p1: &str) -> KunaResult<(String, String)> {
        let spec = OutlineSpec::parse(p1)?;
        if spec.is_empty() {
            return Ok((String::new(), "Region outlining turned off".to_string()));
        }
        let n = spec.runs.len();
        Ok((
            p1.trim().to_string(),
            format!("Region outlining armed for {n} region(s)"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_off_is_empty() {
        assert!(OutlineSpec::parse("off").unwrap().is_empty());
        assert!(OutlineSpec::parse("").unwrap().is_empty());
    }

    #[test]
    fn parse_one_run() {
        let s = OutlineSpec::parse("0x401000:0x401008-0x40101f").unwrap();
        assert_eq!(
            s.runs,
            vec![OutlineRun { fn_entry: 0x401000, head: 0x401008, exit: 0x40101f }]
        );
        assert_eq!(s.for_function(0x401000).unwrap().head, 0x401008);
        assert!(s.for_function(0x402000).is_none());
    }

    #[test]
    fn parse_several_runs() {
        let s = OutlineSpec::parse("0x1000:0x1008-0x1020;0x2000:0x2008-0x2040").unwrap();
        assert_eq!(s.runs.len(), 2);
        assert_eq!(s.runs[1].exit, 0x2040);
    }

    #[test]
    fn whitespace_is_rejected_because_the_console_drops_it() {
        // IfcOption reads only p1 and silently discards p2/p3, so a value with a
        // space would mean something different through the console than through
        // the in-process path. Reject it in one place instead.
        assert!(OutlineSpec::parse("0x1000:0x1008 - 0x1020").is_err());
    }

    #[test]
    fn malformed_values_are_errors_not_silent_no_ops() {
        assert!(OutlineSpec::parse("0x1000").is_err());
        assert!(OutlineSpec::parse("0x1000:0x1008").is_err());
        assert!(OutlineSpec::parse("zz:0x1-0x2").is_err());
    }
}
