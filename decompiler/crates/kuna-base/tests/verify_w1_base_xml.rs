//! Verifier adversarial tests for item `w1-base-xml` (round 1).
//!
//! These tests pin the Rust XML parser byte-for-byte against an independent
//! C++ oracle harness built from the VENDORED `decompiler/cpp/xml.cc`
//! (g++ -std=c++11, ASan/UBSan).  The harness emits, per input, a canonical
//! trace: the escaped input bytes, the full SAX callback sequence (with the
//! `xml_parse` return code), and the DOM walk (or `xml_tree` error text).
//! The oracle output is committed as `tests/data/w1_xml_oracle.out`; the
//! harness itself is verifier tooling and stays out of tree (see
//! `docs/rust-port/reviews/w1-base-xml.md` for how it was produced).
//!
//! Corpus = HANDPICKED (hunt-list-targeted edge cases: signed-char high
//! bytes, the 0xFF == EOF-sentinel collision, NUL-as-EOF, char-ref i32
//! wrapping, mode-arming order, trailing-Misc quirks, PI/DTD error texts)
//! ++ FUZZ (deterministic LCG-driven fragment assembly + byte mutations;
//! constants and seed chosen by the verifier, independent of the porter's
//! differential run).
//!
//! To regenerate the corpus input files for the C++ harness:
//!   KUNA_XML_VERIFY_DUMP=/tmp/corpus cargo test -p kuna-base \
//!       --test verify_w1_base_xml -- --ignored dump_corpus

use kuna_base::xml::{xml_escape, xml_parse, xml_tree, Attributes, ContentHandler, Element};

// ---------------------------------------------------------------------------
// Trace machinery (must match the C++ harness format exactly)
// ---------------------------------------------------------------------------

/// Escape bytes for the trace: printable ASCII except backslash stays raw,
/// everything else becomes lowercase `\xNN`.
fn esc(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        if (0x20..=0x7e).contains(&b) && b != b'\\' {
            out.push(b as char);
        } else {
            out.push_str(&format!("\\x{b:02x}"));
        }
    }
    out
}

struct RecHandler {
    lines: Vec<String>,
}

impl ContentHandler for RecHandler {
    fn start_document(&mut self) {
        self.lines.push("startDocument".to_string());
    }
    fn end_document(&mut self) {
        self.lines.push("endDocument".to_string());
    }
    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {}
    fn end_prefix_mapping(&mut self, _prefix: &str) {}
    fn start_element(
        &mut self,
        _namespace_uri: &str,
        local_name: &str,
        _qualified_name: &str,
        atts: &Attributes,
    ) {
        let mut l = format!("startElement {}", esc(local_name.as_bytes()));
        for i in 0..atts.get_length() {
            l.push_str(&format!(
                " {}={}",
                esc(atts.get_local_name(i).as_bytes()),
                esc(atts.get_value(i))
            ));
        }
        self.lines.push(l);
    }
    fn end_element(&mut self, _namespace_uri: &str, local_name: &str, _qualified_name: &str) {
        self.lines.push(format!("endElement {}", esc(local_name.as_bytes())));
    }
    fn characters(&mut self, text: &[u8], start: i32, length: i32) {
        let piece = &text[start as usize..(start + length) as usize];
        self.lines.push(format!("characters {}", esc(piece)));
    }
    fn ignorable_whitespace(&mut self, text: &[u8], start: i32, length: i32) {
        let piece = &text[start as usize..(start + length) as usize];
        self.lines.push(format!("ignorableWhitespace {}", esc(piece)));
    }
    fn set_version(&mut self, version: &[u8]) {
        self.lines.push(format!("setVersion {}", esc(version)));
    }
    fn set_encoding(&mut self, encoding: &[u8]) {
        self.lines.push(format!("setEncoding {}", esc(encoding)));
    }
    fn processing_instruction(&mut self, _target: &str, _data: &str) {}
    fn skipped_entity(&mut self, _name: &str) {}
    fn set_error(&mut self, errmsg: &str) {
        self.lines.push(format!("setError {}", esc(errmsg.as_bytes())));
    }
}

