//! Independent verifier adversarial tests for item `w4-fw-arch-frontends`
//! (`decompiler/cpp/{sleigh_arch,raw_arch,xml_arch}.{cc,hh}`).
//!
//! These tests target the spots the verifier hunt list flagged as most fragile:
//!   - the `adjustvma` auto-base integer parse (`istringstream >> long` with
//!     `unsetf(dec|hex|oct)`) — boundary / trailing-garbage inputs,
//!   - `XmlArchitectureCapability::isFileMatch` leading-whitespace skipping
//!     (`std::ws` classic-locale set vs Rust `is_ascii_whitespace`),
//!   - `normalizeArchitecture`'s colon-counting boundaries (leading colon,
//!     5-field overflow, 3-field default-append),
//!   - `LanguageDescription::getCompiler` exact/default/first fallback ordering.
//!
//! Two tests (`adjustvma_*_diverges_from_cpp`, `is_file_match_*vtab*`) DOCUMENT
//! divergences from the C++ oracle and are written to PASS asserting the *current
//! Rust* value; each carries the C++ oracle value in a comment so the divergence
//! is pinned (flip the asserted value if/when the port is repaired). See the
//! verdict file `docs/rust-port/reviews/w4-fw-arch-frontends.md`.

use std::io::Write;
use std::rc::Rc;

use kuna_decomp::raw_arch::RawBinaryArchitecture;
use kuna_decomp::sleigh_arch::normalize_architecture;
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_base::xml::Element;

use kuna_decomp::capability::ArchitectureCapability;

// ---------------------------------------------------------------------------
// adjustvma auto-base parse (the istringstream unsetf(dec|hex|oct) semantics)
//
// adjustvma is parsed via the crate-private `parse_auto_base`, reached publicly
// through `restore_xml(.., adjustvma_str, ..)`.  We exercise it through the raw
// frontend (whose restore reads `adjustvma` the same way) and read `adjustvma()`.
// ---------------------------------------------------------------------------

/// Drive `parse_auto_base` through the public `RawBinaryArchitecture::restore_xml`
/// (no children) and return the resulting adjustvma.
fn adjustvma_of(s: &str) -> i64 {
    let mut arch = RawBinaryArchitecture::new("f", "");
    arch.restore_xml("f", "t", s, &[]).unwrap();
    arch.adjustvma()
}

#[test]
fn adjustvma_clean_decimal_hex_octal_signed_match_cpp() {
    // These are the values a real save file carries (writeUnsignedInteger emits
    // pure decimal); the C++ oracle agrees on every one.
    assert_eq!(adjustvma_of("4096"), 4096); // cpp: 4096
    assert_eq!(adjustvma_of("0x1000"), 0x1000); // cpp: 4096
    assert_eq!(adjustvma_of("0X1F"), 0x1f); // cpp: 31
    assert_eq!(adjustvma_of("010"), 8); // cpp: 8 (octal)
    assert_eq!(adjustvma_of("-0x10"), -16); // cpp: -16
    assert_eq!(adjustvma_of("+5"), 5); // cpp: 5
    assert_eq!(adjustvma_of(""), 0); // cpp: 0 (>> leaves default)
    assert_eq!(adjustvma_of("abc"), 0); // cpp: 0
    // C++ istringstream `>>` SKIPS leading whitespace and IGNORES trailing
    // whitespace; trailing spaces are not "garbage" for the integer extraction.
    assert_eq!(adjustvma_of("  0x1f  "), 0x1f); // cpp: 31
}

#[test]
fn adjustvma_trailing_garbage_diverges_from_cpp() {
    // F1 (minor): the C++ `istringstream >> long` (and `strtoll(.,0,0)`) stops at
    // the first invalid character and returns the PREFIX it parsed.  The Rust
    // `parse_auto_base` trims, then requires the ENTIRE remaining string to be a
    // valid integer, returning 0 otherwise.  So any adjustvma attribute with
    // trailing non-digit garbage diverges.  These assert the CURRENT (divergent)
    // Rust value; the C++ oracle value is in the trailing comment.
    //
    // Oracle (g++ -std=c++11, istringstream + unsetf(dec|hex|oct) >> long):
    //   "5x"    -> 5      "0x1fZ" -> 31     "0789" -> 7   (octal, stops at '8')
    assert_eq!(adjustvma_of("5x"), 0); // DIVERGENCE: cpp = 5
    assert_eq!(adjustvma_of("0x1fZ"), 0); // DIVERGENCE: cpp = 31
    assert_eq!(adjustvma_of("0789"), 0); // DIVERGENCE: cpp = 7
    // "08" coincidentally agrees (cpp parses leading 0 then stops at '8' -> 0;
    // Rust octal-parses "8" which fails -> 0).
    assert_eq!(adjustvma_of("08"), 0); // cpp = 0 (agree)
}

