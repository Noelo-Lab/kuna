use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p6_variables::funcdata_spacebase::spacebase_tests::{build_fd, make_sp_in, reg_addr};

use super::{
    eligible_effect_lane, overlaps, proven_unobserved, stack_memory_access, EffectRange,
    FrameRange, Save,
};

const SLOT: u64 = u64::MAX - 7;

enum Out {
    Unique(int4),
    At(Rc<AddrSpace>, u64, int4),
}

struct Fx {
    fd: Funcdata,
    block: BlockId,
    reg: Rc<AddrSpace>,
    other: Rc<AddrSpace>,
    stack: Rc<AddrSpace>,
    sp: VarnodeId,
    pc: u64,
}

impl Fx {
    fn new() -> Self {
        let mut fd = build_fd();
        let sp = make_sp_in(&mut fd);
        fd.spacebase();
        let reg = Rc::clone(
            fd.get_arch()
                .manage()
                .get_space_by_name("register")
                .unwrap(),
        );
        let other = Rc::clone(fd.get_arch().manage().get_space_by_name("unique").unwrap());
        let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
        let root = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(root);
        Self {
            fd,
            block,
            reg,
            other,
            stack,
            sp,
            pc: 0x1100,
        }
    }

    fn input(&mut self, offset: u64, size: int4) -> VarnodeId {
        let vn = self.fd.new_varnode(size, &reg_addr(&self.fd, offset), None);
        self.fd.set_input_varnode(vn).unwrap()
    }

    fn constant(&mut self, size: int4, value: u64) -> VarnodeId {
        self.fd.new_constant(size, value)
    }

    fn emit(
        &mut self,
        code: OpCode,
        inputs: &[VarnodeId],
        output: Option<Out>,
    ) -> (OpId, Option<VarnodeId>) {
        self.pc += 1;
        let op = self
            .fd
            .new_op(inputs.len() as int4, reg_addr(&self.fd, self.pc));
        self.fd.op_set_opcode_code(op, code);
        for (slot, input) in inputs.iter().enumerate() {
            self.fd.op_set_input(op, *input, slot as int4).unwrap();
        }
        let output = output.map(|out| match out {
            Out::Unique(size) => self.fd.new_unique_out(size, op).unwrap(),
            Out::At(space, offset, size) => self
                .fd
                .new_varnode_out(size, &Address::new(space, offset), op)
                .unwrap(),
        });
        self.fd.op_insert(op, self.block, None);
        (op, output)
    }

    fn frame_pointer(&mut self, offset: i64) -> VarnodeId {
        let amount = self.constant(8, offset as u64);
        let sp = self.sp;
        self.emit(OpCode::CPUI_INT_ADD, &[sp, amount], Some(Out::Unique(8)))
            .1
            .unwrap()
    }

    fn copy_save(&mut self, source: VarnodeId) -> Save {
        let stack = Rc::clone(&self.stack);
        let op = self
            .emit(OpCode::CPUI_COPY, &[source], Some(Out::At(stack, SLOT, 4)))
            .0;
        Save {
            op,
            slot: FrameRange {
                start: SLOT,
                size: 4,
                exact: true,
            },
            source,
        }
    }

    fn store(&mut self, space: &AddrSpace, pointer: VarnodeId, value: VarnodeId) -> OpId {
        let space_id = self.constant(4, space.get_index() as u64);
        self.emit(OpCode::CPUI_STORE, &[space_id, pointer, value], None)
            .0
    }
}

#[test]
fn frame_overlap_handles_adjacency_zero_crossing_and_large_disjoint_ranges() {
    let fx = Fx::new();
    for (left, left_size, right, right_size, expected) in [
        (u64::MAX - 7, 4, u64::MAX - 7, 4, true),
        (u64::MAX - 7, 4, u64::MAX - 4, 4, true),
        (u64::MAX - 7, 4, u64::MAX - 3, 4, false),
        (u64::MAX - 1, 4, 0, 1, true),
        (u64::MAX - 1, 4, 1, 1, true),
        (u64::MAX - 1, 4, 2, 1, false),
        (0x10000, 65536, 0x20000, 16, false),
        (0x10000, 65536, 0x1ffff, 16, true),
        (0, 0, 0, 1, false),
        (0, -1, 0, 1, false),
    ] {
        let left = FrameRange {
            start: left,
            size: left_size,
            exact: true,
        };
        let right = FrameRange {
            start: right,
            size: right_size,
            exact: true,
        };
        assert_eq!(overlaps(&fx.stack, left, right), expected);
        assert_eq!(overlaps(&fx.stack, right, left), expected);
    }
}

