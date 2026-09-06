//! Port of `decompiler/cpp/xml.hh` + `xml.cc` (generated from `xml.y`) — the
//! lightweight (and incomplete) XML parser used for marshaling data to and
//! from the decompiler (W1, item `w1-base-xml`).
//!
//! Per LOSS-006 (`docs/rust-port/losses.md`) the bison-generated automaton is
//! replaced by a hand-written parser that reproduces the *grammar semantics*
//! of `xml.y` exactly — the same accepted language (including the quirks
//! below), the same scanner-mode protocol, the same SAX callback sequence,
//! and the same error strings:
//!
//! - The scanner ([`XmlScan`]) is a line-for-line transcription, including
//!   the 4-byte lookahead, the synthetic `'\n'` injected at end of stream, a
//!   NUL byte acting as end of stream, and the **signed-char** semantics of
//!   the oracle platform (bytes >= 0x80 become negative lookahead values, so
//!   they fail `isChar` in comments/CDATA, and byte 0xFF is indistinguishable
//!   from end of stream — exactly as on x86-64 g++).
//! - bison maps any token value <= 0 to `$end`; the hand parser does too.
//! - The 8 declared shift/reduce conflicts (`%expect 8`) all resolve as
//!   shifts: whitespace runs are absorbed greedily, and `<?x` at the very
//!   start of the document is committed to the `<?xml ...?>` declaration.
//! - Quirks preserved deliberately: processing instructions are rejected
//!   ("Processing instructions are not supported"), DTDs are rejected
//!   ("DTD's not supported") — but a `<!DOCTYPE` as the *first* thing in the
//!   document is a plain "syntax error" because `doctypepro` only follows a
//!   non-empty `prologpre`; a closing tag's name is **not** checked against
//!   its opening tag (`xml.y:191` discards it); the document grammar accepts
//!   exactly one trailing `Misc` after the root element, so a trailing
//!   comment can never parse (the synthetic `'\n'` always follows it);
//!   unknown entity references decode to byte 0xFF (`(char)-1`); character
//!   references append only the **low byte** of the converted value.
//! - Content and attribute values are **byte strings** (`Vec<u8>`), exactly
//!   like the C++ `std::string`: datatest `<bytechunk>` payloads and
//!   reference-decoded bytes (e.g. `&#xc2;&#xa3;`) round-trip byte-for-byte
//!   and need not be valid UTF-8.  Element and attribute *names* are
//!   scanner-restricted to ASCII and exposed as `&str`.
//! - All-whitespace content runs go to `ContentHandler::ignorable_whitespace`
//!   (which the DOM builder drops), mirroring `print_content` in `xml.y`.
//!
//! Deliberate API departures (all invisible to the C++ oracle's tests):
//!
//! - Input is `&[u8]` instead of `std::istream` (callers read files/strings
//!   into memory first; `DocumentStorage::open_document` does the read).
//! - `Element::getParent` is not ported: the Rust DOM is an owned tree
//!   (children held as `Rc<Element>` so `DocumentStorage::register_tag` can
//!   alias into it).  No vendored C++ outside `xml.cc` calls `getParent`.
//! - `ContentHandler::setDocumentLocator` is not ported (`Locator` is a
//!   `void*` placeholder upstream and the parser never invokes it).
//! - `xml_parse`'s `dbg` parameter (bison's `yydebug` trace) is dropped.
//! - `DecoderError` is [`KunaError::Decoder`] (see `error.rs`): it stays
//!   *outside* the C++ `LowlevelError` hierarchy.
//!
//! Nesting depth note (LOSS-010): bison's parse stack grows on the heap up
//! to a hard cap (`YYMAXDEPTH` 10000, xml.cc:971), so the C++ parser rejects
//! deeply nested documents cleanly — element nesting to depth 4997 parses,
//! depth >= 4998 fails with "memory exhausted" and `xml_parse` returns 2
//! (xml.cc:2042-2043).  The hand parser is *iterative* over element nesting
//! (an explicit open-element stack) and does **not** emulate the cap:
//! arbitrarily deep documents are accepted, a documented accept/reject
//! divergence on pathological input (`docs/rust-port/losses.md` LOSS-010).
//! Teardown is likewise non-recursive ([`Element`]'s worklist `Drop`), so
//! neither parsing nor dropping a deep DOM can overflow the Rust call stack.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::error::{KunaError, KunaResult};
use crate::types::Wrap;

// ---------------------------------------------------------------------------
// Token codes (XmlScan::token, xml.y:45-53). Values <= 255 are byte values.
// ---------------------------------------------------------------------------

const CHARDATA_TOK: i32 = 258;
const CDATA_TOK: i32 = 259;
const ATTVALUE_TOK: i32 = 260;
const COMMENT_TOK: i32 = 261;
const CHARREF_TOK: i32 = 262;
const NAME_TOK: i32 = 263;
const SNAME_TOK: i32 = 264;
const ELEMBRACE_TOK: i32 = 265;
const COMMBRACE_TOK: i32 = 266;
/// bison `$end`: every token value `<= 0` is normalized to this (the
/// generated parser does `if (yychar <= YYEOF) yytoken = YYEOF`).
const TOK_EOF: i32 = 0;

/// The error message bison emits for any plain parse error
/// (`YYERROR_VERBOSE` is 0 in the generated `xml.cc`).
const SYNTAX_ERROR: &str = "syntax error";

/// `Attributes::bogus_uri` — placeholder namespace URI (`xml.y:31`).
pub const BOGUS_URI: &str = "http://unused.uri";

#[inline]
const fn ch(c: u8) -> i32 {
    c as i32
}

#[inline]
fn is_whitespace_tok(t: i32) -> bool {
    // grammar `whitespace: ' ' | '\n' | '\r' | '\t'` (xml.y:143-146)
    t == ch(b' ') || t == ch(b'\n') || t == ch(b'\r') || t == ch(b'\t')
}

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

/// The \e attributes for a single XML element.
///
/// A container for name/value pairs for the formal attributes, as collected
/// during parsing.  This object is used to initialize the [`Element`] object
/// but is not part of the final, in-memory DOM model.  Attribute *values* are
/// byte strings (see module docs).
pub struct Attributes {
    /// The name of the XML element
    elementname: String,
    /// List of names for each formal XML attribute
    name: Vec<String>,
    /// List of values for each formal XML attribute
    value: Vec<Vec<u8>>,
}

impl Attributes {
    /// Construct from element name (C++ `Attributes(string *el)`).
    pub fn new(el: String) -> Self {
        Attributes { elementname: el, name: Vec::new(), value: Vec::new() }
    }

    /// Get the namespace URI associated with this element (always the
    /// placeholder [`BOGUS_URI`], as in C++).
    pub fn get_elem_uri(&self) -> &str {
        BOGUS_URI
    }

    /// Get the name of this element.
    pub fn get_elem_name(&self) -> &str {
        &self.elementname
    }

    /// Add a formal attribute.
    pub fn add_attribute(&mut self, nm: String, vl: Vec<u8>) {
        self.name.push(nm);
        self.value.push(vl);
    }

    // The official SAX interface

    /// Get the number of attributes associated with the element.
    pub fn get_length(&self) -> i32 {
        // cast: size_t -> int4 narrowing, as the C++ `getLength` return
        self.name.len() as i32
    }

    /// Get the namespace URI associated with the i-th attribute (always the
    /// placeholder [`BOGUS_URI`], as in C++).
    pub fn get_uri(&self, _i: i32) -> &str {
        BOGUS_URI
    }

    /// Get the local name of the i-th attribute.
    pub fn get_local_name(&self, i: i32) -> &str {
        &self.name[i as usize]
    }

    /// Get the qualified name of the i-th attribute.
    pub fn get_qname(&self, i: i32) -> &str {
        &self.name[i as usize]
    }

    /// Get the value of the i-th attribute.
    pub fn get_value(&self, i: i32) -> &[u8] {
        &self.value[i as usize]
    }

    /// Get the value of the attribute with the given qualified name.
    ///
    /// C++ quirk preserved: a missing attribute returns the *bogus URI*
    /// string, not an error (`xml.hh:73-77`).
    pub fn get_value_by_name(&self, qualified_name: &str) -> &[u8] {
        for i in 0..self.name.len() {
            if self.name[i] == qualified_name {
                return &self.value[i];
            }
        }
        BOGUS_URI.as_bytes()
    }
}

// ---------------------------------------------------------------------------
// ContentHandler (the SAX interface)
// ---------------------------------------------------------------------------

/// The SAX interface for parsing XML documents.
///
/// This is the formal interface for handling the low-level string pieces of
/// an XML document as they are scanned by the parser.  All methods are
/// required, mirroring the C++ pure-virtual interface.  Methods that the
/// parser never invokes (`start_prefix_mapping`, `end_prefix_mapping`,
/// `processing_instruction`, `skipped_entity`) are retained for interface
/// parity; `setDocumentLocator` is not ported (see module docs).
pub trait ContentHandler {
    /// Start processing a new XML document.
    fn start_document(&mut self);
    /// End processing for the current XML document.
    fn end_document(&mut self);
    /// Start a new prefix to namespace URI mapping (never invoked).
    fn start_prefix_mapping(&mut self, prefix: &str, uri: &str);
    /// Finish the current prefix (never invoked).
    fn end_prefix_mapping(&mut self, prefix: &str);
    /// Callback indicating a new XML element has started.
    fn start_element(
        &mut self,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        atts: &Attributes,
    );
    /// Callback indicating parsing of the current XML element is finished.
    fn end_element(&mut self, namespace_uri: &str, local_name: &str, qualified_name: &str);
    /// Callback with raw characters to be inserted in the current XML element
    /// (`text[start..start+length]`, indices as C++ `int4`).
    fn characters(&mut self, text: &[u8], start: i32, length: i32);
    /// Callback with whitespace character data for the current XML element.
    fn ignorable_whitespace(&mut self, text: &[u8], start: i32, length: i32);
    /// Set the XML version as specified by the current document.
    fn set_version(&mut self, version: &[u8]);
    /// Set the character encoding as specified by the current document.
    fn set_encoding(&mut self, encoding: &[u8]);
    /// Callback for a formal \e processing \e instruction (never invoked:
    /// the grammar rejects processing instructions).
    fn processing_instruction(&mut self, target: &str, data: &str);
    /// Callback for an XML entity skipped by the parser (never invoked).
    fn skipped_entity(&mut self, name: &str);
    /// Callback for handling an error condition during XML parsing.
    fn set_error(&mut self, errmsg: &str);
}

// ---------------------------------------------------------------------------
// Element / Document
// ---------------------------------------------------------------------------

/// A list of XML elements (C++ `typedef vector<Element *> List`).
pub type List = Vec<Rc<Element>>;

/// An XML element.  A node in the DOM tree.
///
/// This is the main node for the in-memory representation of the XML (DOM)
/// tree.  Unlike C++ there is no parent pointer (see module docs): the tree
/// is owned top-down, children shared via `Rc` so [`DocumentStorage`] can
/// register elements by name.
#[derive(Debug, Default)]
pub struct Element {
    /// The (local) name of the element
    name: String,
    /// Character content of the element (byte string)
    content: Vec<u8>,
    /// A list of attribute names for \b this element
    attr: Vec<String>,
    /// A (corresponding) list of attribute values for \b this element
    value: Vec<Vec<u8>>,
    /// A list of child Element objects
    children: List,
}

impl Element {
    /// Construct an empty element (C++ `Element(Element *par)`; the parent
    /// link is not ported).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the local name of the element.
    pub fn set_name(&mut self, nm: &str) {
        self.name = nm.to_string();
    }

    /// Append new character content to \b this element:
    /// `str[start..start+length]` (C++ `addContent`).
    pub fn add_content(&mut self, str_: &[u8], start: i32, length: i32) {
        self.content.extend_from_slice(&str_[start as usize..(start + length) as usize]);
    }

    /// Add a new child Element to the model, with \b this as the parent.
    pub fn add_child(&mut self, child: Rc<Element>) {
        self.children.push(child);
    }

