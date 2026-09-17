//! Unit tests for the `indirectonly` port: the `check_indirect_use` walk on
//! hand-built def-use graphs, and the illegal-input filter in
//! `mark_indirect_only`.
//!
//! The end-to-end behaviour (the merge a marked input unlocks) is covered by
//! `tests/stages/kuna-indirectonly.xml` (two-pass: off = the bug, on = the fix).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::options::KUNA_OPTION_NAMES;
use crate::varnode::varnode_flags;

use super::{check_indirect_use, mark_indirect_only, OptionIndirectOnly};

const SP_OFF: u64 = 0x20;

struct Fx {
    fd: Funcdata,
    bl: BlockId,
    pc: u64,
}

impl Fx {
    fn new() -> Fx {
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
        manage
            .insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &reg, 1, true, false)))
            .unwrap();
        let stack = Rc::clone(manage.get_stack_space().unwrap());
        let sp_data = VarnodeStorage { space: Some(Rc::clone(&reg)), offset: SP_OFF, size: 8 };
        manage.add_spacebase_pointer(&stack, &sp_data, 8, true).unwrap();
        let glb = ArchContext::new(manage);
        let entry = Address::new(Rc::clone(&reg), 0x1000);
        let mut fd =
            Funcdata::new("io", "io", Rc::new(glb), entry, 0x1000_0000, 0x40).unwrap();
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        Fx { fd, bl, pc: 0x1000 }
    }

    fn reg(&self) -> Rc<AddrSpace> {
        Rc::clone(self.fd.get_arch().manage().get_space_by_name("register").unwrap())
    }

    fn input(&mut self, off: u64) -> VarnodeId {
        let vn = self.fd.new_varnode(8, &Address::new(self.reg(), off), None);
        self.fd.set_input_varnode(vn).unwrap()
    }

    fn konst(&mut self, v: u64) -> VarnodeId {
        self.fd.new_constant(8, v)
    }

    fn emit(
        &mut self,
        code: OpCode,
        ins: &[VarnodeId],
        out: Option<int4>,
    ) -> (OpId, Option<VarnodeId>) {
        self.pc += 1;
        let bl = self.bl;
        let op = self.fd.new_op(ins.len() as int4, Address::new(self.reg(), self.pc));
        self.fd.op_set_opcode_code(op, code);
        let out = out.map(|size| self.fd.new_unique_out(size, op).unwrap());
        for (slot, vn) in ins.iter().enumerate() {
            self.fd.op_set_input(op, *vn, slot as int4).unwrap();
        }
        self.fd.op_insert(op, bl, None);
        (op, out)
    }

    /// A plain call-effect INDIRECT reading `vn` in slot 0.
    fn indirect(&mut self, vn: VarnodeId) -> VarnodeId {
        let iop = self.konst(0);
        self.emit(OpCode::CPUI_INDIRECT, &[vn, iop], Some(8)).1.unwrap()
    }

    /// An INDIRECT carrying the `indirect_store` flag (the STORE-caused form).
    fn indirect_store(&mut self, vn: VarnodeId) -> VarnodeId {
        let iop = self.konst(0);
        let (op, out) = self.emit(OpCode::CPUI_INDIRECT, &[vn, iop], Some(8));
        self.fd.obank_mut().get_mut(op).unwrap().set_flag(pcodeop_flags::indirect_store);
        out.unwrap()
    }

    fn phi(&mut self, a: VarnodeId, b: VarnodeId) -> VarnodeId {
        self.emit(OpCode::CPUI_MULTIEQUAL, &[a, b], Some(8)).1.unwrap()
    }

    /// A real arithmetic read — the case that must fail the whole test.
    fn direct_read(&mut self, vn: VarnodeId) -> VarnodeId {
        let k = self.konst(1);
        self.emit(OpCode::CPUI_INT_ADD, &[vn, k], Some(8)).1.unwrap()
    }

    fn is_flagged(&self, vn: VarnodeId) -> bool {
        self.fd.vbank().get(vn).map(|v| v.is_indirect_only()).unwrap_or(false)
    }
}

