//! Port of `ActionConditionalConst` (`decompiler/cpp/coreaction.cc:4292-4780`) —
//! constant propagation down conditional branches and through MULTIEQUAL (phi)
//! edges.
//!
//! # What this is
//!
//! When a CBRANCH keys on a Varnode being compared to a constant (`x == k`), the
//! value of `x` is *known* down one out-edge.  [`apply`] walks the block graph,
//! finds each such "constant point" (also the boolean itself, which is `0`/`1`
//! down its two edges), and rewrites every dominated read of the Varnode with a
//! constant Varnode.
//!
//! The interesting case the reviews named is **constant-propagation-through-phi**
//! ([`handle_phi_nodes`]): a value flows into a MULTIEQUAL on the constant edge,
//! and if excising that edge leaves no alternate data-flow path that rejoins the
//! original Varnode, the edge is replaced with a freshly-materialized constant
//! COPY.  This is what turns `*ptr = a + b` (the un-propagated form) into
//! `*ptr = b` (the recovered constant) in `condconst.xml` #1, and similarly
//! resolves #3 (`ptr[2] = 10`) and #5 (`ptr[4] = 0x10`).
//!
//! # Faithfulness
//!
//! Every method of the C++ class is reproduced as a free function over
//! `&mut Funcdata` (the class is otherwise stateless apart from `count`, which is
//! threaded as a return delta).  `ConstPoint` is a local struct; the
//! `vector<PcodeOpNode>` edge lists reuse [`crate::expression::PcodeOpNode`].
//! `uintb` is `u64`; `executeSimple` is reproduced via the engine `OpBehavior`
//! table exactly as `RuleCollapseConstants` does.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::expression::PcodeOpNode;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{BlockId, OpId, VarnodeId};

/// Description of a point in control-flow where a Varnode can propagate as a
/// constant down a conditional branch (C++ `ActionConditionalConst::ConstPoint`,
/// `coreaction.hh:585`).
#[derive(Clone, Copy)]
struct ConstPoint {
    /// Varnode that is constant for some reads (C++ `Varnode *vn`).
    vn: VarnodeId,
    /// Representative of the constant, may be null (C++ `Varnode *constVn`).
    const_vn: Option<VarnodeId>,
    /// The constant value (C++ `uintb value`).
    value: uintb,
    /// Block that dominates all reads where `vn` is constant (C++ `constBlock`).
    const_block: BlockId,
    /// Input edge from the condition block (C++ `int4 inSlot`).
    in_slot: int4,
    /// Is `true` if the block is dominated by the constant path (C++ `blockIsDom`).
    block_is_dom: bool,
}

impl ConstPoint {
    /// Construct from a constant Varnode (C++ first ctor): `value = c->getOffset()`.
    fn from_const_vn(
        data: &Funcdata,
        v: VarnodeId,
        c: VarnodeId,
        bl: BlockId,
        slot: int4,
        is_dom: bool,
    ) -> ConstPoint {
        let value = data.vbank().get(c).expect("ConstPoint: stale const vn").get_offset();
        ConstPoint { vn: v, const_vn: Some(c), value, const_block: bl, in_slot: slot, block_is_dom: is_dom }
    }

    /// Construct from a constant value (C++ second ctor): `constVn = 0`.
    fn from_value(v: VarnodeId, val: uintb, bl: BlockId, slot: int4, is_dom: bool) -> ConstPoint {
        ConstPoint { vn: v, const_vn: None, value: val, const_block: bl, in_slot: slot, block_is_dom: is_dom }
    }
}

/// The unique descendant op of a Varnode, or `None` if there are zero or more
/// than one (C++ `Varnode::loneDescend`, `varnode.cc`).
fn lone_descend(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    let v = data.vbank().get(vn).expect("loneDescend: stale vn");
    let mut it = v.descend_iter();
    let first = it.next()?;
    if it.next().is_some() {
        return None; // More than 1 descendant
    }
    Some(first)
}

/// Clear all marks on the given list of PcodeOps (C++
/// `ActionConditionalConst::clearMarks`, `coreaction.cc:4295`).
fn clear_marks(data: &mut Funcdata, op_list: &[OpId]) {
    for &op in op_list {
        data.obank_mut().get_mut(op).expect("clearMarks: stale op").clear_mark();
    }
}

