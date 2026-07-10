//! Porter in-module tests for `pcodeinject.rs`.
//!
//! Cover the engine-neutral injection framework: parameter ordering, the
//! `<input>`/`<output>` decode, payload-attribute parsing, and the
//! `PcodeInjectLibrary` namespace registration/lookup — checked against the C++
//! semantics (`pcodeinject.cc`).

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, UniqueSpace};

use super::*;

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
    register_ids(&mut reg);
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

// orderParameters: inputs first (0..n), then outputs (n..n+m).
#[test]
fn order_parameters_assigns_input_then_output_indices() {
    let mut core = InjectPayloadCore::new(b"p", CALLOTHERFIXUP_TYPE);
    core.inputlist.push(InjectParameter::new(b"a", 4));
    core.inputlist.push(InjectParameter::new(b"b", 2));
    core.output.push(InjectParameter::new(b"out", 8));
    core.order_parameters();
    assert_eq!(core.get_input(0).get_index(), 0);
    assert_eq!(core.get_input(1).get_index(), 1);
    assert_eq!(core.get_output(0).get_index(), 2);
    assert_eq!(core.size_input(), 2);
    assert_eq!(core.size_output(), 1);
}

// decode_payload_params over a real <pcode> body's <input>/<output> children.
#[test]
fn decode_payload_params_from_pcode_children() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<pcode>
      <input name="base" size="2"/>
      <input name="inner" size="2"/>
      <output name="res" size="2"/>
      <body><![CDATA[ res = base + inner; ]]></body>
    </pcode>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element_id(&ELEM_PCODE).unwrap();
    let mut core = InjectPayloadCore::new(b"seg_pcode", EXECUTABLEPCODE_TYPE);
    core.decode_payload_attributes(&mut dec).unwrap();
    core.decode_payload_params(&mut dec).unwrap();
    assert_eq!(core.size_input(), 2);
    assert_eq!(core.size_output(), 1);
    assert_eq!(core.get_input(0).get_name(), b"base");
    assert_eq!(core.get_input(0).get_size(), 2);
    assert_eq!(core.get_input(1).get_name(), b"inner");
    assert_eq!(core.get_output(0).get_name(), b"res");
    // indices: inputs 0,1 then output 2
    assert_eq!(core.get_output(0).get_index(), 2);
    // body is the next (non-input/output) element; leave it for the caller.
    assert_eq!(dec.peek_element().unwrap(), ELEM_BODY);
    dec.close_element(elem).unwrap();
}

// decodePayloadAttributes: paramshift + incidentalcopy + dynamic + inject suffix.
#[test]
fn decode_payload_attributes_reads_flags_and_inject_suffix() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<pcode paramshift="2" incidentalcopy="true" inject="uponentry"></pcode>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element_id(&ELEM_PCODE).unwrap();
    let mut core = InjectPayloadCore::new(b"hook", CALLMECHANISM_TYPE);
    core.decode_payload_attributes(&mut dec).unwrap();
    assert_eq!(core.get_param_shift(), 2);
    assert!(core.is_incidental_copy());
    // inject="uponentry" appends the marker to the name.
    assert_eq!(core.get_name(), b"hook@@inject_uponentry");
    dec.close_element(elem).unwrap();
}

// decodeParameter errors with the C++ message when name is missing.
#[test]
fn decode_parameter_missing_name_errors() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<input size="4"/>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let err = InjectPayloadCore::decode_parameter(&mut dec).unwrap_err();
    assert!(err.to_string().contains("Missing inject parameter name"));
}

