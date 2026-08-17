//! The decompileAt end-to-end proof (phase-2 step 6): drive a FULL ghidra-mode
//! session — registerProgram → setAction("decompile","c") → decompileAt(entry)
//! — for a TINY canned function and prove C comes out.
//!
//! Unlike the pre-canned byte streams of `protocol_e2e.rs`, this drives the LIVE
//! decompiler: `decompileAt` runs `decompile_func`, whose flow-follow + pipeline
//! issue getPcode/getCodeLabel/getRegister/... queries whose exact sequence
//! depends on engine internals.  Pre-canning that sequence would be brittle, so
//! the MockJava here is the **interactive single-threaded loopback** shared
//! machinery in `tests/ghidra_sim/mod.rs` (extracted from this test): the
//! process writes queries into a shared buffer; the mock decodes each query's
//! command element id and generates a response on demand (empty getUserOpName
//! to end the probe, a name for getCodeLabel, a canned `<inst>` RETURN for
//! getPcode at the entry, safe minimal answers for everything else).
//!
//! The real-program twin of this test — the same machinery answering from
//! kuna's own analysis of a vendored ELF — is `ghidra_sim_e2e.rs`.
//!
//! ## What this covers
//!   * The whole wire lifecycle end to end over the real command loop.
//!   * `decompileAt` names the function from `getCodeLabel`, builds+decompiles a
//!     `Funcdata` for the raw entry (flow-discovered extent), and emits a
//!     well-formed `<doc>` containing the `Funcdata::encode` `<function>`/`<ast>`.
//!   * The dual `<function>` (the Clang markup) is present when C is requested,
//!     decodes, and its `opref`/`varref` tokens are a subset of the `<ast>`'s op
//!     times / varnode create-indices — the click-to-address contract, end to end.
//!
//! ## What this stubs / canned
//!   * The decompiled function is a synthetic single-instruction `RETURN #0x0`
//!     (the mock's getPcode answer for the entry; BadData elsewhere) — enough to
//!     produce a non-empty AST without needing a real processor `.sla`.
//!   * getRegister returns a fixed register storage, getRegisterName/getComments
//!     return empty, getBytes returns DataUnavail — defensive minimal answers so
//!     the decompile of this register-free function completes; the specs are the
//!     same four tiny valid documents `protocol_e2e.rs` proves build a live arch.
//!   * There is NO symbol scope (Phase 3): unresolved refs degrade to
//!     placeholders, which is the accepted phase-2 quality.

mod ghidra_sim;

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{Decoder, ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_SIZE};
use kuna_base::space::AddrSpaceManager;
use kuna_num::opcodes::{OpCode, OpcodeEncoder};
use kuna_sleigh::translate::{ATTRIB_CODE, ELEM_OP};

use kuna_ghidra::ids::{
    ELEM_COMMAND_GETBYTES, ELEM_COMMAND_GETCODELABEL, ELEM_COMMAND_GETPCODE,
    ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETREGISTERNAME, ELEM_COMMAND_GETUSEROPNAME,
};
use kuna_ghidra::process::GhidraProcess;
use kuna_ghidra::translate::{build_registry, TspecModel};

use ghidra_sim::{
    cmd_decompile_at, cmd_register_program, cmd_set_action, parse_decompile_doc, resp_empty,
    resp_string, AnswerSource, MockReader, MockState, MockWriter,
};

// The four registerProgram documents — the same tiny valid specs
// `protocol_e2e.rs` proves build a live `Architecture` (single init query, empty
// warnings).  `<default_proto/>` is empty (no register storage in the model), so
// the register-free RETURN function needs no register recovery.
const TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x10000000\">\
<spaces defaultspace=\"ram\">\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"8\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
<space name=\"register\" index=\"4\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
</spaces></sleigh>";
const PSPEC: &[u8] = b"<processor_spec><programcounter register=\"PC\"/></processor_spec>";
const CSPEC: &[u8] = b"<compiler_spec><default_proto/></compiler_spec>";
const CORETYPES: &[u8] =
    b"<coretypes><type name=\"int\" size=\"4\" metatype=\"int\" id=\"-1\"/></coretypes>";

const ENTRY: u64 = 0x1000;
const FUNC_NAME: &[u8] = b"myfunc";

