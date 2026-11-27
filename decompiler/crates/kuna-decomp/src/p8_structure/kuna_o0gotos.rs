//! (kuna `gotoreduce auto`) Keep the `goto` statements an unoptimized
//! function was written with.
//!
//! An unoptimizing gcc compiles a `goto L;` that is the whole arm of an `if`
//! into a `nop` the test jumps to, kept so a debugger can stop on the
//! statement's line, followed by a jump to `L` or falling into it, and lays
//! these out right before `L`:
//!
//! ```text
//!       jne  G1            ; if (g1(a) != 0) goto out;
//!       ...
//!       jne  G2            ; if (g2(a) != 0) goto out;
//!       movl $0, r         ; r = 0;
//!       jmp  OUT
//! G1:   nop
//!       jmp  OUT
//! G2:   nop
//! OUT:  ...                ; out: free(p); return r;
//! ```
//!
//! A bare `break;`, `continue;` or void `return;` compiles the same way, so a
//! jump of this form is taken for a `goto` only when nothing else explains it:
//! it does not come from inside the loop its target follows (the loop's
//! bottom test or back jump is laid out right before the target; when the
//! loop's last statement is `if (c) { S; break; }`, gcc folds the back jump
//! into that test and lays `S` out after it), nor from inside a jump-table
//! `switch` whose cases end at the target, and the target is not a loop's
//! head or test.
//!
//! [`record`] reads the raw graph at the first heritage pass of a function
//! that [`crate::p3_dataflow::kuna_unoptimized`] calls unoptimized and sets
//! [`GOTO_BIT`] in the label of each such edge; a removed block's in-edges
//! keep their labels when they are re-pointed at its successor.  After
//! structuring, `gotoreduce`'s passes leave alone every goto to a target one
//! of these edges reaches ([`keeps`]): the source jumps there with `goto`,
//! and a copy of the tail in each `if` is a graph the source does not have.
//! A `{ S; goto L; }` is a plain jump, like the end of an `if` arm, and clang
//! writes no `nop`, so a target reached only by those is duplicated as under
//! `on`.

use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_gotoreduce::GotoReduceMode;

/// Edge-label bit of a jump the record takes for a `goto`; the ported edge
/// flags use bits 0..9, `loopcontinue auto` bits 10..15 and `returndup auto`
/// bits 16..30.
pub(crate) const GOTO_BIT: uint4 = 0x8000_0000;
/// The widest run of instructions without p-code a jump may land on.
const MAX_GAP: u64 = 8;
/// The most blocks of statement markers walked back over in front of a target.
const MAX_MARKERS: usize = 16;

/// What the first heritage pass recorded.
#[derive(Debug, Clone, Default)]
pub struct Record {
    /// Edges labelled as `goto` statements.
    pub labels: u32,
}

/// A jump into block `t`: out-edge `slot` of `p`, whose branch at `from`
/// lands on `land`; `marker` when it lands in front of its block (through
/// the jump-only block `via`, if any), otherwise a plain jump from a block
/// with statements.
struct Jump {
    p: BlockId,
    slot: int4,
    from: u64,
    land: u64,
    t: BlockId,
    via: Option<BlockId>,
    marker: bool,
}

/// Read and label the `goto` statements; once, at the first heritage pass.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0gotos.is_some() {
        return;
    }
    if fd.get_arch().goto_reduce != GotoReduceMode::Auto {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let layout = Layout::new(fd);
    let jumps = jumps(fd);
    let mut targets: Vec<BlockId> = jumps.iter().filter(|j| j.marker).map(|j| j.t).collect();
    targets.sort_unstable();
    targets.dedup();
    let mut edges: Vec<(BlockId, int4)> = Vec::new();
    for t in targets {
        let Some(st) = head(fd, t) else { continue };
        if continue_target(fd, t, st) {
            continue;
        }
        let lp = exited_loop(fd, &layout, &jumps, t, st);
        let sw = switch_start(fd, &layout, &jumps, t);
        for j in jumps.iter().filter(|j| j.marker && j.t == t && is_goto(j.from, st, lp, sw)) {
            edges.push((j.p, j.slot));
            if let Some(m) = j.via {
                let b = fd.bblocks_ref().block(m);
                edges.extend((0..b.size_in()).map(|k| (b.get_in(k), b.get_in_rev_index(k))));
            }
        }
    }
    edges.sort_unstable();
    edges.dedup();
    for &(p, slot) in &edges {
        fd.bblocks_mut().set_out_edge_flag(p, slot, GOTO_BIT);
    }
    fd.kuna_o0gotos = Some(Box::new(Record { labels: edges.len() as u32 }));
}

