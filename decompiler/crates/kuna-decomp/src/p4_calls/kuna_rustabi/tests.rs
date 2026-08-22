//! Tests for the Rust two-register return classification.
//!
//! These pin the classifier on hand-built shapes: what counts as a
//! discriminant, that the concatenation is still found after the rule pool
//! reshapes it, and that the gate fails closed. The end-to-end witness -- a
//! rustc `Result<u32,u32>` producer and the `match` that consumes it -- lives in
//! `tests/stages/kuna-rustabi.xml` and
//! `kuna-console/tests/verify_rustabi_pair.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::context::{ArchContext, TypeOp};

/// A fixture with a `join` space, so the return-recovery concatenation storage
/// can be built, and `rustabi` forced on (`Always`, since a hand-built fixture
/// has no loader verdict to detect).
fn build_fd(mode: RustAbiMode) -> Funcdata {
    build_fd_lang(mode, true)
}

/// As [`build_fd`], with the loader's source-language verdict spelled out.
fn build_fd_lang(mode: RustAbiMode, is_rust: bool) -> Funcdata {
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
    m.insert_space(Rc::new(JoinSpace::new(3, false))).unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.rust_abi = mode.as_u8();
    ctx.source_is_rust = is_rust;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

/// An unwritten Varnode at a ram address.
fn unwritten(fd: &mut Funcdata, off: u64, sz: int4) -> VarnodeId {
    let r = space(fd, "ram");
    fd.new_varnode(sz, &Address::new(r, off), None)
}

/// Build `out = <opc>(inputs...)`, placing `out` in `out_space` at `out_off`.
fn mk_def(
    fd: &mut Funcdata,
    opc: OpCode,
    inputs: &[VarnodeId],
    out_space: &str,
    out_off: u64,
    out_size: int4,
) -> VarnodeId {
    let r = space(fd, "ram");
    let o = space(fd, out_space);
    let op = fd.new_op(inputs.len() as int4, Address::new(r, 0x2000 + out_off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        let _ = fd.op_set_input(op, vn, i as int4);
    }
    fd.new_varnode_out(out_size, &Address::new(o, out_off), op).expect("varnode out")
}

// -- the option parse ------------------------------------------------------

#[test]
fn the_three_modes_parse_and_round_trip() {
    for (tok, want) in [
        ("off", RustAbiMode::Off),
        ("auto", RustAbiMode::Auto),
        ("always", RustAbiMode::Always),
    ] {
        let (mode, msg) = parse_rust_abi_mode(tok).expect("token parses");
        assert_eq!(mode, want, "`{tok}` parses to {want:?}");
        assert!(msg.contains(tok), "the confirmation names the mode: {msg}");
        assert_eq!(mode.as_str(), tok);
        assert_eq!(RustAbiMode::from_u8(mode.as_u8()), mode, "wire encoding round-trips");
    }
    assert!(parse_rust_abi_mode("pair").is_err(), "an unknown token is a parse error");
}

/// `on` is accepted as a spelling of `auto` so the habitual `option X on` works.
#[test]
fn on_is_a_spelling_of_auto() {
    assert_eq!(parse_rust_abi_mode("on").expect("parses").0, RustAbiMode::Auto);
}

// -- the discriminant shape ------------------------------------------------

#[test]
fn a_small_constant_is_discriminant_shaped() {
    let mut fd = build_fd(RustAbiMode::Always);
    let k = fd.new_constant(8, 1);
    assert!(is_discriminant_shaped(&fd, k), "a variant tag is a small constant");
}

#[test]
fn a_wide_constant_is_not_discriminant_shaped() {
    let mut fd = build_fd(RustAbiMode::Always);
    let k = fd.new_constant(8, 0x1234_5678);
    assert!(
        !is_discriminant_shaped(&fd, k),
        "a value too wide to be a tag is payload, not a discriminant",
    );
}

/// The branchless rustc shape: `xor %eax,%eax; setb %al` leaves a Varnode whose
/// known non-zero bits are a single bit, with no constant anywhere.
#[test]
fn a_one_bit_nonzero_mask_is_discriminant_shaped() {
    let mut fd = build_fd(RustAbiMode::Always);
    let a = unwritten(&mut fd, 0x3000, 8);
    let b = unwritten(&mut fd, 0x3100, 8);
    let flag = mk_def(&mut fd, OpCode::CPUI_INT_LESS, &[a, b], "ram", 0x10, 1);
    let wide = mk_def(&mut fd, OpCode::CPUI_INT_ZEXT, &[flag], "ram", 0x20, 8);
    fd.vbank_mut().get_mut(wide).expect("wide").set_nz_mask(1);
    assert!(
        is_discriminant_shaped(&fd, wide),
        "a zero-extended flag is exactly how rustc materializes a two-variant tag",
    );
}

#[test]
fn a_wide_nonzero_mask_is_not_discriminant_shaped() {
    let mut fd = build_fd(RustAbiMode::Always);
    let a = unwritten(&mut fd, 0x3000, 8);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x30, 8);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    assert!(!is_discriminant_shaped(&fd, payload), "a 32-bit value is a payload");
}

// -- the pair classification ----------------------------------------------

#[test]
fn a_small_low_half_classifies_as_a_scalar_pair() {
    let mut fd = build_fd(RustAbiMode::Always);
    let tag = fd.new_constant(8, 1);
    let a = unwritten(&mut fd, 0x3000, 8);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x40, 8);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    assert_eq!(classify_return_pair(&fd, payload, tag), ReturnRepr::ScalarPair);
}

