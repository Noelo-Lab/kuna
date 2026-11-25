//! Close finite indirect dispatches without losing their incoming-return edge.
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::jumptable::JumpTable;
use kuna_base::address::calc_mask;
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeSet, HashMap, HashSet};

/// What a dispatch can jump to: local constants and the incoming return.
#[derive(Default)]
pub(crate) struct Destinations {
    pub(crate) constants: BTreeSet<u64>,
    pub(crate) returns: bool,
}

/// The destinations a bounded COPY/PHI/mask walk proves for `root`, or `None`
/// when any value is unknown.
pub(crate) fn destinations(
    data: &Funcdata,
    root: VarnodeId,
    encoding: (i32, u64),
    mut budget: usize,
) -> Option<Destinations> {
    collect(data, root, encoding, &mut HashSet::new(), &mut budget)
}

fn raw_constant(
    data: &Funcdata,
    mut cursor: OpId,
    vn: VarnodeId,
    budget: &mut usize,
) -> Option<u64> {
    let value = data.vbank().get(vn)?;
    if !(1..=8).contains(&value.get_size()) {
        return None;
    }
    if value.is_constant() {
        return Some(value.get_offset());
    }
    let address = data.obank().get(cursor)?.get_addr().clone();
    while *budget != 0 {
        *budget -= 1;
        cursor = data.obank().dead_prev(cursor)?;
        let op = data.obank().get(cursor)?;
        if *op.get_addr() != address {
            return None;
        }
        let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        if out.get_addr() != value.get_addr() || out.get_size() != value.get_size() {
            continue;
        }
        return match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                raw_constant(data, cursor, op.get_in(0)?, budget)
            }
            OpCode::CPUI_INT_2COMP => {
                raw_constant(data, cursor, op.get_in(0)?, budget).map(u64::wrapping_neg)
            }
            _ => None,
        }
        .map(|v| v & calc_mask(value.get_size()));
    }
    None
}

/// New direct/fallthrough edges into previously decoded instructions. Recovered
/// indirect edges are tracked separately because following them need not create
/// any new operations. Both can add values to a provisional dispatch.
///
/// Flow appends the operations it generates, and moves injected or inlined
/// sequences only next to other new operations, so the operations created
/// since `mark` are the dead list's tail.
pub(crate) fn reentry_edges(
    data: &Funcdata,
    mark: u32,
    target: impl Fn(&kuna_base::address::Address) -> Option<OpId>,
    fallthru: impl Fn(OpId) -> Option<OpId>,
) -> Vec<(OpId, OpId)> {
    let old = |op: OpId| data.obank().get(op).is_some_and(|o| o.get_time() < mark);
    let mut fresh = Vec::new();
    let mut cursor = data.obank().dead_back();
    while let Some(id) = cursor.filter(|&id| !old(id)) {
        fresh.push(id);
        cursor = data.obank().dead_prev(id);
    }
    fresh.reverse();
    fresh.into_iter().flat_map(|id| {
        let mut edges = Vec::new();
        let Some(op) = data.obank().get(id) else { return edges };
        if matches!(op.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
            if let Some(to) = op.get_in(0).and_then(|v| data.vbank().get(v))
                .and_then(|v| target(v.get_addr())).filter(|&to| old(to))
            { edges.push((id, to)); }
        }
        if !matches!(op.code(), OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN) {
            if let Some(to) = fallthru(id).filter(|&to| old(to)) { edges.push((id, to)); }
        }
        edges
    }).collect()
}

/// Operations one round's reachability walks may visit before every
/// provisional dispatch is assumed reachable.
pub(super) const REACH_BUDGET: usize = 1 << 20;

/// For each provisional dispatch, the new edges whose destination reaches it
/// through decoded code. Only those edges can bring it another value, so only
/// those dispatches are retried. A walk over budget reaches every dispatch.
pub(crate) fn reaching_edges(
    data: &Funcdata,
    dispatches: &BTreeSet<OpId>,
    edges: &[(OpId, OpId)],
    target: impl Fn(&kuna_base::address::Address) -> Option<OpId>,
    fallthru: impl Fn(OpId) -> Option<OpId>,
) -> std::collections::BTreeMap<OpId, Vec<(OpId, OpId)>> {
    let mut budget = REACH_BUDGET;
    let mut out: std::collections::BTreeMap<OpId, Vec<(OpId, OpId)>> = Default::default();
    for &edge in edges {
        let reached = reached(data, edge.1, dispatches, &mut budget, &target, &fallthru)
            .unwrap_or_else(|| dispatches.clone());
        for dispatch in reached {
            out.entry(dispatch).or_default().push(edge);
        }
    }
    out
}

