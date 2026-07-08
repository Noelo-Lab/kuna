//! Independent adversarial verification tests for `w8-sig-pack`
//! (port of `decompiler/cpp/signature.{cc,hh}`, `analyzesigs.{cc,hh}`,
//! `paramid.{cc,hh}`).
//!
//! Written by the VERIFIER, not the porter.  They re-derive expected behavior
//! from the C++ algorithm and target the hunt-list fragile spots for this item:
//!
//!   - **Wrapping / high-bit hashing.** `SignatureEntry::localHash` mixes the op
//!     hash with `localhash ^= ophash ^ (ophash<<9) ^ (ophash<<18)`
//!     (signature.cc:311).  For a `CPUI_CPOOLREF`-defined Varnode `getOpHash`
//!     folds the *full 64-bit* `getOffset()` of the last input into `ophash`
//!     (`(ophash + 0xfeedface) ^ off`, signature.cc:114), so `ophash` carries
//!     arbitrary high bits.  C++ unsigned `<<` drops the bits off the top (mod
//!     2^64); Rust's `<<` does the same for an in-range shift amount (18 < 64) —
//!     it does NOT panic on lost high bits — so the plain `<<` is faithful.
//!     `cpoolref_high_offset_must_not_panic` confirms the full pipeline runs and
//!     stays deterministic on a maximal tag offset.
//!   - **Determinism / iteration order.** `generate` must be reproducible and its
//!     `simple_signature` `<sig>` list sorted ascending (the C++
//!     `getSignatureVector` `sort`).
//!   - **paramid walk.** an input read directly ranks `DIRECTREAD`; the `walkforward`
//!     terminal-rank short-circuit (paramid.cc:46) must stop at the best rank.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::XmlEncode;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::paramid::{param_rank, ParamIDIO, ParamMeasure};
use kuna_decomp::context::{ArchContext, BlockId, OpId, TypeOp};
use kuna_decomp::signature::{set_settings, sig_mods, simple_signature, GraphSigManager};

// The signature settings live in a process-global atomic (the faithful C++
// `SigManager::settings` static).  Serialize the settings-touching tests in this
// (separate) test binary so they do not race the cargo parallel runner.
static SETTINGS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
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

fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn typeop(opc: OpCode) -> TypeOp {
    let flags = match opc {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_AND => {
            pcodeop_flags::commutative
        }
        _ => 0,
    };
    TypeOp::new(opc, flags, format!("{opc:?}"))
}

fn add_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, ninput: i32, pc: Address) -> OpId {
    let op = fd.new_op(ninput, pc);
    fd.op_set_opcode(op, typeop(opc));
    fd.op_insert(op, bl, None);
    op
}

/// High-bit hashing: a Varnode defined by `CPUI_CPOOLREF` whose last input is a
/// constant with set high bits drives `ophash` (signature.cc:114) to a full
/// 64-bit value, so `ophash << 18` (signature.cc:311 -> signature.rs:879) loses
/// bits off the top.  C++ unsigned `<<` and Rust `<<` (in-range amount) agree:
/// the high bits drop, no panic.  Pass == the full pipeline runs deterministically.
#[test]
fn cpoolref_high_offset_must_not_panic() {
    let _guard = SETTINGS_LOCK.lock().unwrap();
    set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);

    let mut fd = build_fd();
    let rs = ramspace(&fd);
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);

    // out = CPOOLREF(ref, tag) where tag's offset has high bits set.
    // CPOOLREF hashes `op->getIn(numInput-1)->getOffset()` -> the tag constant.
    let cpool = add_op(&mut fd, bl, OpCode::CPUI_CPOOLREF, 2, Address::new(Rc::clone(&rs), 0x1000));
    let refvn = fd.new_constant(8, 0x10);
    // 0xFFFF_FFFF_FFFF_FFFF: every top bit set -> ophash<<18 overflows u64.
    let tag = fd.new_constant(8, 0xFFFF_FFFF_FFFF_FFFF);
    fd.op_set_input(cpool, refvn, 0).unwrap();
    fd.op_set_input(cpool, tag, 1).unwrap();
    let _out =
        fd.new_varnode_out(8, &Address::new(Rc::clone(&rs), 0x200), cpool).unwrap();
    fd.structure_reset();

    let mut mgr = GraphSigManager::new().unwrap();
    // This calls local_hash on the CPOOLREF output (the high-bit `<<` mixing).
    mgr.set_current_function(&fd).unwrap();
    mgr.generate(&fd);
    let v1 = mgr.get_signature_vector();

    // Re-run: the high-offset hash must be reproducible (no UB-dependent value).
    let mut mgr2 = GraphSigManager::new().unwrap();
    mgr2.set_current_function(&fd).unwrap();
    mgr2.generate(&fd);
    assert_eq!(v1, mgr2.get_signature_vector(), "CPOOLREF high-offset hash must be deterministic");
}

