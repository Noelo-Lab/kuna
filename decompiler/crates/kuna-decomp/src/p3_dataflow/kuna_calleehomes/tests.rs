use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, BlockId, OpId, VarnodeId};
use crate::dtype::{type_class, type_metatype};
use crate::fspec::{FuncCallSpecs, FuncProto, ParamEntry, ParameterPieces, ProtoModel};
use crate::funcdata::Funcdata;
use crate::p6_variables::funcdata_spacebase::spacebase_tests::{build_fd, make_sp_in, reg_addr};

use super::{
    caller_home_cell, caller_home_unobserved, frame_value_overlaps, is_home_slot,
    verified_home_gap, MAX_CALLER_SCAN_OPS,
};

const HOME: i64 = -40;

pub(crate) enum Out {
    Unique(int4),
    At(Rc<AddrSpace>, u64, int4),
}

pub(crate) struct Fx {
    pub(crate) fd: Funcdata,
    block: BlockId,
    pub(crate) reg: Rc<AddrSpace>,
    other: Rc<AddrSpace>,
    pub(crate) stack: Rc<AddrSpace>,
    sp: VarnodeId,
    pc: u64,
}

impl Fx {
    pub(crate) fn new() -> Self {
        Self::with_return_address(Some((0, 8)))
    }

    fn with_return_address(return_address: Option<(u64, u32)>) -> Self {
        let original = build_fd();
        let mut manager = Rc::clone(&original.get_arch().manage);
        let entry = original.get_address().clone();
        drop(original);
        let manager_mut = Rc::get_mut(&mut manager).unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            0,
            0,
            0,
        ));
        manager_mut.insert_space(Rc::clone(&ram)).unwrap();
        let code_index = manager_mut
            .get_space_by_name("register")
            .unwrap()
            .get_index();
        manager_mut.set_default_code_space(code_index).unwrap();
        manager_mut.set_default_data_space(ram.get_index()).unwrap();
        let stack = Rc::clone(manager.get_stack_space().unwrap());
        let mut arch = ArchContext::new_shared(Rc::clone(&manager));
        arch.default_return_addr =
            return_address.map(|(offset, size)| kuna_num::pcoderaw::VarnodeData {
                space: Some(Rc::clone(&stack)),
                offset,
                size,
            });
        let fd = Funcdata::new("func", "func", Rc::new(arch), entry, 0x1000_0000, 0x40).unwrap();
        let mut fd = fd;
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

    pub(crate) fn constant(&mut self, size: int4, value: u64) -> VarnodeId {
        self.fd.new_constant(size, value)
    }

    pub(crate) fn emit(
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

    pub(crate) fn frame_pointer(&mut self, offset: i64) -> VarnodeId {
        let amount = self.constant(8, offset as u64);
        let sp = self.sp;
        self.emit(OpCode::CPUI_INT_ADD, &[sp, amount], Some(Out::Unique(8)))
            .1
            .unwrap()
    }

    pub(crate) fn marker(&mut self) -> OpId {
        let value = self.constant(8, 0);
        self.emit(OpCode::CPUI_COPY, &[value], Some(Out::Unique(8)))
            .0
    }

    fn stack_load(&mut self, offset: i64) -> OpId {
        let space_id = self.reg.get_index() as u64;
        self.stack_load_in(offset, space_id)
    }

    fn stack_load_in(&mut self, offset: i64, space_id: u64) -> OpId {
        let pointer = self.frame_pointer(offset);
        let space = self.constant(4, space_id);
        self.emit(OpCode::CPUI_LOAD, &[space, pointer], Some(Out::Unique(8)))
            .0
    }

    fn store_pointer_to_global(&mut self, pointer: VarnodeId) -> OpId {
        let space_id = self
            .fd
            .get_arch()
            .manage()
            .get_default_data_space()
            .unwrap()
            .get_index() as u64;
        let space = self.constant(4, space_id);
        let address = self.constant(8, 0x8000);
        self.emit(OpCode::CPUI_STORE, &[space, address, pointer], None)
            .0
    }

    fn direct_call(&mut self, argument: Option<VarnodeId>) -> OpId {
        let target = self.constant(8, 0x2000);
        let mut inputs = vec![target];
        inputs.extend(argument);
        self.emit(OpCode::CPUI_CALL, &inputs, None).0
    }
}

fn locked_home_call(fx: &mut Fx, stack_base: u64, variadic: bool) -> FuncCallSpecs {
    let op = fx.marker();
    locked_home_call_on(fx, op, stack_base, variadic)
}

