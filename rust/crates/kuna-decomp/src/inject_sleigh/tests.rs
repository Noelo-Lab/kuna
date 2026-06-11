//! Porter in-module tests for `inject_sleigh.rs`.
//!
//! Cover the SLEIGH payload decode (`<callfixup>`/`<callotherfixup>`/`<pcode>`/
//! `<case_pcode>`) against real cspec/pspec fixtures, the
//! `PcodeInjectLibrarySleigh` allocate/register namespace parity, the
//! `manualCallOtherFixup` parameter ordering, and a `parse_inject` smoke test
//! over a synthetic SLEIGH language.

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_sleigh::pcodeparse::{SnippetLanguage, SnippetSymbol};
use kuna_sleigh::slghsymbol::{SleighSymbol, SymbolKind, UserOpSymbol, VarnodeSymbol};

use super::*;
use crate::pcodeinject;
use crate::pcodeinject::{
    CALLFIXUP_TYPE, CALLMECHANISM_TYPE, CALLOTHERFIXUP_TYPE, ELEM_PCODE, EXECUTABLEPCODE_TYPE,
};

fn registry() -> IdRegistry {
    let mut reg = IdRegistry::with_base_ids();
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

// --- InjectPayloadCallfixup::decode ---------------------------------------

#[test]
fn decode_callfixup_reads_name_targets_and_pcode() {
    // The x86 / ARM <callfixup name=... ><target.../><pcode>...</pcode> shape.
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callfixup name="switch8_r3">
    <target name="switch8_r3"/>
    <target name="__ARM_common_switch8"/>
    <pcode>
      <input name="r3" size="4"/>
      <body><![CDATA[ r3 = r3 + 1; ]]></body>
    </pcode>
  </callfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = InjectPayloadCallfixup::new(b"x86win.cspec");
    p.decode(&mut dec).unwrap();

    assert_eq!(p.payload.core.get_name(), b"switch8_r3");
    assert_eq!(p.target_symbol_names.len(), 2);
    assert_eq!(p.target_symbol_names[0], b"switch8_r3");
    assert_eq!(p.target_symbol_names[1], b"__ARM_common_switch8");
    assert_eq!(p.payload.core.size_input(), 1);
    assert_eq!(p.payload.core.get_type(), CALLFIXUP_TYPE);
    assert_eq!(p.payload.parsestring, b" r3 = r3 + 1; ");
}

#[test]
fn decode_callfixup_missing_pcode_errors() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callfixup name="nope"><target name="t"/></callfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = InjectPayloadCallfixup::new(b"src");
    let err = p.decode(&mut dec).unwrap_err();
    assert!(err.to_string().contains("<callfixup> is missing <pcode> subtag: nope"));
}

// --- InjectPayloadCallother::decode ---------------------------------------

#[test]
fn decode_callotherfixup_reads_targetop_and_pcode() {
    // The ARM_apcs <callotherfixup targetop="setISAMode"> shape.
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callotherfixup targetop="setISAMode">
    <pcode incidentalcopy="true">
      <input name="newTB"/>
      <body><![CDATA[ r0 = r0; ]]></body>
    </pcode>
  </callotherfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = InjectPayloadCallother::new(b"ARM_apcs.cspec");
    p.decode(&mut dec).unwrap();

    assert_eq!(p.payload.core.get_name(), b"setISAMode");
    assert_eq!(p.payload.core.get_type(), CALLOTHERFIXUP_TYPE);
    assert_eq!(p.payload.core.size_input(), 1);
    assert!(p.payload.core.is_incidental_copy());
    assert_eq!(p.payload.core.get_input(0).get_name(), b"newTB");
    assert_eq!(p.payload.parsestring, b" r0 = r0; ");
}

#[test]
fn decode_callotherfixup_without_pcode_errors() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callotherfixup targetop="x"><target name="t"/></callotherfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = InjectPayloadCallother::new(b"src");
    let err = p.decode(&mut dec).unwrap_err();
    assert!(err.to_string().contains("<callotherfixup> does not contain a <pcode> tag"));
}

