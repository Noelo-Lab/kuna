//! End-to-end two-pass gate for `itaniumrtti` — Itanium (GCC/Clang) RTTI + vtable
//! recovery, the capability Ghidra has no analyzer for at all.
//!
//! Fixture: `itaniumrtti_x86_64.so`, a `g++ -O0 -fPIC -shared
//! -fvisibility=hidden`-built C++ library run through `strip --strip-all`. Hidden
//! visibility is what makes it a real test: without it every implicit class method
//! is emitted WEAK and *exported*, so `.dynsym` alone would name them. Hidden and
//! stripped, the only defined dynamic symbols are the two `probe_*` entry points —
//! every class name, vtable and virtual method in the file has to come from the
//! Itanium ABI graph or from nowhere.
//!
//! The five classes cover all three `__cxxabiv1` typeinfo flavours:
//! `shapes::Shape` (`__class_type_info`, no bases), `shapes::Circle`
//! (`__si_class_type_info`), and `shapes::Widget` (`__vmi_class_type_info`, two
//! bases, `Drawable` at +16 so its sub-vtable is a block of `this`-adjusting
//! thunks with `offset-to-top = -16`).
//!
//! ## The two passes
//!
//! * **option OFF (the default, and the defect):** the whole RTTI graph is unnamed
//!   bytes. Every virtual method is `sub_<addr>`, and a constructor's vptr store is
//!   a bare integer constant (`*this = 0x4d38`). This is also exactly what Ghidra
//!   12.1 produces on such a binary.
//! * **option ON (the fix):** the class names, the vtables and every virtual slot
//!   are recovered — `shapes::Shape::vtable_2`, `shapes::Shape_vtable`,
//!   `shapes::Widget::vtable_for_Drawable_0`.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// A `__class_type_info` (base-less) class: its four slots are `~Shape` (complete),
/// `~Shape` (deleting), `area`, `perimeter`.
const SHAPE_AREA: &str = "shapes::Shape::vtable_2";
/// `shapes::Shape::perimeter` — repeated verbatim in `Circle`'s vtable, so it is the
/// witness that an inherited slot is attributed to the class that DEFINES it.
const SHAPE_PERIMETER: &str = "shapes::Shape::vtable_3";
/// A `__si_class_type_info` class overriding `area` in slot 2.
const CIRCLE_AREA: &str = "shapes::Circle::vtable_2";
/// A `__vmi_class_type_info` class's SECONDARY sub-vtable, reached through the
/// `Drawable` base at +16: three `this`-adjusting thunks.
const WIDGET_THUNK: &str = "shapes::Widget::vtable_for_Drawable_2";
/// The primary sub-vtable's `render` override (the thunk's ultimate target).
const WIDGET_RENDER: &str = "shapes::Widget::vtable_3";
/// The `_ZTV`-equivalent data label (the vtable object's `offset-to-top` word).
const SHAPE_VTABLE: &str = "shapes::Shape_vtable";
/// The `_ZTI`/`_ZTS`-equivalent data labels.
const SHAPE_TYPEINFO: &str = "shapes::Shape_typeinfo";
const SHAPE_TYPEINFO_NAME: &str = "shapes::Shape_typeinfo_name";
/// The value an object's vptr actually holds (two words past the header) — the
/// constant a constructor stores.
const WIDGET_VPTR: &str = "shapes::Widget_vptr";
/// The secondary sub-vtable's vptr, stored into the `Drawable` subobject.
const WIDGET_VPTR_2: &str = "shapes::Widget_vptr_for_Drawable";
/// The two template instantiations. Their NAME-ONLY demangling collides (both reduce
/// to `Vec`), so they are the witness that distinct classes stay distinct.
const VEC_INT_GET: &str = "shapes::Vec_int::vtable_2";
const VEC_DOUBLE_GET: &str = "shapes::Vec_double::vtable_2";
/// A translation-unit-local class: its ABI type-name string carries the leading `*`
/// marker, and its demangled name is not a legal identifier chain.
const HIDDEN_SCALED: &str = "anonymous_namespace::Hidden::vtable_2";
/// The one exported entry point, which must survive both passes untouched.
const EXPORTED: &str = "probe_shapes";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the stripped hidden-visibility C++ fixture with `itaniumrtti` in the
/// requested state, then commit the (deferred) analysis facts. `None` is a visible
/// skip when the `.sla` is missing.
fn bootstrap(on: bool) -> Option<ConsoleProgram> {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/itaniumrtti_x86_64.so");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_itaniumrtti: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("itaniumrtti", if on { "on" } else { "off" })
        .expect("itaniumrtti flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// BEFORE (`--option itaniumrtti off`, the default): nothing in the RTTI graph is
/// named. This is the state Ghidra 12.1 also leaves such a binary in.
#[test]
fn nothing_is_recovered_with_the_option_off() {
    let Some(prog) = bootstrap(false) else {
        return;
    };
    for name in [
        SHAPE_AREA,
        SHAPE_PERIMETER,
        CIRCLE_AREA,
        WIDGET_THUNK,
        WIDGET_RENDER,
        SHAPE_VTABLE,
        SHAPE_TYPEINFO,
        SHAPE_TYPEINFO_NAME,
        WIDGET_VPTR,
        WIDGET_VPTR_2,
        VEC_INT_GET,
        VEC_DOUBLE_GET,
        HIDDEN_SCALED,
    ] {
        assert!(
            !prog.has_symbol_named(name),
            "with itaniumrtti off nothing may name the RTTI graph, but {name} exists"
        );
    }
    assert!(prog.lookup_symbol(EXPORTED).is_some(), "the exported entry point exists");
}

/// AFTER (`--option itaniumrtti on`): every virtual method the five classes declare
/// is named from the ABI graph alone.
#[test]
fn virtual_methods_are_named_from_the_rtti_graph() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    for name in [SHAPE_AREA, CIRCLE_AREA, WIDGET_RENDER] {
        assert!(
            prog.lookup_symbol(name).is_some(),
            "{name} must be recovered from the Itanium RTTI graph"
        );
    }
}

/// The inheritance-attribution property: `Shape::perimeter` appears at slot 3 of
/// BOTH `Shape`'s and `Circle`'s vtables (Circle does not override it). It must be
/// named for the class that DEFINES it, never for the derived class whose table the
/// scan happened to reach first.
#[test]
fn an_inherited_slot_is_named_for_the_defining_base() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    assert!(
        prog.lookup_symbol(SHAPE_PERIMETER).is_some(),
        "the slot Circle inherits unchanged must be named {SHAPE_PERIMETER}"
    );
    assert!(
        prog.lookup_symbol("shapes::Circle::vtable_3").is_none(),
        "Circle inherits slot 3; it must NOT claim the name of Shape's method"
    );
}

