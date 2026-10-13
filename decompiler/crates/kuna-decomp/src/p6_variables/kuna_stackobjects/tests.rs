use super::*;
use crate::context::{ArchContext, BlockId};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace, UniqueSpace};

struct Fixture {
    fd: Funcdata,
    ram: Rc<AddrSpace>,
    stack: Rc<AddrSpace>,
}

impl Fixture {
    fn new(big: bool, width: u32) -> Self {
        let mut manage = AddrSpaceManager::new();
        manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manage
            .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
            .unwrap();
        let space = |kind, name, index| {
            Rc::new(AddrSpace::new(
                kind,
                name,
                big,
                width,
                1,
                index,
                addrspace_flags::hasphysical,
                1,
                1,
            ))
        };
        let ram = space(spacetype::IPTR_PROCESSOR, "ram", 2);
        let stack = space(spacetype::IPTR_SPACEBASE, "stack", 3);
        manage.insert_space(Rc::clone(&ram)).unwrap();
        manage.insert_space(Rc::clone(&stack)).unwrap();
        manage
            .insert_space(space(spacetype::IPTR_IOP, "iop", 4))
            .unwrap();
        let fd = Funcdata::new(
            "probe",
            "probe",
            Rc::new(ArchContext::new(manage)),
            Address::new(Rc::clone(&ram), 0x1000),
            0x10000000,
            0x100,
        )
        .unwrap();
        Self { fd, ram, stack }
    }

    fn block(&mut self) -> BlockId {
        let root = self.fd.bblocks_root_pub();
        self.fd.bblocks_mut().new_block_basic(root)
    }

    fn op(
        &mut self,
        block: BlockId,
        pc: u64,
        code: OpCode,
        inputs: &[VarnodeId],
        out: Option<(u64, i32)>,
    ) -> (OpId, Option<VarnodeId>) {
        let op = self
            .fd
            .new_op(inputs.len() as i32, Address::new(Rc::clone(&self.ram), pc));
        self.fd.op_set_opcode_code(op, code);
        self.fd.op_set_all_input(op, inputs).unwrap();
        let out = out.map(|(at, size)| {
            self.fd
                .new_varnode_out(size, &Address::new(Rc::clone(&self.stack), at), op)
                .unwrap()
        });
        self.fd.bb_insert_op(op, block, None);
        (op, out)
    }

    fn store(&mut self, block: BlockId, pc: u64, at: u64, size: i32) -> VarnodeId {
        let constant = self.fd.new_constant(size, 5);
        let (_, out) = self.op(block, pc, OpCode::CPUI_COPY, &[constant], Some((at, size)));
        let out = out.unwrap();
        self.fd.vbank_mut().get_mut(out).unwrap().set_stack_store();
        out
    }

    fn point(&mut self, block: BlockId, pc: u64) -> OpId {
        let value = self.fd.new_constant(1, 0);
        let op = self.op(block, pc, OpCode::CPUI_COPY, &[value], None).0;
        self.fd.new_unique_out(1, op).unwrap();
        op
    }

    fn bytes(&self, point: OpId, start: u64, size: i32) -> Vec<BTreeSet<ByteSource>> {
        let mut query = Origins {
            fd: &self.fd,
            space: Rc::clone(&self.stack),
            budget: MAX_STEPS,
            cache: BTreeMap::new(),
        };
        (0..size)
            .map(|byte| {
                query.before(
                    point,
                    self.stack.wrap_offset(start.wrapping_add(byte as u64)),
                )
            })
            .collect()
    }
}

fn write(pc: u64) -> ByteSource {
    ByteSource::Write(SourcePoint {
        space: 2,
        address: pc,
    })
}

#[test]
fn whole_and_partial_overwrites_change_only_their_bytes() {
    let mut f = Fixture::new(false, 8);
    let block = f.block();
    f.store(block, 0x1001, 0x20, 8);
    let first = f.point(block, 0x1002);
    f.store(block, 0x1003, 0x22, 1);
    let partial = f.point(block, 0x1004);
    f.store(block, 0x1005, 0x20, 8);
    let last = f.point(block, 0x1006);
    assert_eq!(
        f.bytes(first, 0x20, 8),
        vec![BTreeSet::from([write(0x1001)]); 8]
    );
    let mut expected = vec![BTreeSet::from([write(0x1001)]); 8];
    expected[2] = BTreeSet::from([write(0x1003)]);
    assert_eq!(f.bytes(partial, 0x20, 8), expected);
    assert_eq!(
        f.bytes(last, 0x20, 8),
        vec![BTreeSet::from([write(0x1005)]); 8]
    );
}