#[test]
fn an_input_read_only_by_an_indirect_is_marked() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    fx.indirect(vn);
    assert!(check_indirect_use(&fx.fd, vn));
    assert_eq!(mark_indirect_only(&mut fx.fd), 1);
    assert!(fx.is_flagged(vn));
}

/// The counterexample the whole design rests on: one genuine read anywhere in
/// the def-set and the input must NOT be marked, because the merge consumer
/// would then let an uninitialized value join a real variable.
#[test]
fn an_input_with_a_direct_read_is_not_marked() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    fx.indirect(vn);
    fx.direct_read(vn);
    assert!(!check_indirect_use(&fx.fd, vn));
    assert_eq!(mark_indirect_only(&mut fx.fd), 0);
    assert!(!fx.is_flagged(vn));
}

#[test]
fn flow_is_followed_through_multiequal() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let other = fx.konst(7);
    let merged = fx.phi(vn, other);
    fx.indirect(merged);
    assert!(check_indirect_use(&fx.fd, vn));

    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let other = fx.konst(7);
    let merged = fx.phi(vn, other);
    fx.direct_read(merged);
    assert!(!check_indirect_use(&fx.fd, vn));
}

#[test]
fn an_indirect_from_a_store_keeps_the_walk_going() {
    // The value survives the STORE, so a read of the INDIRECT's output still
    // counts against the input.
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let out = fx.indirect_store(vn);
    fx.direct_read(out);
    assert!(!check_indirect_use(&fx.fd, vn));

    // ... and a chain that ends in another INDIRECT still passes.
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let out = fx.indirect_store(vn);
    fx.indirect(out);
    assert!(check_indirect_use(&fx.fd, vn));
}

/// A plain (non-STORE) INDIRECT is the accepting case and the walk stops there:
/// what happens to its output is irrelevant.
#[test]
fn a_plain_indirect_output_is_not_followed() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let out = fx.indirect(vn);
    fx.direct_read(out);
    assert!(check_indirect_use(&fx.fd, vn));
}

#[test]
fn an_input_with_no_reads_at_all_is_marked() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    assert!(check_indirect_use(&fx.fd, vn));
    assert_eq!(mark_indirect_only(&mut fx.fd), 1);
}

/// `markIndirectOnly` only ever looks at *illegal* inputs — one that
/// `ActionDirectWrite` reached is skipped whatever its def-set looks like.
#[test]
fn a_directwrite_input_is_skipped() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    fx.indirect(vn);
    fx.fd.vbank_mut().get_mut(vn).unwrap().set_flags_pub(varnode_flags::directwrite);
    assert!(check_indirect_use(&fx.fd, vn));
    assert_eq!(mark_indirect_only(&mut fx.fd), 0);
    assert!(!fx.is_flagged(vn));
}

/// A cycle through MULTIEQUALs must terminate.
#[test]
fn a_multiequal_cycle_terminates() {
    let mut fx = Fx::new();
    let vn = fx.input(0x100);
    let other = fx.konst(7);
    let merged = fx.phi(vn, other);
    let (op, _) = fx.emit(OpCode::CPUI_MULTIEQUAL, &[merged, merged], Some(8));
    let looped = fx.fd.obank().get(op).unwrap().get_out().unwrap();
    fx.fd.op_set_input(op, looped, 1).unwrap();
    fx.indirect(looped);
    assert!(check_indirect_use(&fx.fd, vn));
}

#[test]
fn option_name_is_registered_and_parses() {
    assert_eq!(OptionIndirectOnly::NAME, "indirectonly");
    assert!(KUNA_OPTION_NAMES.contains(&OptionIndirectOnly::NAME));
    let (v, msg) = OptionIndirectOnly.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"));
    let (v, msg) = OptionIndirectOnly.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"));
    assert!(OptionIndirectOnly.apply("maybe").is_err());
}
