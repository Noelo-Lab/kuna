//! End-to-end gate for the `itaniumrtti` constructor/destructor recovery: a
//! function that stores a class's vptr through `this` is named `C::C` or `C::~C`.
//!
//! Fixtures: `itaniumctors_O0_x86_64.so` and `itaniumctors_O2_x86_64.so`, one
//! source built stripped and hidden at `-O0` and `-O2`, so only the Itanium RTTI
//! graph names anything. The addresses below are the `-O0` build's; the names they
//! must carry are the ones its unstripped twin's symbol table gives them.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// `zoo::Animal::Animal(int)` and its copy constructor.
const ANIMAL_CTORS: [u64; 2] = [0x1384, 0x13ae];
/// `zoo::Animal`'s complete and deleting destructors, vtable slots 1 and 2.
const ANIMAL_DTORS: [u64; 2] = [0x13f6, 0x1414];
/// `zoo::Dog::Dog(int, int)` and its copy constructor.
const DOG_CTORS: [u64; 2] = [0x145c, 0x151c];
const DOG_DTORS: [u64; 2] = [0x14b4, 0x14ec];
/// `zoo::Cat::Cat(int)`, which also stores the `Named` subobject's vptr at +16.
const CAT_CTOR: u64 = 0x167c;
/// `zoo::Counter` has no virtual destructor: its constructor and its destructor
/// store the vptr the same way.
const COUNTER_CTOR_AND_DTOR: [u64; 2] = [0x1598, 0x15c2];
/// `zoo::Animal::speak`, slot 0: a virtual method that is not a destructor.
const ANIMAL_SPEAK: u64 = 0x13e0;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(fixture: &str, rtti: bool) -> ConsoleProgram {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips");
    prog.arch_mut()
        .set_kuna_option("itaniumrtti", if rtti { "on" } else { "off" })
        .expect("itaniumrtti flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    prog.function_named_at(vma).unwrap_or_default()
}

#[test]
fn constructors_and_destructors_are_named_from_their_vptr_stores() {
    let prog = bootstrap("itaniumctors_O0_x86_64.so", true);
    for vma in ANIMAL_CTORS {
        assert_eq!(name_at(&prog, vma), "Animal", "{vma:#x} is zoo::Animal::Animal");
    }
    for vma in ANIMAL_DTORS {
        assert_eq!(name_at(&prog, vma), "~Animal", "{vma:#x} is zoo::Animal::~Animal");
    }
    for vma in DOG_CTORS {
        assert_eq!(name_at(&prog, vma), "Dog", "{vma:#x} is zoo::Dog::Dog");
    }
    for vma in DOG_DTORS {
        assert_eq!(name_at(&prog, vma), "~Dog", "{vma:#x} is zoo::Dog::~Dog");
    }
    assert_eq!(name_at(&prog, CAT_CTOR), "Cat");
    assert!(prog.has_symbol_named("zoo::Dog::~Dog"));
    for slot_name in ["zoo::Dog::vtable_1", "zoo::Dog::vfunc_1"] {
        assert!(
            !prog.has_symbol_named(slot_name),
            "the destructor's slot-index name is replaced, not kept beside it"
        );
    }
}

#[test]
fn a_virtual_method_and_an_ambiguous_class_are_left_alone() {
    let prog = bootstrap("itaniumctors_O0_x86_64.so", true);
    let speak = name_at(&prog, ANIMAL_SPEAK);
    assert!(speak.ends_with("_0") && !speak.contains("Animal"), "slot 0 keeps its slot name, got {speak}");
    for vma in COUNTER_CTOR_AND_DTOR {
        let name = name_at(&prog, vma);
        assert!(!name.contains("Counter"), "{vma:#x} must stay unnamed, got {name}");
    }
}

#[test]
fn an_inlining_factory_is_not_a_constructor() {
    let prog = bootstrap("itaniumctors_O2_x86_64.so", true);
    for name in ["zoo::Dog::Dog", "zoo::Cat::Cat"] {
        assert!(!prog.has_symbol_named(name), "-O2 inlines every constructor; {name} names a factory");
    }
    assert!(prog.lookup_symbol("make_dog").is_some());
}

#[test]
fn nothing_is_named_with_the_option_off() {
    let prog = bootstrap("itaniumctors_O0_x86_64.so", false);
    for vma in DOG_CTORS.into_iter().chain(DOG_DTORS) {
        let name = name_at(&prog, vma);
        assert!(!name.contains("Dog"), "{vma:#x} is named {name} with itaniumrtti off");
    }
}