// --- ExecutablePcodeSleigh::decode ----------------------------------------

#[test]
fn decode_executable_case_pcode() {
    // The Dalvik <case_pcode> body shape (a non-<pcode> executable element).
    let reg = registry();
    let mgr = manager();
    let xml = br#"<case_pcode>
      <input name="index" size="4"/>
      <input name="table" size="4"/>
      <output name="finaladdr" size="4"/>
      <body><![CDATA[ finaladdr = table + index; ]]></body>
    </case_pcode>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = ExecutablePcodeSleigh::new(b"cspec", b"sa_index2case");
    p.decode(&mut dec).unwrap();

    assert_eq!(p.core.get_type(), EXECUTABLEPCODE_TYPE);
    assert_eq!(p.core.size_input(), 2);
    assert_eq!(p.core.size_output(), 1);
    // indices: inputs 0,1 then output 2.
    assert_eq!(p.core.get_output(0).get_index(), 2);
    assert_eq!(p.parsestring, b" finaladdr = table + index; ");
}

#[test]
fn decode_executable_rejects_unexpected_element() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callfixup name="x"></callfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = ExecutablePcodeSleigh::new(b"src", b"nm");
    let err = p.decode(&mut dec).unwrap_err();
    assert!(err
        .to_string()
        .contains("Expecting <pcode>, <case_pcode>, <addr_pcode>, <default_pcode>, or <size_pcode>"));
}

// --- PcodeInjectLibrarySleigh allocate/register/decode --------------------

#[test]
fn library_allocate_routes_by_type() {
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let cf = lib.allocate_inject(b"src", b"cf", CALLFIXUP_TYPE);
    let co = lib.allocate_inject(b"src", b"co", CALLOTHERFIXUP_TYPE);
    let ex = lib.allocate_inject(b"src", b"ex", EXECUTABLEPCODE_TYPE);
    let cm = lib.allocate_inject(b"src", b"cm", CALLMECHANISM_TYPE);
    assert_eq!((cf, co, ex, cm), (0, 1, 2, 3));
    assert!(matches!(lib.injection[0], SleighPayload::Callfixup(_)));
    assert!(matches!(lib.injection[1], SleighPayload::Callother(_)));
    assert!(matches!(lib.injection[2], SleighPayload::Executable(_)));
    assert!(matches!(lib.injection[3], SleighPayload::Sleigh(_)));
}

#[test]
fn library_decode_inject_registers_in_namespace() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callotherfixup targetop="setISAMode">
    <pcode><input name="a"/><body><![CDATA[ a = a; ]]></body></pcode>
  </callotherfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let id = lib.decode_inject(b"src", b"", CALLOTHERFIXUP_TYPE, &mut dec).unwrap();
    assert_eq!(id, 0);
    // registered under the targetop name in the callother namespace.
    assert_eq!(lib.base.get_payload_id(CALLOTHERFIXUP_TYPE, b"setISAMode"), 0);
    assert_eq!(lib.base.get_call_other_target(0), b"setISAMode");
}

#[test]
fn manual_call_other_fixup_orders_params_in_then_out() {
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let id = lib
        .manual_call_other_fixup(b"myop", b"out", &[b"a".to_vec(), b"b".to_vec()], b"out = a + b;")
        .unwrap();
    assert_eq!(id, 0);
    let payload = lib.get_payload(id);
    assert_eq!(payload.core().size_input(), 2);
    assert_eq!(payload.core().size_output(), 1);
    assert_eq!(payload.core().get_input(0).get_name(), b"a");
    assert_eq!(payload.core().get_input(0).get_index(), 0);
    assert_eq!(payload.core().get_input(1).get_index(), 1);
    assert_eq!(payload.core().get_output(0).get_name(), b"out");
    assert_eq!(payload.core().get_output(0).get_index(), 2);
    // FAITHFUL C++ QUIRK: `manualCallOtherFixup` constructs the payload via
    // `InjectPayloadCallother(sourceName)`, whose ctor hard-codes the payload
    // name to "unknown" (the `name` argument is used only for the sourceName
    // string, not the payload name). `registerInject` then registers the
    // payload under "unknown", so the callother-target reverse map says
    // "unknown" — not "myop". The userop caller does not consult this map (it
    // uses the returned injectid directly), so the quirk is harmless. We pin it
    // here to prove the port matches C++ `manualCallOtherFixup` exactly.
    assert_eq!(lib.base.get_call_other_target(0), b"unknown");
}

