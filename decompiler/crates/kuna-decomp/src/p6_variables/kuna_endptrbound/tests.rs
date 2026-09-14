//! Unit tests for `endptrbound`: the walk detector on hand-built loops, the
//! frame-hint coalescence, and the bound rewrite.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::varmap::{MapState, RangeType, COPY_CONSTANT, TYPELOCK};

use super::{
    coalesce_hints, find_pointer_walks, gather_walks, rebase_bounds, OptionEndPtrBound, PointerWalk,
    ELEM_ENDPTRBOUND,
};

const SP_OFF: u64 = 0x20;
const START: i64 = -0x58;
const END: i64 = -0x50;

struct Fx {
    fd: Funcdata,
    sp: VarnodeId,
    pc: u64,
}

impl Fx {
    fn new(option: bool) -> Fx {
        let mut manage = AddrSpaceManager::new();
        manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manage.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        let reg = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        manage.insert_space(Rc::clone(&reg)).unwrap();
        manage.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &reg, 1, true, false))).unwrap();
        let stack = Rc::clone(manage.get_stack_space().unwrap());
        let sp_data = VarnodeStorage { space: Some(Rc::clone(&reg)), offset: SP_OFF, size: 8 };
        manage.add_spacebase_pointer(&stack, &sp_data, 8, true).unwrap();
        let mut glb = ArchContext::new(manage);
        glb.end_ptr_bound = option;
        let entry = Address::new(Rc::clone(&reg), 0x1000);
        let mut fd = Funcdata::new("walk", "walk", Rc::new(glb), entry, 0x1000_0000, 0x40).unwrap();
        let vn = fd.new_varnode(8, &Address::new(reg, SP_OFF), None);
        let sp = fd.set_input_varnode(vn).unwrap();
        Fx { fd, sp, pc: 0x1000 }
    }

    fn reg(&self) -> Rc<AddrSpace> {
        Rc::clone(self.fd.get_arch().manage().get_space_by_name("register").unwrap())
    }

    fn stack(&self) -> Rc<AddrSpace> {
        Rc::clone(self.fd.get_arch().manage().get_stack_space().unwrap())
    }

    fn block(&mut self) -> BlockId {
        let root = self.fd.bblocks_root_pub();
        self.fd.bblocks_mut().new_block_basic(root)
    }

    fn edge(&mut self, from: BlockId, to: BlockId) {
        self.fd.bblocks_mut().add_edge(from, to);
    }

    fn konst(&mut self, v: i64) -> VarnodeId {
        self.fd.new_constant(8, v as u64)
    }

    fn register_input(&mut self, off: u64, size: int4) -> VarnodeId {
        let vn = self.fd.new_varnode(size, &Address::new(self.reg(), off), None);
        self.fd.set_input_varnode(vn).unwrap()
    }

    fn emit(&mut self, bl: BlockId, code: OpCode, ins: &[VarnodeId], out: Option<int4>) -> (OpId, Option<VarnodeId>) {
        self.pc += 1;
        let op = self.fd.new_op(ins.len() as int4, Address::new(self.reg(), self.pc));
        self.fd.op_set_opcode_code(op, code);
        let out = out.map(|size| self.fd.new_unique_out(size, op).unwrap());
        for (slot, vn) in ins.iter().enumerate() {
            self.fd.op_set_input(op, *vn, slot as int4).unwrap();
        }
        self.fd.op_insert(op, bl, None);
        (op, out)
    }

    fn val(&mut self, bl: BlockId, code: OpCode, ins: &[VarnodeId], size: int4) -> VarnodeId {
        self.emit(bl, code, ins, Some(size)).1.unwrap()
    }

    fn stack_addr(&mut self, bl: BlockId, off: i64) -> VarnodeId {
        let (sp, c) = (self.sp, self.konst(off));
        self.val(bl, OpCode::CPUI_PTRSUB, &[sp, c], 8)
    }

    fn add(&mut self, bl: BlockId, base: VarnodeId, k: i64) -> VarnodeId {
        let c = self.konst(k);
        self.val(bl, OpCode::CPUI_INT_ADD, &[base, c], 8)
    }

    fn load(&mut self, bl: BlockId, ptr: VarnodeId, width: int4) -> OpId {
        let space = self.konst(0);
        self.emit(bl, OpCode::CPUI_LOAD, &[space, ptr], Some(width)).0
    }

    fn cmp(&mut self, bl: BlockId, code: OpCode, a: VarnodeId, b: VarnodeId) -> (OpId, VarnodeId) {
        let (op, out) = self.emit(bl, code, &[a, b], Some(1));
        (op, out.unwrap())
    }

    fn cbranch(&mut self, bl: BlockId, cond: VarnodeId) -> OpId {
        let dest = self.konst(0);
        self.emit(bl, OpCode::CPUI_CBRANCH, &[dest, cond], None).0
    }

    fn phi(&mut self, bl: BlockId, inputs: int4) -> (OpId, VarnodeId) {
        let placeholder: Vec<VarnodeId> = (0..inputs).map(|_| self.konst(0)).collect();
        let (op, out) = self.emit(bl, OpCode::CPUI_MULTIEQUAL, &placeholder, Some(8));
        (op, out.unwrap())
    }

    fn set_in(&mut self, op: OpId, slot: int4, vn: VarnodeId) {
        self.fd.op_set_input(op, vn, slot).unwrap();
    }

    fn finish(&mut self, entry: BlockId) {
        let root = self.fd.bblocks_root_pub();
        self.fd.bblocks_mut().set_start_block(root, entry);
        self.fd.structure_reset();
    }

    fn walks(&self) -> Vec<PointerWalk> {
        find_pointer_walks(&self.fd, self.sp)
    }
}

