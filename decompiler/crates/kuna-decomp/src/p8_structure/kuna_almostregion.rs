//! S8 latent-region detection: find code the compiler **inlined** by looking for
//! *almost-regions* — block sets that would have collapsed into a single-entry
//! region if the structurer had not been forced to virtualize one edge into a
//! `goto`.
//!
//! # The premise, and its measured limits
//!
//! SAILR (Basque et al., USENIX Security 2024) observes that a residual `goto` is
//! not noise: it marks a place where the *compiler* restructured control flow past
//! what a source-level `if`/`while` nest can express. `crossjumprevert`,
//! `gotoreduce` and `taildup` already exploit that observation to undo three
//! specific optimizations. This pass exploits it to answer a different question —
//! *which bytes of this function did not come from this function?*
//!
//! The premise was measured against DWARF `DW_TAG_inlined_subroutine` ground truth
//! over 1,483 paired `-O2` / `-O2 -fno-inline` functions (5 decbench binaries,
//! `coreutils/tr`, `coreutils/ls`, `bzip2`, `gzip`, `tar`), and it is real but
//! **weak on its own**:
//!
//! - Inlining raises the raw structurer goto count 1.63x (3,513 vs 2,151).
//! - Per *byte* of code, however, inlined bytes are only **1.61x** as goto-dense as
//!   ordinary bytes (0.826 vs 0.514 gotos per 100 bytes). Goto *density* does not
//!   separate inlined from non-inlined code at all (0.671 vs 0.699) — the apparent
//!   function-level association is a size effect.
//!
//! So "there is a goto here" is not a usable inlining detector. What this pass
//! reports is the far more specific structural fact underneath it: a `goto` whose
//! removal *exposes a single-entry region that does not otherwise exist*. A
//! function's body is a single-entry region by construction; a random slice of a
//! caller is not. That is the signal.
//!
//! # Why the edge is deleted rather than un-virtualized
//!
//! Virtualization is recorded on `sblocks`, never on `bblocks`: all eleven
//! `set_goto_branch` call sites are on the structurer's own graph, and
//! [`KunaRegionIdentifier`](crate::p7_regions::kuna_regionid) rebuilds from
//! `bblocks` with no edge-flag filter. P7 therefore *already* sees the fully
//! un-virtualized CFG, and "un-virtualize the edge and re-run region
//! identification" would be a no-op — it would return the same regions P7 already
//! failed to find.
//!
//! The operation that actually carries the intended meaning is the inverse: build a
//! scratch region graph over `bblocks`, **delete** the edge the structurer
//! virtualized, and require that a region appears in `G - e` which is *not* a region
//! in `G`. Step 14 below asserts both directions, so a set that was already a
//! region is never reported.
//!
//! # Where it runs, and why that is load-bearing
//!
//! Immediately after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! and **before** [`ActionGotoReduce`](crate::p8_structure::kuna_gotoreduce::ActionGotoReduce)
//! and the rest of the goto-quality family. Those passes exist to *delete* gotos
//! by duplication, and they consume ~19% of the raw virtualized-edge set (3,513
//! raw vs 2,863 surviving over the same 1,483 functions). Reading the residue
//! instead of the raw set silently discards the evidence this pass exists to
//! read: the stage witness reports a region in a function whose final C contains
//! no `goto` at all, because `returndup` duplicated the shared return tail away
//! before printing.
//!
//! # What this pass does not do
//!
//! It is **report-only**. No p-code and no structured node is touched, so
//! `almostregion off` is byte-identical and `on` differs from `off` by comment
//! lines alone. Acting on a finding — excising the region and emitting a call to a
//! synthesized pseudofunction — is a separate, `destructive` decision and is not
//! made here.
//!
//! Detecting inlining *only* by this signal would also be a mistake, and the
//! measurements above say why: a callee that inlines tidily leaves no goto at all.
//! `openssh-portable/ssh::sshkey_drop_cert` at `-O2` has three DWARF-confirmed
//! inlined callees and **zero** gotos even with every goto-removal pass disabled.
//! This pass is one scored signal, deliberately narrow, and it says so in its
//! report rather than claiming coverage it does not have.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4};
use kuna_base::marshal::ElementId;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockType};
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::p7_regions::kuna_regiongraph::{
    kuna_subgraph_between_nodes, KunaIncrementalDominators, KunaNodeId, KunaNodePool,
    KunaNodeSet, KunaRegionGraph, NodeKind,
};

/// kuna ElementId for the `almostregion` option (4123; 4122 is the
/// `orchain`/`funcboundflow` high-water mark).
pub const ELEM_ALMOSTREGION: ElementId = ElementId::new("almostregion", 4123);