/// A narrow low half is not enough: the payload register has to hold something
/// the function put there. An untouched register is exactly the leftover shape
/// the uncomputed-half repair exists to drop.
#[test]
fn a_leftover_payload_half_is_not_a_scalar_pair() {
    let mut fd = build_fd(RustAbiMode::Always);
    let tag = fd.new_constant(8, 1);
    let untouched = unwritten(&mut fd, 0x3200, 8);
    assert_eq!(
        classify_return_pair(&fd, untouched, tag),
        ReturnRepr::Scalar,
        "a register the function never wrote is not a payload",
    );
}

#[test]
fn two_wide_halves_are_not_a_scalar_pair() {
    let mut fd = build_fd(RustAbiMode::Always);
    let a = unwritten(&mut fd, 0x3000, 8);
    let lo = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x50, 8);
    let hi = mk_def(&mut fd, OpCode::CPUI_INT_MULT, &[a, a], "ram", 0x60, 8);
    for v in [lo, hi] {
        fd.vbank_mut().get_mut(v).expect("half").set_nz_mask(0xffff_ffff);
    }
    assert_eq!(
        classify_return_pair(&fd, hi, lo),
        ReturnRepr::Scalar,
        "a genuine two-word struct return keeps today's answer",
    );
}

// -- finding the concatenation --------------------------------------------

/// The shape `buildReturnOutput` leaves behind.
#[test]
fn a_bare_piece_in_join_space_holds_the_pair() {
    let mut fd = build_fd(RustAbiMode::Always);
    let tag = fd.new_constant(8, 1);
    let a = unwritten(&mut fd, 0x3000, 8);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x70, 8);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    let whole = mk_def(&mut fd, OpCode::CPUI_PIECE, &[payload, tag], "join", 0x0, 16);
    assert!(holds_scalar_pair(&fd, whole));
}

/// `RuleConcatZext` rewrites `PIECE(ZEXT(V), W)` as `ZEXT(PIECE(V, W))` the
/// moment the payload register is written 32-bit, which is the common rustc
/// case. The pair is still there, one reshaping deep.
#[test]
fn the_pair_is_found_through_the_pools_zero_extension() {
    let mut fd = build_fd(RustAbiMode::Always);
    let tag = fd.new_constant(8, 1);
    let a = unwritten(&mut fd, 0x3000, 4);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x80, 4);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    let narrow = mk_def(&mut fd, OpCode::CPUI_PIECE, &[payload, tag], "unique", 0x100, 12);
    let whole = mk_def(&mut fd, OpCode::CPUI_INT_ZEXT, &[narrow], "join", 0x0, 16);
    assert!(
        holds_scalar_pair(&fd, whole),
        "matching only a bare PIECE would miss the overwhelmingly common shape",
    );
}

/// Storage is half the signature: a concatenation that is not the recovered
/// return storage is somebody else's op.
#[test]
fn a_concatenation_outside_join_space_is_not_the_recovered_return() {
    let mut fd = build_fd(RustAbiMode::Always);
    let tag = fd.new_constant(8, 1);
    let a = unwritten(&mut fd, 0x3000, 8);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0x90, 8);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    let whole = mk_def(&mut fd, OpCode::CPUI_PIECE, &[payload, tag], "unique", 0x200, 16);
    assert!(!holds_scalar_pair(&fd, whole));
}

