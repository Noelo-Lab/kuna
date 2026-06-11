//! Verifier adversarial tests for item `w2-sleigh-translate` (round 1).
//!
//! Every expectation below was pinned against the C++ oracle: a standalone
//! harness compiled from decompiler/cpp/{translate,space,address,pcoderaw,
//! marshal,opcodes,float,xml,globalcontext}.cc with a minimal concrete
//! `Translate` (registers always-throwing, names empty), reproduced in
//! docs/rust-port/reviews/w2-sleigh-translate.md.  Targets: the big-endian
//! renormalize/overlapJoin paths (the porter's tests cover little-endian
//! only), the `(int4)(offset - unified.offset)` truncation quirk, join
//! allocation-gap and uint4 size-sum wrap behavior, the istream u32
//! saturation in JoinSpace::decodeAttributes, AddrSpace::read suffix
//! parsing, and the >16-input PcodeEmit::decodeOp path.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, ATTRIB_SPACE};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinRecord, JoinSpace,
    RegisterLookup, SpacebaseSpace, VarnodeStorage,
};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{register_translate_ids, PcodeEmit};

/// The oracle harness's DummyTranslate: getRegister always throws a
/// LowlevelError, register names are always empty.
struct NoRegisters;

impl RegisterLookup for NoRegisters {
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        Err(KunaError::sleigh(format!("Unknown register name: {nm}")))
    }
    fn get_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: i32) -> String {
        String::new()
    }
    fn get_exact_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: i32) -> String {
        String::new()
    }
}

/// const(0) / ram(3) / register(4) / join(5), as in the oracle harness.
fn manager(big_end: bool) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big_end,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        big_end,
        4,
        1,
        4,
        0,
        0,
        0,
    )))
    .unwrap();
    m.insert_space(Rc::new(JoinSpace::new(5, big_end))).unwrap();
    m.set_default_code_space(3).unwrap();
    m
}

fn reg(m: &AddrSpaceManager, off: u64, size: u32) -> VarnodeStorage {
    VarnodeStorage {
        space: Some(Rc::clone(m.get_space_by_name("register").unwrap())),
        offset: off,
        size,
    }
}

/// Oracle: BE 3-piece record [(0x10,4),(0x20,4),(0x30,4)] at join+0, size 12.
/// renormalizeJoinAddress(join+3, 6) re-splits into a new record at
/// join+0x10 with pieces [(0x13,1),(0x20,4),(0x30,1)], unified size 6
/// (the pos1 < pos2 branch the porter's little-endian test never enters).
#[test]
fn verify_w2translate_renormalize_big_endian_resplit_oracle() {
    let m = manager(true);
    let join = Rc::clone(m.get_join_space().unwrap());
    let rec = m
        .find_add_join(&[reg(&m, 0x10, 4), reg(&m, 0x20, 4), reg(&m, 0x30, 4)], 0)
        .unwrap();
    assert_eq!(rec.get_unified().offset, 0);
    assert_eq!(rec.get_unified().size, 12);

    let mut addr = Address::new(Rc::clone(&join), 3);
    addr.renormalize(6, &m).unwrap();
    assert!(Rc::ptr_eq(addr.get_space().unwrap(), &join), "stays a join address");
    assert_eq!(addr.get_offset(), 0x10, "12 rounds to 16 for the next allocation");
    let newrec = m.find_join(0x10).unwrap();
    assert_eq!(newrec.get_unified().size, 6);
    assert_eq!(newrec.num_pieces(), 3);
    assert_eq!(
        (newrec.get_piece(0).offset, newrec.get_piece(0).size),
        (0x13, 1),
        "front piece truncated by sizeTrunc1 and rebased to addr1"
    );
    assert_eq!((newrec.get_piece(1).offset, newrec.get_piece(1).size), (0x20, 4));
    assert_eq!(
        (newrec.get_piece(2).offset, newrec.get_piece(2).size),
        (0x30, 1),
        "back piece truncated by sizeTrunc2"
    );
}

/// Oracle: big-endian overlapJoin traverses pieces in forward data order
/// (startPiece=0, dir=+1): for the record above,
///   (reg,0x21) -> 5, (reg,0x31) -> 9, (reg,0x11) with pointSkip=2 -> 3.
#[test]
fn verify_w2translate_overlap_join_big_endian_oracle() {
    let m = manager(true);
    let join = Rc::clone(m.get_join_space().unwrap());
    let register = Rc::clone(m.get_space_by_name("register").unwrap());
    let _rec = m
        .find_add_join(&[reg(&m, 0x10, 4), reg(&m, 0x20, 4), reg(&m, 0x30, 4)], 0)
        .unwrap();
    assert_eq!(join.overlap_join(0, 12, &register, 0x21, 0).unwrap(), 5);
    assert_eq!(join.overlap_join(0, 12, &register, 0x31, 0).unwrap(), 9);
    assert_eq!(join.overlap_join(0, 12, &register, 0x11, 2).unwrap(), 3);
    // size cut-off in data order
    assert_eq!(join.overlap_join(0, 5, &register, 0x31, 0).unwrap(), -1);
}

