//! (kuna `cppsig`) Hermetic tests for the demangled-signature parse and the
//! `this` decision — the two places a mistake shifts every parameter by one.

use super::*;
use crate::demangle::demangle_raw;

fn decl(mangled: &str) -> Decl {
    let dem = demangle_raw(mangled).unwrap_or_else(|| panic!("{mangled} must demangle"));
    parse_decl(&dem).unwrap_or_else(|| panic!("{mangled} -> {dem} must parse"))
}

fn try_decl(mangled: &str) -> Option<Decl> {
    parse_decl(&demangle_raw(mangled)?)
}

#[test]
fn destructor_is_a_proven_this() {
    // `leveldb::Cache::~Cache()` — the headline case (15 of the 25 leveldb
    // improvements are destructors).  A destructor cannot be static.
    let d = decl("_ZN7leveldb5CacheD2Ev");
    assert_eq!(d.this_kind, ThisKind::Proven);
    assert_eq!(d.class, "Cache");
    assert_eq!(d.scope, "leveldb::Cache");
    assert!(d.params.is_empty(), "a destructor declares no parameters");
}

#[test]
fn constructor_is_a_proven_this() {
    // `leveldb::Options::Options()` — name equals the enclosing class.
    let d = decl("_ZN7leveldb7OptionsC1Ev");
    assert_eq!(d.this_kind, ThisKind::Proven);
    assert_eq!(d.class, "Options");
}

#[test]
fn const_member_is_a_proven_this() {
    // `_ZNK` — the trailing `const` can only qualify an implicit object
    // parameter, so a static member can never carry it.
    let d = decl("_ZNK7leveldb12TableBuilder10NumEntriesEv");
    assert_eq!(d.this_kind, ThisKind::Proven);
    assert_eq!(d.class, "TableBuilder");
    assert_eq!(d.qualified, "leveldb::TableBuilder::NumEntries");
}

#[test]
fn plain_nested_member_is_ambiguous() {
    // `leveldb::TableBuilder::WriteBlock(BlockBuilder*, BlockHandle*)` is a
    // non-static member, and `leveldb::Status::OK()` is a STATIC one — Itanium
    // mangles both the same way, so neither is proven.
    let d = decl("_ZN7leveldb12TableBuilder10WriteBlockEPNS_12BlockBuilderEPNS_11BlockHandleE");
    assert_eq!(d.this_kind, ThisKind::Ambiguous);
    assert_eq!(d.class, "TableBuilder");
    assert_eq!(d.params, vec!["leveldb::BlockBuilder*", "leveldb::BlockHandle*"]);

    let s = decl("_ZN7leveldb6Status2OKEv");
    assert_eq!(s.this_kind, ThisKind::Ambiguous, "a static member must not be proven");
}

#[test]
fn unqualified_name_proves_there_is_no_this() {
    // `_Z3fooi` -> `foo(int)`: no `::`, so no implicit object parameter can
    // exist and the declared parameter applies at position 0.
    let d = decl("_Z3fooi");
    assert_eq!(d.this_kind, ThisKind::ProvenNone);
    assert_eq!(d.qualified, "foo");
    assert_eq!(d.params, vec!["int"]);
}

#[test]
fn namespaced_free_function_is_ambiguous_not_proven() {
    // `leveldb::NewMemEnv(leveldb::Env*)` has no `this`, but the mangling cannot
    // say so — only the class-evidence inference can.  It must NOT be proven, or
    // every parameter would shift.
    let d = decl("_ZN7leveldb9NewMemEnvEPNS_3EnvE");
    assert_eq!(d.this_kind, ThisKind::Ambiguous);
    assert_eq!(d.scope, "leveldb");
    assert_eq!(d.params, vec!["leveldb::Env*"]);
}

#[test]
fn overloaded_operators_are_refused() {
    // `operator<`/`operator>`/`operator()` put UNBALANCED brackets into the
    // demangled text, which the depth-tracking parse cannot survive, and the free
    // operator templates are the densest source of false `this` positives.
    assert!(try_decl("_ZN9__gnu_cxxltIPmSt6vectorImSaImEEEEbRKNS_17__normal_iteratorIT_T0_EESA_").is_none());
    assert!(try_decl("_ZNSt6vectorIiSaIiEEixEm").is_none());
}

#[test]
fn a_data_symbol_is_not_a_declaration() {
    // `_ZN3foo3barE` is a variable, not a function: no parameter list, no parse.
    assert!(try_decl("_ZN3foo3barE").is_none());
}

#[test]
fn explicit_function_template_specializations_are_refused() {
    // `_Z5maxofIiET_S0_S0_` -> `int maxof<int>(int, int)`. Refused: an explicit
    // function-template specialization's demangled parameter list is not
    // trustworthy. `cpp_demangle` renders this two-parameter forwarding
    // constructor with ONE parameter, and a short list leaves a live argument
    // register undeclared:
    let short = demangle_raw("_ZNSt4pairIPSt18_Rb_tree_node_baseS1_EC1IRS1_S4_Lb1EEEOT_OT0_")
        .expect("demangles");
    assert!(
        short.ends_with("(std::_Rb_tree_node_base*&)"),
        "the crate really does drop a parameter here: {short}"
    );
    assert!(try_decl("_ZNSt4pairIPSt18_Rb_tree_node_baseS1_EC1IRS1_S4_Lb1EEEOT_OT0_").is_none());
    assert!(try_decl("_Z5maxofIiET_S0_S0_").is_none());
    // It is also where the module's known template collision lives: the `int`
    // and `double` instantiations both reduce to `maxof`.
    assert!(try_decl("_Z5maxofIdET_S0_S0_").is_none());
    // A templated CLASS with an ordinary member is still accepted — only an
    // explicitly specialized FUNCTION name is refused.
    let d = decl("_ZNSt6vectorIiSaIiEE9push_backERKi");
    assert_eq!(d.class, "vector");
    assert_eq!(d.params, vec!["int const&".to_string()]);
}

