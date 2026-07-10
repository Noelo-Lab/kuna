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

// ===========================================================================
// Injection emission: a real callfixup from specs/ emits ops into a Funcdata.
// ===========================================================================

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::funcdata::Funcdata;
use crate::pcodeinject::InjectContext;
use crate::context::{ArchContext, TypeOp};

/// A `SnippetLanguage` with a code space `ram`, the const/unique spaces, and a
/// couple of x86-style registers (EAX/ESP).  Enough to compile the simple real
/// x86 callfixups (`mcount`, `get_pc_thunk_ax`).
struct CodeLang {
    manager: AddrSpaceManager,
    ram: Rc<AddrSpace>,
}

impl CodeLang {
    fn new() -> CodeLang {
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
        CodeLang { manager, ram }
    }
}

impl SnippetLanguage for CodeLang {
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<SnippetSymbol> {
        // x86 registers used by get_pc_thunk_ax / mcount.  EAX at ram:0, ESP at
        // ram:0x10, each 4 bytes.  (A faithful register would live in a register
        // space, but for snippet resolution the (space,offset,size) is what the
        // builder consumes; ram suffices for the emit test.)
        let reg = |off: u64| {
            let sym = SleighSymbol::new_varnode(name, Rc::clone(&self.ram), off, 4);
            if let SymbolKind::Varnode(v) = sym.kind() {
                Some(SnippetSymbol::Varnode(v.clone(), name.to_vec()))
            } else {
                None
            }
        };
        match name {
            b"EAX" => reg(0),
            b"ESP" => reg(0x10),
            _ => None,
        }
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

/// A `PcodeEmit` that records each dumped op as `(opcode, has_out, isize)`.
#[derive(Default)]
struct CollectEmitter {
    ops: Vec<(OpCode, bool, usize)>,
}
impl PcodeEmit for CollectEmitter {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.is_some(), vars.len()));
    }
}

/// A `PcodeEmit` that actually builds the emitted ops into a [`Funcdata`],
/// resolving each `VarnodeData` to a real `Varnode` (constants via
/// `new_constant`, everything else via `new_varnode` at its address).
struct FuncdataEmitter<'a> {
    fd: &'a mut Funcdata,
    const_space: Rc<AddrSpace>,
    /// Count of ops created, for assertions.
    created: usize,
    /// Count of ops whose output was successfully linked via the now-real
    /// `op_set_output` (the `banks_mut` split-borrow `setDef` path landed by
    /// `w4x-flow-linkage`).  Inputs, the op, AND the output are all built — an
    /// emitted op with an output increments this once the output `setDef`
    /// succeeds.
    outputs_linked: usize,
}
impl FuncdataEmitter<'_> {
    fn resolve(&mut self, vn: &VarnodeData) -> crate::context::VarnodeId {
        let space = vn.space.clone().expect("emitted varnode has a space");
        if Rc::ptr_eq(&space, &self.const_space) {
            self.fd.new_constant(vn.size as i32, vn.offset)
        } else {
            self.fd.new_varnode(vn.size as i32, &Address::new(space, vn.offset), None)
        }
    }
}
impl PcodeEmit for FuncdataEmitter<'_> {
    fn dump(&mut self, addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        let op = self.fd.new_op(vars.len() as i32, addr.clone());
        self.fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        for (slot, v) in vars.iter().enumerate() {
            let vn = self.resolve(v);
            self.fd.op_set_input(op, vn, slot as i32).expect("op_set_input builds the input link");
        }
        if let Some(o) = outvar {
            let vn = self.resolve(o);
            // The `op_set_output` setDef step is now the real `banks_mut`
            // split-borrow path (landed by `w4x-flow-linkage`); it links the
            // output Varnode as the op's def.
            self.fd.op_set_output(op, vn).expect("op_set_output links the output");
            self.outputs_linked += 1;
        }
        self.created += 1;
    }
}

