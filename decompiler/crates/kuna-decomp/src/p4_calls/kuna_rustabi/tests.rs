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
