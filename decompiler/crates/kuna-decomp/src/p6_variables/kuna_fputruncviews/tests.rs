use super::*;
use std::rc::Rc;

use crate::context::{ArchContext, BlockId, VarnodeId};
use crate::dtype::TypeFactoryImpl;
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

struct Fixture {
    fd: Funcdata,
    ram: Rc<AddrSpace>,
    left: BlockId,
    right: BlockId,
    join: BlockId,
}

impl Fixture {
    fn new() -> Self {
        let mut spaces = AddrSpaceManager::new();
        spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        spaces
            .insert_space(Rc::new(UniqueSpace::new(1, 0, true)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            true,
            4,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        spaces.insert_space(Rc::clone(&ram)).unwrap();
        let factory = Rc::new(TypeFactoryImpl::new());
        factory.set_default_alignment_map();
        factory.set_max_basetype_size(8);
        for (name, size, meta) in [
            ("undefined", 1, M::TYPE_UNKNOWN),
            ("int", 4, M::TYPE_INT),
            ("uint", 4, M::TYPE_UINT),
            ("float", 4, M::TYPE_FLOAT),
            ("double", 8, M::TYPE_FLOAT),
        ] {
            factory.set_core_type(name, size, meta, false).unwrap();
        }
        factory.cache_core_types().unwrap();
        let mut arch = ArchContext::new(spaces);
        arch.fpu_trunc_views = true;
        arch.types = Some(factory);
        let mut fd = Funcdata::new(
            "probe",
            "probe",
            Rc::new(arch),
            Address::new(Rc::clone(&ram), 0x1000),
            0x10000000,
            0x100,
        )
        .unwrap();
        fd.set_high_level();
        let graph = fd.bblocks_root_pub();
        let entry = fd.bblocks_mut().new_block_basic(graph);
        let left = fd.bblocks_mut().new_block_basic(graph);
        let right = fd.bblocks_mut().new_block_basic(graph);
        let join = fd.bblocks_mut().new_block_basic(graph);
        for (index, block) in [entry, left, right, join].into_iter().enumerate() {
            fd.bblocks_mut().block_mut(block).set_index(index as i32);
        }
        for (a, b) in [(entry, left), (entry, right), (left, join), (right, join)] {
            fd.bblocks_mut().add_edge(a, b);
        }
        fd.bblocks_mut().calc_forward_dominator(graph, &[entry]);
        Self {
            fd,
            ram,
            left,
            right,
            join,
        }
    }

    fn value(
        &mut self,
        block: BlockId,
        pc: u64,
        code: OpCode,
        inputs: &[VarnodeId],
        size: i32,
        meta: M,
    ) -> (OpId, VarnodeId) {
        let op = self
            .fd
            .new_op(inputs.len() as i32, Address::new(Rc::clone(&self.ram), pc));
        self.fd.op_set_opcode_code(op, code);
        self.fd.op_set_all_input(op, inputs).unwrap();
        let out = self.fd.new_unique_out(size, op).unwrap();
        let ty = self
            .fd
            .get_arch()
            .types()
            .unwrap()
            .get_base(size, meta)
            .unwrap();
        self.fd.vn_update_type(out, ty);
        self.fd.bb_insert_op(op, block, None);
        self.fd.obank_mut().mark_alive(op);
        (op, out)
    }

    fn root(&mut self, meta: M) -> VarnodeId {
        let input = self.fd.new_constant(8, 8183.75f64.to_bits());
        self.value(
            self.left,
            0x1010,
            OpCode::CPUI_FLOAT_TRUNC,
            &[input],
            4,
            meta,
        )
        .1
    }

    fn floating_copy(&mut self, root: VarnodeId) -> OpId {
        self.value(
            self.left,
            0x1014,
            OpCode::CPUI_COPY,
            &[root],
            4,
            M::TYPE_FLOAT,
        )
        .0
    }

    fn bridges(&self) -> Vec<OpId> {
        self.fd
            .obank()
            .iter_alive()
            .filter(|op| {
                self.fd.obank().get(*op).unwrap().get_addlflags()
                    & pcodeop_addlflags::kuna_fpu_view_copy
                    != 0
            })
            .collect()
    }
}

#[test]
fn mixed_phi_slot_orders_leave_required_merge_invariants_intact() {
    for reverse in [false, true] {
        let mut f = Fixture::new();
        if reverse {
            std::mem::swap(&mut f.left, &mut f.right);
        }
        let root = f.root(M::TYPE_FLOAT);
        let bits = f.fd.new_constant(4, 1.5f32.to_bits().into());
        let other = f
            .value(
                f.right,
                0x1020,
                OpCode::CPUI_COPY,
                &[bits],
                4,
                M::TYPE_FLOAT,
            )
            .1;
        let inputs = if reverse {
            [other, root]
        } else {
            [root, other]
        };
        let (_phi, out) = f.value(
            f.join,
            0x1030,
            OpCode::CPUI_MULTIEQUAL,
            &inputs,
            4,
            M::TYPE_INT,
        );
        recover(&mut f.fd).unwrap();
        assert_eq!(f.bridges().len(), 1);
        f.fd.with_covermerge(|merge, fd| merge.merge_marker(fd))
            .unwrap();
        assert_ne!(
            f.fd.vbank().get(root).unwrap().get_high(),
            f.fd.vbank().get(out).unwrap().get_high()
        );
        assert!(!f.fd.vbank().get(root).unwrap().is_type_lock());
        assert_eq!(
            f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
            M::TYPE_INT
        );
    }
}

#[test]
fn an_unsigned_logical_result_keeps_its_unsigned_type_without_a_lock() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_UINT);
    let use_ = f.floating_copy(root);
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges().len(), 1);
    assert_eq!(
        f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
        M::TYPE_UINT
    );
    assert!(!f.fd.vbank().get(root).unwrap().is_type_lock());
    assert_ne!(f.fd.obank().get(use_).unwrap().get_in(0), Some(root));
}

