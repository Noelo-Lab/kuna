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
/// `ClangToken.CONST_COLOR` (== `SyntaxHighlight::ConstColor`).
pub const CONST_COLOR: u64 = 5;

/// The 19 legal callback-query command element ids (ids.rs 239..=257).
pub const QUERY_COMMAND_IDS: std::ops::RangeInclusive<u32> = 239..=257;

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
}

/// Decode a `decompileAt` `<doc>` payload: the first `<function>`'s name/entry
/// echo + `<ast>` refs, and the second (markup) `<function>` flattened to C
/// with its opref/varref sets.
pub fn parse_decompile_doc(doc: &[u8], manager: &AddrSpaceManager) -> ParsedDoc {
    let mut parsed = ParsedDoc::default();
    let mut dec = PackedDecode::new(manager);
    dec.ingest_stream(doc).expect("<doc> ingests");
    let did = dec.open_element().expect("open <doc>");
    assert_eq!(did, ELEM_DOC.get_id(), "root is not <doc>");

    // --- the first <function>: the Funcdata::encode <function>/<ast> ---
    let fid = dec.open_element().expect("open <function>");
    assert_eq!(fid, ELEM_FUNCTION_ID, "first child of <doc> is not <function>");
    parsed.name = String::from_utf8_lossy(
        &dec.read_string_id(&ATTRIB_NAME).expect("<function name>"),
    )
    .into_owned();
    // The base address is the first child element.
    if dec.peek_element().expect("peek in <function>") == ELEM_ADDR.get_id() {
        let addr = Address::decode(&mut dec).expect("<function> base <addr>");
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
        if cid == ELEM_AST_ID {
            skip_attributes(&mut dec).expect("<ast> attributes");
            loop {
                let a = dec.peek_element().expect("peek <ast> child");
                if a == 0 {
                    break;
                }
                let aid = dec.open_element().expect("open <ast> child");
                let mut got = BTreeMap::new();
                if aid == ELEM_VARNODES_ID {
                    walk(&mut dec, &[ATTRIB_REF_ID], &mut got).expect("walk <varnodes>");
                    parsed
                        .ast_var_refs
                        .extend(got.get(&ATTRIB_REF_ID).cloned().unwrap_or_default());
                } else {
                    walk(&mut dec, &[ATTRIB_UNIQ_ID], &mut got).expect("walk <ast> child");
                    parsed
                        .ast_op_times
                        .extend(got.get(&ATTRIB_UNIQ_ID).cloned().unwrap_or_default());
                }
                dec.close_element(aid).expect("close <ast> child");
            }
        } else {
            // Some other (future: localdb/prototype/…) child: skip through.
            let mut sink = BTreeMap::new();
            walk(&mut dec, &[], &mut sink).expect("walk <function> child");
        }
        dec.close_element(cid).expect("close <function> child");
    }
    dec.close_element(fid).expect("close <function>");

    // --- the second <function>: the Clang token markup ---
    if dec.peek_element().expect("peek after <function>") == ELEM_FUNCTION_ID {
        parsed.has_markup = true;
        let mid = dec.open_element().expect("open markup <function>");
        let mut refs = BTreeMap::new();
        let mut c = String::new();
        let mut mangled = 0usize;
        flatten_markup(&mut dec, mid, &mut c, &mut refs, &mut mangled)
            .expect("flatten markup <function>");
        dec.close_element(mid).expect("close markup <function>");
        parsed.markup_oprefs = refs.get(&ATTRIB_OPREF_ID).cloned().unwrap_or_default();
        parsed.markup_varrefs = refs.get(&ATTRIB_VARREF_ID).cloned().unwrap_or_default();
        parsed.c_text = c;
        parsed.mangled_tokens = mangled;
    }
    dec.close_element(did).expect("close <doc>");
    parsed
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
