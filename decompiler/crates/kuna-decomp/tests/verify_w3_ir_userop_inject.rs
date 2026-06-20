//! Verifier adversarial tests for item `w3-ir-userop-inject`.
//!
//! INDEPENDENT of the porter's in-module tests. They drive only the public
//! `userop`/`pcodeinject`/`inject_sleigh` API and target the hunt-list spots the
//! review flagged as most fragile for this wave (the IR-injection layer):
//!
//!  - **Allocation-order fidelity** of `InjectPayload::orderParameters`
//!    (pcodeinject.cc:67-79): indices run inputs-first then outputs, sequential
//!    from 0 — the "create_index allocation order" the wave instructions call
//!    out. Tested with multiple inputs AND outputs and via the post-decode path.
//!  - **`UserOpManage::getOp(uint4)` index/builtin boundary**
//!    (userop.cc:409-416): `i < useroplist.size()` returns the (possibly null)
//!    slot WITHOUT consulting builtinmap; a builtin id far beyond the list falls
//!    through to builtinmap. The hole-in-range case must NOT leak a builtin.
//!  - **`registerOp` segment table** (userop.cc:527-538): a second SegmentOp for
//!    the same space throws "Multiple segmentops defined for same space"; the
//!    per-space `segmentop` vector is grown to the space index.
//!  - **`SnippetLayout::build` empty + boundary** (pcodeinject.cc:152-174): zero
//!    params leaves `uniqReserve == 0x10`; the 0x10 + k*0x20 stride is exact.
//!  - **`getPayloadId` namespace routing + the four reverse vectors**
//!    (pcodeinject.cc:285-340): each type routes to its own map; wrong-type and
//!    out-of-range/negative ids return -1 / "".
//!  - **`manual_call_other_fixup` parameter order** (inject_sleigh.cc:504-518):
//!    inputs pushed in order, then output, then orderParameters re-indexes.

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, UniqueSpace};
use kuna_base::types::int4;

use kuna_decomp::inject_sleigh::PcodeInjectLibrarySleigh;
use kuna_decomp::pcodeinject::{
    self, InjectArchitecture, InjectContext, InjectParameter, InjectPayloadCore,
    PcodeInjectLibraryBase, SnippetLayout, CALLFIXUP_TYPE, CALLMECHANISM_TYPE, CALLOTHERFIXUP_TYPE,
    EXECUTABLEPCODE_TYPE,
};
use kuna_decomp::userop::{
    register_ids, UserOpManage, UserPcodeOp, UseropArchitecture, BUILTIN_VOLATILE_READ,
};

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

struct TestArch {
    user_op_names: Vec<Vec<u8>>,
    lib: PcodeInjectLibrarySleigh,
    uniq: Rc<AddrSpace>,
    code: Rc<AddrSpace>,
}

