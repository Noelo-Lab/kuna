//! Join a loop latch the compiler duplicated into each path that reaches it.
//!
//! An optimizing compiler copies a loop's latch (the step and the test that jumps
//! back) into every path through the body that reaches it: tail duplication, jump
//! threading through the latch, rotation of a loop whose body ends in an `if`.  The
//! loop header then has two back edges whose source blocks end in the same
//! operations, and the structurer prints two loops, one directly inside the other,
//! where the source has one loop with an `if` in it.
//!
//! [`ActionLatchJoin`] runs on the p-code graph, next to the upstream
//! `ActionNodeJoin`, so every later pass sees one latch.  For two back edges into
//! one header whose source blocks `A` and `B` leave to the same two targets (or
//! both only to the header), it pairs their ops from the end backwards and keeps
//! the longest run that is operation-for-operation identical: same opcode, same
//! constants, same output storage, and inputs that are either the same value, the
//! outputs of corresponding ops of the run, or values from before the run (which
//! meet in a new MULTIEQUAL).  The run must contain the branch and the op that
//! computes its condition; a condition tested with the opposite sense and the
//! targets swapped is the same test.  The two runs become one new block `J` that
//! both `A` and `B` now fall into; `J` keeps `A`'s ops, `B`'s copies are deleted,
//! and the header's and exit's MULTIEQUAL inputs from `A` and `B` collapse to the
//! one from `J`.
//!
//! The join refuses a run with a call, an INDIRECT or a user op, outputs that do
//! not name the same storage where storage matters, a value of the run used
//! anywhere but the run itself and the MULTIEQUALs of the two targets, and a
//! self-loop latch.  The join is exact: each path still runs the same operations
//! in the same order.  Unoptimized functions are skipped (they do not duplicate
//! latches).

use std::collections::HashMap;
use std::io::Write as _;

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Joins performed per apply, a bound on the restart loop.
const MAX_JOINS: usize = 16;

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
        if crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data) {
            return 0;
        }
        let dry = std::env::var_os("KUNA_LJ_DRY").is_some();
        let mut joins = 0;
        while joins < MAX_JOINS {
            let plan = match find_plan(data) {
                Some(p) => p,
                None => break,
            };
            if dry {
                break;
            }
            execute(data, &plan);
            joins += 1;
        }
        self.base_mut().count += joins as i32;
        0
    }
}

/// One accepted join: `ops_a[i]` and `ops_b[i]` are the corresponding ops of the
/// two latch copies, in execution order.
struct Plan {
    a: BlockId,
    b: BlockId,
    ops_a: Vec<OpId>,
    ops_b: Vec<OpId>,
    /// TEMP stats: run-input merges, stores in the run.
    run_merges: usize,
    stores: usize,
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
        if latches.len() < 2 {
            continue;
        }
        for x in 0..latches.len() {
            for y in x + 1..latches.len() {
                let r = match_latches(data, h, latches[x], latches[y]);
                dump(data, h, latches[x], latches[y], &r);
                if let Ok(p) = r {
                    return Some(p);
                }
            }
        }
    }
    None
}

/// Why a pair of latch copies is refused (the TEMP classification dump prints it).
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