// -- the gate --------------------------------------------------------------

#[test]
fn the_gate_fails_closed_when_the_option_is_off() {
    let mut fd = build_fd(RustAbiMode::Off);
    let tag = fd.new_constant(8, 1);
    let a = unwritten(&mut fd, 0x3000, 8);
    let payload = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, a], "ram", 0xa0, 8);
    fd.vbank_mut().get_mut(payload).expect("payload").set_nz_mask(0xffff_ffff);
    let whole = mk_def(&mut fd, OpCode::CPUI_PIECE, &[payload, tag], "join", 0x0, 16);
    assert!(!holds_scalar_pair(&fd, whole), "`off` must be byte-identical to the pre-fix engine");
    assert!(!live(&fd));
}

/// `auto` asks the loader's verdict; a non-Rust image can never reach the rules.
#[test]
fn auto_is_inert_when_the_image_is_not_rust() {
    assert!(live(&build_fd_lang(RustAbiMode::Auto, true)), "a rustc image reaches the rules");
    assert!(
        !live(&build_fd_lang(RustAbiMode::Auto, false)),
        "`auto` on a non-Rust image is exactly `off`",
    );
    assert!(
        live(&build_fd_lang(RustAbiMode::Always, false)),
        "`always` drops the language test",
    );
}

/// A model-less fixture cannot claim any storage is parameter storage, so the
/// sret veto is inert -- the fail-closed direction.
#[test]
fn without_a_prototype_model_nothing_is_an_incoming_pointer() {
    let mut fd = build_fd(RustAbiMode::Always);
    let vn = unwritten(&mut fd, 0x3000, 8);
    assert!(!traces_to_incoming_pointer(&fd, vn, 0));
}

// =========================================================================
// The consumer seam
// =========================================================================

/// A fixture whose manager carries a register lookup, so
/// `construct_join_address` can build the pair storage the CALL output needs.
fn build_call_fd(mode: RustAbiMode) -> Funcdata {
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
    m.insert_space(Rc::new(JoinSpace::new(3, false))).unwrap();
    m.set_register_lookup(Rc::new(NamelessRegisters));
    let mut ctx = ArchContext::new(m);
    ctx.rust_abi = mode.as_u8();
    ctx.source_is_rust = true;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, addr, 0x1000_0000, 0x40).unwrap()
}

/// A [`kuna_base::space::RegisterLookup`] that knows no register names, which is
/// what forces `construct_join_address` down the formal-JoinRecord path (no
/// parent register covers the two halves).
struct NamelessRegisters;

impl kuna_base::space::RegisterLookup for NamelessRegisters {
    fn get_register(&self, _nm: &str) -> kuna_base::error::KunaResult<kuna_base::space::VarnodeStorage> {
        Err(kuna_base::error::KunaError::lowlevel("no registers"))
    }
    fn get_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
        String::new()
    }
    fn get_exact_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
        String::new()
    }
}

/// A CALL in a basic block, plus the two register INDIRECT creations the
/// output trials would be — `lo` at `ram:0x00` and `hi` at `ram:0x40`, each read
/// once so the payload half has a descendant. Returns `(callop, lo, hi)`.
fn build_call_with_pair(fd: &mut Funcdata, hi_read: bool) -> (OpId, VarnodeId, VarnodeId) {
    let ram = space(fd, "ram");
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);

    let call = fd.new_op(1, Address::new(Rc::clone(&ram), 0x1010));
    fd.obank_mut()
        .change_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CPUI_CALL".to_string()));
    let target = fd.new_code_ref(&Address::new(Rc::clone(&ram), 0x2000));
    let _ = fd.op_set_input(call, target, 0);

    let mut mk_half = |fd: &mut Funcdata, off: u64| -> VarnodeId {
        let ind = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1010));
        fd.obank_mut()
            .change_opcode(ind, TypeOp::new(OpCode::CPUI_INDIRECT, 0, "CPUI_INDIRECT".to_string()));
        let c0 = fd.new_constant(8, 0);
        let c1 = fd.new_constant(8, 0);
        let _ = fd.op_set_input(ind, c0, 0);
        let _ = fd.op_set_input(ind, c1, 1);
        let out = fd
            .new_varnode_out(8, &Address::new(Rc::clone(&ram), off), ind)
            .expect("half varnode");
        fd.op_insert(ind, bl, None);
        fd.mark_indirect_creation(ind, false).expect("indirect creation");
        out
    };
    let lo = mk_half(fd, 0x00);
    let hi = mk_half(fd, 0x40);
    fd.op_insert(call, bl, None);

    // A read of each half after the call, so both are live values.
    for (half, off) in [(Some(lo), 0x100u64), (if hi_read { Some(hi) } else { None }, 0x108)] {
        let Some(h) = half else { continue };
        let use_op = fd.new_op(1, Address::new(Rc::clone(&ram), 0x1020 + off));
        fd.obank_mut()
            .change_opcode(use_op, TypeOp::new(OpCode::CPUI_COPY, 0, "CPUI_COPY".to_string()));
        let _ = fd.op_set_input(use_op, h, 0);
        let _ = fd.new_varnode_out(8, &Address::new(Rc::clone(&ram), 0x200 + off), use_op);
        fd.op_insert(use_op, bl, None);
    }
    (call, lo, hi)
}

