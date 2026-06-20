//! Porter in-module tests for `userop.rs`.
//!
//! Cover the `UserPcodeOp` hierarchy constructors (incl. the BUILTIN_* ids and
//! kuna's `BUILTIN_MEMSET`), `UserOpManage` registration/lookup parity, and the
//! decode of real `<callotherfixup>`/`<segmentop>`/`<jumpassist>` elements
//! grepped from the vendored cspec/pspec files.

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, UniqueSpace};

use super::*;
use crate::inject_sleigh::PcodeInjectLibrarySleigh;
use crate::pcodeinject;

// A test architecture owning the SLEIGH injection library (mirrors the C++
// `Architecture` owning `pcodeinjectlib`). Decode goes through the real
// PcodeInjectLibrarySleigh allocate/decode/register chain, so the userop decode
// tests exercise the genuine library bookkeeping (parse/compile is the seam).
struct TestArch {
    user_op_names: Vec<Vec<u8>>,
    lib: PcodeInjectLibrarySleigh,
    uniq: Rc<AddrSpace>,
    code: Rc<AddrSpace>,
}

impl TestArch {
    fn new(user_op_names: Vec<Vec<u8>>) -> TestArch {
        let uniq: Rc<AddrSpace> = Rc::new(UniqueSpace::new(2, 0, false));
        let code: Rc<AddrSpace> = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        TestArch { user_op_names, lib: PcodeInjectLibrarySleigh::new(0x1000), uniq, code }
    }
}

impl InjectArchitecture for TestArch {
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.code)
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.uniq)
    }
}

impl UseropArchitecture for TestArch {
    fn get_user_op_names(&self) -> Vec<Vec<u8>> {
        self.user_op_names.clone()
    }
    fn decode_inject(
        &mut self,
        src: &[u8],
        suffix: &[u8],
        tp: int4,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<int4> {
        self.lib.decode_inject(src, suffix, tp, decoder)
    }
    fn get_call_other_target(&self, injectid: int4) -> Vec<u8> {
        self.lib.base.get_call_other_target(injectid)
    }
    fn payload_io_sizes(&self, injectid: int4) -> KunaResult<(int4, int4, int4, int4)> {
        let core = self.lib.get_payload(injectid).core();
        let in0 = if core.size_input() > 0 { core.get_input(0).get_size() as int4 } else { 0 };
        let in1 = if core.size_input() > 1 { core.get_input(1).get_size() as int4 } else { 0 };
        Ok((core.size_output(), core.size_input(), in0, in1))
    }
}

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    register_ids(&mut reg);
    pcodeinject::register_ids(&mut reg);
    reg
}

fn manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

// --- Constructors / BUILTIN ids -------------------------------------------

#[test]
fn builtin_ids_are_pinned() {
    assert_eq!(BUILTIN_STRINGDATA, 0x10000000);
    assert_eq!(BUILTIN_VOLATILE_READ, 0x10000001);
    assert_eq!(BUILTIN_VOLATILE_WRITE, 0x10000002);
    assert_eq!(BUILTIN_MEMCPY, 0x10000003);
    assert_eq!(BUILTIN_STRNCPY, 0x10000004);
    assert_eq!(BUILTIN_WCSNCPY, 0x10000005);
    // (kuna) GH-9230
    assert_eq!(BUILTIN_MEMSET, 0x10000006);
}

#[test]
fn volatile_read_flags_track_functional() {
    let nonfunc = UserPcodeOp::new_volatile_read(b"read_volatile", false);
    assert_eq!(nonfunc.get_display(), userop_flags::NO_OPERATOR);
    assert_eq!(nonfunc.get_index(), BUILTIN_VOLATILE_READ as int4);
    let func = UserPcodeOp::new_volatile_read(b"read_volatile", true);
    assert_eq!(func.get_display(), 0);
}

#[test]
fn volatile_write_flags_track_functional() {
    let nonfunc = UserPcodeOp::new_volatile_write(b"write_volatile", false);
    assert_eq!(nonfunc.get_display(), userop_flags::ANNOTATION_ASSIGNMENT);
    let func = UserPcodeOp::new_volatile_write(b"write_volatile", true);
    assert_eq!(func.get_display(), 0);
}