/// `p = buf; do { *p (width); p += step; } while (p != end);` as three blocks:
/// entry, the self-looping header, and the exit.  `exit_on_true` puts the exit on
/// the branch's true edge; the compare is built by `test` from `(p, p1, end)`.
struct DoWhile {
    fx: Fx,
    cmp: OpId,
}

fn do_while(
    start: i64,
    end: i64,
    step: i64,
    width: int4,
    exit_on_true: bool,
    test: impl FnOnce(&mut Fx, BlockId, VarnodeId, VarnodeId, VarnodeId) -> (OpId, VarnodeId),
) -> DoWhile {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, start);
    let e = fx.stack_addr(entry, end);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, width);
    let p1 = fx.add(head, p, step);
    let (cmp, cond) = test(&mut fx, head, p, p1, e);
    fx.cbranch(head, cond);
    if exit_on_true {
        fx.edge(head, head);
        fx.edge(head, exit);
    } else {
        fx.edge(head, exit);
        fx.edge(head, head);
    }
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    DoWhile { fx, cmp }
}

fn not_equal(fx: &mut Fx, bl: BlockId, _p: VarnodeId, p1: VarnodeId, e: VarnodeId) -> (OpId, VarnodeId) {
    fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
}

#[test]
fn a_do_while_walk_to_one_past_the_buffer_is_found() {
    let dw = do_while(START, END, 1, 1, false, not_equal);
    assert_eq!(dw.fx.walks(), vec![PointerWalk { cmp: dw.cmp, slot: 1, start: START, end: END, step: 1 }]);
}

#[test]
fn each_exit_form_of_the_comparison_is_found() {
    let eq = do_while(START, END, 1, 1, true, |fx, bl, _p, p1, e| fx.cmp(bl, OpCode::CPUI_INT_EQUAL, e, p1));
    assert_eq!(eq.fx.walks().len(), 1);
    assert_eq!(eq.fx.walks()[0].slot, 0);
    let less = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| fx.cmp(bl, OpCode::CPUI_INT_LESS, p1, e));
    assert_eq!(less.fx.walks().len(), 1);
    let lesseq =
        do_while(START, END, 1, 1, true, |fx, bl, _p, p1, e| fx.cmp(bl, OpCode::CPUI_INT_LESSEQUAL, e, p1));
    assert_eq!(lesseq.fx.walks().len(), 1);
    let negated = do_while(START, END, 1, 1, true, |fx, bl, _p, p1, e| {
        let (op, c) = fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e);
        (op, fx.val(bl, OpCode::CPUI_BOOL_NEGATE, &[c], 1))
    });
    assert_eq!(negated.fx.walks().len(), 1);
}

#[test]
fn a_comparison_whose_exit_edge_keeps_looping_is_not_a_bound() {
    let flipped = do_while(START, END, 1, 1, true, not_equal);
    assert!(flipped.fx.walks().is_empty());
    let wrong_way = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| fx.cmp(bl, OpCode::CPUI_INT_LESS, e, p1));
    assert!(wrong_way.fx.walks().is_empty());
    let inclusive =
        do_while(START, END, 1, 1, true, |fx, bl, _p, p1, e| fx.cmp(bl, OpCode::CPUI_INT_LESSEQUAL, p1, e));
    assert!(inclusive.fx.walks().is_empty());
}