fn walk(el: &Element, depth: usize, lines: &mut Vec<String>) {
    lines.push(format!("T EL {depth} {}", esc(el.get_name().as_bytes())));
    for i in 0..el.get_num_attributes() {
        lines.push(format!(
            "T AT {}={}",
            esc(el.get_attribute_name(i).as_bytes()),
            esc(el.get_attribute_value_at(i))
        ));
    }
    lines.push(format!("T CT {}", esc(el.get_content())));
    for child in el.get_children() {
        walk(child, depth + 1, lines);
    }
}

/// Full per-case trace: `IN`, SAX (`P ...`, `P RET n`), DOM (`T ...`), `END`.
fn full_trace(input: &[u8]) -> Vec<String> {
    let mut v = vec![format!("IN {}", esc(input))];
    let mut h = RecHandler { lines: Vec::new() };
    let ret = xml_parse(input, &mut h);
    for l in &h.lines {
        v.push(format!("P {l}"));
    }
    v.push(format!("P RET {ret}"));
    match xml_tree(input) {
        Ok(doc) => {
            v.push("T OK".to_string());
            walk(doc.get_root(), 0, &mut v);
        }
        Err(e) => v.push(format!("T ERR {}", esc(e.explain().as_bytes()))),
    }
    v.push("END".to_string());
    v
}

// ---------------------------------------------------------------------------
// Corpus
// ---------------------------------------------------------------------------