#[test]
fn effect_lanes_must_be_contained_and_unaffected_inputs() {
    let fx = Fx::new();
    let effect = EffectRange {
        address: Address::new(Rc::clone(&fx.reg), 0x100),
        size: 16,
    };
    let inside = Address::new(Rc::clone(&fx.reg), 0x104);
    assert!(eligible_effect_lane(
        &effect,
        &inside,
        4,
        true,
        true,
        crate::fspec::effect_type::UNAFFECTED,
    ));
    for (address, size, input, unaffected, status) in [
        (0x10c, 8, true, true, crate::fspec::effect_type::UNAFFECTED),
        (0x0fc, 8, true, true, crate::fspec::effect_type::UNAFFECTED),
        (0x104, 4, false, true, crate::fspec::effect_type::UNAFFECTED),
        (0x104, 4, true, false, crate::fspec::effect_type::UNAFFECTED),
        (
            0x104,
            4,
            true,
            true,
            crate::fspec::effect_type::UNKNOWN_EFFECT,
        ),
    ] {
        assert!(!eligible_effect_lane(
            &effect,
            &Address::new(Rc::clone(&fx.reg), address),
            size,
            input,
            unaffected,
            status,
        ));
    }
}

#[test]
fn cross_space_store_cannot_be_a_candidate_or_be_ignored_as_an_observer() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let pointer = fx.frame_pointer(-8);
    let register_space = Rc::clone(&fx.reg);
    let stack_store = fx.store(&register_space, pointer, source);
    let other = Rc::clone(&fx.other);
    let other_store = fx.store(&other, pointer, source);
    let (resolved, _) = crate::ruleaction_4::RuleLoadVarnode::check_spacebase(&fx.fd, stack_store)
        .expect("the containing register space must resolve to the stack space");
    assert!(Rc::ptr_eq(&resolved, &fx.stack));
    assert!(stack_memory_access(&fx.fd, stack_store, &fx.stack));
    assert!(!stack_memory_access(&fx.fd, other_store, &fx.stack));

    let save = Save {
        op: stack_store,
        slot: FrameRange {
            start: SLOT,
            size: 4,
            exact: true,
        },
        source,
    };
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn stack_memory_space_classifies_loads_but_both_overlapping_loads_observe() {
    let mut valid = Fx::new();
    let source = valid.input(0x40, 4);
    let save = valid.copy_save(source);
    let pointer = valid.frame_pointer(-8);
    let memory_space_id = valid.reg.get_index() as u64;
    let space_id = valid.constant(4, memory_space_id);
    let valid_load = valid
        .emit(
            OpCode::CPUI_LOAD,
            &[space_id, pointer],
            Some(Out::Unique(4)),
        )
        .0;
    let (resolved, _) =
        crate::ruleaction_4::RuleLoadVarnode::check_spacebase(&valid.fd, valid_load)
            .expect("the valid fixture load must resolve through the stack spacebase");
    assert!(Rc::ptr_eq(&resolved, &valid.stack));
    assert!(stack_memory_access(&valid.fd, valid_load, &valid.stack));
    assert!(!proven_unobserved(&valid.fd, &[save], &valid.stack));

    let mut other = Fx::new();
    let source = other.input(0x40, 4);
    let save = other.copy_save(source);
    let pointer = other.frame_pointer(-8);
    let other_space_id = other.constant(4, other.other.get_index() as u64);
    let other_load = other
        .emit(
            OpCode::CPUI_LOAD,
            &[other_space_id, pointer],
            Some(Out::Unique(4)),
        )
        .0;
    assert!(crate::ruleaction_4::RuleLoadVarnode::check_spacebase(&other.fd, other_load).is_none());
    assert!(!stack_memory_access(&other.fd, other_load, &other.stack));
    assert!(!proven_unobserved(&other.fd, &[save], &other.stack));
}

