//! Tests for the `kuna_naming` port: the `kunaIsGeneratedName` matcher (both
//! schemes), the `kunaStorageComment` renderer (register / stack +/- / join /
//! tmp branches), the `OptionNameStyle::apply` parse, and the re-exported
//! address-name helpers.

use super::*;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use std::rc::Rc;

// --- kuna_is_generated_name ------------------------------------------------

#[test]
fn is_generated_name_ghidra_param() {
    assert!(kuna_is_generated_name("param_1"));
    assert!(kuna_is_generated_name("param_10"));
    // prefix without digits, or with trailing non-digits, is not generated.
    assert!(!kuna_is_generated_name("param_"));
    assert!(!kuna_is_generated_name("param_1a"));
    assert!(!kuna_is_generated_name("paramx"));
}

#[test]
fn is_generated_name_angr_v_and_a() {
    assert!(kuna_is_generated_name("v1"));
    assert!(kuna_is_generated_name("a0"));
    assert!(kuna_is_generated_name("v123"));
    // single letter with no digit, or a non-digit second char, is not generated.
    assert!(!kuna_is_generated_name("v"));
    assert!(!kuna_is_generated_name("va"));
    assert!(!kuna_is_generated_name("vx1"));
    // other prefixes are user names.
    assert!(!kuna_is_generated_name("foo"));
    assert!(!kuna_is_generated_name("b1"));
    assert!(!kuna_is_generated_name(""));
}

// --- re-exported address-name helpers --------------------------------------

#[test]
fn address_name_helpers_roundtrip() {
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    let addr = kuna_base::address::Address::new(ram, 0x401040);
    // lowercase hex, no leading zeros, no 0x.
    assert_eq!(kuna_global_data_name(&addr), "dat_401040");
    assert_eq!(kuna_function_name(&addr), "sub_401040");
    assert_eq!(kuna_label_name(&addr), "label_401040");
    // arg index, 0-indexed, negative clamps to 0.
    assert_eq!(kuna_arg_name(0), "a0");
    assert_eq!(kuna_arg_name(3), "a3");
    assert_eq!(kuna_arg_name(-5), "a0");
}

// --- kuna_storage_comment --------------------------------------------------

fn ram_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn join_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_JOIN,
        "join",
        false,
        8,
        1,
        5,
        0,
        0,
        0,
    ))
}

#[test]
fn storage_comment_register_lowercased() {
    let spc = ram_space();
    let view = StorageView {
        space: Some(&spc),
        offset: 0x10,
        addr_size: 8,
        word_size: 1,
        register_name: "RAX".to_string(),
        is_stack: false,
        join_pieces: None,
    };
    // Register name takes priority, rendered lowercase.
    assert_eq!(kuna_storage_comment(&view), "rax");
}

#[test]
fn storage_comment_stack_negative() {
    let spc = ram_space();
    // A negative stack offset (two's-complement in 8-byte storage) -> "stack - 0x..".
    let view = StorageView {
        space: Some(&spc),
        offset: 0xfffffffffffffff0, // -16
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: true,
        join_pieces: None,
    };
    assert_eq!(kuna_storage_comment(&view), "stack - 0x10");
}

#[test]
fn storage_comment_stack_positive() {
    let spc = ram_space();
    let view = StorageView {
        space: Some(&spc),
        offset: 0x20,
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: true,
        join_pieces: None,
    };
    assert_eq!(kuna_storage_comment(&view), "stack + 0x20");
}

#[test]
fn storage_comment_join_pieces() {
    let spc = join_space();
    let view = StorageView {
        space: Some(&spc),
        offset: 0,
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: false,
        // The caller has already lowercased the piece names; render ':'-joined.
        join_pieces: Some(vec!["rdx".to_string(), "rax".to_string()]),
    };
    assert_eq!(kuna_storage_comment(&view), "rdx:rax");
}

#[test]
fn storage_comment_join_empty_falls_through_to_tmp() {
    let spc = join_space();
    let view = StorageView {
        space: Some(&spc),
        offset: 0,
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: false,
        join_pieces: Some(Vec::new()), // jr->numPieces()==0 -> falls through
    };
    assert_eq!(kuna_storage_comment(&view), "tmp");
}

#[test]
fn storage_comment_no_space_is_tmp() {
    // The "no representative storage at all" path -> "tmp".
    let view = StorageView {
        space: None,
        offset: 0,
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: false,
        join_pieces: None,
    };
    assert_eq!(kuna_storage_comment(&view), "tmp");
}

#[test]
fn storage_comment_unique_temp_is_tmp() {
    // Non-register, non-stack, non-join (e.g. a unique-space temp) -> "tmp".
    let spc = ram_space();
    let view = StorageView {
        space: Some(&spc),
        offset: 0x100,
        addr_size: 8,
        word_size: 1,
        register_name: String::new(),
        is_stack: false,
        join_pieces: None,
    };
    assert_eq!(kuna_storage_comment(&view), "tmp");
}

// --- kuna_angr_naming ------------------------------------------------------

#[test]
fn angr_naming_gate() {
    assert!(kuna_angr_naming(true));
    assert!(!kuna_angr_naming(false));
}

// --- OptionNameStyle::apply ------------------------------------------------

#[test]
fn namestyle_apply_angr() {
    let opt = OptionNameStyle;
    let (val, msg) = opt.apply("angr").unwrap();
    assert!(val);
    assert_eq!(msg, "Naming scheme set to angr");
}

#[test]
fn namestyle_apply_ghidra() {
    let opt = OptionNameStyle;
    let (val, msg) = opt.apply("ghidra").unwrap();
    assert!(!val);
    assert_eq!(msg, "Naming scheme set to ghidra");
}

#[test]
fn namestyle_apply_invalid_errors() {
    let opt = OptionNameStyle;
    let err = opt.apply("bogus").unwrap_err();
    assert!(format!("{err}").contains("namestyle must be"));
}