/// The `wanted` operations reachable from `start` over raw flow: fallthrough
/// unless the operation always transfers, direct targets, every operation of an
/// instruction with an internal relative branch, and the recovered targets of
/// an indirect jump. `None` when the budget runs out.
pub(super) fn reached(
    data: &Funcdata,
    start: OpId,
    wanted: &BTreeSet<OpId>,
    budget: &mut usize,
    target: &impl Fn(&kuna_base::address::Address) -> Option<OpId>,
    fallthru: &impl Fn(OpId) -> Option<OpId>,
) -> Option<BTreeSet<OpId>> {
    let mut found = BTreeSet::new();
    let mut seen = HashSet::new();
    let mut work = vec![start];
    while let Some(id) = work.pop() {
        if !seen.insert(id) {
            continue;
        }
        if *budget == 0 {
            return None;
        }
        *budget -= 1;
        if wanted.contains(&id) {
            found.insert(id);
            if found.len() == wanted.len() {
                break;
            }
        }
        let op = data.obank().get(id)?;
        match op.code() {
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                let destination = op.get_in(0).and_then(|v| data.vbank().get(v))?;
                if destination.is_constant() {
                    let address = op.get_addr();
                    let mut cursor = data.obank().dead_prev(id);
                    while let Some(prev) = cursor.filter(|&p| data.obank().get(p).is_some_and(|o| o.get_addr() == address)) {
                        work.push(prev);
                        cursor = data.obank().dead_prev(prev);
                    }
                    work.extend(fallthru(id));
                } else {
                    work.extend(target(destination.get_addr()));
                    if op.code() == OpCode::CPUI_CBRANCH {
                        work.extend(fallthru(id));
                    }
                }
            }
            OpCode::CPUI_BRANCHIND => {
                if let Some(index) = data.find_jump_table_index(id) {
                    for address in data.get_jump_table(index as i32).address_table() {
                        work.extend(target(address));
                    }
                }
            }
            OpCode::CPUI_RETURN => {}
            _ if op.get_halt_type() != 0 => {}
            _ => work.extend(fallthru(id)),
        }
    }
    Some(found)
}

#[cfg(test)]
fn reentered_since(
    data: &Funcdata,
    mark: u32,
    target: impl Fn(&kuna_base::address::Address) -> Option<OpId>,
    fallthru: impl Fn(OpId) -> Option<OpId>,
) -> bool {
    !reentry_edges(data, mark, target, fallthru).is_empty()
}