    /// Add a new name/value attribute pair to \b this element.
    pub fn add_attribute(&mut self, nm: &str, vl: &[u8]) {
        self.attr.push(nm.to_string());
        self.value.push(vl.to_vec());
    }

    /// Get the local name of \b this element.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the list of child elements.
    pub fn get_children(&self) -> &List {
        &self.children
    }

    /// Get the character content of \b this element (byte string; see
    /// module docs for why this is not guaranteed UTF-8).
    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    /// Get an attribute value by name.
    ///
    /// Look up the value for the given attribute name and return it (the
    /// *first* match, as in C++).  `Err(KunaError::Decoder)` with explain
    /// `"Unknown attribute: <nm>"` if the attribute does not exist
    /// (C++ throws `DecoderError`).
    pub fn get_attribute_value(&self, nm: &str) -> KunaResult<&[u8]> {
        for i in 0..self.attr.len() {
            if self.attr[i] == nm {
                return Ok(&self.value[i]);
            }
        }
        Err(KunaError::decoder(format!("Unknown attribute: {nm}")))
    }

    /// Get the number of attributes for \b this element.
    pub fn get_num_attributes(&self) -> i32 {
        // cast: size_t -> int4 narrowing, as the C++ `getNumAttributes` return
        self.attr.len() as i32
    }

    /// Get the name of the i-th attribute.
    pub fn get_attribute_name(&self, i: i32) -> &str {
        &self.attr[i as usize]
    }

    /// Get the value of the i-th attribute (C++ `getAttributeValue(int4)`).
    pub fn get_attribute_value_at(&self, i: i32) -> &[u8] {
        &self.value[i as usize]
    }
}

/// Non-recursive teardown.  The C++ `Element::~Element` (xml.cc:2417-2424)
/// deletes its children recursively — safe there only because bison's
/// `YYMAXDEPTH` cap bounds the DOM depth below ~5000.  The Rust parser
/// accepts unbounded nesting (LOSS-010), so the default (recursive) drop
/// glue could overflow the call stack on a deep DOM; instead the subtree is
/// drained iteratively through a worklist.
impl Drop for Element {
    fn drop(&mut self) {
        let mut worklist: List = std::mem::take(&mut self.children);
        while let Some(child) = worklist.pop() {
            if let Ok(mut el) = Rc::try_unwrap(child) {
                // Sole owner: steal the grandchildren so `el` drops at the
                // end of this iteration with empty `children` (its own Drop
                // then has nothing left to walk).
                worklist.append(&mut el.children);
            }
            // else: the child is shared (e.g. registered in
            // `DocumentStorage::tagmap`); dropping the handle only
            // decrements the strong count, and the surviving owner tears
            // the subtree down the same way later.
        }
    }
}

/// A complete in-memory XML document.
///
/// This is actually just an [`Element`] object itself (`Deref` mirrors the
/// C++ inheritance for read access), with the document's \e root element as
/// its only child.
#[derive(Debug, Default)]
pub struct Document {
    element: Element,
}

impl Document {
    /// Construct an (empty) document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the root Element of the document.
    ///
    /// Panics if the document has no root (C++ dereferences
    /// `*children.begin()` — undefined behavior — only reachable through a
    /// successfully parsed document, which always has a root).
    pub fn get_root(&self) -> &Rc<Element> {
        self.element
            .children
            .first()
            .expect("Document::get_root: document has no root element")
    }
}

impl std::ops::Deref for Document {
    type Target = Element;
    fn deref(&self) -> &Element {
        &self.element
    }
}

// ---------------------------------------------------------------------------
// TreeHandler
// ---------------------------------------------------------------------------

/// A SAX interface implementation for constructing an in-memory DOM model.
///
/// This implementation builds a DOM model of the XML stream being parsed,
/// creating an [`Element`] object for each XML element tag in the stream.
/// Where the C++ handler mutates elements through parent pointers, this one
/// keeps the open elements on an explicit stack (`stack[0]` is the
/// document-level element, the top is C++ `cur`) and links each element into
/// its parent when it *ends* — child order is identical because a parent
/// acquires no other children while one child is open.
pub struct TreeHandler {
    /// Open elements: `[0]` = document element, last = the \e current element
    stack: Vec<Element>,
    /// The last error condition returned by the parser (if not empty)
    error: String,
}

impl TreeHandler {
    /// Constructor (owns the document-level element; cf. C++
    /// `TreeHandler(Element *rt)`).
    pub fn new() -> Self {
        TreeHandler { stack: vec![Element::new()], error: String::new() }
    }

    /// Get the current error message.
    pub fn get_error(&self) -> &str {
        &self.error
    }

    /// Yield the finished document element (only meaningful after a
    /// successful parse).
    fn into_document(mut self) -> Document {
        let element = self
            .stack
            .pop()
            .expect("TreeHandler invariant: document element missing");
        Document { element }
    }
}

impl Default for TreeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentHandler for TreeHandler {
    fn start_document(&mut self) {}
    fn end_document(&mut self) {}
    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {}
    fn end_prefix_mapping(&mut self, _prefix: &str) {}

    fn start_element(
        &mut self,
        _namespace_uri: &str,
        local_name: &str,
        _qualified_name: &str,
        atts: &Attributes,
    ) {
        let mut newel = Element::new();
        newel.set_name(local_name);
        for i in 0..atts.get_length() {
            newel.add_attribute(atts.get_local_name(i), atts.get_value(i));
        }
        self.stack.push(newel);
    }

    fn end_element(&mut self, _namespace_uri: &str, _local_name: &str, _qualified_name: &str) {
        let el = self
            .stack
            .pop()
            .expect("TreeHandler invariant: endElement without startElement");
        self.stack
            .last_mut()
            .expect("TreeHandler invariant: document element missing")
            .add_child(Rc::new(el));
    }

    fn characters(&mut self, text: &[u8], start: i32, length: i32) {
        self.stack
            .last_mut()
            .expect("TreeHandler invariant: document element missing")
            .add_content(text, start, length);
    }

    fn ignorable_whitespace(&mut self, _text: &[u8], _start: i32, _length: i32) {}
    fn processing_instruction(&mut self, _target: &str, _data: &str) {}
    fn set_version(&mut self, _val: &[u8]) {}
    fn set_encoding(&mut self, _val: &[u8]) {}
    fn skipped_entity(&mut self, _name: &str) {}

    fn set_error(&mut self, errmsg: &str) {
        self.error = errmsg.to_string();
    }
}

// ---------------------------------------------------------------------------
// DocumentStorage
// ---------------------------------------------------------------------------

/// A container for parsed XML documents.
///
/// This holds multiple XML documents that have already been parsed.
/// Documents can be put in this container, either by handing it raw bytes
/// via [`DocumentStorage::parse_document`] or a filename via
/// [`DocumentStorage::open_document`].  If they are explicitly registered,
/// specific XML Elements can be looked up by name via
/// [`DocumentStorage::get_tag`].
#[derive(Default)]
pub struct DocumentStorage {
    /// The list of documents held by this container
    doclist: Vec<Document>,
    /// The map from name to registered XML elements
    tagmap: BTreeMap<String, Rc<Element>>,
}

impl DocumentStorage {
    /// Construct an empty container.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse an XML document from the given bytes.
    ///
    /// Parsing starts immediately, attempting to make an in-memory DOM tree.
    /// `Err(KunaError::Decoder)` for any parsing error (C++ throws an
    /// "XmlException", i.e. `DecoderError`).  On error nothing is added to
    /// the storage (the C++ version leaves an inert null slot in `doclist`).
    pub fn parse_document(&mut self, s: &[u8]) -> KunaResult<&Document> {
        let doc = xml_tree(s)?;
        self.doclist.push(doc);
        Ok(self.doclist.last().expect("just pushed"))
    }

    /// Open and parse an XML file.
    ///
    /// The given filename is opened on the local filesystem and an attempt is
    /// made to parse its contents into an in-memory DOM tree.
    /// `Err(KunaError::Decoder)` with explain
    /// `"Unable to open xml document <filename>"` if the file cannot be read.
    pub fn open_document(&mut self, filename: &str) -> KunaResult<&Document> {
        let bytes = std::fs::read(filename)
            .map_err(|_| KunaError::decoder(format!("Unable to open xml document {filename}")))?;
        self.parse_document(&bytes)
    }

    /// Register the given XML Element object under its tag name.
    ///
    /// Only one Element can be stored on \b this object per tag name (a
    /// later registration replaces the earlier, as the C++ `map` assignment
    /// does).
    pub fn register_tag(&mut self, el: &Rc<Element>) {
        self.tagmap.insert(el.get_name().to_string(), Rc::clone(el));
    }

    /// Retrieve a registered XML Element by name; `None` if there is no
    /// matching registered element (C++ returns a null pointer).
    pub fn get_tag(&self, nm: &str) -> Option<&Element> {
        self.tagmap.get(nm).map(|rc| rc.as_ref())
    }
}

// ---------------------------------------------------------------------------
// XmlScan — the XML character scanner (xml.y:33-104, 222-449)
// ---------------------------------------------------------------------------

/// Modes of the scanner (`XmlScan::mode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    CharData,
    CData,
    AttValueSingle,
    AttValueDouble,
    Comment,
    CharRef,
    Name,
    SName,
    Single,
}

/// The XML character scanner.
///
/// Tokenize a byte stream suitably for the main XML parser.  The scanner
/// expects an ASCII or UTF-8 encoding.  Characters in XML tag and attribute
/// names are restricted to ASCII "letters", but extended UTF-8 characters can
/// be used in any other character data: attribute values, content, comments.
///
/// Transcription notes: the C++ scanner reads `char` (signed on the oracle
/// platform) into an `int4` lookahead, so bytes >= 0x80 appear as *negative*
/// values; `getxmlchar` injects a synthetic `'\n'` when the stream ends (or
/// when a NUL byte is read) and `-1` thereafter.  Both behaviors are
/// reproduced exactly.
struct XmlScan<'a> {
    /// The current scanning mode
    curmode: Mode,
    /// The bytes being scanned (replaces the C++ `istream`)
    input: &'a [u8],
    /// Index of the next byte to pull from `input`
    inpos: usize,
    /// Lookahead into the byte stream
    lookahead: [i32; 4],
    /// Current position in the lookahead buffer
    pos: usize,
    /// Has end of stream been reached
    endofstream: bool,
}

/// A scanned token: the token code (byte value, or one of the `*_TOK`
/// constants) plus the token string for codes > 255 (C++ `yylval.str`).
type ScanToken = (i32, Option<Vec<u8>>);

impl<'a> XmlScan<'a> {
    /// Construct scanner given the input bytes.
    fn new(input: &'a [u8]) -> Self {
        let mut scan = XmlScan {
            curmode: Mode::Single,
            input,
            inpos: 0,
            lookahead: [0; 4],
            pos: 0,
            endofstream: false,
        };
        // Fill lookahead buffer (return values discarded, as in C++)
        scan.getxmlchar();
        scan.getxmlchar();
        scan.getxmlchar();
        scan.getxmlchar();
        scan
    }

    /// Set the scanning mode.
    fn setmode(&mut self, m: Mode) {
        self.curmode = m;
    }

    /// Get the next byte in the stream.
    ///
    /// Maintain a lookahead of 4 bytes at all times so that we can check for
    /// special XML character sequences without consuming.
    fn getxmlchar(&mut self) -> i32 {
        let ret = self.lookahead[self.pos];
        if !self.endofstream {
            match self.input.get(self.inpos) {
                // C++ reads into a (signed) char: s.get(c); eof or '\0' ends
                // the stream with a synthetic '\n', otherwise the *signed*
                // value is buffered.
                Some(&b) if b != 0 => {
                    self.lookahead[self.pos] = i32::from(b as i8); // signed char semantics
                    self.inpos += 1;
                }
                _ => {
                    self.endofstream = true;
                    self.lookahead[self.pos] = ch(b'\n');
                }
            }
        } else {
            self.lookahead[self.pos] = -1;
        }
        self.pos = (self.pos + 1) & 3;
        ret
    }