/// `simple_signature` emits one `<sig val=..>` per feature, and the underlying
/// feature vector is sorted ascending (C++ `getSignatureVector` `sort`), so the
/// output is order-deterministic regardless of `sigmap`/`blockmap` insertion.
#[test]
fn simple_signature_feature_vector_is_sorted_and_deterministic() {
    let _guard = SETTINGS_LOCK.lock().unwrap();
    set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);

    let mut fd = build_fd();
    let rs = ramspace(&fd);
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);

    let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
    let p = fd.set_input_varnode(p).unwrap();
    let c = fd.new_constant(4, 5);
    let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
    let out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
    fd.op_set_input(add, p, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let ret = add_op(&mut fd, bl, OpCode::CPUI_RETURN, 2, Address::new(Rc::clone(&rs), 0x1004));
    let iop = fd.new_constant(8, 0);
    fd.op_set_input(ret, iop, 0).unwrap();
    fd.op_set_input(ret, out, 1).unwrap();
    fd.structure_reset();

    let mut m = GraphSigManager::new().unwrap();
    m.set_current_function(&fd).unwrap();
    m.generate(&fd);
    let v = m.get_signature_vector();
    assert!(!v.is_empty());
    assert!(v.windows(2).all(|w| w[0] <= w[1]), "feature vector must be sorted: {v:?}");

    // simple_signature emits exactly one <sig> per (sorted) feature.
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        simple_signature(&fd, &mut enc).unwrap();
    }
    let xml = String::from_utf8(buf).unwrap();
    let nsig = xml.matches("<sig ").count();
    assert_eq!(nsig, v.len(), "one <sig> per feature; got xml: {xml}");
}

/// paramid: an input Varnode read directly by an ordinary op ranks DIRECTREAD
/// (the input terminal rank), and the `best` walk stops there.  Re-derived from
/// `walkforward` default arm (paramid.cc:81) + terminalrank (paramid.cc:149).
#[test]
fn paramid_direct_read_hits_terminal_rank() {
    let mut fd = build_fd();
    let rs = ramspace(&fd);
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);

    let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
    let p = fd.set_input_varnode(p).unwrap();
    let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
    let _out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
    fd.op_set_input(add, p, 0).unwrap();
    fd.op_set_input(add, p, 1).unwrap();
    fd.structure_reset();

    let paddr = Address::new(Rc::clone(&rs), 0x100);
    let mut pm = ParamMeasure::new(&paddr, 4, None, ParamIDIO::Input);
    pm.calculate_rank(&fd, true, p, None);
    assert_eq!(pm.get_measure(), param_rank::DIRECTREAD);

    // The worst (max) walk for the same input: INT_ADD default arm still gives
    // DIRECTREAD, and the not-best accumulation maxes toward it but the worst
    // terminal rank is INDIRECT, never reached here -> DIRECTREAD.
    let mut pmw = ParamMeasure::new(&paddr, 4, None, ParamIDIO::Input);
    pmw.calculate_rank(&fd, false, p, None);
    assert_eq!(pmw.get_measure(), param_rank::DIRECTREAD);
}
