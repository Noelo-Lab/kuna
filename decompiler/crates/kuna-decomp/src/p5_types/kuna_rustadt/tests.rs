//! Tests for the Rust tagged two-variant return type.
//!
//! Every public entry point is driven directly, and every refusal in
//! [`super::analyze`] has its own case: the pass is a *classifier*, and a
//! classifier is only as good as the shapes it declines. The end-to-end witness
//! -- a rustc `Result<u32,u32>` producer with three return paths -- lives in
//! `tests/stages/kuna-rustadt.xml` and
//! `kuna-console/tests/verify_rustadt_variant.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace, UniqueSpace,
};

use crate::action::ActionContext;
use crate::context::{ArchContext, BlockId, TypeOp};
use crate::dtype::TypeFactoryImpl;

/// A [`kuna_base::space::RegisterLookup`] that knows no register names, which
/// forces `construct_join_address` down the formal-`JoinRecord` path -- the one
/// [`analyze`] reads the pair geometry out of.
struct NamelessRegisters;

impl kuna_base::space::RegisterLookup for NamelessRegisters {
    fn get_register(
        &self,
        _nm: &str,
    ) -> kuna_base::error::KunaResult<kuna_base::space::VarnodeStorage> {
        Err(kuna_base::error::KunaError::lowlevel("no registers"))
    }
    fn get_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
        String::new()
    }
    fn get_exact_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
        String::new()
    }
}