/// A summary that proves the callee wrote nothing at all.
fn proves_nothing_written() -> CalleeReturnWrites {
    CalleeReturnWrites { writes: Vec::new(), store_spaces: Vec::new(), complete: true }
}

/// A summary the probe could not complete: it proves nothing.
fn proves_nothing() -> CalleeReturnWrites {
    CalleeReturnWrites { writes: Vec::new(), store_spaces: Vec::new(), complete: false }
}

fn callee_entry(fd: &Funcdata) -> Address {
    Address::new(space(fd, "ram"), 0x2000)
}

#[test]
fn the_call_seam_builds_the_pair_the_model_asked_for() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    assert_eq!(
        classify_call_output_pair(&fd, &[lo, hi], Some(&entry)),
        CallPairRepr::ScalarPair,
        "an unprobed callee leaves the model rule and the caller's reads standing",
    );
    assert!(build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)));

    let out = fd.obank().get(call).and_then(|o| o.get_out()).expect("the CALL gained an output");
    let outvn = fd.vbank().get(out).expect("output varnode");
    assert!(outvn.get_addr().is_join(), "the pair lands in join space");
    assert_eq!(outvn.get_size(), 16, "both registers are covered");
    for half in [lo, hi] {
        let def = fd.vbank().get(half).and_then(|v| v.get_def()).expect("half still defined");
        assert_eq!(
            fd.obank().get(def).expect("def").code(),
            OpCode::CPUI_SUBPIECE,
            "each half is now a SUBPIECE of the whole, not an INDIRECT creation",
        );
    }
}

/// The refutation this seam exists to answer. `one_scalar_callee` is
/// `movq %rdi,%rax; addq $7,%rax; ret` -- it provably never writes the second
/// return register -- and its caller reads that register twice anyway. Nothing
/// on the caller's side distinguishes that from a real `ScalarPair` consumer,
/// so the veto has to come from the callee.
#[test]
fn a_callee_proven_not_to_write_the_payload_is_not_paired() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    fd.kuna_set_callee_ret_writes(&entry, Rc::new(proves_nothing_written()));
    assert_eq!(
        classify_call_output_pair(&fd, &[lo, hi], Some(&entry)),
        CallPairRepr::CalleeScalar,
    );
    assert!(
        !build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)),
        "the callee refutes the pair",
    );
    assert!(fd.obank().get(call).and_then(|o| o.get_out()).is_none(), "the CALL is untouched");
    for half in [lo, hi] {
        let def = fd.vbank().get(half).and_then(|v| v.get_def()).expect("half still defined");
        assert!(
            fd.obank().get(def).expect("def").is_indirect_creation(),
            "the INDIRECT creations survive exactly as they did with the option off",
        );
    }
}

/// The veto is one-sided: a probe that could not finish proves nothing, and the
/// seam must not read "no recorded write" as "never written".
#[test]
fn an_incomplete_probe_vetoes_nothing() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    fd.kuna_set_callee_ret_writes(&entry, Rc::new(proves_nothing()));
    assert_eq!(
        classify_call_output_pair(&fd, &[lo, hi], Some(&entry)),
        CallPairRepr::ScalarPair,
    );
    assert!(build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)));
}

