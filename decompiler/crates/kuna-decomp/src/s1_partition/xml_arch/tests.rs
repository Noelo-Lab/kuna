//! Unit tests for the `xml_arch` frontend port: the capability match predicates
//! (`isFileMatch`/`isXmlMatch`), the `adjustvma` auto-base parse, and the
//! `<xml_savefile>` `restoreXml` child-walk ordering.

use super::*;
use crate::capability::ArchitectureCapability;
use kuna_base::xml::Element;
use std::rc::Rc;

/// Build a named element (with no attributes) for the restore-walk tests.
fn named(name: &str) -> Rc<Element> {
    let mut el = Element::new();
    el.set_name(name);
    Rc::new(el)
}

// ---------------------------------------------------------------------------
// XmlArchitectureCapability predicates
// ---------------------------------------------------------------------------

#[test]
fn capability_name_and_xml_match() {
    let cap = XmlArchitectureCapability::new();
    assert_eq!(cap.get_name(), "xml");
    assert!(cap.is_xml_match("xml_savefile"));
    assert!(!cap.is_xml_match("raw_savefile"));
    assert!(!cap.is_xml_match("binaryimage"));
}

#[test]
fn capability_file_match_reads_binaryimage_prefix() {
    use std::io::Write;
    // A file beginning (after whitespace) with "<bi" matches; anything else does
    // not (the C++ reads three chars after skipping leading whitespace).
    let dir = std::env::temp_dir().join(format!("kuna_xmlarch_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);

    let yes = dir.join("yes.xml");
    std::fs::File::create(&yes).unwrap().write_all(b"   \n  <binaryimage arch=\"x\">").unwrap();
    let no = dir.join("no.xml");
    std::fs::File::create(&no).unwrap().write_all(b"<decompilertest>").unwrap();
    let short = dir.join("short.xml");
    std::fs::File::create(&short).unwrap().write_all(b"<b").unwrap();

    let cap = XmlArchitectureCapability::new();
    assert!(cap.is_file_match(yes.to_str().unwrap()));
    assert!(!cap.is_file_match(no.to_str().unwrap()));
    // Fewer than three readable chars after whitespace -> no match.
    assert!(!cap.is_file_match(short.to_str().unwrap()));
    // A non-existent file never matches.
    assert!(!cap.is_file_match(dir.join("missing.xml").to_str().unwrap()));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn build_architecture_carries_filename_target() {
    let cap = XmlArchitectureCapability::new();
    let arch = cap.build_architecture("/bin/a.out", "x86:LE:64:default:gcc");
    assert_eq!(arch.sleigh().get_filename(), "/bin/a.out");
    assert_eq!(arch.sleigh().get_target(), "x86:LE:64:default:gcc");
    assert_eq!(arch.adjustvma(), 0);
}

// ---------------------------------------------------------------------------
// adjustvma auto-base parse (the istringstream unsetf(dec|hex|oct) semantics)
// ---------------------------------------------------------------------------

#[test]
fn parse_auto_base_decimal_hex_octal_signed() {
    // decimal
    assert_eq!(parse_auto_base("4096"), 4096);
    assert_eq!(parse_auto_base("0"), 0);
    // hex (0x / 0X prefix)
    assert_eq!(parse_auto_base("0x1000"), 0x1000);
    assert_eq!(parse_auto_base("0X20"), 0x20);
    // octal (leading 0, more than one digit)
    assert_eq!(parse_auto_base("010"), 8);
    // signed
    assert_eq!(parse_auto_base("-16"), -16);
    assert_eq!(parse_auto_base("-0x10"), -16);
    assert_eq!(parse_auto_base("+5"), 5);
    // unparseable -> 0 (the C++ >> leaves the default 0)
    assert_eq!(parse_auto_base("garbage"), 0);
    assert_eq!(parse_auto_base(""), 0);
}

// ---------------------------------------------------------------------------
// restoreXml child-walk ordering (xml_arch.cc:116)
// ---------------------------------------------------------------------------

#[test]
fn restore_xml_walks_binaryimage_specext_coretypes_then_rest() {
    let mut arch = XmlArchitecture::new("file", "");
    let children = vec![
        named("binaryimage"),
        named("specextensions"),
        named("coretypes"),
        named("save_state"),
    ];
    let res = arch
        .restore_xml("/img", "x86:LE:64:default:gcc", "0x1000", &children)
        .unwrap();
    // Header restored.
    assert_eq!(arch.sleigh().get_filename(), "/img");
    assert_eq!(arch.sleigh().get_target(), "x86:LE:64:default:gcc");
    assert_eq!(arch.adjustvma(), 0x1000);
    // The three recognized tags are queued for registration in order; the 4th is
    // the "rest" (base state).
    assert_eq!(res.to_register.len(), 3);
    assert_eq!(res.to_register[0].get_name(), "binaryimage");
    assert_eq!(res.to_register[1].get_name(), "specextensions");
    assert_eq!(res.to_register[2].get_name(), "coretypes");
    assert_eq!(res.rest.as_ref().unwrap().get_name(), "save_state");
}

#[test]
fn restore_xml_skips_absent_optional_children() {
    // No specextensions: binaryimage, then coretypes; the walk advances each
    // conditional independently (C++ `if ((*iter)->getName() == ...)`).
    let mut arch = XmlArchitecture::new("file", "");
    let children = vec![named("binaryimage"), named("coretypes")];
    let res = arch.restore_xml("/i", "t", "0", &children).unwrap();
    assert_eq!(res.to_register.len(), 2);
    assert_eq!(res.to_register[0].get_name(), "binaryimage");
    assert_eq!(res.to_register[1].get_name(), "coretypes");
    assert!(res.rest.is_none());

    // Only a binaryimage (no coretypes/specextensions, no rest).
    let mut arch2 = XmlArchitecture::new("file", "");
    let res2 = arch2.restore_xml("/i", "t", "0", &[named("binaryimage")]).unwrap();
    assert_eq!(res2.to_register.len(), 1);
    assert!(res2.rest.is_none());
}

#[test]
fn restore_xml_unrecognized_first_child_is_rest() {
    // If the first child is not "binaryimage", none of the three conditionals
    // fire and the first child becomes the "rest" element (the C++ `if` walk
    // does not consume it).
    let mut arch = XmlArchitecture::new("file", "");
    let children = vec![named("save_state")];
    let res = arch.restore_xml("/i", "t", "0", &children).unwrap();
    assert!(res.to_register.is_empty());
    assert_eq!(res.rest.as_ref().unwrap().get_name(), "save_state");
}

// ---------------------------------------------------------------------------
// build_loader rejects a non-binaryimage element
// ---------------------------------------------------------------------------

#[test]
fn build_loader_requires_binaryimage_tag() {
    let mut arch = XmlArchitecture::new("file", "");
    let res = arch.build_loader(named("not_a_binaryimage"));
    assert!(res.is_err());
    assert!(format!("{}", res.unwrap_err()).contains("binaryimage"));
}