/// Hunt-list-targeted handpicked inputs.  Order is load-bearing: the oracle
/// file records cases in this order followed by the fuzz cases.
fn handpicked() -> Vec<Vec<u8>> {
    let cases: &[&[u8]] = &[
        // -- document shape / trailing Misc quirks --
        b"",
        b"   ",
        b"<a/>",
        b"<a/>\n",
        b"<a></a>",
        b"  \t\n<a/>",
        b"<a/>\n\n\n",
        b"<a/>x",
        b"<a/>\n<b/>",
        b"<a/><a/>",
        b"<a/><!--c-->",
        b"<a/> <!--c-->",
        b"<a/><!--c--> ",
        b"<a/><?pi?>",
        b"<a/>\n<?pi?>\n",
        b"<a/>&#32;",
        b"&#65;<a/>",
        b"<!--c-->",
        b"<!--c1--><!--c2--><a/>",
        // -- XML declaration / prolog --
        b"<?xml version='1.0'?><a/>",
        b"<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<a/>",
        b"<?xml  version = '1.0'\tencoding= \"x\" ?><a/>",
        b"<?xml version=\"1.0\" encoding='L1'?><a/>",
        b"<?xml version='1.0' ?  ><a/>",
        b"<?xml version='1.0' standalone='yes'?><a/>",
        b"<?xml version='1.0' encoding='UTF-8'standalone?><a/>",
        b"<?xmlversion='1.0'?><a/>",
        b"<?xm l?><a/>",
        b"<?xpi?><a/>",
        b" <?xml version='1.0'?><a/>",
        b"<?xml?><a/>",
        // -- DOCTYPE (position-dependent error text) --
        b"<!DOCTYPE x><a/>",
        b" <!DOCTYPE x><a/>",
        b"<!--c--><!DOCTYPE x><a/>",
        b"<?xml version='1.0'?><!DOCTYPE x><a/>",
        b" <!DOCTYPO><a/>",
        b" <!DOCTYP",
        // -- entities / character references --
        b"<a>&lt;&gt;&amp;&quot;&apos;</a>",
        b"<a>&bogus;</a>",
        b"<a v='&bogus;'/>",
        b"<a>&#65;&#x41;&#x4a;&#x4A;</a>",
        b"<a>&#256;&#x141;&#xc2;&#xa3;</a>",
        b"<a>&#4294967296;</a>",
        b"<a>&#99999999999999999999;</a>",
        b"<a>&#x100000041;</a>",
        b"<a>&#2147483647;&#2147483648;</a>",
        b"<a>&#xffffffff;</a>",
        b"<a>&#;</a>",
        b"<a>&#x;</a>",
        b"<a>&#xZ;</a>",
        b"<a>&amp</a>",
        b"<a>& amp;</a>",
        b"<a>&l-t;</a>",
        b"<a>&LT;</a>",
        b"<a>A&#32;B</a>",
        b"<a v=\"A&#32;B\"/>",
        b"<a v='&#38;&amp;x'/>",
        b"<a>&#x46;&#x6f;&#x6F;</a>",
        // -- attributes --
        b"<a b=\"1\" c='two'/>",
        b"<a v='say \"hi\"'/>",
        b"<a v=\"it's\"/>",
        b"<a v='>raw]]>ok'/>",
        b"<a v='val<'/>",
        b"<a v='unterminated",
        b"<a b=c/>",
        b"<a b='x'c='y'/>",
        b"<a  b = \"v\"  />",
        b"<a/ >",
        b"<a\nb='1'/>",
        b"<a b='1'\t/>",
        b"<a b\n=\n'1'/>",
        b"<a v=''/>",
        b"<a v='l1\nl2\tend  '/>",
        // -- names / tags --
        b"< a/>",
        b"<>",
        b"</a>",
        b"<1a/>",
        b"<a.b-c:d_e1 x:y='1'/>",
        b"<<a/>",
        b"<a",
        b"<a ",
        b"<a b='v'",
        // -- element structure / ETag quirks --
        b"<a><b>t</wrong></a>",
        b"<a><b/></a>",
        b"<a> x <b/> y </a>",
        b"<a></ a>",
        b"<a></a >",
        b"<a></a\t\n>",
        b"<a>x</a",
        b"<a><b></b></a>",
        b"<a>text&lt;more</a>",
        b"<a>text",
        // -- CDATA --
        b"<a><![CDATA[<not>&amp;]]></a>",
        b"<a><![CDATA[]]></a>",
        b"<a><![CDATA[a]]b]]></a>",
        b"<a><![CDATA[]]]></a>",
        b"<a><![CDATA[ ]]>x</a>",
        b"<a><![CDATA[abc</a>",
        b"<a><![CDATA [x]]></a>",
        // -- comments --
        b"<a><!--x--y--></a>",
        b"<a><!--c---></a>",
        b"<a><!----></a>",
        b"<a><!-- - --></a>",
        b"<a><!--unterminated</a>",
        // -- stray "]]>" / brackets in content --
        b"<a>]]></a>",
        b"<a>a]]b</a>",
        b"<a>a]>b</a>",
        // -- NUL and high bytes (signed-char / EOF-sentinel semantics) --
        b"<a>x\x00y</a>",
        b"\x00<a/>",
        b"<a>\xff</a>",
        b"<a v='\xff'/>",
        b"\xff<a/>",
        b"<a>\x80</a>",
        b"<a>ab\xc3\xa9cd</a>",
        b"<a v='\xc3\xa9'/>",
        b"<a><!--\xc3\xa9--></a>",
        b"<a><![CDATA[\xc3\xa9]]></a>",
        b"<a><!--\x01--></a>",
        b"<\xc3a/>",
        b"<a \xc3='1'/>",
        b"<a>x<b/>\xffz</a>",
        // -- whitespace-only content (ignorableWhitespace routing) --
        b"<a> \t\r\n </a>",
        b"<a>x\r\ny</a>",
        // -- PI in content --
        b"<a><?pi?></a>",
        b"<a><?pi data?></a>",
    ];
    cases.iter().map(|c| c.to_vec()).collect()
}

/// Verifier-chosen LCG (Knuth MMIX constants; seed differs from anything the
/// porter used).
struct Lcg(u64);