/// Oracle: the `int4 smallOff = (int4)(offset - unified.offset)` truncating
/// cast maps offsets far past the record back INSIDE it:
///   BE 3-piece record: getEquivalentAddress(0x1_0000_0001) -> (reg,0x11), pos 0
///   LE 2-piece record [(8,4),(0,4)]: -> (reg,0x1), pos 1.
#[test]
fn verify_w2translate_equivalent_address_truncation_quirk_oracle() {
    let m = manager(true);
    let rec = m
        .find_add_join(&[reg(&m, 0x10, 4), reg(&m, 0x20, 4), reg(&m, 0x30, 4)], 0)
        .unwrap();
    let mut pos = -1i32;
    let addr = rec.get_equivalent_address(0x1_0000_0001, &mut pos);
    assert!(!addr.is_invalid(), "C++ does NOT reject the far offset");
    assert_eq!((addr.get_offset(), pos), (0x11, 0));

    let m = manager(false);
    let rec = m.find_add_join(&[reg(&m, 8, 4), reg(&m, 0, 4)], 0).unwrap();
    let addr = rec.get_equivalent_address(0x1_0000_0001, &mut pos);
    assert!(!addr.is_invalid());
    assert_eq!((addr.get_offset(), pos), (0x1, 1));
}

/// Oracle: LE 2-piece record occupies join [0,8); allocation rounds to 16.
///   renormalize(join+8, 1)  -> "Join address not covered by a JoinRecord"
///   renormalize(join+4, 8)  -> "Join address range not covered"
///   findAddJoin sizes 0x80000000 + 0x80000000 (uint4 sum wraps to 0)
///                           -> "Cannot create a zero size join"
#[test]
fn verify_w2translate_join_gap_pastend_and_sum_wrap_oracle() {
    let m = manager(false);
    let join = Rc::clone(m.get_join_space().unwrap());
    let _rec = m.find_add_join(&[reg(&m, 8, 4), reg(&m, 0, 4)], 0).unwrap();

    let mut addr = Address::new(Rc::clone(&join), 8);
    assert_eq!(
        addr.renormalize(1, &m).unwrap_err().explain(),
        "Join address not covered by a JoinRecord"
    );
    let mut addr = Address::new(Rc::clone(&join), 4);
    assert_eq!(
        addr.renormalize(8, &m).unwrap_err().explain(),
        "Join address range not covered"
    );
    assert_eq!(
        m.find_add_join(&[reg(&m, 0, 0x8000_0000), reg(&m, 0x100, 0x8000_0000)], 0)
            .unwrap_err()
            .explain(),
        "Cannot create a zero size join"
    );
}

/// Oracle: mergeSequence merges contiguous IPTR_SPACEBASE pieces WITHOUT the
/// formal-register-name check (stack (4,4)+(0,4) -> single (0,8) even though
/// no register names exist), while contiguous register pieces with no exact
/// name are an informal merge and the original sequence is kept.  Also:
/// constructJoinAddress for contiguous registers with NO parent name falls
/// through to a formal 2-piece JoinRecord.
#[test]
fn verify_w2translate_merge_sequence_spacebase_and_noname_join_oracle() {
    let mut m = manager(false);
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let stack = Rc::new(SpacebaseSpace::new("stack", 6, 8, &ram, 1, true, false));
    m.insert_space(Rc::clone(&stack)).unwrap();

    let svn = |off: u64, size: u32| VarnodeStorage {
        space: Some(Rc::clone(&stack)),
        offset: off,
        size,
    };
    let mut seq = vec![svn(4, 4), svn(0, 4)];
    JoinRecord::merge_sequence(&mut seq, &NoRegisters);
    assert_eq!(seq.len(), 1, "spacebase pieces merge without a register name");
    assert_eq!((seq[0].offset, seq[0].size), (0, 8));

    let mut seq = vec![reg(&m, 6, 2), reg(&m, 4, 2)];
    let orig = seq.clone();
    JoinRecord::merge_sequence(&mut seq, &NoRegisters);
    assert_eq!(seq, orig, "informal register merge keeps the original sequence");

    // contiguous registers, no parent name -> formal JoinRecord (offset 0
    // since this manager has no prior records)
    let register = Rc::clone(m.get_space_by_name("register").unwrap());
    let hi = Address::new(Rc::clone(&register), 4);
    let lo = Address::new(Rc::clone(&register), 0);
    let addr = m.construct_join_address(&NoRegisters, &hi, 4, &lo, 4).unwrap();
    assert!(Rc::ptr_eq(addr.get_space().unwrap(), m.get_join_space().unwrap()));
    let rec = m.find_join(addr.get_offset()).unwrap();
    assert_eq!(rec.num_pieces(), 2);
    assert_eq!((rec.get_piece(0).offset, rec.get_piece(1).offset), (4, 0));
}