/// Whether `gotoreduce auto` leaves the gotos to basic block `b` alone: in a
/// recorded function, a `goto` statement jumps to `b`.
pub(crate) fn keeps(fd: &Funcdata, b: BlockId) -> bool {
    if fd.kuna_o0gotos.is_none() {
        return false;
    }
    let blk = fd.bblocks_ref().block(b);
    (0..blk.size_in()).any(|i| blk.get_in_label(i) & GOTO_BIT != 0)
}

/// A jump from `from` into a block starting at `st` is a `goto` unless it
/// leaves the loop `[top, bottom]` that the block follows, or the `switch`
/// whose dispatch starts at `sw`.
fn is_goto(from: u64, st: u64, lp: Option<(u64, u64)>, sw: Option<u64>) -> bool {
    !lp.is_some_and(|(top, bottom)| (top..=bottom).contains(&from)) && !sw.is_some_and(|lo| (lo..st).contains(&from))
}

/// The blocks by address, and the case blocks of each jump table.
struct Layout {
    heads: Vec<(u64, BlockId)>,
    tails: Vec<(u64, BlockId)>,
    cases: Vec<(BlockId, u64)>,
}

impl Layout {
    fn new(fd: &Funcdata) -> Self {
        let blocks: Vec<BlockId> = (0..fd.bblocks_get_size()).map(|i| fd.bblocks_get_block(i)).collect();
        let mut heads: Vec<(u64, BlockId)> = blocks.iter().filter_map(|&b| head(fd, b).map(|a| (a, b))).collect();
        heads.sort_unstable_by_key(|x| x.0);
        let mut tails: Vec<(u64, BlockId)> = blocks.iter().filter_map(|&b| tail_addr(fd, b).map(|a| (a, b))).collect();
        tails.sort_unstable_by_key(|x| x.0);
        let mut cases = Vec::new();
        for &b in &blocks {
            if tail_code(fd, b) != Some(OpCode::CPUI_BRANCHIND) {
                continue;
            }
            let blk = fd.bblocks_ref().block(b);
            let lo = (0..blk.size_in()).filter_map(|k| head(fd, blk.get_in(k))).chain(head(fd, b)).min().unwrap_or(u64::MAX);
            cases.extend((0..blk.size_out()).map(|k| (blk.get_out(k), lo)));
        }
        Layout { heads, tails, cases }
    }

    /// The block whose last instruction is the last one before `a`.
    fn before(&self, a: u64) -> Option<BlockId> {
        let i = self.tails.partition_point(|x| x.0 < a);
        (i > 0).then(|| self.tails[i - 1].1)
    }

    /// The block that starts first after `a`.
    fn after(&self, a: u64) -> Option<BlockId> {
        let i = self.heads.partition_point(|x| x.0 <= a);
        self.heads.get(i).map(|x| x.1)
    }

    /// Where the dispatch of the jump table that has case `b` starts.
    fn case_start(&self, b: BlockId) -> Option<u64> {
        self.cases.iter().find(|c| c.0 == b).map(|c| c.1)
    }
}

