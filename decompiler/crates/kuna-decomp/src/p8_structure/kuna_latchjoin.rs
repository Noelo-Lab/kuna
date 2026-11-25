//! Join a loop latch the compiler duplicated into each path that reaches it
//! (`option dedupitetail latch`, the default).
//!
//! An optimizing compiler copies a loop's latch (the step and the test that jumps
//! back) into every path through the body that reaches it: tail duplication, jump
//! threading through the latch, rotation of a loop whose body ends in an `if`.  The
//! loop header then has two back edges whose source blocks end in the same
//! operations, and the structurer prints two loops, one directly inside the other,
//! where the source has one loop with an `if` in it.
//!
//! [`ActionLatchJoin`] runs on the p-code graph next to the upstream
//! `ActionNodeJoin`, so every later pass sees one latch.  For a header with exactly
//! two back edges, from blocks `A` and `B` that end in a conditional branch to the
//! same two targets, it pairs their ops from the branch backwards and keeps the
//! longest run that is operation-for-operation identical: same opcode, equal
//! constants, the same output storage where storage matters, and inputs that are
//! the same value, the outputs of corresponding ops of the run, or values from
//! before the run, which meet in a new MULTIEQUAL.  A test of the opposite sense
//! with its targets swapped is the same test; up to [`MAX_SKIPS`] side-effect-free
//! ops that only one copy has are stepped over and stay in that copy; two loads
//! from different addresses stay in their copies.  The run must contain the op that
//! computes the branch condition.  Both copies then fall into one new block `J`
//! that holds `A`'s run; `B`'s run is deleted and the targets' MULTIEQUAL inputs
//! from `A` and `B` become the one from `J`.
//!
//! Calls, stores, user ops, unconditional and self-loop latches, a value of the
//! run used anywhere but the run and the targets' MULTIEQUALs, and unoptimized
//! functions are refused.  The join is exact: each path runs the same operations
//! in the same order.  The drive decompiles a function again without the join
//! when the joined loop prints a goto ([`joined_gotos`]).

use std::collections::HashMap;

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Joins performed per apply, a bound on the restart loop.
const MAX_JOINS: usize = 16;
/// Operations one copy has and the other lacks that the backward walk steps over.
const MAX_SKIPS: usize = 2;
/// Functions larger than this many ops are not decompiled a second time to check
/// a joined loop's gotos: the second run costs as much as the first, and on the
/// decbench corpus every function where the unjoined result printed fewer gotos
/// had under 500 ops.
const MAX_RECHECK_OPS: usize = 1000;

/// An operation that reads and writes no memory and has no other effect, so it
/// can run before operations it followed when it reads none of their outputs: it
/// writes a register or a temporary and reads only values in SSA form.
fn pure(data: &Funcdata, op: OpId) -> bool {
    let p = data.obank().get(op).unwrap();
    let reg_or_temp = |v: VarnodeId| {
        let vn = data.vbank().get(v).unwrap();
        let space = vn.get_space();
        !storage_matters(data, v)
            && (space.get_type() == spacetype::IPTR_INTERNAL
                || (space.get_type() == spacetype::IPTR_PROCESSOR && space.get_name() == "register"))
    };
    let ssa = |v: VarnodeId| {
        let vn = data.vbank().get(v).unwrap();
        vn.is_constant() || !vn.is_free()
    };
    !matches!(
        p.code(),
        OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_STORE
            | OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_NEW
            | OpCode::CPUI_CPOOLREF
            | OpCode::CPUI_SEGMENTOP
    ) && p.get_out().is_some_and(reg_or_temp)
        && (0..p.num_input()).all(|s| p.get_in(s).is_some_and(ssa))
}

/// \brief Join identical copies of a loop latch into one block.
pub struct ActionLatchJoin {
    base: ActionBase,
}

impl ActionLatchJoin {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLatchJoin { base: ActionBase::new(0, "latchjoin", g) })
    }
}