#[test]
fn a_flipped_branch_is_read_the_other_way() {
    let mut dw = do_while(START, END, 1, 1, true, not_equal);
    assert!(dw.fx.walks().is_empty());
    let head = dw.fx.fd.obank().get(dw.cmp).unwrap().get_parent().unwrap();
    let cbranch = dw.fx.fd.bb_op_tail(head).unwrap();
    dw.fx.fd.obank_mut().get_mut(cbranch).unwrap().set_flag(pcodeop_flags::boolean_flip);
    assert_eq!(dw.fx.walks().len(), 1);
}

#[test]
fn a_pointer_carried_through_a_call_effect_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let iop = fx.konst(0);
    let after_call = fx.val(head, OpCode::CPUI_INDIRECT, &[p1, iop], 8);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, after_call, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, after_call);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_narrower_read_first_does_not_hide_the_step_wide_read() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, START + 8);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    fx.load(head, p, 2);
    let p1 = fx.add(head, p, 2);
    let (cmp, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert_eq!(fx.walks(), vec![PointerWalk { cmp, slot: 1, start: START, end: START + 8, step: 2 }]);
}

#[test]
fn a_read_wider_or_narrower_than_the_step_is_not_a_walk() {
    assert!(do_while(START, END, 1, 2, false, not_equal).fx.walks().is_empty());
    assert!(do_while(START, START + 8, 2, 1, false, not_equal).fx.walks().is_empty());
}

#[test]
fn a_decrementing_walk_is_not_a_walk() {
    assert!(do_while(END, START, -1, 1, false, not_equal).fx.walks().is_empty());
}

#[test]
fn a_span_that_is_not_whole_steps_or_is_one_step_is_not_a_walk() {
    assert!(do_while(START, START + 7, 2, 2, false, not_equal).fx.walks().is_empty());
    assert!(do_while(START, START + 1, 1, 1, false, not_equal).fx.walks().is_empty());
    assert!(do_while(START, START - 8, 1, 1, false, not_equal).fx.walks().is_empty());
    assert!(do_while(-0x20000, -0x20000 + 0x10001, 1, 1, false, not_equal).fx.walks().is_empty());
    assert_eq!(do_while(-0x20000, -0x20000 + 0x10000, 1, 1, false, not_equal).fx.walks().len(), 1);
}

#[test]
fn a_bound_that_is_not_a_stack_address_is_not_a_walk() {
    let global = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, _e| {
        let g = fx.konst(0x140074000);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, g)
    });
    assert!(global.fx.walks().is_empty());
    let other = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, _e| {
        let heap = fx.register_input(0x40, 8);
        let bound = fx.add(bl, heap, 8);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, bound)
    });
    assert!(other.fx.walks().is_empty());
}

#[test]
fn a_walk_that_does_not_start_on_the_stack_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let arg = fx.register_input(0x48, 8);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, arg);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

/// The loop exits on an unrelated condition; the pointer is compared against a
/// stack address only at a branch whose both edges stay inside the loop.
#[test]
fn a_midpoint_comparison_inside_the_loop_is_not_a_bound() {
    let mut fx = Fx::new(true);
    let (entry, head, side, latch, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let mid = fx.stack_addr(entry, START + 8);
    let more = fx.register_input(0x50, 1);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, at_mid) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p1, mid);
    fx.cbranch(head, at_mid);
    fx.edge(head, latch);
    fx.edge(head, side);
    fx.load(side, p, 1);
    fx.edge(side, latch);
    fx.cbranch(latch, more);
    fx.edge(latch, exit);
    fx.edge(latch, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_comparison_that_feeds_no_branch_is_not_a_bound() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    let more = fx.register_input(0x50, 1);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, flag) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    let space = fx.konst(0);
    let slot = fx.stack_addr(head, -0x10);
    fx.emit(head, OpCode::CPUI_STORE, &[space, slot, flag], None);
    fx.cbranch(head, more);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

