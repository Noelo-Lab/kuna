//! (kuna `swiftmetadata`) The name derivations the pass makes from imported
//! descriptor names; the end-to-end naming is `verify_swiftmetadata`.

use super::*;

#[test]
fn member_of_an_imported_method_descriptor() {
    assert_eq!(
        member_after_protocol("Swift.Hashable.hash(into:)").as_deref(),
        Some("hash(into:)")
    );
    assert_eq!(
        member_after_protocol("Swift.Hashable.hashValue.getter").as_deref(),
        Some("hashValue.getter")
    );
    assert_eq!(
        member_after_protocol("Swift.Equatable.==(_:_:)").as_deref(),
        Some("==(_:_:)")
    );
    assert_eq!(
        member_after_protocol("__C.UIHostingController.init(coder:rootView:)").as_deref(),
        Some("init(coder:rootView:)")
    );
    // A dot inside a parameter list does not split the member.
    assert_eq!(member_after_protocol("M.P.f(_:)").as_deref(), Some("f(_:)"));
    assert_eq!(
        member_after_protocol("M.P.sub(x:)").as_deref(),
        Some("sub(x:)")
    );
    assert_eq!(member_after_protocol("nodots"), None);
}

#[test]
fn import_names_strip_their_descriptor_tag() {
    let mut imports = HashMap::new();
    imports.insert(0x1000u64, "_$s7SwiftUIMXM".to_string());
    imports.insert(0x1008u64, "_$sSHMp".to_string());
    imports.insert(0x1010u64, "_$sSo8NSObjectCMn".to_string());
    assert_eq!(
        import_name(&imports, 0x1000, "module_descriptor").as_deref(),
        Some("SwiftUI")
    );
    assert_eq!(
        import_name(&imports, 0x1008, "protocol_descriptor").as_deref(),
        Some("Swift.Hashable")
    );
    assert_eq!(
        import_name(&imports, 0x1010, "nominal_type_descriptor").as_deref(),
        Some("__C.NSObject")
    );
    // The wrong tag, or an unknown slot, names nothing.
    assert_eq!(import_name(&imports, 0x1008, "module_descriptor"), None);
    assert_eq!(import_name(&imports, 0x2000, "module_descriptor"), None);
}

/// Several records claiming one function (code the linker folded): one
/// type's claims keep the first member with the count served, one protocol
/// member's claims from different types drop the type, anything else keeps
/// the placeholder; a name already given to another address gets `.2`.
#[test]
fn folded_functions_are_named_without_a_false_owner() {
    let md = Metadata::default();
    let imports = HashMap::new();
    let namer = Namer {
        md: &md,
        imports: &imports,
        memo: Default::default(),
        code: vec![(0x1000, 0x2000)],
        claims: Default::default(),
    };
    let claim = |addr: u64, name: &str, owner: &str, req: Option<(&str, &str)>, getter: bool| {
        namer.claim(
            addr,
            name.to_string(),
            owner,
            req.map(|(p, m)| (p.to_string(), m.to_string())),
            getter,
        );
    };
    claim(0x1000, "M.A.vtable#0.getter", "M.A", None, true);
    claim(
        0x1010,
        "<M.A:M.P>.requirement#0",
        "M.A",
        Some(("M.P", "requirement#0")),
        true,
    );
    claim(
        0x1010,
        "<M.A:M.P>.requirement#1",
        "M.A",
        Some(("M.P", "requirement#1")),
        false,
    );
    claim(
        0x1010,
        "<M.A:M.P>.requirement#1",
        "M.A",
        Some(("M.P", "requirement#1")),
        false,
    );
    claim(
        0x1020,
        "<M.A:Swift.Hashable>.hash(into:)",
        "M.A",
        Some(("Swift.Hashable", "hash(into:)")),
        false,
    );
    claim(
        0x1020,
        "<M.B:Swift.Hashable>.hash(into:)",
        "M.B",
        Some(("Swift.Hashable", "hash(into:)")),
        false,
    );
    claim(
        0x1030,
        "<M.C:Swift.Hashable>.hash(into:)",
        "M.C",
        Some(("Swift.Hashable", "hash(into:)")),
        false,
    );
    claim(
        0x1030,
        "<M.D:Swift.Hashable>.hash(into:)",
        "M.D",
        Some(("Swift.Hashable", "hash(into:)")),
        false,
    );
    claim(
        0x1040,
        "<M.A:M.P>.requirement#2",
        "M.A",
        Some(("M.P", "requirement#2")),
        true,
    );
    claim(0x1040, "M.B.vtable#3.getter", "M.B", None, true);
    claim(0x1050, "M.E.vtable#1.getter", "M.E", None, true);
    claim(0x1050, "M.E.vtable#2.getter", "M.E", None, true);
    claim(0x3000, "M.F.vtable#0.getter", "M.F", None, true);
    let mut out = AnalysisOutput::default();
    namer.resolve_claims(&mut out);
    let names: Vec<(u64, &str)> = out
        .fid_names
        .iter()
        .map(|f| (f.addr, f.name.as_str()))
        .collect();
    assert_eq!(
        names,
        vec![
            (0x1000, "M.A.vtable#0.getter"),
            (0x1010, "<M.A:M.P>.requirement#0.merged#2"),
            (0x1020, "<*:Swift.Hashable>.hash(into:)"),
            (0x1030, "<*:Swift.Hashable>.hash(into:).2"),
            (0x1050, "M.E.vtable#1.getter.merged#2"),
        ]
    );
    assert!(!namer.ambiguous_self(0x1000));
    assert!(namer.ambiguous_self(0x1010), "a method shares the body");
    assert!(namer.ambiguous_self(0x1020), "two types share the body");
    assert!(namer.ambiguous_self(0x1040));
    assert!(!namer.ambiguous_self(0x1050), "two getters of one class");
    assert!(!namer.ambiguous_self(0x3000), "not code: never claimed");
}