/// The canned answer source: fixed constants over the tiny inline tspec (the
/// original MockJava of this test, now behind the shared [`AnswerSource`]).
struct CannedSource {
    /// The tspec address-space manager (decodes the getPcode `<addr>`, encodes
    /// the canned `<inst>` / register responses).
    manager: Rc<AddrSpaceManager>,
}

impl CannedSource {
    /// The canned getPcode answer for the entry: `<inst offset=1>` + the pc
    /// `<addr>` + one `<op>` `RETURN #0x0:8` (output `<void/>`, one constant
    /// input).  A RETURN terminates flow, so this single instruction is the whole
    /// function body.
    fn return_inst_doc(&self) -> Vec<u8> {
        let ram = Rc::clone(self.manager.get_space_by_name("ram").expect("ram"));
        let cst = Rc::clone(self.manager.get_constant_space().expect("constant"));
        let elem_inst = ElementId::new("inst", 98); // ELEM_INST (kuna-decomp)
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&elem_inst);
            e.write_signed_integer(&kuna_base::marshal::ATTRIB_OFFSET, 1); // instr length
            Address::new(Rc::clone(&ram), ENTRY).encode(&mut e).expect("pc addr");
            // <op>: RETURN, isize=1, void output, one constant input.
            e.open_element(&ELEM_OP);
            e.write_signed_integer(&ATTRIB_SIZE, 1); // isize = one input
            e.write_opcode(&ATTRIB_CODE, OpCode::CPUI_RETURN);
            e.open_element(&kuna_base::marshal::ELEM_VOID);
            e.close_element(&kuna_base::marshal::ELEM_VOID);
            Address::new(cst, 0).encode_sized(&mut e, 8).expect("const input");
            e.close_element(&ELEM_OP);
            e.close_element(&elem_inst);
        }
        doc
    }

    /// A fixed register storage `<addr space=register offset=0 size=8>` (the
    /// getRegister answer shape AddressXML.encode sends).
    fn register_addr_doc(&self) -> Vec<u8> {
        let reg = Rc::clone(self.manager.get_space_by_name("register").expect("register"));
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            Address::new(reg, 0).encode_sized(&mut e, 8).expect("register addr");
        }
        doc
    }
}

impl AnswerSource for CannedSource {
    /// Decode a query's command element id and produce its response bytes.
    fn respond(&mut self, doc: &[u8]) -> Vec<u8> {
        let mut dec = PackedDecode::new(&self.manager);
        dec.ingest_stream(doc).expect("query doc ingests");
        let el = dec.open_element().expect("query has a command element");
        if el == ELEM_COMMAND_GETUSEROPNAME.get_id() {
            // Empty name at index 0 terminates the init-time user-op probe.
            resp_string(b"")
        } else if el == ELEM_COMMAND_GETCODELABEL.get_id() {
            // The primary symbol name -> <function name>.
            resp_string(FUNC_NAME)
        } else if el == ELEM_COMMAND_GETPCODE.get_id() {
            let addr = Address::decode(&mut dec).expect("getPcode carries an <addr>");
            if addr.get_offset() == ENTRY {
                resp_string(&self.return_inst_doc())
            } else {
                resp_empty() // BadData: flow should not ask past the RETURN
            }
        } else if el == ELEM_COMMAND_GETREGISTER.get_id() {
            // A fixed valid register storage so a register lookup never errors
            // (the register-free RETURN function should not reach here).
            resp_string(&self.register_addr_doc())
        } else if el == ELEM_COMMAND_GETREGISTERNAME.get_id() {
            resp_string(b"") // no named register -> render by raw location
        } else if el == ELEM_COMMAND_GETBYTES.get_id() {
            resp_empty() // DataUnavail (no loadimage for a register/const-only fn)
        } else {
            // getComments / getMappedSymbols / getTrackedRegisters / getDataType
            // / ... : an empty response (the decompile of this function issues
            // none of them, but answer defensively so nothing desyncs).
            resp_empty()
        }
    }
}