pub(crate) fn locked_home_call_on(
    fx: &mut Fx,
    op: OpId,
    stack_base: u64,
    variadic: bool,
) -> FuncCallSpecs {
    let manager = fx.fd.get_arch().manage();
    let mut model = ProtoModel::new(manager);
    model.build_param_list("standard").unwrap();
    model.set_name("win64-home-test");
    let stack = Rc::clone(&fx.stack);
    let entry = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        stack,
        stack_base,
        64,
        1,
        8,
        0,
        true,
        false,
        &[],
        manager,
    )
    .unwrap();
    model.input_mut().push_entry(entry);
    model.input_mut().finish_decode();
    model.output_mut().finish_decode();
    let void = Rc::new(crate::dtype::Datatype::new(
        0,
        crate::dtype::type_metatype::TYPE_VOID,
    ));
    let mut proto = FuncProto::new();
    proto.set_internal(Rc::new(model), void);
    proto.set_dotdotdot(variadic);
    proto.set_input_lock(true);
    proto.set_model_lock(true);
    assert!(proto.is_input_locked());
    assert!(proto.is_model_locked());
    let mut call = FuncCallSpecs::new(op, Address::new(Rc::clone(&fx.reg), 0x2000));
    call.proto_mut().copy(&proto);
    call
}

fn call_with_proto(fx: &mut Fx, proto: &FuncProto) -> FuncCallSpecs {
    let op = fx.marker();
    let mut call = FuncCallSpecs::new(op, Address::new(Rc::clone(&fx.reg), 0x2000));
    call.proto_mut().copy(proto);
    call
}

pub(crate) fn call_with_r8_argument(
    fx: &mut Fx,
    first_actual: VarnodeId,
    stack_base: u64,
    r8_actual: VarnodeId,
) -> OpId {
    let stack_ref = fx.fd.new_varnode(
        8,
        &Address::new(Rc::clone(&fx.stack), (-56i64) as u64),
        None,
    );
    let (_, placeholder) = fx.emit(OpCode::CPUI_COPY, &[stack_ref], Some(Out::Unique(8)));
    let placeholder = placeholder.unwrap();
    let target = fx.constant(8, 0x2000);
    let call_op = fx
        .emit(
            OpCode::CPUI_CALL,
            &[target, first_actual, placeholder, r8_actual],
            None,
        )
        .0;
    let mut call = locked_home_call_on(fx, call_op, stack_base, false);
    let value_type = Rc::new(crate::dtype::Datatype::new(8, type_metatype::TYPE_PTR));
    call.proto_mut().set_param(
        0,
        "first",
        &ParameterPieces {
            addr: Address::new(Rc::clone(&fx.reg), 0x20),
            type_: Some(Rc::clone(&value_type)),
            flags: 0,
        },
    );
    call.proto_mut().set_param(
        1,
        "stack_arg",
        &ParameterPieces {
            addr: Address::new(Rc::clone(&fx.stack), stack_base),
            type_: Some(Rc::clone(&value_type)),
            flags: 0,
        },
    );
    call.proto_mut().set_param(
        2,
        "third",
        &ParameterPieces {
            addr: Address::new(Rc::clone(&fx.reg), 0x28),
            type_: Some(value_type),
            flags: 0,
        },
    );
    call.proto_mut().resolve_extra_pop();
    call.resolve_spacebase_relative(&mut fx.fd, placeholder)
        .unwrap();
    fx.fd.push_call_specs(call);
    call_op
}

#[test]
fn only_the_exact_model_declared_home_gap_contains_deferred_cells() {
    assert!(is_home_slot(8, 8));
    assert!(is_home_slot(32, 8));
    assert!(!is_home_slot(7, 8));
    assert!(!is_home_slot(36, 8));
    assert!(!is_home_slot(40, 8));
    assert!(!is_home_slot(8, 0));

    let mut fx = Fx::new();
    let call = locked_home_call(&mut fx, 40, false);
    assert!(verified_home_gap(&fx.fd, &call).is_some());
    assert!(caller_home_cell(&fx.fd, &call, 16, 8).is_none());
    let wrong = locked_home_call(&mut fx, 48, false);
    assert!(verified_home_gap(&fx.fd, &wrong).is_none());
    let variadic = locked_home_call(&mut fx, 40, true);
    assert!(verified_home_gap(&fx.fd, &variadic).is_none());
}

#[test]
fn home_gap_requires_a_declared_eight_byte_return_slot_before_it() {
    let mut fx = Fx::with_return_address(None);
    let call = locked_home_call(&mut fx, 40, false);
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut fx = Fx::with_return_address(Some((1, 8)));
    let call = locked_home_call(&mut fx, 40, false);
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut fx = Fx::with_return_address(Some((0, 4)));
    let call = locked_home_call(&mut fx, 40, false);
    assert!(verified_home_gap(&fx.fd, &call).is_none());
}