#[test]
fn internal_string_op_is_display_string() {
    let op = UserPcodeOp::new_internal_string();
    assert_eq!(op.get_name(), b"stringdata");
    assert_eq!(op.get_index(), BUILTIN_STRINGDATA as int4);
    assert_eq!(op.get_display(), userop_flags::DISPLAY_STRING);
    assert_eq!(op.get_type(), userop_type::string_data);
}

#[test]
fn append_size_matches_cpp() {
    assert_eq!(UserPcodeOp::append_size(b"read_volatile", 1), b"read_volatile_1");
    assert_eq!(UserPcodeOp::append_size(b"read_volatile", 2), b"read_volatile_2");
    assert_eq!(UserPcodeOp::append_size(b"read_volatile", 4), b"read_volatile_4");
    assert_eq!(UserPcodeOp::append_size(b"read_volatile", 8), b"read_volatile_8");
    // odd size -> "_<dec>"
    assert_eq!(UserPcodeOp::append_size(b"x", 3), b"x_3");
    assert_eq!(UserPcodeOp::append_size(b"x", 16), b"x_16");
}

#[test]
fn datatype_userop_pushes_only_nonnull_inputs() {
    let t = Rc::new(crate::dtype::Datatype::new(4, crate::dtype::type_metatype::TYPE_INT));
    let op = UserPcodeOp::new_datatype(
        b"builtin_memcpy",
        BUILTIN_MEMCPY as int4,
        Some(Rc::clone(&t)),
        &[Some(Rc::clone(&t)), None, Some(Rc::clone(&t))],
    );
    match &op.kind {
        UserOpKind::Datatype(d) => {
            // null in1 is skipped: only two inputs pushed.
            assert_eq!(d.in_types.len(), 2);
            assert!(d.out_type.is_some());
        }
        _ => panic!("expected Datatype kind"),
    }
}

// --- UserOpManage registration / lookup -----------------------------------

#[test]
fn initialize_assigns_unspecialized_by_index_skipping_blanks() {
    let mut mgr = UserOpManage::new();
    let mut arch = TestArch::new(vec![b"op0".to_vec(), Vec::new(), b"op2".to_vec()]);
    mgr.initialize(&arch).unwrap();
    // (initialize does not mutate the lib; arch borrowed immutably here)
    let _ = &mut arch;

    let op0 = mgr.get_op(0).unwrap();
    assert_eq!(op0.get_name(), b"op0");
    assert_eq!(op0.get_index(), 0);
    assert!(op0.is_unspecialized());
    // index 1 was a blank name -> hole.
    assert!(mgr.get_op(1).is_none());
    let op2 = mgr.get_op(2).unwrap();
    assert_eq!(op2.get_name(), b"op2");
    assert_eq!(op2.get_index(), 2);
    // by-name lookup
    assert_eq!(mgr.get_op_by_name(b"op2").unwrap().get_index(), 2);
    assert!(mgr.get_op_by_name(b"nope").is_none());
}

#[test]
fn register_op_conflicting_index_for_same_name_errors() {
    let mut mgr = UserOpManage::new();
    // op "a" at index 0
    mgr.register_op(UserPcodeOp::new_unspecialized(b"a", 0)).unwrap();
    // re-register "a" at a different index -> conflict
    let err = mgr.register_op(UserPcodeOp::new_unspecialized(b"a", 1)).unwrap_err();
    assert!(err.to_string().contains("Conflicting indices for userop name a"));
}

#[test]
fn register_op_same_index_different_name_errors() {
    let mut mgr = UserOpManage::new();
    mgr.register_op(UserPcodeOp::new_unspecialized(b"a", 0)).unwrap();
    let err = mgr.register_op(UserPcodeOp::new_unspecialized(b"b", 0)).unwrap_err();
    assert!(err.to_string().contains("User op b has same index as a"));
}

#[test]
fn register_op_negative_index_errors() {
    let mut mgr = UserOpManage::new();
    let err = mgr.register_op(UserPcodeOp::new_unspecialized(b"x", -1)).unwrap_err();
    assert!(err.to_string().contains("UserOp not assigned an index"));
}

// --- decode: <callotherfixup> ---------------------------------------------

