//! (kuna `msvcsig`) Hermetic tests for the MSVC declaration parse and the
//! prototype it builds: the convention, `this`, the hidden return pointer and
//! the by-value parameter cut are each a place a mistake moves every argument.

use super::*;
use kuna_decomp::dtype::TypeFactoryImpl;

fn decl(mangled: &str) -> MsvcDecl {
    let dem = demangle_raw(mangled).unwrap_or_else(|| panic!("{mangled} must demangle"));
    parse(&dem).unwrap_or_else(|| panic!("{mangled} -> {dem} must parse"))
}

fn try_decl(mangled: &str) -> Option<MsvcDecl> {
    parse(&demangle_raw(mangled)?)
}

/// A type factory shaped like the 32-bit x86 one a PE32 runs against.
fn factory() -> TypeFactoryImpl {
    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);
    types.setup_sizes(Some(4), 4, 4);
    types.set_core_type("char", 1, type_metatype::TYPE_INT, true).expect("char core type");
    types.cache_core_types().expect("cache core types");
    types
}

/// Build `mangled` as an x86-32 import (or definition) against `evidence`, and
/// render each input as `name:size`.
fn shape(mangled: &str, evidence: &[&str], import: bool) -> Option<(Vec<String>, Option<i32>, i32, Option<&'static str>)> {
    let types = factory();
    let nontrivial = nontrivial_classes(evidence.iter().copied());
    let sig = build(&decl(mangled), &nontrivial, import, true, &types, 4, 1)?;
    let ins = sig
        .pieces
        .intypes
        .iter()
        .map(|t| match t.get_ptr_to() {
            Some(to) => format!("{}*:{}", to.get_name(), t.get_size()),
            None => format!("{}:{}", t.get_name(), t.get_size()),
        })
        .collect();
    Some((ins, sig.pieces.outtype.map(|t| t.get_size()), sig.pieces.first_var_arg_slot, sig.model))
}

#[test]
fn thiscall_member_takes_this_and_states_its_return() {
    // `public: int __thiscall Arr::IsValid(void)`.
    let d = decl("?IsValid@Arr@@QAEHXZ");
    assert_eq!(d.convention, Convention::Thiscall);
    assert!(d.this);
    assert_eq!(d.class, "Arr");
    assert_eq!(d.ret, Ret::Type("int".into()));
    assert!(d.params.is_empty());
    let (ins, out, slot, model) = shape("?IsValid@Arr@@QAEHXZ", &[], true).unwrap();
    assert_eq!(ins.len(), 1, "only `this`: {ins:?}");
    assert_eq!(out, Some(4));
    assert_eq!(slot, -1);
    assert_eq!(model, Some("__thiscall"));
}

#[test]
fn copy_constructor_has_no_stated_return() {
    // `public: __thiscall Arr::Arr(class Arr const &)`.
    let d = decl("??0Arr@@QAE@ABV0@@Z");
    assert!(d.this);
    assert_eq!(d.ret, Ret::Unstated);
    assert_eq!(d.params, vec!["class Arr const &".to_string()]);
    let (ins, out, _, model) = shape("??0Arr@@QAE@ABV0@@Z", &[], true).unwrap();
    assert_eq!(ins.len(), 2);
    assert_eq!(out, None, "a constructor's return is left to recovery");
    assert_eq!(model, Some("__thiscall"));
}

#[test]
fn cdecl_free_function_has_no_this() {
    // `void __cdecl Note(long)`: `long` is 4 bytes on every Windows target.
    let d = decl("?Note@@YAXJ@Z");
    assert_eq!(d.convention, Convention::Cdecl);
    assert!(!d.this);
    let (ins, out, slot, model) = shape("?Note@@YAXJ@Z", &[], true).unwrap();
    assert_eq!(ins.len(), 1);
    assert!(ins[0].ends_with(":4"), "{ins:?}");
    assert_eq!(out, Some(0), "void");
    assert_eq!(slot, -1);
    assert_eq!(model, Some("__cdecl"));
}

