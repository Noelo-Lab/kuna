//! (kuna `returnbranch`) Preserve a computed transfer that the instruction spec
//! classified as RETURN.
//!
//! A bounded straight-line entry prefix can prove the target is a constant,
//! rather than the incoming return address. Reclassify only the flow kind;
//! ordinary SSA/jump-table recovery must still prove its destinations, including
//! any backedges discovered later. Unknown inputs, partial writes, calls and
//! stores into a fact's space never supply constants. Delay-slot writes are
//! evaluated in emitted p-code order.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::address::{calc_mask, Address};
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Loc(i32, u64, i32);

impl Loc {
    fn overlaps(self, rhs: Self) -> bool {
        self.0 == rhs.0
            && self.1 < rhs.1.saturating_add(rhs.2 as u64)
            && rhs.1 < self.1.saturating_add(self.2 as u64)
    }
}

fn location(data: &Funcdata, vn: VarnodeId) -> Option<Loc> {
    let v = data.vbank().get(vn)?;
    Some(Loc(v.get_space().get_index(), v.get_offset(), v.get_size()))
}

fn read_known(loc: Loc, big: bool, known: &[(Loc, u64)]) -> Option<u64> {
    known.iter().find_map(|&(written, value)| {
        if written.0 != loc.0 || written.1 > loc.1 || written.2 < loc.2 {
            return None;
        }
        let offset = loc.1 - written.1;
        let slack = (written.2 - loc.2) as u64;
        if offset > slack {
            return None;
        }
        let byte = if big { slack - offset } else { offset };
        Some((value >> (8 * byte)) & calc_mask(loc.2))
    })
}

/// Constant storage facts along a straight-line run of raw p-code. Every
/// bounded target proof uses this one evaluator, so they agree on what each
/// operation preserves: a call or an unknown user operation stops the proof, a
/// store forgets every fact in the space it writes, and a write of any other
/// kind forgets the storage it overlaps unless its value is proven.
pub(crate) struct Prefix<'a, F> {
    data: &'a Funcdata,
    known: Vec<(Loc, u64)>,
    previous: Option<Address>,
    mode_switch: F,
    /// Image reads of table loads: space, address, size.
    loads: Vec<(i32, u64, i32)>,
}

impl<'a, F: Fn(u64) -> bool> Prefix<'a, F> {
    /// The function entry: tracked context values are known, except any
    /// overlapping the incoming return address. `mode_switch` names user
    /// operations that only select an instruction set and leave every register
    /// unchanged.
    fn at_entry(data: &'a Funcdata, mode_switch: F) -> Self {
        let incoming_return = data
            .get_arch()
            .indirect_return_addr
            .as_ref()
            .and_then(|v| Some(Loc(v.space.as_ref()?.get_index(), v.offset, v.size as i32)));
        let known = data
            .get_arch()
            .get_tracked_set(data.get_address())
            .iter()
            .take(64)
            .filter_map(|t| {
                let size = t.loc.size as i32;
                let loc = Loc(t.loc.space.as_ref()?.get_index(), t.loc.offset, size);
                if incoming_return.is_some_and(|ret| ret.overlaps(loc)) {
                    return None;
                }
                (1..=8).contains(&size).then(|| (loc, t.val & calc_mask(size)))
            })
            .collect();
        Prefix { data, known, previous: None, mode_switch, loads: Vec::new() }
    }

    /// Temporaries do not survive their instruction.
    fn enter(&mut self, id: OpId) {
        let Some(op) = self.data.obank().get(id) else { return };
        if self.previous.as_ref() != Some(op.get_addr()) {
            let manage = self.data.get_arch().manage();
            self.known.retain(|(loc, _)| {
                manage.get_space(loc.0).is_some_and(|s| s.get_type() != spacetype::IPTR_INTERNAL)
            });
            self.previous = Some(op.get_addr().clone());
        }
    }

    /// The proven value of a one- to eight-byte input of operation `id`.
    pub(crate) fn input(&mut self, id: OpId, slot: i32) -> Option<u64> {
        self.enter(id);
        let vn = self.data.obank().get(id)?.get_in(slot)?;
        self.value(vn)
    }

    fn value(&self, vn: VarnodeId) -> Option<u64> {
        let node = self.data.vbank().get(vn)?;
        if !(1..=8).contains(&node.get_size()) {
            return None;
        }
        if node.is_constant() {
            return Some(node.get_offset() & calc_mask(node.get_size()));
        }
        read_known(location(self.data, vn)?, node.get_space().is_big_endian(), &self.known)
    }

    /// Apply operation `id`, which must not transfer control. `false` means
    /// the operation may change anything and the proof stops.
    pub(crate) fn execute(&mut self, id: OpId) -> bool {
        self.execute_with_table_loads(id, false)
    }