impl TestArch {
    fn new(names: Vec<Vec<u8>>) -> TestArch {
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
        TestArch { user_op_names: names, lib: PcodeInjectLibrarySleigh::new(0x1000), uniq, code }
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
    ) -> kuna_base::error::KunaResult<int4> {
        self.lib.decode_inject(src, suffix, tp, decoder)
    }
    fn get_call_other_target(&self, injectid: int4) -> Vec<u8> {
        self.lib.base.get_call_other_target(injectid)
    }
    fn payload_io_sizes(&self, injectid: int4) -> kuna_base::error::KunaResult<(int4, int4, int4, int4)> {
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

fn space_manager() -> AddrSpaceManager {
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

// ---------------------------------------------------------------------------
// F-target 1: orderParameters allocation order (inputs-first, then outputs)
// ---------------------------------------------------------------------------

/// pcodeinject.cc:67-79 — `orderParameters` runs a single counter `id` across
/// ALL inputs in declaration order, THEN all outputs. With 3 inputs + 2 outputs
/// the indices must be 0,1,2 / 3,4 — outputs continue the input counter, they do
/// not restart at 0. A `BTreeMap` or any reordering of the param vectors would
/// break this; we assert the exact sequence on both the input and output sides.
#[test]
fn order_parameters_inputs_then_outputs_single_counter() {
    let mut core = InjectPayloadCore::new(b"p", EXECUTABLEPCODE_TYPE);
    core.inputlist.push(InjectParameter::new(b"i0", 4));
    core.inputlist.push(InjectParameter::new(b"i1", 4));
    core.inputlist.push(InjectParameter::new(b"i2", 2));
    core.output.push(InjectParameter::new(b"o0", 4));
    core.output.push(InjectParameter::new(b"o1", 1));
    core.order_parameters();

    assert_eq!(core.get_input(0).get_index(), 0);
    assert_eq!(core.get_input(1).get_index(), 1);
    assert_eq!(core.get_input(2).get_index(), 2);
    // outputs continue the counter, they do NOT restart at 0.
    assert_eq!(core.get_output(0).get_index(), 3);
    assert_eq!(core.get_output(1).get_index(), 4);

    // Re-ordering is idempotent (re-runs reproduce the same indices).
    core.order_parameters();
    assert_eq!(core.get_output(1).get_index(), 4);
}

/// Same allocation order, but exercised through the real decode path
/// (`decodePayloadParams`): the XML declares <output> BEFORE <input>. The C++
/// `decodePayloadParams` (pcodeinject.cc:111-132) pushes each child onto the
/// correct vector as it is *peeked in document order*, then orderParameters
/// indexes inputs-first regardless of document order. So an output declared
/// first still gets the HIGHER index. This catches a port that mistakenly keys
/// indices off document order.
#[test]
fn decode_payload_params_indexes_inputs_first_despite_doc_order() {
    let reg = registry();
    let mgr = space_manager();
    // <output> appears textually before <input>.
    let xml = br#"<pcode>
      <output name="res" size="4"/>
      <input name="a" size="4"/>
      <input name="b" size="2"/>
      <body><![CDATA[ res = a + b; ]]></body>
    </pcode>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element_id(&pcodeinject::ELEM_PCODE).unwrap();
    let mut core = InjectPayloadCore::new(b"p", EXECUTABLEPCODE_TYPE);
    core.decode_payload_attributes(&mut dec).unwrap();
    core.decode_payload_params(&mut dec).unwrap();
    // Stop before <body> — params already consumed.
    let _ = elem;

    assert_eq!(core.size_input(), 2);
    assert_eq!(core.size_output(), 1);
    // inputs got 0,1; the (textually-first) output got 2.
    assert_eq!(core.get_input(0).get_index(), 0);
    assert_eq!(core.get_input(1).get_index(), 1);
    assert_eq!(core.get_output(0).get_index(), 2);
}

// ---------------------------------------------------------------------------
// F-target 2: getOp(uint4) index/builtin boundary
// ---------------------------------------------------------------------------

/// userop.cc:409-416 — `getOp(uint4 i)` returns `useroplist[i]` (possibly the
/// null slot) WHEN `i < useroplist.size()`, and only falls through to
/// `builtinmap` when `i >= useroplist.size()`. A builtin id (0x10000001) is far
/// beyond any list, so it routes to the builtin map; a hole inside the list
/// range must return None and must NOT accidentally surface a builtin.
#[test]
fn get_op_hole_in_range_does_not_leak_builtin() {
    let mut mgr = UserOpManage::new();
    // names: index 0 set, index 1 blank (hole), index 2 set.
    let arch = TestArch::new(vec![b"op0".to_vec(), Vec::new(), b"op2".to_vec()]);
    mgr.initialize(&arch).unwrap();

    // Register a volatile-read builtin via decode_volatile so builtinmap is
    // non-empty (id BUILTIN_VOLATILE_READ).
    let reg = registry();
    let mgr_spaces = space_manager();
    let xml = br#"<volatile outputop="write_volatile" inputop="read_volatile"/>"#;
    let mut dec = XmlDecode::new(&mgr_spaces, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element().unwrap();
    mgr.decode_volatile(&mut dec).unwrap();
    dec.close_element(elem).unwrap();

    // The hole at index 1 returns None (NOT a builtin).
    assert!(mgr.get_op(1).is_none());
    // index 0/2 are real.
    assert_eq!(mgr.get_op(0).unwrap().get_name(), b"op0");
    assert_eq!(mgr.get_op(2).unwrap().get_name(), b"op2");
    // The builtin id (way beyond useroplist.len()==3) routes to builtinmap.
    let rd = mgr.get_op(BUILTIN_VOLATILE_READ).unwrap();
    assert_eq!(rd.get_name(), b"read_volatile");
    // An unregistered id beyond the list and not a builtin -> None.
    assert!(mgr.get_op(0x09999999).is_none());
}

// ---------------------------------------------------------------------------
// F-target 3: registerOp "Multiple segmentops defined for same space"
// ---------------------------------------------------------------------------

/// userop.cc:527-538 — registering two SegmentOps that resolve to the same
/// address-space index must throw on the second. We build the SegmentOps
/// directly (bypassing decode) with the same space and DIFFERENT names/indices
/// so the per-space `segmentop` collision (not the name/index collision) is the
/// thing that trips.
#[test]
fn register_op_rejects_two_segmentops_for_same_space() {
    let mut mgr = UserOpManage::new();
    // AddrSpace::new(tp, nm, big_end, size, ws, ind, flags, dl, dead): the index
    // is the 6th positional arg (== 7 here).
    let space: Rc<AddrSpace> = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        7, // index 7
        addrspace_flags::hasphysical,
        1,
        1,
    ));

    let mut seg0 = UserPcodeOp::new_segment(b"seg0", 0);
    if let kuna_decomp::userop::UserOpKind::Segment(s) = &mut seg0.kind {
        s.spc = Some(Rc::clone(&space));
    }
    mgr.register_op(seg0).unwrap();
    // segmentop vector grown to the space index (7) -> len 8.
    assert_eq!(mgr.num_segment_ops(), 8);
    assert!(mgr.get_segment_op(7).is_some());
    // a lower space index slot is a hole.
    assert!(mgr.get_segment_op(0).is_none());

    let mut seg1 = UserPcodeOp::new_segment(b"seg1", 1);
    if let kuna_decomp::userop::UserOpKind::Segment(s) = &mut seg1.kind {
        s.spc = Some(space);
    }
    let err = mgr.register_op(seg1).unwrap_err();
    assert!(err.to_string().contains("Multiple segmentops defined for same space"));

    // get_segment_op past the end returns None (boundary, not panic).
    assert!(mgr.get_segment_op(99).is_none());
}

// ---------------------------------------------------------------------------
// F-target 4: SnippetLayout empty + stride boundary
// ---------------------------------------------------------------------------

struct LayoutArch {
    uniq: Rc<AddrSpace>,
    code: Rc<AddrSpace>,
}
impl InjectArchitecture for LayoutArch {
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.code)
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.uniq)
    }
}