/// `continue`-style path: the header branches around the exit test straight to
/// the latch, so a pointer that already reached the end can step past it.
#[test]
fn an_exit_test_a_back_edge_can_bypass_is_not_a_bound() {
    let mut fx = Fx::new(true);
    let (entry, head, test, latch, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    let skip = fx.register_input(0x50, 1);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    fx.cbranch(head, skip);
    fx.edge(head, test);
    fx.edge(head, latch);
    let (_, cond) = fx.cmp(test, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(test, cond);
    fx.edge(test, exit);
    fx.edge(test, latch);
    fx.edge(latch, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn back_edges_with_different_steps_are_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, one, two, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, START + 8);
    let pick = fx.register_input(0x50, 1);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 3);
    fx.load(head, p, 1);
    fx.cbranch(head, pick);
    fx.edge(head, one);
    fx.edge(head, two);
    let p1 = fx.add(one, p, 1);
    let (_, c1) = fx.cmp(one, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(one, c1);
    fx.edge(one, exit);
    fx.edge(one, head);
    let p2 = fx.add(two, p, 2);
    let (_, c2) = fx.cmp(two, OpCode::CPUI_INT_NOTEQUAL, p2, e);
    fx.cbranch(two, c2);
    fx.edge(two, exit);
    fx.edge(two, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.set_in(phi, 2, p2);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_read_past_the_element_or_through_a_variable_index_is_not_a_walk() {
    let peek = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| {
        fx.load(bl, p1, 1);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(peek.fx.walks().is_empty());
    let indexed = do_while(START, END, 1, 1, false, |fx, bl, p, p1, e| {
        let i = fx.register_input(0x58, 8);
        let at = fx.val(bl, OpCode::CPUI_INT_ADD, &[p, i], 8);
        fx.load(bl, at, 1);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(indexed.fx.walks().is_empty());
}

#[test]
fn a_pointer_handed_to_a_call_or_stored_is_not_a_walk() {
    let called = do_while(START, END, 1, 1, false, |fx, bl, p, p1, e| {
        let target = fx.konst(0x401000);
        fx.emit(bl, OpCode::CPUI_CALL, &[target, p], None);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(called.fx.walks().is_empty());
    let stored = do_while(START, END, 1, 1, false, |fx, bl, p, p1, e| {
        let space = fx.konst(0);
        let slot = fx.stack_addr(bl, -0x10);
        fx.emit(bl, OpCode::CPUI_STORE, &[space, slot, p], None);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(stored.fx.walks().is_empty());
}

#[test]
fn the_pointer_may_become_a_length_handed_to_a_call() {
    let dw = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| {
        let s = fx.stack_addr(bl, START);
        let len = fx.val(bl, OpCode::CPUI_INT_SUB, &[p1, s], 8);
        let target = fx.konst(0x401000);
        fx.emit(bl, OpCode::CPUI_CALL, &[target, len], None);
        let zero = fx.konst(0);
        let low = fx.val(bl, OpCode::CPUI_SUBPIECE, &[p1, zero], 4);
        fx.emit(bl, OpCode::CPUI_CALL, &[target, low], None);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert_eq!(dw.fx.walks().len(), 1);
}

/// `p = buf; while (p != end) { *p; p++; }` with the read before (declined) or
/// after (accepted) the header's exit test.
fn while_loop(read_before_test: bool) -> (Fx, OpId) {
    let mut fx = Fx::new(true);
    let (entry, head, body, exit) = (fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    if read_before_test {
        fx.load(head, p, 1);
    }
    let (cmp, cond) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p, e);
    fx.cbranch(head, cond);
    fx.edge(head, body);
    fx.edge(head, exit);
    if !read_before_test {
        fx.load(body, p, 1);
    }
    let p1 = fx.add(body, p, 1);
    fx.edge(body, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    (fx, cmp)
}

#[test]
fn a_test_of_the_pointer_itself_must_guard_every_read() {
    let (guarded, cmp) = while_loop(false);
    assert_eq!(guarded.walks(), vec![PointerWalk { cmp, slot: 1, start: START, end: END, step: 1 }]);
    let (unguarded, _) = while_loop(true);
    assert!(unguarded.walks().is_empty());
}

#[test]
fn exits_that_disagree_on_the_end_are_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, latch, exit) = (fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let near = fx.stack_addr(entry, START + 8);
    let far = fx.stack_addr(entry, START + 16);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, at_near) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p1, near);
    fx.cbranch(head, at_near);
    fx.edge(head, latch);
    fx.edge(head, exit);
    let (_, before_far) = fx.cmp(latch, OpCode::CPUI_INT_NOTEQUAL, p1, far);
    fx.cbranch(latch, before_far);
    fx.edge(latch, exit);
    fx.edge(latch, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_second_exit_on_the_data_keeps_the_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, latch, exit) = (fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    let space = fx.konst(0);
    let byte = fx.register_input(0x60, 1);
    fx.emit(head, OpCode::CPUI_STORE, &[space, p, byte], None);
    let p1 = fx.add(head, p, 1);
    let (cmp, at_end) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p1, e);
    fx.cbranch(head, at_end);
    fx.edge(head, latch);
    fx.edge(head, exit);
    let zero = fx.konst(0);
    let (_, more) = fx.cmp(latch, OpCode::CPUI_INT_NOTEQUAL, byte, zero);
    fx.cbranch(latch, more);
    fx.edge(latch, exit);
    fx.edge(latch, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert_eq!(fx.walks(), vec![PointerWalk { cmp, slot: 1, start: START, end: END, step: 1 }]);
}

#[test]
fn the_option_off_gathers_nothing() {
    let on = do_while(START, END, 1, 1, false, not_equal);
    assert_eq!(gather_walks(&on.fx.fd, &on.fx.stack()).len(), 1);
    let mut fx = Fx::new(false);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert_eq!(fx.walks().len(), 1);
    assert!(gather_walks(&fx.fd, &fx.stack()).is_empty());
}

fn char_array(len: int4) -> Rc<Datatype> {
    let f = factory();
    let elem = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    f.get_type_array(len, elem).unwrap()
}

fn map_local(fx: &mut Fx, off: i64, ty: Rc<Datatype>) {
    let stack = fx.stack();
    let addr = Address::new(Rc::clone(&stack), stack.wrap_offset(off as u64));
    let inv = Address::new_invalid();
    fx.fd.get_scope_local_mut().unwrap().add_symbol("buf", ty, &addr, &inv).unwrap();
}

fn bound_def(fx: &Fx, cmp: OpId, slot: int4) -> (OpCode, Option<i64>) {
    let bound = fx.fd.obank().get(cmp).unwrap().get_in(slot).unwrap();
    let def = fx.fd.vbank().get(bound).unwrap().get_def().unwrap();
    let o = fx.fd.obank().get(def).unwrap();
    let k = o.get_in(1).and_then(|c| fx.fd.vbank().get(c)).filter(|c| c.is_constant()).map(|c| c.get_offset() as i64);
    (o.code(), k)
}

#[test]
fn the_bound_is_rebuilt_on_an_array_covering_exactly_the_walk() {
    let mut dw = do_while(START, END, 1, 1, false, not_equal);
    map_local(&mut dw.fx, START, char_array(8));
    assert!(dw.fx.fd.start_type_recovery());
    let walks = dw.fx.walks();
    let stack = dw.fx.stack();
    assert_eq!(rebase_bounds(&mut dw.fx.fd, &stack, &walks), 1);
    assert_eq!(bound_def(&dw.fx, dw.cmp, 1), (OpCode::CPUI_INT_ADD, Some(8)));
    let again = dw.fx.walks();
    assert_eq!(again, walks, "the rebuilt bound is still the same walk");
    assert_eq!(rebase_bounds(&mut dw.fx.fd, &stack, &again), 0, "a rebuilt bound is not rebuilt again");
}

#[test]
fn the_bound_is_left_alone_without_that_exact_array() {
    let cases: Vec<(i64, Rc<Datatype>)> =
        vec![(START, char_array(16)), (START - 1, char_array(9)), (START, factory().get_base(8, type_metatype::TYPE_INT).unwrap())];
    for (off, ty) in cases {
        let mut dw = do_while(START, END, 1, 1, false, not_equal);
        map_local(&mut dw.fx, off, ty);
        assert!(dw.fx.fd.start_type_recovery());
        let walks = dw.fx.walks();
        let stack = dw.fx.stack();
        assert_eq!(rebase_bounds(&mut dw.fx.fd, &stack, &walks), 0);
        assert_eq!(bound_def(&dw.fx, dw.cmp, 1), (OpCode::CPUI_PTRSUB, Some(END)));
    }
    let mut none = do_while(START, END, 1, 1, false, not_equal);
    map_local(&mut none.fx, START, char_array(8));
    assert!(none.fx.fd.start_type_recovery());
    let stack = none.fx.stack();
    assert_eq!(rebase_bounds(&mut none.fx.fd, &stack, &[]), 0, "nothing is rebuilt without a proven walk");
    let mut early = do_while(START, END, 1, 1, false, not_equal);
    map_local(&mut early.fx, START, char_array(8));
    let (walks, stack) = (early.fx.walks(), early.fx.stack());
    assert_eq!(walks.len(), 1);
    assert_eq!(rebase_bounds(&mut early.fx.fd, &stack, &walks), 0, "nothing is rebuilt before type recovery");
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "stack", false, 8, 1, 4, addrspace_flags::hasphysical, 1, 1))
}

fn base(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

fn state(space: &Rc<AddrSpace>, types: &TypeFactoryImpl) -> MapState {
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(space), 0, 0x100);
    let dflt = types.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    MapState::new(Rc::clone(space), &rn, &RangeList::new(), dflt)
}

fn walk(start: i64, end: i64, step: i64) -> PointerWalk {
    PointerWalk { cmp: OpId::default(), slot: 1, start, end, step }
}

fn layout(state: &mut MapState) -> Vec<(i64, i32, RangeType, i32)> {
    let mut v: Vec<_> = state.hints_mut().iter().map(|h| (h.sstart, h.size, h.range_type, h.highind)).collect();
    v.sort();
    v
}

#[test]
fn constant_initializers_and_pointer_hints_become_one_buffer() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    let char1 = base(1, type_metatype::TYPE_INT);
    st.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x24, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x20, Some(Rc::clone(&char1)), 0, RangeType::Open, -1);
    st.add_range_pub(0x28, Some(base(8, type_metatype::TYPE_UINT)), 0, RangeType::Fixed, -1);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 1, RangeType::Open, 7), (0x28, 8, RangeType::Fixed, -1)]);
    let buf = st.hints_mut().iter().find(|h| h.sstart == 0x20).unwrap().type_.clone();
    assert!(Rc::ptr_eq(&buf, &char1), "the element type comes from the walk's own byte hint");
}

#[test]
fn a_store_straddling_either_edge_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let cases = [
        (0x24, 8, RangeType::Fixed, 0),
        (0x24, 8, RangeType::Fixed, COPY_CONSTANT),
        (0x1c, 8, RangeType::Fixed, 0),
        (0x1c, 8, RangeType::Fixed, COPY_CONSTANT),
        (0x1e, 4, RangeType::Open, 0),
    ];
    for (at, size, rt, flags) in cases {
        let mut st = state(&space, &types);
        st.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
        st.add_range_pub(at, Some(base(size, type_metatype::TYPE_UNKNOWN)), flags, rt, -1);
        let before = layout(&mut st);
        coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
        assert_eq!(layout(&mut st), before, "hint at {at:#x} size {size}");
    }
}

#[test]
fn a_fresh_walk_leaves_the_buffer_open_to_the_next_hint() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, -1);
    st.add_range_pub(0x24, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x28, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, -1);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 1, RangeType::Open, 7), (0x28, 1, RangeType::Open, -1)]);
}