    fn execute_with_table_loads(&mut self, id: OpId, table_loads: bool) -> bool {
        self.enter(id);
        let Some(op) = self.data.obank().get(id) else { return false };
        match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => return false,
            OpCode::CPUI_CALLOTHER => {
                return op.get_out().is_none()
                    && op.get_in(0).and_then(|v| self.value(v)).is_some_and(&self.mode_switch)
                    && op.get_in(1).and_then(|v| self.value(v)) == Some(0);
            }
            OpCode::CPUI_STORE => {
                let space = op.get_in(0).and_then(|v| self.data.vbank().get(v));
                match space.filter(|v| v.is_constant()) {
                    Some(space) => {
                        let index = space.get_offset();
                        self.known.retain(|(loc, _)| loc.0 as u64 != index);
                    }
                    None => self.known.clear(),
                }
                return true;
            }
            _ => {}
        }
        let Some(out) = op.get_out().and_then(|v| location(self.data, v)) else {
            return true;
        };
        let result = (|| {
            if !(1..=8).contains(&out.2) { return None; }
            if table_loads && op.code() == OpCode::CPUI_LOAD {
                let index = i32::try_from(self.value(op.get_in(0)?)?).ok()?;
                let space = self.data.get_arch().manage().get_space(index)?;
                let at = self.value(op.get_in(1)?)?;
                self.loads.push((index, at, out.2));
                let ptr = at.checked_mul(space.get_word_size() as u64)?;
                let mut emulator = crate::kuna_emulatefunction::EmulateFunction::new(self.data);
                emulator.set_readonly_image_required(true);
                return emulator.get_load_image_value(space, ptr, out.2).ok();
            }
            if !matches!(
                    op.code(),
                    OpCode::CPUI_COPY
                        | OpCode::CPUI_INT_ZEXT
                        | OpCode::CPUI_INT_SEXT
                        | OpCode::CPUI_INT_2COMP
                        | OpCode::CPUI_INT_NEGATE
                        | OpCode::CPUI_INT_EQUAL
                        | OpCode::CPUI_INT_NOTEQUAL
                        | OpCode::CPUI_INT_ADD
                        | OpCode::CPUI_INT_SUB
                        | OpCode::CPUI_INT_MULT
                        | OpCode::CPUI_INT_AND
                        | OpCode::CPUI_INT_OR
                        | OpCode::CPUI_INT_XOR
                        | OpCode::CPUI_INT_LEFT
                        | OpCode::CPUI_INT_RIGHT
                        | OpCode::CPUI_SUBPIECE
                )
            {
                return None;
            }
            let behavior = self.data.get_arch().op_behavior(op.code())?;
            let size = self.data.vbank().get(op.get_in(0)?)?.get_size();
            let x = self.value(op.get_in(0)?)?;
            if behavior.is_unary() {
                behavior.evaluate_unary(out.2, size, x).ok()
            } else {
                behavior.evaluate_binary(out.2, size, x, self.value(op.get_in(1)?)?).ok()
            }
        })();
        self.known.retain(|(loc, _)| !loc.overlaps(out));
        if let Some(value) = result {
            self.known.push((out, value & calc_mask(out.2)));
        }
        true
    }
}

/// The user-op name a SLEIGH spec gives the marker that only selects an
/// instruction set, in every language that declares one. Named rather than
/// gated on an architecture id, which the Ghidra front end spells as a session
/// number.
pub(crate) fn names_isa_mode_switch(name: &[u8]) -> bool {
    name == b"setISAMode"
}

/// Is `first` the start of the function, give or take decoded instructions
/// that emitted no p-code at all? Such an instruction cannot have changed
/// anything, so the prefix beginning after it is still the entry prefix.
fn entry_prefix_starts_at(
    data: &Funcdata,
    visited: &crate::flow::VisitedMap,
    first: &Address,
) -> bool {
    let mut at = data.get_address().clone();
    while &at < first {
        let Some(stat) = visited.get(&at) else {
            return false;
        };
        if !stat.seqnum.get_addr().is_invalid() || stat.size <= 0 {
            return false;
        }
        at = &at + stat.size as i64;
    }
    &at == first
}

fn transfers_control(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN
    )
}