impl Lcg {
    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        ((self.next_u64() >> 33) as usize) % n
    }
    fn coin(&mut self, num: usize, den: usize) -> bool {
        self.below(den) < num
    }
}

const NAME_INITIAL: &[u8] = b"abcdefgzXYZ_:";
const NAME_REST: &[u8] = b"abcz019._:-";
const SPECIAL_BYTES: &[u8] = b"<>&;'\"-]![?#x/= \n\r\t";

fn gen_name(r: &mut Lcg) -> Vec<u8> {
    let mut v = vec![NAME_INITIAL[r.below(NAME_INITIAL.len())]];
    for _ in 0..r.below(5) {
        v.push(NAME_REST[r.below(NAME_REST.len())]);
    }
    v
}

fn gen_text(r: &mut Lcg, out: &mut Vec<u8>) {
    for _ in 0..r.below(8) + 1 {
        match r.below(10) {
            0 => out.push(b'\xc3'),
            1 => out.push(b'\xa9'),
            2 => out.extend_from_slice(b" "),
            3 => out.push(b'\n'),
            4 => out.push(b']'),
            _ => out.push(b'a' + (r.below(26) as u8)), // cast: index < 26 fits u8
        }
    }
}

fn gen_reference(r: &mut Lcg, out: &mut Vec<u8>) {
    match r.below(6) {
        0 => out.extend_from_slice(b"&lt;"),
        1 => out.extend_from_slice(b"&amp;"),
        2 => out.extend_from_slice(b"&nope;"),
        3 => out.extend_from_slice(b"&#65;"),
        4 => out.extend_from_slice(b"&#x4a;"),
        _ => {
            out.extend_from_slice(b"&#");
            if r.coin(1, 2) {
                out.push(b'x');
            }
            for _ in 0..r.below(12) + 1 {
                out.push(b'0' + (r.below(10) as u8)); // cast: digit fits u8
            }
            out.push(b';');
        }
    }
}

fn gen_attribute(r: &mut Lcg, out: &mut Vec<u8>) {
    out.push(b' ');
    out.extend_from_slice(&gen_name(r));
    if r.coin(1, 4) {
        out.push(b' ');
    }
    out.push(b'=');
    if r.coin(1, 4) {
        out.push(b' ');
    }
    let q = if r.coin(1, 2) { b'\'' } else { b'"' };
    out.push(q);
    for _ in 0..r.below(3) {
        if r.coin(1, 3) {
            gen_reference(r, out);
        } else {
            gen_text(r, out);
        }
    }
    out.push(q);
}

fn gen_element(r: &mut Lcg, depth: usize, out: &mut Vec<u8>) {
    let name = gen_name(r);
    out.push(b'<');
    out.extend_from_slice(&name);
    for _ in 0..r.below(3) {
        gen_attribute(r, out);
    }
    if r.coin(1, 3) {
        if r.coin(1, 4) {
            out.push(b' ');
        }
        out.extend_from_slice(b"/>");
        return;
    }
    out.push(b'>');
    for _ in 0..r.below(4) {
        match r.below(8) {
            0 if depth < 5 => gen_element(r, depth + 1, out),
            1 => {
                out.extend_from_slice(b"<![CDATA[");
                gen_text(r, out);
                out.extend_from_slice(b"]]>");
            }
            2 => {
                out.extend_from_slice(b"<!--");
                gen_text(r, out);
                out.extend_from_slice(b"-->");
            }
            3 => gen_reference(r, out),
            _ => gen_text(r, out),
        }
    }
    out.extend_from_slice(b"</");
    out.extend_from_slice(&name);
    if r.coin(1, 6) {
        out.push(b' ');
    }
    out.push(b'>');
}

