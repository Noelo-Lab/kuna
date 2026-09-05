//! Shared MockJava machinery for the ghidra-mode test harness.
//!
//! One end of the decompiler-process pipe, in-process: the interactive
//! single-threaded loopback proven by `decompile_at_e2e.rs`, extracted so both
//! the canned-answer e2e test and the real-program **ghidra-sim** harness
//! (`ghidra_sim_e2e.rs`) drive the SAME machinery.  The process writes queries
//! into a shared buffer; the mock decodes each query frame and asks a pluggable
//! [`AnswerSource`] for the response.  This is sound because packed protocol
//! payloads are 0x00-free (protocol.rs), so the 4-byte query-open marker
//! `[0,0,1,4]` never occurs inside a payload and query frames are unambiguously
//! delimited.
//!
//! Contents:
//!   * the [`AnswerSource`] trait + [`MockState`]/[`MockReader`]/[`MockWriter`]
//!     loopback (the process's `sin`/`sout`),
//!   * wire builders mirroring `DecompileProcess.java`'s writer (command
//!     frames, query responses),
//!   * a session-output tracer ([`trace_session`]) splitting the raw output
//!     stream into per-command responses with their nested queries + warnings,
//!   * a `decompileAt` `<doc>` parser ([`parse_decompile_doc`]) that decodes
//!     the dual `<function>` and **flattens the Clang markup to C text**, and
//!   * the static badness scanners (register/Unique/placeholder leaks) and the
//!     normalized line-diff ratio used by the differential-C pins.
//!
//! The real-program answer source lives in [`oracle`] (backed by
//! `kuna_console::engine::bootstrap_from_object` — the same in-process load the
//! CLI uses).

#![allow(dead_code)] // shared by several test binaries; each uses a subset

pub mod oracle;

use std::cell::RefCell;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::rc::Rc;

use kuna_base::address::{Address, ELEM_ADDR};
use kuna_base::error::KunaError;
use kuna_base::marshal::{Decoder, PackedDecode, ATTRIB_CONTENT, ATTRIB_NAME};
use kuna_base::space::AddrSpaceManager;

use kuna_ghidra::ids::ELEM_DOC;

// ---------------------------------------------------------------------------
// Raw numeric marshaling ids used only for decoding responses
// ---------------------------------------------------------------------------

/// Varnode::getCreateIndex back-ref (`<addr ref=…>`, marshal.rs).
pub const ATTRIB_REF_ID: u32 = 18;
/// `<seqnum uniq>` == PcodeOp::getTime (address.rs).
pub const ATTRIB_UNIQ_ID: u32 = 29;
/// EmitMarkup opref (prettyprint.rs).
pub const ATTRIB_OPREF_ID: u32 = 41;
/// EmitMarkup varref (prettyprint.rs).
pub const ATTRIB_VARREF_ID: u32 = 42;
/// EmitMarkup `<break indent=…>` (prettyprint.rs).
pub const ATTRIB_INDENT_ID: u32 = 38;
/// EmitMarkup `color` (prettyprint.rs) — the ClangToken syntax highlight.
pub const ATTRIB_COLOR_ID: u32 = 37;
/// `<function>` (funcdata_encode.rs).
pub const ELEM_FUNCTION_ID: u32 = 116;
/// `<ast>` (funcdata_encode.rs).
pub const ELEM_AST_ID: u32 = 115;
/// `<varnodes>` — the mandatory first child of `<ast>` (funcdata_encode.rs).
pub const ELEM_VARNODES_ID: u32 = 119;
/// `<break>` — the markup line break (prettyprint.rs).
pub const ELEM_BREAK_ID: u32 = 17;
/// `<funcname>` (prettyprint.rs) → Java ClangFuncNameToken.
pub const ELEM_FUNCNAME_ID: u32 = 19;
/// `<label>` (prettyprint.rs) → Java ClangLabelToken.
pub const ELEM_LABEL_ID: u32 = 21;
/// `<variable>` (prettyprint.rs) → Java ClangVariableToken.
pub const ELEM_VARIABLE_ID: u32 = 26;
/// `<field>` (type.cc vocabulary) → Java ClangFieldToken.
pub const ELEM_FIELD_ID: u32 = 49;
/// `<type>` (type.cc vocabulary) → Java ClangTypeToken.
pub const ELEM_TYPE_ID: u32 = 60;
/// `<vardecl>` (prettyprint.rs) → Java ClangVariableDecl, whose `symref` MUST
/// resolve in the decoded LocalSymbolMap or declaration-line rename/retype is
/// dead (and Java logs "Invalid symbol reference" per declaration).
pub const ELEM_VARDECL_ID: u32 = 25;
/// `ClangToken.CONST_COLOR` (== `SyntaxHighlight::ConstColor`).
pub const CONST_COLOR: u64 = 5;

// --- Phase-4 first-`<function>` vocabulary (upstream numbers) --------------
/// `<addr>` (address.rs).
pub const ELEM_ADDR_ID: u32 = 11;
/// `<localdb>` (database.cc vocabulary).
pub const ELEM_LOCALDB_ID: u32 = 228;
/// `<scope>`.
pub const ELEM_SCOPE_ID: u32 = 80;
/// `<parent>`.
pub const ELEM_PARENT_ID: u32 = 77;
/// `<rangelist>`.
pub const ELEM_RANGELIST_ID: u32 = 13;
/// `<symbollist>`.
pub const ELEM_SYMBOLLIST_ID: u32 = 81;
/// `<mapsym>`.
pub const ELEM_MAPSYM_ID: u32 = 76;
/// `<symbol>`.
pub const ELEM_SYMBOL_ID: u32 = 6;
/// `<equatesymbol>`.
pub const ELEM_EQUATESYMBOL_ID: u32 = 69;
/// `<hash>` (a dynamic SymbolEntry).
pub const ELEM_HASH_ID: u32 = 73;
/// `<highlist>` (funcdata.cc vocabulary).
pub const ELEM_HIGHLIST_ID: u32 = 117;
/// `<high>` (variable.cc vocabulary).
pub const ELEM_HIGH_ID: u32 = 82;
/// `<jumptablelist>`.
pub const ELEM_JUMPTABLELIST_ID: u32 = 118;
/// `<jumptable>`.
pub const ELEM_JUMPTABLE_ID: u32 = 213;
/// `<dest>` (one switch case target).
pub const ELEM_DEST_ID: u32 = 212;
/// `<loadtable>` (jumpload-collected memory reads).
pub const ELEM_LOADTABLE_ID: u32 = 214;
/// `<prototype>` (fspec.cc vocabulary).
pub const ELEM_PROTOTYPE_ID: u32 = 169;
/// `<returnsym>`.
pub const ELEM_RETURNSYM_ID: u32 = 172;
/// `<typeref>` (type.cc vocabulary).
pub const ELEM_TYPEREF_ID: u32 = 63;
/// `<def>` (a typedef).
pub const ELEM_DEF_ID: u32 = 43;
/// `<void>`.
pub const ELEM_VOID_ID: u32 = 10;
/// `<override>` (Java-skipped).
pub const ELEM_OVERRIDE_ID: u32 = 223;
/// `<parammeasures>` (paramid.cc vocabulary).
pub const ELEM_PARAMMEASURES_ID: u32 = 106;
/// `<rank>` — REQUIRED per measure (ParamMeasure.decode throws).
pub const ELEM_RANK_ID: u32 = 108;
/// `<input>` / `<output>` measure wrappers.
pub const ELEM_INPUT_ID: u32 = 2;
pub const ELEM_OUTPUT_ID: u32 = 4;
/// symbol/scope `id` attribute.
pub const ATTRIB_ID_ID: u32 = 9;
/// symbol `cat` (signed; -1 none, 0 parameter).
pub const ATTRIB_CAT_ID: u32 = 61;
/// symbol `index` (the parameter slot).
pub const ATTRIB_INDEX_ID: u32 = 10;
/// high `class`.
pub const ATTRIB_CLASS_ID: u32 = 66;
/// high `repref`.
pub const ATTRIB_REPREF_ID: u32 = 67;
/// high/vardecl `symref`.
pub const ATTRIB_SYMREF_ID: u32 = 68;
/// prototype `model`.
pub const ATTRIB_MODEL_ID: u32 = 13;
/// prototype `extrapop`.
pub const ATTRIB_EXTRAPOP_ID: u32 = 6;
/// `Symbol::ID_BASE` — the internal symbol-id range marker (top byte 0x40).
pub const SYMBOL_ID_BASE_TOP: u64 = 0x40;
/// range `first` attribute (address.rs).
pub const ATTRIB_FIRST_ID: u32 = 27;

