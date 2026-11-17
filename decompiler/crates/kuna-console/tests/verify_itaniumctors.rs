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

/// `itaniumctors_wrap_O2_x86_64.so`: helpers a constructor or destructor was
/// inlined into, which store the vptr through their first parameter exactly as a
/// constructor does (`make_gadget`, `place_gadget`, `Holder::Holder`,
/// `drop_widget`), while `kit::Gadget`'s destructor still stores it.
const WRAP_HELPERS: [u64; 4] = [0x1160, 0x1180, 0x1190, 0x11b0];
const WRAP_GADGET_DTORS: [u64; 2] = [0x12c0, 0x1300];
/// `kit::Holder::Holder(int)` in the unstripped `-O0` build, where the
/// always_inline `Gadget` constructor is inlined into it.
const WRAP_HOLDER_CTOR_O0: u64 = 0x1222;

#[test]
fn an_optimized_helper_that_stores_a_vptr_is_not_a_constructor() {
    let prog = bootstrap("itaniumctors_wrap_O2_x86_64.so", true);
    for vma in WRAP_GADGET_DTORS {
        assert_eq!(name_at(&prog, vma), "~Gadget", "{vma:#x} is kit::Gadget::~Gadget");
    }
    for vma in WRAP_HELPERS {
        let name = name_at(&prog, vma);
        assert!(name.starts_with("sub_"), "{vma:#x} must stay unnamed, got {name}");
    }
    assert!(!prog.has_symbol_named("kit::Gadget::Gadget"));
}

#[test]
fn a_symbol_table_name_gets_no_recovered_name_beside_it() {
    let prog = bootstrap("itaniumctors_wrap_O0sym_x86_64.so", true);
    assert_eq!(name_at(&prog, WRAP_HOLDER_CTOR_O0), "Holder");
    assert!(
        !prog.has_symbol_named("kit::Gadget::Gadget"),
        "kit::Holder::Holder's own symbol must stay its only name"
    );
}

/// `itaniumctors_impostor_O0_x86_64.so`: `imp::Mid::~Mid`, a pure virtual
/// destructor no vtable slot holds, and `imp::Holder::Holder`, the always_inline
/// `Gadget` constructor inlined into it, both store a vptr through `this`.
const IMPOSTOR_NOT_CTORS_O0: [u64; 2] = [0x11ce, 0x1342];
/// `imp::Mid::Mid`, which calls `Base::Base` before it stores Mid's vptr.
const IMPOSTOR_MID_CTOR_O0: u64 = 0x171a;
/// `itaniumctors_impostor_O2_x86_64.so`: `Obj::Release`, `Cell::reset` and
/// `Once::Release` store their class's vptr from a vtable slot, and `Once`'s own
/// destructors (slots 2 and 3) share `Once::Release`'s code.
const IMPOSTOR_NOT_DTORS_O2: [u64; 5] = [0x14b0, 0x1310, 0x13e0, 0x1250, 0x13a0];
const IMPOSTOR_OBJ_DTORS_O2: [u64; 2] = [0x1220, 0x1360];

#[test]
fn a_destructor_or_an_inlined_constructor_is_not_a_constructor() {
    let prog = bootstrap("itaniumctors_impostor_O0_x86_64.so", true);
    assert_eq!(name_at(&prog, IMPOSTOR_MID_CTOR_O0), "Mid");
    for vma in IMPOSTOR_NOT_CTORS_O0 {
        let name = name_at(&prog, vma);
        assert!(name.starts_with("sub_"), "{vma:#x} must stay unnamed, got {name}");
    }
}

#[test]
fn a_method_that_inlined_a_destructor_is_not_one() {
    let prog = bootstrap("itaniumctors_impostor_O2_x86_64.so", true);
    for vma in IMPOSTOR_OBJ_DTORS_O2 {
        assert_eq!(name_at(&prog, vma), "~Obj", "{vma:#x} is imp::Obj::~Obj");
    }
    for vma in IMPOSTOR_NOT_DTORS_O2 {
        let name = name_at(&prog, vma);
        assert!(!name.starts_with('~'), "{vma:#x} must keep its slot name, got {name}");
    }
}