#[test]
fn synthetic_widening_retains_only_the_native_store_extent() {
    let mut f = Fixture::new(false, 8);
    let block = f.block();
    f.store(block, 0x1001, 0x22, 1);
    let constant = f.fd.new_constant(8, 0xff0000);
    f.op(block, 0x1001, OpCode::CPUI_COPY, &[constant], Some((0x20, 8)));
    let point = f.point(block, 0x1002);
    let mut expected = vec![BTreeSet::from([ByteSource::Unknown]); 8];
    expected[2] = BTreeSet::from([write(0x1001)]);
    assert_eq!(f.bytes(point, 0x20, 8), expected);
}

#[test]
fn synthetic_slice_retains_the_native_store_identity() {
    let mut f = Fixture::new(false, 8);
    let block = f.block();
    f.store(block, 0x1001, 0x20, 4);
    let constant = f.fd.new_constant(3, 5);
    f.op(block, 0x1001, OpCode::CPUI_COPY, &[constant], Some((0x20, 3)));
    let point = f.point(block, 0x1002);
    assert_eq!(f.bytes(point, 0x20, 4), vec![BTreeSet::from([write(0x1001)]); 4]);
}

#[test]
fn joins_retain_all_definition_families_and_missing_initializers() {
    let mut f = Fixture::new(false, 8);
    let entry = f.block();
    let left = f.block();
    let right = f.block();
    let join = f.block();
    for (from, to) in [(entry, left), (entry, right), (left, join), (right, join)] {
        f.fd.bblocks_mut().add_edge(from, to);
    }
    f.store(left, 0x1010, 0x20, 4);
    let point = f.point(join, 0x1030);
    assert_eq!(
        f.bytes(point, 0x20, 4),
        vec![BTreeSet::from([ByteSource::Incoming, write(0x1010)]); 4]
    );
    f.store(right, 0x1020, 0x20, 4);
    assert_eq!(
        f.bytes(point, 0x20, 4),
        vec![BTreeSet::from([write(0x1010), write(0x1020)]); 4]
    );
}

#[test]
fn loop_phis_retain_initial_and_backedge_definitions_without_iteration_ids() {
    let mut f = Fixture::new(false, 8);
    let entry = f.block();
    let body = f.block();
    f.fd.bblocks_mut().add_edge(entry, body);
    f.fd.bblocks_mut().add_edge(body, body);
    let initial = f.store(entry, 0x1001, 0x20, 4);
    let (_, phi) = f.op(
        body,
        0x1010,
        OpCode::CPUI_MULTIEQUAL,
        &[initial, initial],
        Some((0x20, 4)),
    );
    let point = f.point(body, 0x1011);
    let update = f.store(body, 0x1012, 0x20, 4);
    let phi = phi.unwrap();
    f.fd.op_set_input(f.fd.vbank().get(phi).unwrap().get_def().unwrap(), update, 1)
        .unwrap();
    assert_eq!(
        f.bytes(point, 0x20, 4),
        vec![BTreeSet::from([write(0x1001), write(0x1012)]); 4]
    );
}

#[test]
fn a_calls_own_indirect_guard_describes_its_output_not_its_input() {
    let mut f = Fixture::new(false, 8);
    let block = f.block();
    let initial = f.store(block, 0x1001, 0x20, 4);
    let destination = f.fd.new_constant(8, 0x2000);
    let call = f
        .op(block, 0x1002, OpCode::CPUI_CALL, &[destination], None)
        .0;
    let address = Address::new(Rc::clone(&f.stack), 0x20);
    let guard = f.fd.new_indirect_op(call, &address, 4, 0);
    f.fd.op_set_input(guard, initial, 0).unwrap();
    let later = f.point(block, 0x1003);
    assert_eq!(
        f.bytes(call, 0x20, 4),
        vec![BTreeSet::from([write(0x1001)]); 4]
    );
    let effect = ByteSource::Effect(SourcePoint {
        space: 2,
        address: 0x1002,
    });
    assert_eq!(
        f.bytes(later, 0x20, 4),
        vec![BTreeSet::from([write(0x1001), effect]); 4]
    );
}

