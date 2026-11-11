//! The fixes of ktenum on their kuna-fixes cases (the enum constants in Kotlin and in Java output),
//! each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

fn text_with(id: &str, fixes: KunaFixes, java: bool) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    if java {
        case.options.push(("kt-enable".into(), "0".into()));
        case.compare = case
            .compare
            .iter()
            .map(|c| c.replace(".kt", ".java"))
            .collect();
    }
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn constants_built_in_the_values_array_keep_their_arguments() {
    let without = KunaFixes::DEFAULT.without(KunaFix::EnumConstantInValuesArray);
    for java in [false, true] {
        let on = text_with("pkg/TestEnumValuesArray", KunaFixes::DEFAULT, java);
        let off = text_with("pkg/TestEnumValuesArray", without, java);
        for (fixed, lost) in [
            ("   FIRST(7, \"one\"),\n", "   FIRST,\n"),
            ("   SECOND(-3, null),\n", "   SECOND,\n"),
            ("   THIRD(42, \"three\");\n", "   THIRD;\n"),
        ] {
            assert!(on.contains(fixed) && !on.contains(lost), "{on}");
            assert!(off.contains(lost) && !off.contains(fixed), "{off}");
        }
    }
}

/// The case's default text, and its text with `fix` alone off.
fn on_off(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT, false),
        text_with(id, KunaFixes::DEFAULT.without(fix), false),
    )
}

#[test]
fn extension_properties_keep_their_receiver() {
    let (on, off) = on_off("pkg/TestExtensionProperty", KunaFix::KtExtensionProperty);
    for (fixed, lost) in [
        ("public val String.firstChar: Char\n", "public val firstChar: Char\n"),
        ("public val <T> List<T>.secondOrNull: T?\n", "public val secondOrNull: T?\n"),
        ("public var StringBuilder.lastChar: Char\n", "public var lastChar: Char\n"),
        ("      this.setCharAt(this.length - 1, value)\n", "`$this$lastChar`.setCharAt("),
    ] {
        assert!(on.contains(fixed) && !on.contains(lost), "{on}");
        assert!(off.contains(lost) && !off.contains(fixed), "{off}");
    }
    let (on, off) = on_off("pkg/TestMemberExtensionProperty", KunaFix::KtExtensionProperty);
    let fixed = "         return this * this@TestMemberExtensionProperty.scale\n";
    assert!(on.contains("private final val Int.scaled: Int") && on.contains(fixed), "{on}");
    assert!(off.contains("`$this$scaled` * this.scale") && !off.contains(fixed), "{off}");
}

#[test]
fn interface_companion_field_is_hidden() {
    let (on, off) = on_off("pkg/TestInterfaceCompanionField", KunaFix::KtInterfaceCompanionField);
    let field = "TestInterfaceCompanionField.Key Key = TestInterfaceCompanionField.Key.`$$INSTANCE`;";
    assert!(!on.contains(field) && on.contains("public companion object Key {"), "{on}");
    assert!(off.contains(field), "{off}");
}

#[test]
fn jvm_overloads_methods_are_hidden() {
    let (on, off) = on_off("pkg/TestJvmOverloads", KunaFix::KtGeneratedOverloadHidden);
    for overload in ["fun scaled(x: Int): Int {", "fun scaled(x: Int, factor: Int): Int {", "`join$default`("] {
        assert!(!on.contains(overload), "{on}");
        assert!(off.contains(overload), "{off}");
    }
    assert!(on.contains("public fun scaled(x: Int, factor: Int = 2, offset: Int = this.base): Int {"), "{on}");
}

#[test]
fn boxed_receiver_keeps_its_type_argument() {
    let on = text_with("pkg/TestBoxedReceiverTypeArgument", KunaFixes::DEFAULT, false);
    assert!(on.contains("this.firstOr<Int>(bound as MutableList<out Int>)"), "{on}");
    assert!(!on.contains("this.firstOr(bound"), "{on}");
}