    /// Peek at the next (i-th) byte without consuming.
    fn next(&self, i: usize) -> i32 {
        self.lookahead[(self.pos + i) & 3]
    }

    /// Is the given byte a \e letter.
    fn is_letter(val: i32) -> bool {
        (0x41..=0x5a).contains(&val) || (0x61..=0x7a).contains(&val)
    }

    /// Is the given byte/character the valid start of an XML name.
    fn is_initial_name_char(val: i32) -> bool {
        Self::is_letter(val) || val == ch(b'_') || val == ch(b':')
    }

    /// Is the given byte/character valid for an XML name.
    fn is_name_char(val: i32) -> bool {
        Self::is_letter(val)
            || (ch(b'0')..=ch(b'9')).contains(&val)
            || val == ch(b'.')
            || val == ch(b'-')
            || val == ch(b'_')
            || val == ch(b':')
    }

    /// Is the given byte/character valid as an XML character.
    fn is_char(val: i32) -> bool {
        // Negative values (bytes >= 0x80 under signed char, or end of
        // stream) fail this test, exactly as in the C++ oracle build.
        val >= 0x20 || val == 0xd || val == 0xa || val == 0x9
    }

    /// Scan for the next token in Single Character mode.
    fn scan_single(&mut self) -> ScanToken {
        let res = self.getxmlchar();
        if res == ch(b'<') {
            if Self::is_initial_name_char(self.next(0)) {
                return (ELEMBRACE_TOK, None);
            }
            return (COMMBRACE_TOK, None);
        }
        (res, None)
    }