/// Build a `Funcdata` whose IR-boundary glb carries const/unique/iop/fspec
/// spaces (the standard W3 seam manager), entry at ram:0x1000.
fn emit_funcdata() -> (Funcdata, Rc<AddrSpace>) {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let const_space = Rc::clone(m.get_constant_space().unwrap());
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let entry = Address::new(Rc::clone(&ram), 0x1000);
    let fd = Funcdata::new("inj", "inj", glb, entry, 0x10000000, 0x40).unwrap();
    (fd, const_space)
}

/// Decode the named `<callfixup>` out of an x86 cspec, returning the payload.
fn decode_real_callfixup(name: &str) -> InjectPayloadCallfixup {
    use std::path::PathBuf;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let cspec = root.join("specs/Ghidra/Processors/x86/data/languages/x86gcc.cspec");
    let text = std::fs::read_to_string(&cspec).expect("read x86gcc.cspec");
    // Slice out the <callfixup name="..."> ... </callfixup> block.
    let needle = format!("<callfixup name=\"{name}\">");
    let start = text.find(&needle).unwrap_or_else(|| panic!("callfixup {name} present"));
    let end = text[start..].find("</callfixup>").expect("callfixup close") + start + "</callfixup>".len();
    let block = &text[start..end];

    let reg = registry();
    let mgr = manager();
    let mut dec = XmlDecode::new(&mgr, &reg);
    dec.ingest_stream(block.as_bytes()).expect("ingest callfixup");
    let mut p = InjectPayloadCallfixup::new(b"x86gcc.cspec");
    p.decode(&mut dec).expect("decode callfixup");
    p
}

#[test]
fn real_mcount_callfixup_emits_copy_into_funcdata() {
    // The real x86 `mcount` callfixup body is `temp:1 = 0;` — a single COPY of a
    // zero constant into a 1-byte unique temp.  No parameters, no registers.
    let p = decode_real_callfixup("mcount");
    assert_eq!(p.payload.core.get_name(), b"mcount");
    assert!(p.payload.core.inputlist.is_empty());

    let lang = CodeLang::new();
    let (tpl, _new_base) =
        parse_inject(&lang, &p, &p.payload.parsestring, 0x1000).expect("compile mcount snippet");
    assert!(!tpl.get_opvec().is_empty(), "mcount compiles to at least one op");

    let engine = SleighInjectEngine::new(
        lang.get_constant_space(),
        lang.get_unique_space(),
        lang.get_default_code_space(),
    );

    let (mut fd, const_space) = emit_funcdata();
    let mut context = InjectContext {
        baseaddr: Some(fd.get_address().clone()),
        nextaddr: Some(fd.get_address().clone()),
        ..Default::default()
    };
    let n_before = fd.obank().iter_all().count();
    let outputs_linked;
    {
        let mut emitter = FuncdataEmitter { fd: &mut fd, const_space, created: 0, outputs_linked: 0 };
        engine
            .emit_payload(&p, &tpl, &mut context, &mut emitter)
            .expect("emit mcount payload");
        assert!(emitter.created >= 1, "at least one op emitted");
        outputs_linked = emitter.outputs_linked;
    }
    // The Funcdata now holds the COPY op (the only op of `temp:1 = 0`).
    let n_after = fd.obank().iter_all().count();
    assert!(n_after > n_before, "ops were created in the funcdata");
    // The COPY has an output, so output-wiring went through the now-real
    // `op_set_output` setDef path exactly once (the op, its const input, AND the
    // output ARE built).
    assert_eq!(outputs_linked, 1, "the COPY's output was linked via op_set_output");
    // Stronger than the old seam check: the COPY op now actually has its output
    // Varnode linked (the real setDef ran), so the op's output is the 1-byte
    // unique temp and that Varnode is defined by the COPY.
    let copy_out = fd
        .obank()
        .iter_all()
        .find_map(|(_, op)| {
            let o = fd.obank().get(op).unwrap();
            if o.code() == OpCode::CPUI_COPY {
                Some((op, o.get_out()))
            } else {
                None
            }
        })
        .expect("the COPY op exists");
    let (copy_op, out) = copy_out;
    let out = out.expect("the COPY's output Varnode is linked");
    let outvn = fd.vbank().get(out).expect("output varnode live");
    assert_eq!(outvn.get_size(), 1, "output is the 1-byte unique temp");
    assert_eq!(outvn.get_def(), Some(copy_op), "output is defined by the COPY");
}

