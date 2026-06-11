//! Porter in-module tests for `libdecomp.rs` — the `startDecompilerLibrary`
//! init order: the id registry is built, capabilities are initialized and
//! ordered (raw last), and the spec-search paths are recorded.

use super::*;
use crate::capability::NamedCapability;

#[test]
fn start_builds_registry_with_inject_ids() {
    let lib = start_decompiler_library(None, &[], CapabilityRegistry::new());
    // The injection marshaling ids are registered (register_ids ran).  The
    // <callfixup> element id (pcodeinject.cc:29 -> ELEM_CALLFIXUP) resolves to a
    // nonzero id (0 == not found).
    assert_ne!(lib.registry.find_element("callfixup", 0), 0);
    // A base id (e.g. <input>) is present from with_base_ids.
    assert_ne!(lib.registry.find_element("input", 0), 0);
}

#[test]
fn start_initializes_and_sorts_capabilities_raw_last() {
    let mut caps = CapabilityRegistry::new();
    caps.register(Box::new(NamedCapability::new("xml", false, Some("save_state"))));
    caps.register(Box::new(NamedCapability::new("raw", false, None)));
    caps.register(Box::new(NamedCapability::new("sleigh", true, None)));
    let lib = start_decompiler_library(None, &[], caps);
    assert!(lib.capabilities.is_initialized());
    assert_eq!(lib.capabilities.names(), vec!["xml", "sleigh", "raw"]);
}

#[test]
fn start_records_sleighhome_and_extra_paths_in_order() {
    let extra = vec!["/specs/a".to_string(), "/specs/b".to_string()];
    let lib = start_decompiler_library(Some("/ghidra"), &extra, CapabilityRegistry::new());
    // sleighhome first, then the extra paths in declaration order (C++ order).
    assert_eq!(lib.spec_paths, vec!["/ghidra", "/specs/a", "/specs/b"]);
}

#[test]
fn start_no_paths_is_empty() {
    let lib = start_decompiler_library(None, &[], CapabilityRegistry::new());
    assert!(lib.spec_paths.is_empty());
}

#[test]
fn shutdown_consumes_state() {
    let lib = start_decompiler_library(None, &[], CapabilityRegistry::new());
    // Just exercises the consuming drop path (C++ shutdownDecompilerLibrary is
    // empty; the Rust state drops normally).
    shutdown_decompiler_library(lib);
}
