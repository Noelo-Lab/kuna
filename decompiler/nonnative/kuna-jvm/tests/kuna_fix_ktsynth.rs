//! The fixes `kt_enum_when_mappings`, `kt_constructor_accessor_call` and
//! `kt_empty_primary_constructor`, and the receivers `kt_extension_call` and
//! the branching delegations `kt_legacy_constructor_parameter_checks` now
//! cover, on their kuna-fixes cases, each alone off (`KunaFixes::without`)
//! and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn an_enum_when_names_the_constants() {
    let id = "pkg/TestKtEnumWhenMappings";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    assert!(!fixed.contains("EnumSwitchMapping"), "{fixed}");
    for line in [
        "      when (c) {\n         TestKtEnumWhenMappings.Color.RED -> var10000 = \"red\"\n",
        "         TestKtEnumWhenMappings.Color.RED, TestKtEnumWhenMappings.Color.BLUE -> var4 = 1 + 2\n         else -> var4 = 1 + 3\n",
        "      when (s) {\n         TestKtEnumWhenMappings.Size.LARGE -> var4 *= 5\n",
        "         null -> var10000 = \"none\"\n         TestKtEnumWhenMappings.Color.GREEN -> var10000 = \"go\"\n         else -> var10000 = \"stop\"\n",
        "         TestKtEnumWhenMappings.Size.MEDIUM -> var10000 = 2\n         else -> throw NoWhenBranchMatchedException()\n",
    ] {
        assert!(fixed.contains(line), "{line}\n{fixed}");
    }
    let broken = text_with(id, KunaFixes::DEFAULT.without(KunaFix::KtEnumWhenMappings));
    assert_eq!(broken.matches("EnumSwitchMapping").count(), 6, "{broken}");
}

#[test]
fn an_accessor_call_names_the_real_constructor() {
    let id = "pkg/TestKtConstructorAccessorCall";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    for call in [
        "return TestKtConstructorAccessorCall(name)\n",
        "return TestKtConstructorAccessorCall(name, size)\n",
        "return TestKtConstructorAccessorCall(name, 2L)\n",
    ] {
        assert!(fixed.contains(call), "{call}\n{fixed}");
    }
    assert!(!fixed.contains(", null)"), "{fixed}");
    let broken = text_with(id, KunaFixes::DEFAULT.without(KunaFix::KtConstructorAccessorCall));
    assert_eq!(broken.matches(", null)").count(), 3, "{broken}");
}

#[test]
fn an_empty_primary_constructor_keeps_its_parentheses() {
    let id = "pkg/TestKtEmptyPrimaryConstructor";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    assert!(fixed.contains("public class TestKtEmptyPrimaryConstructor() {\n"), "{fixed}");
    assert!(fixed.contains("   public class Builder() {\n"), "{fixed}");
    let broken = text_with(id, KunaFixes::DEFAULT.without(KunaFix::KtEmptyPrimaryConstructor));
    assert!(broken.contains("public class TestKtEmptyPrimaryConstructor {\n"), "{broken}");
    assert!(broken.contains("   public class Builder {\n"), "{broken}");
}

#[test]
fn a_member_extension_on_a_copy_of_this_is_called_on_its_receiver() {
    let id = "pkg/TestKtMemberExtensionReceiver";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    assert!(
        fixed.contains(".seconds = unit.toSeconds(value.toLong()).clampToInt()\n"),
        "{fixed}"
    );
    let broken = text_with(id, KunaFixes::DEFAULT.without(KunaFix::KtExtensionCall));
    assert!(broken.contains(".clampToInt(unit.toSeconds("), "{broken}");
}

#[test]
fn a_legacy_check_before_a_branching_delegation_is_dropped() {
    let id = "pkg/TestKtLegacyCheckBranchingDelegation";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("constructor(response: pkg.TestKtLegacyCheckBranchingDelegation.Response)  {\n"),
        "{fixed}"
    );
    assert!(fixed.contains("            this(var10001, var2)\n"), "{fixed}");
    let broken = text_with(
        id,
        KunaFixes::DEFAULT.without(KunaFix::KtLegacyConstructorParameterChecks),
    );
    assert!(
        broken.contains("Response) : Intrinsics.checkParameterIsNotNull(response, \"response\") {"),
        "{broken}"
    );
}