/// pcodeinject.cc:152-174 — with NO inputs and NO outputs the reservation never
/// advances: `uniqReserve` stays at the initial 0x10, and both id lists are
/// empty. (The empty-container boundary of `ExecutablePcode::build`.)
#[test]
fn snippet_layout_empty_payload_keeps_initial_reserve() {
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
    let arch = LayoutArch { uniq, code };
    let core = InjectPayloadCore::new(b"empty", EXECUTABLEPCODE_TYPE);
    let mut ctx = InjectContext::default();
    let layout = SnippetLayout::build(&core, &mut ctx, &arch);
    assert!(layout.input_list.is_empty());
    assert!(layout.output_list.is_empty());
    assert_eq!(layout.uniq_reserve, 0x10);
    assert!(ctx.inputlist.is_empty());
    assert!(ctx.output.is_empty());
}

/// Output-only payload: the first output still starts at 0x10 (the input loop
/// reserved nothing), and the stride is exactly 0x20.
#[test]
fn snippet_layout_output_only_starts_at_first_slot() {
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
    let arch = LayoutArch { uniq, code };
    let mut core = InjectPayloadCore::new(b"o", EXECUTABLEPCODE_TYPE);
    core.output.push(InjectParameter::new(b"o0", 8));
    core.output.push(InjectParameter::new(b"o1", 8));
    core.order_parameters();
    let mut ctx = InjectContext::default();
    let layout = SnippetLayout::build(&core, &mut ctx, &arch);
    assert_eq!(layout.output_list, vec![0x10, 0x30]);
    assert_eq!(layout.uniq_reserve, 0x50);
    assert_eq!(ctx.output[0].offset, 0x10);
    assert_eq!(ctx.output[1].offset, 0x30);
}

// ---------------------------------------------------------------------------
// F-target 5: getPayloadId namespace routing + reverse-vector boundaries
// ---------------------------------------------------------------------------