/// `itaniumctors_rebuild_O2_x86_64.so` (g++) and `..._clang_O2_x86_64.so`: each
/// virtual method here stores its class's vptr first, from its class's vtable,
/// because it rebuilds `*this` by placement new or deletes it: `Again::reinit`,
/// `Fresh::reinit`, `Redo::reset`, `Self::destroy`, `Fill::reinit`,
/// `Grow::reset` and `Base::morph`.
const REBUILD_NOT_DTORS_GCC: [u64; 7] = [0x1240, 0x1310, 0x1330, 0x1380, 0x13b0, 0x13f0, 0x1440];
const REBUILD_NOT_DTORS_CLANG: [u64; 7] = [0x1190, 0x11e0, 0x1230, 0x1280, 0x1420, 0x1350, 0x1500];
/// `rb::Base`'s complete and deleting destructors.
const REBUILD_BASE_DTORS_GCC: [u64; 2] = [0x12f0, 0x1350];
const REBUILD_BASE_DTORS_CLANG: [u64; 2] = [0x1760, 0x1490];
/// `rb::Iface::~Iface`: the abstract class's deleting destructor is clang's trap.
const REBUILD_IFACE_DTOR_CLANG: u64 = 0x1780;

#[test]
fn a_method_that_rebuilds_this_is_not_a_destructor() {
    for (fixture, not_dtors, base) in [
        ("itaniumctors_rebuild_O2_x86_64.so", REBUILD_NOT_DTORS_GCC, REBUILD_BASE_DTORS_GCC),
        ("itaniumctors_rebuild_clang_O2_x86_64.so", REBUILD_NOT_DTORS_CLANG, REBUILD_BASE_DTORS_CLANG),
    ] {
        let prog = bootstrap(fixture, true);
        for vma in not_dtors {
            let name = name_at(&prog, vma);
            assert!(!name.starts_with('~'), "{fixture} {vma:#x} must keep its slot name, got {name}");
        }
        for vma in base {
            assert_eq!(name_at(&prog, vma), "~Base", "{fixture} {vma:#x} is rb::Base::~Base");
        }
    }
    let prog = bootstrap("itaniumctors_rebuild_clang_O2_x86_64.so", true);
    assert_eq!(name_at(&prog, REBUILD_IFACE_DTOR_CLANG), "~Iface");
}

/// `itaniumctors_byvalue_O2_x86_64.so` and `..._i386.so`: virtual methods that
/// return their own class by value build it in the return slot, the first
/// parameter, and read `this` from the second: `Vec::scaled`, `Vec::shifted`,
/// `Num::twice`, `Num::half`, `Color::darker`, `Color::brighter`, `Copy::copy`.
const BYVALUE_NOT_DTORS_X64: [u64; 7] = [0x11f0, 0x1220, 0x1250, 0x1270, 0x12b0, 0x12f0, 0x1320];
const BYVALUE_NOT_DTORS_I386: [u64; 7] = [0x1000, 0x1030, 0x1070, 0x1090, 0x10d0, 0x1120, 0x1150];
/// `Zero::zero` and `Zero::unit` never read `*this`; on x86-64 they return the
/// slot's address.
const BYVALUE_ZERO_X64: [u64; 2] = [0x1340, 0x1360];
/// `bv::Pair`'s complete and deleting destructors, beside two by-value methods.
const BYVALUE_PAIR_DTORS_X64: [u64; 2] = [0x13e0, 0x1400];
const BYVALUE_PAIR_DTORS_I386: [u64; 2] = [0x11d0, 0x11e0];
/// `bv::Pager::collect`, which returns a `List` through a slot spilled across
/// calls it was not passed to.
const BYVALUE_COLLECT_X64: u64 = 0x1430;

#[test]
fn a_method_returning_its_class_by_value_is_not_a_destructor() {
    for (fixture, not_dtors, pair) in [
        ("itaniumctors_byvalue_O2_x86_64.so", &BYVALUE_NOT_DTORS_X64[..], BYVALUE_PAIR_DTORS_X64),
        ("itaniumctors_byvalue_O2_i386.so", &BYVALUE_NOT_DTORS_I386[..], BYVALUE_PAIR_DTORS_I386),
    ] {
        let prog = bootstrap(fixture, true);
        for &vma in not_dtors {
            let name = name_at(&prog, vma);
            assert!(!name.starts_with('~'), "{fixture} {vma:#x} must keep its slot name, got {name}");
        }
        for vma in pair {
            assert_eq!(name_at(&prog, vma), "~Pair", "{fixture} {vma:#x} is bv::Pair::~Pair");
        }
    }
    let prog = bootstrap("itaniumctors_byvalue_O2_x86_64.so", true);
    for vma in BYVALUE_ZERO_X64 {
        let name = name_at(&prog, vma);
        assert!(!name.starts_with('~'), "{vma:#x} must keep its slot name, got {name}");
    }
}

#[test]
fn a_by_value_return_slot_is_not_a_constructor() {
    let prog = bootstrap("itaniumctors_byvalue_O2_x86_64.so", true);
    let name = name_at(&prog, BYVALUE_COLLECT_X64);
    assert!(name.starts_with("sub_"), "bv::Pager::collect must stay unnamed, got {name}");
}