/// The address mask emitted by the transfer itself, before SSA can combine it
/// with a source instruction's destructive mask.
pub(crate) fn target_encoding(data: &Funcdata, branch: OpId) -> Option<(i32, u64)> {
    let op = data.obank().get(branch)?;
    let root = data.vbank().get(op.get_in(0)?)?;
    let size = root.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    let mut result = calc_mask(size);
    let mut vn = op.get_in(0)?;
    let mut cursor = branch;
    for _ in 0..64 {
        let Some(previous) = data.obank().dead_prev(cursor) else {
            break;
        };
        cursor = previous;
        let producer = data.obank().get(cursor)?;
        if producer.get_addr() != op.get_addr() {
            break;
        }
        let Some(out) = producer.get_out().and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        let value = data.vbank().get(vn)?;
        if out.get_addr() != value.get_addr() || out.get_size() != value.get_size() {
            continue;
        }
        match producer.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => vn = producer.get_in(0)?,
            OpCode::CPUI_INT_AND => {
                let a = producer.get_in(0)?;
                let b = producer.get_in(1)?;
                if let Some(bits) = raw_constant(data, cursor, a, &mut 64) {
                    result &= bits;
                    vn = b;
                } else if let Some(bits) = raw_constant(data, cursor, b, &mut 64) {
                    result &= bits;
                    vn = a;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some((size, result))
}

pub(crate) fn incoming_return(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(value) = data.vbank().get(vn).filter(|v| v.is_input()) else {
        return false;
    };
    let Some(ret) = data
        .get_arch()
        .indirect_return_addr
        .as_ref()
        .or(data.get_arch().default_return_addr.as_ref())
    else {
        return false;
    };
    let Some(space) = &ret.space else {
        return false;
    };
    if space.get_index() != value.get_space().get_index() {
        return false;
    }
    let size = value.get_size() as u32;
    let address_size = data
        .get_address()
        .get_space()
        .map_or(0, |s| s.get_addr_size()) as u32;
    if size != ret.size && size != address_size {
        return false;
    }
    if size > ret.size {
        return false;
    }
    let low = ret.offset
        + if space.is_big_endian() {
            (ret.size - size) as u64
        } else {
            0
        };
    value.get_offset() == low
}

fn collect(
    data: &Funcdata,
    vn: VarnodeId,
    encoding: (i32, u64),
    seen: &mut HashSet<VarnodeId>,
    budget: &mut usize,
) -> Option<Destinations> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    if !seen.insert(vn) {
        return Some(Destinations::default());
    }
    let value = data.vbank().get(vn)?;
    if !(1..=8).contains(&value.get_size()) {
        return None;
    }
    if value.is_constant() {
        return Some(Destinations {
            constants: [value.get_offset()].into_iter().collect(),
            returns: false,
        });
    }
    if incoming_return(data, vn) {
        return Some(Destinations {
            constants: BTreeSet::new(),
            returns: true,
        });
    }
    let op = data.obank().get(value.get_def()?)?;
    let mut result = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            collect(data, op.get_in(0)?, encoding, seen, budget)?
        }
        OpCode::CPUI_MULTIEQUAL => {
            let mut union = Destinations::default();
            for i in 0..op.num_input() {
                let part = collect(data, op.get_in(i)?, encoding, seen, budget)?;
                union.constants.extend(part.constants);
                union.returns |= part.returns;
            }
            union
        }
        OpCode::CPUI_INT_AND => {
            let rhs = data.vbank().get(op.get_in(1)?)?;
            if !rhs.is_constant() {
                return None;
            }
            let bits = rhs.get_offset() & calc_mask(value.get_size());
            if bits != calc_mask(value.get_size())
                && (value.get_size() > encoding.0 || bits != (encoding.1 & calc_mask(value.get_size())))
            {
                return None;
            }
            let next_encoding = if bits == calc_mask(value.get_size()) {
                encoding
            } else {
                (encoding.0, u64::MAX)
            };
            let mut part = collect(data, op.get_in(0)?, next_encoding, seen, budget)?;
            part.constants = part.constants.into_iter().map(|v| v & bits).collect();
            part
        }
        OpCode::CPUI_SUBPIECE => {
            let shift = data.vbank().get(op.get_in(1)?)?;
            if !shift.is_constant() || shift.get_offset() != 0 {
                return None;
            }
            let mut part = collect(data, op.get_in(0)?, encoding, seen, budget)?;
            if part.returns
                && value.get_size() as u32 != data.get_address().get_space()?.get_addr_size()
            {
                return None;
            }
            part.constants = part
                .constants
                .into_iter()
                .map(|v| v & calc_mask(value.get_size()))
                .collect();
            part
        }
        _ => return None,
    };
    result.constants = result
        .constants
        .into_iter()
        .map(|v| v & calc_mask(value.get_size()))
        .collect();
    Some(result)
}