#[test]
fn manual_call_other_fixup_no_output_is_optional() {
    let mut lib = PcodeInjectLibrarySleigh::new(0x1000);
    let id = lib.manual_call_other_fixup(b"voidop", b"", &[b"a".to_vec()], b"a = a;").unwrap();
    let payload = lib.get_payload(id);
    assert_eq!(payload.core().size_input(), 1);
    // empty outname => no output parameter.
    assert_eq!(payload.core().size_output(), 0);
}

// --- parse_inject smoke test over a synthetic language --------------------

struct TestLang {
    manager: AddrSpaceManager,
    ram: Rc<AddrSpace>,
}

impl TestLang {
    fn new() -> TestLang {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(2, 0, false))).unwrap();
        TestLang { manager, ram }
    }
}

impl SnippetLanguage for TestLang {
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<SnippetSymbol> {
        // a single register r0 at ram:0, size 4
        if name == b"r0" {
            let sym = SleighSymbol::new_varnode(b"r0", Rc::clone(&self.ram), 0, 4);
            if let SymbolKind::Varnode(v) = sym.kind() {
                return Some(SnippetSymbol::Varnode(v.clone(), name.to_vec()));
            }
        }
        let _ = UserOpSymbol::default(); // keep the import live for clarity
        None
    }
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.ram)
    }
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager.get_constant_space().unwrap())
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager.get_space_by_name("unique").unwrap())
    }
    fn num_spaces(&self) -> i32 {
        self.manager.num_spaces()
    }
    fn get_space(&self, i: i32) -> Option<Rc<AddrSpace>> {
        self.manager.get_space(i).cloned()
    }
}

// keep VarnodeSymbol import meaningful (the SnippetSymbol::Varnode payload type)
#[allow(dead_code)]
fn _assert_varnode_symbol_type(_: VarnodeSymbol) {}

#[test]
fn parse_inject_compiles_a_callother_snippet() {
    // Decode a callotherfixup whose body assigns a temporary, then compile it
    // through parse_inject (the public-API PcodeSnippet path).
    let reg = registry();
    let mgr = manager();
    let xml = br#"<callotherfixup targetop="noop">
    <pcode>
      <input name="a" size="4"/>
      <body><![CDATA[ a = a + 1; ]]></body>
    </pcode>
  </callotherfixup>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let mut p = InjectPayloadCallother::new(b"src");
    p.decode(&mut dec).unwrap();

    let lang = TestLang::new();
    let (tpl, new_tempbase) =
        parse_inject(&lang, &p, &p.payload.parsestring, 0x1000).expect("snippet should compile");
    // The template has at least one op (the INT_ADD + COPY).
    assert!(!tpl.get_opvec().is_empty());
    // For a non-executable payload the tempbase advances (>= the input).
    assert!(new_tempbase >= 0x1000);
}

#[test]
fn decode_inject_debug_entry_reads_name_and_type() {
    let reg = registry();
    let mgr = manager();
    let xml = br#"<inject name="setISAMode" type="2"/>"#;
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(xml).unwrap();
    let elem = dec.open_element_id(&pcodeinject::ELEM_INJECT).unwrap();
    let (name, ptype) = decode_inject_debug_entry(&mut dec).unwrap();
    assert_eq!(name, b"setISAMode");
    assert_eq!(ptype, CALLOTHERFIXUP_TYPE);
    dec.close_element(elem).unwrap();
}

// silence the unused-import lint for ELEM_PCODE (used only in module code via
// the decode paths; referenced here to keep the import meaningful in tests).
#[allow(dead_code)]
const _: u32 = ELEM_PCODE.get_id();