/// Does the entry prefix compute a constant target for this real RETURN? Zero
/// is not one: `move ra, zero` marks the end of a backtrace, and a function
/// start placed on such a delay slot would otherwise turn its real return into
/// a jump to address zero.
pub(crate) fn computed_target(
    data: &Funcdata,
    target: OpId,
    visited: &crate::flow::VisitedMap,
    mode_switch: impl Fn(u64) -> bool,
) -> bool {
    let Some(ret) = data.obank().get(target) else {
        return false;
    };
    if !data.get_arch().return_branch
        || ret.code() != OpCode::CPUI_RETURN
        || ret.get_halt_type() != 0
    {
        return false;
    }
    let Some(input) = ret.get_in(0).and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    if input.is_constant()
        || data.get_override().get_flow_override(ret.get_addr())
            != crate::overrides::flow_type::NONE
    {
        return false;
    }
    let mut prefix = Prefix::at_entry(data, mode_switch);
    for (i, id) in data.obank().iter_dead().take(256).enumerate() {
        let op = data.obank().get(id).expect("live raw op");
        if i == 0 && !entry_prefix_starts_at(data, visited, op.get_addr()) {
            return false;
        }
        if id == target {
            return prefix.input(id, 0).is_some_and(|to| to != 0);
        }
        if transfers_control(op.code()) || !prefix.execute(id) {
            return false;
        }
    }
    false
}

/// The operations an edge replays: `from`'s block up to `from`, then the
/// dispatch instruction's operations before the jump, or for an indexed
/// bypass the straight-line suffix starting at `table_entry`.
fn edge_path(
    data: &Funcdata,
    from: OpId,
    dispatch: OpId,
    table_entry: Option<OpId>,
) -> Option<(Vec<OpId>, Vec<OpId>)> {
    let mut run = Vec::new();
    let mut cursor = from;
    loop {
        let op = data.obank().get(cursor)?;
        run.push(cursor);
        if op.is_block_start() {
            break;
        }
        if run.len() >= 256 {
            return None;
        }
        cursor = data.obank().dead_prev(cursor)?;
    }
    run.reverse();
    let address = data.obank().get(dispatch)?.get_addr().clone();
    let mut slot = Vec::new();
    let mut cursor = dispatch;
    while table_entry != Some(cursor) {
        let Some(prev) = data.obank().dead_prev(cursor) else {
            if table_entry.is_some() { return None; }
            break;
        };
        if table_entry.is_none() && data.obank().get(prev)?.get_addr() != &address {
            break;
        }
        slot.push(prev);
        if slot.len() >= 64 {
            return None;
        }
        cursor = prev;
    }
    slot.reverse();
    let transfer = |id: OpId| data.obank().get(id).map(|op| transfers_control(op.code()));
    if run.iter().any(|&id| id != from && transfer(id) != Some(false))
        || slot.iter().any(|&id| transfer(id) != Some(false))
    {
        return None;
    }
    Some((run, slot))
}

/// An exact local proof of the target after following `from` back into a
/// dispatch instruction: the run from `from`'s block start, then the dispatch
/// instruction's own operations before the jump, through [`Prefix`]. For an
/// indexed bypass, replay the actual straight-line suffix starting at its
/// entry, admitting only immutable image reads at known addresses.
pub(crate) fn target_on_edge(
    data: &Funcdata,
    from: OpId,
    dispatch: OpId,
    table_entry: Option<OpId>,
    mode_switch: impl Fn(u64) -> bool,
) -> Option<u64> {
    let (run, slot) = edge_path(data, from, dispatch, table_entry)?;
    let mut prefix = Prefix { data, known: Vec::new(), previous: None, mode_switch, loads: Vec::new() };
    for id in run {
        if !prefix.execute(id) {
            return None;
        }
    }
    for id in slot {
        if !prefix.execute_with_table_loads(id, table_entry.is_some()) {
            return None;
        }
    }
    prefix.input(dispatch, 0)
}

/// Index values one bounded operation may take before the proof gives up.
const MAX_INDEX_VALUES: u64 = 1024;

/// The values an operation can produce whatever its inputs: an AND with a
/// small constant mask, or a one-byte load or zero extension.
fn index_bound(data: &Funcdata, id: OpId) -> Option<u64> {
    let op = data.obank().get(id)?;
    let out = data.vbank().get(op.get_out()?)?;
    let input = |slot| data.vbank().get(op.get_in(slot)?);
    match op.code() {
        OpCode::CPUI_INT_AND => (0..2)
            .filter_map(|slot| input(slot).filter(|v| v.is_constant()))
            .map(|v| v.get_offset() & calc_mask(out.get_size()))
            .min()
            .filter(|&mask| mask < MAX_INDEX_VALUES),
        OpCode::CPUI_LOAD if out.get_size() == 1 => Some(0xff),
        OpCode::CPUI_INT_ZEXT if input(0)?.get_size() == 1 => Some(0xff),
        _ => None,
    }
}