#[test]
fn static_member_has_no_this() {
    let d = decl("?s@A@@SAHH@Z"); // public: static int __cdecl A::s(int)
    assert!(!d.this);
    assert_eq!(d.params.len(), 1);
}

#[test]
fn member_returning_a_class_takes_the_hidden_pointer_after_this() {
    // `public: class std::locale __thiscall std::ios_base::getloc(void) const`:
    // an instance method always returns a class through the hidden pointer.
    let (ins, out, slot, _) = shape("?getloc@ios_base@std@@QBE?AVlocale@2@XZ", &[], true).unwrap();
    assert_eq!(ins.len(), 2, "this, then the hidden pointer: {ins:?}");
    assert!(ins[1].starts_with("locale"), "{ins:?}");
    assert_eq!(out, Some(4), "the hidden pointer comes back in EAX");
    assert_eq!(slot, -1);
}

#[test]
fn free_function_returning_a_class_needs_evidence() {
    // `class Arr __cdecl Load(long)`: a trivially copyable 4-byte Arr would come
    // back in EAX, so with nothing to say otherwise the declaration is refused.
    assert!(shape("?Load@@YA?AVArr@@J@Z", &[], true).is_none());
    // The copy constructor, the destructor and the vftable each prove it cannot.
    for witness in ["??0Arr@@QAE@ABV0@@Z", "??1Arr@@QAE@XZ", "??_7Arr@@6B@", "??_GArr@@UAEPAXI@Z"] {
        let (ins, out, _, model) = shape("?Load@@YA?AVArr@@J@Z", &[witness], true)
            .unwrap_or_else(|| panic!("{witness} proves a hidden return"));
        assert_eq!(ins.len(), 2, "hidden pointer, then the long: {ins:?}");
        assert_eq!(out, Some(4));
        assert_eq!(model, Some("__cdecl"));
    }
    // A default constructor is not a witness: one with a member initializer is
    // emitted even for a class MSVC returns in a register.
    assert!(shape("?Load@@YA?AVArr@@J@Z", &["??0Arr@@QAE@XZ"], true).is_none());
    // Nor is a witness for a different class.
    assert!(shape("?Load@@YA?AVArr@@J@Z", &["??1Other@@QAE@XZ"], true).is_none());
}

#[test]
fn by_value_parameter_opens_the_tail_of_a_cdecl_import() {
    // `class Arr __cdecl MakeStack(class Arr,class Arr &,class Arr &)`.
    let witness = ["??0Arr@@QAE@ABV0@@Z"];
    let (ins, out, slot, model) = shape("?MakeStack@@YA?AVArr@@V1@AAV1@1@Z", &witness, true).unwrap();
    assert_eq!(ins.len(), 1, "only the hidden pointer is placed: {ins:?}");
    assert_eq!(out, Some(4));
    assert_eq!(slot, 1, "everything from the by-value class on is open");
    assert_eq!(model, Some("__cdecl"));
    // A defined function keeps its own parameter recovery instead.
    assert!(shape("?MakeStack@@YA?AVArr@@V1@AAV1@1@Z", &witness, false).is_none());
}

#[test]
fn by_value_parameter_of_a_callee_pops_convention_is_refused() {
    // `public: class Arr __thiscall Arr::operator=(class Arr)`: the callee pops
    // the by-value slot, whose size the name does not give.
    assert!(shape("??4Arr@@QAE?AV0@V0@@Z", &[], true).is_none());
    // `void __stdcall f(struct tagVARIANT)`.
    assert!(shape("?f@@YGXUtagVARIANT@@@Z", &[], true).is_none());
}