#[test]
fn home_gap_requires_a_stored_locked_resolved_known_model() {
    let mut fx = Fx::new();

    let no_model = FuncProto::new();
    let call = call_with_proto(&mut fx, &no_model);
    assert!(!call.proto().has_model());
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let valid = locked_home_call(&mut fx, 40, false);
    let base = valid.proto().model().as_ref().clone();

    let mut no_store = FuncProto::new();
    no_store.set_model(Some(Rc::new(base.clone())));
    no_store.set_model_lock(true);
    let call = call_with_proto(&mut fx, &no_store);
    assert!(call.proto().has_model());
    assert!(!call.proto().has_store());
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut unlocked = FuncProto::new();
    unlocked.set_internal(
        Rc::new(base.clone()),
        Rc::new(crate::dtype::Datatype::new(
            1,
            crate::dtype::type_metatype::TYPE_VOID,
        )),
    );
    let call = call_with_proto(&mut fx, &unlocked);
    assert!(call.proto().has_store());
    assert!(!call.proto().is_input_locked());
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut unlocked_model = FuncProto::new();
    unlocked_model.set_internal(
        Rc::new(base.clone()),
        Rc::new(crate::dtype::Datatype::new(
            1,
            crate::dtype::type_metatype::TYPE_VOID,
        )),
    );
    unlocked_model.set_input_lock(true);
    unlocked_model.set_model_lock(false);
    let call = call_with_proto(&mut fx, &unlocked_model);
    assert!(call.proto().is_input_locked());
    assert!(!call.proto().is_model_locked());
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut unresolved = FuncProto::new();
    unresolved.set_internal(
        Rc::new(ProtoModel::new(fx.fd.get_arch().manage())),
        Rc::new(crate::dtype::Datatype::new(
            1,
            crate::dtype::type_metatype::TYPE_VOID,
        )),
    );
    unresolved.set_input_lock(true);
    unresolved.set_model_lock(true);
    let call = call_with_proto(&mut fx, &unresolved);
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut merged = FuncProto::new();
    merged.set_internal(
        Rc::new(ProtoModel::new_merged(fx.fd.get_arch().manage())),
        Rc::new(crate::dtype::Datatype::new(
            1,
            crate::dtype::type_metatype::TYPE_VOID,
        )),
    );
    merged.set_input_lock(true);
    merged.set_model_lock(true);
    let call = call_with_proto(&mut fx, &merged);
    assert!(call.proto().model().is_merged());
    assert!(verified_home_gap(&fx.fd, &call).is_none());

    let mut unknown = FuncProto::new();
    unknown.set_internal(
        Rc::new(ProtoModel::new_unknown("test-unknown", &base)),
        Rc::new(crate::dtype::Datatype::new(
            1,
            crate::dtype::type_metatype::TYPE_VOID,
        )),
    );
    unknown.set_input_lock(true);
    unknown.set_model_lock(true);
    let call = call_with_proto(&mut fx, &unknown);
    assert!(call.proto().is_model_unknown());
    assert!(verified_home_gap(&fx.fd, &call).is_none());
}

#[test]
fn caller_stack_reads_and_direct_stack_variables_observe_home_cells() {
    let mut fx = Fx::new();
    let marker = fx.marker();
    fx.stack_load(HOME);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let other_space = fx.other.get_index() as u64;
    fx.stack_load_in(HOME, other_space);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let stack_var = fx
        .fd
        .new_varnode(8, &Address::new(Rc::clone(&fx.stack), HOME as u64), None);
    fx.emit(OpCode::CPUI_COPY, &[stack_var], Some(Out::Unique(8)));
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));
}

#[test]
fn caller_forwarding_or_storing_a_home_address_is_an_escape() {
    let mut fx = Fx::new();
    let marker = fx.marker();
    let pointer = fx.frame_pointer(HOME);
    assert_eq!(frame_value_overlaps(&fx.fd, pointer, HOME, 8), Some(true));
    fx.store_pointer_to_global(pointer);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let pointer = fx.frame_pointer(HOME);
    let _call = fx.direct_call(Some(pointer));
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let pointer = fx.frame_pointer(HOME);
    fx.emit(OpCode::CPUI_RETURN, &[pointer], None);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));
}

#[test]
fn disjoint_frame_pointer_escapes_still_block_home_proof() {
    let mut fx = Fx::new();
    let marker = fx.marker();
    let pointer = fx.frame_pointer(-80);
    assert_eq!(frame_value_overlaps(&fx.fd, pointer, HOME, 8), Some(false));
    fx.store_pointer_to_global(pointer);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let pointer = fx.frame_pointer(-80);
    assert_eq!(frame_value_overlaps(&fx.fd, pointer, HOME, 8), Some(false));
    let ram = Rc::clone(fx.fd.get_arch().manage().get_default_data_space().unwrap());
    let (_, published) = fx.emit(OpCode::CPUI_COPY, &[pointer], Some(Out::At(ram, 0x8000, 8)));
    assert!(!fx.fd.vbank().get(published.unwrap()).unwrap().is_persist());
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    fx.frame_pointer(-80);
    assert!(caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));
}

#[test]
fn an_unknown_other_call_remains_a_barrier() {
    let mut fx = Fx::new();
    let marker = fx.marker();
    fx.direct_call(None);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));

    let mut fx = Fx::new();
    let marker = fx.marker();
    let target = fx.constant(8, 0x2000);
    fx.emit(OpCode::CPUI_CALLIND, &[target], None);
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));
}

#[test]
fn oversized_callers_fail_closed_at_the_scan_budget() {
    let mut fx = Fx::new();
    let marker = fx.marker();
    for _ in 0..MAX_CALLER_SCAN_OPS {
        fx.marker();
    }
    assert!(!caller_home_unobserved(&fx.fd, marker, HOME as u64, 8));
}