#[test]
fn matching_lane_load_before_candidate_save_still_blocks_restriction() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let pointer = fx.frame_pointer(-8);
    let memory_space_id = fx.reg.get_index() as u64;
    let space_id = fx.constant(4, memory_space_id);
    let register_space = Rc::clone(&fx.reg);
    let load = fx
        .emit(
            OpCode::CPUI_LOAD,
            &[space_id, pointer],
            Some(Out::At(register_space, 0x40, 4)),
        )
        .0;
    assert!(stack_memory_access(&fx.fd, load, &fx.stack));
    let save = fx.copy_save(source);
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn any_non_save_opcode_writing_candidate_stack_bytes_blocks_restriction() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let save = fx.copy_save(source);
    let left = fx.constant(4, 1);
    let right = fx.constant(4, 2);
    let stack = Rc::clone(&fx.stack);
    fx.emit(
        OpCode::CPUI_INT_ADD,
        &[left, right],
        Some(Out::At(stack, SLOT, 4)),
    );
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn live_load_from_candidate_stack_bytes_is_an_observer() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let save = fx.copy_save(source);
    let pointer = fx.frame_pointer(-8);
    let memory_space_id = fx.reg.get_index() as u64;
    let space_id = fx.constant(4, memory_space_id);
    let load = fx
        .emit(
            OpCode::CPUI_LOAD,
            &[space_id, pointer],
            Some(Out::Unique(4)),
        )
        .0;
    assert!(stack_memory_access(&fx.fd, load, &fx.stack));
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn surviving_stack_copy_restore_is_still_a_value_read() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let save = fx.copy_save(source);
    let slot = fx
        .fd
        .new_varnode(4, &Address::new(Rc::clone(&fx.stack), SLOT), None);
    let register_space = Rc::clone(&fx.reg);
    fx.emit(
        OpCode::CPUI_COPY,
        &[slot],
        Some(Out::At(register_space, 0x40, 4)),
    );
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn frame_pointer_copy_to_persistent_or_stack_storage_is_an_escape() {
    for stack_output in [false, true] {
        let mut fx = Fx::new();
        let source = fx.input(0x40, 4);
        let save = fx.copy_save(source);
        let pointer = fx.frame_pointer(-8);
        let output = if stack_output {
            Out::At(Rc::clone(&fx.stack), SLOT - 0x20, 8)
        } else {
            Out::At(Rc::clone(&fx.reg), 0x300, 8)
        };
        let copied = fx
            .emit(OpCode::CPUI_COPY, &[pointer], Some(output))
            .1
            .unwrap();
        if !stack_output {
            fx.fd
                .vbank_mut()
                .get_mut(copied)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::persist);
        }
        assert!(overlaps(
            &fx.stack,
            super::frame_range(&fx.fd, pointer, 8).unwrap(),
            save.slot
        ));
        assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
    }
}

#[test]
fn returning_a_disjoint_frame_pointer_is_still_an_escape() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let save = fx.copy_save(source);
    let pointer = fx.frame_pointer(-32);
    let returned = super::frame_range(&fx.fd, pointer, 8).unwrap();
    assert!(!overlaps(&fx.stack, returned, save.slot));
    let return_space = fx.constant(4, fx.reg.get_index() as u64);
    fx.emit(OpCode::CPUI_RETURN, &[return_space, pointer], None);
    assert!(!proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn returning_loaded_disjoint_field_contents_is_not_a_frame_pointer_escape() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let save = fx.copy_save(source);
    let pointer = fx.frame_pointer(-32);
    let space_id = fx.constant(4, fx.reg.get_index() as u64);
    let contents = fx
        .emit(
            OpCode::CPUI_LOAD,
            &[space_id, pointer],
            Some(Out::Unique(4)),
        )
        .1
        .unwrap();
    let one = fx.constant(4, 0x3f800000);
    let result = fx
        .emit(
            OpCode::CPUI_FLOAT_ADD,
            &[contents, one],
            Some(Out::Unique(4)),
        )
        .1
        .unwrap();
    let return_space = fx.constant(4, fx.reg.get_index() as u64);
    fx.emit(OpCode::CPUI_RETURN, &[return_space, result], None);
    assert!(proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn direct_save_through_stack_pointer_temporaries_is_accepted_without_escapes() {
    let mut fx = Fx::new();
    let source = fx.input(0x40, 4);
    let pointer = fx.frame_pointer(-8);
    let register_space = Rc::clone(&fx.reg);
    let op = fx.store(&register_space, pointer, source);
    let save = Save {
        op,
        slot: FrameRange {
            start: SLOT,
            size: 4,
            exact: true,
        },
        source,
    };
    assert!(proven_unobserved(&fx.fd, &[save], &fx.stack));
}

#[test]
fn opaque_direct_indirect_and_user_calls_block_restriction() {
    for code in [
        OpCode::CPUI_CALL,
        OpCode::CPUI_CALLIND,
        OpCode::CPUI_CALLOTHER,
    ] {
        let mut fx = Fx::new();
        let source = fx.input(0x40, 4);
        let save = fx.copy_save(source);
        let target = fx.constant(8, 0x2000);
        fx.emit(code, &[target], None);
        assert!(
            !proven_unobserved(&fx.fd, &[save], &fx.stack),
            "{code:?} without a complete memory summary must block saved-lane pruning"
        );
    }
}
