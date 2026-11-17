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
}

impl<'a, F: Fn(u64) -> bool> Prefix<'a, F> {
    /// No storage is known. `mode_switch` names user operations that only
    /// select an instruction set and leave every register unchanged.
    pub(crate) fn new(data: &'a Funcdata, mode_switch: F) -> Self {
        Prefix { data, known: Vec::new(), previous: None, mode_switch }
    }

    /// The function entry: tracked context values are known, except any
    /// overlapping the incoming return address.
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
        Prefix { data, known, previous: None, mode_switch }
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
            if !(1..=8).contains(&out.2)
                || !matches!(
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

fn transfers_control(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN
    )
}

/// Does the entry prefix compute a constant target for this real RETURN?
pub(crate) fn computed_target(
    data: &Funcdata,
    target: OpId,
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
        if i == 0 && op.get_addr() != data.get_address() {
            return false;
        }
        if id == target {
            return prefix.input(id, 0).is_some();
        }
        if transfers_control(op.code()) || !prefix.execute(id) {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use kuna_base::address::Address;
    use kuna_base::space::{AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use std::rc::Rc;

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
        assert!(computed_target(&fd, ret, |_| false));
        fd.obank_mut()
            .get_mut(ret)
            .unwrap()
            .set_flag(crate::op::pcodeop_flags::badinstruction);
        assert!(!computed_target(&fd, ret, |_| false));
        fd.obank_mut()
            .get_mut(ret)
            .unwrap()
            .clear_flag(crate::op::pcodeop_flags::badinstruction);
        let at = Address::new(fd.get_address().get_space().unwrap().clone(), 0x1004);
        fd.get_override_mut()
            .insert_flow_override(at, crate::overrides::flow_type::RETURN);
        assert!(!computed_target(&fd, ret, |_| false));
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
        assert!(!computed_target(&fd, ret, |_| false));
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
            assert_eq!(computed_target(&fd, ret, |_| false), overwritten);
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
        assert!(!computed_target(&fd, ret, |_| false));
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
            assert!(!computed_target(&fd, ret, |_| false), "{code:?}");
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
            assert!(!computed_target(&fd, ret, |_| false), "{code:?}");
            assert_eq!(
                computed_target(&fd, ret, |id| id == 7),
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
        assert!(!computed_target(&fd, ret, |_| false));
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
            assert_eq!(computed_target(&fd, ret, |_| false), kept, "space {space}");
        }
        let mut fd = function();
        emit(&mut fd, 0x1000, OpCode::CPUI_COPY, Some((0x10, 8)), &[(true, 0x1020, 8)]);
        emit(&mut fd, 0x1004, OpCode::CPUI_STORE, None,
             &[(false, 0x28, 8), (false, 0x18, 8), (true, 0, 8)]);
        let ret = emit(&mut fd, 0x1008, OpCode::CPUI_RETURN, None, &[(false, 0x10, 8)]);
        assert!(!computed_target(&fd, ret, |_| false), "unknown space");
    }
}