/// Every jump that lands on a run of instructions without p-code in front
/// of its block, and every plain jump at the end of a block with statements.
fn jumps(fd: &Funcdata) -> Vec<Jump> {
    let mut out = Vec::new();
    for i in 0..fd.bblocks_get_size() {
        let p = fd.bblocks_get_block(i);
        let Some(op) = fd.bb_op_tail(p).and_then(|o| fd.obank().get(o)) else { continue };
        let blk = fd.bblocks_ref().block(p);
        let slot = match op.code() {
            OpCode::CPUI_CBRANCH if blk.size_out() == 2 => {
                if op.is_boolean_flip() {
                    0
                } else {
                    1
                }
            }
            OpCode::CPUI_BRANCH if blk.size_out() == 1 => 0,
            _ => continue,
        };
        let from = op.get_addr().get_offset();
        let plain = op.code() == OpCode::CPUI_BRANCH && !jump_only(fd, p);
        let Some(land) = branch_dest(fd, p) else { continue };
        let d = blk.get_out(slot);
        let Some(s) = head(fd, d) else { continue };
        if land < s && s - land <= MAX_GAP {
            let (t, via) = if jump_only(fd, d) { (fd.bblocks_ref().block(d).get_out(0), Some(d)) } else { (d, None) };
            out.push(Jump { p, slot, from, land, t, via, marker: true });
        } else if plain && land == s {
            out.push(Jump { p, slot, from, land, t: d, via: None, marker: false });
        }
    }
    out
}

/// Whether `t` is where a loop goes on: a backward branch enters it, or the
/// first branch it runs into goes backward.
fn continue_target(fd: &Funcdata, t: BlockId, st: u64) -> bool {
    let tb = fd.bblocks_ref().block(t);
    if (0..tb.size_in()).map(|k| tb.get_in(k)).any(|q| backward(fd, q) && branch_dest(fd, q).is_some_and(|d| d <= st)) {
        return true;
    }
    let mut cur = t;
    for _ in 0..3 {
        if branch_dest(fd, cur).is_some() {
            return backward(fd, cur);
        }
        let cb = fd.bblocks_ref().block(cur);
        if cb.size_out() != 1 {
            return false;
        }
        cur = cb.get_out(0);
    }
    false
}

/// The address range of the loop that `t` follows: the block laid out right
/// before `t` and its markers ends in the loop's backward branch, or is the
/// arm of the loop's last `if` that gcc laid out after the folded test.
fn exited_loop(fd: &Funcdata, layout: &Layout, jumps: &[Jump], t: BlockId, st: u64) -> Option<(u64, u64)> {
    let mut g = jumps.iter().filter(|j| j.marker && j.t == t && j.via.is_none()).map(|j| j.land).fold(st, u64::min);
    for _ in 0..MAX_MARKERS {
        let b = layout.before(g)?;
        if jump_only(fd, b) && fd.bblocks_ref().block(b).get_out(0) == t {
            let hb = head(fd, b)?;
            g = jumps.iter().filter(|j| j.via == Some(b)).map(|j| j.land).fold(hb, u64::min);
            continue;
        }
        if backward(fd, b) {
            return Some((branch_dest(fd, b)?, tail_addr(fd, b)?));
        }
        let q = tail_arm(fd, b)?;
        return Some((branch_dest(fd, q)?, st));
    }
    None
}

/// The backward test in front of `b` when `b` is the arm of the loop's last
/// `if (c) { S; break; }`: entered only by that test's fall-through, and the
/// test is no `for` or `while` test (those are reached by the loop's entry
/// jump from before the loop, and what follows them is after the loop).
fn tail_arm(fd: &Funcdata, b: BlockId) -> Option<BlockId> {
    let blk = fd.bblocks_ref().block(b);
    if blk.size_in() != 1 {
        return None;
    }
    let q = blk.get_in(0);
    let qb = fd.bblocks_ref().block(q);
    if tail_addr(fd, q)? >= head(fd, b)? || !backward(fd, q) || qb.size_out() != 2 {
        return None;
    }
    let top = branch_dest(fd, q)?;
    let entered = (0..qb.size_in()).any(|k| tail_addr(fd, qb.get_in(k)).is_some_and(|a| a < top));
    (!entered).then_some(q)
}