#[test]
fn a_typed_value_or_locked_symbol_inside_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let cases = [
        (0, type_metatype::TYPE_INT, 4),
        (TYPELOCK, type_metatype::TYPE_UNKNOWN, 4),
        (TYPELOCK, type_metatype::TYPE_UNKNOWN, 1),
        (TYPELOCK | COPY_CONSTANT, type_metatype::TYPE_UNKNOWN, 1),
    ];
    for (flags, meta, size) in cases {
        let mut st = state(&space, &types);
        st.add_range_pub(0x24, Some(base(size, meta)), flags, RangeType::Fixed, -1);
        let before = layout(&mut st);
        coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
        assert_eq!(layout(&mut st), before);
    }
}

/// A read or store of a non-constant value wider or narrower than one element --
/// a union's scalar view of the bytes -- keeps the frame as it was.
#[test]
fn a_non_constant_access_that_is_not_one_element_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    for (at, size) in [(0x20, 8), (0x22, 2)] {
        let mut st = state(&space, &types);
        st.add_range_pub(at, Some(base(size, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Fixed, -1);
        let before = layout(&mut st);
        coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
        assert_eq!(layout(&mut st), before, "access at {at:#x} size {size}");
    }
    let mut st = state(&space, &types);
    st.add_range_pub(0x23, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Fixed, -1);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 1, RangeType::Open, 7)], "an untyped one-element access is an element");
}

