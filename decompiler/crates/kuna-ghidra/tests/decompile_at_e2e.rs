//! The decompileAt end-to-end proof (phase-2 step 6): drive a FULL ghidra-mode
//! session — registerProgram → setAction("decompile","c") → decompileAt(entry)
//! — for a TINY real function and prove C comes out.
//!
//! Unlike the pre-canned byte streams of `protocol_e2e.rs`, this drives the LIVE
//! decompiler: `decompileAt` runs `decompile_func`, whose flow-follow + pipeline
//! issue getPcode/getCodeLabel/getRegister/... queries whose exact sequence
//! depends on engine internals.  Pre-canning that sequence would be brittle, so
//! the MockJava here is an **interactive single-threaded loopback**: the process
//! writes queries into a shared buffer; the mock decodes each query's command
//! element id and generates a response on demand (empty getUserOpName to end the
//! probe, a name for getCodeLabel, a canned `<inst>` RETURN for getPcode at the
//! entry, safe minimal answers for everything else).  This is sound because
//! packed protocol payloads are 0x00-free (protocol.rs:21), so the 4-byte
//! query-open marker `[0,0,1,4]` never occurs inside a payload and query frames
//! are unambiguously delimited.
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

use std::cell::RefCell;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::{
    Decoder, ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_NAME, ATTRIB_SIZE,
};
use kuna_base::space::AddrSpaceManager;
use kuna_num::opcodes::{OpCode, OpcodeEncoder};
use kuna_sleigh::translate::{ATTRIB_CODE, ELEM_OP};

use kuna_ghidra::ids::{
    ELEM_COMMAND_GETBYTES, ELEM_COMMAND_GETCODELABEL, ELEM_COMMAND_GETPCODE,
    ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETREGISTERNAME, ELEM_COMMAND_GETUSEROPNAME, ELEM_DOC,
};
use kuna_ghidra::process::GhidraProcess;
use kuna_ghidra::translate::{build_registry, TspecModel};

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

// Marshaling attribute ids used only for decoding the response (raw numeric ids;
// PackedDecode returns them straight from open_element / get_next_attribute_id).
const ATTRIB_REF_ID: u32 = 18; // Varnode::getCreateIndex back-ref (marshal.rs)
const ATTRIB_UNIQ_ID: u32 = 29; // <seqnum uniq> == PcodeOp::getTime (address.rs)
const ATTRIB_OPREF_ID: u32 = 41; // EmitMarkup opref (prettyprint.rs)
const ATTRIB_VARREF_ID: u32 = 42; // EmitMarkup varref (prettyprint.rs)
const ELEM_FUNCTION_ID: u32 = 116; // <function> (funcdata_encode.rs)

// ---------------------------------------------------------------------------
// Interactive loopback MockJava
// ---------------------------------------------------------------------------

/// State shared between the process's `sin` ([`MockReader`]) and `sout`
/// ([`MockWriter`]): the proactive command stream, the process's output buffer
/// (scanned for query frames), and the response queue fed back to the process.
struct MockState {
    /// The proactive command frames (registerProgram/setAction/decompileAt).
    commands: Vec<u8>,
    /// Cursor into `commands`.
    cmd_cursor: usize,
    /// Everything the process has written (queries + command-response framing).
    from_process: Vec<u8>,
    /// How far `from_process` has been scanned for query frames.
    parsed: usize,
    /// Bytes queued to feed the process back (query responses).
    to_process: Vec<u8>,
    /// The tspec address-space manager (decodes the getPcode `<addr>`, encodes
    /// the canned `<inst>` / register responses).
    manager: Rc<AddrSpaceManager>,
}

impl MockState {
    /// Scan `from_process` for complete query frames `{4}{14}<doc>{15}{5}`,
    /// generating a response for each into `to_process`.  Safe because packed
    /// payloads are 0x00-free, so `[0,0,1,4]` marks only a query open.
    fn pump(&mut self) {
        loop {
            let rest = &self.from_process[self.parsed..];
            let start = match find_subseq(rest, &[0, 0, 1, 4]) {
                Some(i) => self.parsed + i,
                None => {
                    // No query pending; keep the last 3 bytes in case a marker
                    // straddles the next write.
                    self.parsed = self.from_process.len().saturating_sub(3);
                    return;
                }
            };
            let after_qopen = start + 4;
            // Expect the string-open marker {14}.
            if self.from_process.len() < after_qopen + 4 {
                self.parsed = start;
                return; // incomplete — wait for more bytes
            }
            if self.from_process[after_qopen..after_qopen + 4] != [0, 0, 1, 14] {
                // Not a well-formed query open; skip this marker.
                self.parsed = start + 1;
                continue;
            }
            let doc_start = after_qopen + 4;
            // The packed doc is 0x00-free, so {15} delimits it unambiguously.
            let close = match find_subseq(&self.from_process[doc_start..], &[0, 0, 1, 15]) {
                Some(i) => doc_start + i,
                None => {
                    self.parsed = start;
                    return; // incomplete — wait for the string close
                }
            };
            let doc = self.from_process[doc_start..close].to_vec();
            let after_sclose = close + 4; // past {15}
            if self.from_process.len() < after_sclose + 4 {
                self.parsed = start;
                return; // the {5} query close has not arrived yet
            }
            let resp = self.generate_response(&doc);
            self.to_process.extend_from_slice(&resp);
            self.parsed = after_sclose + 4; // past the {5} query close
        }
    }

