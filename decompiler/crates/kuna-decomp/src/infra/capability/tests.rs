//! Porter in-module tests for `capability.rs` — the `ArchitectureCapability`
//! registry (registration order, `sortCapabilities` raw-last, the file/XML/name
//! lookups, and the version constants).

use super::*;

#[test]
fn register_preserves_order_and_len() {
    let mut reg = CapabilityRegistry::new();
    assert!(reg.is_empty());
    reg.register(Box::new(NamedCapability::new("xml", false, Some("save_state"))));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("sleigh", true, None)));
    assert_eq!(reg.len(), 3);
    assert_eq!(reg.names(), vec!["xml", "raw", "sleigh"]);
}

#[test]
fn sort_capabilities_moves_raw_to_last() {
    // C++ sortCapabilities: "raw" goes to the end, others slide down keeping
    // their relative order.
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("xml", false, Some("save_state"))));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("sleigh", true, None)));
    reg.sort_capabilities();
    assert_eq!(reg.names(), vec!["xml", "sleigh", "raw"]);
}

#[test]
fn sort_capabilities_noop_without_raw() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("xml", false, None)));
    reg.register(Box::new(NamedCapability::new("sleigh", true, None)));
    reg.sort_capabilities();
    assert_eq!(reg.names(), vec!["xml", "sleigh"]);
}

#[test]
fn sort_capabilities_already_last_is_idempotent() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("xml", false, None)));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.sort_capabilities();
    reg.sort_capabilities();
    assert_eq!(reg.names(), vec!["xml", "raw"]);
}

#[test]
fn initialize_all_sorts_and_marks_initialized() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("xml", false, None)));
    assert!(!reg.is_initialized());
    reg.initialize_all();
    assert!(reg.is_initialized());
    // raw moved to the end during initialize_all's sort step.
    assert_eq!(reg.names(), vec!["xml", "raw"]);
}

#[test]
fn find_capability_by_file_first_match() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("nomatch", false, None)));
    reg.register(Box::new(NamedCapability::new("yes", true, None)));
    reg.register(Box::new(NamedCapability::new("also", true, None)));
    let cap = reg.find_capability_by_file("/tmp/a.out").expect("a file match");
    // First match wins (C++ returns the first capa whose isFileMatch is true).
    assert_eq!(cap.get_name(), "yes");
}

#[test]
fn find_capability_by_file_none_when_no_match() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("nomatch", false, None)));
    assert!(reg.find_capability_by_file("/tmp/a.out").is_none());
}

#[test]
fn find_capability_by_xml_matches_root_name() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("xml", false, Some("save_state"))));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    let cap = reg.find_capability_by_xml("save_state").expect("xml match");
    assert_eq!(cap.get_name(), "xml");
    assert!(reg.find_capability_by_xml("binaryimage").is_none());
}

#[test]
fn get_capability_by_name() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("xml", false, None)));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    assert_eq!(reg.get_capability("raw").map(|c| c.get_name()), Some("raw"));
    assert!(reg.get_capability("absent").is_none());
}

#[test]
fn version_constants_match_cpp() {
    // C++ majorversion=6, minorversion=1 (architecture.cc:36-37).
    assert_eq!(get_major_version(), 6);
    assert_eq!(get_minor_version(), 1);
    assert_eq!(MAJOR_VERSION, 6);
    assert_eq!(MINOR_VERSION, 1);
}