fn gen_doc(r: &mut Lcg) -> Vec<u8> {
    let mut v = Vec::new();
    if r.coin(1, 3) {
        v.extend_from_slice(b"<?xml version='1.0'");
        if r.coin(1, 2) {
            v.extend_from_slice(b" encoding=\"UTF-8\"");
        }
        if r.coin(1, 4) {
            v.push(b' ');
        }
        v.extend_from_slice(b"?>");
    }
    for _ in 0..r.below(3) {
        match r.below(4) {
            0 => {
                v.extend_from_slice(b"<!--");
                gen_text(r, &mut v);
                v.extend_from_slice(b"-->");
            }
            1 => v.extend_from_slice(b"<!DOCTYPE x>"),
            _ => v.push(b'\n'),
        }
    }
    gen_element(r, 0, &mut v);
    match r.below(8) {
        0 => v.extend_from_slice(b"<!--t-->"),
        1 => {}
        2 => v.extend_from_slice(b" <!--t-->"),
        _ => v.push(b'\n'),
    }
    v
}

fn mutate(r: &mut Lcg, v: &mut Vec<u8>) {
    for _ in 0..r.below(3) + 1 {
        if v.is_empty() {
            return;
        }
        let pos = r.below(v.len());
        let b = if r.coin(3, 4) {
            SPECIAL_BYTES[r.below(SPECIAL_BYTES.len())]
        } else {
            r.below(256) as u8 // cast: value < 256 fits u8
        };
        match r.below(3) {
            0 => v.insert(pos, b),
            1 => {
                v.remove(pos);
            }
            _ => v[pos] = b,
        }
    }
}

const FUZZ_CASES: usize = 360;

fn corpus() -> Vec<Vec<u8>> {
    let mut cases = handpicked();
    let mut r = Lcg(0xBADC_0FFE_E0DD_F00D);
    for _ in 0..FUZZ_CASES {
        let mut doc = gen_doc(&mut r);
        if r.coin(1, 2) {
            mutate(&mut r, &mut doc);
        }
        cases.push(doc);
    }
    cases
}

// ---------------------------------------------------------------------------
// Oracle file handling
// ---------------------------------------------------------------------------

const ORACLE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/w1_xml_oracle.out"
);

struct Oracle {
    escape_line: String,
    /// One entry per case: the trace lines from `IN ...` through `END`.
    cases: Vec<Vec<String>>,
}

fn load_oracle() -> Oracle {
    let text = std::fs::read_to_string(ORACLE_PATH)
        .unwrap_or_else(|e| panic!("read {ORACLE_PATH}: {e}"));
    let mut lines = text.lines();
    let escape_line = lines.next().expect("oracle: missing ESCAPE line").to_string();
    assert!(escape_line.starts_with("ESCAPE "), "oracle: bad first line");
    let mut cases = Vec::new();
    let mut cur: Option<Vec<String>> = None;
    for line in lines {
        if line == "CASE" || line.starts_with("CASE ") {
            assert!(cur.is_none(), "oracle: CASE inside case");
            cur = Some(Vec::new());
        } else {
            let body = cur.as_mut().expect("oracle: line outside case");
            body.push(line.to_string());
            if line == "END" {
                cases.push(cur.take().unwrap());
            }
        }
    }
    assert!(cur.is_none(), "oracle: trailing unterminated case");
    Oracle { escape_line, cases }
}