/// `almostregion` value: the pass is off.
pub const ALMOSTREGION_OFF: int4 = 0;
/// `almostregion` value: emit the per-function `// inline-candidate xN` slug only.
pub const ALMOSTREGION_ON: int4 = 1;
/// `almostregion` value: the slug plus one warning line per candidate region.
pub const ALMOSTREGION_REPORT: int4 = 2;

/// Maximum virtualized edges probed in one function. Each probe rebuilds two
/// dominator trees, so this is the pass's cost ceiling.
pub const MAX_PROBES: usize = 24;
/// Maximum basic blocks in a function the pass will look at.
pub const MAX_BLOCKS: int4 = 512;
/// A candidate must span at least this many basic blocks (below it, the "region"
/// is a shared tail, not a callee body).
pub const MIN_REGION_BLOCKS: usize = 2;
/// A candidate spanning more than this many blocks is a structuring failure, not
/// an inlined callee.
pub const MAX_REGION_BLOCKS: usize = 64;
/// Depth limit when walking the dominator chain above a goto source for a head.
pub const MAX_HEAD_CLIMB: usize = 4;

/// A latent single-entry region: a block set that becomes a region once one
/// virtualized edge is deleted, and is not a region while that edge is present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlmostRegion {
    /// Region head (entry) address.
    pub head: Address,
    /// The region's single exit / join address.
    pub exit: Address,
    /// Start address of the block the virtualized edge leaves.
    pub goto_src: Address,
    /// Start address of the block the virtualized edge targets.
    pub goto_tgt: Address,
    /// Start address of every basic block in the region, ascending.
    ///
    /// A region is a block *set*, not an address interval: an inlined callee is
    /// routinely laid out in several disjoint pieces (DWARF records 3-6 ranges for
    /// the callees inlined into `coreutils/tr::parse_str`), so consumers must take
    /// the members and never `head..exit`.
    pub members: Vec<Address>,
}

impl AlmostRegion {
    /// Basic blocks in the region.
    pub fn blocks(&self) -> usize {
        self.members.len()
    }
    /// Does the region occupy one contiguous run of blocks in address order?
    pub fn contiguous(&self) -> bool {
        self.members.len() < 2
    }
}

/// \brief Report block sets that would be single-entry regions but for one
/// virtualized edge, as candidate inlined-callee bodies (SAILR goto premise).
/// Option-gated (`almostregion`), report-only.
pub struct ActionAlmostRegion {
    base: ActionBase,
}

impl ActionAlmostRegion {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionAlmostRegion { base: ActionBase::new(0, "almostregion", g) })
    }
}