// PcodeInjectLibrary: each namespace is independent; getPayloadId routes by type.
#[test]
fn library_namespaces_register_and_lookup_independently() {
    let mut lib = PcodeInjectLibraryBase::new(0x1000);
    lib.register_call_fixup(b"memcpy", 0).unwrap();
    lib.register_call_other_fixup(b"setISAMode", 1).unwrap();
    lib.register_call_mechanism(b"vararg", 2).unwrap();
    lib.register_exe_script(b"switch_index2addr", 3).unwrap();

    assert_eq!(lib.get_payload_id(CALLFIXUP_TYPE, b"memcpy"), 0);
    assert_eq!(lib.get_payload_id(CALLOTHERFIXUP_TYPE, b"setISAMode"), 1);
    assert_eq!(lib.get_payload_id(CALLMECHANISM_TYPE, b"vararg"), 2);
    assert_eq!(lib.get_payload_id(EXECUTABLEPCODE_TYPE, b"switch_index2addr"), 3);

    // Cross-namespace lookups miss (return -1) — the C++ separate-map semantics.
    assert_eq!(lib.get_payload_id(CALLFIXUP_TYPE, b"setISAMode"), -1);
    assert_eq!(lib.get_payload_id(CALLOTHERFIXUP_TYPE, b"memcpy"), -1);

    // id->name reverse lookups
    assert_eq!(lib.get_call_fixup_name(0), b"memcpy");
    assert_eq!(lib.get_call_other_target(1), b"setISAMode");
    assert_eq!(lib.get_call_mechanism_name(2), b"vararg");
    // out-of-range / negative -> empty
    assert_eq!(lib.get_call_fixup_name(99), b"");
    assert_eq!(lib.get_call_fixup_name(-1), b"");
}

// Duplicate registration in a namespace throws the C++ message.
#[test]
fn library_duplicate_registration_errors() {
    let mut lib = PcodeInjectLibraryBase::new(0);
    lib.register_call_fixup(b"dup", 0).unwrap();
    let err = lib.register_call_fixup(b"dup", 1).unwrap_err();
    assert!(err.to_string().contains("Duplicate <callfixup>: dup"));

    lib.register_call_other_fixup(b"co", 0).unwrap();
    let err = lib.register_call_other_fixup(b"co", 1).unwrap_err();
    assert!(err.to_string().contains("Duplicate <callotherfixup>: co"));
}

// id->name vectors are sparse-filled to the injectid (the C++ while-push loop):
// a name registered at a high injectid leaves empty strings before it.
#[test]
fn library_reverse_vector_is_padded_to_injectid() {
    let mut lib = PcodeInjectLibraryBase::new(0);
    lib.register_call_fixup(b"late", 5).unwrap();
    assert_eq!(lib.get_call_fixup_name(5), b"late");
    assert_eq!(lib.get_call_fixup_name(0), b"");
    assert_eq!(lib.get_call_fixup_name(4), b"");
}

// SnippetLayout reserves 0x10 + k*0x20 in unique space, filling context storage.
#[test]
fn snippet_layout_reserves_unique_slots() {
    struct Arch {
        uniq: Rc<AddrSpace>,
        code: Rc<AddrSpace>,
    }
    impl InjectArchitecture for Arch {
        fn get_default_code_space(&self) -> Rc<AddrSpace> {
            Rc::clone(&self.code)
        }
        fn get_unique_space(&self) -> Rc<AddrSpace> {
            Rc::clone(&self.uniq)
        }
    }
    let uniq: Rc<AddrSpace> = Rc::new(UniqueSpace::new(2, 0, false));
    let code = Rc::new(AddrSpace::new(
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
    let arch = Arch { uniq: Rc::clone(&uniq), code };

    let mut core = InjectPayloadCore::new(b"s", EXECUTABLEPCODE_TYPE);
    core.inputlist.push(InjectParameter::new(b"a", 4));
    core.inputlist.push(InjectParameter::new(b"b", 2));
    core.output.push(InjectParameter::new(b"r", 4));
    core.order_parameters();

    let mut ctx = InjectContext::default();
    let layout = SnippetLayout::build(&core, &mut ctx, &arch);

    assert_eq!(layout.input_list, vec![0x10, 0x30]);
    assert_eq!(layout.output_list, vec![0x50]);
    assert_eq!(layout.uniq_reserve, 0x70);
    assert_eq!(ctx.inputlist.len(), 2);
    assert_eq!(ctx.inputlist[0].offset, 0x10);
    assert_eq!(ctx.inputlist[0].size, 4);
    assert_eq!(ctx.inputlist[1].offset, 0x30);
    assert_eq!(ctx.output[0].offset, 0x50);
    assert!(Rc::ptr_eq(ctx.inputlist[0].space.as_ref().unwrap(), &uniq));
}