#[test]
fn test_decompile_at_emits_c() {
    // The tspec manager the mock uses to decode/encode <addr> elements.
    let registry = build_registry();
    let model = TspecModel::decode(TSPEC, &registry).expect("tspec parses");
    let manager = Rc::new(model.manager);

    // The packed <addr> for the decompileAt entry (ram:ENTRY).
    let addr_packed = {
        let ram = Rc::clone(manager.get_space_by_name("ram").expect("ram"));
        let mut v = Vec::new();
        {
            let mut e = PackedEncode::new(&mut v);
            Address::new(ram, ENTRY).encode(&mut e).expect("entry addr");
        }
        v
    };

    // The proactive command stream: registerProgram + setAction + decompileAt.
    let mut commands = Vec::new();
    cmd_register_program(&mut commands, PSPEC, CSPEC, TSPEC, CORETYPES);
    cmd_set_action(&mut commands, "0", "decompile", "c");
    cmd_decompile_at(&mut commands, "0", &addr_packed);

    let shared = Rc::new(RefCell::new(MockState::new(
        commands,
        CannedSource { manager: Rc::clone(&manager) },
    )));

    let reader = MockReader { shared: Rc::clone(&shared) };
    let writer = MockWriter { shared: Rc::clone(&shared) };

    let mut process = GhidraProcess::new(reader, writer);
    // Drive the three commands (registerProgram, setAction, decompileAt).
    for cmd in ["registerProgram", "setAction", "decompileAt"] {
        assert_eq!(
            process.read_command().unwrap_or_else(|e| panic!("{cmd}: {e:?}")),
            0,
            "{cmd} should complete without terminating the loop"
        );
    }

    let out = shared.borrow().from_process.clone();
    // Drop the process (releases the session engine's client Rc clones) so the
    // shared state has no live borrower for the assertions below.
    let _ = process.into_inner();

    // Sanity: no engine-construction failure warning leaked into the stream.
    let text = String::from_utf8_lossy(&out);
    assert!(
        !text.contains("could not build the decompiler engine"),
        "engine construction failed: {text}"
    );
    assert!(
        !text.contains("could not decompile the function"),
        "decompileAt degraded to the incomplete-function shape: {text}"
    );

    // Extract the decompileAt <doc> payload (the last command response) and
    // decode it with the shared dual-<function> parser.
    let trace = ghidra_sim::trace_session(&out);
    let doc = trace
        .responses
        .last()
        .and_then(|r| r.payload.clone())
        .expect("decompileAt produced a response payload");
    assert!(!doc.is_empty(), "decompileAt emitted an EMPTY payload (no <doc>)");

    let parsed = parse_decompile_doc(&doc, &manager);

    assert_eq!(
        parsed.name.as_bytes(),
        FUNC_NAME,
        "<function name> must be the getCodeLabel name"
    );
    assert!(
        !parsed.ast_op_times.is_empty(),
        "the <ast> has no ops (empty function body) — expected the RETURN op"
    );
    assert!(
        parsed.has_markup,
        "the C-code markup <function> is missing (send_c_code + decompile action)"
    );

    // The click-to-address contract: every markup token ref resolves against the
    // AST it was emitted alongside.
    assert!(
        parsed.markup_oprefs.is_subset(&parsed.ast_op_times),
        "markup oprefs {:?} are not a subset of the AST op times {:?}",
        parsed.markup_oprefs,
        parsed.ast_op_times
    );
    assert!(
        parsed.markup_varrefs.is_subset(&parsed.ast_var_refs),
        "markup varrefs {:?} are not a subset of the AST varnode refs {:?}",
        parsed.markup_varrefs,
        parsed.ast_var_refs
    );
    // The markup must actually reference the syntax tree (a bare `return;` still
    // tags its statement/op), proving the two documents are cross-linked rather
    // than trivially both empty.
    assert!(
        !parsed.markup_oprefs.is_empty() || !parsed.markup_varrefs.is_empty(),
        "the markup carried NO opref/varref — the dual <function> is not cross-linked to the <ast>"
    );

    // The flattened markup IS the C the GUI renders: it must name the function
    // and carry a body.
    assert!(
        parsed.c_text.contains("myfunc"),
        "flattened markup C does not name the function:\n{}",
        parsed.c_text
    );
    assert!(
        parsed.c_text.contains('{') && parsed.c_text.contains('}'),
        "flattened markup C has no function body:\n{}",
        parsed.c_text
    );
}