/// A fixture with a `join` space, a register lookup and a type factory: enough
/// to build the return-recovery concatenation storage AND intern the synthesized
/// aggregate.
fn build_fd(mode: RustAdtMode, is_rust: bool) -> Funcdata {
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

    let tf = TypeFactoryImpl::new();
    tf.setup_sizes(Some(4), 8, 8);
    tf.set_default_alignment_map();
    tf.set_max_basetype_size(8);

    let mut ctx = ArchContext::new(m);
    ctx.types = Some(Rc::new(tf));
    ctx.rust_adt = mode.as_u8();
    ctx.source_is_rust = is_rust;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("prod", "prod", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

/// The `join` address covering `ram:0x40` (the payload half, `hi_size` bytes)
/// over `ram:0x00` (the tag half, `lo_size` bytes).
fn join_addr(fd: &Funcdata, hi_size: int4, lo_size: int4) -> Address {
    let ram = space(fd, "ram");
    let manage = fd.get_arch().manage.clone();
    let rl = manage.register_lookup().expect("fixture register lookup");
    manage
        .construct_join_address(
            rl.as_ref(),
            &Address::new(Rc::clone(&ram), 0x40),
            hi_size,
            &Address::new(ram, 0x00),
            lo_size,
        )
        .expect("join address")
}

fn set_opcode(fd: &mut Funcdata, op: OpId, opc: OpCode) {
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
}

/// A fresh basic block hung off the graph root.
fn new_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().new_block_basic(root)
}

/// A payload the function COMPUTES: `hi = free + 1`, `hi_size` bytes wide at
/// `ram:0x40+off`. Arithmetic rather than a COPY on purpose -- a payload that is
/// only a move of something the function never wrote is the unit variant, and
/// [`ReturnSite::written_payload`] is supposed to say so.
fn payload_value(fd: &mut Funcdata, bl: BlockId, off: u64, hi_size: int4, at: u64) -> VarnodeId {
    let ram = space(fd, "ram");
    let src = fd.new_varnode(hi_size, &Address::new(Rc::clone(&ram), 0x200 + off), None);
    let one = fd.new_constant(hi_size, 1);
    let op = fd.new_op(2, Address::new(Rc::clone(&ram), at));
    set_opcode(fd, op, OpCode::CPUI_INT_ADD);
    let _ = fd.op_set_input(op, src, 0);
    let _ = fd.op_set_input(op, one, 1);
    let out = fd
        .new_varnode_out(hi_size, &Address::new(ram, 0x40 + off), op)
        .expect("payload varnode");
    fd.op_insert(op, bl, None);
    out
}

/// `RETURN(0, PIECE(payload, tag))` at the end of `bl`, with the concatenation
/// living in the two-piece `join` storage. Returns the RETURN op.
fn return_pair(
    fd: &mut Funcdata,
    bl: BlockId,
    tag: u64,
    hi_size: int4,
    lo_size: int4,
    at: u64,
) -> OpId {
    let ram = space(fd, "ram");
    let ja = join_addr(fd, hi_size, lo_size);
    let payload = payload_value(fd, bl, at & 0xff, hi_size, at);

    let piece = fd.new_op(2, Address::new(Rc::clone(&ram), at + 1));
    set_opcode(fd, piece, OpCode::CPUI_PIECE);
    let tagc = fd.new_constant(lo_size, tag);
    let _ = fd.op_set_input(piece, payload, 0);
    let _ = fd.op_set_input(piece, tagc, 1);
    let whole = fd.new_varnode_out(hi_size + lo_size, &ja, piece).expect("join varnode");
    fd.op_insert(piece, bl, None);

    let ret = fd.new_op(2, Address::new(ram, at + 2));
    set_opcode(fd, ret, OpCode::CPUI_RETURN);
    let zero = fd.new_constant(8, 0);
    let _ = fd.op_set_input(ret, zero, 0);
    let _ = fd.op_set_input(ret, whole, 1);
    fd.op_insert(ret, bl, None);
    ret
}

/// The canonical witness: an entry block branching to two return blocks whose
/// tags are `0` and `1`.
fn two_variant_fd(mode: RustAdtMode) -> Funcdata {
    let mut fd = build_fd(mode, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);
    // One op in the entry so it is reachable from the op walk `variant_regions`
    // uses (a block with no ops is not a block the pass can pin anything to).
    payload_value(&mut fd, entry, 0x01, 8, ENTRY_ADDR);
    return_pair(&mut fd, a, 0, 8, 8, 0x1100);
    return_pair(&mut fd, b, 1, 8, 8, 0x1200);
    fd
}

/// The address of the marker op [`two_variant_fd`] puts in the entry block.
const ENTRY_ADDR: u64 = 0x1050;

/// The entry block of a [`two_variant_fd`] fixture, found by its marker op.
fn entry_block(fd: &Funcdata) -> BlockId {
    for op in fd.obank().iter_alive() {
        let o = fd.obank().get(op).unwrap();
        if o.get_addr().get_offset() == ENTRY_ADDR {
            return o.get_parent().expect("marker op is in a block");
        }
    }
    panic!("entry marker op not found");
}

// -- the option ------------------------------------------------------------

#[test]
fn the_three_modes_parse_and_round_trip() {
    for (tok, want) in [
        ("off", RustAdtMode::Off),
        ("0", RustAdtMode::Off),
        ("false", RustAdtMode::Off),
        ("auto", RustAdtMode::Auto),
        ("on", RustAdtMode::Auto),
        ("always", RustAdtMode::Always),
        ("  ALWAYS  ", RustAdtMode::Always),
    ] {
        let (mode, msg) = parse_rust_adt_mode(tok).expect("parses");
        assert_eq!(mode, want, "token {tok}");
        assert!(msg.contains(want.as_str()), "message names the mode: {msg}");
        assert_eq!(RustAdtMode::from_u8(mode.as_u8()), mode, "wire round-trip");
    }
}

#[test]
fn an_unknown_mode_token_is_an_error() {
    assert!(parse_rust_adt_mode("maybe").is_err());
    assert!(parse_rust_adt_mode("").is_err());
}

#[test]
fn an_unrecognized_wire_value_reads_as_off() {
    assert_eq!(RustAdtMode::from_u8(7), RustAdtMode::Off);
}

#[test]
fn the_gate_is_the_mode_crossed_with_the_detected_language() {
    assert!(!live(&build_fd(RustAdtMode::Off, true)), "off never acts");
    assert!(!live(&build_fd(RustAdtMode::Auto, false)), "auto needs a rustc image");
    assert!(live(&build_fd(RustAdtMode::Auto, true)));
    assert!(live(&build_fd(RustAdtMode::Always, false)), "always drops the language test");
}

// -- analyze: the accepted shape ------------------------------------------

#[test]
fn two_return_paths_with_distinct_tags_are_the_two_variant_case() {
    let fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("the two-variant shape is recognized");
    assert_eq!(shape.total, 16);
    assert_eq!(shape.payload_off, 8, "the tag half is the least-significant piece");
    assert_eq!(shape.payload_size, 8);
    assert_eq!(shape.tags, [0, 1]);
    assert_eq!(shape.sites.len(), 2);
    assert_eq!(shape.variant_of(0), Some(0));
    assert_eq!(shape.variant_of(1), Some(1));
    assert_eq!(shape.variant_of(9), None, "a tag that is not one of the two has no variant");
    for s in &shape.sites {
        assert!(s.payload.is_some(), "the payload half is isolated at every site");
    }
}

#[test]
fn a_narrow_payload_inside_a_wider_register_is_still_found() {
    // rustc puts a 4-byte payload at the bottom of an 8-byte second return
    // register; the search steps the width down rather than widening.
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);

    let ram = space(&fd, "ram");
    let ja = join_addr(&fd, 8, 8);
    for (bl, tag, at) in [(a, 0u64, 0x1100u64), (b, 1, 0x1200)] {
        let narrow = payload_value(&mut fd, bl, at & 0xff, 4, at);
        let zx = fd.new_op(1, Address::new(Rc::clone(&ram), at + 1));
        set_opcode(&mut fd, zx, OpCode::CPUI_INT_ZEXT);
        let _ = fd.op_set_input(zx, narrow, 0);
        let wide = fd
            .new_varnode_out(8, &Address::new(Rc::clone(&ram), 0x300 + at), zx)
            .expect("zext out");
        fd.op_insert(zx, bl, None);

        let piece = fd.new_op(2, Address::new(Rc::clone(&ram), at + 2));
        set_opcode(&mut fd, piece, OpCode::CPUI_PIECE);
        let tagc = fd.new_constant(8, tag);
        let _ = fd.op_set_input(piece, wide, 0);
        let _ = fd.op_set_input(piece, tagc, 1);
        let whole = fd.new_varnode_out(16, &ja, piece).expect("join varnode");
        fd.op_insert(piece, bl, None);

        let ret = fd.new_op(2, Address::new(Rc::clone(&ram), at + 3));
        set_opcode(&mut fd, ret, OpCode::CPUI_RETURN);
        let zero = fd.new_constant(8, 0);
        let _ = fd.op_set_input(ret, zero, 0);
        let _ = fd.op_set_input(ret, whole, 1);
        fd.op_insert(ret, bl, None);
    }
    let shape = analyze(&fd).expect("recognized");
    let p = shape.sites[0].payload.expect("payload isolated");
    assert_eq!(fd.vbank().get(p).unwrap().get_size(), 4, "the 4-byte value, not the 8-byte ZEXT");
}