/// One decoded `<localdb>` `<mapsym>` (what `HighSymbol.decodeMapSym` sees).
#[derive(Debug)]
pub struct SimSymbol {
    /// `<symbol id>` — MUST be nonzero (`HighSymbol.decodeHeader` throws).
    pub id: u64,
    /// `cat` (-1 none, 0 parameter).
    pub cat: i64,
    /// `index` when cat >= 0.
    pub index: Option<u64>,
    /// The symbol's name.
    pub name: String,
    /// Number of SymbolEntry pairs — MUST be >= 1 (`insertSymbol` NPEs on 0).
    pub entries: usize,
    /// Whether every entry carried its uselimit `<rangelist>`.
    pub entries_have_rangelists: bool,
    /// The `<symbol>` body carried a type child (`<type>`/`<typeref>`/`<def>`).
    pub has_type: bool,
    /// The type child's `size` attribute (Java derives the SymbolEntry size
    /// from the data-type; the `<addr>` itself is unsized, database.cc:196).
    /// A `<typeref>` carries no size unless variable-length — see `type_name`.
    pub type_size: Option<i64>,
    /// The type child's `name` attribute.
    pub type_name: Option<String>,
    /// The symbol's data-type body was `<void/>` — a 0-sized type, which
    /// `MappedEntry.decode` rejects.
    pub type_is_void: bool,
    /// The first mapped entry's storage `(space name, offset)`.
    pub entry_storage: Option<(String, u64)>,
    /// The first entry's uselimit start (`<range first>`), when non-empty.
    pub first_use: Option<u64>,
    /// The `<hash val>` of a DYNAMIC entry (0 = mapped storage).
    pub hash: u64,
}

/// One decoded `<high>` (what `HighFunction.decodeHigh` sees).
#[derive(Debug)]
pub struct SimHigh {
    /// `class` — REQUIRED, one of other/global/constant/param/local.
    pub class: String,
    /// `repref` — must resolve in the `<ast>` (decodeInstances throws).
    pub repref: Option<u64>,
    /// `symref` — REQUIRED for local/param (HighLocal.decode throws).
    pub symref: Option<u64>,
}

/// The decoded `<prototype>` header (what `FunctionPrototype.decodePrototype`
/// requires).
#[derive(Debug)]
pub struct SimProto {
    /// `model` attribute value.
    pub model: String,
    /// `extrapop` attribute present (int or "unknown").
    pub has_extrapop: bool,
    /// `<returnsym>` with an `<addr>` first child then a data-type child.
    pub returnsym_ok: bool,
}

/// The 19 legal callback-query command element ids (ids.rs 239..=257).
pub const QUERY_COMMAND_IDS: std::ops::RangeInclusive<u32> = 239..=257;

/// A `<coretypes>` registerProgram document mirroring the standalone default
/// core-type set (`Architecture::build_core_types`) EXACTLY: same names, same
/// sizes, same metatypes, and — because no `id` attribute is given — the same
/// `hash_name` ids `set_core_type` assigns.  Phase 3 decodes the wire
/// coretypes for real, so the sim must send a complete set (the old one-entry
/// stub left the engine without char/bool/void core types and broke init);
/// mirroring the default keeps the ghidra-mode factory identical to the
/// oracle's, which is what makes `<typeref>` name+id resolution agree.
pub const DEFAULT_CORETYPES_XML: &[u8] = b"<coretypes>\
<type name=\"void\" size=\"1\" metatype=\"void\"/>\
<type name=\"bool\" size=\"1\" metatype=\"bool\"/>\
<type name=\"uint1\" size=\"1\" metatype=\"uint\"/>\
<type name=\"uint2\" size=\"2\" metatype=\"uint\"/>\
<type name=\"uint4\" size=\"4\" metatype=\"uint\"/>\
<type name=\"uint8\" size=\"8\" metatype=\"uint\"/>\
<type name=\"int1\" size=\"1\" metatype=\"int\"/>\
<type name=\"int2\" size=\"2\" metatype=\"int\"/>\
<type name=\"int4\" size=\"4\" metatype=\"int\"/>\
<type name=\"int8\" size=\"8\" metatype=\"int\"/>\
<type name=\"float4\" size=\"4\" metatype=\"float\"/>\
<type name=\"float8\" size=\"8\" metatype=\"float\"/>\
<type name=\"float10\" size=\"10\" metatype=\"float\"/>\
<type name=\"float16\" size=\"16\" metatype=\"float\"/>\
<type name=\"xunknown1\" size=\"1\" metatype=\"unknown\"/>\
<type name=\"xunknown2\" size=\"2\" metatype=\"unknown\"/>\
<type name=\"xunknown4\" size=\"4\" metatype=\"unknown\"/>\
<type name=\"xunknown8\" size=\"8\" metatype=\"unknown\"/>\
<type name=\"code\" size=\"1\" metatype=\"code\"/>\
<type name=\"char\" size=\"1\" metatype=\"int\" char=\"true\"/>\
<type name=\"wchar2\" size=\"2\" metatype=\"int\" utf=\"true\"/>\
<type name=\"wchar4\" size=\"4\" metatype=\"int\" utf=\"true\"/>\
</coretypes>";

// ---------------------------------------------------------------------------
// Interactive loopback MockJava
// ---------------------------------------------------------------------------

/// The pluggable mock-Java answer half: given a decoded query *document* (the
/// packed bytes between the `{14}`/`{15}` of a `{4}..{5}` query frame),
/// produce the full response byte frame to feed back to the process.
pub trait AnswerSource {
    fn respond(&mut self, doc: &[u8]) -> Vec<u8>;
}

/// State shared between the process's `sin` ([`MockReader`]) and `sout`
/// ([`MockWriter`]): the proactive command stream, the process's output buffer
/// (scanned for query frames), and the response queue fed back to the process.
pub struct MockState<S: AnswerSource> {
    /// The proactive command frames (registerProgram/setAction/decompileAt/…).
    pub commands: Vec<u8>,
    /// Cursor into `commands`.
    pub cmd_cursor: usize,
    /// Everything the process has written (queries + command-response framing).
    pub from_process: Vec<u8>,
    /// How far `from_process` has been scanned for query frames.
    pub parsed: usize,
    /// Bytes queued to feed the process back (query responses).
    pub to_process: Vec<u8>,
    /// The answer generator.
    pub source: S,
}

