//! Tests for `RuleAddCarryChain` (kuna GH-8913): the carry-chain wide-add fusion
//! `PIECE(hi, lo) -> INT_ADD(PIECE(hipart,b), zext(a))` and its gate-off / decline
//! paths.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, BlockId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn no_replace(
) -> impl FnMut(&mut crate::varnode::VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()>
{
    |_, _, _| Ok(())
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    // Create on the dead list (NOT marked alive): the PIECE op is moved to the
    // block by `op_insert` (which marks it alive); the other ops stay dead, which
    // is fine — the rule reads them by id.  Pre-marking-alive here would make
    // `op_insert`'s `mark_alive` double-unlink the op from the dead list.
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, defop: OpId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// A written byte at a ram address (output of a fresh COPY).
fn mk_byte(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(1, &Address::new(r, off), None);
    set_def(fd, vn, defop)
}

/// Build the carry-chain idiom rooted at a 2-byte PIECE, with the INT_CARRY
/// carry form, placing the PIECE op in a basic block.  Returns (piece_op, a, b,
/// hipart).
fn build_idiom(fd: &mut Funcdata) -> (OpId, VarnodeId, VarnodeId, VarnodeId) {
    let bl = mk_block(fd);

    let a = mk_byte(fd, 0x40); // index (variable)
    let b = mk_byte(fd, 0x44); // base low byte
    let hipart = mk_byte(fd, 0x48); // base high byte

    // lo = INT_ADD(a, b)  -> lovn (byte)
    let loadd = mk_op(fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(fd, a, loadd, 0);
    wire(fd, b, loadd, 1);
    let lovn = fd.new_unique(1, None);
    let lovn = set_def(fd, lovn, loadd);

    // carry = INT_CARRY(a, b)  -> carryvn (byte)
    let carryop = mk_op(fd, 0x108, 2, OpCode::CPUI_INT_CARRY);
    wire(fd, a, carryop, 0);
    wire(fd, b, carryop, 1);
    let carryvn = fd.new_unique(1, None);
    let carryvn = set_def(fd, carryvn, carryop);

    // hi = INT_ADD(hipart, carry)  -> hivn (byte)
    let hiadd = mk_op(fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(fd, hipart, hiadd, 0);
    wire(fd, carryvn, hiadd, 1);
    let hivn = fd.new_unique(1, None);
    let hivn = set_def(fd, hivn, hiadd);

    // op = PIECE(hivn, lovn)  -> out (2 bytes), inserted in the block.
    let op = mk_op(fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(fd, hivn, op, 0);
    wire(fd, lovn, op, 1);
    let out = fd.new_unique(2, None);
    let _out = set_def(fd, out, op);
    fd.op_insert(op, bl, None);

    (op, a, b, hipart)
}

#[test]
fn fuses_carry_chain_to_wide_add() {
    for high_level in [false, true] {
        let mut fd = build_fd();
        let (op, a, _b, _hipart) = build_idiom(&mut fd);
        if high_level {
            fd.set_high_level();
        }

        let mut rule = RuleAddCarryChain::new(true);
        assert_eq!(rule.apply_op(op, &mut fd), 1);

        // op is now INT_ADD(base, zext(index)).
        let o = fd.obank().get(op).unwrap();
        assert_eq!(o.code(), OpCode::CPUI_INT_ADD);
        let base_out = o.get_in(0).unwrap();
        let zext_out = o.get_in(1).unwrap();
        for output in [base_out, zext_out] {
            assert_eq!(fd.vbank().get(output).unwrap().get_high().is_some(), high_level);
        }

        // in0 = PIECE(hipart, b)
        let baseop = fd.vbank().get(base_out).unwrap().get_def().unwrap();
        assert_eq!(fd.obank().get(baseop).unwrap().code(), OpCode::CPUI_PIECE);

        // in1 = INT_ZEXT(a)
        let zextop = fd.vbank().get(zext_out).unwrap().get_def().unwrap();
        assert_eq!(fd.obank().get(zextop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
        assert_eq!(fd.obank().get(zextop).unwrap().get_in(0), Some(a));
        // zext output width matches the 2-byte PIECE output.
        assert_eq!(fd.vbank().get(zext_out).unwrap().get_size(), 2);
    }
}

#[test]
fn gate_off_is_noop() {
    let mut fd = build_fd();
    let (op, _a, _b, _hi) = build_idiom(&mut fd);
    let mut rule = RuleAddCarryChain::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_PIECE);
}

#[test]
fn non_carry_high_input_declines() {
    // hi = INT_ADD(hipart, <not a carry of a,b>) -> rule declines.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_byte(&mut fd, 0x40);
    let b = mk_byte(&mut fd, 0x44);
    let hipart = mk_byte(&mut fd, 0x48);
    let other = mk_byte(&mut fd, 0x4c); // not a carry
    let loadd = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, a, loadd, 0);
    wire(&mut fd, b, loadd, 1);
    let lovn = fd.new_unique(1, None);
    let lovn = set_def(&mut fd, lovn, loadd);
    let hiadd = mk_op(&mut fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, hipart, hiadd, 0);
    wire(&mut fd, other, hiadd, 1);
    let hivn = fd.new_unique(1, None);
    let hivn = set_def(&mut fd, hivn, hiadd);
    let op = mk_op(&mut fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(&mut fd, hivn, op, 0);
    wire(&mut fd, lovn, op, 1);
    let out = fd.new_unique(2, None);
    let _ = set_def(&mut fd, out, op);
    fd.op_insert(op, bl, None);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_PIECE);
}

#[test]
fn wide_halves_decline() {
    // 2-byte halves (not 1-byte) -> first-cut guard declines.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let r = ram(&fd);
    // Build hi/lo as 2-byte written varnodes.
    let mk_word = |fd: &mut Funcdata, off: u64| -> VarnodeId {
        let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
        let vn = fd.new_varnode(2, &Address::new(Rc::clone(&r), off), None);
        set_def(fd, vn, defop)
    };
    let hivn = mk_word(&mut fd, 0x40);
    let lovn = mk_word(&mut fd, 0x44);
    let op = mk_op(&mut fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(&mut fd, hivn, op, 0);
    wire(&mut fd, lovn, op, 1);
    let out = fd.new_unique(4, None);
    let _ = set_def(&mut fd, out, op);
    fd.op_insert(op, bl, None);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn specs_one_rule_in_analysis() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_PIECE]);
}

/// Native semantics for the two GH-8913 table-address additions, not a spelling
/// contract or execution of the original 6502 function at low host addresses.
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
mod native_address_consumer {
    use super::*;
    use crate::dtype::{type_metatype, Datatype, TypeFactory};
    use crate::fspec::{parameter_pieces_flags, FuncCallSpecs, ParameterPieces, ProtoModel};
    use std::process::Command;

    fn value(
        fd: &mut Funcdata,
        block: BlockId,
        opcode: OpCode,
        inputs: &[VarnodeId],
        datatype: &Rc<Datatype>,
    ) -> VarnodeId {
        let op = fd.new_op(inputs.len() as int4, Address::new(ram(fd), 0x2000));
        fd.op_set_opcode_code(op, opcode);
        for (slot, &input) in inputs.iter().enumerate() {
            fd.op_set_input(op, input, slot as int4).unwrap();
        }
        let output = fd.new_unique_out(datatype.get_size(), op).unwrap();
        fd.vn_update_type(output, Rc::clone(datatype));
        fd.op_insert_end(op, block);
        output
    }

    // These are real native input providers, defined in the consumer below.
    // Their call results stay explicit: each state read must happen once.
    fn provider(
        fd: &mut Funcdata,
        block: BlockId,
        name: &str,
        entry_offset: u64,
        datatype: &Rc<Datatype>,
    ) -> VarnodeId {
        let op = fd.new_op(1, Address::new(ram(fd), 0x2000));
        fd.op_set_opcode_code(op, OpCode::CPUI_CALL);
        let output = fd.new_unique_out(datatype.get_size(), op).unwrap();
        fd.vn_update_type(output, Rc::clone(datatype));
        let entry = Address::new(ram(fd), entry_offset);
        let mut call = FuncCallSpecs::new(op, entry.clone());
        call.set_funcdata(entry, name).unwrap();
        let void_type = fd.get_arch().types_rc().unwrap().get_type_void().unwrap();
        call.proto_mut().attach_internal_store(void_type);
        call.proto_mut().set_output(&ParameterPieces {
            addr: Address::new(ram(fd), 0x400),
            type_: Some(Rc::clone(datatype)),
            flags: parameter_pieces_flags::TYPELOCK,
        });
        let index = fd.push_call_specs(call);
        let handle = fd.new_varnode_call_specs(index as u64);
        fd.op_set_input(op, handle, 0).unwrap();
        fd.op_insert_end(op, block);
        output
    }

    fn carry_address(
        fd: &mut Funcdata,
        block: BlockId,
        index: VarnodeId,
        low: VarnodeId,
        high: VarnodeId,
        folded: bool,
        enabled: bool,
    ) -> VarnodeId {
        let types = fd.get_arch().types_rc().unwrap();
        let byte = types.get_base(1, type_metatype::TYPE_UINT).unwrap();
        let word = types.get_base(2, type_metatype::TYPE_UINT).unwrap();
        let sum_low = value(fd, block, OpCode::CPUI_INT_ADD, &[index, low], &byte);
        let carry = if folded {
            let low_value = fd.vbank().get(low).unwrap().get_offset();
            let threshold = fd.new_constant(1, low_value.wrapping_neg() & 0xff);
            value(fd, block, OpCode::CPUI_INT_LESSEQUAL, &[threshold, index], &byte)
        } else {
            value(fd, block, OpCode::CPUI_INT_CARRY, &[index, low], &byte)
        };
        let sum_high = value(fd, block, OpCode::CPUI_INT_ADD, &[high, carry], &byte);
        let address = value(fd, block, OpCode::CPUI_PIECE, &[sum_high, sum_low], &word);
        let op = fd.vbank().get(address).unwrap().get_def().unwrap();
        // Use the architecture gate, as the registered rule does. Checking the
        // application count ensures both native programs actually exercise it.
        let mut rule = RuleAddCarryChain::new(false);
        assert_eq!(rule.apply_op(op, fd), if enabled { 1 } else { 0 });
        address
    }

    fn byte_pointer(
        fd: &mut Funcdata,
        block: BlockId,
        memory: VarnodeId,
        address: VarnodeId,
        pointer: &Rc<Datatype>,
    ) -> VarnodeId {
        let scale = fd.new_constant(8, 1);
        value(fd, block, OpCode::CPUI_PTRADD, &[memory, address, scale], pointer)
    }

    fn read_byte(
        fd: &mut Funcdata,
        block: BlockId,
        memory: VarnodeId,
        address: VarnodeId,
        pointer: &Rc<Datatype>,
        byte: &Rc<Datatype>,
    ) -> VarnodeId {
        let location = byte_pointer(fd, block, memory, address, pointer);
        let space = fd.new_varnode_space(&ram(fd));
        value(fd, block, OpCode::CPUI_LOAD, &[space, location], byte)
    }

    fn write_byte(
        fd: &mut Funcdata,
        block: BlockId,
        memory: VarnodeId,
        offset: u64,
        source: VarnodeId,
        pointer: &Rc<Datatype>,
    ) {
        let address = fd.new_constant(2, offset);
        let location = byte_pointer(fd, block, memory, address, pointer);
        let space = fd.new_varnode_space(&ram(fd));
        let op = fd.new_op(3, Address::new(ram(fd), 0x2000));
        fd.op_set_opcode_code(op, OpCode::CPUI_STORE);
        for (slot, input) in [space, location, source].into_iter().enumerate() {
            fd.op_set_input(op, input, slot as int4).unwrap();
        }
        fd.op_insert_end(op, block);
    }

    fn document(enabled: bool, folded_base: Option<u16>) -> String {
        let mut arch = crate::architecture::Architecture::new(
            "native carry consumer",
            kuna_sleigh::sleigh::Sleigh::new(
                Box::new(kuna_sleigh::loadimage::RawLoadImage::new("unused")),
                Box::new(kuna_sleigh::globalcontext::ContextInternal::new()),
            ),
        );
        arch.ctypes = true;
        let mut context = ArchContext::new(build_manager());
        context.add_carry_chain = enabled;
        // IR construction and emission must use the same interned types.
        context.types = Some(arch.types_rc());
        let context = Rc::new(context);
        let entry = Address::new(Rc::clone(context.manage().get_space_by_name("ram").unwrap()), 0x1000);
        let mut fd = Funcdata::new("carry_fixture", "carry_fixture", context, entry.clone(), 0x10000000, 0x40).unwrap();
        let types = fd.get_arch().types_rc().unwrap();
        types.setup_sizes(Some(4), 8, 8);
        types.set_default_alignment_map();
        types.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
        for (name, size) in [("uint1", 1), ("uint2", 2), ("uint4", 4), ("uint8", 8)] {
            types.set_core_type(name, size, type_metatype::TYPE_UINT, false).unwrap();
        }
        types.cache_core_types().unwrap();
        let byte = types.get_base(1, type_metatype::TYPE_UINT).unwrap();
        let word = types.get_base(2, type_metatype::TYPE_UINT).unwrap();
        let trace = types.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let pointer = types.get_type_pointer(8, Rc::clone(&byte), 1).unwrap();
        let mut model = ProtoModel::new(fd.get_arch().manage());
        model.build_param_list("standard").unwrap();
        fd.get_func_proto_mut().set_internal(Rc::new(model), Rc::clone(&trace));
        let block = mk_block(&mut fd);
        fd.set_high_level();
        let memory = provider(&mut fd, block, "fixture_memory", 0x8000, &pointer);
        let screen = provider(&mut fd, block, "fixture_screen", 0x8010, &byte);
        let hovered = provider(&mut fd, block, "fixture_hovered", 0x8020, &byte);
        let (low, high) = if let Some(base) = folded_base {
            (fd.new_constant(1, u64::from(base & 0xff)), fd.new_constant(1, u64::from(base >> 8)))
        } else {
            let base = provider(&mut fd, block, "fixture_base", 0x8030, &word);
            let zero = fd.new_constant(4, 0);
            let one = fd.new_constant(4, 1);
            (
                value(&mut fd, block, OpCode::CPUI_SUBPIECE, &[base, zero], &byte),
                value(&mut fd, block, OpCode::CPUI_SUBPIECE, &[base, one], &byte),
            )
        };
        let two = fd.new_constant(1, 2);
        let screen_offset = value(&mut fd, block, OpCode::CPUI_INT_MULT, &[screen, two], &byte);
        let first = carry_address(&mut fd, block, screen_offset, low, high, folded_base.is_some(), enabled);
        let one = fd.new_constant(2, 1);
        let first_next = value(&mut fd, block, OpCode::CPUI_INT_ADD, &[first, one], &word);
        let table_low = read_byte(&mut fd, block, memory, first, &pointer, &byte);
        let table_high = read_byte(&mut fd, block, memory, first_next, &pointer, &byte);
        let hover_offset = value(&mut fd, block, OpCode::CPUI_INT_MULT, &[hovered, two], &byte);
        let second = carry_address(&mut fd, block, hover_offset, table_low, table_high, false, enabled);
        let second_next = value(&mut fd, block, OpCode::CPUI_INT_ADD, &[second, one], &word);
        let result_low = read_byte(&mut fd, block, memory, second, &pointer, &byte);
        write_byte(&mut fd, block, memory, 0x425, result_low, &pointer);
        // Preserve the original ordering even when the read aliases 0x425:
        // low-byte store precedes the high-byte read and store.
        let result_high = read_byte(&mut fd, block, memory, second_next, &pointer, &byte);
        write_byte(&mut fd, block, memory, 0x426, result_high, &pointer);
        let addresses = value(&mut fd, block, OpCode::CPUI_PIECE, &[first, second], &trace);
        let ret = fd.new_op(2, entry);
        fd.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
        let destination = fd.new_constant(8, 0);
        fd.op_set_input(ret, destination, 0).unwrap();
        fd.op_set_input(ret, addresses, 1).unwrap();
        fd.op_insert_end(ret, block);
        let ops: Vec<_> = fd.obank().iter_alive().collect();
        for (index, op) in ops.into_iter().enumerate() {
            if let Some(output) = fd.obank().get(op).unwrap().get_out() {
                let size = fd.vbank().get(output).unwrap().get_size();
                if size == 1 || size == 2 {
                    // The authored address model is unsigned byte/word,
                    // including the new helper outputs created by the rule.
                    fd.vn_update_type(output, types.get_base(size, type_metatype::TYPE_UINT).unwrap());
                }
                let high = fd.vbank().get(output).unwrap().get_high().unwrap();
                fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name(&format!("v{index}"));
            }
        }
        fd.seed_sblocks_copy();
        format!(
            "{}\n{}",
            crate::decompile_drive::print_c_recompile_prelude(&arch),
            crate::printc::PrintC::new().doc_function_full(&fd, &arch),
        )
    }

    #[test]
    fn generated_c_preserves_page_cross_wrap_and_table_memory_off_on() {
        let compiler = ["clang", "gcc"].into_iter()
            .find(|compiler| Command::new(compiler).arg("--version").output().is_ok())
            .expect("carry-chain native consumer requires a C compiler");
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let directory = std::env::temp_dir().join(format!("kuna-carry-native-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        for folded_base in [None, Some(0xd11a_u16), Some(0xd2c6_u16)] {
            for enabled in [false, true] {
                let generated = document(enabled, folded_base);
                // Keep the generated body/typedefs/CONCAT support unchanged.
                // SUBPIECE and CARRY use their numeric p-code runtime semantics.
                let program = r#"
#include <assert.h>
#include <limits.h>
#include <stdint.h>
#include <string.h>
_Static_assert(CHAR_BIT == 8 && sizeof(unsigned short) == 2 && sizeof(unsigned int) == 4, "native address model");
static unsigned char memory[65536], expected[65536];
static unsigned char screen, hovered;
static unsigned short base;
static unsigned int reads[4];
static unsigned char *fixture_memory(void) { ++reads[0]; return memory; }
static unsigned char fixture_screen(void) { ++reads[1]; return screen; }
static unsigned char fixture_hovered(void) { ++reads[2]; return hovered; }
static unsigned short fixture_base(void) { ++reads[3]; return base; }
static uint8_t SUB21(uint16_t value, unsigned int bytes) {
    assert(bytes < 2);
    return (uint8_t)(value >> (bytes * CHAR_BIT));
}
static uint8_t CARRY1(uint8_t left, uint8_t right) {
    return (unsigned int)left + right > UINT8_MAX;
}
GENERATED
int main(void) {
    static const uint16_t bases[] = { BASES };
    static const uint8_t screens[] = {0,1,28,29,30,114,115,116,127,128,255};
    static const uint8_t hovers[] = {0,1,28,29,127,128,255};
    static const uint16_t targets[] = {0,0xff,0x100,0xfffe,0xffff,0x424,0x425,0x426};
    for (unsigned int i = 0; i < 65536; ++i)
        memory[i] = (unsigned char)((i * 73u) ^ (i >> 8) ^ 0xa5u);
    for (unsigned int b = 0; b < sizeof(bases)/sizeof(bases[0]); ++b)
    for (unsigned int s = 0; s < sizeof(screens)/sizeof(screens[0]); ++s)
    for (unsigned int h = 0; h < sizeof(hovers)/sizeof(hovers[0]); ++h)
    for (unsigned int t = 0; t < sizeof(targets)/sizeof(targets[0]); ++t) {
        base = bases[b]; screen = screens[s]; hovered = hovers[h];
        uint16_t first = (uint16_t)((uint32_t)base + (uint8_t)(2u * screen));
        memory[first] = (uint8_t)targets[t];
        memory[(uint16_t)(first + 1u)] = (uint8_t)(targets[t] >> 8);
        memcpy(expected, memory, sizeof(memory));
        // Independent byte-addressed reference: wide arithmetic and explicit
        // reductions, not the carry-chain operations used to build the IR.
        uint16_t table = (uint16_t)((uint16_t)expected[first] |
            ((uint16_t)expected[(uint16_t)(first + 1u)] << 8));
        uint16_t second = (uint16_t)((uint32_t)table + (uint8_t)(2u * hovered));
        expected[0x425] = expected[second];
        expected[0x426] = expected[(uint16_t)(second + 1u)];
        memset(reads, 0, sizeof(reads));
        uint32_t addresses = carry_fixture();
        assert(addresses == ((uint32_t)first << 16 | second));
        assert(memcmp(memory, expected, sizeof(memory)) == 0);
        assert(reads[0] == 1 && reads[1] == 1 && reads[2] == 1 && reads[3] == BASE_READS);
    }
    return 0;
}
"#.replace("GENERATED", &generated)
                    .replace("BASES", &folded_base.map_or_else(
                        || "0xd11a,0xd2c6,0,0xff,0xff00,0xffff".to_owned(),
                        |base| format!("0x{base:x}"),
                    ))
                    .replace("BASE_READS", if folded_base.is_some() { "0" } else { "1" });
                let source = directory.join("consumer.c");
                std::fs::write(&source, &program).unwrap();
                for optimization in ["0", "2"] {
                    let executable = directory.join(format!("consumer-O{optimization}"));
                    let result = Command::new(compiler)
                        .args(["-std=c11", &format!("-O{optimization}")])
                        .arg(&source).arg("-o").arg(&executable).output().unwrap();
                    assert!(result.status.success(),
                        "carry enabled={enabled} folded_base={folded_base:?} O{optimization}\n{}\n{program}",
                        String::from_utf8_lossy(&result.stderr));
                    assert!(Command::new(&executable).status().unwrap().success(),
                        "carry enabled={enabled} folded_base={folded_base:?} O{optimization}\n{program}");
                }
            }
        }
        std::fs::remove_dir_all(directory).unwrap();
    }
}
