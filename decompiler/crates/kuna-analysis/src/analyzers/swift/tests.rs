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