    /// Decode a query's command element id and produce its response bytes.
    fn generate_response(&self, doc: &[u8]) -> Vec<u8> {
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

/// The process's `sin`: serves query responses (from `to_process`) first, then
/// the proactive command stream.
struct MockReader {
    shared: Rc<RefCell<MockState>>,
}
impl Read for MockReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut st = self.shared.borrow_mut();
        st.pump(); // answer any pending query before serving
        if !st.to_process.is_empty() {
            let n = min(buf.len(), st.to_process.len());
            buf[..n].copy_from_slice(&st.to_process[..n]);
            st.to_process.drain(..n);
            return Ok(n);
        }
        if st.cmd_cursor >= st.commands.len() {
            return Ok(0); // EOF: the test stops driving before this is hit
        }
        let n = min(buf.len(), st.commands.len() - st.cmd_cursor);
        let start = st.cmd_cursor;
        buf[..n].copy_from_slice(&st.commands[start..start + n]);
        st.cmd_cursor += n;
        Ok(n)
    }
}

/// The process's `sout`: accumulates output; flush triggers the query pump.
struct MockWriter {
    shared: Rc<RefCell<MockState>>,
}
impl Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.shared.borrow_mut().from_process.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.shared.borrow_mut().pump();
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Wire builders (mirroring DecompileProcess.java's writer)
// ---------------------------------------------------------------------------

fn burst(v: &mut Vec<u8>, code: u8) {
    v.extend_from_slice(&[0, 0, 1, code]);
}

/// `writeString`: {14} bytes {15}.
fn wire_string(v: &mut Vec<u8>, s: &[u8]) {
    burst(v, 14);
    v.extend_from_slice(s);
    burst(v, 15);
}

/// A query response wrapping a string stream: {8}{14}s{15}{9}.
fn resp_string(s: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 8);
    wire_string(&mut r, s);
    burst(&mut r, 9);
    r
}

/// An empty query response: {8}{9} (read_all -> false: BadData / DataUnavail /
/// "not found", per the caller).
fn resp_empty() -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 8);
    burst(&mut r, 9);
    r
}

/// Build the proactive command stream: registerProgram + setAction + decompileAt.
fn build_commands(addr_packed: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    // registerProgram: {2}"registerProgram" + 4 string streams + {3}
    burst(&mut v, 2);
    wire_string(&mut v, b"registerProgram");
    wire_string(&mut v, PSPEC);
    wire_string(&mut v, CSPEC);
    wire_string(&mut v, TSPEC);
    wire_string(&mut v, CORETYPES);
    burst(&mut v, 3);
    // setAction: archid + "decompile" + "c"
    burst(&mut v, 2);
    wire_string(&mut v, b"setAction");
    wire_string(&mut v, b"0");
    wire_string(&mut v, b"decompile");
    wire_string(&mut v, b"c");
    burst(&mut v, 3);
    // decompileAt: archid + packed <addr>
    burst(&mut v, 2);
    wire_string(&mut v, b"decompileAt");
    wire_string(&mut v, b"0");
    wire_string(&mut v, addr_packed);
    burst(&mut v, 3);
    v
}

// ---------------------------------------------------------------------------
// Response-stream tokenizer + <doc> extractor
// ---------------------------------------------------------------------------