/// pcodeinject.cc:285-340 — the four namespaces are independent maps; a name in
/// one is invisible to another type; the default branch (any non-CALLFIXUP /
/// non-CALLOTHER / non-CALLMECH type) routes to scriptMap. Reverse vectors are
/// padded to the injectid and return "" out of range / for negatives.
#[test]
fn get_payload_id_routes_by_type_and_default_is_script() {
    let mut lib = PcodeInjectLibraryBase::new(0x1000);
    lib.register_call_fixup(b"f", 10).unwrap();
    lib.register_call_other_fixup(b"f", 11).unwrap(); // same NAME, different namespace
    lib.register_call_mechanism(b"f", 12).unwrap();
    lib.register_exe_script(b"f", 13).unwrap();

    // Each type sees only its own "f".
    assert_eq!(lib.get_payload_id(CALLFIXUP_TYPE, b"f"), 10);
    assert_eq!(lib.get_payload_id(CALLOTHERFIXUP_TYPE, b"f"), 11);
    assert_eq!(lib.get_payload_id(CALLMECHANISM_TYPE, b"f"), 12);
    assert_eq!(lib.get_payload_id(EXECUTABLEPCODE_TYPE, b"f"), 13);
    // A novel type value (e.g. 99) hits the `else` -> scriptMap (C++ default).
    assert_eq!(lib.get_payload_id(99, b"f"), 13);
    // Missing name -> -1.
    assert_eq!(lib.get_payload_id(CALLFIXUP_TYPE, b"ghost"), -1);

    // reverse-vector padding to the injectid; holes are "".
    assert_eq!(lib.get_call_fixup_name(10), b"f");
    assert_eq!(lib.get_call_fixup_name(0), b"");
    assert_eq!(lib.get_call_fixup_name(9), b"");
    // out of range and negative -> "".
    assert_eq!(lib.get_call_fixup_name(10_000), b"");
    assert_eq!(lib.get_call_fixup_name(-5), b"");
    assert_eq!(lib.get_call_other_target(-1), b"");
    assert_eq!(lib.get_call_mechanism_name(-1), b"");
}

// ---------------------------------------------------------------------------
// F-target 6: manual_call_other_fixup parameter push order
// ---------------------------------------------------------------------------

/// inject_sleigh.cc:504-518 — inputs are pushed in the given order, then the
/// (non-empty) output, then `orderParameters` re-indexes. The parsestring is the
/// snippet; the payload is registered under CALLOTHERFIXUP. Two inputs + one
/// output => indices 0,1 (inputs) and 2 (output).
#[test]
fn manual_call_other_fixup_pushes_inputs_then_output_in_order() {
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let id = lib
        .manual_call_other_fixup(b"myop", b"out", &[b"x".to_vec(), b"y".to_vec()], b"out = x + y;")
        .unwrap();
    let payload = lib.get_payload(id);
    let core = payload.core();
    assert_eq!(core.get_type(), CALLOTHERFIXUP_TYPE);
    assert_eq!(core.size_input(), 2);
    assert_eq!(core.size_output(), 1);
    // order preserved: x then y.
    assert_eq!(core.get_input(0).get_name(), b"x");
    assert_eq!(core.get_input(1).get_name(), b"y");
    assert_eq!(core.get_output(0).get_name(), b"out");
    // indices: inputs 0,1 ; output 2.
    assert_eq!(core.get_input(0).get_index(), 0);
    assert_eq!(core.get_input(1).get_index(), 1);
    assert_eq!(core.get_output(0).get_index(), 2);
    // C++ FIDELITY: manualCallOtherFixup (inject_sleigh.cc:504-518) never sets
    // the payload's `name`, so allocateInject's InjectPayloadCallother default
    // name "unknown" is what registerInject/registerCallOtherFixup record — NOT
    // the "myop" argument (the userop-name binding happens separately, in
    // UserOpManage::manualCallOtherFixup). The Rust port reproduces this: the
    // callother target name for this id is "unknown", and "myop" is unbound.
    assert_eq!(core.get_name(), b"unknown");
    assert_eq!(lib.base.get_call_other_target(id), b"unknown");
    assert_eq!(lib.base.get_payload_id(CALLOTHERFIXUP_TYPE, b"unknown"), id);
    assert_eq!(lib.base.get_payload_id(CALLOTHERFIXUP_TYPE, b"myop"), -1);
}

/// Empty output name => NO output parameter pushed (the C++
/// `if (outname.size()!=0)` guard). Inputs still indexed 0..n.
#[test]
fn manual_call_other_fixup_empty_output_pushes_no_output() {
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let id = lib.manual_call_other_fixup(b"voidop", b"", &[b"a".to_vec()], b"a = a;").unwrap();
    let core = lib.get_payload(id).core();
    assert_eq!(core.size_input(), 1);
    assert_eq!(core.size_output(), 0);
    assert_eq!(core.get_input(0).get_index(), 0);
}