// -- analyze: every refusal ------------------------------------------------

#[test]
fn one_return_path_is_not_a_two_variant_case() {
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let a = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, a);
    return_pair(&mut fd, a, 0, 8, 8, 0x1100);
    assert!(analyze(&fd).is_none(), "the branchless single-return shape is out of scope");
}

#[test]
fn a_return_that_is_not_a_join_pair_is_refused() {
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);
    return_pair(&mut fd, a, 0, 8, 8, 0x1100);
    // The second return hands back a plain register, not the concatenation.
    let ram = space(&fd, "ram");
    let plain = payload_value(&mut fd, b, 0x99, 8, 0x1200);
    let ret = fd.new_op(2, Address::new(ram, 0x1202));
    set_opcode(&mut fd, ret, OpCode::CPUI_RETURN);
    let zero = fd.new_constant(8, 0);
    let _ = fd.op_set_input(ret, zero, 0);
    let _ = fd.op_set_input(ret, plain, 1);
    fd.op_insert(ret, b, None);
    assert!(analyze(&fd).is_none());
}

#[test]
fn a_computed_tag_on_any_path_refuses_the_whole_function() {
    // A path whose tag is a `setCC` rather than a constant leaves that site's
    // variant unknowable, and a partially classified union hands the rest back
    // to `ScoreUnionFields` -- which is what the pass exists to keep out.
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);
    return_pair(&mut fd, a, 0, 8, 8, 0x1100);

    let ram = space(&fd, "ram");
    let ja = join_addr(&fd, 8, 8);
    let payload = payload_value(&mut fd, b, 0x77, 8, 0x1200);
    let computed = payload_value(&mut fd, b, 0x78, 8, 0x1201);
    let piece = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1202));
    set_opcode(&mut fd, piece, OpCode::CPUI_PIECE);
    let _ = fd.op_set_input(piece, payload, 0);
    let _ = fd.op_set_input(piece, computed, 1);
    let whole = fd.new_varnode_out(16, &ja, piece).expect("join varnode");
    fd.op_insert(piece, b, None);
    let ret = fd.new_op(2, Address::new(ram, 0x1203));
    set_opcode(&mut fd, ret, OpCode::CPUI_RETURN);
    let zero = fd.new_constant(8, 0);
    let _ = fd.op_set_input(ret, zero, 0);
    let _ = fd.op_set_input(ret, whole, 1);
    fd.op_insert(ret, b, None);

    assert!(analyze(&fd).is_none());
}