fn match_latches(data: &Funcdata, h: BlockId, a: BlockId, b: BlockId) -> Verdict {
    let g = data.bblocks_ref();
    let (ba, bb) = (g.block(a), g.block(b));
    let nout = ba.size_out();
    if nout == 1 && std::env::var_os("KUNA_LJ_COND").is_some() {
        return Err("uncond latch (COND)".into());
    }
    if nout != bb.size_out() || !(1..=2).contains(&nout) {
        return Err(format!("outs {}/{}", nout, bb.size_out()));
    }
    let targets: Vec<BlockId> = (0..nout).map(|i| ba.get_out(i)).collect();
    if nout == 2 {
        if targets[0] == targets[1] {
            return Err("same targets".into());
        }
        let tb = [bb.get_out(0), bb.get_out(1)];
        if !(tb.contains(&targets[0]) && tb.contains(&targets[1])) {
            return Err("targets differ".into());
        }
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
    // The condition pair and its sense.
    let mut cond: Option<(VarnodeId, VarnodeId, bool)> = None;
    if nout == 2 {
        let (pa, pb) = (data.obank().get(ta).unwrap(), data.obank().get(tb).unwrap());
        if pa.code() != OpCode::CPUI_CBRANCH || pb.code() != OpCode::CPUI_CBRANCH {
            return Err("no cbranch".into());
        }
        let negated = true_target(data, a, ta) != true_target(data, b, tb);
        cond = Some((pa.get_in(1).unwrap(), pb.get_in(1).unwrap(), negated));
    }
    // Walk back pairing ops by shape.
    let mut ia = ops_a.len();
    let mut ib = ops_b.len();
    let mut pairs: Vec<(OpId, OpId, Vec<i32>)> = Vec::new();
    let mut stop = String::from("start of block");
    while ia > 0 && ib > 0 {
        let (oa, ob) = (ops_a[ia - 1], ops_b[ib - 1]);
        let negated = match cond {
            Some((ca, cb, true)) => {
                data.obank().get(oa).unwrap().get_out() == Some(ca)
                    && data.obank().get(ob).unwrap().get_out() == Some(cb)
            }
            _ => false,
        };
        match shape(data, oa, ob, negated) {
            Ok(slots) => pairs.push((oa, ob, slots)),
            Err(e) => {
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
    // Validate dataflow, cutting the run where one copy reads a value of the run the
    // other copy does not.
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
        // Returns Err(cut) when the run must start after index `cut`.
        let strict = std::env::var_os("KUNA_LJ_STRICT").is_some();
        let run_merges = std::cell::Cell::new(0usize);
        let check_pair = |va: VarnodeId, vb: VarnodeId, negated_ok: bool, at: Option<usize>| -> Result<(), Result<usize, String>> {
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
            if let Some(k) = at {
                if strict {
                    return Err(Ok(k));
                }
                run_merges.set(run_merges.get() + 1);
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
                let negated_ok = !(pa.code() == OpCode::CPUI_CBRANCH && matches!(cond, Some((_, _, true))));
                match check_pair(va, vb, negated_ok, Some(k)) {
                    Ok(()) => {}
                    Err(Ok(cut)) => {
                        start += cut + 1;
                        continue 'outer;
                    }
                    Err(Err(e)) => return Err(e),
                }
            }
        }
        // Outputs: used only inside the run or by the target MULTIEQUALs on the
        // copy's own edge.
        for (oa, ob, _) in run.iter() {
            for (o, blk, pos) in [(*oa, a, &pos_a), (*ob, b, &pos_b)] {
                if let Some(w) = data.obank().get(o).unwrap().get_out() {
                    for d in data.vbank().get(w).unwrap().descend_iter() {
                        let dop = data.obank().get(d).unwrap();
                        if pos.contains_key(&d) {
                            continue;
                        }
                        let parent = dop.get_parent();
                        let ok = dop.code() == OpCode::CPUI_MULTIEQUAL
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
        // Target MULTIEQUALs.
        for &t in &targets {
            let tb = data.bblocks_ref().block(t);
            let (sa, sb) = (tb.get_in_index(a), tb.get_in_index(b));
            for op in bb_ops(data, t) {
                let p = data.obank().get(op).unwrap();
                if p.code() != OpCode::CPUI_MULTIEQUAL {
                    continue;
                }
                let (va, vb) = (p.get_in(sa).unwrap(), p.get_in(sb).unwrap());
                match check_pair(va, vb, true, None) {
                    Ok(()) => {}
                    Err(Ok(cut)) => {
                        start += cut + 1;
                        continue 'outer;
                    }
                    Err(Err(e)) => return Err(format!("phi: {e}")),
                }
            }
        }
        // The run must hold the branch and the op computing its condition.
        if let Some((ca, _, _)) = cond {
            if def_pos(ca, &pos_a).is_none() {
                return Err(format!("condition not in run (run {}, {stop})", run.len()));
            }
        } else if run.iter().all(|(o, _, _)| data.obank().get(*o).unwrap().code() == OpCode::CPUI_BRANCH) {
            return Err("only a branch".into());
        }
        let stores = run
            .iter()
            .filter(|(o, _, _)| data.obank().get(*o).unwrap().code() == OpCode::CPUI_STORE)
            .count();
        if stores > 0 && std::env::var_os("KUNA_LJ_NOSTORE").is_some() {
            return Err("store (NOSTORE)".into());
        }
        return Ok(Plan {
            a,
            b,
            ops_a: run.iter().map(|p| p.0).collect(),
            ops_b: run.iter().map(|p| p.1).collect(),
            run_merges: run_merges.get(),
            stores,
        });
    }
}

fn execute(data: &mut Funcdata, plan: &Plan) {
    let (a, b) = (plan.a, plan.b);
    let nout = data.bblocks_ref().block(a).size_out();
    let targets: Vec<BlockId> = (0..nout).map(|i| data.bblocks_ref().block(a).get_out(i)).collect();
    let pos_a: HashMap<OpId, usize> = plan.ops_a.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let pos_b: HashMap<OpId, usize> = plan.ops_b.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let in_run = |data: &Funcdata, v: VarnodeId, pos: &HashMap<OpId, usize>| -> bool {
        data.vbank().get(v).unwrap().get_def().is_some_and(|d| pos.contains_key(&d))
    };
    // External input pairs that need a MULTIEQUAL, in first-use order.
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
    for (i, (&oa, &ob)) in plan.ops_a.iter().zip(plan.ops_b.iter()).enumerate() {
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
        let _ = i;
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
    let _ = &pos_b;

    // Control flow: A and B fall into a new block J that takes their out edges.
    let addr = data.obank().get(plan.ops_a[0]).unwrap().get_addr().clone();
    let j = if nout == 2 {
        let (t0, t1) = (targets[0], targets[1]);
        let (a0, b0) = (target_pairs[0].1, target_pairs[0].2);
        let (a1, b1) = (target_pairs[1].1, target_pairs[1].2);
        data.node_join_create_block(a, b, t0, t1, a0 > b0, a1 > b1, &addr)
    } else {
        let t = targets[0];
        let (sa, sb) = (target_pairs[0].1, target_pairs[0].2);
        let graph = data.bblocks_root_pub();
        let j = data.bblocks_mut().new_block_basic(graph);
        data.set_basic_block_range(j, &addr, &addr);
        let (keep, drop) = if sa > sb { (b, a) } else { (a, b) };
        data.bblocks_mut().remove_edge(drop, t);
        let s = data.bblocks_ref().block(keep).get_out_index(t);
        data.bblocks_mut().move_out_edge(keep, s, j);
        data.bblocks_mut().add_edge(a, j);
        data.bblocks_mut().add_edge(b, j);
        data.structure_reset();
        j
    };

    // MULTIEQUALs for the values that differ between the copies.
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
    let value = |data: &Funcdata, va: VarnodeId, vb: VarnodeId, merged: &HashMap<(VarnodeId, VarnodeId), VarnodeId>| {
        let _ = data;
        merged.get(&(va, vb)).copied().unwrap_or(va)
    };

    // Move A's run into J, reading merged values where the copies differ.
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
            let v = value(data, va, vb, &merged);
            if v != va {
                data.op_set_input(oa, v, s).expect("latchjoin: opSetInput");
            }
        }
        data.op_uninsert(oa);
        data.op_insert_end(oa, j);
    }

    // Target MULTIEQUALs: the inputs from A and B become the one from J.
    for (t, sa, sb, phis) in &target_pairs {
        let (lo, hi) = if sa > sb { (*sb, *sa) } else { (*sa, *sb) };
        for &(op, va, vb) in phis {
            let v = value(data, va, vb, &merged);
            data.op_remove_input(op, hi);
            data.op_set_input(op, v, lo).expect("latchjoin: opSetInput");
            if data.obank().get(op).unwrap().num_input() == 1 {
                data.op_uninsert(op);
                data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
                data.op_insert_begin(op, *t);
            }
        }
    }

    // B's copy is now dead.
    for &ob in plan.ops_b.iter().rev() {
        data.op_destroy(ob);
    }
}

/// TEMP classification dump (KUNA_LJ_DUMP=<dir>): one JSON line per latch pair.
fn dump(data: &Funcdata, h: BlockId, a: BlockId, b: BlockId, r: &Verdict) {
    let dir = match std::env::var_os("KUNA_LJ_DUMP") {
        Some(d) => d,
        None => return,
    };
    let desc = |bl: BlockId| -> String {
        let ops = bb_ops(data, bl);
        let blk = data.bblocks_ref().block(bl);
        let outs: Vec<String> = (0..blk.size_out())
            .map(|i| format!("\"{}\"", start_of(data, blk.get_out(i))))
            .collect();
        let names: Vec<String> = ops
            .iter()
            .map(|&o| {
                let p = data.obank().get(o).unwrap();
                format!("{:?}", p.code()).trim_start_matches("CPUI_").to_lowercase()
            })
            .collect();
        format!("{{\"at\":\"{}\",\"outs\":[{}],\"ops\":\"{}\"}}", start_of(data, bl), outs.join(","), names.join(" "))
    };
    let (verdict, run) = match r {
        Ok(p) => (format!("ok nout={} rm={} st={}", data.bblocks_ref().block(a).size_out(), p.run_merges, p.stores), p.ops_a.len()),
        Err(e) => (e.replace('"', "'"), 0),
    };
    let tdesc = |bl: BlockId| -> String {
        let blk = data.bblocks_ref().block(bl);
        let mut v = Vec::new();
        for i in 0..blk.size_out() {
            let t = blk.get_out(i);
            let ops: Vec<String> = bb_ops(data, t)
                .iter()
                .map(|&o| format!("{:?}", data.obank().get(o).unwrap().code()).trim_start_matches("CPUI_").to_lowercase())
                .collect();
            let tb = data.bblocks_ref().block(t);
            v.push(format!("{{\"t\":\"{}\",\"in\":{},\"out\":{},\"ops\":\"{}\"}}", start_of(data, t), tb.size_in(), tb.size_out(), ops.join(" ")));
        }
        format!("[{}]", v.join(","))
    };
    let extra = if verdict.starts_with("targets differ") || verdict.starts_with("outs") {
        format!(",\"ta\":{},\"tb\":{}", tdesc(a), tdesc(b))
    } else {
        String::new()
    };
    let line = format!(
        "{{\"fn\":\"{}\",\"addr\":\"{:x}\",\"h\":\"{}\",\"a\":{},\"b\":{},\"v\":\"{}\",\"run\":{}{}}}\n",
        data.get_name(),
        data.get_address().get_offset(),
        start_of(data, h),
        desc(a),
        desc(b),
        verdict,
        run,
        extra
    );
    let path = std::path::Path::new(&dir).join(format!("{}.jsonl", std::process::id()));
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

fn start_of(data: &Funcdata, bl: BlockId) -> String {
    match data.bb_op_head(bl) {
        Some(o) => format!("{:x}", data.obank().get(o).unwrap().get_addr().get_offset()),
        None => "-".to_string(),
    }
}