impl<S: AnswerSource> MockState<S> {
    pub fn new(commands: Vec<u8>, source: S) -> Self {
        MockState {
            commands,
            cmd_cursor: 0,
            from_process: Vec::new(),
            parsed: 0,
            to_process: Vec::new(),
            source,
        }
    }

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
            let resp = self.source.respond(&doc);
            self.to_process.extend_from_slice(&resp);
            self.parsed = after_sclose + 4; // past the {5} query close
        }
    }
}

/// The process's `sin`: serves query responses (from `to_process`) first, then
/// the proactive command stream.
pub struct MockReader<S: AnswerSource> {
    pub shared: Rc<RefCell<MockState<S>>>,
}
impl<S: AnswerSource> Read for MockReader<S> {
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
pub struct MockWriter<S: AnswerSource> {
    pub shared: Rc<RefCell<MockState<S>>>,
}
impl<S: AnswerSource> Write for MockWriter<S> {
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

pub fn burst(v: &mut Vec<u8>, code: u8) {
    v.extend_from_slice(&[0, 0, 1, code]);
}

/// `writeString`: {14} bytes {15}.
pub fn wire_string(v: &mut Vec<u8>, s: &[u8]) {
    burst(v, 14);
    v.extend_from_slice(s);
    burst(v, 15);
}

/// A query response wrapping a string stream: {8}{14}s{15}{9}.  Carries either
/// raw text (getCodeLabel/getRegisterName/…) or a packed document.
pub fn resp_string(s: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 8);
    wire_string(&mut r, s);
    burst(&mut r, 9);
    r
}

/// An empty query response: {8}{9} (read_all -> false: BadData / DataUnavail /
/// "not found", per the caller).
pub fn resp_empty() -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 8);
    burst(&mut r, 9);
    r
}

/// An exception frame instead of a query response: {10}"type"{14..15}"msg"{11}
/// — what `DecompileProcess.readResponse` writes when the callback THREW
/// (DecompileProcess.java:384-392), before logging `Unexpected Exception: …`.
/// `getRegister` on a name the language does not define is the throwing case
/// (`DecompileCallback.getRegister` → `No Register Defined: <name>`).
pub fn resp_exception(extype: &str, msg: &str) -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 10);
    wire_string(&mut r, extype.as_bytes());
    wire_string(&mut r, msg.as_bytes());
    burst(&mut r, 11);
    r
}

/// A byte-burst query response: {8}{12} raw {13}{9} (getBytes / getStringData;
/// the payload is already nibble-doubled / header-prefixed by the caller).
pub fn resp_bytes(raw: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    burst(&mut r, 8);
    burst(&mut r, 12);
    r.extend_from_slice(raw);
    burst(&mut r, 13);
    burst(&mut r, 9);
    r
}

/// registerProgram: {2}"registerProgram" + the four spec string streams + {3}.
pub fn cmd_register_program(
    v: &mut Vec<u8>,
    pspec: &[u8],
    cspec: &[u8],
    tspec: &[u8],
    coretypes: &[u8],
) {
    burst(v, 2);
    wire_string(v, b"registerProgram");
    wire_string(v, pspec);
    wire_string(v, cspec);
    wire_string(v, tspec);
    wire_string(v, coretypes);
    burst(v, 3);
}

/// setAction: archid + actionstring + printstring.
pub fn cmd_set_action(v: &mut Vec<u8>, archid: &str, action: &str, print: &str) {
    burst(v, 2);
    wire_string(v, b"setAction");
    wire_string(v, archid.as_bytes());
    wire_string(v, action.as_bytes());
    wire_string(v, print.as_bytes());
    burst(v, 3);
}

/// decompileAt: archid + the packed `<addr>` of the entry.
pub fn cmd_decompile_at(v: &mut Vec<u8>, archid: &str, addr_packed: &[u8]) {
    burst(v, 2);
    wire_string(v, b"decompileAt");
    wire_string(v, archid.as_bytes());
    wire_string(v, addr_packed);
    burst(v, 3);
}

/// flushNative: archid only.
pub fn cmd_flush_native(v: &mut Vec<u8>, archid: &str) {
    burst(v, 2);
    wire_string(v, b"flushNative");
    wire_string(v, archid.as_bytes());
    burst(v, 3);
}

/// deregisterProgram: archid only (terminates the process loop).
pub fn cmd_deregister_program(v: &mut Vec<u8>, archid: &str) {
    burst(v, 2);
    wire_string(v, b"deregisterProgram");
    wire_string(v, archid.as_bytes());
    burst(v, 3);
}

// ---------------------------------------------------------------------------
// Response-stream tokenizer + session tracer
// ---------------------------------------------------------------------------

/// Find the first occurrence of `needle` in `hay`.
pub fn find_subseq(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    (0..=hay.len() - needle.len()).find(|&i| &hay[i..i + needle.len()] == needle)
}

