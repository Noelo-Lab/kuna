//! Unit tests for the `raw_arch` frontend port: the capability match predicates
//! (`isFileMatch` always true, `isXmlMatch` on `raw_savefile`), the
//! `resolveArchitecture` `archid = getTarget()` override, and the
//! `<raw_savefile>` `restoreXml` child-walk (only `coretypes` before the base
//! state).

use super::*;
use crate::capability::ArchitectureCapability;
use crate::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_base::marshal::IdRegistry;
use kuna_base::xml::Element;
use kuna_sleigh::translate::register_translate_ids;
use std::rc::Rc;

fn named(name: &str) -> Rc<Element> {
    let mut el = Element::new();
    el.set_name(name);
    Rc::new(el)
}

/// A synthetic 2-language database (matching the sleigh_arch test fixture) so
/// `resolveArchitecture` has indices to find.
fn synthetic_db() -> LanguageDatabase {
    let xml = r#"<language_definitions>
  <language processor="x86" endian="little" size="64" variant="default"
            version="1" slafile="x86-64.sla" processorspec="x86-64.pspec"
            id="x86:LE:64:default">
    <compiler name="default" spec="x86-64.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let mut r = IdRegistry::with_base_ids();
    register_translate_ids(&mut r);
    register_sleigh_arch_ids(&mut r);
    let mut db = LanguageDatabase::new();
    db.load_language_definitions(xml.as_bytes(), &r).unwrap();
    db
}

// ---------------------------------------------------------------------------
// RawBinaryArchitectureCapability predicates
// ---------------------------------------------------------------------------

#[test]
fn capability_name_file_and_xml_match() {
    let cap = RawBinaryArchitectureCapability::new();
    assert_eq!(cap.get_name(), "raw");
    // isFileMatch is always true (a file can always be opened as raw binary).
    assert!(cap.is_file_match("/anything"));
    assert!(cap.is_file_match(""));
    // isXmlMatch only on raw_savefile.
    assert!(cap.is_xml_match("raw_savefile"));
    assert!(!cap.is_xml_match("xml_savefile"));
}

#[test]
fn build_architecture_carries_filename_target() {
    let cap = RawBinaryArchitectureCapability::new();
    let arch = cap.build_architecture("/bin/raw.bin", "x86:LE:64:default:gcc");
    assert_eq!(arch.sleigh().get_filename(), "/bin/raw.bin");
    assert_eq!(arch.sleigh().get_target(), "x86:LE:64:default:gcc");
    assert_eq!(arch.adjustvma(), 0);
}

// ---------------------------------------------------------------------------
// resolveArchitecture: archid = getTarget(), then base resolve
// ---------------------------------------------------------------------------

#[test]
fn resolve_architecture_copies_target_into_archid() {
    let db = synthetic_db();
    let mut arch = RawBinaryArchitecture::new("raw.bin", "x86:LE:64:default:gcc");
    arch.resolve_architecture(&db).unwrap();
    // archid was set from the target (NOT from any loader getArchType), then the
    // base located the index.
    assert_eq!(arch.sleigh().language_index(), 0);
    assert_eq!(arch.sleigh().arch_id(), "x86:LE:64:default:gcc");
}

#[test]
fn resolve_architecture_unknown_target_errors() {
    let db = synthetic_db();
    let mut arch = RawBinaryArchitecture::new("raw.bin", "PowerPC:BE:32:default:default");
    let res = arch.resolve_architecture(&db);
    assert!(res.is_err());
    assert!(format!("{}", res.unwrap_err()).contains("No sleigh specification"));
}

// ---------------------------------------------------------------------------
// restoreXml child-walk (raw_savefile only has coretypes before the rest)
// ---------------------------------------------------------------------------

#[test]
fn restore_xml_registers_coretypes_then_rest() {
    let mut arch = RawBinaryArchitecture::new("file", "");
    let children = vec![named("coretypes"), named("save_state")];
    let res = arch.restore_xml("/raw", "x86:LE:64:default:gcc", "0x400000", &children).unwrap();
    assert_eq!(arch.sleigh().get_filename(), "/raw");
    assert_eq!(arch.sleigh().get_target(), "x86:LE:64:default:gcc");
    assert_eq!(arch.adjustvma(), 0x400000);
    assert_eq!(res.to_register.len(), 1);
    assert_eq!(res.to_register[0].get_name(), "coretypes");
    assert_eq!(res.rest.as_ref().unwrap().get_name(), "save_state");
}

#[test]
fn restore_xml_no_coretypes_first_child_is_rest() {
    let mut arch = RawBinaryArchitecture::new("file", "");
    let children = vec![named("save_state")];
    let res = arch.restore_xml("/raw", "t", "0", &children).unwrap();
    assert!(res.to_register.is_empty());
    assert_eq!(res.rest.as_ref().unwrap().get_name(), "save_state");
}

#[test]
fn restore_xml_negative_adjustvma() {
    let mut arch = RawBinaryArchitecture::new("file", "");
    let res = arch.restore_xml("/raw", "t", "-0x100", &[]).unwrap();
    assert_eq!(arch.adjustvma(), -0x100);
    assert!(res.to_register.is_empty());
    assert!(res.rest.is_none());
}