#[test]
fn index_evidence_past_the_end_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, 15);
    let before = layout(&mut st);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), before);
}

#[test]
fn the_widest_nested_walk_wins_and_the_window_bounds_it() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1), walk(0x20, 0x30, 1), walk(0xf8, 0x108, 4)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 1, RangeType::Open, 15)]);
}

#[test]
fn walks_that_cross_without_nesting_are_both_left_alone() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    coalesce_hints(&mut st, &[walk(0x20, 0x30, 1), walk(0x28, 0x38, 1)], &space, &types);
    assert!(layout(&mut st).is_empty());
}

#[test]
fn entry_edges_that_start_at_different_addresses_are_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, left, right, head, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let pick = fx.register_input(0x50, 1);
    let e = fx.stack_addr(entry, START + 12);
    fx.cbranch(entry, pick);
    fx.edge(entry, left);
    fx.edge(entry, right);
    let s1 = fx.stack_addr(left, START);
    let s2 = fx.stack_addr(right, START + 4);
    fx.edge(left, head);
    fx.edge(right, head);
    let (phi, p) = fx.phi(head, 3);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s1);
    fx.set_in(phi, 1, s2);
    fx.set_in(phi, 2, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_back_edge_stepping_another_phi_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let s2 = fx.stack_addr(entry, START - 8);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi_p, p) = fx.phi(head, 2);
    let (phi_q, q) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let q1 = fx.add(head, q, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, q1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi_p, 0, s);
    fx.set_in(phi_p, 1, q1);
    fx.set_in(phi_q, 0, s2);
    fx.set_in(phi_q, 1, q1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_phi_whose_inputs_do_not_match_its_block_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, exit) = (fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 3);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.set_in(phi, 2, s);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

