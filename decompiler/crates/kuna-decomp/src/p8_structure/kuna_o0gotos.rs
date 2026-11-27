//! TEMP dump of gotoreduce/taildup conversions (KUNA_O0G_DUMP=<dir>).

use std::io::Write;

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::block::BlockType;
use crate::context::BlockId;
use crate::funcdata::Funcdata;

fn leaves(data: &Funcdata, root: BlockId, out: &mut Vec<BlockId>) {
    let sg = data.sblocks_ref();
    let mut stack = vec![root];
    while let Some(b) = stack.pop() {
        if sg.block(b).get_type() == BlockType::Copy {
            if let Some(bb) = sg.block(b).get_copy() {
                out.push(bb);
            }
            continue;
        }
        for i in 0..sg.block(b).get_size() {
            stack.push(sg.block(b).get_block(i));
        }
    }
}

fn op_info(data: &Funcdata, bb: BlockId) -> (String, u64, Option<u64>, bool) {
    let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else { return ("none".into(), 0, None, false) };
    let dest = op
        .get_in(0)
        .and_then(|v| data.vbank().get(v))
        .filter(|v| v.get_space().get_type() != spacetype::IPTR_CONSTANT)
        .map(|v| v.get_offset());
    let dest = if matches!(op.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) { dest } else { None };
    (format!("{:?}", op.code()), op.get_addr().get_offset(), dest, op.is_fallthru_true())
}