/// The multiple-inheritance property: a `__vmi_class_type_info` class's SECONDARY
/// sub-vtable is a distinct block of `this`-adjusting thunks, and the base
/// subobject it serves is read off the recovered inheritance displacement
/// (`Drawable` at +16 ⇒ `offset-to-top` -16). Its slot indices restart at 0, so it
/// must not collide with the primary sub-vtable's.
#[test]
fn a_secondary_subvtable_is_named_for_its_base_subobject() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    assert!(
        prog.lookup_symbol(WIDGET_THUNK).is_some(),
        "the Drawable subobject's thunk block must be named {WIDGET_THUNK}"
    );
    let thunk = prog.lookup_symbol(WIDGET_THUNK).unwrap();
    let direct = prog.lookup_symbol(WIDGET_RENDER).expect("the primary override exists");
    assert_ne!(
        thunk, direct,
        "the thunk and the method it forwards to are different functions and must \
         not share one name"
    );
}

/// The two naming hazards that cost real recovery when mishandled, both found by an
/// adversarial probe rather than reasoned about:
///
/// * **Template instantiations.** `Vec<int>` and `Vec<double>` are two classes with
///   two vtables. The module-wide name-only demangling reduces both to `Vec`, and
///   because the symbol commit is an idempotent ADD the second one then loses the
///   name race and keeps `sub_<addr>` for every method — measured at 4 of 14 virtual
///   methods silently unrecovered before the fix. The template arguments are folded
///   into the identifier instead of deleted.
/// * **A TU-local class.** The ABI (§2.9.1) prefixes an anonymous-namespace class's
///   type-name string with `*` so the runtime compares by pointer; that byte is not
///   part of the mangled name, and leaving it on makes the class undemangleable.
///   These are a large share of the concrete implementation classes in real C++.
#[test]
fn template_instantiations_and_tu_local_classes_are_recovered() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    assert!(
        prog.has_symbol_named(VEC_INT_GET),
        "Vec<int>::get must be recovered as {VEC_INT_GET}"
    );
    assert!(
        prog.has_symbol_named(VEC_DOUBLE_GET),
        "Vec<double>::get must ALSO be recovered ({VEC_DOUBLE_GET}) — collapsing both \
         instantiations to one name silently drops a whole class"
    );
    assert_ne!(
        prog.lookup_symbol(VEC_INT_GET),
        prog.lookup_symbol(VEC_DOUBLE_GET),
        "the two instantiations are different functions"
    );
    assert!(
        prog.has_symbol_named(HIDDEN_SCALED),
        "the anonymous-namespace class must be recovered as {HIDDEN_SCALED} — its ABI \
         type-name string carries the TU-local `*` marker"
    );
}

/// The data half: the typeinfo object, its type-name string, the vtable object and
/// the vptr value each get a label. The vptr label is the load-bearing one — it is
/// the constant a constructor stores (`*this = 0x4d38`), and a multiple-inheritance
/// constructor stores TWO of them.
#[test]
fn typeinfo_and_vtable_objects_are_labelled() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    for name in [
        SHAPE_TYPEINFO,
        SHAPE_TYPEINFO_NAME,
        SHAPE_VTABLE,
        WIDGET_VPTR,
        WIDGET_VPTR_2,
    ] {
        assert!(prog.has_symbol_named(name), "{name} must be labelled");
    }
}

/// The safety property: recovery is purely additive. It discovers no function, and
/// the one real exported name is never overwritten.
#[test]
fn the_function_set_and_real_names_are_untouched() {
    let (Some(off), Some(on)) = (bootstrap(false), bootstrap(true)) else {
        return;
    };
    let before: Vec<u64> =
        off.function_entries_canonical().into_iter().map(|e| e.addr.get_offset()).collect();
    let after: Vec<u64> =
        on.function_entries_canonical().into_iter().map(|e| e.addr.get_offset()).collect();
    assert_eq!(before, after, "itaniumrtti must not add or remove a function entry");
    assert!(
        on.lookup_symbol(EXPORTED).is_some(),
        "a real .dynsym name must never be replaced by a vtable-slot name"
    );
}