#[test]
fn a_loop_retains_the_previous_iterations_effect_at_the_same_call_point() {
    let mut f = Fixture::new(false, 8);
    let entry = f.block();
    let body = f.block();
    f.fd.bblocks_mut().add_edge(entry, body);
    f.fd.bblocks_mut().add_edge(body, body);
    let initial = f.store(entry, 0x1001, 0x20, 4);
    let (phi, out) = f.op(
        body,
        0x1010,
        OpCode::CPUI_MULTIEQUAL,
        &[initial, initial],
        Some((0x20, 4)),
    );
    let destination = f.fd.new_constant(8, 0x2000);
    let call = f
        .op(body, 0x1011, OpCode::CPUI_CALL, &[destination], None)
        .0;
    let address = Address::new(Rc::clone(&f.stack), 0x20);
    let guard = f.fd.new_indirect_op(call, &address, 4, 0);
    f.fd.op_set_input(guard, out.unwrap(), 0).unwrap();
    let changed = f.fd.obank().get(guard).unwrap().get_out().unwrap();
    f.fd.op_set_input(phi, changed, 1).unwrap();
    let effect = ByteSource::Effect(SourcePoint {
        space: 2,
        address: 0x1011,
    });
    assert_eq!(
        f.bytes(call, 0x20, 4),
        vec![BTreeSet::from([write(0x1001), effect]); 4]
    );
}

#[test]
fn assembled_and_truncated_values_keep_byte_sources_in_both_endian_orders() {
    for big in [false, true] {
        let mut f = Fixture::new(big, 8);
        let block = f.block();
        let low = f.store(block, 0x1001, 0x20, 4);
        let high = f.store(block, 0x1002, 0x24, 4);
        let (_, piece) = f.op(
            block,
            0x1003,
            OpCode::CPUI_PIECE,
            &[high, low],
            Some((0x20, 8)),
        );
        let piece = piece.unwrap();
        f.fd.vbank_mut().get_mut(piece).unwrap().set_stack_store();
        let offset = f.fd.new_constant(4, 1);
        f.op(
            block,
            0x1004,
            OpCode::CPUI_SUBPIECE,
            &[piece, offset],
            Some((0x20, 2)),
        );
        let point = f.point(block, 0x1005);
        assert_eq!(
            f.bytes(point, 0x20, 2),
            vec![BTreeSet::from([write(0x1001)]); 2]
        );
        let expected = if big {
            vec![write(0x1002), write(0x1001)]
        } else {
            vec![write(0x1001), write(0x1002)]
        };
        let mut query = Origins {
            fd: &f.fd,
            space: Rc::clone(&f.stack),
            budget: MAX_STEPS,
            cache: BTreeMap::new(),
        };
        for (byte, source) in [(0, expected[0]), (7, expected[1])] {
            assert_eq!(query.value(piece, byte).unwrap(), BTreeSet::from([source]));
        }
    }
}

#[test]
fn wrapped_frame_offsets_and_exhausted_queries_stay_bounded() {
    let mut f = Fixture::new(false, 4);
    let block = f.block();
    f.store(block, 0x1001, 0xfffffffc, 8);
    let point = f.point(block, 0x1002);
    assert_eq!(
        f.bytes(point, 0xfffffffc, 8),
        vec![BTreeSet::from([write(0x1001)]); 8]
    );
    let mut query = Origins {
        fd: &f.fd,
        space: Rc::clone(&f.stack),
        budget: 0,
        cache: BTreeMap::new(),
    };
    assert_eq!(
        query.before(point, 0),
        BTreeSet::from([ByteSource::Unknown])
    );
}