/// Collect COPY, INDIRECT, and MULTIEQUAL ops reachable from the given Varnode,
/// without going through excised edges (C++
/// `ActionConditionalConst::collectReachable`, `coreaction.cc:4309`).
fn collect_reachable(
    data: &mut Funcdata,
    vn: VarnodeId,
    phi_node_edges: &mut [PcodeOpNode],
    reachable: &mut Vec<OpId>,
) {
    // sort(phiNodeEdges.begin(),phiNodeEdges.end());
    let obank = data.obank();
    phi_node_edges.sort_by(|a, b| {
        if a.less(b, obank) {
            std::cmp::Ordering::Less
        } else if b.less(a, obank) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let mut count: usize = 0;
    let mut vn = vn;
    // if (vn->isWritten()) { op = vn->getDef(); if (op->code()==MULTIEQUAL) {...} }
    if data.vbank().get(vn).expect("collectReachable: stale vn").is_written() {
        let op = data.vbank().get(vn).expect("collectReachable: stale vn").get_def().unwrap();
        if data.obank().get(op).expect("collectReachable: stale def").code() == OpCode::CPUI_MULTIEQUAL {
            data.obank_mut().get_mut(op).expect("collectReachable: stale def").set_mark();
            reachable.push(op);
        }
    }
    loop {
        // for(iter=vn->beginDescend();iter!=vn->endDescend();++iter)
        let descend: Vec<OpId> =
            data.vbank().get(vn).expect("collectReachable: stale vn").descend_iter().collect();
        for op in descend {
            if data.obank().get(op).expect("collectReachable: stale op").is_mark() {
                continue;
            }
            let opc = data.obank().get(op).expect("collectReachable: stale op").code();
            if opc == OpCode::CPUI_MULTIEQUAL {
                let num_input = data.obank().get(op).expect("collectReachable: stale op").num_input();
                // Find the incoming slot for the current Varnode, then test if it
                // is *not* an excised edge (binary_search of phiNodeEdges).
                let mut slot = 0;
                while slot < num_input {
                    if data.obank().get(op).expect("collectReachable: stale op").get_in(slot) != Some(vn) {
                        slot += 1;
                        continue;
                    }
                    // Don't count as flow if coming thru an excised edge.
                    let tmp = PcodeOpNode::new(op, slot);
                    if !binary_search_edge(data, phi_node_edges, &tmp) {
                        break;
                    }
                    slot += 1;
                }
                if slot == num_input {
                    continue; // Was the MULTIEQUAL reached
                }
            } else if opc != OpCode::CPUI_COPY && opc != OpCode::CPUI_INDIRECT {
                continue;
            }
            reachable.push(op);
            data.obank_mut().get_mut(op).expect("collectReachable: stale op").set_mark();
        }
        if count >= reachable.len() {
            break;
        }
        vn = data.obank().get(reachable[count]).expect("collectReachable: stale op").get_out().unwrap();
        count += 1;
    }
}

/// `binary_search(phiNodeEdges.begin(),phiNodeEdges.end(),tmpOp)` over a list
/// already sorted by [`PcodeOpNode::less`].
fn binary_search_edge(data: &Funcdata, edges: &[PcodeOpNode], target: &PcodeOpNode) -> bool {
    let obank = data.obank();
    // Standard binary search over the `less` strict-weak ordering.
    let mut lo = 0usize;
    let mut hi = edges.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if edges[mid].less(target, obank) {
            lo = mid + 1;
        } else if target.less(&edges[mid], obank) {
            hi = mid;
        } else {
            return true;
        }
    }
    false
}

/// Does the output of the given op reunite with the alternate flow (C++
/// `ActionConditionalConst::flowToAlternatePath`, `coreaction.cc:4355`).
fn flow_to_alternate_path(data: &mut Funcdata, op: OpId) -> bool {
    // if (op->isMark()) return true;
    if data.obank().get(op).expect("flowToAlternatePath: stale op").is_mark() {
        return true;
    }
    let mut mark_set: Vec<VarnodeId> = Vec::new();
    let vn = data.obank().get(op).expect("flowToAlternatePath: stale op").get_out().unwrap();
    mark_set.push(vn);
    data.vbank_mut().get_mut(vn).expect("flowToAlternatePath: stale vn").set_mark();
    let mut count: usize = 0;
    let mut found_path = false;
    while count < mark_set.len() {
        let vn = mark_set[count];
        count += 1;
        let descend: Vec<OpId> =
            data.vbank().get(vn).expect("flowToAlternatePath: stale vn").descend_iter().collect();
        for next_op in descend {
            let opc = data.obank().get(next_op).expect("flowToAlternatePath: stale op").code();
            if opc == OpCode::CPUI_MULTIEQUAL {
                if data.obank().get(next_op).expect("flowToAlternatePath: stale op").is_mark() {
                    found_path = true;
                    break;
                }
            } else if opc != OpCode::CPUI_COPY && opc != OpCode::CPUI_INDIRECT {
                continue;
            }
            let out_vn = data.obank().get(next_op).expect("flowToAlternatePath: stale op").get_out().unwrap();
            if data.vbank().get(out_vn).expect("flowToAlternatePath: stale out").is_mark() {
                continue;
            }
            data.vbank_mut().get_mut(out_vn).expect("flowToAlternatePath: stale out").set_mark();
            mark_set.push(out_vn);
        }
        if found_path {
            break;
        }
    }
    for &v in &mark_set {
        data.vbank_mut().get_mut(v).expect("flowToAlternatePath: stale vn").clear_mark();
    }
    found_path
}

/// Test if flow from a specific edge is disjoint from other edges (C++
/// `ActionConditionalConst::flowTogether`, `coreaction.cc:4400`).
fn flow_together(
    data: &mut Funcdata,
    edges: &[PcodeOpNode],
    i: usize,
    result: &mut [int4],
) -> bool {
    let mut reachable: Vec<OpId> = Vec::new();
    let mut excise: Vec<PcodeOpNode> = Vec::new(); // No edge excised
    let start = data.obank().get(edges[i].op).expect("flowTogether: stale op").get_out().unwrap();
    collect_reachable(data, start, &mut excise, &mut reachable);
    let mut res = false;
    for j in 0..edges.len() {
        if i == j {
            continue;
        }
        if result[j] == 0 {
            continue; // Check for disconnected path
        }
        if data.obank().get(edges[j].op).expect("flowTogether: stale op").is_mark() {
            result[i] = 2; // Disconnected paths, which flow together
            result[j] = 2;
            res = true;
        }
    }
    clear_marks(data, &reachable);
    res
}

/// Place a COPY of a constant at the end of a basic block (C++
/// `ActionConditionalConst::placeCopy`, `coreaction.cc:4427`).
fn place_copy(data: &mut Funcdata, op: OpId, bl: BlockId, const_vn: VarnodeId) -> VarnodeId {
    let last_op = last_op_of(data, bl);
    // Decide insertion point + the address to stamp on the new COPY.
    let (before, addr) = match last_op {
        None => {
            // iter = bl->endOp();  addr = op->getAddr();
            (None, data.obank().get(op).expect("placeCopy: stale op").get_addr().clone())
        }
        Some(lop) if data.obank().get(lop).expect("placeCopy: stale lastOp").is_branch() => {
            // iter = lastOp->getBasicIter();  addr = lastOp->getAddr();  (insert before branch)
            (Some(lop), data.obank().get(lop).expect("placeCopy: stale lastOp").get_addr().clone())
        }
        Some(lop) => {
            // iter = bl->endOp();  addr = lastOp->getAddr();
            (None, data.obank().get(lop).expect("placeCopy: stale lastOp").get_addr().clone())
        }
    };
    // copyOp = data.newOp(1,addr); opSetOpcode(copyOp, CPUI_COPY);
    let copy_op = data.new_op(1, addr);
    data.op_set_opcode(copy_op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    // outVn = data.newUniqueOut(constVn->getSize(), copyOp);
    let const_size = data.vbank().get(const_vn).expect("placeCopy: stale const vn").get_size();
    let out_vn = data.new_unique_out(const_size, copy_op).expect("placeCopy: newUniqueOut");
    // data.opSetInput(copyOp,constVn,0);  data.opInsert(copyOp, bl, iter);
    data.op_set_input(copy_op, const_vn, 0).expect("placeCopy: opSetInput");
    data.op_insert(copy_op, bl, before);
    out_vn
}

/// Place a single COPY assignment shared by multiple MULTIEQUALs (C++
/// `ActionConditionalConst::placeMultipleConstants`, `coreaction.cc:4462`).
fn place_multiple_constants(
    data: &mut Funcdata,
    phi_node_edges: &[PcodeOpNode],
    marks: &[int4],
    const_vn: VarnodeId,
) {
    let mut blocks: Vec<BlockId> = Vec::new();
    let mut op: Option<OpId> = None;
    for i in 0..phi_node_edges.len() {
        if marks[i] != 2 {
            continue; // Check that the MULTIEQUAL is marked as flowing together
        }
        op = Some(phi_node_edges[i].op);
        let parent = data.obank().get(phi_node_edges[i].op).expect("placeMultiple: stale op").get_parent().unwrap();
        let bl = data.bblocks_ref().block(parent).get_in(phi_node_edges[i].slot);
        blocks.push(bl);
    }
    // rootBlock = FlowBlock::findCommonBlock(blocks);
    let root_block = data.bblocks_mut().find_common_block_set(&blocks);
    let out_vn = place_copy(data, op.expect("placeMultiple: no flow-together op"), root_block, const_vn);
    for i in 0..phi_node_edges.len() {
        if marks[i] != 2 {
            continue;
        }
        data.op_set_input(phi_node_edges[i].op, out_vn, phi_node_edges[i].slot).expect("placeMultiple: opSetInput");
    }
}

/// Execute the op on the given constant inputs (C++ `PcodeOp::executeSimple`,
/// `op.cc:502`) via the engine [`OpBehavior`](kuna_num::opbehavior::OpBehavior)
/// table.  Returns `None` on an evaluation error (the C++ sets `evalError`).
fn execute_simple(data: &Funcdata, op: OpId, in_vals: &[uintb]) -> Option<uintb> {
    let (eval_type, opc, out_size, in0_size) = {
        let o = data.obank().get(op).expect("executeSimple: stale op");
        let out = o.get_out().expect("executeSimple: op has no output");
        let out_size = data.vbank().get(out).expect("executeSimple: stale out").get_size();
        let in0 = o.get_in(0).expect("executeSimple: op has no input 0");
        let in0_size = data.vbank().get(in0).expect("executeSimple: stale in0").get_size();
        (o.get_eval_type(), o.code(), out_size, in0_size)
    };
    let behave = data.get_arch().op_behavior(opc)?;
    if (eval_type & pcodeop_flags::unary) != 0 {
        behave.evaluate_unary(out_size, in0_size, in_vals[0]).ok()
    } else if (eval_type & pcodeop_flags::binary) != 0 {
        behave.evaluate_binary(out_size, in0_size, in_vals[0], in_vals[1]).ok()
    } else if (eval_type & pcodeop_flags::ternary) != 0 {
        behave.evaluate_ternary(out_size, in0_size, in_vals[0], in_vals[1], in_vals[2]).ok()
    } else {
        None // C++ throws LowlevelError -> not simply executable
    }
}

/// Try to push the constant at the front point through to the output of the
/// given PcodeOp (C++ `ActionConditionalConst::pushConstant`,
/// `coreaction.cc:4487`).
fn push_constant(data: &mut Funcdata, points: &mut Vec<ConstPoint>, op: OpId) {
    // if ((op->getEvalType() & special) != 0) return;
    if (data.obank().get(op).expect("pushConstant: stale op").get_eval_type() & pcodeop_flags::special) != 0 {
        return;
    }
    // if (op->getOpcode()->isFloatingPointOp()) return;
    let opc = data.obank().get(op).expect("pushConstant: stale op").code();
    if crate::typeop::type_op_info(opc).is_floating_point_op() {
        return;
    }
    // C++ `Varnode *outvn = op->getOut();` then immediately reads
    // `outvn->getSize()`.  An op with no output (a void CPUI_CALL, a STORE, a
    // branch) can never have a constant pushed *to* its result, so there is
    // nothing to do — return.  (In C++ `op->getOut()` is null here and the next
    // `outvn->getSize()` reads past a null pointer; the practical effect is the
    // size compares `> sizeof(uintb)` and the function returns.  The defined
    // transcription reaches that same "no constant pushed" outcome directly.)
    let out_vn = match data.obank().get(op).expect("pushConstant: stale op").get_out() {
        Some(v) => v,
        None => return,
    };
    // if (outvn->getSize() > sizeof(uintb)) return;
    if data.vbank().get(out_vn).expect("pushConstant: stale out").get_size() as usize > std::mem::size_of::<uintb>() {
        return;
    }
    let front = points[0];
    let slot = data.obank().get(op).expect("pushConstant: stale op").get_slot(front.vn);
    let num_input = data.obank().get(op).expect("pushConstant: stale op").num_input();
    let mut in_vals = [0u64; 3];
    for i in 0..num_input {
        if i == slot {
            in_vals[i as usize] = front.value;
        } else {
            let in_vn = data.obank().get(op).expect("pushConstant: stale op").get_in(i).unwrap();
            let v = data.vbank().get(in_vn).expect("pushConstant: stale in");
            if v.get_size() as usize > std::mem::size_of::<uintb>() {
                return;
            }
            if v.is_constant() {
                in_vals[i as usize] = v.get_offset();
            } else {
                return; // Not all inputs are constant
            }
        }
    }
    let outval = match execute_simple(data, op, &in_vals) {
        Some(v) => v,
        None => return, // evalError
    };
    // points.emplace_back(outvn,outval,front.constBlock,front.inSlot,front.blockIsDom);
    points.push(ConstPoint::from_value(out_vn, outval, front.const_block, front.in_slot, front.block_is_dom));
}

/// Replace MULTIEQUAL edges with a constant if there is no alternate flow (C++
/// `ActionConditionalConst::handlePhiNodes`, `coreaction.cc:4525`).
///
/// This is the **constant-propagation-through-phi** core.  Returns the number of
/// rewrites made (the C++ `count` delta).
fn handle_phi_nodes(
    data: &mut Funcdata,
    var_vn: VarnodeId,
    const_vn: VarnodeId,
    phi_node_edges: &mut [PcodeOpNode],
) -> int4 {
    let mut count: int4 = 0;
    let mut alternate_flow: Vec<OpId> = Vec::new();
    let mut results: Vec<int4> = vec![0; phi_node_edges.len()];
    collect_reachable(data, var_vn, phi_node_edges, &mut alternate_flow);
    let mut alternate = 0;
    for i in 0..phi_node_edges.len() {
        if !flow_to_alternate_path(data, phi_node_edges[i].op) {
            results[i] = 1; // Mark as disconnecting
            alternate += 1;
        }
    }
    clear_marks(data, &alternate_flow);

    let mut has_flow_together = false;
    if alternate > 1 {
        // Multiple MULTIEQUAL are disjoint from the non-constant flow.
        for i in 0..results.len() {
            if results[i] == 0 {
                continue; // Is this a disconnected path
            }
            if flow_together(data, phi_node_edges, i, &mut results) {
                has_flow_together = true;
            }
        }
    }
    // Add COPY assignment for each edge that has its own disconnected path going forward.
    for i in 0..phi_node_edges.len() {
        if results[i] != 1 {
            continue; // Disconnected path that does not flow into another path
        }
        let op = phi_node_edges[i].op;
        let slot = phi_node_edges[i].slot;
        let parent = data.obank().get(op).expect("handlePhiNodes: stale op").get_parent().unwrap();
        let bl = data.bblocks_ref().block(parent).get_in(slot);
        // (kuna) GH-9203: decline placing a const COPY in a loop block.
        if data.get_arch().condexe_block_placement && data.bblocks_ref().block(bl).has_loop_in() {
            results[i] = 0;
            continue;
        }
        let out_vn = place_copy(data, op, bl, const_vn);
        data.op_set_input(op, out_vn, slot).expect("handlePhiNodes: opSetInput");
        count += 1;
    }
    if has_flow_together {
        place_multiple_constants(data, phi_node_edges, &results, const_vn);
        count += 1;
    }
    count
}

/// Test if we can reach the given Varnode via a path other than through the
/// immediate edge (C++ `ActionConditionalConst::testAlternatePath`,
/// `coreaction.cc:4583`).
fn test_alternate_path(data: &Funcdata, vn: VarnodeId, op: OpId, slot: int4, depth: int4) -> bool {
    let num_input = data.obank().get(op).expect("testAlternatePath: stale op").num_input();
    for i in 0..num_input {
        if i == slot {
            continue;
        }
        let in_vn = data.obank().get(op).expect("testAlternatePath: stale op").get_in(i).unwrap();
        if in_vn == vn {
            return true;
        }
        if data.vbank().get(in_vn).expect("testAlternatePath: stale in").is_written() {
            let cur_op = data.vbank().get(in_vn).expect("testAlternatePath: stale in").get_def().unwrap();
            let opc = data.obank().get(cur_op).expect("testAlternatePath: stale def").code();
            if opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRSUB || opc == OpCode::CPUI_PTRADD {
                let in0 = data.obank().get(cur_op).expect("testAlternatePath: stale def").get_in(0);
                let in1 = data.obank().get(cur_op).expect("testAlternatePath: stale def").get_in(1);
                if in0 == Some(vn) || in1 == Some(vn) {
                    return true;
                }
            } else if opc == OpCode::CPUI_MULTIEQUAL {
                if depth == 0 {
                    continue;
                }
                if test_alternate_path(data, vn, cur_op, -1, depth - 1) {
                    return true;
                }
            }
        }
    }
    false
}

/// At each ConstPoint, replace reads of the Varnode down the constant path with
/// a constant Varnode (C++ `ActionConditionalConst::propagateConstant`,
/// `coreaction.cc:4617`).  Returns the number of rewrites made.
fn propagate_constant(data: &mut Funcdata, points: &mut Vec<ConstPoint>, use_multiequal: bool) -> int4 {
    let mut count: int4 = 0;
    let mut phi_node_edges: Vec<PcodeOpNode> = Vec::new();
    while !points.is_empty() {
        let var_vn = points[0].vn;
        let mut const_vn: Option<VarnodeId> = points[0].const_vn;
        let const_block = points[0].const_block;
        let point_value = points[0].value;
        let point_in_slot = points[0].in_slot;
        let point_block_is_dom = points[0].block_is_dom;

        // Snapshot the descend list (the C++ advances an iterator while erasing
        // descendants, which it handles by skipping duplicate `*iter == op`).
        let descend: Vec<OpId> =
            data.vbank().get(var_vn).expect("propagateConstant: stale vn").descend_iter().collect();
        let mut di = 0usize;
        while di < descend.len() {
            let op = descend[di];
            // Advance the iterator off the current op (as this descendant may be erased).
            while di < descend.len() && descend[di] == op {
                di += 1;
            }
            let opc = data.obank().get(op).expect("propagateConstant: stale op").code();
            if opc == OpCode::CPUI_INDIRECT {
                continue; // Don't propagate constant into these
            } else if opc == OpCode::CPUI_MULTIEQUAL {
                if !use_multiequal {
                    continue;
                }
                // if (varVn->isAddrTied() && varVn->getAddr() == op->getOut()->getAddr()) continue;
                let var_addr_tied = data.vbank().get(var_vn).expect("propagateConstant: stale vn").is_addr_tied();
                if var_addr_tied {
                    let out = data.obank().get(op).expect("propagateConstant: stale op").get_out().unwrap();
                    let var_addr = data.vbank().get(var_vn).expect("propagateConstant: stale vn").get_addr().clone();
                    let out_addr = data.vbank().get(out).expect("propagateConstant: stale out").get_addr().clone();
                    if var_addr == out_addr {
                        continue;
                    }
                }
                let bl = data.obank().get(op).expect("propagateConstant: stale op").get_parent().unwrap();
                if bl == const_block {
                    // The immediate edge from the conditional block, into a MULTIEQUAL.
                    let in_at_slot = data.obank().get(op).expect("propagateConstant: stale op").get_in(point_in_slot);
                    if in_at_slot == Some(var_vn) {
                        // The compiler may still intend the constant to be the same variable.
                        if point_value > 1 {
                            continue;
                        }
                        let out = data.obank().get(op).expect("propagateConstant: stale op").get_out().unwrap();
                        if data.vbank().get(out).expect("propagateConstant: stale out").is_addr_tied() {
                            continue;
                        }
                        if test_alternate_path(data, var_vn, op, point_in_slot, 2) {
                            continue;
                        }
                        phi_node_edges.push(PcodeOpNode::new(op, point_in_slot));
                    }
                } else if point_block_is_dom {
                    let num_input = data.obank().get(op).expect("propagateConstant: stale op").num_input();
                    for slot in 0..num_input {
                        if data.obank().get(op).expect("propagateConstant: stale op").get_in(slot) == Some(var_vn) {
                            let in_block = data.bblocks_ref().block(bl).get_in(slot);
                            if data.bblocks_ref().dominates(const_block, Some(in_block)) {
                                phi_node_edges.push(PcodeOpNode::new(op, slot));
                            }
                        }
                    }
                }
                continue;
            } else if opc == OpCode::CPUI_COPY {
                // Don't propagate into COPY unless it feeds something interesting.
                let out = data.obank().get(op).expect("propagateConstant: stale op").get_out().unwrap();
                let lone = lone_descend(data, out);
                let follow = match lone {
                    None => continue,
                    Some(f) => f,
                };
                if data.obank().get(follow).expect("propagateConstant: stale follow").is_marker() {
                    continue;
                }
                if data.obank().get(follow).expect("propagateConstant: stale follow").code() == OpCode::CPUI_COPY {
                    continue;
                }
                // ...fall through: COPY into something more interesting.
            }
            if !point_block_is_dom {
                continue;
            }
            let op_parent = data.obank().get(op).expect("propagateConstant: stale op").get_parent().unwrap();
            if data.bblocks_ref().dominates(const_block, Some(op_parent)) {
                if const_vn.is_none() {
                    let size = data.vbank().get(var_vn).expect("propagateConstant: stale vn").get_size();
                    const_vn = Some(data.new_constant(size, point_value));
                }
                let cvn = const_vn.unwrap();
                if opc == OpCode::CPUI_RETURN {
                    // CPUI_RETURN can't directly take constants as inputs.
                    let addr = data.obank().get(op).expect("propagateConstant: stale op").get_addr().clone();
                    let copy_before_ret = data.new_op(1, addr);
                    data.op_set_opcode(copy_before_ret, crate::typeop::type_op_for(OpCode::CPUI_COPY));
                    data.op_set_input(copy_before_ret, cvn, 0).expect("propagateConstant: opSetInput RETURN");
                    let (var_size, var_addr) = {
                        let v = data.vbank().get(var_vn).expect("propagateConstant: stale vn");
                        (v.get_size(), v.get_addr().clone())
                    };
                    data.new_varnode_out(var_size, &var_addr, copy_before_ret).expect("propagateConstant: newVarnodeOut");
                    let cb_out = data.obank().get(copy_before_ret).expect("propagateConstant: stale copy").get_out().unwrap();
                    data.op_set_input(op, cb_out, 1).expect("propagateConstant: opSetInput RETURN2");
                    data.op_insert_before(copy_before_ret, op);
                } else {
                    let slot = data.obank().get(op).expect("propagateConstant: stale op").get_slot(var_vn);
                    data.op_set_input(op, cvn, slot).expect("propagateConstant: opSetInput"); // Replace ref with constant!
                }
                count += 1; // We made a change
            } else {
                push_constant(data, points, op);
            }
        }
        if !phi_node_edges.is_empty() {
            if const_vn.is_none() {
                let size = data.vbank().get(var_vn).expect("propagateConstant: stale vn").get_size();
                const_vn = Some(data.new_constant(size, point_value));
            }
            count += handle_phi_nodes(data, var_vn, const_vn.unwrap(), &mut phi_node_edges);
            phi_node_edges.clear();
        }
        points.remove(0); // points.pop_front();
    }
    count
}

/// Find a Varnode being compared to a constant creating the given CBRANCH
/// boolean (C++ `ActionConditionalConst::findConstCompare`,
/// `coreaction.cc:4712`).
fn find_const_compare(
    data: &Funcdata,
    points: &mut Vec<ConstPoint>,
    bool_vn: VarnodeId,
    bl: BlockId,
    block_dom: &[bool; 2],
    flip_edge: bool,
) {
    let mut bool_vn = bool_vn;
    let mut flip_edge = flip_edge;
    if !data.vbank().get(bool_vn).expect("findConstCompare: stale bool").is_written() {
        return;
    }
    let mut comp_op = data.vbank().get(bool_vn).expect("findConstCompare: stale bool").get_def().unwrap();
    let mut opc = data.obank().get(comp_op).expect("findConstCompare: stale comp").code();
    if opc == OpCode::CPUI_BOOL_NEGATE {
        flip_edge = !flip_edge;
        bool_vn = data.obank().get(comp_op).expect("findConstCompare: stale comp").get_in(0).unwrap();
        if !data.vbank().get(bool_vn).expect("findConstCompare: stale bool2").is_written() {
            return;
        }
        comp_op = data.vbank().get(bool_vn).expect("findConstCompare: stale bool2").get_def().unwrap();
        opc = data.obank().get(comp_op).expect("findConstCompare: stale comp2").code();
    }
    // Out edge where the value is constant.
    let mut const_edge: int4 = if opc == OpCode::CPUI_INT_EQUAL {
        1
    } else if opc == OpCode::CPUI_INT_NOTEQUAL {
        0
    } else {
        return;
    };
    // Find the variable and verify it is compared to a constant.
    let mut var_vn = data.obank().get(comp_op).expect("findConstCompare: stale comp").get_in(0).unwrap();
    let mut const_vn = data.obank().get(comp_op).expect("findConstCompare: stale comp").get_in(1).unwrap();
    if !data.vbank().get(const_vn).expect("findConstCompare: stale const").is_constant() {
        if !data.vbank().get(var_vn).expect("findConstCompare: stale var").is_constant() {
            return;
        }
        std::mem::swap(&mut const_vn, &mut var_vn);
    }
    // if (varVn->loneDescend() != 0) return;
    if lone_descend(data, var_vn).is_some() {
        return;
    }
    if flip_edge {
        const_edge = 1 - const_edge;
    }
    let out_block = data.bblocks_ref().block(bl).get_out(const_edge);
    let rev_index = data.bblocks_ref().block(bl).get_out_rev_index(const_edge);
    points.push(ConstPoint::from_const_vn(
        data,
        var_vn,
        const_vn,
        out_block,
        rev_index,
        block_dom[const_edge as usize],
    ));
}

/// Last op of a basic block (C++ `BlockBasic::lastOp` = `op.back()`).
fn last_op_of(data: &Funcdata, bl: BlockId) -> Option<OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_tail,
        _ => None,
    }
}

