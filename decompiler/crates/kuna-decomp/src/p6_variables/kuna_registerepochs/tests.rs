use super::single_family;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p6_variables::funcdata_spacebase::spacebase_tests::{build_fd, reg_addr};
use kuna_num::opcodes::OpCode;

struct Fixture {
    fd: Funcdata,
    block: BlockId,
    pc: u64,
}

impl Fixture {
    fn new() -> Self {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(root);
        Self {
            fd,
            block,
            pc: 0x1000,
        }
    }

    fn empty_op(&mut self, code: OpCode, count: i32, size: i32) -> (OpId, VarnodeId) {
        let op = self.fd.new_op(count, reg_addr(&self.fd, self.pc));
        self.pc += 1;
        self.fd.op_set_opcode_code(op, code);
        let output = self.fd.new_unique_out(size, op).unwrap();
        self.fd.bb_insert_op(op, self.block, None);
        (op, output)
    }

    fn op(&mut self, code: OpCode, inputs: &[VarnodeId], size: i32) -> VarnodeId {
        let (op, output) = self.empty_op(code, inputs.len() as i32, size);
        self.fd.op_set_all_input(op, inputs).unwrap();
        output
    }

    fn seed(&mut self, value: u64) -> VarnodeId {
        let constant = self.fd.new_constant(8, value);
        self.op(OpCode::CPUI_COPY, &[constant], 8)
    }
}

#[test]
fn ordinary_same_width_marker_edges_form_one_family() {
    let mut f = Fixture::new();
    let seed = f.seed(7);
    let copy = f.op(OpCode::CPUI_COPY, &[seed], 8);
    let cast = f.op(OpCode::CPUI_CAST, &[copy], 8);
    let annotation = f.fd.new_constant(8, 0);
    let indirect = f.op(OpCode::CPUI_INDIRECT, &[cast, annotation], 8);
    assert!(single_family(&f.fd, &[seed, copy, cast, indirect]));
    assert!(single_family(&f.fd, &[indirect, seed]));

    let narrow = f.op(OpCode::CPUI_CAST, &[seed], 4);
    let constant = f.fd.new_constant(8, 1);
    let computed = f.op(OpCode::CPUI_INT_ADD, &[seed, constant], 8);
    assert!(!single_family(&f.fd, &[seed, narrow]));
    assert!(!single_family(&f.fd, &[seed, computed]));
}

#[test]
fn a_grounded_phi_cycle_remains_connected() {
    let mut f = Fixture::new();
    let seed = f.seed(7);
    let (phi, joined) = f.empty_op(OpCode::CPUI_MULTIEQUAL, 2, 8);
    let carried = f.op(OpCode::CPUI_COPY, &[joined], 8);
    f.fd.op_set_input(phi, seed, 0).unwrap();
    f.fd.op_set_input(phi, carried, 1).unwrap();
    assert!(single_family(&f.fd, &[joined, seed, carried]));
    assert!(single_family(&f.fd, &[carried, seed]));
}

#[test]
fn small_repeated_phi_inputs_remain_connected() {
    let mut f = Fixture::new();
    let first = f.seed(7);
    let second = f.seed(9);
    let inputs: Vec<_> = std::iter::repeat(second)
        .take(16)
        .chain(std::iter::repeat(first).take(16))
        .collect();
    let joined = f.op(OpCode::CPUI_MULTIEQUAL, &inputs, 8);
    assert!(single_family(&f.fd, &[first, second, joined]));
    assert!(single_family(&f.fd, &[joined, first, second]));
}

#[test]
fn repeated_phi_readers_cannot_hide_quadratic_operand_scans() {
    let mut f = Fixture::new();
    let first = f.seed(7);
    let second = f.seed(9);
    let inputs: Vec<_> = std::iter::repeat(second)
        .take(256)
        .chain(std::iter::repeat(first).take(256))
        .collect();
    f.op(OpCode::CPUI_MULTIEQUAL, &inputs, 8);
    assert!(!single_family(&f.fd, &[first, second]));
}

#[test]
fn oversized_phi_expansion_declines_before_growing_the_worklist() {
    let mut f = Fixture::new();
    let seed = f.seed(7);
    let joined = f.op(OpCode::CPUI_MULTIEQUAL, &vec![seed; 8193], 8);
    assert!(!single_family(&f.fd, &[joined, seed]));
}

#[test]
fn oversized_value_lists_decline_before_collection() {
    let mut f = Fixture::new();
    let seed = f.seed(7);
    assert!(!single_family(&f.fd, &vec![seed; 8193]));
    assert!(single_family(&f.fd, &[seed, seed]));
}