#[test]
fn integer_frame_addresses_keep_unbounded_positive_parameter_aliases() {
    use crate::kuna_stackranges::{frame_address, may_alias};
    for width in [4, 8] {
        let mut f = Fixture::new(false, width);
        let block = f.block();
        let ty = Rc::new(Datatype::new(width as i32, crate::dtype::type_metatype::TYPE_UINT));
        let base = f.fd.new_varnode(
            width as i32,
            &Address::new(Rc::clone(&f.ram), 0x80),
            Some(Rc::clone(&ty)),
        );
        let base = f.fd.set_input_varnode(base).unwrap();
        f.fd.vbank_mut().get_mut(base).unwrap().set_spacebase();
        let index = f.fd.new_varnode(
            width as i32,
            &Address::new(Rc::clone(&f.ram), 0x90),
            Some(Rc::clone(&ty)),
        );
        let index = f.fd.set_input_varnode(index).unwrap();
        let (op, _) = f.op(block, 0x1001, OpCode::CPUI_INT_ADD, &[base, index], None);
        let address = f.fd.new_unique_out(width as i32, op).unwrap();
        f.fd.vn_update_type(address, ty);
        assert!(!may_alias(&f.fd, base, 8, 8));

        f.fd.vbank_mut().get_mut(index).unwrap().set_nz_mask(7);
        assert!(frame_address(&f.fd, address).is_some());
        assert!(!may_alias(&f.fd, address, 8, 8));
        f.fd.vbank_mut().get_mut(index).unwrap().set_nz_mask(15);
        assert!(may_alias(&f.fd, address, 8, 8));
        f.fd.vbank_mut().get_mut(index).unwrap()
            .set_nz_mask(kuna_base::address::calc_mask(width as i32));
        assert!(frame_address(&f.fd, address).is_none());
        assert!(may_alias(&f.fd, address, 8, 8));
    }
}

#[test]
fn wide_duplicate_pointer_choices_exhaust_the_alias_proof() {
    let mut f = Fixture::new(false, 8);
    let block = f.block();
    let global = f.fd.new_constant(8, 0x4000);
    let (op, _) = f.op(block, 0x1001, OpCode::CPUI_MULTIEQUAL, &vec![global; 1024], None);
    let address = f.fd.new_unique_out(8, op).unwrap();
    assert!(crate::kuna_stackranges::may_alias(&f.fd, address, 8, 8));
}

#[test]
fn absent_call_contracts_keep_frame_effects_and_later_locked_views() {
    use crate::dtype::{type_metatype, DatatypeKind};
    use crate::fspec::{FuncCallSpecs, ParameterPieces};
    for opcode in [OpCode::CPUI_CALL, OpCode::CPUI_CALLIND] {
        let mut f = Fixture::new(false, 8);
        let block = f.block();
        f.store(block, 0x1001, 0x20, 4);
        let target = f.fd.new_constant(8, 0x2000);
        let argument = f.fd.new_constant(8, 0x20);
        let (call, _) = f.op(block, 0x1002, opcode, &[target, argument], None);
        f.fd.obank_mut().mark_alive(call);
        let index = f.fd.push_call_specs(FuncCallSpecs::new(
            call, Address::new(Rc::clone(&f.ram), 0x2000),
        ));
        let point = f.point(block, 0x1003);
        let expected = vec![BTreeSet::from([
            write(0x1001), ByteSource::Effect(SourcePoint { space: 2, address: 0x1002 }),
        ]); 4];
        assert!(crate::kuna_stackuses::declared_view(&f.fd, call, 1).is_none());
        assert!(!f.fd.get_call_specs(index).proto().has_store());
        assert_eq!(f.bytes(point, 0x20, 4), expected);

        let value_type = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let mut pointer = Datatype::new(8, type_metatype::TYPE_PTR);
        pointer.kind = DatatypeKind::Pointer {
            ptrto: Rc::clone(&value_type), spaceid: None, truncate: None, wordsize: 1,
        };
        let parameter_address = Address::new(Rc::clone(&f.ram), 0x80);
        let proto = f.fd.get_call_specs_mut(index).proto_mut();
        proto.attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        proto.set_param(0, "value", &ParameterPieces {
            addr: parameter_address, type_: Some(Rc::new(pointer)), flags: 0,
        });
        assert!(crate::kuna_stackuses::declared_view(&f.fd, call, 1).is_none());
        f.fd.get_call_specs_mut(index).proto_mut().set_input_lock(true);
        let view = crate::kuna_stackuses::declared_view(&f.fd, call, 1).unwrap();
        assert!(Rc::ptr_eq(&view, &value_type));
        assert_eq!(f.bytes(point, 0x20, 4), expected);
    }
}