/// Drive the whole conditional-constant propagation pass (C++
/// `ActionConditionalConst::apply`, `coreaction.cc:4748`).  Returns the number of
/// rewrites made (the C++ `count` at exit).
pub(crate) fn condconst_apply(data: &mut Funcdata) -> int4 {
    let mut count: int4 = 0;
    let mut use_multiequal = true;
    // Determining if conditional constants should apply to MULTIEQUAL operations
    // may require flow calculations.
    let stack_space = data.get_arch().manage().get_stack_space().cloned();
    if let Some(spc) = stack_space {
        // If the stack hasn't been heritaged yet, don't propagate into MULTIEQUAL.
        // (The C++ `numHeritagePasses` throws for a non-heritaged space; that
        // throw never fires at this point in the pipeline, but if it did the
        // space is simply un-heritaged -> numPasses <= 0 -> useMultiequal=false.)
        let num_passes = data.num_heritage_passes(&spc).unwrap_or(0);
        if num_passes <= 0 {
            use_multiequal = false;
        }
    }
    let mut points: Vec<ConstPoint> = Vec::new();
    let block_count = data.bblocks_get_size();
    for i in 0..block_count {
        let bl = data.bblocks_get_block(i);
        let c_branch = match last_op_of(data, bl) {
            Some(op) => op,
            None => continue,
        };
        if data.obank().get(c_branch).expect("condconst: stale cbranch").code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let bool_vn = data.obank().get(c_branch).expect("condconst: stale cbranch").get_in(1).unwrap();
        // Make sure the boolean constant holds down each branch.
        let out0 = data.bblocks_ref().block(bl).get_out(0);
        let out1 = data.bblocks_ref().block(bl).get_out(1);
        let block_dom = [
            data.bblocks_ref().restricted_by_conditional(out0, bl),
            data.bblocks_ref().restricted_by_conditional(out1, bl),
        ];
        let flip_edge = data.obank().get(c_branch).expect("condconst: stale cbranch").is_boolean_flip();
        // if (boolVn->loneDescend() == 0)  -- the boolean is read more than once
        if lone_descend(data, bool_vn).is_none() {
            // Implied constants: bool=0 down false branch, bool=1 down true branch.
            let false_out = data.bblocks_ref().block(bl).get_false_out();
            let true_out = data.bblocks_ref().block(bl).get_true_out();
            let rev0 = data.bblocks_ref().block(bl).get_out_rev_index(0);
            let rev1 = data.bblocks_ref().block(bl).get_out_rev_index(1);
            points.push(ConstPoint::from_value(
                bool_vn,
                if flip_edge { 1 } else { 0 },
                false_out,
                rev0,
                block_dom[0],
            ));
            points.push(ConstPoint::from_value(
                bool_vn,
                if flip_edge { 0 } else { 1 },
                true_out,
                rev1,
                block_dom[1],
            ));
        }
        find_const_compare(data, &mut points, bool_vn, bl, &block_dom, flip_edge);
        count += propagate_constant(data, &mut points, use_multiequal);
    }
    count
}