impl Action for ActionLatchJoin {
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
        Some(Box::new(ActionLatchJoin { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().latch_join
            || data.kuna_latch_join_withdrawn
            || crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data)
        {
            return 0;
        }
        let mut joins = 0;
        while joins < MAX_JOINS {
            let Some(plan) = find_plan(data) else { break };
            execute(data, &plan);
            joins += 1;
        }
        data.kuna_latch_joins += joins as u32;
        self.base_mut().count += joins as i32;
        0
    }
}

/// The gotos the finished function prints when a latch was joined, the function
/// has at most [`MAX_RECHECK_OPS`] ops and a goto jumps within a loop that holds a
/// joined latch, else `None`.  The drive then
/// decompiles the function again without the join and keeps the result with
/// fewer gotos.
pub fn joined_gotos(fd: &Funcdata) -> Option<i32> {
    if fd.kuna_latch_joins == 0 || fd.obank().iter_alive().count() > MAX_RECHECK_OPS || gotos_near_joins(fd) == 0 {
        return None;
    }
    Some(crate::p2_lift::kuna_loweredswitchkeep::printed_gotos(fd))
}

/// Gotos the finished structure prints from one place to another inside a loop
/// that holds a joined latch; every printed goto when no such loop is found.
fn gotos_near_joins(fd: &Funcdata) -> i32 {
    use crate::block::{block_flags, BlockKind, BlockType};
    let g = fd.sblocks_ref();
    let Some(root) = g.root else { return 0 };
    let mut nodes = vec![root];
    let mut i = 0;
    while i < nodes.len() {
        nodes.extend(g.block(nodes[i]).get_list().iter().copied());
        i += 1;
    }
    let mut inside: std::collections::BTreeSet<BlockId> = std::collections::BTreeSet::new();
    for &n in &nodes {
        let b = g.block(n);
        if b.get_type() != BlockType::Copy || !b.get_copy().is_some_and(|c| fd.kuna_latch_blocks.contains(&c)) {
            continue;
        }
        let mut up = b.get_parent();
        while let Some(p) = up {
            if matches!(g.block(p).get_type(), BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop) {
                let mut sub = vec![p];
                while let Some(s) = sub.pop() {
                    if inside.insert(s) {
                        sub.extend(g.block(s).get_list().iter().copied());
                    }
                }
                break;
            }
            up = g.block(p).get_parent();
        }
    }
    if inside.is_empty() {
        return crate::p2_lift::kuna_loweredswitchkeep::printed_gotos(fd);
    }
    let rw = crate::p8_structure::kuna_gotonext::goto_rewrites(
        g,
        root,
        fd.get_arch().recover_loop_break,
        &crate::p8_structure::kuna_switchmerge::SwitchPlans::default(),
    );
    let near = |n: BlockId, t: Option<BlockId>| inside.contains(&n) && t.and_then(|t| g.get_front_leaf(t)).is_some_and(|l| inside.contains(&l));
    let mut count = 0;
    for &n in &nodes {
        let b = g.block(n);
        match b.get_type() {
            BlockType::Goto => {
                if b.get_goto_type() == block_flags::f_goto_goto
                    && !rw.silent.contains(&n)
                    && !rw.breaks.contains(&n)
                    && near(n, b.get_goto_target())
                {
                    count += 1;
                }
            }
            BlockType::MultiGoto => {
                if let BlockKind::MultiGoto { gotoedges, .. } = b.kind() {
                    count += gotoedges.iter().filter(|&&t| near(n, Some(t))).count() as i32;
                }
            }
            BlockType::Switch => {
                count += b
                    .switch_caseblocks()
                    .iter()
                    .filter(|c| c.gototype == block_flags::f_goto_goto && !rw.case_breaks.contains(&(n, c.block)) && near(n, Some(c.block)))
                    .count() as i32;
            }
            _ => {
                if b.get_if_goto_type() == block_flags::f_goto_goto
                    && !rw.breaks.contains(&n)
                    && b.get_if_goto_target().is_some()
                    && near(n, b.get_if_goto_target())
                {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Whether a decompile without the join is structured and prints fewer gotos
/// than the `with` the joined one printed.
pub fn fewer_gotos(plain: &Funcdata, with: i32) -> bool {
    plain.sblocks_get_size() > 0 && crate::p2_lift::kuna_loweredswitchkeep::printed_gotos(plain) < with
}

/// One accepted join: `ops_a[i]` and `ops_b[i]` are the corresponding ops of the
/// two latch copies, in execution order.
struct Plan {
    a: BlockId,
    b: BlockId,
    ops_a: Vec<OpId>,
    ops_b: Vec<OpId>,
}

fn bb_ops(data: &Funcdata, bl: BlockId) -> Vec<OpId> {
    let mut out = Vec::new();
    let mut op = data.bb_op_head(bl);
    while let Some(o) = op {
        out.push(o);
        op = data.bb_op_next(o);
    }
    out
}

/// The target the CBRANCH ending `bl` takes when its condition input is true.
fn true_target(data: &Funcdata, bl: BlockId, cbr: OpId) -> BlockId {
    let blk = data.bblocks_ref().block(bl);
    if data.obank().get(cbr).unwrap().is_boolean_flip() {
        blk.get_out(0)
    } else {
        blk.get_out(1)
    }
}

/// The first header whose two back edges come from latch copies that join.
fn find_plan(data: &Funcdata) -> Option<Plan> {
    let graph = data.bblocks_ref();
    let root = graph.root?;
    let n = graph.block(root).get_size();
    for i in 0..n {
        let h = data.bblocks_ref().block(root).get_block(i);
        let hb = data.bblocks_ref().block(h);
        if hb.size_in() < 3 {
            continue;
        }
        let mut latches: Vec<BlockId> = Vec::new();
        for j in 0..hb.size_in() {
            if hb.is_back_edge_in(j) && !hb.is_irreducible_in(j) {
                let l = hb.get_in(j);
                if l != h && !latches.contains(&l) {
                    latches.push(l);
                }
            }
        }
        // With a third back edge the joined loop still has two latches.
        if latches.len() != 2 {
            continue;
        }
        if let Ok(p) = match_latches(data, h, latches[0], latches[1]) {
            return Some(p);
        }
    }
    None
}

/// A join, or why the pair of latch copies is refused.
type Verdict = Result<Plan, String>;

fn same_storage(data: &Funcdata, x: VarnodeId, y: VarnodeId) -> bool {
    let a = data.vbank().get(x).unwrap();
    let b = data.vbank().get(y).unwrap();
    a.get_size() == b.get_size() && a.get_addr() == b.get_addr()
}

/// Storage whose identity matters: anything but a temporary or a register value.
fn storage_matters(data: &Funcdata, v: VarnodeId) -> bool {
    let vn = data.vbank().get(v).unwrap();
    vn.is_addr_tied() || vn.is_persist() || (vn.is_free() && !vn.is_constant())
}

/// The comparison that is true exactly when `code` is false, with its operands in
/// the same order (`false`) or swapped (`true`).
fn complement(code: OpCode) -> Option<(OpCode, bool)> {
    Some(match code {
        OpCode::CPUI_INT_EQUAL => (OpCode::CPUI_INT_NOTEQUAL, false),
        OpCode::CPUI_INT_NOTEQUAL => (OpCode::CPUI_INT_EQUAL, false),
        OpCode::CPUI_INT_LESS => (OpCode::CPUI_INT_LESSEQUAL, true),
        OpCode::CPUI_INT_LESSEQUAL => (OpCode::CPUI_INT_LESS, true),
        OpCode::CPUI_INT_SLESS => (OpCode::CPUI_INT_SLESSEQUAL, true),
        OpCode::CPUI_INT_SLESSEQUAL => (OpCode::CPUI_INT_SLESS, true),
        _ => return None,
    })
}

/// Shape check of one op pair: opcode (or its complement when `negated`), arity,
/// constants, output size and storage.  Returns the input slot mapping.
fn shape(data: &Funcdata, oa: OpId, ob: OpId, negated: bool) -> Result<Vec<i32>, String> {
    let pa = data.obank().get(oa).unwrap();
    let pb = data.obank().get(ob).unwrap();
    let (ca, cb) = (pa.code(), pb.code());
    match ca {
        OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => return Err("marker".into()),
        OpCode::CPUI_CALL
        | OpCode::CPUI_CALLIND
        | OpCode::CPUI_CALLOTHER
        | OpCode::CPUI_NEW
        | OpCode::CPUI_CPOOLREF
        | OpCode::CPUI_BRANCHIND
        | OpCode::CPUI_STORE
        | OpCode::CPUI_RETURN => return Err(format!("op {ca:?}")),
        _ => {}
    }
    let mut swap = false;
    if negated {
        match complement(ca) {
            Some((c, s)) if c == cb => swap = s,
            _ => return Err(format!("not complement {ca:?}/{cb:?}")),
        }
    } else if ca != cb {
        return Err(format!("opcode {ca:?}/{cb:?}"));
    }
    if pa.num_input() != pb.num_input() {
        return Err("arity".into());
    }
    let n = pa.num_input();
    let slots: Vec<i32> = (0..n).map(|s| if swap { n - 1 - s } else { s }).collect();
    let skip0 = matches!(ca, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH);
    for s in 0..n {
        if skip0 && s == 0 {
            continue;
        }
        let va = data.vbank().get(pa.get_in(s).unwrap()).unwrap();
        let vb = data.vbank().get(pb.get_in(slots[s as usize]).unwrap()).unwrap();
        if va.is_constant() || vb.is_constant() {
            if !(va.is_constant()
                && vb.is_constant()
                && va.get_offset() == vb.get_offset()
                && va.get_size() == vb.get_size())
            {
                return Err("constant".into());
            }
        }
    }
    match (pa.get_out(), pb.get_out()) {
        (None, None) => {}
        (Some(wa), Some(wb)) => {
            let sa = data.vbank().get(wa).unwrap().get_size();
            let sb = data.vbank().get(wb).unwrap().get_size();
            if sa != sb {
                return Err("out size".into());
            }
            if (storage_matters(data, wa) || storage_matters(data, wb)) && !same_storage(data, wa, wb) {
                return Err("out storage".into());
            }
        }
        _ => return Err("out".into()),
    }
    Ok(slots)
}

/// The identical run of the latch copies `a` and `b` of header `h`: ops paired by
/// shape from the branch backwards, then cut to start after any op one copy
/// computes from the run differently from the other; outputs may be read only by
/// the run and by the targets' MULTIEQUALs on the copy's own edge.
fn match_latches(data: &Funcdata, h: BlockId, a: BlockId, b: BlockId) -> Verdict {
    let g = data.bblocks_ref();
    let (ba, bb) = (g.block(a), g.block(b));
    if ba.size_out() != 2 || bb.size_out() != 2 {
        return Err(format!("outs {}/{}", ba.size_out(), bb.size_out()));
    }
    let targets = [ba.get_out(0), ba.get_out(1)];
    if targets[0] == targets[1] {
        return Err("same targets".into());
    }
    let tb = [bb.get_out(0), bb.get_out(1)];
    if !(tb.contains(&targets[0]) && tb.contains(&targets[1])) {
        return Err("targets differ".into());
    }
    if targets.iter().any(|&t| t == a || t == b) || !targets.contains(&h) {
        return Err("self target".into());
    }
    let ops_a = bb_ops(data, a);
    let ops_b = bb_ops(data, b);
    let (ta, tb) = match (ops_a.last(), ops_b.last()) {
        (Some(&x), Some(&y)) => (x, y),
        _ => return Err("empty".into()),
    };
    let (pa, pb) = (data.obank().get(ta).unwrap(), data.obank().get(tb).unwrap());
    if pa.code() != OpCode::CPUI_CBRANCH || pb.code() != OpCode::CPUI_CBRANCH {
        return Err("no cbranch".into());
    }
    let negated = true_target(data, a, ta) != true_target(data, b, tb);
    let cond = (pa.get_in(1).unwrap(), pb.get_in(1).unwrap(), negated);
    let try_shape = |oa: OpId, ob: OpId| -> Result<Vec<i32>, String> {
        let negated = cond.2
            && data.obank().get(oa).unwrap().get_out() == Some(cond.0)
            && data.obank().get(ob).unwrap().get_out() == Some(cond.1);
        shape(data, oa, ob, negated)
    };
    let mut ia = ops_a.len();
    let mut ib = ops_b.len();
    let mut pairs: Vec<(OpId, OpId, Vec<i32>)> = Vec::new();
    let mut skipped: Vec<(OpId, bool)> = Vec::new();
    let mut stop = String::from("start of block");
    while ia > 0 && ib > 0 {
        let (oa, ob) = (ops_a[ia - 1], ops_b[ib - 1]);
        match try_shape(oa, ob) {
            Ok(slots) => pairs.push((oa, ob, slots)),
            Err(e) => {
                if !pairs.is_empty() {
                    let room = MAX_SKIPS - skipped.len();
                    let step = |ops: &[OpId], i: usize, other: OpId, side_a: bool| -> Option<usize> {
                        (1..=room.min(i.saturating_sub(1))).find(|&k| {
                            (0..k).all(|j| pure(data, ops[i - 1 - j]))
                                && if side_a { try_shape(ops[i - 1 - k], other) } else { try_shape(other, ops[i - 1 - k]) }.is_ok()
                        })
                    };
                    if let Some(k) = step(&ops_a, ia, ob, true) {
                        skipped.extend((0..k).map(|j| (ops_a[ia - 1 - j], true)));
                        ia -= k;
                        continue;
                    }
                    if let Some(k) = step(&ops_b, ib, oa, false) {
                        skipped.extend((0..k).map(|j| (ops_b[ib - 1 - j], false)));
                        ib -= k;
                        continue;
                    }
                }
                stop = e;
                break;
            }
        }
        ia -= 1;
        ib -= 1;
    }
    pairs.reverse();

    if pairs.is_empty() {
        return Err(format!("no common tail: {stop}"));
    }
    let mut start = 0usize;
    'outer: loop {
        if start >= pairs.len() {
            return Err(format!("cut to nothing ({stop})"));
        }
        let run = &pairs[start..];
        let pos_a: HashMap<OpId, usize> = run.iter().enumerate().map(|(i, p)| (p.0, i)).collect();
        let pos_b: HashMap<OpId, usize> = run.iter().enumerate().map(|(i, p)| (p.1, i)).collect();
        let def_pos = |v: VarnodeId, pos: &HashMap<OpId, usize>| -> Option<usize> {
            data.vbank().get(v).unwrap().get_def().and_then(|d| pos.get(&d).copied())
        };
        let check_pair = |va: VarnodeId, vb: VarnodeId, negated_ok: bool| -> Result<(), Result<usize, String>> {
            if va == vb {
                return Ok(());
            }
            let (ja, jb) = (def_pos(va, &pos_a), def_pos(vb, &pos_b));
            if ja.is_some() || jb.is_some() {
                if ja == jb {
                    return Ok(());
                }
                return Err(Ok(ja.unwrap_or(0).max(jb.unwrap_or(0))));
            }
            if !negated_ok {
                return Err(Err("negated condition from outside".into()));
            }
            let (xa, xb) = (data.vbank().get(va).unwrap(), data.vbank().get(vb).unwrap());
            if xa.is_constant() && xb.is_constant() && xa.get_offset() == xb.get_offset() && xa.get_size() == xb.get_size() {
                return Ok(());
            }
            if xa.is_constant() || xb.is_constant() {
                return Err(Err("constant merge".into()));
            }
            if storage_matters(data, va) || storage_matters(data, vb) {
                if same_storage(data, va, vb) && xa.is_free() && xb.is_free() {
                    return Ok(());
                }
                return Err(Err("merge of tied storage".into()));
            }
            if xa.get_size() != xb.get_size() {
                return Err(Err("merge size".into()));
            }
            Ok(())
        };
        for (k, (oa, ob, slots)) in run.iter().enumerate() {
            let (pa, pb) = (data.obank().get(*oa).unwrap(), data.obank().get(*ob).unwrap());
            let skip0 = matches!(pa.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH);
            for s in 0..pa.num_input() {
                if skip0 && s == 0 {
                    continue;
                }
                let va = pa.get_in(s).unwrap();
                let vb = pb.get_in(slots[s as usize]).unwrap();
                // Loads from two different addresses stay in their copies: a
                // merged address hides which memory, a stack slot say, is read.
                if pa.code() == OpCode::CPUI_LOAD
                    && s == 1
                    && va != vb
                    && def_pos(va, &pos_a).is_none()
                    && def_pos(vb, &pos_b).is_none()
                {
                    start += k + 1;
                    continue 'outer;
                }
                let negated_ok = !(pa.code() == OpCode::CPUI_CBRANCH && cond.2);
                match check_pair(va, vb, negated_ok) {
                    Ok(()) => {}
                    Err(Ok(cut)) => {
                        start += cut + 1;
                        continue 'outer;
                    }
                    Err(Err(e)) => return Err(e),
                }
            }
        }
        // A stepped-over op now runs before the run ops it followed: it may not
        // read their outputs, and they may not read storage outside SSA form.
        if !skipped.is_empty()
            && run.iter().any(|(oa, ob, _)| {
                [*oa, *ob].iter().any(|&o| {
                    let p = data.obank().get(o).unwrap();
                    (0..p.num_input()).any(|s| {
                        let v = data.vbank().get(p.get_in(s).unwrap()).unwrap();
                        v.is_free() && !v.is_constant() && !v.is_annotation()
                    })
                })
            })
        {
            return Err("stepped over a read outside SSA".into());
        }
        for &(x, side_a) in &skipped {
            let pos = if side_a { &pos_a } else { &pos_b };
            let px = data.obank().get(x).unwrap();
            for s in 0..px.num_input() {
                if let Some(j) = def_pos(px.get_in(s).unwrap(), pos) {
                    start += j + 1;
                    continue 'outer;
                }
            }
        }
        for (oa, ob, _) in run.iter() {
            let complemented = data.obank().get(*oa).unwrap().code() != data.obank().get(*ob).unwrap().code();
            for (o, blk, pos) in [(*oa, a, &pos_a), (*ob, b, &pos_b)] {
                if let Some(w) = data.obank().get(o).unwrap().get_out() {
                    for d in data.vbank().get(w).unwrap().descend_iter() {
                        let dop = data.obank().get(d).unwrap();
                        if pos.contains_key(&d) {
                            continue;
                        }
                        let parent = dop.get_parent();
                        let ok = !complemented
                            && dop.code() == OpCode::CPUI_MULTIEQUAL
                            && parent.is_some_and(|p| targets.contains(&p))
                            && {
                                let p = parent.unwrap();
                                let pb = data.bblocks_ref().block(p);
                                let slot = pb.get_in_index(blk);
                                (0..dop.num_input()).all(|s| dop.get_in(s) != Some(w) || s == slot)
                            };
                        if !ok {
                            return Err("value used outside".into());
                        }
                    }
                }
            }
        }
        for &t in &targets {
            let tb = data.bblocks_ref().block(t);
            let (sa, sb) = (tb.get_in_index(a), tb.get_in_index(b));
            for op in bb_ops(data, t) {
                let p = data.obank().get(op).unwrap();
                if p.code() != OpCode::CPUI_MULTIEQUAL {
                    continue;
                }
                let (va, vb) = (p.get_in(sa).unwrap(), p.get_in(sb).unwrap());
                match check_pair(va, vb, true) {
                    Ok(()) => {}
                    Err(Ok(cut)) => {
                        start += cut + 1;
                        continue 'outer;
                    }
                    Err(Err(e)) => return Err(format!("phi: {e}")),
                }
            }
        }
        if def_pos(cond.0, &pos_a).is_none() {
            return Err(format!("condition not in run (run {}, {stop})", run.len()));
        }
        return Ok(Plan {
            a,
            b,
            ops_a: run.iter().map(|p| p.0).collect(),
            ops_b: run.iter().map(|p| p.1).collect(),
        });
    }
}

/// Route `A` and `B` into a new block `J` holding `A`'s run behind a MULTIEQUAL
/// for each value the copies differ in, delete `B`'s run, and give the targets'
/// MULTIEQUALs one input from `J` in place of the two.
fn execute(data: &mut Funcdata, plan: &Plan) {
    let (a, b) = (plan.a, plan.b);
    let targets = [data.bblocks_ref().block(a).get_out(0), data.bblocks_ref().block(a).get_out(1)];
    let pos_a: HashMap<OpId, usize> = plan.ops_a.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let in_run = |data: &Funcdata, v: VarnodeId, pos: &HashMap<OpId, usize>| -> bool {
        data.vbank().get(v).unwrap().get_def().is_some_and(|d| pos.contains_key(&d))
    };
    let mut merges: Vec<(VarnodeId, VarnodeId)> = Vec::new();
    let need = |data: &Funcdata, va: VarnodeId, vb: VarnodeId, merges: &mut Vec<(VarnodeId, VarnodeId)>| {
        if va == vb || in_run(data, va, &pos_a) {
            return;
        }
        let xa = data.vbank().get(va).unwrap();
        if xa.is_free() {
            return;
        }
        if !merges.contains(&(va, vb)) {
            merges.push((va, vb));
        }
    };
    let mut slot_maps: Vec<Vec<i32>> = Vec::new();
    for (&oa, &ob) in plan.ops_a.iter().zip(plan.ops_b.iter()) {
        let negated = {
            let pa = data.obank().get(oa).unwrap();
            let pb = data.obank().get(ob).unwrap();
            pa.code() != pb.code()
        };
        let slots = shape(data, oa, ob, negated).expect("latchjoin: shape changed");
        let pa = data.obank().get(oa).unwrap();
        let pb = data.obank().get(ob).unwrap();
        let skip0 = matches!(pa.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH);
        let pairs: Vec<(VarnodeId, VarnodeId)> = (0..pa.num_input())
            .filter(|&s| !(skip0 && s == 0))
            .map(|s| (pa.get_in(s).unwrap(), pb.get_in(slots[s as usize]).unwrap()))
            .collect();
        for (va, vb) in pairs {
            need(data, va, vb, &mut merges);
        }
        slot_maps.push(slots);
    }
    let mut target_pairs: Vec<(BlockId, i32, i32, Vec<(OpId, VarnodeId, VarnodeId)>)> = Vec::new();
    for &t in &targets {
        let tb = data.bblocks_ref().block(t);
        let (sa, sb) = (tb.get_in_index(a), tb.get_in_index(b));
        let mut phis = Vec::new();
        for op in bb_ops(data, t) {
            let p = data.obank().get(op).unwrap();
            if p.code() != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            let (va, vb) = (p.get_in(sa).unwrap(), p.get_in(sb).unwrap());
            phis.push((op, va, vb));
        }
        for &(_, va, vb) in &phis {
            need(data, va, vb, &mut merges);
        }
        target_pairs.push((t, sa, sb, phis));
    }

    let addr = data.obank().get(plan.ops_a[0]).unwrap().get_addr().clone();
    let (a0, b0) = (target_pairs[0].1, target_pairs[0].2);
    let (a1, b1) = (target_pairs[1].1, target_pairs[1].2);
    let j = data.node_join_create_block(a, b, targets[0], targets[1], a0 > b0, a1 > b1, &addr);
    data.kuna_latch_blocks.push(j);

    let mut merged: HashMap<(VarnodeId, VarnodeId), VarnodeId> = HashMap::new();
    for &(va, vb) in &merges {
        let multi = data.new_op(2, addr.clone());
        data.op_set_opcode(multi, crate::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL));
        let (size, saddr, keep_storage) = {
            let xa = data.vbank().get(va).unwrap();
            let xb = data.vbank().get(vb).unwrap();
            let same = xa.get_addr() == xb.get_addr() && xa.get_size() == xb.get_size();
            let internal = xa.get_space().get_type() == spacetype::IPTR_INTERNAL;
            (xa.get_size(), xa.get_addr().clone(), same && !internal)
        };
        let out = if keep_storage {
            data.new_varnode_out(size, &saddr, multi).expect("latchjoin: newVarnodeOut")
        } else {
            data.new_unique_out(size, multi).expect("latchjoin: newUniqueOut")
        };
        data.op_set_input(multi, va, 0).expect("latchjoin: opSetInput");
        data.op_set_input(multi, vb, 1).expect("latchjoin: opSetInput");
        data.op_insert_end(multi, j);
        merged.insert((va, vb), out);
    }
    let value = |va: VarnodeId, vb: VarnodeId| merged.get(&(va, vb)).copied().unwrap_or(va);

    for (i, (&oa, &ob)) in plan.ops_a.iter().zip(plan.ops_b.iter()).enumerate() {
        let slots = &slot_maps[i];
        let skip0 = matches!(data.obank().get(oa).unwrap().code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH);
        let n = data.obank().get(oa).unwrap().num_input();
        for s in 0..n {
            if skip0 && s == 0 {
                continue;
            }
            let va = data.obank().get(oa).unwrap().get_in(s).unwrap();
            let vb = data.obank().get(ob).unwrap().get_in(slots[s as usize]).unwrap();
            let v = value(va, vb);
            if v != va {
                data.op_set_input(oa, v, s).expect("latchjoin: opSetInput");
            }
        }
        data.op_uninsert(oa);
        data.op_insert_end(oa, j);
    }

    for (t, sa, sb, phis) in &target_pairs {
        let (lo, hi) = if sa > sb { (*sb, *sa) } else { (*sa, *sb) };
        for &(op, va, vb) in phis {
            let v = value(va, vb);
            data.op_remove_input(op, hi);
            data.op_set_input(op, v, lo).expect("latchjoin: opSetInput");
            if data.obank().get(op).unwrap().num_input() == 1 {
                data.op_uninsert(op);
                data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
                data.op_insert_begin(op, *t);
            }
        }
    }

    for &ob in plan.ops_b.iter().rev() {
        data.op_destroy(ob);
    }
}

#[cfg(test)]
#[path = "kuna_latchjoin/tests.rs"]
mod tests;