/// Oracle (istringstream num_get parity): a piece size of "0x100000000" or
/// "-1" extracted into a uint4 stores UINT_MAX (4294967295).  The Rust
/// decodeAttributes saturation must agree.
#[test]
fn verify_w2translate_decode_attributes_u32_saturation_istream_parity() {
    let m = manager(false);
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);

    for size_text in ["0x100000000", "-1"] {
        let xml = format!(
            "<spaceid space=\"join\" piece1=\"register:0x10:{size_text}\" logicalsize=\"4\"/>"
        );
        let mut dec = XmlDecode::new(&m, &registry);
        dec.ingest_stream(xml.as_bytes()).unwrap();
        let el = dec.open_element().unwrap();
        let spc = loop {
            let attrib_id = dec.get_next_attribute_id().unwrap();
            if attrib_id == ATTRIB_SPACE.get_id() {
                break dec.read_space().unwrap();
            }
        };
        dec.rewind_attributes();
        let mut size: u32 = 0;
        let offset = spc.decode_attributes(&mut dec, &mut size).unwrap();
        dec.close_element(el).unwrap();
        assert_eq!(size, 4, "logicalsize wins for the single-piece form");
        let rec = m.find_join(offset).unwrap();
        assert_eq!(
            rec.get_piece(0).size,
            u32::MAX,
            "{size_text} must saturate like istream u32 extraction"
        );
    }
}

/// Oracle: AddrSpace::read suffix parsing on a plain (no-register) space:
///   "0x10:2+1" -> (0x11, 2)   (':' size then '+' offset)
///   "0x10+3:5" -> (0x13, 8)   ('+' first: the size override is IGNORED,
///                              size stays the default space size)
///   ":2"       -> (0x0, 2)
#[test]
fn verify_w2translate_read_offset_suffix_oracle() {
    let m = manager(false);
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let mut size = 0i32;
    let off = ram.read("0x10:2+1", &mut size, &m).unwrap();
    assert_eq!((off, size), (0x11, 2));
    let off = ram.read("0x10+3:5", &mut size, &m).unwrap();
    assert_eq!((off, size), (0x13, 8));
    let off = ram.read(":2", &mut size, &m).unwrap();
    assert_eq!((off, size), (0x0, 2));
}

/// The C++ decodeOp switches from a 16-entry stack array to a heap vector
/// for isize > 16; the Rust port collapsed the two paths into one buffer.
/// Pin that a 17-input op decodes identically (count, order, values).
#[test]
fn verify_w2translate_decode_op_seventeen_inputs() {
    struct Collect(Vec<(OpCode, Option<u64>, Vec<u64>)>);
    impl PcodeEmit for Collect {
        fn dump(
            &mut self,
            _addr: &Address,
            opc: OpCode,
            outvar: Option<&VarnodeData>,
            vars: &[VarnodeData],
        ) {
            self.0.push((
                opc,
                outvar.map(|v| v.offset),
                vars.iter().map(|v| v.offset).collect(),
            ));
        }
    }

    let m = manager(false);
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

    let mut xml = String::from("<op code=\"CALLOTHER\" size=\"17\">");
    xml.push_str("<addr space=\"register\" offset=\"0x0\" size=\"4\"/>"); // output
    for i in 0..17u64 {
        xml.push_str(&format!(
            "<addr space=\"ram\" offset=\"0x{:x}\" size=\"8\"/>",
            0x1000 + 8 * i
        ));
    }
    xml.push_str("</op>");

    let mut dec = XmlDecode::new(&m, &registry);
    dec.ingest_stream(xml.as_bytes()).unwrap();
    let mut emit = Collect(Vec::new());
    let addr = Address::new(Rc::clone(&ram), 0x400000);
    emit.decode_op(&addr, &mut dec).unwrap();
    assert_eq!(emit.0.len(), 1);
    let (opc, out, ins) = &emit.0[0];
    assert_eq!(*opc, OpCode::CPUI_CALLOTHER);
    assert_eq!(*out, Some(0));
    assert_eq!(ins.len(), 17);
    let expect: Vec<u64> = (0..17u64).map(|i| 0x1000 + 8 * i).collect();
    assert_eq!(*ins, expect);
}

/// The shared-manager BTreeMap surface of Translate::get_all_registers must
/// iterate in C++ map<VarnodeData,string> order (space index, offset, BIG
/// sizes first) — pin the size tie-break the porter's test never exercises.
#[test]
// mutable_key_type: VarnodeData's Ord reads only the immutable space index,
// offset and size (see the Translate::get_all_registers note in translate.rs)
#[allow(clippy::mutable_key_type)]
fn verify_w2translate_register_map_order_big_sizes_first() {
    let m = manager(false);
    let mut map: BTreeMap<VarnodeData, String> = BTreeMap::new();
    let vd = |off: u64, size: u32| VarnodeData {
        space: Some(Rc::clone(m.get_space_by_name("register").unwrap())),
        offset: off,
        size,
    };
    map.insert(vd(0, 4), "EAX".into());
    map.insert(vd(0, 8), "EDX_EAX".into());
    map.insert(vd(0, 2), "AX".into());
    map.insert(vd(8, 4), "EDX".into());
    let names: Vec<&str> = map.values().map(String::as_str).collect();
    assert_eq!(names, ["EDX_EAX", "EAX", "AX", "EDX"]);
}