impl Action for ActionAlmostRegion {
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
        Some(Box::new(ActionAlmostRegion { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option almostregion off|on|report`, carried on the
        // Architecture boundary.
        let mode = data.get_arch().almost_region;
        if mode == ALMOSTREGION_OFF {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        if data.bblocks_get_size() > MAX_BLOCKS {
            return 0; // cost ceiling; see MAX_BLOCKS
        }

        let edges = virtualized_edges(data);
        if edges.is_empty() || edges.len() > MAX_PROBES {
            return 0;
        }

        let found = match probe_all(data, &edges) {
            Ok(v) => v,
            // A scratch-graph or dominator failure is a decline, never a hard
            // error: this pass observes and must not be able to fail a decompile.
            Err(_) => return 0,
        };
        if found.is_empty() {
            return 0;
        }

        let n = found.len();
        data.warning_header(&format!(
            "almostregion: {n} latent single-entry region(s) exposed by deleting one \
             virtualized edge - candidate inlined code"
        ));
        if mode >= ALMOSTREGION_REPORT {
            for r in &found {
                let addr = r.head.clone();
                data.warning(
                    &format!(
                        "almostregion: latent region head 0x{:x} exit 0x{:x}, {} \
                         blocks [{}], exposed by deleting the goto 0x{:x} - 0x{:x}",
                        r.head.get_offset(),
                        r.exit.get_offset(),
                        r.blocks(),
                        r.members
                            .iter()
                            .map(|a| format!("0x{:x}", a.get_offset()))
                            .collect::<Vec<_>>()
                            .join(" "),
                        r.goto_src.get_offset(),
                        r.goto_tgt.get_offset()
                    ),
                    &addr,
                );
            }
        }
        0 // report-only: never claim a change
    }
}

/// Every virtualized edge in the structured tree, as `(source, target)` pairs of
/// **`bblocks`** ids, skipping gotos the printer does not render.
///
/// The structurer records virtualization on `sblocks` only, so each side is mapped
/// down to the basic block it stands for: the target via `get_front_leaf`, the
/// source via `get_exit_leaf` (through the condition arm for a `BlockIf`).
fn virtualized_edges(data: &Funcdata) -> Vec<(BlockId, BlockId)> {
    let mut out: Vec<(BlockId, BlockId)> = Vec::new();
    let root = data.sblocks_root();
    for n in collect_nodes(data, root) {
        let g = data.sblocks_ref();
        let blk = g.block(n);
        let is_goto_node = blk.get_type() == BlockType::Goto;
        let (src_s, tgt_s) = match blk.get_type() {
            BlockType::Goto => {
                if blk.get_goto_type() != block_flags::f_goto_goto {
                    continue;
                }
                match blk.get_goto_target() {
                    Some(t) => (n, t),
                    None => continue,
                }
            }
            BlockType::If => {
                // `if (c) goto L;` - the commonest rendering of a virtualized edge.
                if blk.get_if_goto_type() != block_flags::f_goto_goto {
                    continue;
                }
                match blk.get_if_goto_target() {
                    // The goto leaves the condition block, which is child 0.
                    Some(t) if blk.get_size() > 0 => (blk.get_block(0), t),
                    _ => continue,
                }
            }
            _ => continue,
        };
        // `goto_prints` answers "does this render as a goto rather than a
        // fall-through", but it hard-returns false for every non-Goto node, so it
        // must only gate BlockGoto. Applying it to a BlockIf drops every
        // `if (c) goto L;` - which is the commonest virtualized-edge rendering.
        if is_goto_node && !g.goto_prints(n) {
            continue; // invisible goto (fall-through); not a virtualized edge
        }
        let src = match g.get_exit_leaf(src_s) {
            Some(b) => b,
            None => continue,
        };
        let tgt = match g.get_front_leaf(tgt_s) {
            Some(b) => b,
            None => continue,
        };
        let (src_bb, tgt_bb) = (g.block(src).get_copy(), g.block(tgt).get_copy());
        if let (Some(s), Some(t)) = (src_bb, tgt_bb) {
            if s != t && !out.contains(&(s, t)) {
                out.push((s, t));
            }
        }
    }
    out
}

/// The scratch region graph over `bblocks`: a node per basic block, every CFG
/// out-edge, a synthetic head into every zero-in-degree block, and a synthetic
/// sink out of every zero-out-degree block (so the post-dominator tree has a root).
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

/// Probe every virtualized edge and collect the latent regions.
fn probe_all(
    data: &Funcdata,
    edges: &[(BlockId, BlockId)],
) -> KunaResult<Vec<AlmostRegion>> {
    let sc = build_scratch(data);
    let mut block_of: BTreeMap<KunaNodeId, BlockId> = BTreeMap::new();
    for (&bl, &nid) in sc.node_of.iter() {
        block_of.insert(nid, bl);
    }

    let mut out: Vec<AlmostRegion> = Vec::new();
    for &(src, tgt) in edges {
        let (Some(&sn), Some(&tn)) = (sc.node_of.get(&src), sc.node_of.get(&tgt)) else {
            continue;
        };
        // G - e
        let mut cut = sc.graph.clone();
        cut.remove_edge(sn, tn);

        let mut doms_cut = KunaIncrementalDominators::new(&sc.pool, &cut, sc.head, false)?;
        let mut pdoms_cut = KunaIncrementalDominators::new(&sc.pool, &cut, sc.sink, true)?;
        let mut doms_full =
            KunaIncrementalDominators::new(&sc.pool, &sc.graph, sc.head, false)?;

        // Head candidates: the goto target first (the shape where a callee body's
        // head is reached by both fall-through and the virtualized edge), then the
        // dominator chain above the goto source, bounded.
        // Head candidates, in the order the three shapes occur:
        //   1. the goto target itself - a body whose head is reached both by
        //      fall-through and by the virtualized edge;
        //   2. the dominator chain above the *target* - the edge enters the body
        //      below its head, so the head is an ancestor of the target (this is
        //      the cross-jump / jump-threaded-into-the-middle shape);
        //   3. the dominator chain above the *source* - the edge leaves the body,
        //      giving it a second exit.
        let mut heads: Vec<KunaNodeId> = vec![tn];
        for seed in [tn, sn] {
            let mut climb = doms_cut.idom(seed);
            for _ in 0..MAX_HEAD_CLIMB {
                match climb {
                    Some(h) if h != sc.head && h != sc.sink => {
                        if !heads.contains(&h) {
                            heads.push(h);
                        }
                        climb = doms_cut.idom(h);
                    }
                    _ => break,
                }
            }
        }

        for h in heads {
            if h == sc.head || h == sc.sink {
                continue;
            }
            if let Some(r) = maximal_region(
                &sc,
                &cut,
                h,
                &mut doms_cut,
                &mut pdoms_cut,
                &mut doms_full,
                &block_of,
                data,
                src,
                tgt,
                sn,
                tn,
            )? {
                if !out.iter().any(|o| o.head == r.head && o.exit == r.exit) {
                    out.push(r);
                }
                break; // one region per virtualized edge
            }
        }
    }
    Ok(out)
}

/// Climb the post-dominator chain from `h` looking for the maximal `x` such that
/// `(h, x)` is a region in `cut` but **not** in the full graph.
#[allow(clippy::too_many_arguments)]
fn maximal_region(
    sc: &Scratch,
    cut: &KunaRegionGraph,
    h: KunaNodeId,
    doms_cut: &mut KunaIncrementalDominators,
    pdoms_cut: &mut KunaIncrementalDominators,
    doms_full: &mut KunaIncrementalDominators,
    block_of: &BTreeMap<KunaNodeId, BlockId>,
    data: &Funcdata,
    src: BlockId,
    tgt: BlockId,
    sn: KunaNodeId,
    tn: KunaNodeId,
) -> KunaResult<Option<AlmostRegion>> {
    let mut pd = pdoms_cut.idom(h);
    while let Some(x) = pd {
        // A region in `G - e` that is *not* a region in `G`: the deleted edge is
        // exactly what was breaking it.
        if x != h
            && x != sc.sink
            && kuna_check_region(&sc.pool, cut, h, x, doms_cut)?
            && !kuna_check_region(&sc.pool, &sc.graph, h, x, doms_full)?
        {
            let mut frontier = KunaNodeSet::new();
            frontier.insert(sc.pool.key(x));
            let mut slice = KunaRegionGraph::new();
            kuna_subgraph_between_nodes(&sc.pool, cut, h, &frontier, false, &mut slice)?;
            let mut members: Vec<KunaNodeId> = Vec::new();
            slice.get_nodes(&mut members);
            let count = members.len();
            // Incidence: the deleted edge must be the one that *broke* this region -
            // entering the body below the head, or leaving it - not a distant edge
            // whose removal happened to perturb dominance. Without this the pass
            // reports regions that have nothing to do with the goto.
            let incident = members.contains(&sn) || members.contains(&tn);
            if incident && (MIN_REGION_BLOCKS..=MAX_REGION_BLOCKS).contains(&count) {
                let head_bl = block_of.get(&h).copied();
                let exit_bl = block_of.get(&x).copied();
                if let (Some(hb), Some(xb)) = (head_bl, exit_bl) {
                    let mut addrs: Vec<Address> = members
                        .iter()
                        .filter_map(|m| block_of.get(m))
                        .map(|&b| data.bblocks_block_start(b))
                        .collect();
                    addrs.sort_by_key(|a| a.get_offset());
                    return Ok(Some(AlmostRegion {
                        head: data.bblocks_block_start(hb),
                        exit: data.bblocks_block_start(xb),
                        goto_src: data.bblocks_block_start(src),
                        goto_tgt: data.bblocks_block_start(tgt),
                        members: addrs,
                    }));
                }
            }
        }
        if !doms_cut.dominates(h, x) {
            break;
        }
        if pdoms_cut.idom(x) == Some(x) {
            break;
        }
        pd = pdoms_cut.idom(x);
    }
    Ok(None)
}

/// Is `[start, end)` a single-entry region of `g`?
///
/// Transcribed from the private `KunaRegionIdentifier::check_region`
/// (`p7_regions/kuna_regionid.rs:1504`, angr RI `_check_region`) so this pass can
/// run the predicate over a scratch graph it owns and may mutate edge-by-edge.
/// `kuna_regionid.rs` is a line-faithful port pinned by `docs/spec/07-regions.md`
/// and is deliberately not modified to expose it.
pub fn kuna_check_region(
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

/// Pre-order walk of the structured tree.
fn collect_nodes(data: &Funcdata, root: BlockId) -> Vec<BlockId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        out.push(id);
        let g = data.sblocks_ref();
        let n = g.block(id).get_size();
        for i in 0..n {
            stack.push(g.block(id).get_block(i));
        }
    }
    out
}

//===========================================================================
// The option
//===========================================================================

/// The `almostregion off|on|report` ArchOption (SAILR goto premise, report-only).
pub struct OptionAlmostRegion;

impl OptionAlmostRegion {
    /// The option name.
    pub const NAME: &'static str = "almostregion";

    /// Parse `off`/`on`/`report` into the reporting level (0 = off) plus a
    /// confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((
                ALMOSTREGION_OFF,
                "Latent-region (inline-candidate) reporting turned off".to_string(),
            )),
            // Empty parameter reads as `on`, matching `options::on_or_off`.
            "on" | "" => Ok((
                ALMOSTREGION_ON,
                "Latent-region (inline-candidate) reporting turned on (per-function \
                 count only)"
                    .to_string(),
            )),
            "report" => Ok((
                ALMOSTREGION_REPORT,
                "Latent-region (inline-candidate) reporting turned on, with one line \
                 per candidate region"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::lowlevel(
                "almostregion: must be off, on, or report",
            )),
        }
    }
}