#[test]
fn user_locks_and_partial_inputs_are_not_rewritten() {
    for flag in [
        varnode_flags::typelock,
        varnode_flags::namelock,
        varnode_flags::mapped,
        varnode_flags::proto_partial,
    ] {
        let mut f = Fixture::new();
        let root = f.root(M::TYPE_FLOAT);
        let use_ = f.floating_copy(root);
        f.fd.vbank_mut().get_mut(root).unwrap().set_flags_pub(flag);
        let high = f.fd.vbank().get(root).unwrap().get_high().unwrap();
        f.fd.high_bank_mut().get_mut(high).unwrap().flags_dirty();
        recover(&mut f.fd).unwrap();
        assert!(f.bridges().is_empty());
        assert_eq!(f.fd.obank().get(use_).unwrap().get_in(0), Some(root));
        assert_eq!(
            f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
            M::TYPE_FLOAT
        );
    }
}

#[test]
fn a_conversion_used_as_part_of_a_wider_value_is_not_rewritten() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    f.floating_copy(root);
    let upper = f.fd.new_constant(4, 0x3ff00000);
    f.value(
        f.left,
        0x1018,
        OpCode::CPUI_PIECE,
        &[upper, root],
        8,
        M::TYPE_FLOAT,
    );
    recover(&mut f.fd).unwrap();
    assert!(f.bridges().is_empty());
    assert_eq!(
        f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
        M::TYPE_FLOAT
    );
}

#[test]
fn ordinary_numeric_conversions_keep_their_existing_recovery() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_INT);
    f.value(f.left, 0x1014, OpCode::CPUI_COPY, &[root], 4, M::TYPE_INT);
    recover(&mut f.fd).unwrap();
    assert!(f.bridges().is_empty());
}

#[test]
fn floating_reads_of_a_narrower_piece_are_not_rewritten() {
    let mut f = Fixture::new();
    let input = f.fd.new_constant(8, 8183.75f64.to_bits());
    let root = f
        .value(
            f.left,
            0x1010,
            OpCode::CPUI_FLOAT_TRUNC,
            &[input],
            8,
            M::TYPE_FLOAT,
        )
        .1;
    let zero = f.fd.new_constant(4, 0);
    f.value(
        f.left,
        0x1014,
        OpCode::CPUI_SUBPIECE,
        &[root, zero],
        4,
        M::TYPE_FLOAT,
    );
    recover(&mut f.fd).unwrap();
    assert!(f.bridges().is_empty());
    assert_eq!(
        f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
        M::TYPE_FLOAT
    );
}