/// Two back edges step by one and by two; the header tests the pointer itself
/// and a two-byte read is guarded, but a pointer one short of the end can jump
/// over it.
#[test]
fn back_edges_that_step_differently_are_not_a_walk_at_either_width() {
    let mut fx = Fx::new(true);
    let (entry, head, body, one, two, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, START + 8);
    let pick = fx.register_input(0x50, 1);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 3);
    let (_, at_end) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p, e);
    fx.cbranch(head, at_end);
    fx.edge(head, body);
    fx.edge(head, exit);
    fx.load(body, p, 2);
    fx.cbranch(body, pick);
    fx.edge(body, one);
    fx.edge(body, two);
    let p1 = fx.add(one, p, 1);
    fx.edge(one, head);
    let p2 = fx.add(two, p, 2);
    fx.edge(two, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.set_in(phi, 2, p2);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

/// `while (1) { *p; if (p == end) break; p++; }` with the test in a later block
/// than the read, which therefore also reads `end`.
#[test]
fn a_read_before_a_later_test_of_the_pointer_is_not_guarded() {
    let mut fx = Fx::new(true);
    let (entry, head, test, latch, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    fx.edge(head, test);
    let (_, at_end) = fx.cmp(test, OpCode::CPUI_INT_EQUAL, p, e);
    fx.cbranch(test, at_end);
    fx.edge(test, latch);
    fx.edge(test, exit);
    let p1 = fx.add(latch, p, 1);
    fx.edge(latch, head);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

/// `for (p = buf; p != end; p++) *p; return *p;` reads `end` after the loop.
#[test]
fn a_read_after_a_loop_that_tested_the_pointer_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, head, body, exit) = (fx.block(), fx.block(), fx.block(), fx.block());
    let s = fx.stack_addr(entry, START);
    let e = fx.stack_addr(entry, END);
    fx.edge(entry, head);
    let (phi, p) = fx.phi(head, 2);
    let (_, at_end) = fx.cmp(head, OpCode::CPUI_INT_EQUAL, p, e);
    fx.cbranch(head, at_end);
    fx.edge(head, body);
    fx.edge(head, exit);
    fx.load(body, p, 1);
    let p1 = fx.add(body, p, 1);
    fx.edge(body, head);
    fx.load(exit, p, 1);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_bound_that_is_one_of_two_addresses_is_not_a_walk() {
    let mut fx = Fx::new(true);
    let (entry, left, right, join, head, exit) = (fx.block(), fx.block(), fx.block(), fx.block(), fx.block(), fx.block());
    let pick = fx.register_input(0x50, 1);
    fx.cbranch(entry, pick);
    fx.edge(entry, left);
    fx.edge(entry, right);
    let e1 = fx.stack_addr(left, END);
    let e2 = fx.stack_addr(right, END + 4);
    fx.edge(left, join);
    fx.edge(right, join);
    let (phi_e, e) = fx.phi(join, 2);
    let s = fx.stack_addr(join, START);
    fx.edge(join, head);
    let (phi, p) = fx.phi(head, 2);
    fx.load(head, p, 1);
    let p1 = fx.add(head, p, 1);
    let (_, cond) = fx.cmp(head, OpCode::CPUI_INT_NOTEQUAL, p1, e);
    fx.cbranch(head, cond);
    fx.edge(head, exit);
    fx.edge(head, head);
    fx.set_in(phi_e, 0, e1);
    fx.set_in(phi_e, 1, e2);
    fx.set_in(phi, 0, s);
    fx.set_in(phi, 1, p1);
    fx.finish(entry);
    assert!(fx.walks().is_empty());
}

#[test]
fn a_pointer_read_by_a_call_effect_another_phi_or_itself_is_not_a_walk() {
    let indirect = do_while(START, END, 1, 1, false, |fx, bl, p, p1, e| {
        let iop = fx.konst(0);
        fx.val(bl, OpCode::CPUI_INDIRECT, &[p, iop], 8);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(indirect.fx.walks().is_empty());
    let phi = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| {
        let other = fx.konst(0);
        fx.val(bl, OpCode::CPUI_MULTIEQUAL, &[p1, other], 8);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(phi.fx.walks().is_empty());
    let itself = do_while(START, START + 16, 8, 8, false, |fx, bl, p, p1, e| {
        let space = fx.konst(0);
        fx.emit(bl, OpCode::CPUI_STORE, &[space, p, p], None);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(itself.fx.walks().is_empty());
    let control = do_while(START, START + 16, 8, 8, false, not_equal);
    assert_eq!(control.fx.walks().len(), 1);
}

#[test]
fn a_read_before_the_pointer_is_not_a_walk() {
    let before = do_while(START, END, 1, 1, false, |fx, bl, p, p1, e| {
        let one = fx.konst(1);
        let q = fx.val(bl, OpCode::CPUI_INT_SUB, &[p, one], 8);
        fx.load(bl, q, 1);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert!(before.fx.walks().is_empty());
    let back_one = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| {
        let one = fx.konst(1);
        let q = fx.val(bl, OpCode::CPUI_INT_SUB, &[p1, one], 8);
        fx.load(bl, q, 1);
        fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
    });
    assert_eq!(back_one.fx.walks().len(), 1, "p + 1 - 1 is the element itself");
}

/// The end address also reaches something other than the comparison: a call
/// with or without a result, or a length computed from it.
#[test]
fn a_bound_also_used_elsewhere_keeps_its_definition() {
    let uses: [fn(&mut Fx, BlockId, VarnodeId, VarnodeId) -> OpCode; 3] = [
        |fx, bl, _p1, e| {
            let target = fx.konst(0x401000);
            fx.emit(bl, OpCode::CPUI_CALL, &[target, e], None);
            OpCode::CPUI_CALL
        },
        |fx, bl, _p1, e| {
            let target = fx.konst(0x401000);
            fx.emit(bl, OpCode::CPUI_CALL, &[target, e], Some(8));
            OpCode::CPUI_CALL
        },
        |fx, bl, p1, e| {
            fx.val(bl, OpCode::CPUI_INT_SUB, &[e, p1], 8);
            OpCode::CPUI_INT_SUB
        },
    ];
    for other in uses {
        let mut used = None;
        let mut dw = do_while(START, END, 1, 1, false, |fx, bl, _p, p1, e| {
            used = Some(other(fx, bl, p1, e));
            fx.cmp(bl, OpCode::CPUI_INT_NOTEQUAL, p1, e)
        });
        map_local(&mut dw.fx, START, char_array(8));
        assert!(dw.fx.fd.start_type_recovery());
        let (walks, stack) = (dw.fx.walks(), dw.fx.stack());
        let e = dw.fx.fd.obank().get(dw.cmp).unwrap().get_in(1).unwrap();
        assert_eq!(rebase_bounds(&mut dw.fx.fd, &stack, &walks), 1);
        assert_eq!(bound_def(&dw.fx, dw.cmp, 1), (OpCode::CPUI_INT_ADD, Some(8)));
        let def = dw.fx.fd.vbank().get(e).unwrap().get_def().unwrap();
        assert_eq!(dw.fx.fd.obank().get(def).unwrap().code(), OpCode::CPUI_PTRSUB, "{used:?} still reads the address");
        let fd = &dw.fx.fd;
        assert!(fd.vbank().get(e).unwrap().descend_iter().any(|d| Some(fd.obank().get(d).unwrap().code()) == used));
    }
}

#[test]
fn typed_index_evidence_of_another_width_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let int4 = base(4, type_metatype::TYPE_INT);
    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, -1);
    st.add_range_pub(0x20, Some(Rc::clone(&int4)), 0, RangeType::Open, 3);
    let before = layout(&mut st);
    coalesce_hints(&mut st, &[walk(0x20, 0x30, 1)], &space, &types);
    assert_eq!(layout(&mut st), before, "a byte walk keeps an int array's indexing");

    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(Rc::clone(&int4)), 0, RangeType::Open, 3);
    coalesce_hints(&mut st, &[walk(0x20, 0x30, 4)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 4, RangeType::Open, 3)]);
    let elem = st.hints_mut()[0].type_.clone();
    assert!(Rc::ptr_eq(&elem, &int4), "same-width index evidence types the elements");

    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, 3);
    coalesce_hints(&mut st, &[walk(0x20, 0x30, 1)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 1, RangeType::Open, 15)], "untyped index evidence is not a type");
}

#[test]
fn option_identity() {
    assert_eq!(ELEM_ENDPTRBOUND.get_id(), 4169);
    assert_eq!(OptionEndPtrBound::NAME, "endptrbound");
    assert!(OptionEndPtrBound.apply("on").unwrap().0);
    assert!(!OptionEndPtrBound.apply("off").unwrap().0);
    assert!(OptionEndPtrBound.apply("maybe").is_err());
}