#[test]
fn operators_parse_with_their_symbols() {
    let d = decl("??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QAEAAV01@H@Z");
    assert!(d.this);
    assert_eq!(d.class, "basic_ostream");
    assert!(d.qualified.ends_with("::operator<<"), "{}", d.qualified);
    assert_eq!(d.params, vec!["int".to_string()]);
    assert!(matches!(d.ret, Ret::Type(ref t) if t.ends_with('&')));
    let d = decl("??RArr@@QAEHH@Z"); // operator()(int)
    assert_eq!(d.params, vec!["int".to_string()]);
    let d = decl("??MArr@@QBE_NABV0@@Z"); // operator<(class Arr const &) const
    assert_eq!(d.params.len(), 1);
    let d = decl("??BArr@@QBEHXZ"); // operator int(void) const
    assert!(d.params.is_empty());
    assert_eq!(d.ret, Ret::Type("int".into()));
    let d = decl("??2@YAPAXI@Z"); // void * __cdecl operator new(unsigned int)
    assert!(!d.this);
    assert_eq!(d.qualified, "operator new");
}

#[test]
fn template_class_destructor_and_constructor_parse() {
    let d = decl("??1?$basic_ios@DU?$char_traits@D@std@@@std@@UAE@XZ");
    assert!(d.this);
    assert_eq!(d.ret, Ret::Unstated);
    assert_eq!(d.class, "basic_ios");
    let ev = nontrivial_classes(["??1?$basic_ios@DU?$char_traits@D@std@@@std@@UAE@XZ"].into_iter());
    assert!(ev.contains("std::basic_ios<char,struct std::char_traits<char> >"), "{ev:?}");
}

#[test]
fn function_pointer_parameter_is_pointer_wide() {
    // `operator<<(std::ostream & (__cdecl *)(std::ostream &))`, the `endl` overload.
    let (ins, ..) = shape(
        "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QAEAAV01@P6AAAV01@AAV01@@Z@Z",
        &[],
        true,
    )
    .unwrap();
    assert_eq!(ins.len(), 2);
    assert!(ins[1].ends_with(":4"), "{ins:?}");
}

#[test]
fn sixty_four_bit_integers_take_two_slots() {
    // `int64_t __cdecl h(char *,uint64_t)`.
    let (ins, out, ..) = shape("?h@@YA_JPAD_K@Z", &[], true).unwrap();
    assert_eq!(ins.len(), 2);
    assert!(ins[1].ends_with(":8"), "{ins:?}");
    assert_eq!(out, Some(8));
}

#[test]
fn real_varargs_are_recorded() {
    let (ins, _, slot, _) = shape("?v@@YAXPBDZZ", &[], true).unwrap();
    assert_eq!(ins.len(), 1);
    assert_eq!(slot, 1);
}

#[test]
fn special_names_and_data_are_not_declarations() {
    assert!(try_decl("??_7Arr@@6B@").is_none(), "vftable");
    assert!(try_decl("??_GArr@@UAEPAXI@Z").is_none(), "scalar deleting destructor");
    assert!(try_decl("?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A").is_none(), "data");
    assert!(try_decl("?f@A@@W3AEXXZ").is_none(), "adjustor thunk");
}

#[test]
fn function_template_is_refused() {
    // `void __cdecl f<int>(int)`: the same policy as the Itanium arm.
    assert!(try_decl("??$f@H@@YAXH@Z").is_none());
}

#[test]
fn x64_keeps_the_default_model() {
    // `public: class B __cdecl A::f(int)` on x64: no model, but the hidden
    // pointer still follows `this`.
    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);
    types.setup_sizes(Some(8), 8, 4);
    types.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    types.cache_core_types().unwrap();
    let sig = build(&decl("?f@A@@QEAA?AVB@@H@Z"), &HashSet::new(), true, false, &types, 8, 1).unwrap();
    assert_eq!(sig.model, None);
    assert_eq!(sig.pieces.intypes.len(), 3);
    // A by-value parameter is refused off x86-32, import or not.
    assert!(build(&decl("?MakeStack@@YA?AVArr@@V1@AAV1@1@Z"), &HashSet::new(), true, false, &types, 8, 1).is_none());
}

#[test]
fn unknown_conventions_are_refused() {
    assert!(try_decl("?f@@YQXH@Z").is_none(), "__vectorcall");
}