/// Give the caller-return state a defined selector value without bypassing any
/// dispatcher operations or changing other readers of the return address.
/// Labels and guards must have been recovered before changing the selector.
pub(crate) fn normalize_return_selector(data: &mut Funcdata, table: &JumpTable) -> bool {
    let Some(branch) = table.get_indirect_op() else { return false };
    let Some(encoding) = table.kuna_target_encoding() else { return false };
    let Some(root) = data.obank().get(branch).and_then(|op| op.get_in(0)) else {
        return false;
    };
    let budget = 128.max(4 * table.address_table().len());
    let Some(proof) = destinations(data, root, encoding, budget) else { return false };
    // A tagged selector and its byte destination can differ. Check the pair
    // the switch will actually use, and reserve every case label for its case.
    let mut labels = BTreeSet::new();
    let mut matched = BTreeSet::new();
    for (i, address) in table.address_table().iter().enumerate() {
        let label = table.get_label_by_index(i as i32);
        if label == crate::jumptable::NO_LABEL { continue; }
        labels.insert(label);
        if address.get_space().is_some_and(|space| {
            crate::kuna_constselectjump::target_offset(label, space.get_word_size(), data.get_arch().funcptr_align)
                == address.get_offset()
        }) {
            matched.insert(label);
        }
    }
    if !proof.returns || proof.constants.is_empty() || !proof.constants.is_subset(&matched) {
        return false;
    }
    let size = data.vbank().get(root).unwrap().get_size();
    let mask = encoding.1 & calc_mask(size);
    let sentinel = (0..=labels.len() as u64).find_map(|ordinal| {
        let mut value = 0;
        let mut source = ordinal;
        let mut bits = mask;
        while bits != 0 {
            let bit = bits & bits.wrapping_neg();
            if source & 1 != 0 { value |= bit; }
            source >>= 1;
            bits &= bits - 1;
        }
        (source == 0 && !labels.contains(&value)).then_some(value)
    });
    let Some(sentinel) = sentinel else { return false };
    let replacement = clone_selector(data, root, sentinel, &mut HashMap::new());
    data.op_set_input(branch, replacement, 0).expect("validated branch selector");
    true
}