#[test]
fn adjustvma_negative_zero_and_plus_prefix() {
    // "-0" -> wrapping_neg(0) == 0 (cpp: 0).  A lone sign is 0 (cpp: 0).
    assert_eq!(adjustvma_of("-0"), 0); // cpp: 0
    assert_eq!(adjustvma_of("-"), 0); // cpp: 0
    assert_eq!(adjustvma_of("+"), 0); // cpp: 0
    assert_eq!(adjustvma_of("0x"), 0); // cpp: 0
}

// ---------------------------------------------------------------------------
// XmlArchitectureCapability::isFileMatch leading-whitespace skip
// ---------------------------------------------------------------------------

fn write_temp(name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("kuna_w4fwarch_{}_{}", std::process::id(), name));
    let _ = std::fs::create_dir_all(dir.parent().unwrap());
    std::fs::File::create(&dir).unwrap().write_all(bytes).unwrap();
    dir
}

#[test]
fn is_file_match_skips_classic_whitespace_then_matches_bi() {
    let cap = XmlArchitectureCapability::new();
    // space / tab / newline / carriage-return / form-feed are all skipped by both
    // C++ `std::ws` (classic locale isspace) and Rust is_ascii_whitespace.
    for ws in [b" ".as_slice(), b"\t", b"\n", b"\r", b"\x0c", b" \t\n\r"] {
        let mut buf = ws.to_vec();
        buf.extend_from_slice(b"<binaryimage>");
        let p = write_temp("ws_yes", &buf);
        assert!(cap.is_file_match(p.to_str().unwrap()), "ws {ws:?} should be skipped");
        let _ = std::fs::remove_file(&p);
    }
    // A clean <bi... matches; a non-bi prefix does not.
    let p = write_temp("plain", b"<binaryimage arch='x'>");
    assert!(cap.is_file_match(p.to_str().unwrap()));
    let _ = std::fs::remove_file(&p);
    let p2 = write_temp("decomp", b"<decompilertest>");
    assert!(!cap.is_file_match(p2.to_str().unwrap()));
    let _ = std::fs::remove_file(&p2);
}

#[test]
fn is_file_match_vertical_tab_diverges_from_cpp() {
    // F2 (minor): C++ `std::ws` uses the classic-locale `isspace`, which INCLUDES
    // vertical tab (0x0b).  Rust `is_ascii_whitespace()` does NOT skip 0x0b.  So a
    // <binaryimage> file whose first byte is a vertical tab is matched by C++ but
    // NOT by the Rust port.  Asserts the CURRENT (divergent) Rust value.
    let cap = XmlArchitectureCapability::new();
    let mut buf = vec![0x0b_u8]; // vertical tab
    buf.extend_from_slice(b"<binaryimage>");
    let p = write_temp("vtab", &buf);
    // DIVERGENCE: C++ isFileMatch would return true here (it skips \v then sees
    // '<','b','i'); the Rust port leaves \v in place, reads '\v','<','b' -> false.
    assert!(
        !cap.is_file_match(p.to_str().unwrap()),
        "documents the \\v divergence (cpp would match)"
    );
    let _ = std::fs::remove_file(&p);
}

// ---------------------------------------------------------------------------
// normalizeArchitecture colon-counting boundaries (sleigh_arch.cc:529)
// ---------------------------------------------------------------------------

#[test]
fn normalize_architecture_five_fields_extra_colon_keeps_tail() {
    // 5+ colons: the loop performs 4 successful finds (pos[0..3]) then exits with
    // i==4.  `compile = nm.substr(pos[3]+1)` takes EVERYTHING after the 4th colon,
    // including any further colons.  C++ oracle: "a:b:c:d:e:f" -> "a:b:c:d:e:f".
    assert_eq!(normalize_architecture("a:b:c:d:e:f").unwrap(), "a:b:c:d:e:f");
    // exactly 5 fields round-trips.
    assert_eq!(
        normalize_architecture("x86:LE:64:default:gcc").unwrap(),
        "x86:LE:64:default:gcc"
    );
}

#[test]
fn normalize_architecture_leading_colon_is_skipped_and_can_throw() {
    // The first find is `find(':', curpos+1)` with curpos==0, i.e. it starts at
    // index 1 — a colon at index 0 is NOT counted.  ":LE:64:v" has colons at
    // 0,3,6; only indices 3 and 6 are found (i==2) -> NOT 3 or 4 -> throw.
    // C++ oracle: ":LE:64:v" -> THROW.
    assert!(normalize_architecture(":LE:64:v").is_err());

    // ":a:b:c:d" has colons at 0,2,4,6,8.  find(':',1)=2, find(':',3)=4,
    // find(':',5)=6, find(':',7)=8 -> i==4.  processor = substr(0,2) = ":a".
    // (The leading colon is absorbed into the processor field.)  Oracle:
    // ":a:b:c:d" -> ":a:b:c:d:default" (normalizeProcessor(":a")==":a").
    assert_eq!(normalize_architecture(":a:b:c:d").unwrap(), ":a:b:c:d:default");

    // Fewer than 4 colons -> error.
    assert!(normalize_architecture("x86:LE:64").is_err());
    assert!(normalize_architecture("plain").is_err());
}