/// Find the first occurrence of `needle` in `hay`.
fn find_subseq(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    (0..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

/// Split the process output into `(burst_code, payload)` tokens.  Payloads are
/// 0x00-free, so each is exactly the bytes between one marker and the next.
fn tokenize(stream: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut toks = Vec::new();
    let mut i = 0usize;
    while i < stream.len() {
        // Skip any non-zero garbage, then the (1+) zero run, then 0x01.
        while i < stream.len() && stream[i] != 0 {
            i += 1;
        }
        while i < stream.len() && stream[i] == 0 {
            i += 1;
        }
        if i >= stream.len() || stream[i] != 1 {
            break;
        }
        i += 1;
        if i >= stream.len() {
            break;
        }
        let code = stream[i];
        i += 1;
        let start = i;
        while i < stream.len() && stream[i] != 0 {
            i += 1;
        }
        toks.push((code, stream[start..i].to_vec()));
    }
    toks
}

/// Extract the payload string-stream of the LAST command response (the
/// decompileAt `<doc>`): within the last `{6}..{7}`, the `{14}` token that is NOT
/// nested inside a `{4}..{5}` query.
fn extract_last_response_payload(stream: &[u8]) -> Vec<u8> {
    let toks = tokenize(stream);
    let last6 = toks
        .iter()
        .rposition(|(c, _)| *c == 6)
        .expect("a command-response open in the output");
    let mut in_query = false;
    for (code, payload) in &toks[last6..] {
        match code {
            4 => in_query = true,
            5 => in_query = false,
            14 if !in_query => return payload.clone(),
            7 => break,
            _ => {}
        }
    }
    Vec::new()
}

// ---------------------------------------------------------------------------
// <doc> decoder
// ---------------------------------------------------------------------------

/// Recursively walk the current (already-opened) element, collecting the values
/// of the `target` attribute ids into `out`, then recursing into children.
fn walk(
    dec: &mut PackedDecode,
    targets: &[u32],
    out: &mut BTreeMap<u32, BTreeSet<u64>>,
) -> Result<(), kuna_base::error::KunaError> {
    loop {
        let aid = dec.get_next_attribute_id()?;
        if aid == 0 {
            break;
        }
        if targets.contains(&aid) {
            let v = dec.read_unsigned_integer()?;
            out.entry(aid).or_default().insert(v);
        }
        // Un-targeted attributes are left un-read; the next get_next_attribute_id
        // auto-skips them (PackedDecode contract).
    }
    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        let id = dec.open_element()?;
        walk(dec, targets, out)?;
        dec.close_element(id)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------

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

    let shared = Rc::new(RefCell::new(MockState {
        commands: build_commands(&addr_packed),
        cmd_cursor: 0,
        from_process: Vec::new(),
        parsed: 0,
        to_process: Vec::new(),
        manager: Rc::clone(&manager),
    }));

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

    // Extract the decompileAt <doc> payload and decode it.
    let doc = extract_last_response_payload(&out);
    assert!(!doc.is_empty(), "decompileAt emitted an EMPTY payload (no <doc>)");

    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&doc).expect("<doc> ingests");
    let did = dec.open_element().expect("open <doc>");
    assert_eq!(did, ELEM_DOC.get_id(), "root is not <doc>");

    // --- the first <function>: the Funcdata::encode <function>/<ast> ---
    let fid = dec.open_element().expect("open <function>");
    assert_eq!(fid, ELEM_FUNCTION_ID, "first child of <doc> is not <function>");
    let name = dec.read_string_id(&ATTRIB_NAME).expect("<function name>");
    assert_eq!(
        name, FUNC_NAME,
        "<function name> must be the getCodeLabel name"
    );
    // Collect the AST's op times (<seqnum uniq>) and varnode create-indices
    // (<addr ref>).
    let mut ast = BTreeMap::new();
    walk(&mut dec, &[ATTRIB_REF_ID, ATTRIB_UNIQ_ID], &mut ast).expect("walk <function>");
    dec.close_element(fid).expect("close <function>");

    let op_times = ast.get(&ATTRIB_UNIQ_ID).cloned().unwrap_or_default();
    let var_refs = ast.get(&ATTRIB_REF_ID).cloned().unwrap_or_default();
    assert!(
        !op_times.is_empty(),
        "the <ast> has no ops (empty function body) — expected the RETURN op"
    );

    // --- the second <function>: the Clang token markup ---
    let next = dec.peek_element().expect("peek after <function>");
    assert_eq!(
        next, ELEM_FUNCTION_ID,
        "the C-code markup <function> is missing (send_c_code + decompile action)"
    );
    let mid = dec.open_element().expect("open markup <function>");
    let mut markup = BTreeMap::new();
    walk(&mut dec, &[ATTRIB_OPREF_ID, ATTRIB_VARREF_ID], &mut markup)
        .expect("walk markup <function>");
    dec.close_element(mid).expect("close markup <function>");
    dec.close_element(did).expect("close <doc>");

    let oprefs = markup.get(&ATTRIB_OPREF_ID).cloned().unwrap_or_default();
    let varrefs = markup.get(&ATTRIB_VARREF_ID).cloned().unwrap_or_default();

    // The click-to-address contract: every markup token ref resolves against the
    // AST it was emitted alongside.
    assert!(
        oprefs.is_subset(&op_times),
        "markup oprefs {oprefs:?} are not a subset of the AST op times {op_times:?}"
    );
    assert!(
        varrefs.is_subset(&var_refs),
        "markup varrefs {varrefs:?} are not a subset of the AST varnode refs {var_refs:?}"
    );
    // The markup must actually reference the syntax tree (a bare `return;` still
    // tags its statement/op), proving the two documents are cross-linked rather
    // than trivially both empty.
    assert!(
        !oprefs.is_empty() || !varrefs.is_empty(),
        "the markup carried NO opref/varref — the dual <function> is not cross-linked to the <ast>"
    );
}