pub(crate) fn dump(data: &Funcdata, pass: &str, node: BlockId, is_goto: bool, chain: &[BlockId]) {
    let Ok(dir) = std::env::var("KUNA_O0G_DUMP") else { return };
    let sg = data.sblocks_ref();
    let mut node_bbs = Vec::new();
    if is_goto {
        leaves(data, node, &mut node_bbs);
    } else if sg.block(node).get_size() > 0 {
        leaves(data, sg.block(node).get_block(0), &mut node_bbs);
    }
    let (mut loops, mut ifs, mut switches) = (0, 0, 0);
    let mut cur = node;
    while let Some(up) = sg.block(cur).get_parent() {
        match sg.block(up).get_type() {
            BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => loops += 1,
            BlockType::If => ifs += 1,
            BlockType::Switch => switches += 1,
            _ => {}
        }
        cur = up;
    }
    let bb0 = chain[0];
    let g = data.bblocks_ref();
    let start = |b: BlockId| data.bb_op_head(b).and_then(|o| data.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
    let mut edges = Vec::new();
    for i in 0..g.block(bb0).size_in() {
        let src = g.block(bb0).get_in(i);
        let slot = g.block(bb0).get_in_rev_index(i);
        let lab = g.block(bb0).get_in_label(i);
        let (code, addr, dest, flip) = op_info(data, src);
        let jump = match code.as_str() {
            "CPUI_BRANCH" => true,
            "CPUI_CBRANCH" => (slot == 1) != flip,
            _ => false,
        };
        let other = if g.block(src).size_out() == 2 { start(g.block(src).get_out(1 - slot)) } else { 0 };
        let jid = crate::p8_structure::kuna_o0jumps::id_of(lab);
        let rtag = (lab >> 16) & 0x7fff;
        let gbit = lab & GOTO_BIT != 0;
        edges.push(format!(
            "{{\"code\":\"{}\",\"addr\":{},\"dest\":{},\"jump\":{},\"jid\":{},\"rtag\":{},\"node\":{},\"src_start\":{},\"src_in\":{},\"other\":{},\"src_out\":{},\"gbit\":{}}}",
            code,
            addr,
            dest.map_or("null".to_string(), |d| d.to_string()),
            jump,
            jid,
            rtag,
            node_bbs.contains(&src),
            start(src),
            g.block(src).size_in(),
            other,
            g.block(src).size_out(),
            gbit
        ));
    }
    let mut blocks = Vec::new();
    for &b in chain {
        let ops = data.bb_ops(b);
        let printed = ops.iter().filter(|&&o| data.obank().get(o).is_some_and(|op| !op.not_printed())).count();
        let calls = ops.iter().filter(|&&o| data.obank().get(o).is_some_and(|op| op.is_call())).count();
        let (code, _, _, _) = op_info(data, b);
        blocks.push(format!(
            "{{\"start\":{},\"ops\":{},\"printed\":{},\"calls\":{},\"tail\":\"{}\",\"in\":{}}}",
            start(b),
            ops.len(),
            printed,
            calls,
            code,
            g.block(b).size_in()
        ));
    }
    let line = format!(
        "{{\"rec\":{},\"fn\":{},\"pass\":\"{}\",\"goto\":{},\"unopt\":{},\"o0r\":{},\"o0r_dec\":{},\"o0j\":{},\"loops\":{},\"ifs\":{},\"sw\":{},\"t_start\":{},\"edges\":[{}],\"chain\":[{}]}}\n",
        data.kuna_o0gotos.is_some(),
        data.get_address().get_offset(),
        pass,
        is_goto,
        crate::p3_dataflow::kuna_unoptimized::is_unoptimized(data),
        data.kuna_o0returns.is_some(),
        crate::p8_structure::kuna_o0returns::decided(data),
        data.kuna_o0jumps.is_some(),
        loops,
        ifs,
        switches,
        start(bb0),
        edges.join(","),
        blocks.join(",")
    );
    let path = format!("{}/{}.jsonl", dir, std::process::id());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

// ---------------------------------------------------------------------------
// record (heritage pass 1)
// ---------------------------------------------------------------------------

pub const GOTO_BIT: kuna_base::types::uint4 = 0x8000_0000;
const MAX_GAP: u64 = 8;

#[derive(Debug, Clone, Default)]
pub struct Record {
    pub labels: u32,
}

fn head(fd: &Funcdata, b: BlockId) -> Option<u64> {
    fd.bb_op_head(b).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset())
}

fn tail_addr(fd: &Funcdata, b: BlockId) -> Option<u64> {
    fd.bb_op_tail(b).and_then(|o| fd.obank().get(o)).map(|o| o.get_addr().get_offset())
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
    match (branch_dest(fd, b), tail_addr(fd, b)) {
        (Some(d), Some(a)) => d <= a,
        _ => false,
    }
}

fn jump_only(fd: &Funcdata, b: BlockId) -> bool {
    let codes: Vec<OpCode> = fd
        .bb_ops(b)
        .iter()
        .filter_map(|&o| fd.obank().get(o).map(|op| op.code()))
        .filter(|c| !matches!(c, OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT))
        .collect();
    codes == [OpCode::CPUI_BRANCH] && fd.bblocks_ref().block(b).size_out() == 1
}

/// The bottom test of the loop whose last statement `if (c) { S; break; }` is
/// laid out after it: gcc folds the loop's back jump into the `if`'s test, so
/// `b` (S) is entered only by the fall-through of a backward branch that no
/// jump from before the loop reaches (a `for` or `while` test is reached by
/// the loop's entry jump, and code after it is after the loop).
fn tail_arm(fd: &Funcdata, b: BlockId) -> Option<BlockId> {
    let mut cur = b;
    for _ in 0..4 {
        let blk = fd.bblocks_ref().block(cur);
        if blk.size_in() != 1 {
            return None;
        }
        let q = blk.get_in(0);
        let h = head(fd, cur)?;
        if tail_addr(fd, q)? >= h {
            return None;
        }
        if backward(fd, q) {
            let qb = fd.bblocks_ref().block(q);
            if qb.size_out() != 2 {
                return None;
            }
            let top = branch_dest(fd, q)?;
            let entered_from_before = (0..qb.size_in()).any(|k| tail_addr(fd, qb.get_in(k)).is_some_and(|a| a < top));
            return (!entered_from_before).then_some(q);
        }
        cur = q;
    }
    None
}

struct Cand {
    p: BlockId,
    slot: i32,
    marker: bool,
    via: Option<BlockId>,
    t: BlockId,
    land: u64,
    from: u64,
}

pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_o0gotos.is_some() {
        return;
    }
    if fd.get_arch().goto_reduce != crate::p8_structure::kuna_gotoreduce::GotoReduceMode::Auto {
        return;
    }
    if !crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd) {
        return;
    }
    let min_direct: usize = std::env::var("KUNA_O0G_MINDIRECT").ok().and_then(|v| v.parse().ok()).unwrap_or(1000);
    let n = fd.bblocks_get_size();
    let blocks: Vec<BlockId> = (0..n).map(|i| fd.bblocks_get_block(i)).collect();
    let mut heads: Vec<(u64, BlockId)> = blocks.iter().filter_map(|&b| head(fd, b).map(|a| (a, b))).collect();
    heads.sort_unstable_by_key(|x| x.0);
    let mut tails: Vec<(u64, BlockId)> = blocks.iter().filter_map(|&b| tail_addr(fd, b).map(|a| (a, b))).collect();
    tails.sort_unstable_by_key(|x| x.0);
    // jump-table case starts, each with the lowest address of its dispatch
    let mut table_start: Vec<(BlockId, u64)> = Vec::new();
    for &b in &blocks {
        let Some(op) = fd.bb_op_tail(b).and_then(|o| fd.obank().get(o)) else { continue };
        if op.code() != OpCode::CPUI_BRANCHIND {
            continue;
        }
        let blk = fd.bblocks_ref().block(b);
        let mut lo = head(fd, b).unwrap_or(u64::MAX);
        for k in 0..blk.size_in() {
            if let Some(h) = head(fd, blk.get_in(k)) {
                lo = lo.min(h);
            }
        }
        for k in 0..blk.size_out() {
            table_start.push((blk.get_out(k), lo));
        }
    }
    let mut cands: Vec<Cand> = Vec::new();
    for &p in &blocks {
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
        let is_branch = op.code() == OpCode::CPUI_BRANCH;
        let Some(land) = branch_dest(fd, p) else { continue };
        let d = blk.get_out(slot);
        let Some(s) = head(fd, d) else { continue };
        if land < s && s - land <= MAX_GAP {
            if jump_only(fd, d) {
                let t = fd.bblocks_ref().block(d).get_out(0);
                cands.push(Cand { p, slot, marker: true, via: Some(d), t, land, from });
            } else {
                cands.push(Cand { p, slot, marker: true, via: None, t: d, land, from });
            }
        } else if is_branch && land == s && !jump_only(fd, p) {
            cands.push(Cand { p, slot, marker: false, via: None, t: d, land, from });
        }
    }
    let prev_tail = |a: u64| -> Option<BlockId> {
        let i = tails.partition_point(|x| x.0 < a);
        (i > 0).then(|| tails[i - 1].1)
    };
    let next_head = |a: u64| -> Option<BlockId> {
        let i = heads.partition_point(|x| x.0 <= a);
        heads.get(i).map(|x| x.1)
    };
    let mut targets: Vec<BlockId> = cands.iter().map(|c| c.t).collect();
    targets.sort_unstable();
    targets.dedup();
    let mut goto_edges: Vec<usize> = Vec::new();
    let mut dump = String::new();
    for &t in &targets {
        let st = head(fd, t).unwrap_or(0);
        let mut g = cands.iter().filter(|x| x.t == t && x.via.is_none() && x.land <= st).map(|x| x.land).min().unwrap_or(st);
        let mut lp: Option<(u64, u64)> = None;
        for _ in 0..16 {
            let Some(b) = prev_tail(g) else { break };
            if jump_only(fd, b) && fd.bblocks_ref().block(b).get_out(0) == t {
                let hb = head(fd, b).unwrap_or(g);
                g = cands.iter().filter(|x| x.via == Some(b)).map(|x| x.land).min().unwrap_or(hb).min(hb);
                continue;
            }
            if backward(fd, b) {
                lp = Some((branch_dest(fd, b).unwrap_or(0), tail_addr(fd, b).unwrap_or(0)));
            } else if let Some(q) = tail_arm(fd, b) {
                lp = Some((branch_dest(fd, q).unwrap_or(0), st));
            }
            break;
        }
        let tb = fd.bblocks_ref().block(t);
        let mut cont = (0..tb.size_in()).any(|k| {
            let q = tb.get_in(k);
            backward(fd, q) && branch_dest(fd, q).is_some_and(|d| d <= st)
        });
        let mut cur = t;
        for _ in 0..3 {
            let cb = fd.bblocks_ref().block(cur);
            if branch_dest(fd, cur).is_some() {
                cont |= backward(fd, cur);
                break;
            }
            if cb.size_out() != 1 {
                break;
            }
            cur = cb.get_out(0);
        }
        let case_of = |b: BlockId| table_start.iter().find(|y| y.0 == b).map(|y| y.1);
        let arm_end = cands.iter().filter(|x| x.t == t && !x.marker).filter_map(|x| next_head(x.from)).filter_map(case_of).min();
        let bare = (0..tb.size_in()).map(|k| tb.get_in(k)).filter(|&q| jump_only(fd, q)).filter_map(case_of).min();
        let sw: Option<u64> = match (arm_end, bare) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        let mine: Vec<usize> = (0..cands.len()).filter(|&i| cands[i].t == t).collect();
        let goto_of = |c: &Cand| {
            !cont
                && !lp.is_some_and(|(lo, hi)| c.from >= lo && c.from <= hi)
                && !sw.is_some_and(|lo| c.from >= lo && c.from < st)
        };
        let marker = mine.iter().any(|&i| cands[i].marker && goto_of(&cands[i]));
        let direct = mine.iter().filter(|&&i| !cands[i].marker && goto_of(&cands[i])).count();
        let label = marker || direct >= min_direct;
        for &i in &mine {
            let c = &cands[i];
            let ge = label && goto_of(c);
            if ge {
                goto_edges.push(i);
            }
            if std::env::var_os("KUNA_O0G_RDUMP").is_some() {
                dump.push_str(&format!(
                    "{{\"fn\":{},\"addr\":{},\"land\":{},\"d\":{},\"t\":{},\"marker\":{},\"via\":{},\"exit\":{},\"cont\":{},\"switch\":{},\"inloop\":{},\"insw\":{},\"nmarker\":{},\"ndirect\":{},\"goto\":{},\"code\":\"{}\"}}\n",
                    fd.get_address().get_offset(),
                    c.from,
                    c.land,
                    head(fd, fd.bblocks_ref().block(c.p).get_out(c.slot)).unwrap_or(0),
                    st,
                    c.marker,
                    c.via.is_some(),
                    lp.is_some(),
                    cont,
                    sw.is_some(),
                    lp.is_some_and(|(lo, hi)| c.from >= lo && c.from <= hi),
                    sw.is_some_and(|lo| c.from >= lo && c.from < st),
                    marker,
                    direct,
                    ge,
                    if fd.bb_op_tail(c.p).and_then(|o| fd.obank().get(o)).is_some_and(|o| o.code() == OpCode::CPUI_BRANCH) { "BR" } else { "CB" }
                ));
            }
        }
    }
    let mut count = 0;
    for &i in &goto_edges {
        let c = &cands[i];
        fd.bblocks_mut().set_out_edge_flag(c.p, c.slot, GOTO_BIT);
        if let Some(m) = c.via {
            let b = fd.bblocks_ref().block(m);
            let ins: Vec<(BlockId, i32)> = (0..b.size_in()).map(|k| (b.get_in(k), b.get_in_rev_index(k))).collect();
            for (q, qs) in ins {
                fd.bblocks_mut().set_out_edge_flag(q, qs, GOTO_BIT);
            }
        }
        count += 1;
    }
    if let Ok(dir) = std::env::var("KUNA_O0G_RDUMP") {
        let path = format!("{}/{}.rjsonl", dir, std::process::id());
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = f.write_all(dump.as_bytes());
        }
    }
    fd.kuna_o0gotos = Some(Box::new(Record { labels: count }));
}

/// Whether `gotoreduce auto` leaves the gotos to basic block `b` alone: in a
/// recorded function, `b` is a label the author's `goto` statements jump to.
pub(crate) fn keeps(fd: &Funcdata, b: BlockId) -> bool {
    if std::env::var_os("KUNA_O0G_NOKEEP").is_some() {
        return false;
    }
    if fd.kuna_o0gotos.is_none() {
        return false;
    }
    let blk = fd.bblocks_ref().block(b);
    (0..blk.size_in()).any(|i| blk.get_in_label(i) & GOTO_BIT != 0)
}