#[test]
fn one_tag_value_across_every_path_is_not_two_variants() {
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);
    return_pair(&mut fd, a, 0, 8, 8, 0x1100);
    return_pair(&mut fd, b, 0, 8, 8, 0x1200);
    assert!(analyze(&fd).is_none());
}

#[test]
fn three_distinct_tags_are_out_of_scope() {
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    for (i, tag) in [0u64, 1, 2].into_iter().enumerate() {
        let bl = new_block(&mut fd);
        fd.bblocks_mut().add_edge(entry, bl);
        return_pair(&mut fd, bl, tag, 8, 8, 0x1100 + 0x100 * i as u64);
    }
    assert!(analyze(&fd).is_none(), "three-or-more variants are the next PR's problem");
}

// -- const_low: the tag reader --------------------------------------------

#[test]
fn a_fully_shifted_out_tag_reads_as_zero() {
    // `ZEXT(payload) << 0x40` is what the rule pool leaves of a zero tag, and
    // its low eight bytes are provably zero.
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let bl = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, bl);
    let ram = space(&fd, "ram");
    let narrow = payload_value(&mut fd, bl, 0x10, 4, 0x1100);
    let zx = fd.new_op(1, Address::new(Rc::clone(&ram), 0x1101));
    set_opcode(&mut fd, zx, OpCode::CPUI_INT_ZEXT);
    let _ = fd.op_set_input(zx, narrow, 0);
    let wide = fd.new_varnode_out(16, &Address::new(Rc::clone(&ram), 0x400), zx).unwrap();
    fd.op_insert(zx, bl, None);
    let shl = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1102));
    set_opcode(&mut fd, shl, OpCode::CPUI_INT_LEFT);
    let c = fd.new_constant(4, 0x40);
    let _ = fd.op_set_input(shl, wide, 0);
    let _ = fd.op_set_input(shl, c, 1);
    let out = fd.new_varnode_out(16, &Address::new(ram, 0x420), shl).unwrap();
    fd.op_insert(shl, bl, None);

    assert_eq!(const_low(&fd, out, 8, 0), Some(0));
    assert_eq!(const_low(&fd, out, 9, 0), None, "a shift shorter than the request proves nothing");
    assert_eq!(const_low(&fd, narrow, 4, 0), None, "a computed value is not a constant");
    assert!(is_computed(&fd, narrow, 0), "the function computed it");
    assert_eq!(const_low(&fd, out, 0, 0), None, "a zero-width request is refused");
}

// -- synthesize ------------------------------------------------------------

#[test]
fn the_synthesized_type_is_a_tag_and_a_two_member_variant_union() {
    let fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    assert_eq!(ty.get_metatype(), type_metatype::TYPE_STRUCT);
    assert_eq!(ty.get_size(), shape.total, "the type covers the whole returned value");
    let tag = ty.get_field(0).expect("tag field");
    assert_eq!(tag.name, "tag");
    assert_eq!(tag.offset, 0);
    let pay = ty.get_field(1).expect("payload field");
    assert_eq!(pay.name, "payload");
    assert_eq!(pay.offset, shape.payload_off);
    let un = payload_union(&ty).expect("the payload field is the variant union");
    assert_eq!(un.get_metatype(), type_metatype::TYPE_UNION);
    assert_eq!(un.get_field(0).unwrap().name, "Variant0");
    assert_eq!(un.get_field(1).unwrap().name, "Variant1");
    assert_eq!(un.get_field(0).unwrap().offset, 0, "a union member IS the variant overlay");
    assert_eq!(un.get_field(1).unwrap().offset, 0);
}

#[test]
fn the_variant_names_are_neutral_not_ok_and_err() {
    // Tagged `Result` is Ok=0/Err=1; tagged `Option` is None=0/Some=1. Opposite
    // polarity, identical codegen -- so a name is a coin flip that reads as
    // confident, and the pass refuses to make it.
    let fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    let un = payload_union(&ty).expect("union");
    for n in ["Ok", "Err", "Some", "None"] {
        assert!(
            (0..2).all(|i| un.get_field(i).unwrap().name != n),
            "no member is named {n} without corroboration"
        );
    }
}