/// What an indexed bypass can bring to its loaded table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Bypass {
    /// Every value it can carry reads a recovered row and reaches a recovered
    /// destination, so the recovered table already describes the edge.
    Rows,
    /// It can read memory outside the recovered rows or arrive with a target
    /// no row supplies.
    Escapes,
    /// The local replay cannot tell.
    Unknown,
}

/// One replay of an edge for one value of the enumerated operation.
enum Replay {
    Rows,
    Leaves,
    Escapes,
    Unproved,
}

/// Classifies what an indexed bypass brings to its table.
/// - `rows` checks one replay's table loads, in order, against the recovered
///   rows; `known` names the recovered destinations.
/// - The replay starts after the last call on `from`'s block.
/// - With nothing proven, the bounded operations on the index side of the row
///   load are enumerated over their whole range, nearest the jump first. A
///   bounded operation is an AND with a small constant mask, or a one-byte load
///   or zero extension.
/// - When `taken` is set, `from` is a conditional branch. A value whose proven
///   condition leaves by the other edge does not count; an unproven condition
///   may take this edge.
/// - A replay escapes when it reads a table load outside the rows, or when its
///   proven target comes from no row. An enumerated value counts as an escape
///   only at zero: a wider bound than the table may still be unreachable.
/// - `budget` bounds the replayed operations.
pub(crate) fn classify_indexed_edge(
    data: &Funcdata,
    from: OpId,
    dispatch: OpId,
    entry: OpId,
    taken: Option<bool>,
    mode_switch: impl Fn(u64) -> bool + Copy,
    rows: impl Fn(&[(i32, u64, i32)]) -> bool,
    known: impl Fn(u64) -> bool,
    budget: &mut usize,
) -> Bypass {
    let Some((run, slot)) = edge_path(data, from, dispatch, Some(entry)) else { return Bypass::Unknown };
    let calls = |id: &OpId| data.obank().get(*id).is_some_and(|op| {
        matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER)
    });
    let run = &run[run.iter().rposition(calls).map_or(0, |at| at + 1)..];
    let path: Vec<(OpId, bool)> = run.iter().map(|&id| (id, false))
        .chain(slot.iter().map(|&id| (id, true))).collect();
    let mut replay = |seed: Option<(usize, u64)>| -> Replay {
        let mut prefix = Prefix { data, known: Vec::new(), previous: None, mode_switch, loads: Vec::new() };
        for (at, &(id, table)) in path.iter().enumerate() {
            let Some(left) = budget.checked_sub(1) else { return Replay::Unproved };
            *budget = left;
            if !prefix.execute_with_table_loads(id, table) {
                return Replay::Unproved;
            }
            if let Some((_, value)) = seed.filter(|&(seeded, _)| seeded == at) {
                let Some(out) = data.obank().get(id).and_then(|op| op.get_out()).and_then(|vn| location(data, vn))
                else { return Replay::Unproved };
                prefix.known.retain(|(loc, _)| !loc.overlaps(out));
                prefix.known.push((out, value & calc_mask(out.2)));
            }
            if let (Some(taken), false) = (taken, table) {
                if id == from && prefix.input(from, 1).is_some_and(|cond| (cond != 0) != taken) {
                    return Replay::Leaves;
                }
            }
        }
        if !prefix.loads.is_empty() && !rows(&prefix.loads) {
            return Replay::Escapes;
        }
        match prefix.input(dispatch, 0) {
            Some(target) if !prefix.loads.is_empty() => {
                if known(target) { Replay::Rows } else { Replay::Escapes }
            }
            Some(_) if seed.is_none() => Replay::Escapes,
            _ => Replay::Unproved,
        }
    };
    match replay(None) {
        Replay::Rows | Replay::Leaves => return Bypass::Rows,
        Replay::Escapes => return Bypass::Escapes,
        Replay::Unproved => {}
    }
    // Walk the target's dependencies back from the jump. Only operations behind
    // the row load are index candidates.
    let mut needed: Vec<Loc> = data.obank().get(dispatch).and_then(|op| op.get_in(0))
        .and_then(|vn| location(data, vn)).into_iter().collect();
    let mut seeds = Vec::new();
    let mut behind_row = false;
    for (at, &(id, table)) in path.iter().enumerate().rev() {
        let Some(op) = data.obank().get(id) else { return Bypass::Unknown };
        let Some(out) = op.get_out().and_then(|vn| location(data, vn)) else { continue };
        if !needed.iter().any(|loc| loc.overlaps(out)) {
            continue;
        }
        needed.retain(|loc| !out.overlaps(*loc) || loc.1 < out.1 || loc.1 + loc.2 as u64 > out.1 + out.2 as u64);
        let row_load = table && op.code() == OpCode::CPUI_LOAD;
        if behind_row && !row_load {
            if let Some(bound) = index_bound(data, id) {
                seeds.push((at, bound));
            }
        }
        behind_row |= row_load;
        for slot in 0..op.num_input() {
            if let Some(input) = op.get_in(slot).filter(|&vn| data.vbank().get(vn).is_some_and(|v| !v.is_constant())) {
                needed.extend(location(data, input));
            }
        }
    }
    for &(at, bound) in seeds.iter().take(3) {
        let mut reached = false;
        let mut complete = true;
        for value in 0..=bound {
            match replay(Some((at, value))) {
                Replay::Rows => reached = true,
                Replay::Leaves => {}
                Replay::Escapes if value == 0 => return Bypass::Escapes,
                Replay::Escapes | Replay::Unproved => {
                    complete = false;
                    break;
                }
            }
        }
        if complete && reached {
            return Bypass::Rows;
        }
    }
    Bypass::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use kuna_base::address::{Address, SeqNum};
    use kuna_base::space::{AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use std::rc::Rc;

    /// Every decoded instruction emitted p-code, so the first raw op is the
    /// entry's; the prefix map only matters in the tests that build one.
    fn proves(fd: &Funcdata, ret: OpId, mode_switch: impl Fn(u64) -> bool) -> bool {
        computed_target(fd, ret, &crate::flow::VisitedMap::new(), mode_switch)
    }

    fn function() -> Funcdata {
        function_with_known_return(false)
    }

    fn function_with_known_return(tracked: bool) -> Funcdata {
        function_with_option(tracked, false)
    }

    fn function_with_option(tracked: bool, off: bool) -> Funcdata {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            0,
            1,
            1,
        ));
        manager.insert_space(ram.clone()).unwrap();
        let mut arch = ArchContext::new(manager);
        arch.opbehaviors = vec![None; OpCode::CPUI_MAX as usize];
        use kuna_num::opbehavior::*;
        let behaviors: Vec<Rc<dyn OpBehavior>> = vec![
            Rc::new(OpBehaviorCopy),
            Rc::new(OpBehaviorSubpiece),
            Rc::new(OpBehaviorEqual),
            Rc::new(OpBehaviorNotEqual),
            Rc::new(OpBehaviorIntZext),
            Rc::new(OpBehaviorIntSext),
            Rc::new(OpBehaviorIntAdd),
            Rc::new(OpBehaviorIntSub),
            Rc::new(OpBehaviorInt2Comp),
            Rc::new(OpBehaviorIntNegate),
            Rc::new(OpBehaviorIntXor),
            Rc::new(OpBehaviorIntAnd),
            Rc::new(OpBehaviorIntOr),
            Rc::new(OpBehaviorIntLeft),
            Rc::new(OpBehaviorIntRight),
            Rc::new(OpBehaviorIntMult),
        ];
        for behavior in behaviors {
            let code = behavior.get_opcode();
            arch.opbehaviors[code as usize] = Some(behavior);
        }
        arch.return_branch = !off;
        if tracked {
            let loc = kuna_num::pcoderaw::VarnodeData {
                space: Some(ram.clone()),
                offset: 0x10,
                size: 8,
            };
            arch.indirect_return_addr = Some(loc.clone());
            arch.tracked_sets
                .default_value_mut()
                .push(kuna_sleigh::globalcontext::TrackedContext { loc, val: 0x1020 });
        }
        Funcdata::new(
            "probe",
            "probe",
            Rc::new(arch),
            Address::new(ram, 0x1000),
            0,
            0x400,
        )
        .unwrap()
    }

    // Inputs are (constant, offset/value, byte width); outputs use register offsets.
    fn emit(
        fd: &mut Funcdata,
        pc: u64,
        code: OpCode,
        out: Option<(u64, i32)>,
        ins: &[(bool, u64, i32)],
    ) -> OpId {
        let ram = fd
            .get_arch()
            .manage()
            .get_space_by_name("ram")
            .unwrap()
            .clone();
        let op = fd.new_op(ins.len() as i32, Address::new(ram.clone(), pc));
        fd.op_set_opcode(op, crate::typeop::type_op_for(code));
        for (slot, &(constant, value, size)) in ins.iter().enumerate() {
            let v = if constant {
                fd.new_constant(size, value)
            } else {
                fd.new_varnode(size, &Address::new(ram.clone(), value), None)
            };
            fd.op_set_input(op, v, slot as i32).unwrap();
        }
        if let Some((offset, size)) = out {
            let v = fd.new_varnode(size, &Address::new(ram, offset), None);
            fd.op_set_output(op, v).unwrap();
        }
        op
    }

    #[test]
    fn known_subregister_reads_match_the_written_bytes_without_crossing_storage() {
        let value = 0x8070605040302010u64;
        let known = [(Loc(2, 0x100, 8), value)];
        for big in [false, true] {
            let bytes = if big {
                value.to_be_bytes()
            } else {
                value.to_le_bytes()
            };
            for size in [1, 2, 4, 8] {
                for offset in 0..9usize {
                    let expected = if offset + size <= 8 {
                        let mut word = [0; 8];
                        let start = if big { 8 - size } else { 0 };
                        word[start..start + size].copy_from_slice(&bytes[offset..offset + size]);
                        Some(if big {
                            u64::from_be_bytes(word)
                        } else {
                            u64::from_le_bytes(word)
                        })
                    } else {
                        None
                    };
                    assert_eq!(
                        read_known(Loc(2, 0x100 + offset as u64, size as i32), big, &known),
                        expected
                    );
                }
            }
            assert!(read_known(Loc(3, 0x100, 4), big, &known).is_none());
        }
    }

    /// A `visited` map in which each `(address, size)` is an instruction that
    /// emitted no p-code, as a MIPS `nop` does.
    fn without_pcode(fd: &Funcdata, run: &[(u64, i32)]) -> crate::flow::VisitedMap {
        let ram = fd.get_address().get_space().unwrap().clone();
        run.iter()
            .map(|&(at, size)| {
                (
                    Address::new(ram.clone(), at),
                    crate::flow::VisitStat { seqnum: SeqNum::new(Address::default(), 0), size },
                )
            })
            .collect()
    }

    /// The whole run before the first operation emitted no p-code, so the
    /// prefix that follows it is still the entry prefix.
    #[test]
    fn instructions_emitting_no_pcode_may_precede_the_entry_prefix() {
        for leading in [&[(0x1000, 4)][..], &[(0x1000, 4), (0x1004, 4)][..]] {
            let first = leading.last().unwrap().0 + 4;
            let mut fd = function();
            emit(&mut fd, first, OpCode::CPUI_COPY, Some((0x20, 8)), &[(true, 0x1080, 8)]);
            let ret = emit(&mut fd, first + 4, OpCode::CPUI_RETURN, None, &[(false, 0x20, 8)]);
            let visited = without_pcode(&fd, leading);
            assert!(computed_target(&fd, ret, &visited, |_| false), "{leading:?}");
            assert!(!proves(&fd, ret, |_| false), "{leading:?}");
        }
    }

    /// Only a decoded instruction that emitted nothing may be skipped: an
    /// unrecorded gap, an instruction that did emit p-code, one whose length
    /// does not reach the first operation, and an operation before the entry
    /// all leave the prefix unproven.
    #[test]
    fn a_gap_before_the_entry_prefix_is_not_assumed_empty() {
        let mut fd = function();
        emit(&mut fd, 0x1004, OpCode::CPUI_COPY, Some((0x20, 8)), &[(true, 0x1080, 8)]);
        let ret = emit(&mut fd, 0x1008, OpCode::CPUI_RETURN, None, &[(false, 0x20, 8)]);
        let ram = fd.get_address().get_space().unwrap().clone();
        assert!(!computed_target(&fd, ret, &crate::flow::VisitedMap::new(), |_| false));
        let mut emitted = without_pcode(&fd, &[(0x1000, 4)]);
        emitted.get_mut(&Address::new(ram.clone(), 0x1000)).unwrap().seqnum =
            SeqNum::new(Address::new(ram.clone(), 0x1000), 1);
        assert!(!computed_target(&fd, ret, &emitted, |_| false));
        assert!(!computed_target(&fd, ret, &without_pcode(&fd, &[(0x1000, 2)]), |_| false));
        assert!(!computed_target(&fd, ret, &without_pcode(&fd, &[(0x1000, 8)]), |_| false));
        let mut below = function();
        emit(&mut below, 0x0ffc, OpCode::CPUI_COPY, Some((0x20, 8)), &[(true, 0x1080, 8)]);
        let ret = emit(&mut below, 0x1000, OpCode::CPUI_RETURN, None, &[(false, 0x20, 8)]);
        assert!(!computed_target(&below, ret, &without_pcode(&below, &[(0x0ffc, 4)]), |_| false));
    }

    /// The marker is named by the language, not by an architecture id.
    #[test]
    fn only_the_isa_mode_user_op_name_is_a_mode_switch() {
        assert!(names_isa_mode_switch(b"setISAMode"));
        assert!(!names_isa_mode_switch(b"setisamode"));
        assert!(!names_isa_mode_switch(b"setISAMode2"));
        assert!(!names_isa_mode_switch(b""));
    }

    #[test]
    fn captured_jump_target_survives_the_delay_slot_overwrite() {
        let mut fd = function();
        emit(
            &mut fd,
            0x1000,
            OpCode::CPUI_COPY,
            Some((0x10, 8)),
            &[(true, 0x1020, 8)],
        );
        emit(
            &mut fd,
            0x1004,
            OpCode::CPUI_COPY,
            Some((0x20, 8)),
            &[(false, 0x10, 8)],
        );
        emit(
            &mut fd,
            0x1004,
            OpCode::CPUI_COPY,
            Some((0x10, 8)),
            &[(false, 0x30, 8)],
        );
        let ret = emit(
            &mut fd,
            0x1004,
            OpCode::CPUI_RETURN,
            None,
            &[(false, 0x20, 8)],
        );
        assert!(proves(&fd, ret, |_| false));
        fd.obank_mut()
            .get_mut(ret)
            .unwrap()
            .set_flag(crate::op::pcodeop_flags::badinstruction);
        assert!(!proves(&fd, ret, |_| false));
        fd.obank_mut()
            .get_mut(ret)
            .unwrap()
            .clear_flag(crate::op::pcodeop_flags::badinstruction);
        let at = Address::new(fd.get_address().get_space().unwrap().clone(), 0x1004);
        fd.get_override_mut()
            .insert_flow_override(at, crate::overrides::flow_type::RETURN);
        assert!(!proves(&fd, ret, |_| false));
    }

    #[test]
    fn option_off_keeps_the_return() {
        let mut fd = function_with_option(false, true);
        emit(
            &mut fd,
            0x1000,
            OpCode::CPUI_COPY,
            Some((0x20, 8)),
            &[(true, 0x1020, 8)],
        );
        let ret = emit(
            &mut fd,
            0x1004,
            OpCode::CPUI_RETURN,
            None,
            &[(false, 0x20, 8)],
        );
        assert!(!proves(&fd, ret, |_| false));
    }

    #[test]
    fn a_zeroed_return_register_keeps_the_return() {
        for (value, kept) in [(0, false), (0x1020, true)] {
            let mut fd = function();
            emit(&mut fd, 0x1000, OpCode::CPUI_COPY, Some((0x10, 8)), &[(true, value, 8)]);
            let ret = emit(&mut fd, 0x1004, OpCode::CPUI_RETURN, None, &[(false, 0x10, 8)]);
            assert_eq!(proves(&fd, ret, |_| false), kept, "{value:#x}");
        }
    }

    #[test]
    fn known_incoming_return_address_still_requires_a_proven_overwrite() {
        for overwritten in [false, true] {
            let mut fd = function_with_known_return(true);
            if overwritten {
                emit(
                    &mut fd,
                    0x1000,
                    OpCode::CPUI_COPY,
                    Some((0x10, 8)),
                    &[(true, 0x1030, 8)],
                );
            }
            emit(
                &mut fd,
                0x1000,
                OpCode::CPUI_COPY,
                Some((0x20, 8)),
                &[(false, 0x10, 8)],
            );
            let ret = emit(
                &mut fd,
                0x1000,
                OpCode::CPUI_RETURN,
                None,
                &[(false, 0x20, 8)],
            );
            assert_eq!(proves(&fd, ret, |_| false), overwritten);
        }
    }

    #[test]
    fn writing_ra_in_the_delay_slot_does_not_retarget_an_incoming_return() {
        let mut fd = function();
        emit(
            &mut fd,
            0x1000,
            OpCode::CPUI_COPY,
            Some((0x20, 8)),
            &[(false, 0x10, 8)],
        );
        emit(
            &mut fd,
            0x1000,
            OpCode::CPUI_COPY,
            Some((0x10, 8)),
            &[(true, 0x1020, 8)],
        );
        let ret = emit(
            &mut fd,
            0x1000,
            OpCode::CPUI_RETURN,
            None,
            &[(false, 0x20, 8)],
        );
        assert!(!proves(&fd, ret, |_| false));
    }

    #[test]
    fn partial_writes_and_unknown_loads_invalidate_full_values() {
        for (code, width, ins) in [
            (OpCode::CPUI_COPY, 4, vec![(true, 0x1020, 4)]),
            (OpCode::CPUI_LOAD, 8, vec![(true, 2, 8), (true, 0x1020, 8)]),
        ] {
            let mut fd = function();
            emit(
                &mut fd,
                0x1000,
                OpCode::CPUI_COPY,
                Some((0x10, 8)),
                &[(true, 0x1020, 8)],
            );
            emit(&mut fd, 0x1004, code, Some((0x10, width)), &ins);
            let ret = emit(
                &mut fd,
                0x1008,
                OpCode::CPUI_RETURN,
                None,
                &[(false, 0x10, 8)],
            );
            assert!(!proves(&fd, ret, |_| false), "{code:?}");
        }
    }

    #[test]
    fn calls_branches_unknown_userops_and_budget_exhaustion_decline() {
        for code in [
            OpCode::CPUI_CALL,
            OpCode::CPUI_CALLIND,
            OpCode::CPUI_BRANCH,
            OpCode::CPUI_BRANCHIND,
            OpCode::CPUI_CBRANCH,
            OpCode::CPUI_CALLOTHER,
        ] {
            let mut fd = function();
            emit(
                &mut fd,
                0x1000,
                OpCode::CPUI_COPY,
                Some((0x10, 8)),
                &[(true, 0x1020, 8)],
            );
            emit(&mut fd, 0x1004, code, None, &[(true, 7, 8), (true, 0, 1)]);
            let ret = emit(
                &mut fd,
                0x1008,
                OpCode::CPUI_RETURN,
                None,
                &[(false, 0x10, 8)],
            );
            assert!(!proves(&fd, ret, |_| false), "{code:?}");
            assert_eq!(
                proves(&fd, ret, |id| id == 7),
                code == OpCode::CPUI_CALLOTHER
            );
        }
        let mut fd = function();
        for i in 0..256 {
            emit(
                &mut fd,
                0x1000 + 4 * i,
                OpCode::CPUI_COPY,
                Some((0x10, 8)),
                &[(true, 0x1020, 8)],
            );
        }
        let ret = emit(
            &mut fd,
            0x1400,
            OpCode::CPUI_RETURN,
            None,
            &[(false, 0x10, 8)],
        );
        assert!(!proves(&fd, ret, |_| false));
    }

    #[test]
    fn a_store_forgets_only_the_facts_in_the_space_it_writes() {
        let ram = 2;
        for (space, kept) in [(ram, false), (7, true)] {
            let mut fd = function();
            emit(&mut fd, 0x1000, OpCode::CPUI_COPY, Some((0x10, 8)), &[(true, 0x1020, 8)]);
            emit(&mut fd, 0x1004, OpCode::CPUI_STORE, None,
                 &[(true, space, 8), (false, 0x18, 8), (true, 0, 8)]);
            let ret = emit(&mut fd, 0x1008, OpCode::CPUI_RETURN, None, &[(false, 0x10, 8)]);
            assert_eq!(proves(&fd, ret, |_| false), kept, "space {space}");
        }
        let mut fd = function();
        emit(&mut fd, 0x1000, OpCode::CPUI_COPY, Some((0x10, 8)), &[(true, 0x1020, 8)]);
        emit(&mut fd, 0x1004, OpCode::CPUI_STORE, None,
             &[(false, 0x28, 8), (false, 0x18, 8), (true, 0, 8)]);
        let ret = emit(&mut fd, 0x1008, OpCode::CPUI_RETURN, None, &[(false, 0x10, 8)]);
        assert!(!proves(&fd, ret, |_| false), "unknown space");
    }

    #[test]
    fn local_edge_constants_stop_at_unknown_values_calls_and_partial_writes() {
        for case in ["constant", "unknown", "call", "partial", "store-other", "store-same"] {
            let mut fd = function();
            let dispatch = emit(&mut fd, 0x1000, OpCode::CPUI_BRANCHIND, None, &[(false, 0x10, 8)]);
            let start = emit(&mut fd, 0x1020, OpCode::CPUI_COPY, Some((0x10, 8)),
                             &[(case != "unknown", if case == "unknown" { 0x18 } else { 0x1040 }, 8)]);
            fd.obank_mut().get_mut(start).unwrap().set_flag(crate::op::pcodeop_flags::startbasic);
            match case {
                "call" => { emit(&mut fd, 0x1024, OpCode::CPUI_CALL, None, &[(true, 0x2000, 8)]); }
                "partial" => { emit(&mut fd, 0x1024, OpCode::CPUI_COPY, Some((0x12, 1)), &[(true, 0, 1)]); }
                "store-other" => { emit(&mut fd, 0x1024, OpCode::CPUI_STORE, None, &[(true, 7, 8), (false, 0x18, 8), (true, 0, 8)]); }
                "store-same" => { emit(&mut fd, 0x1024, OpCode::CPUI_STORE, None, &[(true, 2, 8), (false, 0x18, 8), (true, 0, 8)]); }
                _ => {}
            }
            let edge = emit(&mut fd, 0x1028, OpCode::CPUI_BRANCH, None, &[(true, 0x1000, 8)]);
            let proved = matches!(case, "constant" | "store-other");
            assert_eq!(target_on_edge(&fd, edge, dispatch, None, |_| false), proved.then_some(0x1040), "{case}");
        }
    }
}