    /// Scan for the next token in Character Data mode.
    fn scan_char_data(&mut self) -> ScanToken {
        let mut lvalue: Vec<u8> = Vec::new();

        while self.next(0) != -1 {
            // look for '<' '&' or ']]>'
            if self.next(0) == ch(b'<') {
                break;
            }
            if self.next(0) == ch(b'&') {
                break;
            }
            if self.next(0) == ch(b']') && self.next(1) == ch(b']') && self.next(2) == ch(b'>') {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // (char) low byte, as C++ string +=
        }
        if lvalue.is_empty() {
            return self.scan_single();
        }
        (CHARDATA_TOK, Some(lvalue))
    }

    /// Scan for the next token in CDATA mode.
    fn scan_cdata(&mut self) -> ScanToken {
        let mut lvalue: Vec<u8> = Vec::new();

        while self.next(0) != -1 {
            // Look for "]]>" and non-Char
            if self.next(0) == ch(b']') && self.next(1) == ch(b']') && self.next(2) == ch(b'>') {
                break;
            }
            if !Self::is_char(self.next(0)) {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
        }
        (CDATA_TOK, Some(lvalue)) // CData can be empty
    }

    /// Scan for the next token in Character Reference mode.
    fn scan_char_ref(&mut self) -> ScanToken {
        let mut lvalue: Vec<u8> = Vec::new();
        if self.next(0) == ch(b'x') {
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
            while self.next(0) != -1 {
                let v = self.next(0);
                if v < ch(b'0') {
                    break;
                }
                if v > ch(b'9') && v < ch(b'A') {
                    break;
                }
                if v > ch(b'F') && v < ch(b'a') {
                    break;
                }
                if v > ch(b'f') {
                    break;
                }
                lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
            }
            if lvalue.len() == 1 {
                return (ch(b'x'), None); // Must be at least 1 hex digit
            }
        } else {
            while self.next(0) != -1 {
                let v = self.next(0);
                if v < ch(b'0') {
                    break;
                }
                if v > ch(b'9') {
                    break;
                }
                lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
            }
            if lvalue.is_empty() {
                return self.scan_single();
            }
        }
        (CHARREF_TOK, Some(lvalue))
    }

    /// Scan for the next token in Attribute Value mode.
    fn scan_att_value(&mut self, quote: i32) -> ScanToken {
        let mut lvalue: Vec<u8> = Vec::new();
        while self.next(0) != -1 {
            if self.next(0) == quote {
                break;
            }
            if self.next(0) == ch(b'<') {
                break;
            }
            if self.next(0) == ch(b'&') {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
        }
        if lvalue.is_empty() {
            return self.scan_single();
        }
        (ATTVALUE_TOK, Some(lvalue))
    }

    /// Scan for the next token in Comment mode.
    fn scan_comment(&mut self) -> ScanToken {
        let mut lvalue: Vec<u8> = Vec::new();

        while self.next(0) != -1 {
            if self.next(0) == ch(b'-') && self.next(1) == ch(b'-') {
                break;
            }
            if !Self::is_char(self.next(0)) {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
        }
        (COMMENT_TOK, Some(lvalue))
    }

    /// Scan a Name or return single non-name character.
    fn scan_name(&mut self) -> ScanToken {
        if !Self::is_initial_name_char(self.next(0)) {
            return self.scan_single();
        }
        // cast: (char) low byte, as C++ string +=
        let mut lvalue: Vec<u8> = vec![self.getxmlchar() as u8];
        while self.next(0) != -1 {
            if !Self::is_name_char(self.next(0)) {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
        }
        (NAME_TOK, Some(lvalue))
    }

    /// Scan Name, allow white space before.
    fn scan_sname(&mut self) -> ScanToken {
        let mut whitecount = 0;
        while is_whitespace_tok(self.next(0)) {
            whitecount += 1;
            self.getxmlchar();
        }
        if !Self::is_initial_name_char(self.next(0)) {
            // First non-whitespace is not Name char
            if whitecount > 0 {
                return (ch(b' '), None);
            }
            return self.scan_single();
        }
        // cast: (char) low byte, as C++ string +=
        let mut lvalue: Vec<u8> = vec![self.getxmlchar() as u8];
        while self.next(0) != -1 {
            if !Self::is_name_char(self.next(0)) {
                break;
            }
            lvalue.push(self.getxmlchar() as u8); // cast: (char) low byte, as C++ string +=
        }
        if whitecount > 0 {
            return (SNAME_TOK, Some(lvalue));
        }
        (NAME_TOK, Some(lvalue))
    }

    /// Get the next token.  The mode resets to `Single` before every scan
    /// unless an action re-arms it, exactly as `XmlScan::nexttoken`.
    fn nexttoken(&mut self) -> ScanToken {
        let mymode = self.curmode;
        self.curmode = Mode::Single;
        match mymode {
            Mode::CharData => self.scan_char_data(),
            Mode::CData => self.scan_cdata(),
            Mode::AttValueSingle => self.scan_att_value(ch(b'\'')),
            Mode::AttValueDouble => self.scan_att_value(ch(b'"')),
            Mode::Comment => self.scan_comment(),
            Mode::CharRef => self.scan_char_ref(),
            Mode::Name => self.scan_name(),
            Mode::SName => self.scan_sname(),
            Mode::Single => self.scan_single(),
        }
    }
}

// ---------------------------------------------------------------------------
// Grammar action helpers (xml.y:451-502)
// ---------------------------------------------------------------------------

/// Send character data to the ContentHandler (`print_content`, xml.y:451):
/// an all-whitespace run is reported as ignorable whitespace.
fn print_content(handler: &mut dyn ContentHandler, s: &[u8]) {
    let mut i = 0usize;
    while i < s.len() {
        match s[i] {
            b' ' | b'\n' | b'\r' | b'\t' => i += 1,
            _ => break,
        }
    }
    // cast: size_t -> int4, as the C++ `str.size()` call arguments
    if i == s.len() {
        handler.ignorable_whitespace(s, 0, s.len() as i32);
    } else {
        handler.characters(s, 0, s.len() as i32);
    }
}

/// Convert an XML entity to its equivalent character (`convertEntityRef`).
/// Unknown entities yield -1, which the actions append as byte 0xFF.
fn convert_entity_ref(r: &[u8]) -> i32 {
    match r {
        b"lt" => ch(b'<'),
        b"amp" => ch(b'&'),
        b"gt" => ch(b'>'),
        b"quot" => ch(b'"'),
        b"apos" => ch(b'\''),
        _ => -1,
    }
}

/// Convert an XML character reference to its equivalent character
/// (`convertCharRef`).  The C++ accumulates into an `int4` with no overflow
/// guard; transcribed with wrapping ops per ADR 0003.
fn convert_char_ref(r: &[u8]) -> i32 {
    let (start, mult): (usize, i32) = if r[0] == b'x' { (1, 16) } else { (0, 10) };
    let mut val: i32 = 0;
    for &b in &r[start..] {
        // C++ reads the (signed) char; the scanner restricts these bytes to
        // ASCII [0-9A-Fa-f] so sign extension cannot trigger here.
        let c = i32::from(b as i8);
        let cur = if c <= ch(b'9') {
            c - ch(b'0')
        } else if c <= ch(b'F') {
            10 + c - ch(b'A')
        } else {
            10 + c - ch(b'a')
        };
        val = val.wmul(mult).wadd(cur);
    }
    val
}

// ---------------------------------------------------------------------------
// The parser — hand-written replacement for the bison automaton (LOSS-006)
// ---------------------------------------------------------------------------
//
// Faithfulness notes: the bison grammar arms the scanner mode inside
// reduction actions that all fire as *default reductions* (no lookahead is
// fetched first), so a recursive parser that sets the mode immediately
// before each fetch consumes the byte stream identically.  The shift/reduce
// conflicts (`%expect 8`) resolve as shifts: whitespace runs extend `S`
// greedily, and `<?` followed by `x` at the very start of the document is
// committed to the XML declaration.  bison normalizes any token <= 0 to
// `$end`, which `fetch` reproduces.

struct Parser<'a, 'b> {
    scan: XmlScan<'a>,
    handler: &'b mut dyn ContentHandler,
}

impl Parser<'_, '_> {
    /// Pull the next token, applying bison's `yychar <= YYEOF` => `$end`
    /// normalization (a raw 0xFF byte scans as -1 and is likewise `$end`).
    fn fetch(&mut self) -> ScanToken {
        let (tok, lval) = self.scan.nexttoken();
        if tok <= 0 {
            return (TOK_EOF, None);
        }
        (tok, lval)
    }

    /// Require the next token to be the single character `c`.
    fn expect_char(&mut self, c: u8) -> Result<(), String> {
        let (t, _) = self.fetch();
        if t != ch(c) {
            return Err(SYNTAX_ERROR.to_string());
        }
        Ok(())
    }

    /// `document: element Misc | prolog element Misc` (xml.y:141-142).
    fn parse(&mut self) -> Result<(), String> {
        // --- prolog: prologpre [doctypepro] (xml.y:166-188) ---
        // have_prologpre: at least one prologpre item (XMLDecl or Misc) seen.
        // The XML declaration is only derivable as the *first* prologpre
        // item, and doctypedecl only *after* a prologpre.
        let mut have_prologpre = false;
        let mut tok = self.fetch();
        loop {
            let t = tok.0;
            if t == ELEMBRACE_TOK {
                break; // the root element begins
            }
            if is_whitespace_tok(t) {
                // Misc: S (whitespace absorbed greedily — the shift-preferred
                // resolution of the S conflicts)
                have_prologpre = true;
                tok = self.fetch();
                continue;
            }
            if t == COMMBRACE_TOK {
                let (t2, _) = self.fetch();
                if t2 == ch(b'!') {
                    let (t3, _) = self.fetch();
                    if t3 == ch(b'-') {
                        // commentstart: COMMBRACE '!' '-' '-' (xml.y:159)
                        self.expect_char(b'-')?;
                        self.parse_comment_tail()?;
                        have_prologpre = true;
                        tok = self.fetch();
                        continue;
                    }
                    if t3 == ch(b'D') && have_prologpre {
                        // doctypedecl: COMMBRACE '!' 'D' 'O' 'C' 'T' 'Y' 'P' 'E'
                        // (xml.y:174) — reachable only after a prologpre;
                        // a leading <!DOCTYPE is a plain syntax error.
                        for &c in b"OCTYPE" {
                            self.expect_char(c)?;
                        }
                        return Err("DTD's not supported".to_string());
                    }
                    return Err(SYNTAX_ERROR.to_string());
                }
                if t2 == ch(b'?') {
                    if !have_prologpre {
                        // Initial position: xmldeclstart vs PI — the bison
                        // shift/reduce conflict resolves as shift on 'x'.
                        let (t3, _) = self.fetch();
                        if t3 == ch(b'x') {
                            // xmldeclstart: COMMBRACE '?' 'x' 'm' 'l' VersionInfo
                            self.expect_char(b'm')?;
                            self.expect_char(b'l')?;
                            self.parse_xmldecl_tail()?;
                            have_prologpre = true;
                            tok = self.fetch();
                            continue;
                        }
                    }
                    // PI: COMMBRACE '?' { yyerror(...); YYERROR; } (xml.y:161)
                    return Err("Processing instructions are not supported".to_string());
                }
                return Err(SYNTAX_ERROR.to_string());
            }
            return Err(SYNTAX_ERROR.to_string());
        }

        // --- the root element (ELEMBRACE consumed) ---
        self.parse_element()?;

        // --- exactly ONE trailing Misc, then $end (document: element Misc).
        // The scanner's synthetic '\n' guarantees at least one whitespace
        // token after the last real byte, so a document ending in a comment
        // can never be accepted (faithful upstream quirk). ---
        let (t, _) = self.fetch();
        if is_whitespace_tok(t) {
            let mut t2 = self.fetch().0;
            while is_whitespace_tok(t2) {
                t2 = self.fetch().0;
            }
            if t2 != TOK_EOF {
                return Err(SYNTAX_ERROR.to_string());
            }
        } else if t == COMMBRACE_TOK {
            let (t2, _) = self.fetch();
            if t2 == ch(b'?') {
                return Err("Processing instructions are not supported".to_string());
            }
            if t2 != ch(b'!') {
                return Err(SYNTAX_ERROR.to_string());
            }
            self.expect_char(b'-')?;
            self.expect_char(b'-')?;
            self.parse_comment_tail()?;
            if self.fetch().0 != TOK_EOF {
                return Err(SYNTAX_ERROR.to_string());
            }
        } else {
            return Err(SYNTAX_ERROR.to_string());
        }
        Ok(())
    }

    /// `Comment: commentstart COMMENT '-' '-' '>'` with `commentstart`
    /// already consumed; the comment text is discarded (xml.y:160).
    fn parse_comment_tail(&mut self) -> Result<(), String> {
        // commentstart action: setmode(CommentMode)
        self.scan.setmode(Mode::Comment);
        let (t, _lval) = self.fetch();
        if t != COMMENT_TOK {
            return Err(SYNTAX_ERROR.to_string());
        }
        self.expect_char(b'-')?;
        self.expect_char(b'-')?;
        self.expect_char(b'>')?;
        Ok(())
    }

    /// Rest of the XML declaration after `<?xml`:
    /// `VersionInfo [EncodingDecl] [S] '?' '>'` (xml.y:182-188).
    fn parse_xmldecl_tail(&mut self) -> Result<(), String> {
        // VersionInfo: S 'v' 'e' 'r' 's' 'i' 'o' 'n' Eq AttValue (xml.y:182)
        let mut t = self.fetch().0;
        if !is_whitespace_tok(t) {
            return Err(SYNTAX_ERROR.to_string());
        }
        while is_whitespace_tok(t) {
            t = self.fetch().0;
        }
        if t != ch(b'v') {
            return Err(SYNTAX_ERROR.to_string());
        }
        for &c in b"ersion" {
            self.expect_char(c)?;
        }
        let q = self.parse_eq()?;
        let version = self.parse_att_value(q)?;
        self.handler.set_version(&version);

        // XMLDecl: xmldeclstart ['?' '>' | S '?' '>' | EncodingDecl '?' '>'
        //          | EncodingDecl S '?' '>'] (xml.y:185-188)
        let mut t = self.fetch().0;
        let mut had_ws = false;
        while is_whitespace_tok(t) {
            had_ws = true;
            t = self.fetch().0;
        }
        if had_ws && t == ch(b'e') {
            // EncodingDecl: S 'e' 'n' 'c' 'o' 'd' 'i' 'n' 'g' Eq AttValue
            // (xml.y:183) — the leading S is required.
            for &c in b"ncoding" {
                self.expect_char(c)?;
            }
            let q = self.parse_eq()?;
            let encoding = self.parse_att_value(q)?;
            self.handler.set_encoding(&encoding);
            t = self.fetch().0;
            while is_whitespace_tok(t) {
                t = self.fetch().0;
            }
        }
        if t != ch(b'?') {
            return Err(SYNTAX_ERROR.to_string());
        }
        self.expect_char(b'>')?;
        Ok(())
    }

    /// `Eq: '=' | S '=' | Eq S` (xml.y:175-177): whitespace around `=`,
    /// absorbed greedily (shift-preferred).  Returns the first token *after*
    /// the Eq — the attribute-value opening quote in valid input.
    fn parse_eq(&mut self) -> Result<i32, String> {
        let mut t = self.fetch().0;
        while is_whitespace_tok(t) {
            t = self.fetch().0;
        }
        if t != ch(b'=') {
            return Err(SYNTAX_ERROR.to_string());
        }
        t = self.fetch().0;
        while is_whitespace_tok(t) {
            t = self.fetch().0;
        }
        Ok(t)
    }

    /// `AttValue: attsinglemid '\'' | attdoublemid '"'` (xml.y:150-157).
    /// `open_tok` is the (already consumed) opening quote token.
    fn parse_att_value(&mut self, open_tok: i32) -> Result<Vec<u8>, String> {
        let mode = if open_tok == ch(b'\'') {
            Mode::AttValueSingle
        } else if open_tok == ch(b'"') {
            Mode::AttValueDouble
        } else {
            return Err(SYNTAX_ERROR.to_string());
        };
        let mut buf: Vec<u8> = Vec::new();
        loop {
            // each attsinglemid/attdoublemid action re-arms the value mode
            self.scan.setmode(mode);
            let (t, lval) = self.fetch();
            if t == ATTVALUE_TOK {
                buf.extend_from_slice(&lval.expect("scanner invariant: ATTVALUE carries lvalue"));
            } else if t == open_tok {
                return Ok(buf);
            } else if t == ch(b'&') {
                let v = self.parse_reference()?;
                // C++ `*$$ += $2`: int4 -> char truncation keeps the low byte
                buf.push(v as u8);
            } else {
                // raw '<' (scanned as a brace token), EOF, ... — all invalid
                return Err(SYNTAX_ERROR.to_string());
            }
        }
    }

    /// `Reference: EntityRef | CharRef` (xml.y:213-219); the leading `&` is
    /// already consumed.  Returns the converted character value as C++
    /// `int4` (-1 for an unknown entity).
    fn parse_reference(&mut self) -> Result<i32, String> {
        // refstart action: setmode(NameMode) (xml.y:216)
        self.scan.setmode(Mode::Name);
        let (t, lval) = self.fetch();
        if t == NAME_TOK {
            // EntityRef: refstart NAME ';' (xml.y:219)
            self.expect_char(b';')?;
            Ok(convert_entity_ref(
                &lval.expect("scanner invariant: NAME carries lvalue"),
            ))
        } else if t == ch(b'#') {
            // charrefstart: refstart '#' { setmode(CharRefMode) } (xml.y:217)
            self.scan.setmode(Mode::CharRef);
            let (t2, lv2) = self.fetch();
            if t2 != CHARREF_TOK {
                return Err(SYNTAX_ERROR.to_string());
            }
            // CharRef: charrefstart CHARREF ';' (xml.y:218)
            self.expect_char(b';')?;
            Ok(convert_char_ref(
                &lv2.expect("scanner invariant: CHARREF carries lvalue"),
            ))
        } else {
            Err(SYNTAX_ERROR.to_string())
        }
    }

    /// Parse one complete element (the leading ELEMBRACE already consumed),
    /// iteratively over nesting via an explicit open-element stack.
    fn parse_element(&mut self) -> Result<(), String> {
        // Names of the currently open (STag'd but not yet ETag'd) elements.
        // C++ keeps the equivalent state as Attributes* on the bison value
        // stack; only the element name survives until endElement.
        let mut open: Vec<String> = Vec::new();
        'new_element: loop {
            // elemstart: ELEMBRACE { setmode(NameMode) } (xml.y:158)
            self.scan.setmode(Mode::Name);
            let (t, lval) = self.fetch();
            if t != NAME_TOK {
                return Err(SYNTAX_ERROR.to_string());
            }
            let name_bytes = lval.expect("scanner invariant: NAME carries lvalue");
            let name = String::from_utf8(name_bytes)
                .expect("scanner invariant: XML names are ASCII");
            let mut atts = Attributes::new(name);

            // stagstart: elemstart NAME | stagstart SAttribute (xml.y:198-199)
            // — SNameMode is armed before every token in this loop.
            let empty_elem: bool;
            loop {
                self.scan.setmode(Mode::SName);
                let (t, lval) = self.fetch();
                if t == SNAME_TOK {
                    // SAttribute: SNAME Eq AttValue (xml.y:200)
                    let aname_bytes = lval.expect("scanner invariant: SNAME carries lvalue");
                    let aname = String::from_utf8(aname_bytes)
                        .expect("scanner invariant: XML names are ASCII");
                    let q = self.parse_eq()?;
                    let aval = self.parse_att_value(q)?;
                    atts.add_attribute(aname, aval);
                } else if t == ch(b'>') {
                    // STag: stagstart '>' (xml.y:193)
                    empty_elem = false;
                    break;
                } else if t == ch(b'/') {
                    // EmptyElemTag: stagstart '/' '>' (xml.y:195)
                    self.expect_char(b'>')?;
                    empty_elem = true;
                    break;
                } else if t == ch(b' ') {
                    // scanSName consumed a whole whitespace run followed by a
                    // non-name character: grammar S in STag/EmptyElemTag.
                    // The next character is non-whitespace by construction.
                    let (t2, _) = self.fetch();
                    if t2 == ch(b'>') {
                        empty_elem = false; // STag: stagstart S '>'
                        break;
                    }
                    if t2 == ch(b'/') {
                        self.expect_char(b'>')?; // EmptyElemTag: stagstart S '/' '>'
                        empty_elem = true;
                        break;
                    }
                    return Err(SYNTAX_ERROR.to_string());
                } else {
                    // includes NAME: an attribute without preceding
                    // whitespace is a syntax error, as in bison
                    return Err(SYNTAX_ERROR.to_string());
                }
            }

            // STag/EmptyElemTag action: handler->startElement (xml.y:193-196)
            self.handler.start_element(
                BOGUS_URI,
                atts.get_elem_name(),
                atts.get_elem_name(),
                &atts,
            );
            if empty_elem {
                // element: EmptyElemTag { handler->endElement } (xml.y:190)
                self.handler
                    .end_element(BOGUS_URI, atts.get_elem_name(), atts.get_elem_name());
                if open.is_empty() {
                    return Ok(()); // the root element was empty
                }
                // fall through into the enclosing element's content loop
            } else {
                open.push(atts.elementname);
            }

            // content loop of the innermost open element (xml.y:205-211);
            // every content action re-arms CharDataMode before the next token
            loop {
                self.scan.setmode(Mode::CharData);
                let (t, lval) = self.fetch();
                if t == CHARDATA_TOK {
                    // content CHARDATA { print_content($2) } (xml.y:206)
                    print_content(
                        &mut *self.handler,
                        &lval.expect("scanner invariant: CHARDATA carries lvalue"),
                    );
                } else if t == ELEMBRACE_TOK {
                    // content element (xml.y:207)
                    continue 'new_element;
                } else if t == ch(b'&') {
                    // content Reference: a single converted char (xml.y:208)
                    let v = self.parse_reference()?;
                    print_content(&mut *self.handler, &[v as u8]);
                } else if t == COMMBRACE_TOK {
                    let (t2, _) = self.fetch();
                    if t2 == ch(b'/') {
                        // etagbrace: COMMBRACE '/' { setmode(NameMode) } (xml.y:201)
                        self.scan.setmode(Mode::Name);
                        let (t3, _etag_name) = self.fetch();
                        if t3 != NAME_TOK {
                            return Err(SYNTAX_ERROR.to_string());
                        }
                        // NOTE: the closing tag's name is NOT compared with
                        // the opening tag's (xml.y:191 discards $3).
                        // ETag: etagbrace NAME ['S'] '>' (xml.y:202-203)
                        let mut t4 = self.fetch().0;
                        while is_whitespace_tok(t4) {
                            t4 = self.fetch().0;
                        }
                        if t4 != ch(b'>') {
                            return Err(SYNTAX_ERROR.to_string());
                        }
                        let nm = open
                            .pop()
                            .expect("parser invariant: ETag with no open element");
                        self.handler.end_element(BOGUS_URI, &nm, &nm);
                        if open.is_empty() {
                            return Ok(()); // the root element is closed
                        }
                        // now in the parent's content loop
                    } else if t2 == ch(b'!') {
                        let (t3, _) = self.fetch();
                        if t3 == ch(b'-') {
                            // content Comment (xml.y:211)
                            self.expect_char(b'-')?;
                            self.parse_comment_tail()?;
                        } else if t3 == ch(b'[') {
                            // CDStart: COMMBRACE '!' '[' 'C' 'D' 'A' 'T' 'A' '['
                            // (xml.y:163)
                            for &c in b"CDATA[" {
                                self.expect_char(c)?;
                            }
                            self.scan.setmode(Mode::CData);
                            let (t4, lv4) = self.fetch();
                            if t4 != CDATA_TOK {
                                return Err(SYNTAX_ERROR.to_string());
                            }
                            // CDEnd: ']' ']' '>' (xml.y:164)
                            self.expect_char(b']')?;
                            self.expect_char(b']')?;
                            self.expect_char(b'>')?;
                            // content CDSect { print_content($2) } (xml.y:209)
                            print_content(
                                &mut *self.handler,
                                &lv4.expect("scanner invariant: CDATA carries lvalue"),
                            );
                        } else {
                            return Err(SYNTAX_ERROR.to_string());
                        }
                    } else if t2 == ch(b'?') {
                        // content PI -> always an error (xml.y:161)
                        return Err("Processing instructions are not supported".to_string());
                    } else {
                        return Err(SYNTAX_ERROR.to_string());
                    }
                } else {
                    // ']' (a stray "]]>" in content), EOF, ... — all invalid
                    return Err(SYNTAX_ERROR.to_string());
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Public parse entry points
// ---------------------------------------------------------------------------

/// Start-up the XML parser given input bytes and a handler.
///
/// This runs the low-level XML parser.  Returns 0 if there is no error
/// during parsing or a (non-zero) error condition (in which case the error
/// message was delivered to [`ContentHandler::set_error`]), matching the C++
/// `xml_parse` / bison `yyparse` contract.  (The C++ `dbg` trace parameter
/// is not ported.)
pub fn xml_parse(input: &[u8], handler: &mut dyn ContentHandler) -> i32 {
    let mut parser = Parser { scan: XmlScan::new(input), handler };
    parser.handler.start_document();
    match parser.parse() {
        Ok(()) => {
            parser.handler.end_document();
            0
        }
        Err(msg) => {
            parser.handler.set_error(&msg);
            1
        }
    }
}

/// Parse the given XML bytes into an in-memory document.
///
/// The input is parsed using the standard [`TreeHandler`] for producing an
/// in-memory DOM representation of the XML document.
/// `Err(KunaError::Decoder)` carrying the parser's error message on failure
/// (C++ throws `DecoderError`).
pub fn xml_tree(input: &[u8]) -> KunaResult<Document> {
    let mut handle = TreeHandler::new();
    if 0 != xml_parse(input, &mut handle) {
        return Err(KunaError::decoder(handle.get_error()));
    }
    Ok(handle.into_document())
}

/// Send the given bytes to a stream, escaping characters with special XML
/// meaning (`xml_escape`).
///
/// This makes the following character substitutions:
///   - `<` => `&lt;`
///   - `>` => `&gt;`
///   - `&` => `&amp;`
///   - `"` => `&quot;`
///   - `'` => `&apos;`
///
/// (The C++ version stops at a NUL terminator; the slice length is the
/// boundary here.)
pub fn xml_escape(s: &mut Vec<u8>, str_: &[u8]) {
    for &b in str_ {
        match b {
            b'<' => s.extend_from_slice(b"&lt;"),
            b'>' => s.extend_from_slice(b"&gt;"),
            b'&' => s.extend_from_slice(b"&amp;"),
            b'"' => s.extend_from_slice(b"&quot;"),
            b'\'' => s.extend_from_slice(b"&apos;"),
            _ => s.push(b),
        }
    }
}

// Some helper functions for writing XML documents directly to a stream

/// Output an XML attribute name/value pair to stream (`a_v`).
pub fn a_v(s: &mut Vec<u8>, attr: &str, val: &[u8]) {
    s.push(b' ');
    s.extend_from_slice(attr.as_bytes());
    s.extend_from_slice(b"=\"");
    xml_escape(s, val);
    s.push(b'"');
}

/// Output the given signed integer (decimal) as an XML attribute value
/// (`a_v_i`; C++ `intb` -> `i64`).
pub fn a_v_i(s: &mut Vec<u8>, attr: &str, val: i64) {
    s.push(b' ');
    s.extend_from_slice(attr.as_bytes());
    s.extend_from_slice(b"=\"");
    s.extend_from_slice(format!("{val}").as_bytes());
    s.push(b'"');
}

/// Output the given unsigned integer (`0x` hex) as an XML attribute value
/// (`a_v_u`; C++ `uintb` -> `u64`).
pub fn a_v_u(s: &mut Vec<u8>, attr: &str, val: u64) {
    s.push(b' ');
    s.extend_from_slice(attr.as_bytes());
    s.extend_from_slice(b"=\"0x");
    s.extend_from_slice(format!("{val:x}").as_bytes());
    s.push(b'"');
}

/// Output the given boolean value as an XML attribute (`a_v_b`).
pub fn a_v_b(s: &mut Vec<u8>, attr: &str, val: bool) {
    s.push(b' ');
    s.extend_from_slice(attr.as_bytes());
    s.extend_from_slice(b"=\"");
    s.extend_from_slice(if val { b"true".as_slice() } else { b"false".as_slice() });
    s.push(b'"');
}

/// Read an XML attribute value as a boolean (`xml_readbool`).
///
/// This method is intended to recognize the strings "true", "yes", and "1"
/// as a \b true value.  Anything else is returned as \b false.
pub fn xml_readbool(attr: &[u8]) -> bool {
    if attr.is_empty() {
        return false;
    }
    let firstc = attr[0];
    if firstc == b't' {
        return true;
    }
    if firstc == b'1' {
        return true;
    }
    if firstc == b'y' {
        return true; // For backward compatibility
    }
    false
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repo root")
    }

    /// Parse and unwrap a well-formed document.
    fn parse_ok(input: &str) -> Document {
        xml_tree(input.as_bytes()).unwrap_or_else(|e| panic!("expected Ok, got: {e}"))
    }

    /// Parse expecting failure; return the DecoderError explain string.
    fn parse_err(input: &[u8]) -> String {
        match xml_tree(input) {
            Err(KunaError::Decoder { explain }) => explain,
            Err(other) => panic!("expected KunaError::Decoder, got {other:?}"),
            Ok(doc) => panic!(
                "expected parse failure, got document with root <{}>",
                doc.get_root().get_name()
            ),
        }
    }

    /// Extract the root tag name straight from the raw bytes (skipping the
    /// XML declaration and comments), independently of the parser under test.
    fn raw_root_tag(bytes: &[u8]) -> String {
        let mut i = 0usize;
        loop {
            assert!(i < bytes.len(), "no root tag found in raw bytes");
            let b = bytes[i];
            if b.is_ascii_whitespace() {
                i += 1;
            } else if bytes[i..].starts_with(b"<!--") {
                let end = find_sub(bytes, b"-->", i).expect("unterminated comment");
                i = end + 3;
            } else if bytes[i..].starts_with(b"<?") {
                let end = find_sub(bytes, b"?>", i).expect("unterminated decl");
                i = end + 2;
            } else if b == b'<' {
                let mut j = i + 1;
                while j < bytes.len()
                    && (bytes[j].is_ascii_alphanumeric() || b"._:-".contains(&bytes[j]))
                {
                    j += 1;
                }
                return String::from_utf8(bytes[i + 1..j].to_vec()).unwrap();
            } else {
                panic!("unexpected raw byte {b:#x} before root tag");
            }
        }
    }

    fn find_sub(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
        haystack[from..]
            .windows(needle.len())
            .position(|w| w == needle)
            .map(|p| p + from)
    }

    // -- (a) corpus ---------------------------------------------------------

    #[test]
    fn test_xml_corpus_datatests_and_stage_tests() {
        let root = repo_root();
        let mut count = 0usize;
        for dir in ["tests/datatests", "tests/stages"] {
            let mut paths: Vec<PathBuf> = std::fs::read_dir(root.join(dir))
                .unwrap_or_else(|e| panic!("read_dir {dir}: {e}"))
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|x| x == "xml"))
                .collect();
            paths.sort();
            for path in paths {
                let bytes = std::fs::read(&path).unwrap();
                let doc = xml_tree(&bytes)
                    .unwrap_or_else(|e| panic!("{}: parse failed: {e}", path.display()));
                // every kuna datatest / stage test has root <decompilertest>
                assert_eq!(
                    doc.get_root().get_name(),
                    "decompilertest",
                    "{}",
                    path.display()
                );
                assert_eq!(
                    doc.get_root().get_name(),
                    raw_root_tag(&bytes),
                    "{}",
                    path.display()
                );
                count += 1;
            }
        }
        // 83 datatests + 104 stage testcases (incl. ghangr-dd-argmatch-to-argument-noea / gotoreduce,
        // ghangr-missing-function-call-1101b1, ghangr-tee-o2-tail-jumps-4a1f49 / tailcalljump,
        // branchflip-negated-guard / branchflip,
        // regionstructure-seq + regionstructure-loop + regionstructure-switch +
        // regionstructure-shortcircuit / regionstructure,
        // regionstructure-loop-refine / regionlooprefine,
        // regionstructure-edgeorder / regionedgeorder (SAILR P2 H2/dominance edge-virtualization ordering),
        // ghangr-ifelseflatten / ifelseflatten,
        // switchsharedcase-b2sum / switchsharedcase,
        // ghangr-who-condensing-opt-reversion-72e518 / crossjumprevert,
        // ghangr-morton-my-message-callback-bfd2fa / taildup (return tail with a call),
        // ghangr-true-1804-dedup-ite-tail-8a690c / dedupitetail (the INVERSE of taildup:
        // merge a duplicated if/else leaf prefix/suffix),
        // ghangr-setlocale-rettype / DIV-11,
        // ghangr-noreturn_extern / noreturn_extern, angr test_tail_tail_bytes_ret_dup,
        // ghangr-incorrect-duplication-chcon-a0e113 / noreturn_externmatch, DIV-13,
        // and ghangr-switchmultipred-memmove / switchmultipred, angr
        // test_decompiling_abnormal_switch_case_case3,
        // and ghangr-optimized-memcpy-6301a9 / unrolledguard, angr
        // test_decompiling_optimized_memcpy,
        // and ghdec-whiledo-complex / decbench F3 whiledo isComplex parity,
        // and ghangr-returndup / returndup (angr SAILR gotoless ReturnDuplicatorHigh:
        // duplicate a shared bare-epilogue return into each predecessor so the classic
        // guard shape structures as early returns; decbench F4),
        // and ghangr-noreturn-error / noreturn_error, decbench F2 error(nonzero,...) wrappers,
        // and ghangr-iteregion / iteregion, angr ITERegionConverter assignment-diamond
        // -> `?:` ternary, decbench F5 O0-iproute2-ip-print_link_flags,
        // and ghangr-switchreturn / switchreturn, the continuation of earlyreturn to the
        // WIDE multi-way switch-phi return, O0-libedit-tty__getcharindex,
        // and modes-aggressive / the `mode reliable|aggressive` preset axis
        // (Architecture::apply_mode; returndup-only warning as the discriminator),
        // and ghangr-iteexpr / the `iteexpr` computed-arm ?: extension of iteregion
        // (angr ITERegionConverter over computed arms; ternary in the on-pass only),
        // and the THREE `condfold` witnesses for the short-circuit fold across a
        // COMPLEX sibling (angr Phoenix MultiStatementExpression relaxation),
        // whose admission predicate is the UNION of two rules:
        // ghangr-condfold / Rule A, the bounded prefix (crossing goto in the
        // off-pass, comma-expression operand in the on-pass),
        // ghangr-condfold-newbury / the guard-cascade shape, goto+label in the
        // off-pass and neither at `wide` (it folds through Rule A), and
        // ghangr-condfold-ruleb / Rule B ISOLATED, a guard whose two
        // statement-root calls put it outside Rule A's call cap at every level,
        // and kuna-cnorm-protogap / the DIV-34 brace-placement default flip
        // (no blank line between a prototype and `{`; braceformat restores),
        // and kuna-cnorm-nullprint / the DIV-35 NULL-token default flip
        // (zero pointer constants render NULL; nullprinting off restores),
        // and kuna-cnorm-compoundassign / the DIV-36 emitInplaceOp port
        // (out = out OP y renders out OP= y; inplaceops off restores),
        // and kuna-cnorm-truthycond / the DIV-37 truthy condition option
        // (if (x != 0) renders if (x), if (p == 0) renders if (!p)),
        // and kuna-cnorm-braceelide / the DIV-38 single-statement if-body
        // brace elision (braceless indented statement; braceelide off restores),
        // and kuna-cnorm-warnstyle / the DIV-39 terse end-of-line warning slugs
        // (usage(1); // no-return; warnstyle banner restores the WARNING lines)
        // and ghdec-fast-funcdisc / the bounded fast-project inventory
        // (old fast omits a direct callee; default fast emits its real body),
        // and ghdec-spacebase-unnamed / the symbol-less spacebase PTRSUB leaf
        // (internal PTRSUB(ESP,8) p-code out, &Stack00000008 storage leaf in),
        // and ghdec-opcode-seam / the total opcode -> TypeOp seam
        // (a rule-produced INT_SRIGHT used to panic the whole function away),
        // and ghdec-isamode-inject / the ARM jump-table callotherfixup drain
        // (no setISAMode statement survives into the emitted C),
        // and ghdec-realtypes-pointee / the realtypes unknown-pointee size
        // (void *a3 alongside a3[1] out, unsigned long *a3 in),
        // and ghdec-stalejumptable / a JumpTable outliving its swept BRANCHIND
        // (loweredswitch stranded a real jump table and the model walk panicked),
        // and ghdec-finalorder-entryfirst / BlockGraph::orderBlocks (the entry
        // component printed after an unconditional goto, i.e. as dead code)
        // and ghdec-callsitestackargs / stack-passed call argument recovery
        // (option off truncates a call at the six SysV register arguments)
        // and ghdec-inputtile-overlap / renaming over guardInput's leftover
        // input pieces (gnulib rpl_fcntl at -O2 produced no body at all)
        // and ghdec-iteboolean / short-circuit 0/1 select re-rolling
        // (option off leaves the explicit `if (a && b) v = 1; else v = 0;` diamond)
        // and ghdec-symbol-keyed-local-decls / one declaration per ScopeLocal
        // Symbol (one stack slot declared twice under one name, two types)
        // and ghdec-funcptralign / the cspec-declared function-pointer alignment
        // (every ARM/Thumb indirect call carried an invented & 0xfffffffe)
        // and ghdec-paramcopyhoist / the entry-block anchor for an unmodified
        // parameter's trim COPY (option off sinks `v2 = a1;` below the first guard)
        // and ghdec-subright / non-least-significant truncation lowering
        // (the raw SUBnn p-code operator leaked into the C body)
        // and ghdec-returncopysplit / the read-only output gate on a datatype
        // copy split (a cloned heritage return-copy printed per-element stores
        // into a .rodata string literal)
        // and ghdec-itecondlist / the ITE condition-list tolerance (option off
        // re-rolls only ceil(N/2) of a run of N identical assignment diamonds)
        // and ghdec-peimportcall / PE import-call binding (option off leaves a
        // `call [IAT slot]` as an unnamed `(*dat_...)()` with no no-return effect)
        // and ghdec-undefname / no `$$undef` placeholder survives the naming pass
        // (an identifier containing `$$`, and one stack Symbol split across two names)
        // and ghdec-cspecprotos / the cspec's named prototype models are registered
        // (`option defaultprototype __thiscall` recovers the ECX this-pointer)
        // and gh271-x86-maxlen-nop / a 15-byte instruction decodes (the parser's
        // masked-off tail read past the 16-byte buffer no longer aborts the decode)
        // and funcboundflow / a fall-through into a known function entry is truncated
        // instead of decoding the next function's body into the current one
        // and ghdec-branchflip-armswap / a swapped `if` arm keeps its whole body
        // (the else-if collapse hoisted the arm's tail statement out of the arm)
        // and ghdec-guardarm / the ruleBlockIfNoExit arm tie-break resolves by layout
        // and ghdec-loopcondhoist / the deferred ifNoExit scan passes over a loop head
        // and ghdec-calloverlap / the two partial-range call-overlap guards
        // (a whole-width PXOR write hides the SysV entry inside the XMM0 range)
        // and ghdec-orchain / returndup must not split the operand chain of a
        // short-circuit (option off = the guard cascade, on = the folded condition)
        // and kuna-evalcurrentproto / the compiler spec's <eval_current_prototype>
        // model recovers a __fastcall function's ECX/EDX arguments
        // and kuna-msvcftol / the MSVC __ftol call-fixup recovers the x87 (ST0)
        // argument an argument-less `__ftol()` call had dropped
        // and kuna-ctypes / valid per-architecture C spelling of the core types
        // (option off = the Ghidra vocabulary, on = the target's own C names)
        // and gh180-implied-cover-scarry / overflow guards read the PRE-add
        // operands (implied-marking dirties the operand Covers, so the
        // pre-add/post-add merge is illegal and the guard keeps its own value)
        // and gh181-snipreads-indirect / the snipReads INDIRECT carve-out plus
        // the foldcallret indirect-read barrier (an out-parameter copy printed
        // above the call that fills it, so freecon consumed the pre-call NULL)
        // and gh182-loadguardrange / the ValueSet refinement of indexed-stack
        // guards sizes a stack array by its real index bound (option off = the
        // 4-element cap with split-off never-assigned tail scalars)
        // and gh275-spillargtrial / caller-save spill tolerance in input-trial
        // scoring (option off drops the first atan2 call's second argument to
        // the `movapd [rsp+0x20],xmm1` spill, on recovers it)
        // and oxidizer-securitycheck / rustc bounds-check panic-branch stripping
        // (option off = the `if (idx >= len) panic_bounds_check()` guard, on = the
        // guarded load alone)
        // and oxidizer-cleanupcode / the Rust drop/deallocate call sites are
        // deleted in the pre-SSA window, so the drop glue AND the argument setup
        // that only fed it are both gone (option off = both calls plus both
        // argument constants come back)
        // and kuna-retinputhalf / a returned register half that is an input
        // parameter the function moved there is kept, so the parameter stays in
        // the signature (option off = the half AND the argument are dropped)
        // and kuna-symbolnamerepair / C++ anonymous namespaces survive the
        // name-only demangling (the load used to fail outright)
        // and kuna-noreturn-discstrict / the discovered-no-return tally counts
        // only positive evidence, so a decode gap can no longer forge a no-return
        // verdict (wiring-only on the XML path -- the Listing is real-ELF only)
        // and kuna-symbolnamechars / a symbol name's raw bytes no longer
        // restructure the C document they are printed into (option off = a `*/`
        // closes the header comment, a newline splits the declaration, and two
        // non-UTF-8 names collapse onto one)
        // and kuna-symbolnamebound / a 200-component qualified symbol name folds
        // to the bounded scope path instead of nesting one ~1.5 KB Scope per
        // component (GH-338), and still resolves by either spelling
        // and kuna-arraysubfield / a partial access into an array symbol reports
        // the size it touches, so an 8-byte write into a char[16] is `._0_8_`
        // and a genuine one-byte element access keeps its `[3]` subscript
        // and kuna-msvcfpconst / MSVC `__real@` FP-constant COMDATs are decoded
        // from their mangled names (option off = every operand a dat_<addr>)
        // and kuna-cortexmpriv / the Cortex-M `isCurrentModePrivileged()` guard
        // around every MRS/MSR folds away (option off = the guard block survives)
        // and kuna-unmappedentry / the Listing walk's function worklist refuses a
        // CALL target its own instruction worklist would not decode (wiring-only
        // on the XML path -- the Listing is real-object only, like
        // kuna-listing-flag)
        // and kuna-linuxsyscall / a 32-bit Linux `int 0x80` names the syscall the
        // constant in EAX selects and takes the ABI registers as its arguments
        // (option off = two `(*(code *)swi(0x80))()` indirect calls with every
        // argument dead-coded away)
        // and kuna-entrymainproto / the PE CRT startup's argc/argv/envp call site
        // types the entry function it calls (wiring-only on the XML path -- the
        // analyzer tier is real-object only)
        // and kuna-switchselector / a lowered-switch cascade whose selector the
        // install cannot re-read is declined, so the compiler's if/else-if chain
        // over the real parameter survives (option off = `switch(0)`, every case
        // unreachable and the parameter absent from the prototype)
        // and gh403-funcboundflow-cfg / the funcboundflow truncation of a
        // CONDITIONAL branch's fall-through starts its own basic block, so the
        // branch no longer self-edges (invalid `} while ;`, or a collapsed single
        // out-edge the two-way block readers index off the end of)
        // and kuna-fastfailnoreturn / a Windows `int 0x29` (`__fastfail`) ends the
        // flow instead of raising the stack pointer by 8 and degenerating the frame
        // and kuna-arraycoverwidth / a sixteen-byte transfer through a `char[16]`
        // VM register bank rendered `v2[0] = v3[0]`, a one-byte lvalue
        assert_eq!(count, 245, "corpus file count drifted");
    }

    /// ~20 representative SLEIGH spec files across varied processors
    /// (NOT .sla — those are binary build artifacts).
    const SPEC_FILES: &[(&str, &str)] = &[
        ("specs/Ghidra/Processors/x86/data/languages/x86.ldefs", "language_definitions"),
        ("specs/Ghidra/Processors/x86/data/languages/x86.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/x86/data/languages/x86-64-gcc.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/AARCH64/data/languages/AARCH64.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/AARCH64/data/languages/AARCH64.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/ARM/data/languages/ARM.ldefs", "language_definitions"),
        ("specs/Ghidra/Processors/ARM/data/languages/ARM.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/MIPS/data/languages/mips32be.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/MIPS/data/languages/mips32_16e.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/PowerPC/data/languages/ppc_32.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/Sparc/data/languages/SparcV9_32.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/68000/data/languages/68000.ldefs", "language_definitions"),
        ("specs/Ghidra/Processors/RISCV/data/languages/riscv.ldefs", "language_definitions"),
        ("specs/Ghidra/Processors/6502/data/languages/6502.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/Z80/data/languages/z80.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/HCS08/data/languages/HC05.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/tricore/data/languages/tricore.cspec", "compiler_spec"),
        ("specs/Ghidra/Processors/SuperH4/data/languages/SuperH4.ldefs", "language_definitions"),
        ("specs/Ghidra/Processors/PA-RISC/data/languages/pa-risc32.pspec", "processor_spec"),
        ("specs/Ghidra/Processors/DATA/data/languages/data.pspec", "processor_spec"),
    ];

    #[test]
    fn test_xml_corpus_processor_spec_files() {
        let root = repo_root();
        for (rel, expected_root) in SPEC_FILES {
            let path = root.join(rel);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{}: read failed: {e}", path.display()));
            let doc = xml_tree(&bytes)
                .unwrap_or_else(|e| panic!("{}: parse failed: {e}", path.display()));
            assert_eq!(doc.get_root().get_name(), *expected_root, "{rel}");
            assert_eq!(doc.get_root().get_name(), raw_root_tag(&bytes), "{rel}");
        }
        assert_eq!(SPEC_FILES.len(), 20);
    }

    /// Depth-first search for the first element with the given name.
    fn find_first<'e>(el: &'e Element, name: &str) -> Option<&'e Element> {
        if el.get_name() == name {
            return Some(el);
        }
        for child in el.get_children() {
            if let Some(found) = find_first(child, name) {
                return Some(found);
            }
        }
        None
    }

    /// The raw bytes between the first `<bytechunk ...>` and its
    /// `</bytechunk>`, straight from the file.
    fn raw_first_bytechunk(bytes: &[u8]) -> Vec<u8> {
        let open = find_sub(bytes, b"<bytechunk", 0).expect("no <bytechunk>");
        let gt = find_sub(bytes, b">", open).expect("unterminated bytechunk start tag");
        let close = find_sub(bytes, b"</bytechunk>", gt).expect("no </bytechunk>");
        bytes[gt + 1..close].to_vec()
    }

    #[test]
    fn test_xml_bytechunk_payload_roundtrips_byte_identically() {
        let root = repo_root();
        for rel in [
            "tests/datatests/ccmp.xml",
            "tests/datatests/deindirect.xml",
            "tests/datatests/elseif.xml",
        ] {
            let bytes = std::fs::read(root.join(rel)).unwrap();
            let expected = raw_first_bytechunk(&bytes);
            // sanity: the payload really exercises whitespace preservation
            assert!(expected.starts_with(b"\n"), "{rel}: expected leading newline");
            assert!(expected.contains(&b'\n') && expected.len() > 16, "{rel}");
            let doc = xml_tree(&bytes).unwrap();
            let chunk = find_first(doc.get_root(), "bytechunk")
                .unwrap_or_else(|| panic!("{rel}: no bytechunk element"));
            assert_eq!(
                chunk.get_content(),
                expected.as_slice(),
                "{rel}: bytechunk payload not byte-identical"
            );
        }
    }

    // -- DOM model ----------------------------------------------------------

    #[test]
    fn test_xml_element_tree_and_attributes() {
        let doc = parse_ok("<top a=\"1\" b='two'>\n  <kid x=\"&lt;y&gt;\"/>\n  <kid2></kid2>\n</top>\n");
        let root = doc.get_root();
        assert_eq!(root.get_name(), "top");
        assert_eq!(root.get_num_attributes(), 2);
        assert_eq!(root.get_attribute_name(0), "a");
        assert_eq!(root.get_attribute_value_at(0), b"1");
        assert_eq!(root.get_attribute_name(1), "b");
        assert_eq!(root.get_attribute_value_at(1), b"two");
        assert_eq!(root.get_attribute_value("b").unwrap(), b"two");
        let kids = root.get_children();
        assert_eq!(kids.len(), 2);
        assert_eq!(kids[0].get_name(), "kid");
        assert_eq!(kids[0].get_attribute_value("x").unwrap(), b"<y>");
        assert_eq!(kids[1].get_name(), "kid2");
        assert!(kids[1].get_children().is_empty());
        // all-whitespace runs between children are ignorable -> dropped
        assert_eq!(root.get_content(), b"");
    }

    #[test]
    fn test_xml_element_unknown_attribute_is_decoder_error() {
        let doc = parse_ok("<a b=\"1\"/>\n");
        let err = doc.get_root().get_attribute_value("missing").unwrap_err();
        assert_eq!(err.explain(), "Unknown attribute: missing");
        // DecoderError is NOT in the LowlevelError hierarchy
        assert!(!err.is_lowlevel());
    }

    #[test]
    fn test_xml_element_duplicate_attribute_returns_first() {
        // the C++ parser does not reject duplicate attribute names, and
        // getAttributeValue returns the first match
        let doc = parse_ok("<a v=\"first\" v=\"second\"/>\n");
        assert_eq!(doc.get_root().get_num_attributes(), 2);
        assert_eq!(doc.get_root().get_attribute_value("v").unwrap(), b"first");
    }

    #[test]
    fn test_xml_mismatched_close_tag_accepted() {
        // xml.y:191 discards the ETag name without comparing it ($3 deleted)
        let doc = parse_ok("<a><b>text</wrong></a>\n");
        let root = doc.get_root();
        assert_eq!(root.get_name(), "a");
        assert_eq!(root.get_children().len(), 1);
        assert_eq!(root.get_children()[0].get_name(), "b");
        assert_eq!(root.get_children()[0].get_content(), b"text");
    }

    #[test]
    fn test_xml_mixed_content_and_tag_whitespace() {
        // chardata around a child accumulates on the parent, byte-for-byte
        let doc = parse_ok("<a> x <b/> y </a>\n");
        assert_eq!(doc.get_root().get_content(), b" x  y ");
        // whitespace inside the tags themselves
        let doc = parse_ok("<a >x</a >\n");
        assert_eq!(doc.get_root().get_content(), b"x");
        let doc = parse_ok("<a />\n");
        assert_eq!(doc.get_root().get_name(), "a");
        // CRLF passes through content unnormalized
        let doc = parse_ok("<a>x\r\ny</a>\n");
        assert_eq!(doc.get_root().get_content(), b"x\r\ny");
    }

    #[test]
    fn test_xml_attribute_value_whitespace_preserved() {
        let doc = parse_ok("<a v=\"l1\nl2\tend  \"/>\n");
        assert_eq!(doc.get_root().get_attribute_value("v").unwrap(), b"l1\nl2\tend  ");
        // whitespace around '=' is fine; other quote char is plain data
        let doc = parse_ok("<a v = 'say \"hi\"'/>\n");
        assert_eq!(doc.get_root().get_attribute_value("v").unwrap(), b"say \"hi\"");
    }

    #[test]
    fn test_xml_document_without_trailing_newline_parses() {
        // the scanner injects a synthetic '\n' at end of stream, which
        // supplies the mandatory trailing Misc
        let doc = parse_ok("<a/>");
        assert_eq!(doc.get_root().get_name(), "a");
        let doc = parse_ok("<a></a>");
        assert_eq!(doc.get_root().get_name(), "a");
    }

    #[test]
    fn test_xml_prolog_comments_and_decl() {
        let doc = parse_ok(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!-- a comment -->\n\n<root/>\n",
        );
        assert_eq!(doc.get_root().get_name(), "root");
        // comment-only prolog, no decl
        let doc = parse_ok("<!-- c1 -->\n<!-- c2 -->\n<root/>\n");
        assert_eq!(doc.get_root().get_name(), "root");
        // decl without encoding, with space before ?>
        let doc = parse_ok("<?xml version='1.0' ?><root/>\n");
        assert_eq!(doc.get_root().get_name(), "root");
    }

    // -- entities and references --------------------------------------------

    #[test]
    fn test_xml_named_entities_in_content_and_attributes() {
        let doc = parse_ok("<a>&lt;&gt;&amp;&quot;&apos;</a>\n");
        assert_eq!(doc.get_root().get_content(), b"<>&\"'");
        let doc = parse_ok("<a v=\"&lt;tag&gt; &amp; &quot;x&quot; &apos;y&apos;\"/>\n");
        assert_eq!(
            doc.get_root().get_attribute_value("v").unwrap(),
            b"<tag> & \"x\" 'y'"
        );
    }

    #[test]
    fn test_xml_char_refs_decimal_and_hex() {
        let doc = parse_ok("<a>&#65;&#x42;&#x63;</a>\n");
        assert_eq!(doc.get_root().get_content(), b"ABc");
        // uppercase hex digits
        let doc = parse_ok("<a>&#x4A;</a>\n");
        assert_eq!(doc.get_root().get_content(), b"J");
    }

    #[test]
    fn test_xml_char_ref_high_bytes_appended_raw() {
        // upstream spells UTF-8 byte pairs as consecutive char refs
        // (tests/datatests/stackstring.xml does exactly this)
        let doc = parse_ok("<a>valid&#xc2;&#xa3;</a>\n");
        assert_eq!(doc.get_root().get_content(), b"valid\xc2\xa3");
    }

    #[test]
    fn test_xml_char_ref_truncates_to_low_byte() {
        // C++ appends (char)val: only the low byte of the converted value
        let doc = parse_ok("<a>&#x141;</a>\n"); // 0x141 -> 0x41 'A'
        assert_eq!(doc.get_root().get_content(), b"A");
        let doc = parse_ok("<a>x&#256;y</a>\n"); // 256 -> 0x00
        assert_eq!(doc.get_root().get_content(), b"x\x00y");
    }

    #[test]
    fn test_xml_unknown_entity_becomes_byte_ff() {
        // convertEntityRef returns -1; (char)-1 == byte 0xFF, and the parse
        // SUCCEEDS (upstream does not reject unknown entities)
        let doc = parse_ok("<a>&bogus;</a>\n");
        assert_eq!(doc.get_root().get_content(), b"\xff");
        let doc = parse_ok("<a v='&bogus;'/>\n");
        assert_eq!(doc.get_root().get_attribute_value("v").unwrap(), b"\xff");
    }

    #[test]
    fn test_xml_whitespace_only_reference_dropped_in_content_kept_in_attribute() {
        // print_content routes an all-whitespace piece (here: the single
        // space produced by &#32;) to ignorableWhitespace -> dropped from
        // the DOM; attribute values are accumulated directly and keep it.
        let doc = parse_ok("<a>A&#32;B</a>\n");
        assert_eq!(doc.get_root().get_content(), b"AB");
        let doc = parse_ok("<a v=\"A&#32;B\"/>\n");
        assert_eq!(doc.get_root().get_attribute_value("v").unwrap(), b"A B");
    }

    // -- CDATA / comments ----------------------------------------------------

    #[test]
    fn test_xml_cdata_sections() {
        let doc = parse_ok("<a><![CDATA[<not>&amp;]]></a>\n");
        assert_eq!(doc.get_root().get_content(), b"<not>&amp;");
        // "]]" not followed by '>' stays inside the CDATA
        let doc = parse_ok("<a><![CDATA[a]]b]]></a>\n");
        assert_eq!(doc.get_root().get_content(), b"a]]b");
        // empty CDATA is ignorable
        let doc = parse_ok("<a><![CDATA[]]></a>\n");
        assert_eq!(doc.get_root().get_content(), b"");
        // an all-whitespace CDATA is ignorable too (print_content quirk)
        let doc = parse_ok("<a>x<![CDATA[ ]]>y</a>\n");
        assert_eq!(doc.get_root().get_content(), b"xy");
    }

    #[test]
    fn test_xml_comments_discarded() {
        let doc = parse_ok("<a><!-- ignore &x; <not-a-tag> -->z<!-- a - b --></a>\n");
        assert_eq!(doc.get_root().get_content(), b"z");
        assert!(doc.get_root().get_children().is_empty());
    }

    // -- SAX surface ----------------------------------------------------------

    struct RecordingHandler {
        events: Vec<String>,
    }

    impl ContentHandler for RecordingHandler {
        fn start_document(&mut self) {
            self.events.push("startDocument".into());
        }
        fn end_document(&mut self) {
            self.events.push("endDocument".into());
        }
        fn start_prefix_mapping(&mut self, _p: &str, _u: &str) {
            self.events.push("startPrefixMapping".into());
        }
        fn end_prefix_mapping(&mut self, _p: &str) {
            self.events.push("endPrefixMapping".into());
        }
        fn start_element(&mut self, uri: &str, local: &str, _q: &str, atts: &Attributes) {
            assert_eq!(uri, BOGUS_URI);
            let mut s = format!("startElement:{local}");
            for i in 0..atts.get_length() {
                s.push_str(&format!(
                    "[{}={}]",
                    atts.get_local_name(i),
                    String::from_utf8_lossy(atts.get_value(i))
                ));
            }
            self.events.push(s);
        }
        fn end_element(&mut self, _uri: &str, local: &str, _q: &str) {
            self.events.push(format!("endElement:{local}"));
        }
        fn characters(&mut self, text: &[u8], start: i32, length: i32) {
            let piece = &text[start as usize..(start + length) as usize];
            self.events
                .push(format!("characters:{}", String::from_utf8_lossy(piece)));
        }
        fn ignorable_whitespace(&mut self, _t: &[u8], _s: i32, length: i32) {
            self.events.push(format!("ignorableWhitespace:{length}"));
        }
        fn set_version(&mut self, version: &[u8]) {
            self.events
                .push(format!("setVersion:{}", String::from_utf8_lossy(version)));
        }
        fn set_encoding(&mut self, encoding: &[u8]) {
            self.events
                .push(format!("setEncoding:{}", String::from_utf8_lossy(encoding)));
        }
        fn processing_instruction(&mut self, _t: &str, _d: &str) {
            self.events.push("processingInstruction".into());
        }
        fn skipped_entity(&mut self, _n: &str) {
            self.events.push("skippedEntity".into());
        }
        fn set_error(&mut self, errmsg: &str) {
            self.events.push(format!("setError:{errmsg}"));
        }
    }

    #[test]
    fn test_xml_sax_event_sequence() {
        let mut handler = RecordingHandler { events: Vec::new() };
        let res = xml_parse(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<a b='1'>x<c/> </a>\n",
            &mut handler,
        );
        assert_eq!(res, 0);
        assert_eq!(
            handler.events,
            vec![
                "startDocument",
                "setVersion:1.0",
                "setEncoding:UTF-8",
                "startElement:a[b=1]",
                "characters:x",
                "startElement:c",
                "endElement:c",
                "ignorableWhitespace:1",
                "endElement:a",
                "endDocument",
            ]
        );
    }

    #[test]
    fn test_xml_sax_error_reported_via_set_error() {
        let mut handler = RecordingHandler { events: Vec::new() };
        let res = xml_parse(b"<a><?pi?></a>\n", &mut handler);
        assert_eq!(res, 1);
        assert_eq!(
            handler.events.last().unwrap(),
            "setError:Processing instructions are not supported"
        );
        // no endDocument after a failed parse
        assert!(!handler.events.iter().any(|e| e == "endDocument"));
    }

    // -- malformed input: same accept/reject behavior and error text as C++ --

    #[test]
    fn test_xml_malformed_plain_syntax_errors() {
        for case in [
            &b""[..],                       // empty input
            b"<a>",                         // unclosed element
            b"<a></a>x",                    // garbage after the document
            b"<a/><b/>",                    // second root element
            b"<a b=c/>",                    // unquoted attribute value
            b"<a b='x'c='y'/>",             // no whitespace between attributes
            b"< a/>",                       // space after '<'
            b"<a>]]></a>",                  // stray "]]>" in content
            b"<a v='val<'/>",               // raw '<' in an attribute value
            b"<a>&amp</a>",                 // entity without ';'
            b"<a>&#;</a>",                  // empty char ref
            b"<a>&#x;</a>",                 // hex char ref without digits
            b"<a>&#xZ;</a>",                // hex char ref with bad digit
            b"<a><!--x--y--></a>",          // "--" inside a comment
            b"<a><!--c---></a>",            // comment ending "--->"
            b"</a>",                        // close tag with no open
            b"<a>x\x00y</a>",               // NUL acts as end of stream
            b"<?xml?><a/>",                 // XML decl without version
            b"<?xml version='1.0' standalone='yes'?><a/>", // unsupported decl attr
            b"<1a/>",                       // name starting with a digit
        ] {
            assert_eq!(
                parse_err(case),
                SYNTAX_ERROR,
                "case: {:?}",
                String::from_utf8_lossy(case)
            );
        }
    }

    #[test]
    fn test_xml_malformed_processing_instructions_rejected() {
        // PIs are rejected wherever they occur, with the C++ message
        for case in [
            &b"<?pi data?>\n<a/>\n"[..],      // in the prolog (initial position)
            b"\n<?xml version='1.0'?>\n<a/>", // XML decl NOT first -> it's a PI
            b"<a><?pi?></a>\n",               // in content
            b"<a/><?pi?>",                    // as the (single) trailing Misc
        ] {
            assert_eq!(
                parse_err(case),
                "Processing instructions are not supported",
                "case: {:?}",
                String::from_utf8_lossy(case)
            );
        }
        // ... but "<?x" at the very start commits to the xmldecl path,
        // so a PI whose target starts with 'x' is a plain syntax error
        assert_eq!(parse_err(b"<?xpi?>\n<a/>\n"), SYNTAX_ERROR);
        // ... and whitespace before a trailing PI already consumes the
        // document's single Misc, so the PI is plain trailing garbage
        assert_eq!(parse_err(b"<a/>\n<?pi?>\n"), SYNTAX_ERROR);
    }

    #[test]
    fn test_xml_malformed_doctype_rejected() {
        // after any prologpre item, <!DOCTYPE reaches doctypedecl's action
        assert_eq!(parse_err(b" <!DOCTYPE foo>\n<a/>\n"), "DTD's not supported");
        assert_eq!(
            parse_err(b"<?xml version='1.0'?><!DOCTYPE foo><a/>"),
            "DTD's not supported"
        );
        // ... but at the very start of the document, doctypepro is not
        // derivable and the 'D' is a plain syntax error (upstream quirk)
        assert_eq!(parse_err(b"<!DOCTYPE foo>\n<a/>\n"), SYNTAX_ERROR);
        // partial DOCTYPE keyword
        assert_eq!(parse_err(b" <!DOCTYPO foo>\n<a/>\n"), SYNTAX_ERROR);
    }

    #[test]
    fn test_xml_malformed_trailing_comment_never_accepted() {
        // document: element Misc — exactly one Misc; the synthetic '\n'
        // after a trailing comment can never be absorbed
        assert_eq!(parse_err(b"<a/><!--c-->"), SYNTAX_ERROR);
        assert_eq!(parse_err(b"<a/>\n<!--c-->\n"), SYNTAX_ERROR);
    }

    // -- DocumentStorage ------------------------------------------------------

    #[test]
    fn test_xml_document_storage_register_and_get() {
        let mut store = DocumentStorage::new();
        let root = {
            let doc = store
                .parse_document(b"<top><sub1 a=\"1\"/><sub2/></top>\n")
                .unwrap();
            Rc::clone(doc.get_root())
        };
        store.register_tag(&root);
        // register the children too, as raw_arch/xml_arch do
        let children: Vec<Rc<Element>> = root.get_children().to_vec();
        for child in &children {
            store.register_tag(child);
        }
        assert_eq!(store.get_tag("top").unwrap().get_name(), "top");
        assert_eq!(
            store.get_tag("sub1").unwrap().get_attribute_value("a").unwrap(),
            b"1"
        );
        assert_eq!(store.get_tag("sub2").unwrap().get_name(), "sub2");
        assert!(store.get_tag("nothere").is_none());
    }

    #[test]
    fn test_xml_document_storage_open_document() {
        let root = repo_root();
        let path = root.join("tests/datatests/ccmp.xml");
        let mut store = DocumentStorage::new();
        let doc = store.open_document(path.to_str().unwrap()).unwrap();
        assert_eq!(doc.get_root().get_name(), "decompilertest");
    }

    #[test]
    fn test_xml_document_storage_open_missing_file() {
        let mut store = DocumentStorage::new();
        let err = store.open_document("/no/such/file.xml").unwrap_err();
        match &err {
            KunaError::Decoder { explain } => {
                assert_eq!(explain, "Unable to open xml document /no/such/file.xml");
            }
            other => panic!("expected Decoder error, got {other:?}"),
        }
    }

    #[test]
    fn test_xml_parse_document_error_is_decoder() {
        let mut store = DocumentStorage::new();
        let err = store.parse_document(b"<a>").unwrap_err();
        assert!(matches!(err, KunaError::Decoder { .. }));
        assert_eq!(err.explain(), SYNTAX_ERROR);
    }

    // -- Attributes quirks ----------------------------------------------------

    #[test]
    fn test_xml_attributes_missing_value_returns_bogus_uri() {
        // C++ Attributes::getValue(qualifiedName) returns bogus_uri when the
        // attribute is absent (xml.hh:73-77)
        let mut atts = Attributes::new("el".to_string());
        atts.add_attribute("a".to_string(), b"1".to_vec());
        assert_eq!(atts.get_value_by_name("a"), b"1");
        assert_eq!(atts.get_value_by_name("zz"), BOGUS_URI.as_bytes());
        assert_eq!(atts.get_elem_uri(), BOGUS_URI);
        assert_eq!(atts.get_uri(0), BOGUS_URI);
        assert_eq!(atts.get_length(), 1);
        assert_eq!(atts.get_local_name(0), "a");
        assert_eq!(atts.get_qname(0), "a");
    }

    // -- stream-writing helpers ----------------------------------------------

    #[test]
    fn test_xml_escape_and_attribute_writers() {
        let mut s: Vec<u8> = Vec::new();
        xml_escape(&mut s, b"a<b>c&d\"e'f");
        assert_eq!(s, b"a&lt;b&gt;c&amp;d&quot;e&apos;f");

        let mut s: Vec<u8> = Vec::new();
        a_v(&mut s, "name", b"<v>");
        assert_eq!(s, b" name=\"&lt;v&gt;\"");

        let mut s: Vec<u8> = Vec::new();
        a_v_i(&mut s, "n", -42);
        assert_eq!(s, b" n=\"-42\"");

        let mut s: Vec<u8> = Vec::new();
        a_v_u(&mut s, "off", 0xdeadbeef);
        assert_eq!(s, b" off=\"0xdeadbeef\"");

        let mut s: Vec<u8> = Vec::new();
        a_v_b(&mut s, "flag", true);
        a_v_b(&mut s, "flag2", false);
        assert_eq!(s, b" flag=\"true\" flag2=\"false\"");
    }

    #[test]
    fn test_xml_readbool() {
        assert!(xml_readbool(b"true"));
        assert!(xml_readbool(b"t"));
        assert!(xml_readbool(b"1"));
        assert!(xml_readbool(b"yes")); // backward compatibility
        assert!(!xml_readbool(b"false"));
        assert!(!xml_readbool(b"0"));
        assert!(!xml_readbool(b""));
        assert!(!xml_readbool(b"TRUE")); // only lowercase first char counts
    }

    // -- conversion helpers ---------------------------------------------------

    #[test]
    fn test_xml_convert_entity_and_char_ref() {
        assert_eq!(convert_entity_ref(b"lt"), i32::from(b'<'));
        assert_eq!(convert_entity_ref(b"gt"), i32::from(b'>'));
        assert_eq!(convert_entity_ref(b"amp"), i32::from(b'&'));
        assert_eq!(convert_entity_ref(b"quot"), i32::from(b'"'));
        assert_eq!(convert_entity_ref(b"apos"), i32::from(b'\''));
        assert_eq!(convert_entity_ref(b"unknown"), -1);
        assert_eq!(convert_char_ref(b"65"), 65);
        assert_eq!(convert_char_ref(b"x41"), 0x41);
        assert_eq!(convert_char_ref(b"xFf"), 0xff);
        assert_eq!(convert_char_ref(b"x0"), 0);
        // no overflow guard upstream: the i32 accumulator wraps
        assert_eq!(
            convert_char_ref(b"99999999999999999999"),
            {
                let mut v: i32 = 0;
                for _ in 0..20 {
                    v = v.wmul(10).wadd(9);
                }
                v
            }
        );
    }
}