#[test]
fn synthesizing_the_same_shape_twice_reuses_the_interned_type() {
    // `Datatype::hash_name` makes the registered NAME the type id and the factory
    // refuses a second, different definition of a name it holds; a second pass
    // over the same shape must land on the same type, not fail.
    let fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let a = synthesize(&fd, &shape).expect("first");
    let b = synthesize(&fd, &shape).expect("second");
    assert!(Rc::ptr_eq(&a, &b), "the same shape interns to the same type");
}

#[test]
fn a_name_already_held_by_a_different_shape_steps_aside() {
    let fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let types = fd.get_arch().types().expect("factory");
    let want = format!(
        "RustEnum2_{}_{}_{}_{}_{}_{}",
        shape.total, shape.payload_off, shape.tags[0], shape.tags[1], "undefined8", "undefined8"
    );
    // Squat on the name with a struct of a different size.
    let squat = types.get_type_struct(&want).expect("shell");
    let f = TypeField::new(0, 0, "x", types.get_base(1, type_metatype::TYPE_UINT).unwrap());
    types.set_fields_struct_raw(&squat, vec![f], Vec::new(), 1, 1, 0).expect("squatter");
    let ty = synthesize(&fd, &shape).expect("synthesized around the collision");
    assert_ne!(ty.get_name(), want, "the held name is stepped over, not fought");
    assert_eq!(ty.get_size(), shape.total);
}

#[test]
fn payload_union_refuses_a_type_that_is_not_a_variant_aggregate() {
    let fd = build_fd(RustAdtMode::Always, true);
    let types = fd.get_arch().types().expect("factory");
    let plain = types.get_base(4, type_metatype::TYPE_UINT).unwrap();
    assert!(payload_union(&plain).is_none());
    let shell = types.get_type_struct("NotAVariant").expect("shell");
    let f = TypeField::new(0, 0, "x", Rc::clone(&plain));
    let s = types.set_fields_struct_raw(&shell, vec![f], Vec::new(), 4, 1, 0).expect("struct");
    assert!(payload_union(&s).is_none(), "a struct with no union member is not one");
}

// -- install ---------------------------------------------------------------

#[test]
fn install_locks_the_type_on_every_return_and_is_idempotent() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    assert_eq!(install(&mut fd, &shape, &ty), 2, "both return values are typed");
    for s in &shape.sites {
        let v = fd.vbank().get(s.vn).expect("live");
        assert!(v.is_type_lock(), "locked, so type propagation cannot undo it");
        assert!(Rc::ptr_eq(v.get_type(), &ty));
    }
    assert_eq!(install(&mut fd, &shape, &ty), 0, "a second install changes nothing");
    assert!(already_typed(&fd, &shape));
}

#[test]
fn install_records_the_variant_constructor_for_each_site() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    install(&mut fd, &shape, &ty);
    for s in &shape.sites {
        let (name, payload) = fd.kuna_rustadt_ctor(s.op).expect("a constructor per site");
        assert_eq!(name, format!("Variant{}", shape.variant_of(s.tag).unwrap()));
        assert_eq!(payload, s.payload, "the site's payload is a value the function wrote");
    }
}

#[test]
fn a_path_that_writes_no_payload_gets_an_argument_less_constructor() {
    // rustc leaves the payload register alone on a `None` path; the leftover the
    // caller happened to put there is not a returned value.
    let mut fd = build_fd(RustAdtMode::Always, true);
    let root = fd.bblocks_root_pub();
    let entry = new_block(&mut fd);
    fd.bblocks_mut().set_start_block(root, entry);
    let a = new_block(&mut fd);
    let b = new_block(&mut fd);
    fd.bblocks_mut().add_edge(entry, a);
    fd.bblocks_mut().add_edge(entry, b);

    let ram = space(&fd, "ram");
    let ja = join_addr(&fd, 8, 8);
    // Path A: the payload half is an unwritten register.
    let leftover = fd.new_varnode(8, &Address::new(Rc::clone(&ram), 0x40), None);
    let piece = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1100));
    set_opcode(&mut fd, piece, OpCode::CPUI_PIECE);
    let tagc = fd.new_constant(8, 0);
    let _ = fd.op_set_input(piece, leftover, 0);
    let _ = fd.op_set_input(piece, tagc, 1);
    let whole = fd.new_varnode_out(16, &ja, piece).expect("join varnode");
    fd.op_insert(piece, a, None);
    let ret = fd.new_op(2, Address::new(ram, 0x1101));
    set_opcode(&mut fd, ret, OpCode::CPUI_RETURN);
    let zero = fd.new_constant(8, 0);
    let _ = fd.op_set_input(ret, zero, 0);
    let _ = fd.op_set_input(ret, whole, 1);
    fd.op_insert(ret, a, None);
    // Path B: a real payload.
    return_pair(&mut fd, b, 1, 8, 8, 0x1200);

    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    install(&mut fd, &shape, &ty);
    let unit = shape.sites.iter().find(|s| s.tag == 0).expect("the unit site");
    assert_eq!(fd.kuna_rustadt_ctor(unit.op).unwrap().1, None, "no argument for the unit variant");
    let full = shape.sites.iter().find(|s| s.tag == 1).expect("the payload site");
    assert!(fd.kuna_rustadt_ctor(full.op).unwrap().1.is_some());
}