fn check_range(case_range: std::ops::Range<usize>) {
    let oracle = load_oracle();
    let corpus = corpus();
    assert_eq!(
        oracle.cases.len(),
        corpus.len(),
        "oracle case count != corpus size (regenerate the oracle)"
    );
    let mut mismatches = Vec::new();
    for i in case_range {
        let got = full_trace(&corpus[i]);
        let want = &oracle.cases[i];
        if got != *want {
            mismatches.push(format!(
                "case {i}:\n  input: {}\n  want:\n    {}\n  got:\n    {}",
                esc(&corpus[i]),
                want.join("\n    "),
                got.join("\n    ")
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} trace mismatches vs C++ oracle:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// `xml_escape` over every byte value 1..=255 must match the C++ oracle
/// byte-for-byte (the C++ `*str < '?'` signed-char comparison is an
/// optimization, not a behavior switch — high bytes pass through raw).
#[test]
fn verify_xml_escape_all_bytes_matches_cpp_oracle() {
    let oracle = load_oracle();
    let all: Vec<u8> = (1..=255).collect();
    let mut out = Vec::new();
    xml_escape(&mut out, &all);
    assert_eq!(format!("ESCAPE {}", esc(&out)), oracle.escape_line);
}

/// Handpicked hunt-list cases: SAX event sequence, error texts, return
/// codes, and DOM traces all byte-identical to the vendored C++ parser.
#[test]
fn verify_xml_handpicked_traces_match_cpp_oracle() {
    check_range(0..handpicked().len());
}

/// LCG fuzz corpus (fragment assembly + byte mutations): byte-identical
/// accept/reject behavior, callback sequences, and error texts.
#[test]
fn verify_xml_fuzz_traces_match_cpp_oracle() {
    let n = handpicked().len();
    check_range(n..n + FUZZ_CASES);
}

fn nested_doc(depth: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for _ in 0..depth {
        v.extend_from_slice(b"<a>");
    }
    v.push(b'x');
    for _ in 0..depth {
        v.extend_from_slice(b"</a>");
    }
    v.push(b'\n');
    v
}

/// Iteratively count the levels down to the single leaf and check its
/// content (no recursion in the test itself).
fn check_chain(doc: &kuna_base::xml::Document, depth: usize) {
    let mut el: &Element = doc.get_root();
    let mut levels = 1usize;
    while let Some(child) = el.get_children().first() {
        el = child;
        levels += 1;
    }
    assert_eq!(levels, depth);
    assert_eq!(el.get_content(), b"x");
}

/// The C++ oracle (bison stack, YYMAXDEPTH 10000) accepts element nesting
/// up to depth 4997 (verified: 4997 -> RET 0, 4998 -> "memory exhausted"
/// RET 2).  Inside that envelope the Rust port must parse AND tear down the
/// DOM without incident — including on a default 2 MiB cargo-test thread.
#[test]
fn verify_xml_deep_nesting_oracle_accepted_envelope() {
    for depth in [4990usize, 4997] {
        let v = nested_doc(depth);
        let doc = xml_tree(&v).unwrap_or_else(|e| panic!("depth {depth}: {e}"));
        check_chain(&doc, depth);
        drop(doc); // recursive Drop must survive the oracle-accepted envelope
    }
}

/// F1 (verdict round 1): for nesting beyond the oracle envelope the C++
/// parser fails CLEANLY ("memory exhausted", xml_parse returns 2, no crash
/// at any depth), while the Rust port parses unboundedly and then ABORTS
/// the process in the recursive `Drop` of the DOM (observed: depth 6000 on
/// a 2 MiB cargo-test thread, depth 30000 on an 8 MiB main thread).
///
/// This test encodes the post-repair contract, either repair being
/// acceptable: parsing must yield Ok (unbounded depth kept as a documented
/// loss, with a non-recursive teardown) or the C++ error text
/// "memory exhausted" (cap emulation).  Until repaired it does not fail —
/// it kills the whole test binary with SIGABRT (stack overflow).
#[test]
fn verify_xml_deep_nesting_beyond_oracle_envelope_f1() {
    let v = nested_doc(6000);
    match xml_tree(&v) {
        Ok(doc) => {
            check_chain(&doc, 6000);
            drop(doc); // must not overflow the stack
        }
        Err(e) => assert_eq!(e.explain(), "memory exhausted"),
    }
}

/// Dump the corpus to files for the C++ oracle harness (not a test).
#[test]
#[ignore]
fn dump_corpus() {
    let dir = match std::env::var("KUNA_XML_VERIFY_DUMP") {
        Ok(d) => d,
        Err(_) => return,
    };
    std::fs::create_dir_all(&dir).unwrap();
    for (i, case) in corpus().iter().enumerate() {
        std::fs::write(format!("{dir}/case_{i:04}.xml"), case).unwrap();
    }
}