#[test]
fn decode_call_other_fixup_overrides_unspecialized() {
    // The ARM_apcs <callotherfixup targetop="setISAMode"> shape.
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<callotherfixup targetop="setISAMode">
    <pcode incidentalcopy="true">
      <input name="newTB"/>
      <body><![CDATA[ r0 = r0; ]]></body>
    </pcode>
  </callotherfixup>"#;

    let mut userops = UserOpManage::new();
    // setISAMode must already be a SLEIGH userop (Unspecialized) for the
    // override to succeed.
    let mut arch = TestArch::new(vec![b"setISAMode".to_vec()]);
    userops.initialize(&arch).unwrap();
    let _ = &mut arch;

    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    userops.decode_call_other_fixup(&mut dec, &mut arch).unwrap();

    // The userop is now Injected, keeping setISAMode's index (0).
    let op = userops.get_op_by_name(b"setISAMode").unwrap();
    assert_eq!(op.get_type(), userop_type::injected);
    assert_eq!(op.get_index(), 0);
    assert!(op.get_inject_id().is_some());
    // The library registered the callother target name.
    assert_eq!(arch.lib.base.get_call_other_target(0), b"setISAMode");
}

#[test]
fn decode_call_other_fixup_unknown_userop_errors() {
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<callotherfixup targetop="ghost">
    <pcode><input name="a"/><body><![CDATA[ a = a; ]]></body></pcode>
  </callotherfixup>"#;
    let mut userops = UserOpManage::new();
    let mut arch = TestArch::new(vec![]); // no userops registered
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    let err = userops.decode_call_other_fixup(&mut dec, &mut arch).unwrap_err();
    assert!(err.to_string().contains("Unknown userop name in <callotherfixup>: ghost"));
}

// --- decode: <segmentop> ---------------------------------------------------

#[test]
fn decode_segmentop_reads_sizes_and_constresolve() {
    // The Z80 z80.pspec <segmentop space="ram" userop="segment" ...> shape.
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<segmentop space="ram" userop="segment" farpointer="yes">
    <pcode>
      <input name="base" size="2"/>
      <input name="inner" size="2"/>
      <output name="res" size="2"/>
      <body><![CDATA[ res = (base << 12) + inner; ]]></body>
    </pcode>
    <constresolve>
      <addr space="ram" offset="0x10" size="2"/>
    </constresolve>
  </segmentop>"#;

    let mut userops = UserOpManage::new();
    // "segment" must be a known Unspecialized userop.
    let mut arch = TestArch::new(vec![b"segment".to_vec()]);
    userops.initialize(&arch).unwrap();
    let _ = &mut arch;

    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    userops.decode_segment_op(&mut dec, &mut arch).unwrap();

    let op = userops.get_op_by_name(b"segment").unwrap();
    assert_eq!(op.get_type(), userop_type::segment);
    let seg = op.as_segment().unwrap();
    // two <input> tags => base+inner.
    assert_eq!(seg.baseinsize, 2);
    assert_eq!(seg.innerinsize, 2);
    assert!(seg.supportsfarpointer);
    // constresolve carries the <addr> (ram, 0x10, size 2).
    assert_eq!(seg.constresolve.offset, 0x10);
    assert_eq!(seg.constresolve.size, 2);
    // registered in the per-space table.
    assert_eq!(userops.num_segment_ops() as usize, mgr_spaces.get_space(1).unwrap().get_index() as usize + 1);
    assert!(userops.get_segment_op(mgr_spaces.get_space(1).unwrap().get_index()).is_some());
}

#[test]
fn decode_segmentop_missing_space_errors() {
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<segmentop userop="segment">
    <pcode><input name="inner" size="2"/><output name="res" size="2"/>
      <body><![CDATA[ res = inner; ]]></body></pcode>
  </segmentop>"#;
    let mut userops = UserOpManage::new();
    let mut arch = TestArch::new(vec![b"segment".to_vec()]);
    userops.initialize(&arch).unwrap();
    let _ = &mut arch;
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    let err = userops.decode_segment_op(&mut dec, &mut arch).unwrap_err();
    assert!(err.to_string().contains("<segmentop> expecting space attribute"));
}

// --- decode: <jumpassist> --------------------------------------------------