#[test]
fn real_mcount_callfixup_emit_ops_are_a_copy_of_zero() {
    // Same payload, but capture the raw emitted ops to assert their shape: one
    // COPY with a 1-byte unique output and a single constant-0 input.
    let p = decode_real_callfixup("mcount");
    let lang = CodeLang::new();
    let (tpl, _) = parse_inject(&lang, &p, &p.payload.parsestring, 0x1000).unwrap();
    let engine = SleighInjectEngine::new(
        lang.get_constant_space(),
        lang.get_unique_space(),
        lang.get_default_code_space(),
    );
    let base = Address::new(lang.get_default_code_space(), 0x2000);
    let mut context = InjectContext { baseaddr: Some(base.clone()), nextaddr: Some(base), ..Default::default() };
    let mut emitter = CollectEmitter::default();
    engine.emit_payload(&p, &tpl, &mut context, &mut emitter).unwrap();
    assert_eq!(emitter.ops.len(), 1, "one op for `temp:1 = 0`");
    let (opc, has_out, isize) = emitter.ops[0];
    assert_eq!(opc, OpCode::CPUI_COPY);
    assert!(has_out, "COPY writes the temp");
    assert_eq!(isize, 1, "COPY of a single constant");
}

#[test]
fn real_get_pc_thunk_emits_load_and_add() {
    // The real x86 `get_pc_thunk_ax` callfixup is:
    //   EAX = * ESP;      -> a LOAD of [ESP] into EAX
    //   ESP = ESP + 4;    -> an INT_ADD of ESP and constant 4
    let p = decode_real_callfixup("get_pc_thunk_ax");
    let lang = CodeLang::new();
    let (tpl, _) = parse_inject(&lang, &p, &p.payload.parsestring, 0x1000)
        .expect("compile get_pc_thunk_ax snippet");
    let engine = SleighInjectEngine::new(
        lang.get_constant_space(),
        lang.get_unique_space(),
        lang.get_default_code_space(),
    );
    let base = Address::new(lang.get_default_code_space(), 0x3000);
    let mut context = InjectContext { baseaddr: Some(base.clone()), nextaddr: Some(base), ..Default::default() };
    let mut emitter = CollectEmitter::default();
    engine.emit_payload(&p, &tpl, &mut context, &mut emitter).unwrap();
    // The emitted ops contain a LOAD (EAX = *ESP) and an INT_ADD (ESP+4).
    let opcodes: Vec<OpCode> = emitter.ops.iter().map(|o| o.0).collect();
    assert!(opcodes.contains(&OpCode::CPUI_LOAD), "EAX = *ESP emits a LOAD, got {opcodes:?}");
    assert!(opcodes.contains(&OpCode::CPUI_INT_ADD), "ESP = ESP+4 emits INT_ADD, got {opcodes:?}");
}

#[test]
fn setup_handles_rejects_parameter_count_mismatch() {
    // A callotherfixup with one input parameter, but an InjectContext with no
    // input storage -> checkParameterRestrictions error (setup_handles).
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
    let lang = CodeLang::new();
    let (tpl, _) = parse_inject(&lang, &p, &p.payload.parsestring, 0x1000).unwrap();
    let engine = SleighInjectEngine::new(
        lang.get_constant_space(),
        lang.get_unique_space(),
        lang.get_default_code_space(),
    );
    let base = Address::new(lang.get_default_code_space(), 0x4000);
    // Empty context (no input storage) -> mismatch against the 1-input payload.
    let mut context = InjectContext { baseaddr: Some(base.clone()), nextaddr: Some(base), ..Default::default() };
    let mut emitter = CollectEmitter::default();
    let err = engine.emit_payload(&p, &tpl, &mut context, &mut emitter).unwrap_err();
    assert!(format!("{err}").contains("different number of parameters"));
}