#[test]
fn normalize_architecture_three_fields_appends_default_compiler() {
    // exactly 4 colon-separated fields (3 colons) -> compiler defaults to
    // "default".  C++ oracle: "ARM:LE:32:v8" -> "ARM:LE:32:v8:default".
    assert_eq!(normalize_architecture("ARM:LE:32:v8").unwrap(), "ARM:LE:32:v8:default");
    // The variant field can be empty (trailing colon then nothing).
    // "a:b:c:" -> colons at 1,3,5; find(':',6) past end -> i==3 -> variant =
    // substr(pos[2]+1) = "" ; compile="default".  Oracle: "a:b:c::default".
    assert_eq!(normalize_architecture("a:b:c:").unwrap(), "a:b:c::default");
}

// ---------------------------------------------------------------------------
// getCompiler fallback ordering — default NOT last, and the "first" fallback
// (sleigh_arch.cc:101) — exercised through the ldefs decode + getCompiler.
// ---------------------------------------------------------------------------

#[test]
fn get_compiler_default_in_middle_is_preferred_over_first() {
    use kuna_base::marshal::IdRegistry;
    use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
    use kuna_sleigh::translate::register_translate_ids;

    let mut r = IdRegistry::with_base_ids();
    register_translate_ids(&mut r);
    register_sleigh_arch_ids(&mut r);

    // "default" is the SECOND of three compilers (not first, not last).  An
    // unknown id must still fall back to it, NOT to compilers[0].  The C++ scans
    // the whole list recording the default index, then returns it on no exact
    // match.
    let xml = r#"<language_definitions>
  <language processor="x" endian="little" size="32" variant="default"
            version="1" slafile="x.sla" processorspec="x.pspec" id="x:LE:32:default">
    <compiler name="A" spec="a.cspec" id="aaa"/>
    <compiler name="Default" spec="def.cspec" id="default"/>
    <compiler name="C" spec="c.cspec" id="ccc"/>
  </language>
</language_definitions>"#;
    let mut db = LanguageDatabase::new();
    db.load_language_definitions(xml.as_bytes(), &r).unwrap();
    let d = &db.descriptions()[0];
    // exact match wins.
    assert_eq!(d.get_compiler("ccc").get_spec(), "c.cspec");
    assert_eq!(d.get_compiler("aaa").get_spec(), "a.cspec");
    // unknown -> the middle "default", not compilers[0] ("a.cspec").
    assert_eq!(d.get_compiler("nope").get_spec(), "def.cspec");

    // No default anywhere: unknown -> compilers[0].
    let xml2 = r#"<language_definitions>
  <language processor="y" endian="little" size="32" variant="default"
            version="1" slafile="y.sla" processorspec="y.pspec" id="y:LE:32:default">
    <compiler name="First" spec="first.cspec" id="f1"/>
    <compiler name="Second" spec="second.cspec" id="f2"/>
  </language>
</language_definitions>"#;
    let mut db2 = LanguageDatabase::new();
    db2.load_language_definitions(xml2.as_bytes(), &r).unwrap();
    let d2 = &db2.descriptions()[0];
    assert_eq!(d2.get_compiler("missing").get_spec(), "first.cspec");
}

// ---------------------------------------------------------------------------
// restoreXml child-walk: an out-of-order child is NOT consumed (positional ifs).
// ---------------------------------------------------------------------------

fn named(name: &str) -> Rc<Element> {
    let mut el = Element::new();
    el.set_name(name);
    Rc::new(el)
}

#[test]
fn xml_restore_out_of_order_specext_before_binaryimage_stops_walk() {
    // The C++ walk is three SEQUENTIAL positional `if`s: binaryimage, then
    // specextensions, then coretypes.  If `specextensions` appears FIRST (before
    // any binaryimage), the binaryimage `if` does not fire, iter stays, the
    // specextensions `if` then matches the SAME (first) element and consumes it;
    // the next `if` (coretypes) then looks at the following element.
    let mut arch = XmlArchitecture::new("f", "");
    let children = vec![named("specextensions"), named("coretypes"), named("tail")];
    let res = arch.restore_xml("/i", "t", "0", &children).unwrap();
    assert_eq!(res.to_register.len(), 2);
    assert_eq!(res.to_register[0].get_name(), "specextensions");
    assert_eq!(res.to_register[1].get_name(), "coretypes");
    assert_eq!(res.rest.as_ref().unwrap().get_name(), "tail");

    // But coretypes-FIRST means binaryimage-if no, specextensions-if no,
    // coretypes-if yes -> only coretypes consumed; a following binaryimage is the
    // "rest" (the positional walk never revisits earlier conditionals).
    let mut arch2 = XmlArchitecture::new("f", "");
    let children2 = vec![named("coretypes"), named("binaryimage")];
    let res2 = arch2.restore_xml("/i", "t", "0", &children2).unwrap();
    assert_eq!(res2.to_register.len(), 1);
    assert_eq!(res2.to_register[0].get_name(), "coretypes");
    assert_eq!(res2.rest.as_ref().unwrap().get_name(), "binaryimage");
}