// -- facet pinning ---------------------------------------------------------

#[test]
fn the_facet_comes_from_the_guard_not_from_a_score() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let ty = synthesize(&fd, &shape).expect("synthesized");
    install(&mut fd, &shape, &ty);
    assert!(pin_facets(&mut fd, &shape, &ty) > 0);
    let un = payload_union(&ty).expect("union");
    for s in &shape.sites {
        let want = shape.variant_of(s.tag).expect("variant");
        let got = fd
            .get_union_resolution(&un, s.op, 1)
            .expect("the return site's facet is pinned")
            .get_field_num();
        assert_eq!(got, want, "the variant the path's tag selected");
    }
}

#[test]
fn a_block_that_can_still_reach_both_variants_is_left_unpinned() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let entry = entry_block(&fd);
    let regions = variant_regions(&fd, &shape);
    let ret_blocks: Vec<BlockId> = shape
        .sites
        .iter()
        .map(|s| fd.obank().get(s.op).unwrap().get_parent().unwrap())
        .collect();
    for b in &ret_blocks {
        assert!(regions.iter().any(|(rb, _)| rb == b), "a return block is its own variant");
    }
    assert!(
        !regions.iter().any(|(rb, _)| *rb == entry),
        "the entry reaches both variants, so no facet is claimed for it"
    );
}

#[test]
fn pinning_declines_a_type_that_carries_no_variant_union() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&fd).expect("recognized");
    let types = fd.get_arch().types().expect("factory");
    let plain = types.get_base(8, type_metatype::TYPE_UINT).unwrap();
    assert_eq!(pin_facets(&mut fd, &shape, &plain), 0);
}

// -- the actions -----------------------------------------------------------

#[test]
fn the_install_action_is_inert_with_the_gate_off() {
    let mut fd = two_variant_fd(RustAdtMode::Off);
    let mut act = ActionRustAdt::boxed("typerecovery");
    let mut ctx = ActionContext::default();
    assert_eq!(act.apply(&mut fd, &mut ctx), 0);
    let shape = analyze(&fd).expect("the shape is still there");
    for s in &shape.sites {
        assert!(!fd.vbank().get(s.vn).unwrap().is_type_lock(), "nothing was typed");
    }
}

#[test]
fn the_install_action_types_the_return_when_the_gate_is_on() {
    let mut fd = two_variant_fd(RustAdtMode::Always);
    let mut act = ActionRustAdt::boxed("typerecovery");
    let mut ctx = ActionContext::default();
    assert_eq!(act.apply(&mut fd, &mut ctx), 2);
    assert_eq!(act.apply(&mut fd, &mut ctx), 0, "already typed: nothing more to do");
}

#[test]
fn the_facet_action_is_inert_with_the_gate_off_and_before_the_type_is_installed() {
    let mut ctx = ActionContext::default();
    let mut off = two_variant_fd(RustAdtMode::Off);
    let mut facet = ActionRustAdtFacet::boxed("typerecovery");
    assert_eq!(facet.apply(&mut off, &mut ctx), 0);

    // Gate on, but nothing has installed a locked type yet: nothing to pin to.
    let mut on = two_variant_fd(RustAdtMode::Always);
    let shape = analyze(&on).expect("recognized");
    assert_eq!(facet.apply(&mut on, &mut ctx), 0);
    let ty = synthesize(&on, &shape).expect("synthesized");
    let un = payload_union(&ty).expect("union");
    assert!(on.get_union_resolution(&un, shape.sites[0].op, 1).is_none());
}