#[test]
fn decode_jumpassist_reads_all_scripts() {
    // The Dalvik <jumpassist name="switchAssist"> shape (case/addr/default).
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<jumpassist name="switchAssist">
    <case_pcode>
      <input name="index" size="4"/>
      <output name="finaladdr" size="4"/>
      <body><![CDATA[ finaladdr = index; ]]></body>
    </case_pcode>
    <addr_pcode>
      <input name="index" size="4"/>
      <output name="finaladdr" size="4"/>
      <body><![CDATA[ finaladdr = index; ]]></body>
    </addr_pcode>
    <default_pcode>
      <input name="index" size="4"/>
      <output name="finaladdr" size="4"/>
      <body><![CDATA[ finaladdr = index; ]]></body>
    </default_pcode>
  </jumpassist>"#;

    let mut userops = UserOpManage::new();
    // "switchAssist" must be a known Unspecialized userop.
    let mut arch = TestArch::new(vec![b"switchAssist".to_vec()]);
    userops.initialize(&arch).unwrap();
    let _ = &mut arch;

    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    userops.decode_jump_assist(&mut dec, &mut arch).unwrap();

    let op = userops.get_op_by_name(b"switchAssist").unwrap();
    assert_eq!(op.get_type(), userop_type::jumpassist);
    let ja = op.as_jump_assist().unwrap();
    // index2case present, addr+default present, calcsize absent (-1).
    assert!(ja.index2case >= 0);
    assert!(ja.index2addr >= 0);
    assert!(ja.defaultaddr >= 0);
    assert_eq!(ja.calcsize, -1);
    // index2addr/defaultaddr/case were registered as exe scripts.
    assert_eq!(arch.lib.injection.len(), 3);
}

#[test]
fn decode_jumpassist_missing_addr_pcode_errors() {
    let reg = registry();
    let mgr_spaces = manager();
    // only default_pcode, missing addr_pcode -> error
    let xml = br#"<jumpassist name="sa">
    <default_pcode>
      <input name="index" size="4"/>
      <output name="finaladdr" size="4"/>
      <body><![CDATA[ finaladdr = index; ]]></body>
    </default_pcode>
  </jumpassist>"#;
    let mut userops = UserOpManage::new();
    let mut arch = TestArch::new(vec![b"sa".to_vec()]);
    userops.initialize(&arch).unwrap();
    let _ = &mut arch;
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    let err = userops.decode_jump_assist(&mut dec, &mut arch).unwrap_err();
    assert!(err.to_string().contains("is missing <addr_pcode>"));
}

// --- decode: <volatile> ----------------------------------------------------

#[test]
fn decode_volatile_registers_read_and_write_builtins() {
    let reg = registry();
    let mgr_spaces = manager();
    // The HCS08 <volatile outputop=... inputop=...> attribute shape (the
    // <range> children are consumed by the caller, so we test just attributes).
    let xml = br#"<volatile outputop="write_volatile" inputop="read_volatile"/>"#;
    let mut userops = UserOpManage::new();
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    // open the element so the attribute scan sees the attributes.
    let elem = dec.open_element().unwrap();
    userops.decode_volatile(&mut dec).unwrap();
    dec.close_element(elem).unwrap();

    let rd = userops.get_op(BUILTIN_VOLATILE_READ).unwrap();
    assert_eq!(rd.get_name(), b"read_volatile");
    assert_eq!(rd.get_type(), userop_type::volatile_read);
    // non-functional default => no_operator
    assert_eq!(rd.get_display(), userop_flags::NO_OPERATOR);
    let wr = userops.get_op(BUILTIN_VOLATILE_WRITE).unwrap();
    assert_eq!(wr.get_name(), b"write_volatile");
    assert_eq!(wr.get_type(), userop_type::volatile_write);
}

#[test]
fn decode_volatile_missing_ops_errors() {
    let reg = registry();
    let mgr_spaces = manager();
    let xml = br#"<volatile inputop="read_volatile"/>"#;
    let mut userops = UserOpManage::new();
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element().unwrap();
    let err = userops.decode_volatile(&mut dec).unwrap_err();
    assert!(err.to_string().contains("Missing inputop/outputop"));
    dec.close_element(elem).unwrap();
}