/// A summary that records the payload register as written is a real `ScalarPair`
/// producer -- the rustc `prod` shape -- and must still pair.
#[test]
fn a_callee_that_writes_the_payload_still_pairs() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    let ram_index = space(&fd, "ram").get_index();
    let written = CalleeReturnWrites { writes: vec![(ram_index, 0x40, 8)], store_spaces: Vec::new(), complete: true };
    assert!(!written.proves_untouched(&Address::new(space(&fd, "ram"), 0x40), 8));
    fd.kuna_set_callee_ret_writes(&entry, Rc::new(written));
    assert_eq!(
        classify_call_output_pair(&fd, &[lo, hi], Some(&entry)),
        CallPairRepr::ScalarPair,
    );
    assert!(build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)));
}

/// A write that only *overlaps* the payload half still refutes "untouched".
#[test]
fn a_partial_write_of_the_payload_register_counts() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let ram = space(&fd, "ram");
    let w = CalleeReturnWrites { writes: vec![(ram.get_index(), 0x40, 4)], store_spaces: Vec::new(), complete: true };
    assert!(
        !w.proves_untouched(&Address::new(Rc::clone(&ram), 0x40), 8),
        "`lea 0x7(%rdi),%edx` writes four bytes of an eight-byte half",
    );
    assert!(
        w.proves_untouched(&Address::new(ram, 0x80), 8),
        "a register the walk never wrote is still proven untouched",
    );
    let _ = &mut fd;
}

#[test]
fn the_call_seam_fails_closed_when_the_option_is_off() {
    let mut fd = build_call_fd(RustAbiMode::Off);
    let (call, lo, hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    assert!(!build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)));
    assert!(fd.obank().get(call).and_then(|o| o.get_out()).is_none());
}

#[test]
fn a_payload_half_nothing_reads_is_not_paired() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, hi) = build_call_with_pair(&mut fd, false);
    let entry = callee_entry(&fd);
    assert_eq!(classify_call_output_pair(&fd, &[lo, hi], Some(&entry)), CallPairRepr::Scalar);
    assert!(!build_call_output_pair(call, &mut fd, &[lo, hi], Some(&entry)));
}

#[test]
fn overlapping_halves_are_not_two_halves_of_one_value() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, _hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    assert_eq!(classify_call_output_pair(&fd, &[lo, lo], Some(&entry)), CallPairRepr::Scalar);
    assert!(!build_call_output_pair(call, &mut fd, &[lo, lo], Some(&entry)));
}

#[test]
fn a_single_trial_is_not_a_pair() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, _hi) = build_call_with_pair(&mut fd, true);
    let entry = callee_entry(&fd);
    assert_eq!(classify_call_output_pair(&fd, &[lo], Some(&entry)), CallPairRepr::Scalar);
    assert!(!build_call_output_pair(call, &mut fd, &[lo], Some(&entry)));
}

/// A half that is not a register INDIRECT creation is somebody else's Varnode.
#[test]
fn a_half_that_is_not_an_indirect_creation_is_not_a_pair() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (call, lo, _hi) = build_call_with_pair(&mut fd, true);
    let plain = unwritten(&mut fd, 0x4000, 8);
    let entry = callee_entry(&fd);
    assert_eq!(classify_call_output_pair(&fd, &[lo, plain], Some(&entry)), CallPairRepr::Scalar);
    assert!(!build_call_output_pair(call, &mut fd, &[lo, plain], Some(&entry)));
}

/// With no resolved callee there is nothing to probe, so the seam falls back on
/// the model rule and the caller's reads -- and says so.
#[test]
fn an_unresolved_callee_leaves_the_model_rule_standing() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let (_call, lo, hi) = build_call_with_pair(&mut fd, true);
    assert_eq!(classify_call_output_pair(&fd, &[lo, hi], None), CallPairRepr::ScalarPair);
}

/// A STORE's address is a runtime value, so a body that stores into a space
/// proves nothing about any location in it.
#[test]
fn a_store_into_the_space_defeats_the_proof() {
    let mut fd = build_call_fd(RustAbiMode::Always);
    let ram = space(&fd, "ram");
    let w = CalleeReturnWrites {
        writes: Vec::new(),
        store_spaces: vec![ram.get_index()],
        complete: true,
    };
    assert!(!w.proves_untouched(&Address::new(Rc::clone(&ram), 0x40), 8));
    let _ = &mut fd;
}