#[test]
fn repeated_recovery_preserves_the_same_storage_bridge() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    let reader = f.floating_copy(root);
    recover(&mut f.fd).unwrap();
    let bridges = f.bridges();
    let input = f.fd.obank().get(reader).unwrap().get_in(0);
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges(), bridges);
    assert_eq!(f.fd.obank().get(reader).unwrap().get_in(0), input);
    assert_eq!(
        f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
        M::TYPE_INT
    );
}

#[test]
fn speculative_merges_cannot_rejoin_the_storage_view_and_logical_root() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    let user = f.floating_copy(root);
    recover(&mut f.fd).unwrap();
    let view = f.fd.obank().get(user).unwrap().get_in(0).unwrap();
    let root_high = f.fd.vbank().get(root).unwrap().get_high().unwrap();
    let view_high = f.fd.vbank().get(view).unwrap().get_high().unwrap();
    assert!(keeps_highs_apart(&f.fd, root_high, view_high));
    assert!(keeps_highs_apart(&f.fd, view_high, root_high));
    f.fd.with_covermerge(|merge, fd| merge.merge_linear(fd, &mut [root_high, view_high]))
        .unwrap();
    f.fd.with_covermerge(|merge, fd| merge.merge_adjacent(fd))
        .unwrap();
    assert_ne!(
        f.fd.vbank().get(root).unwrap().get_high(),
        f.fd.vbank().get(view).unwrap().get_high()
    );
}

#[test]
fn speculative_merges_keep_the_logical_root_off_unrelated_integers() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    f.floating_copy(root);
    let seed = f.fd.new_constant(4, 7);
    let (_, other) = f.value(f.right, 0x1020, OpCode::CPUI_COPY, &[seed], 4, M::TYPE_INT);
    recover(&mut f.fd).unwrap();
    let root_high = f.fd.vbank().get(root).unwrap().get_high().unwrap();
    let other_high = f.fd.vbank().get(other).unwrap().get_high().unwrap();
    assert!(keeps_root_alone(&f.fd, root_high, other_high));
    assert!(keeps_root_alone(&f.fd, other_high, root_high));
    f.fd.with_covermerge(|merge, fd| merge.merge_linear(fd, &mut [root_high, other_high]))
        .unwrap();
    assert_ne!(
        f.fd.vbank().get(root).unwrap().get_high(),
        f.fd.vbank().get(other).unwrap().get_high()
    );
}

#[test]
fn repeated_floating_read_slots_share_one_bridge() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    let (user, _) = f.value(
        f.left,
        0x1014,
        OpCode::CPUI_FLOAT_MULT,
        &[root, root],
        4,
        M::TYPE_FLOAT,
    );
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges().len(), 1);
    let operation = f.fd.obank().get(user).unwrap();
    assert_ne!(operation.get_in(0), Some(root));
    assert_eq!(operation.get_in(0), operation.get_in(1));
}

#[test]
fn an_integer_copy_with_only_logical_readers_stays_on_the_integer_root() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    f.floating_copy(root);
    let (copy, integer) = f.value(f.left, 0x1018, OpCode::CPUI_COPY, &[root], 4, M::TYPE_INT);
    let one = f.fd.new_constant(4, 1);
    f.value(
        f.left,
        0x101c,
        OpCode::CPUI_INT_ADD,
        &[integer, one],
        4,
        M::TYPE_INT,
    );
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges().len(), 1);
    assert_eq!(f.fd.obank().get(copy).unwrap().get_in(0), Some(root));
}

#[test]
fn an_integer_read_with_a_double_result_does_not_block_recovery() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_FLOAT);
    f.floating_copy(root);
    let (reader, _) = f.value(
        f.left,
        0x1018,
        OpCode::CPUI_FLOAT_INT2FLOAT,
        &[root],
        8,
        M::TYPE_FLOAT,
    );
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges().len(), 1);
    assert_eq!(f.fd.obank().get(reader).unwrap().get_in(0), Some(root));
}