/// Split the process output into `(burst_code, payload)` tokens.  Payloads are
/// 0x00-free, so each is exactly the bytes between one marker and the next.
pub fn tokenize(stream: &[u8]) -> Vec<(u8, Vec<u8>)> {
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

/// One command's response span (`{6}..{7}`) in the process output.
#[derive(Debug, Default)]
pub struct ResponseTrace {
    /// The response payload: the first `{14}` string stream NOT nested inside
    /// a `{4}..{5}` query frame (`None` for a "Bad command" response).
    pub payload: Option<Vec<u8>>,
    /// The packed query documents this command issued, in wire order.
    pub queries: Vec<Vec<u8>>,
    /// The `{16}..{17}` warnings frame text (empty when the frame was empty
    /// or — deregister — absent).
    pub warnings: String,
}

/// The whole session output split per command response.
#[derive(Debug, Default)]
pub struct SessionTrace {
    pub responses: Vec<ResponseTrace>,
}

/// Split the raw output stream into per-command [`ResponseTrace`]s: for each
/// `{6}..{7}` span, the non-query-nested payload, the nested query docs, and
/// the warnings text.
pub fn trace_session(stream: &[u8]) -> SessionTrace {
    let mut trace = SessionTrace::default();
    let mut in_query = false;
    for (code, payload) in tokenize(stream) {
        match code {
            6 => {
                trace.responses.push(ResponseTrace::default());
                in_query = false;
            }
            4 => in_query = true,
            5 => in_query = false,
            // The warnings text sits between the {16} marker and the {17}
            // marker's leading 0x00, so tokenize() attaches it to the {16}
            // token's payload.
            16 => {
                if let Some(cur) = trace.responses.last_mut() {
                    cur.warnings = String::from_utf8_lossy(&payload).into_owned();
                }
            }
            14 => {
                if let Some(cur) = trace.responses.last_mut() {
                    if in_query {
                        cur.queries.push(payload);
                    } else if cur.payload.is_none() {
                        cur.payload = Some(payload);
                    }
                }
            }
            _ => {}
        }
    }
    trace
}

/// The root element id of a packed query document (one of the 19 command ids).
pub fn query_doc_id(doc: &[u8], manager: &AddrSpaceManager) -> u32 {
    let mut dec = PackedDecode::new(manager);
    dec.ingest_stream(doc).expect("query doc ingests");
    dec.open_element().expect("query doc has a root element")
}

// ---------------------------------------------------------------------------
// <doc> decoding: attribute walk, dual-<function> parse, markup flatten
// ---------------------------------------------------------------------------

/// Consume (skip) all attributes of the current open element, leaving the
/// decoder positioned at its first child.
pub fn skip_attributes(dec: &mut PackedDecode) -> Result<(), KunaError> {
    loop {
        if dec.get_next_attribute_id()? == 0 {
            return Ok(());
        }
    }
}

/// Recursively walk the current (already-opened) element, collecting the values
/// of the `target` attribute ids into `out`, then recursing into children.
pub fn walk(
    dec: &mut PackedDecode,
    targets: &[u32],
    out: &mut BTreeMap<u32, BTreeSet<u64>>,
) -> Result<(), KunaError> {
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

/// A fully-decoded `decompileAt` response document.
#[derive(Debug, Default)]
pub struct ParsedDoc {
    /// `<function name=…>` — must echo the name the mock served.
    pub name: String,
    /// The entry `<addr>` offset — must echo the requested address
    /// (HighFunction.decode throws on mismatch).
    pub entry_offset: Option<u64>,
    /// `<ast>` op times (`<seqnum uniq=…>`).
    pub ast_op_times: BTreeSet<u64>,
    /// `<ast>` varnode create-indices — collected from the `<varnodes>` child
    /// ONLY, because that is all Java's `PcodeSyntaxTree.buildVarnodeRefs`
    /// keys: an `<op>` operand `ref` that is not declared in `<varnodes>`
    /// would silently resolve to null in the GUI, so it must not launder the
    /// subset assertion.
    pub ast_var_refs: BTreeSet<u64>,
    /// Markup `opref` values (must be ⊆ `ast_op_times`).
    pub markup_oprefs: BTreeSet<u64>,
    /// Markup `varref` values (must be ⊆ `ast_var_refs`).
    pub markup_varrefs: BTreeSet<u64>,
    /// Whether the second (markup) `<function>` was present.
    pub has_markup: bool,
    /// The C text flattened from the markup token stream (content strings +
    /// `<break indent>` line breaks), with Java's `getC()` token cleaning
    /// applied ([`illegal_char_cpp_transform`]) — what a script/export user
    /// actually receives from `DecompileResults.getDecompiledFunction()`.
    pub c_text: String,
    /// Tokens whose text the Java-side `IllegalCharCppTransformer` CHANGED —
    /// each is a token kuna emitted with non-identifier characters inside an
    /// identifier-class token (e.g. the whole rendered declarator
    /// `"unsigned long *"` as ONE `<type>` token, which `getC()` mangles to
    /// `unsigned_long__`).  Zero once the markup emitter splits declarators
    /// into base-type + syntax tokens (PR-C).
    pub mangled_tokens: usize,
    // --- Phase-4 first-<function> children ---------------------------------
    /// The `<function>` child element ids in stream order (the Java decode is
    /// order-sensitive: localdb before highlist, ast before highlist).
    pub function_child_ids: Vec<u32>,
    /// `<localdb>` present, with the decoded `<mapsym>` symbols.
    pub localdb: Option<Vec<SimSymbol>>,
    /// The `<scope>`'s first two child element ids (POSITIONALLY parent +
    /// rangelist — `LocalSymbolMap.decodeScope` skips them blind).
    pub scope_prefix: Vec<u32>,
    /// `<highlist>` present, with the decoded `<high>` headers.
    pub highlist: Option<Vec<SimHigh>>,
    /// `<prototype>` header when present.
    pub prototype: Option<SimProto>,
    /// Per `<jumptable>`: (dest count, loadtable count).
    pub jumptables: Vec<(usize, usize)>,
    /// `<parammeasures>` when present: per-measure (is_input, has_rank).
    pub parammeasures: Option<Vec<(bool, bool)>>,
    /// Whether the first `<function>` was present at all (a parammeasures-only
    /// paramid doc has none).
    pub has_function: bool,
    /// Every `<vardecl symref>` in the markup — each SHOULD resolve against
    /// the `<localdb>` ids (Java `ClangVariableDecl.decode` →
    /// `pfactory.getSymbol(symref)`); an unresolvable one leaves
    /// rename/retype dead on that declaration line and logs once per
    /// decompile.
    pub vardecl_symrefs: BTreeSet<u64>,
    /// How many of them do NOT resolve (the create-index fallback) — pinned,
    /// so the systemic case stays fixed and the residue can only shrink.
    pub vardecl_symref_unresolved: usize,
}

/// Decode a `decompileAt` `<doc>` payload: an optional `<parammeasures>`, the
/// first `<function>` (name/entry echo, `<ast>` refs, and the Phase-4
/// `<localdb>`/`<highlist>`/`<prototype>`/`<jumptablelist>` children), and the
/// second (markup) `<function>` flattened to C with its opref/varref sets —
/// dispatching exactly like `DecompileResults.decodeStream`.
pub fn parse_decompile_doc(doc: &[u8], manager: &AddrSpaceManager) -> ParsedDoc {
    let mut parsed = ParsedDoc::default();
    let mut dec = PackedDecode::new(manager);
    dec.ingest_stream(doc).expect("<doc> ingests");
    let did = dec.open_element().expect("open <doc>");
    assert_eq!(did, ELEM_DOC.get_id(), "root is not <doc>");

    loop {
        let c = dec.peek_element().expect("peek <doc> child");
        if c == 0 {
            break;
        }
        if c == ELEM_PARAMMEASURES_ID {
            let pid = dec.open_element().expect("open <parammeasures>");
            parse_parammeasures(&mut dec, &mut parsed);
            dec.close_element(pid).expect("close <parammeasures>");
        } else if c == ELEM_FUNCTION_ID && !parsed.has_function {
            let fid = dec.open_element().expect("open <function>");
            parsed.has_function = true;
            parse_first_function(&mut dec, &mut parsed);
            dec.close_element(fid).expect("close <function>");
        } else if c == ELEM_FUNCTION_ID {
            // --- the second <function>: the Clang token markup ---
            parsed.has_markup = true;
            let mid = dec.open_element().expect("open markup <function>");
            let mut refs = BTreeMap::new();
            let mut ctext = String::new();
            let mut mangled = 0usize;
            flatten_markup(&mut dec, mid, &mut ctext, &mut refs, &mut mangled)
                .expect("flatten markup <function>");
            dec.close_element(mid).expect("close markup <function>");
            parsed.markup_oprefs = refs.get(&ATTRIB_OPREF_ID).cloned().unwrap_or_default();
            parsed.markup_varrefs = refs.get(&ATTRIB_VARREF_ID).cloned().unwrap_or_default();
            parsed.vardecl_symrefs = refs.get(&ATTRIB_SYMREF_ID).cloned().unwrap_or_default();
            parsed.c_text = ctext;
            parsed.mangled_tokens = mangled;
        } else {
            // Java: "Unknown decompiler tag" — the whole result is discarded.
            panic!("unknown <doc> child element id {c}");
        }
    }
    dec.close_element(did).expect("close <doc>");
    parsed
}

/// The first `<function>`: name/entry + the child walk (opened by the caller).
fn parse_first_function(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    parsed.name = String::from_utf8_lossy(
        &dec.read_string_id(&ATTRIB_NAME).expect("<function name>"),
    )
    .into_owned();
    // The base address is the first child element.
    if dec.peek_element().expect("peek in <function>") == ELEM_ADDR.get_id() {
        let addr = Address::decode(dec).expect("<function> base <addr>");
        parsed.entry_offset = Some(addr.get_offset());
    }
    // Walk the remaining children selectively: inside `<ast>`, varnode
    // create-indices come from the `<varnodes>` declarations only (see the
    // `ast_var_refs` field doc) while op times come from the `<block>` bodies.
    loop {
        let c = dec.peek_element().expect("peek <function> child");
        if c == 0 {
            break;
        }
        let cid = dec.open_element().expect("open <function> child");
        parsed.function_child_ids.push(cid);
        if cid == ELEM_AST_ID {
            skip_attributes(dec).expect("<ast> attributes");
            loop {
                let a = dec.peek_element().expect("peek <ast> child");
                if a == 0 {
                    break;
                }
                let aid = dec.open_element().expect("open <ast> child");
                let mut got = BTreeMap::new();
                if aid == ELEM_VARNODES_ID {
                    walk(dec, &[ATTRIB_REF_ID], &mut got).expect("walk <varnodes>");
                    parsed
                        .ast_var_refs
                        .extend(got.get(&ATTRIB_REF_ID).cloned().unwrap_or_default());
                } else {
                    walk(dec, &[ATTRIB_UNIQ_ID], &mut got).expect("walk <ast> child");
                    parsed
                        .ast_op_times
                        .extend(got.get(&ATTRIB_UNIQ_ID).cloned().unwrap_or_default());
                }
                dec.close_element(aid).expect("close <ast> child");
            }
        } else if cid == ELEM_LOCALDB_ID {
            parse_localdb(dec, parsed);
        } else if cid == ELEM_HIGHLIST_ID {
            parse_highlist(dec, parsed);
        } else if cid == ELEM_PROTOTYPE_ID {
            parse_prototype(dec, parsed);
        } else if cid == ELEM_JUMPTABLELIST_ID {
            parse_jumptablelist(dec, parsed);
        } else if cid == ELEM_OVERRIDE_ID || cid == ELEM_SCOPE_ID {
            // Java skips both.
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk skipped <function> child");
        } else {
            // HighFunction.decode: "Unknown element in function" — hard throw.
            panic!("unknown <function> child element id {cid}");
        }
        dec.close_element(cid).expect("close <function> child");
    }
}

/// `<localdb>` (opened): `<scope>` whose first two children are positionally
/// `<parent>` + `<rangelist>`, then `<symbollist>` of `<mapsym>`s.
fn parse_localdb(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    skip_attributes(dec).expect("<localdb> attributes");
    let mut symbols: Vec<SimSymbol> = Vec::new();
    while dec.peek_element().expect("peek <localdb> child") != 0 {
        let sid = dec.open_element().expect("open <localdb> child");
        if sid != ELEM_SCOPE_ID {
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk non-scope localdb child");
            dec.close_element(sid).expect("close non-scope localdb child");
            continue;
        }
        skip_attributes(dec).expect("<scope> attributes");
        // Record the first two child element ids (the blind positional skip).
        let mut child_pos = 0usize;
        while dec.peek_element().expect("peek <scope> child") != 0 {
            let cid = dec.open_element().expect("open <scope> child");
            if child_pos < 2 {
                parsed.scope_prefix.push(cid);
            }
            child_pos += 1;
            if cid == ELEM_SYMBOLLIST_ID {
                while dec.peek_element().expect("peek <symbollist> child") != 0 {
                    let ms = dec.open_element().expect("open <mapsym>");
                    assert_eq!(ms, ELEM_MAPSYM_ID, "<symbollist> child is not <mapsym>");
                    symbols.push(parse_mapsym(dec));
                    dec.close_element(ms).expect("close <mapsym>");
                }
            } else {
                let mut sink = BTreeMap::new();
                walk(dec, &[], &mut sink).expect("walk <scope> child");
            }
            dec.close_element(cid).expect("close <scope> child");
        }
        dec.close_element(sid).expect("close <scope>");
    }
    parsed.localdb = Some(symbols);
}

/// One `<mapsym>` (opened): the `<symbol>`/`<equatesymbol>` header + the
/// SymbolEntry pairs ((`<addr>`|`<hash>`) + `<rangelist>`).
fn parse_mapsym(dec: &mut PackedDecode) -> SimSymbol {
    skip_attributes(dec).expect("<mapsym> attributes");
    let mut sym = SimSymbol {
        id: 0,
        cat: -1,
        index: None,
        name: String::new(),
        entries: 0,
        entries_have_rangelists: true,
        has_type: false,
        type_size: None,
        type_name: None,
        type_is_void: false,
        entry_storage: None,
        first_use: None,
        hash: 0,
    };
    let mut pending_entry_needs_rangelist = false;
    while dec.peek_element().expect("peek <mapsym> child") != 0 {
        let cid = dec.open_element().expect("open <mapsym> child");
        if cid == ELEM_SYMBOL_ID || cid == ELEM_EQUATESYMBOL_ID {
            loop {
                let aid = dec.get_next_attribute_id().expect("symbol attr");
                if aid == 0 {
                    break;
                }
                if aid == ATTRIB_ID_ID {
                    sym.id = dec.read_unsigned_integer().expect("symbol id");
                } else if aid == ATTRIB_CAT_ID {
                    sym.cat = dec.read_signed_integer().expect("symbol cat");
                } else if aid == ATTRIB_INDEX_ID {
                    sym.index = Some(dec.read_unsigned_integer().expect("symbol index"));
                } else if aid == ATTRIB_NAME.get_id() {
                    sym.name = String::from_utf8_lossy(
                        &dec.read_string().expect("symbol name"),
                    )
                    .into_owned();
                }
            }
            // Body: the data-type child (plus <value> for an equate).
            while dec.peek_element().expect("peek <symbol> child") != 0 {
                let b = dec.open_element().expect("open <symbol> body child");
                if b == ELEM_TYPE_ID || b == ELEM_TYPEREF_ID || b == ELEM_DEF_ID
                    || b == ELEM_VOID_ID
                {
                    sym.has_type = true;
                    if b == ELEM_VOID_ID {
                        sym.type_is_void = true;
                    }
                    if b == ELEM_TYPE_ID || b == ELEM_TYPEREF_ID {
                        loop {
                            let aid = dec.get_next_attribute_id().expect("type attr");
                            if aid == 0 {
                                break;
                            }
                            if aid == kuna_base::marshal::ATTRIB_SIZE.get_id() {
                                sym.type_size =
                                    Some(dec.read_signed_integer().expect("type size"));
                            } else if aid == ATTRIB_NAME.get_id() {
                                sym.type_name = Some(
                                    String::from_utf8_lossy(
                                        &dec.read_string().expect("type name"),
                                    )
                                    .into_owned(),
                                );
                            }
                        }
                    }
                }
                let mut sink = BTreeMap::new();
                walk(dec, &[], &mut sink).expect("walk <symbol> body child");
                dec.close_element(b).expect("close <symbol> body child");
            }
        } else if cid == ELEM_ADDR_ID || cid == ELEM_HASH_ID {
            sym.entries += 1;
            if cid == ELEM_HASH_ID && sym.hash == 0 {
                sym.hash = dec
                    .read_unsigned_integer_id(&kuna_base::marshal::ATTRIB_VAL)
                    .unwrap_or(0);
            }
            if pending_entry_needs_rangelist {
                sym.entries_have_rangelists = false; // previous entry had none
            }
            pending_entry_needs_rangelist = true;
            if cid == ELEM_ADDR_ID && sym.entry_storage.is_none() {
                // Capture (space, offset) of the first mapped entry (the
                // <addr> is unsized; the size comes from the type child).
                let (mut spc, mut off) = (None, 0u64);
                loop {
                    let aid = dec.get_next_attribute_id().expect("entry attr");
                    if aid == 0 {
                        break;
                    }
                    if aid == kuna_base::marshal::ATTRIB_SPACE.get_id() {
                        spc = dec.read_space().ok();
                    } else if aid == kuna_base::marshal::ATTRIB_OFFSET.get_id() {
                        off = dec.read_unsigned_integer().expect("entry offset");
                    }
                }
                if let Some(s) = spc {
                    sym.entry_storage = Some((s.get_name().to_string(), off));
                }
            }
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk entry");
        } else if cid == ELEM_RANGELIST_ID {
            pending_entry_needs_rangelist = false;
            if sym.first_use.is_none() {
                let mut got = BTreeMap::new();
                walk(dec, &[ATTRIB_FIRST_ID], &mut got).expect("walk entry rangelist");
                sym.first_use =
                    got.get(&ATTRIB_FIRST_ID).and_then(|s| s.iter().next().copied());
            } else {
                let mut sink = BTreeMap::new();
                walk(dec, &[], &mut sink).expect("walk entry rangelist");
            }
        } else {
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk unknown mapsym child");
        }
        dec.close_element(cid).expect("close <mapsym> child");
    }
    if pending_entry_needs_rangelist {
        sym.entries_have_rangelists = false;
    }
    sym
}

/// `<highlist>` (opened): the `<high>` headers.
fn parse_highlist(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    skip_attributes(dec).expect("<highlist> attributes");
    let mut highs: Vec<SimHigh> = Vec::new();
    while dec.peek_element().expect("peek <highlist> child") != 0 {
        let hid = dec.open_element().expect("open <high>");
        assert_eq!(hid, ELEM_HIGH_ID, "<highlist> child is not <high>");
        let mut h = SimHigh { class: String::new(), repref: None, symref: None };
        loop {
            let aid = dec.get_next_attribute_id().expect("high attr");
            if aid == 0 {
                break;
            }
            if aid == ATTRIB_CLASS_ID {
                h.class = String::from_utf8_lossy(&dec.read_string().expect("high class"))
                    .into_owned();
            } else if aid == ATTRIB_REPREF_ID {
                h.repref = Some(dec.read_unsigned_integer().expect("high repref"));
            } else if aid == ATTRIB_SYMREF_ID {
                h.symref = Some(dec.read_unsigned_integer().expect("high symref"));
            }
        }
        let mut sink = BTreeMap::new();
        walk(dec, &[], &mut sink).expect("walk <high> children");
        dec.close_element(hid).expect("close <high>");
        highs.push(h);
    }
    parsed.highlist = Some(highs);
}

/// `<prototype>` (opened): the model/extrapop header + the `<returnsym>`.
fn parse_prototype(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    let mut proto = SimProto { model: String::new(), has_extrapop: false, returnsym_ok: false };
    loop {
        let aid = dec.get_next_attribute_id().expect("prototype attr");
        if aid == 0 {
            break;
        }
        if aid == ATTRIB_MODEL_ID {
            proto.model =
                String::from_utf8_lossy(&dec.read_string().expect("prototype model")).into_owned();
        } else if aid == ATTRIB_EXTRAPOP_ID {
            proto.has_extrapop = true;
        }
    }
    while dec.peek_element().expect("peek <prototype> child") != 0 {
        let cid = dec.open_element().expect("open <prototype> child");
        if cid == ELEM_RETURNSYM_ID {
            // FunctionPrototype.decodePrototype: <addr> first child, then a
            // data-type child.
            skip_attributes(dec).expect("<returnsym> attributes");
            let mut saw_addr = false;
            let mut saw_type = false;
            let mut first = true;
            while dec.peek_element().expect("peek <returnsym> child") != 0 {
                let b = dec.open_element().expect("open <returnsym> child");
                if first {
                    saw_addr = b == ELEM_ADDR_ID;
                    first = false;
                } else if b == ELEM_TYPE_ID
                    || b == ELEM_TYPEREF_ID
                    || b == ELEM_DEF_ID
                    || b == ELEM_VOID_ID
                {
                    saw_type = true;
                }
                let mut sink = BTreeMap::new();
                walk(dec, &[], &mut sink).expect("walk <returnsym> child");
                dec.close_element(b).expect("close <returnsym> child");
            }
            proto.returnsym_ok = saw_addr && saw_type;
        } else {
            // Effect lists / likelytrash / inject / internallist: Java skips.
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk <prototype> child");
        }
        dec.close_element(cid).expect("close <prototype> child");
    }
    parsed.prototype = Some(proto);
}

/// `<jumptablelist>` (opened): per `<jumptable>` the dest/loadtable counts.
fn parse_jumptablelist(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    skip_attributes(dec).expect("<jumptablelist> attributes");
    while dec.peek_element().expect("peek <jumptablelist> child") != 0 {
        let jid = dec.open_element().expect("open <jumptable>");
        assert_eq!(jid, ELEM_JUMPTABLE_ID, "<jumptablelist> child is not <jumptable>");
        skip_attributes(dec).expect("<jumptable> attributes");
        let mut dests = 0usize;
        let mut loads = 0usize;
        while dec.peek_element().expect("peek <jumptable> child") != 0 {
            let cid = dec.open_element().expect("open <jumptable> child");
            if cid == ELEM_DEST_ID {
                dests += 1;
            } else if cid == ELEM_LOADTABLE_ID {
                loads += 1;
            }
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk <jumptable> child");
            dec.close_element(cid).expect("close <jumptable> child");
        }
        parsed.jumptables.push((dests, loads));
        dec.close_element(jid).expect("close <jumptable>");
    }
}

/// `<parammeasures>` (opened): per input/output measure, whether the REQUIRED
/// `<rank>` child is present.
fn parse_parammeasures(dec: &mut PackedDecode, parsed: &mut ParsedDoc) {
    skip_attributes(dec).expect("<parammeasures> attributes");
    let mut measures: Vec<(bool, bool)> = Vec::new();
    while dec.peek_element().expect("peek <parammeasures> child") != 0 {
        let cid = dec.open_element().expect("open <parammeasures> child");
        if cid == ELEM_INPUT_ID || cid == ELEM_OUTPUT_ID {
            let is_input = cid == ELEM_INPUT_ID;
            skip_attributes(dec).expect("measure attributes");
            let mut has_rank = false;
            while dec.peek_element().expect("peek measure child") != 0 {
                let b = dec.open_element().expect("open measure child");
                if b == ELEM_RANK_ID {
                    has_rank = true;
                }
                let mut sink = BTreeMap::new();
                walk(dec, &[], &mut sink).expect("walk measure child");
                dec.close_element(b).expect("close measure child");
            }
            measures.push((is_input, has_rank));
        } else {
            // <addr> / <proto>.
            let mut sink = BTreeMap::new();
            walk(dec, &[], &mut sink).expect("walk <parammeasures> child");
        }
        dec.close_element(cid).expect("close <parammeasures> child");
    }
    parsed.parammeasures = Some(measures);
}

/// Count `<vardecl symref>`s that do NOT resolve against the document's own
/// `<localdb>` ids (the create-index fallback for a declaration whose high the
/// analysis left symbol-less).
pub fn vardecl_unresolved(parsed: &ParsedDoc) -> usize {
    let ids: BTreeSet<u64> = parsed
        .localdb
        .as_ref()
        .map(|syms| syms.iter().map(|s| s.id).collect())
        .unwrap_or_default();
    parsed
        .vardecl_symrefs
        .iter()
        .filter(|sr| !ids.contains(*sr))
        .count()
}

/// The Phase-4 Java hard-throw traps (r5 §3), asserted the way the Java
/// consumers would fail: one violation would discard the WHOLE decompile
/// result in the GUI.
pub fn assert_phase4_traps(parsed: &ParsedDoc, label: &str) {
    let _ = &parsed.vardecl_symref_unresolved;
    // Child order: localdb before highlist, ast before highlist.
    let pos = |id: u32| parsed.function_child_ids.iter().position(|&c| c == id);
    if let Some(hpos) = pos(ELEM_HIGHLIST_ID) {
        let lpos = pos(ELEM_LOCALDB_ID)
            .unwrap_or_else(|| panic!("{label}: <highlist> present without <localdb>"));
        assert!(lpos < hpos, "{label}: <localdb> must precede <highlist>");
        let apos = pos(ELEM_AST_ID)
            .unwrap_or_else(|| panic!("{label}: <highlist> present without <ast>"));
        assert!(apos < hpos, "{label}: <ast> must precede <highlist>");
    }
    // The scope's first two children are positionally <parent> + <rangelist>.
    if parsed.localdb.is_some() {
        assert_eq!(
            parsed.scope_prefix,
            vec![ELEM_PARENT_ID, ELEM_RANGELIST_ID],
            "{label}: <scope> must open with <parent> then <rangelist> \
             (LocalSymbolMap.decodeScope skips both blind)"
        );
    }
    // Symbols: nonzero ids, >=1 entry with rangelists, cat-0 carries an index,
    // every symbol has a type child.
    let mut localdb_ids: BTreeSet<u64> = BTreeSet::new();
    if let Some(symbols) = &parsed.localdb {
        for s in symbols {
            assert_ne!(
                s.id, 0,
                "{label}: <symbol {}> id 0 — HighSymbol.decodeHeader throws",
                s.name
            );
            assert!(
                s.entries >= 1,
                "{label}: <mapsym {}> has no SymbolEntry — insertSymbol NPEs",
                s.name
            );
            assert!(
                s.entries_have_rangelists,
                "{label}: <mapsym {}> entry missing its uselimit <rangelist>",
                s.name
            );
            assert!(s.has_type, "{label}: <symbol {}> carries no data-type child", s.name);
            // r5 §3 trap 4: `MappedEntry.decode` throws
            // "Invalid symbol 0-sized data-type" — a `<void/>` body or an
            // explicit size 0 is exactly that shape.
            assert!(
                s.type_size.map(|sz| sz > 0).unwrap_or(true) && !s.type_is_void,
                "{label}: <symbol {}> has a 0-sized data-type (MappedEntry.decode throws)",
                s.name
            );
            if s.cat == 0 {
                assert!(
                    s.index.is_some(),
                    "{label}: cat-0 <symbol {}> without an index — the param slot \
                     sort (and the rename full-commit guard) needs it",
                    s.name
                );
            }
            localdb_ids.insert(s.id);
        }
    }
    // Highs: legal class vocabulary; local/param symrefs resolve in the
    // just-decoded localdb; reprefs resolve in the just-decoded ast.
    if let Some(highs) = &parsed.highlist {
        for h in highs {
            assert!(
                matches!(h.class.as_str(), "other" | "global" | "constant" | "param" | "local"),
                "{label}: unknown <high> class '{}'",
                h.class
            );
            let repref = h
                .repref
                .unwrap_or_else(|| panic!("{label}: <high> without repref"));
            assert!(
                parsed.ast_var_refs.contains(&repref),
                "{label}: <high repref={repref}> not declared in <ast> varnodes \
                 (decodeInstances throws)"
            );
            if h.class == "local" || h.class == "param" {
                let symref = h.symref.unwrap_or_else(|| {
                    panic!("{label}: <high class={}> without symref (HighLocal throws)", h.class)
                });
                assert!(
                    localdb_ids.contains(&symref),
                    "{label}: <high class={} symref={symref}> not present in <localdb> \
                     — 'HighLocal is missing symbol'",
                    h.class
                );
            }
        }
    }
    // Every `<vardecl symref>` must resolve in the just-decoded `<localdb>`:
    // an unresolvable one leaves declaration-line rename/retype dead and logs
    // once per declaration per decompile.
    if !parsed.vardecl_symrefs.is_empty() {
        assert!(
            !localdb_ids.is_empty(),
            "{label}: declarations carry symrefs but the <localdb> has no symbols"
        );
        let resolved = parsed
            .vardecl_symrefs
            .iter()
            .filter(|sr| localdb_ids.contains(*sr))
            .count();
        assert!(
            resolved > 0,
            "{label}: NO <vardecl symref> resolves against <localdb> — the \
             declaration-line rename/retype contract is broken wholesale"
        );
    }
    // Prototype: model + extrapop + returnsym(addr+type).
    if let Some(p) = &parsed.prototype {
        assert!(!p.model.is_empty(), "{label}: <prototype> without model");
        assert!(p.has_extrapop, "{label}: <prototype> without extrapop");
        assert!(
            p.returnsym_ok,
            "{label}: <returnsym> must carry <addr> first then a data-type child"
        );
    }
    // Parammeasures: the <rank> child is REQUIRED per measure.
    if let Some(measures) = &parsed.parammeasures {
        for (i, (_is_input, has_rank)) in measures.iter().enumerate() {
            assert!(
                has_rank,
                "{label}: parammeasures measure #{i} without <rank> (ParamMeasure.decode throws)"
            );
        }
    }
}

/// Ordered flatten of the (already-opened) markup element: append every
/// `content` attribute string, render `<break indent=N>` as `'\n' + N spaces`,
/// and collect the `opref`/`varref` attributes on the way.
///
/// Identifier-class tokens (funcname/variable/type/field/label, except
/// CONST_COLOR ones) are passed through [`illegal_char_cpp_transform`] first —
/// exactly what Java's `DecompileResults.getDecompiledFunction()` does
/// (`PrettyPrinter.getText` + `IllegalCharCppTransformer`), so the flattened C
/// is what a script/export consumer actually receives, mangling included.
fn flatten_markup(
    dec: &mut PackedDecode,
    elem_id: u32,
    out: &mut String,
    refs: &mut BTreeMap<u32, BTreeSet<u64>>,
    mangled: &mut usize,
) -> Result<(), KunaError> {
    let mut indent: i64 = 0;
    let mut color: Option<u64> = None;
    let mut content: Option<String> = None;
    loop {
        let aid = dec.get_next_attribute_id()?;
        if aid == 0 {
            break;
        }
        if aid == ATTRIB_SYMREF_ID && elem_id == ELEM_VARDECL_ID {
            let v = dec.read_unsigned_integer()?;
            refs.entry(ATTRIB_SYMREF_ID).or_default().insert(v);
            continue;
        }
        if aid == ATTRIB_CONTENT.get_id() {
            content = Some(String::from_utf8_lossy(&dec.read_string()?).into_owned());
        } else if aid == ATTRIB_INDENT_ID {
            indent = dec.read_signed_integer()?;
        } else if aid == ATTRIB_COLOR_ID {
            color = Some(dec.read_unsigned_integer()?);
        } else if aid == ATTRIB_OPREF_ID || aid == ATTRIB_VARREF_ID {
            let v = dec.read_unsigned_integer()?;
            refs.entry(aid).or_default().insert(v);
        }
    }
    if let Some(text) = content {
        let is_token_to_clean = matches!(
            elem_id,
            ELEM_FUNCNAME_ID | ELEM_VARIABLE_ID | ELEM_TYPE_ID | ELEM_FIELD_ID | ELEM_LABEL_ID
        ) && color != Some(CONST_COLOR);
        if is_token_to_clean {
            let cleaned = illegal_char_cpp_transform(&text);
            if cleaned != text {
                *mangled += 1;
            }
            out.push_str(&cleaned);
        } else {
            out.push_str(&text);
        }
    }
    if elem_id == ELEM_BREAK_ID {
        out.push('\n');
        for _ in 0..indent.max(0) {
            out.push(' ');
        }
    }
    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        let id = dec.open_element()?;
        flatten_markup(dec, id, out, refs, mangled)?;
        dec.close_element(id)?;
    }
    Ok(())
}

/// Java's `IllegalCharCppTransformer.simplify` (Features/Decompiler,
/// `DecompileResults.getDecompiledFunction()`'s token cleaner), transcribed:
/// any character illegal in a C++ identifier becomes `_`, with carve-outs for
/// digits/`_` after the first character, template parameters inside `<…>`,
/// `operator` punctuation at positions 8..=10, and a leading `~`.
pub fn illegal_char_cpp_transform(input: &str) -> String {
    const AFTER_FIRST: u8 = 1;
    const TEMPLATE: u8 = 2;
    const OPERATOR: u8 = 4;
    const FIRST: u8 = 8;
    fn legal(c: char) -> u8 {
        match c {
            '_' => AFTER_FIRST | TEMPLATE | OPERATOR | FIRST,
            '0'..='9' => AFTER_FIRST | TEMPLATE | OPERATOR,
            '*' | '(' | ')' | '[' | ']' | '&' => TEMPLATE | OPERATOR,
            ':' | ',' => TEMPLATE,
            '+' | '-' | '|' | '=' | '!' | '/' | '%' | '^' => OPERATOR,
            '~' => TEMPLATE | OPERATOR | FIRST,
            _ => 0,
        }
    }
    let mut template_depth = 0i32;
    let mut changed = false;
    let mut outbuf: Vec<char> = input.chars().collect();
    for (i, c) in input.chars().enumerate() {
        if c.is_alphabetic() {
            continue;
        }
        if c == '<' {
            template_depth += 1;
            continue;
        }
        if c == '>' {
            template_depth = (template_depth - 1).max(0);
            continue;
        }
        let val = legal(c);
        if val != 0 {
            if (val & AFTER_FIRST) != 0 && i > 0 {
                continue;
            }
            if (val & FIRST) != 0 && i == 0 {
                continue;
            }
            if (val & TEMPLATE) != 0 && template_depth > 0 {
                continue;
            }
            if (val & OPERATOR) != 0 && (8..=10).contains(&i) && input.starts_with("operator") {
                continue;
            }
        }
        outbuf[i] = '_';
        changed = true;
    }
    if changed {
        outbuf.into_iter().collect()
    } else {
        input.to_string()
    }
}

// ---------------------------------------------------------------------------
// Static badness scanners (GUI-brokenness detectors) + differential metric
// ---------------------------------------------------------------------------

/// Split C text into identifier tokens (`[A-Za-z_][A-Za-z0-9_]*`).
pub fn identifiers(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            out.push(&text[start..i]);
        } else {
            i += 1;
        }
    }
    out
}