/// A C++ **reference** parameter is rendered as a POINTER — a reference is a
/// pointer at the ABI level, it is what `cppproto` maps `DW_TAG_reference_type`
/// to, and it is what the DWARF ground truth expects.
#[test]
fn reference_parameters_become_pointers() {
    let d = decl("_ZNSt6vectorIiSaIiEE9push_backERKi");
    assert_eq!(d.params, vec!["int const&".to_string()]);
    // `const Slice&` and `Slice*` must map to the same kuna type shape.
    let s = decl("_ZN7leveldb6Status10CorruptionERKNS_5SliceES3_");
    assert_eq!(s.params, vec!["leveldb::Slice const&".to_string(); 2]);
}

#[test]
fn varargs_slot_is_recorded() {
    // `_Z3fmtPKcz` -> `fmt(char const*, ...)`.
    let d = decl("_Z3fmtPKcz");
    assert!(d.varargs);
    assert_eq!(d.params, vec!["char const*"]);
}

#[test]
fn signature_parens_finds_the_top_level_group() {
    let (p, a, s) = signature_parens("ns::f(std::pair<int, int>, char)").unwrap();
    assert_eq!(p, "ns::f");
    assert_eq!(a, "std::pair<int, int>, char");
    assert_eq!(s, "");
    let (_, _, s) = signature_parens("ns::C::f() const").unwrap();
    assert_eq!(s.trim(), "const");
    assert!(signature_parens("just_a_name").is_none());
}

#[test]
fn split_params_respects_template_commas() {
    assert_eq!(
        split_params("std::pair<int, int>, char, Foo<A, B>*"),
        vec!["std::pair<int, int>", " char", " Foo<A, B>*"]
    );
    assert!(split_params("").is_empty());
}

#[test]
fn last_token_drops_a_return_type_but_not_a_template_space() {
    assert_eq!(last_token("bool ns::f"), "ns::f");
    assert_eq!(last_token("ns::C<int, char>::f"), "ns::C<int, char>::f");
    assert_eq!(last_token("ns::f"), "ns::f");
}

#[test]
fn cv_ref_only_accepts_exactly_the_member_suffixes() {
    assert!(is_cv_ref_only(""));
    assert!(is_cv_ref_only(" const"));
    assert!(is_cv_ref_only(" const &"));
    assert!(is_cv_ref_only(" volatile &&"));
    assert!(!is_cv_ref_only(" (void)"));
    assert!(!is_cv_ref_only(" ::foo"));
}

#[test]
fn qualifier_word_strip_respects_identifier_boundaries() {
    assert_eq!(strip_qualifier_word("Slice const", "const"), Some("Slice "));
    assert_eq!(strip_qualifier_word("Xconst", "const"), None);
    assert_eq!(strip_qualifier_word("const", "const"), Some(""));
}

#[test]
fn strip_template_args_leaves_the_bare_name() {
    assert_eq!(strip_template_args("basic_string<char, traits<char> >"), "basic_string");
    assert_eq!(strip_template_args("Cache"), "Cache");
    assert_eq!(strip_template_args("~Cache"), "~Cache");
}

#[test]
fn msvc_states_access_and_static_outright() {
    // MSVC answers the whole question that Itanium leaves open. A free function
    // carries no access specifier...
    let d = decl("?g@ns@@YAHHH@Z"); // int __cdecl ns::g(int,int)
    assert_eq!(d.this_kind, ThisKind::ProvenNone);
    assert_eq!(d.params, vec!["int".to_string(), "int".to_string()]);
    // ...a non-static member does...
    let m = decl("?foo@Bar@@QEAAXXZ"); // public: void __cdecl Bar::foo(void)
    assert_eq!(m.this_kind, ThisKind::Proven);
    assert_eq!(m.class, "Bar");
    // ...and a STATIC member says so, which Itanium never does.
    let s = decl("?sfoo@Bar@@SAHH@Z"); // public: static int __cdecl Bar::sfoo(int)
    assert_eq!(s.this_kind, ThisKind::ProvenNone);
}

#[test]
fn msvc_32bit_thiscall_is_refused_not_misplaced() {
    // `?foo@Bar@@QAEXXZ` = `public: void __thiscall Bar::foo(void)`. The 32-bit
    // MSVC ABI passes `this` in ECX, NOT as ordinary argument 0, so placing it
    // positionally would be wrong; refuse the symbol until the `__thiscall`
    // prototype model can be selected.
    assert!(try_decl("?foo@Bar@@QAEXXZ").is_none());
}