fn clone_selector(
    data: &mut Funcdata,
    vn: VarnodeId,
    sentinel: u64,
    copies: &mut HashMap<VarnodeId, VarnodeId>,
) -> VarnodeId {
    if let Some(&copy) = copies.get(&vn) { return copy; }
    let value = data.vbank().get(vn).expect("validated selector");
    if value.is_constant() { return vn; }
    let size = value.get_size();
    if incoming_return(data, vn) { return data.new_constant(size, sentinel); }
    let def = value.get_def().expect("validated selector definition");
    let producer = data.obank().get(def).unwrap();
    let code = producer.code();
    let address = producer.get_addr().clone();
    let block = producer.get_parent().expect("live selector producer");
    let inputs: Vec<_> = (0..producer.num_input()).map(|i| producer.get_in(i).unwrap()).collect();
    let op = data.new_op(inputs.len() as i32, address);
    data.op_set_opcode_code(op, code);
    let out = data.new_unique_out(size, op).expect("selector output");
    copies.insert(vn, out);
    for (slot, input) in inputs.into_iter().enumerate() {
        let copy = clone_selector(data, input, sentinel, copies);
        data.op_set_input(op, copy, slot as i32).expect("selector input");
    }
    if code == OpCode::CPUI_MULTIEQUAL {
        data.op_insert_begin(op, block);
    } else {
        data.op_insert_before(op, def);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{ArchContext, BlockId};
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use std::rc::Rc;

    fn function(size: u32, big: bool) -> Funcdata {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        let flags = if big { addrspace_flags::big_endian } else { 0 };
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            big,
            size,
            1,
            2,
            flags,
            1,
            1,
        ));
        let reg = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            big,
            4,
            1,
            3,
            flags,
            1,
            1,
        ));
        manager.insert_space(ram.clone()).unwrap();
        manager.insert_space(reg.clone()).unwrap();
        let mut arch = ArchContext::new(manager);
        arch.indirect_return_addr = Some(kuna_num::pcoderaw::VarnodeData {
            space: Some(reg),
            offset: 0xf8,
            size: 8,
        });
        Funcdata::new(
            "dispatch",
            "dispatch",
            Rc::new(arch),
            Address::new(ram, 0x1000),
            0x10000000,
            0x100,
        )
        .unwrap()
    }
    fn input(fd: &mut Funcdata, offset: u64, size: i32) -> VarnodeId {
        let space = fd
            .get_arch()
            .manage()
            .get_space_by_name("register")
            .unwrap()
            .clone();
        let vn = fd.new_varnode(size, &Address::new(space, offset), None);
        fd.set_input_varnode(vn).unwrap()
    }
    fn operation(
        fd: &mut Funcdata,
        pc: u64,
        code: OpCode,
        size: i32,
        ins: &[VarnodeId],
    ) -> (OpId, Option<VarnodeId>) {
        let op = fd.new_op(
            ins.len() as i32,
            Address::new(fd.get_address().get_space().unwrap().clone(), pc),
        );
        fd.op_set_opcode(op, crate::typeop::type_op_for(code));
        for (i, &v) in ins.iter().enumerate() {
            fd.op_set_input(op, v, i as i32).unwrap();
        }
        let out = (size != 0).then(|| fd.new_unique(size, None));
        if let Some(v) = out {
            fd.op_set_output(op, v).unwrap();
        }
        (op, out)
    }
    fn attach(fd: &mut Funcdata, block: BlockId, op: OpId) {
        fd.obank_mut().mark_alive(op);
        fd.bb_insert_op(op, block, None);
    }
    #[test]
    fn only_new_code_entering_old_code_requests_another_stage() {
        let mut fd = function(8, false);
        let ram = fd.get_address().get_space().unwrap().clone();
        let code = |fd: &mut Funcdata, pc: u64| fd.new_code_ref(&Address::new(ram.clone(), pc));
        let zero = fd.new_constant(8, 0);
        let (old, _) = operation(&mut fd, 0x1000, OpCode::CPUI_COPY, 8, &[zero]);
        let (dispatch, _) = operation(&mut fd, 0x1004, OpCode::CPUI_BRANCHIND, 0, &[zero]);
        let mark = fd.obank().get_uniq_id();
        let at = move |to: &Address| match to.get_offset() {
            0x1000 => Some(old),
            0x1004 => Some(dispatch),
            _ => None,
        };
        let none = |_: OpId| None;
        assert!(!reentered_since(&fd, mark, at, none));
        let (step, _) = operation(&mut fd, 0x1010, OpCode::CPUI_COPY, 8, &[zero]);
        assert!(!reentered_since(&fd, mark, at, none));
        assert!(reentered_since(&fd, mark, at, |op| (op == step).then_some(old)));
        let forward = code(&mut fd, 0x1020);
        let (jump, _) = operation(&mut fd, 0x1014, OpCode::CPUI_BRANCH, 0, &[forward]);
        let (leave, _) = operation(&mut fd, 0x1020, OpCode::CPUI_RETURN, 0, &[zero]);
        let ends = |op: OpId| (op == jump || op == leave).then_some(old);
        assert!(!reentered_since(&fd, mark, at, ends));
        let back = code(&mut fd, 0x1004);
        let (retry, _) = operation(&mut fd, 0x1024, OpCode::CPUI_CBRANCH, 0, &[back, zero]);
        assert!(reentered_since(&fd, mark, at, none));
        assert!(!reentered_since(&fd, fd.obank().get_uniq_id(), at, |op| (op == retry).then_some(old)));
    }

    #[test]
    fn only_edges_that_reach_a_dispatch_are_its_reentries() {
        let mut fd = function(8, false);
        let ram = fd.get_address().get_space().unwrap().clone();
        let zero = fd.new_constant(8, 0);
        let head = fd.new_code_ref(&Address::new(ram.clone(), 0x1000));
        let (a_pre, _) = operation(&mut fd, 0x1000, OpCode::CPUI_COPY, 8, &[zero]);
        let (a, _) = operation(&mut fd, 0x1004, OpCode::CPUI_BRANCHIND, 0, &[zero]);
        let (b_pre, _) = operation(&mut fd, 0x1010, OpCode::CPUI_COPY, 8, &[zero]);
        let (b, _) = operation(&mut fd, 0x1014, OpCode::CPUI_BRANCHIND, 0, &[zero]);
        let (other, _) = operation(&mut fd, 0x1020, OpCode::CPUI_COPY, 8, &[zero]);
        let (leave, _) = operation(&mut fd, 0x1024, OpCode::CPUI_RETURN, 0, &[zero]);
        let (back, _) = operation(&mut fd, 0x1030, OpCode::CPUI_BRANCH, 0, &[head]);
        let at = move |to: &Address| match to.get_offset() {
            0x1000 => Some(a_pre),
            _ => None,
        };
        let next = move |op: OpId| match op {
            o if o == a_pre => Some(a),
            o if o == b_pre => Some(b),
            o if o == other => Some(leave),
            _ => None,
        };
        let both: BTreeSet<OpId> = [a, b].into_iter().collect();
        let edges = [(back, a_pre), (leave, other), (back, b_pre)];
        let reached = reaching_edges(&fd, &both, &edges, at, next);
        assert_eq!(reached.get(&a).map(Vec::as_slice), Some(&edges[..1]));
        assert_eq!(reached.get(&b).map(Vec::as_slice), Some(&edges[2..]));
        assert_eq!(reached.len(), 2, "the edge into unrelated code reaches no dispatch");
    }

    #[test]
    fn only_a_reentry_retry_bypasses_the_select_option_and_keeps_its_targets() {
        let ram = function(8, false).get_address().get_space().unwrap().clone();
        let mut jt = JumpTable::new(Address::new(ram.clone(), 0x1004));
        jt.increment_recovery_count();
        jt.increment_recovery_count();
        use crate::kuna_constselectjump::ConstSelectMode;
        assert!(!jt.kuna_admits_const_select(ConstSelectMode::Reentry) && jt.kuna_admits_const_select(ConstSelectMode::On));
        jt.kuna_retry_flow();
        assert!(jt.kuna_admits_const_select(ConstSelectMode::Reentry));
        assert!(!jt.kuna_keeps_recovered_targets());
        jt.kuna_push_entry(Address::new(ram, 0x1040), 0, 0x1040);
        assert!(jt.kuna_keeps_recovered_targets());
        jt.mark_complete();
        assert!(!jt.kuna_keeps_recovered_targets() && jt.kuna_admits_const_select(ConstSelectMode::Reentry));
    }

    #[test]
    fn only_full_or_pointer_width_low_return_inputs_supply_a_return() {
        for big in [false, true] {
            for address_size in [4, 8] {
                for size in [1, 2, 4, 8] {
                    let mut fd = function(address_size, big);
                    let low = 0xf8 + if big { 8 - size as u64 } else { 0 };
                    let vn = input(&mut fd, low, size);
                    assert_eq!(
                        incoming_return(&fd, vn),
                        size == 8 || size as u32 == address_size
                    );
                }
            }
        }
    }
    #[test]
    fn transfer_mask_is_read_from_raw_storage_before_ssa() {
        let mut fd = function(8, false);
        let ra = input(&mut fd, 0xf8, 8);
        let bits = fd.new_constant(8, !1);
        let (_, out) = operation(&mut fd, 0x1010, OpCode::CPUI_INT_AND, 8, &[ra, bits]);
        let location = fd.vbank().get(out.unwrap()).unwrap().get_addr().clone();
        let raw_read = fd.new_varnode(8, &location, None);
        let (branch, _) = operation(&mut fd, 0x1010, OpCode::CPUI_BRANCHIND, 0, &[raw_read]);
        assert_eq!(target_encoding(&fd, branch), Some((8, !1)));
        let destructive = fd.new_constant(8, 0xfffe);
        let (_, out) = operation(&mut fd, 0x1008, OpCode::CPUI_INT_AND, 8, &[ra, destructive]);
        assert!(collect(&fd, out.unwrap(), (8, !1), &mut HashSet::new(), &mut 128).is_none());
        assert!(collect(&fd, ra, (8, !1), &mut HashSet::new(), &mut 0).is_none());
    }
    #[test]
    fn cyclic_target_proofs_need_a_seed_and_refuse_an_unknown_input() {
        let mut fd = function(8, false);
        let ra = input(&mut fd, 0xf8, 8);
        let constant = fd.new_constant(8, 0x1040);
        let (phi, out) = operation(
            &mut fd,
            0x1010,
            OpCode::CPUI_MULTIEQUAL,
            8,
            &[constant, ra, ra],
        );
        let out = out.unwrap();
        fd.op_set_input(phi, out, 2).unwrap();
        let proof = collect(&fd, out, (8, u64::MAX), &mut HashSet::new(), &mut 128).unwrap();
        assert!(proof.returns);
        assert_eq!(proof.constants, [0x1040].into_iter().collect());
        assert!(collect(&fd, out, (8, u64::MAX), &mut HashSet::new(), &mut 1).is_none());
        let unknown = input(&mut fd, 0xf0, 8);
        fd.op_set_input(phi, unknown, 0).unwrap();
        assert!(collect(&fd, out, (8, u64::MAX), &mut HashSet::new(), &mut 128).is_none());
        fd.op_set_all_input(phi, &[out, out, out]).unwrap();
        let proof = collect(&fd, out, (8, u64::MAX), &mut HashSet::new(), &mut 128).unwrap();
        assert!(!proof.returns && proof.constants.is_empty());
    }

    #[test]
    fn a_raw_negated_left_operand_supplies_the_transfer_mask() {
        let mut fd = function(8, false);
        let ra = input(&mut fd, 0xf8, 8);
        let two = fd.new_constant(8, 2);
        let (_, negative) = operation(&mut fd, 0x1010, OpCode::CPUI_INT_2COMP, 8, &[two]);
        let location = fd
            .vbank()
            .get(negative.unwrap())
            .unwrap()
            .get_addr()
            .clone();
        let raw = fd.new_varnode(8, &location, None);
        let (_, target) = operation(&mut fd, 0x1010, OpCode::CPUI_INT_AND, 8, &[raw, ra]);
        let location = fd.vbank().get(target.unwrap()).unwrap().get_addr().clone();
        let raw = fd.new_varnode(8, &location, None);
        let (branch, _) = operation(&mut fd, 0x1010, OpCode::CPUI_BRANCHIND, 0, &[raw]);
        assert_eq!(target_encoding(&fd, branch), Some((8, !1)));
    }
    fn dispatch(variant: &str) -> (Funcdata, JumpTable, Address) {
        let mut fd = function(8, false);
        let root = fd.bblocks_root_pub();
        let p0 = fd.bblocks_mut().new_block_basic(root);
        let p1 = fd.bblocks_mut().new_block_basic(root);
        let block = fd.bblocks_mut().new_block_basic(root);
        let address = Address::new(
            fd.get_address().get_space().unwrap().clone(),
            if variant == "prefix" { 0x100c } else { 0x1010 },
        );
        fd.set_basic_block_range(block, &address, &address);
        fd.bblocks_mut().add_edge(p0, block);
        fd.bblocks_mut().add_edge(p1, block);
        let constant = fd.new_constant(8, 0x1040);
        let ret = input(&mut fd, if variant == "unknown" { 0xf0 } else { 0xf8 }, 8);
        let (phi, target) = operation(
            &mut fd,
            0x1010,
            OpCode::CPUI_MULTIEQUAL,
            8,
            &[constant, ret],
        );
        attach(&mut fd, block, phi);
        let target = target.unwrap();
        if variant == "pure" || variant == "returned" {
            let (step, bumped) = operation(&mut fd, 0x1010, OpCode::CPUI_INT_ADD, 8, &[ret, constant]);
            attach(&mut fd, block, step);
            if variant == "returned" {
                let (leave, _) = operation(&mut fd, 0x1011, OpCode::CPUI_RETURN, 0, &[constant, bumped.unwrap()]);
                attach(&mut fd, p0, leave);
            }
        }
        if variant == "effect" {
            let (effect, _) = operation(
                &mut fd,
                0x1010,
                OpCode::CPUI_STORE,
                0,
                &[constant, constant, constant],
            );
            attach(&mut fd, block, effect);
        }
        let (branch, _) = operation(&mut fd, 0x1010, OpCode::CPUI_BRANCHIND, 0, &[target]);
        attach(&mut fd, block, branch);
        let (first, _) = operation(&mut fd, 0x1000, OpCode::CPUI_BRANCH, 0, &[constant]);
        attach(&mut fd, p0, first);
        let code = if variant == "conditional" {
            OpCode::CPUI_CBRANCH
        } else {
            OpCode::CPUI_BRANCH
        };
        if variant == "likely" || variant == "internal" {
            let skip = if variant == "likely" {
                fd.new_code_ref(&Address::new(fd.get_address().get_space().unwrap().clone(), 0x1030))
            } else {
                constant
            };
            operation(&mut fd, 0x1020, OpCode::CPUI_CBRANCH, 0, &[skip, constant]);
        }
        let dispatcher =
            fd.new_code_ref(&Address::new(fd.get_address().get_space().unwrap().clone(), 0x1010));
        let (last, _) = operation(&mut fd, 0x1020, code, 0, &[dispatcher]);
        attach(&mut fd, p1, last);
        if variant == "outside-reader" {
            let (reader, _) = operation(&mut fd, 0x1020, OpCode::CPUI_COPY, 8, &[target]);
            attach(&mut fd, p0, reader);
        }
        let site = fd.obank().get(last).unwrap().get_addr().clone();
        if variant == "override" {
            fd.get_override_mut()
                .insert_flow_override(site.clone(), crate::overrides::flow_type::BRANCH);
        }
        let mut jt = JumpTable::new(fd.obank().get(branch).unwrap().get_addr().clone());
        jt.set_indirect_op_addr(branch, fd.obank().get(branch).unwrap().get_addr().clone());
        jt.kuna_set_target_encoding(Some((8, u64::MAX)));
        jt.kuna_set_trivial_model();
        let target = Address::new(
            fd.get_address().get_space().unwrap().clone(),
            if variant == "missing-target" {
                0x1050
            } else {
                0x1040
            },
        );
        jt.kuna_push_entry(target, 0, 0x1040);
        (fd, jt, site)
    }
    #[test]
    fn selector_normalization_preserves_all_operations_and_shared_readers() {
        for variant in ["positive", "effect", "conditional", "unknown", "override",
                        "missing-target", "prefix", "likely", "pure", "returned", "internal",
                        "outside-reader"] {
            let (mut fd, jt, site) = dispatch(variant);
            let branch = jt.get_indirect_op().unwrap();
            let old_root = fd.obank().get(branch).unwrap().get_in(0).unwrap();
            let operations: Vec<_> = fd.obank().iter_alive().collect();
            assert_eq!(normalize_return_selector(&mut fd, &jt),
                       !matches!(variant, "unknown" | "missing-target"), "{variant}");
            assert_eq!(fd.get_override().get_flow_override(&site),
                       if variant == "override" { crate::overrides::flow_type::BRANCH }
                       else { crate::overrides::flow_type::NONE });
            for op in operations { assert!(!fd.obank().get(op).unwrap().is_dead()); }
            if !matches!(variant, "unknown" | "missing-target") {
                let new_root = fd.obank().get(branch).unwrap().get_in(0).unwrap();
                assert_ne!(old_root, new_root);
                let proof = destinations(&fd, new_root, (8, u64::MAX), 128).unwrap();
                assert!(!proof.returns);
                assert_eq!(proof.constants, [0, 0x1040].into_iter().collect());
                assert!(!normalize_return_selector(&mut fd, &jt));
                assert!(destinations(&fd, old_root, (8, u64::MAX), 128).unwrap().returns);
            }
        }
    }
    #[test]
    fn a_reentered_select_admits_the_incoming_return_as_its_last_unlabelled_address() {
        use crate::jumptable::JumpModel;
        for variant in ["return", "unknown", "constants", "not-admitted"] {
            let mut fd = function(8, false);
            let near = fd.new_constant(8, 0x1040);
            let far = fd.new_constant(8, 0x1080);
            let other = match variant {
                "return" | "not-admitted" => input(&mut fd, 0xf8, 8),
                "unknown" => input(&mut fd, 0xf0, 8),
                _ => far,
            };
            let (_, target) = operation(&mut fd, 0x1010, OpCode::CPUI_MULTIEQUAL, 8, &[near, far, other]);
            let (branch, _) = operation(&mut fd, 0x1010, OpCode::CPUI_BRANCHIND, 0, &[target.unwrap()]);
            let mut model = crate::kuna_constselectjump::JumpModelConstSelect::new();
            if variant != "not-admitted" {
                model.kuna_admit_return(Some((8, u64::MAX)));
            }
            let found = model.recover_model(&mut fd, branch, 0, 64).unwrap();
            assert_eq!(found, variant == "return" || variant == "constants", "{variant}");
            assert_eq!(model.kuna_returns_to_caller(), variant == "return", "{variant}");
            if variant == "return" {
                let mut addresses = Vec::new();
                model.build_addresses(&fd, branch, &mut addresses, None, None).unwrap();
                let offsets: Vec<_> = addresses.iter().map(|a| a.get_offset()).collect();
                assert_eq!(offsets, [0x1040, 0x1080, 0x1011]);
                let mut labels = Vec::new();
                model.build_labels(&mut fd, &mut addresses, &mut labels, &model.clone()).unwrap();
                assert_eq!(labels, [0x1040, 0x1080, crate::jumptable::NO_LABEL]);
                assert_eq!(model.get_table_size(), 3);
            }
        }
    }
}