/// Raw-register-identifier leak: identifiers that exactly match a register
/// name of the language (case-sensitive — the CLI's lowercase `// rax`
/// location comments never collide).  Returns (occurrence count, distinct
/// register names seen).
pub fn register_leaks(c: &str, registers: &BTreeSet<String>) -> (usize, BTreeSet<String>) {
    let mut count = 0usize;
    let mut seen = BTreeSet::new();
    for id in identifiers(c) {
        if registers.contains(id) {
            count += 1;
            seen.insert(id.to_string());
        }
    }
    (count, seen)
}

/// `Unique<hex>` token leak: identifiers of the un-named unique-storage
/// rendering (`kuna_unnamed_location_name`), e.g. `Unique00023e00`.  Counts
/// `Stack<hex>` tokens too — the same no-symbol print tail produces both.
pub fn unique_leaks(c: &str) -> usize {
    identifiers(c)
        .into_iter()
        .filter(|id| {
            for prefix in ["Unique", "Stack"] {
                if let Some(rest) = id.strip_prefix(prefix) {
                    if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit()) {
                        return true;
                    }
                }
            }
            false
        })
        .count()
}

/// Placeholder identifiers by kind: distinct addresses appearing as
/// `sub_<hex>` / `FUN_<hex>` / `dat_<hex>` / `DAT_<hex>`.
pub fn placeholder_addrs(c: &str) -> BTreeMap<&'static str, BTreeSet<u64>> {
    let mut out: BTreeMap<&'static str, BTreeSet<u64>> = BTreeMap::new();
    for id in identifiers(c) {
        for kind in ["sub_", "FUN_", "dat_", "DAT_"] {
            if let Some(rest) = id.strip_prefix(kind) {
                if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit()) {
                    if let Ok(addr) = u64::from_str_radix(rest, 16) {
                        out.entry(kind).or_default().insert(addr);
                    }
                }
            }
        }
    }
    out
}

/// Normalize C text for the line diff: per-line trim, drop empty lines.
pub fn normalized_lines(c: &str) -> Vec<String> {
    c.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|l| !l.is_empty())
        .collect()
}

/// Line-level diff ratio after whitespace normalization: `1 - 2*LCS/(|a|+|b|)`.
/// 0.0 = identical, 1.0 = nothing in common.  This is the per-function gap the
/// GUI user experiences between the ghidra-mode markup C and the CLI C.
pub fn line_diff_ratio(a: &str, b: &str) -> f64 {
    let la = normalized_lines(a);
    let lb = normalized_lines(b);
    if la.is_empty() && lb.is_empty() {
        return 0.0;
    }
    // Classic O(n*m) LCS over lines — the fixtures are small functions.
    let n = la.len();
    let m = lb.len();
    let mut prev = vec![0usize; m + 1];
    let mut cur = vec![0usize; m + 1];
    for i in 1..=n {
        for j in 1..=m {
            cur[j] = if la[i - 1] == lb[j - 1] {
                prev[j - 1] + 1
            } else {
                prev[j].max(cur[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    let lcs = prev[m];
    1.0 - (2.0 * lcs as f64) / ((n + m) as f64)
}