/// Where the dispatch starts of a jump-table `switch` whose cases end at
/// `t`: a case laid out right after a plain jump to `t`, or a bare `break;`
/// case that only jumps to `t`.
fn switch_start(fd: &Funcdata, layout: &Layout, jumps: &[Jump], t: BlockId) -> Option<u64> {
    let tb = fd.bblocks_ref().block(t);
    let arm_ends = jumps.iter().filter(|j| !j.marker && j.t == t).filter_map(|j| layout.after(j.from));
    let bare = (0..tb.size_in()).map(|k| tb.get_in(k)).filter(|&q| jump_only(fd, q));
    arm_ends.chain(bare).filter_map(|b| layout.case_start(b)).min()
}

fn head(fd: &Funcdata, b: BlockId) -> Option<u64> {
    fd.bb_op_head(b).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset())
}

fn tail_addr(fd: &Funcdata, b: BlockId) -> Option<u64> {
    fd.bb_op_tail(b).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset())
}

fn tail_code(fd: &Funcdata, b: BlockId) -> Option<OpCode> {
    fd.bb_op_tail(b).and_then(|o| fd.obank().get(o)).map(|o| o.code())
}

fn branch_dest(fd: &Funcdata, b: BlockId) -> Option<u64> {
    let op = fd.obank().get(fd.bb_op_tail(b)?)?;
    if !matches!(op.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
        return None;
    }
    let v = fd.vbank().get(op.get_in(0)?)?;
    (v.get_space().get_type() != spacetype::IPTR_CONSTANT).then(|| v.get_offset())
}

fn backward(fd: &Funcdata, b: BlockId) -> bool {
    matches!((branch_dest(fd, b), tail_addr(fd, b)), (Some(d), Some(a)) if d <= a)
}

/// A block holding nothing but one unconditional jump (and the phis the
/// first heritage pass put there).
fn jump_only(fd: &Funcdata, b: BlockId) -> bool {
    let ops = fd.bb_ops(b);
    let mut codes = ops
        .iter()
        .filter_map(|&o| fd.obank().get(o).map(|op| op.code()))
        .filter(|c| !matches!(c, OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT));
    codes.next() == Some(OpCode::CPUI_BRANCH) && codes.next().is_none() && fd.bblocks_ref().block(b).size_out() == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::edge_flags;

    #[test]
    fn a_jump_from_inside_the_loop_or_switch_its_target_follows_is_no_goto() {
        let lp = Some((0x100, 0x180));
        assert!(!is_goto(0x140, 0x190, lp, None), "a break out of the loop");
        assert!(is_goto(0x0f0, 0x190, lp, None), "a jump from before the loop");
        assert!(!is_goto(0x1a0, 0x200, None, Some(0x180)), "a break out of the switch");
        assert!(is_goto(0x170, 0x200, None, Some(0x180)), "a jump from before the dispatch");
        assert!(is_goto(0x140, 0x190, None, None));
    }

    #[test]
    fn the_goto_bit_is_free_of_the_other_records_and_the_ported_flags() {
        let ported = edge_flags::f_goto_edge
            | edge_flags::f_loop_edge
            | edge_flags::f_defaultswitch_edge
            | edge_flags::f_irreducible
            | edge_flags::f_tree_edge
            | edge_flags::f_forward_edge
            | edge_flags::f_cross_edge
            | edge_flags::f_back_edge
            | edge_flags::f_loop_exit_edge
            | edge_flags::f_immed_copy;
        assert_eq!(GOTO_BIT & ported, 0);
        assert_eq!(GOTO_BIT & (0x3f << 10), 0, "loopcontinue auto ids");
        assert_eq!(GOTO_BIT & (0x7fff << 16), 0, "returndup auto tags");
    }

    #[test]
    fn gotoreduce_auto_runs_both_passes() {
        let opt = crate::p8_structure::kuna_gotoreduce::OptionGotoReduce;
        let (mode, _) = opt.apply("auto").unwrap();
        assert_eq!(mode, GotoReduceMode::Auto);
        assert!(mode.return_tails() && mode.call_tails());
        assert_eq!(mode.as_str(), "auto");
        let (on, _) = opt.apply("on").unwrap();
        assert!(on.return_tails() && on.call_tails());
        let (ret, _) = opt.apply("return").unwrap();
        assert!(ret.return_tails() && !ret.call_tails());
        assert!(opt.apply("sometimes").is_err());
    }
}
