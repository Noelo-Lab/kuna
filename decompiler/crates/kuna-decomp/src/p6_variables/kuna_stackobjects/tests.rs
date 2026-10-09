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