#[test]
fn typed_floating_call_and_return_receive_the_bit_storage_view() {
    use crate::dtype::Datatype;
    use crate::fspec::{FuncCallSpecs, ParameterPieces, ProtoStoreInternal};
    for returning in [false, true] {
        let mut f = Fixture::new();
        let root = f.root(M::TYPE_FLOAT);
        f.floating_copy(root);
        let target = f.fd.new_constant(4, 0x2000);
        let code = if returning {
            OpCode::CPUI_RETURN
        } else {
            OpCode::CPUI_CALL
        };
        let (consumer, _) = f.value(f.left, 0x1018, code, &[target, root], 4, M::TYPE_INT);
        let float =
            f.fd.get_arch()
                .types()
                .unwrap()
                .get_base(4, M::TYPE_FLOAT)
                .unwrap();
        let piece = ParameterPieces {
            addr: Address::new(Rc::clone(&f.ram), 0x10),
            type_: Some(float),
            flags: 0,
        };
        let store = Box::new(ProtoStoreInternal::new(Rc::new(Datatype::new(
            0,
            M::TYPE_VOID,
        ))));
        if returning {
            f.fd.get_func_proto_mut().set_store(store);
            f.fd.get_func_proto_mut().set_output(&piece);
        } else {
            let mut call = FuncCallSpecs::new(consumer, Address::new(Rc::clone(&f.ram), 0x2000));
            call.proto_mut().set_store(store);
            call.proto_mut().set_param(0, "value", &piece);
            f.fd.push_call_specs(call);
        }
        recover(&mut f.fd).unwrap();
        assert_eq!(f.bridges().len(), 1);
        let view = f.fd.obank().get(consumer).unwrap().get_in(1).unwrap();
        assert_ne!(view, root);
        assert_eq!(
            f.fd.vbank().get(view).unwrap().get_type().get_metatype(),
            M::TYPE_FLOAT
        );
        assert_eq!(
            f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
            M::TYPE_INT
        );
    }
}

#[test]
fn integer_phis_do_not_acquire_a_floating_storage_view() {
    for two_truncs in [false, true] {
        for reverse in [false, true] {
            let mut f = Fixture::new();
            let root = f.root(M::TYPE_FLOAT);
            let other = if two_truncs {
                let value = f.fd.new_constant(8, 92.25f64.to_bits());
                f.value(
                    f.right,
                    0x1020,
                    OpCode::CPUI_FLOAT_TRUNC,
                    &[value],
                    4,
                    M::TYPE_FLOAT,
                )
                .1
            } else {
                let value = f.fd.new_constant(4, 92);
                f.value(f.right, 0x1020, OpCode::CPUI_COPY, &[value], 4, M::TYPE_INT)
                    .1
            };
            let inputs = if reverse {
                [other, root]
            } else {
                [root, other]
            };
            let (_, phi) = f.value(
                f.join,
                0x1030,
                OpCode::CPUI_MULTIEQUAL,
                &inputs,
                4,
                M::TYPE_INT,
            );
            recover(&mut f.fd).unwrap();
            assert!(f.bridges().is_empty());
            assert_eq!(
                f.fd.vbank().get(root).unwrap().get_type().get_metatype(),
                M::TYPE_INT
            );
            f.fd.with_covermerge(|merge, fd| merge.merge_marker(fd))
                .unwrap();
            let high = f.fd.vbank().get(phi).unwrap().get_high().unwrap();
            assert_eq!(
                MergeContext::high_get_type(&mut f.fd, high).get_metatype(),
                M::TYPE_INT
            );
        }
    }
}

#[test]
fn an_integer_truncation_meeting_a_float_phi_sibling_keeps_its_logical_root() {
    let mut f = Fixture::new();
    let root = f.root(M::TYPE_INT);
    let bits = f.fd.new_constant(4, 1.5f32.to_bits().into());
    let other = f
        .value(
            f.right,
            0x1020,
            OpCode::CPUI_COPY,
            &[bits],
            4,
            M::TYPE_FLOAT,
        )
        .1;
    let (_, phi) = f.value(
        f.join,
        0x1030,
        OpCode::CPUI_MULTIEQUAL,
        &[root, other],
        4,
        M::TYPE_INT,
    );
    recover(&mut f.fd).unwrap();
    assert_eq!(f.bridges().len(), 1);
    f.fd.with_covermerge(|merge, fd| merge.merge_marker(fd))
        .unwrap();
    assert_ne!(
        f.fd.vbank().get(root).unwrap().get_high(),
        f.fd.vbank().get(phi).unwrap().get_high()
    );
}
