//! The fixes `kt_field_without_property`, `kt_constructor_without_metadata` and
//! `kt_delegation_call_arguments` on their kuna-fixes cases,
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

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

fn fixed_and_without(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT),
        text_with(id, KunaFixes::DEFAULT.without(fix)),
    )
}

const CONTINUATION: &str = "pkg/TestMembersWithoutMetadataKt$counting$$inlined$feed$1$1";
const CELL: &str = "pkg/TestMembersWithoutMetadataKt$named$$inlined$cell$1";

#[test]
fn fields_without_a_property_are_written_as_properties() {
    let (fixed, without) = fixed_and_without(CONTINUATION, KunaFix::KtFieldWithoutProperty);
    for line in [
        "   @JvmField internal var label: Int = 0\n",
        "   @JvmField internal var `L$0`: Any? = null\n",
        "   @JvmField internal var `J$0`: Long = 0L\n",
        "   @JvmField internal var `D$0`: Double = 0.0\n",
        "   @JvmField internal var `F$0`: Float = 0.0F\n",
        "   @JvmField internal var `I$0`: Int = 0\n",
    ] {
        assert!(fixed.contains(line), "{fixed}");
    }
    assert!(!fixed.contains(';'), "{fixed}");
    assert!(without.contains("   open int label;\n"), "{without}");
    assert!(without.contains("   open double D$0;\n"), "{without}");
    assert!(
        without.contains(") : super(`$completion`) {"),
        "the constructor fix still applies: {without}"
    );
}

#[test]
fn constructors_without_metadata_delegate() {
    let (fixed, without) = fixed_and_without(CELL, KunaFix::KtConstructorWithoutMetadata);
    assert!(
        fixed.contains(
            "   public constructor(`$name`: java.lang.String, var2: Int) : super(`$name`, 4) {\n      this.`$n$inlined` = var2\n   }\n"
        ),
        "{fixed}"
    );
    assert!(
        !fixed.contains("fun `TestMembersWithoutMetadataKt"),
        "{fixed}"
    );
    assert!(
        without.contains(
            "   fun `TestMembersWithoutMetadataKt$named$$inlined$cell$1`(`$name`: java.lang.String, var2: Int) {\n      super(`$name`, 4)\n"
        ),
        "{without}"
    );

    let (fixed, without) = fixed_and_without(CONTINUATION, KunaFix::KtConstructorWithoutMetadata);
    assert!(
        fixed.contains(") : super(`$completion`) {\n      this.`this$0` = `this$0`\n   }\n"),
        "{fixed}"
    );
    assert!(
        without.contains("\n      super(`$completion`)\n"),
        "{without}"
    );
}

#[test]
fn an_object_initializer_is_an_init_block() {
    let (fixed, without) =
        fixed_and_without("pkg/NoMetaBag", KunaFix::KtConstructorWithoutMetadata);
    assert!(
        fixed.contains("         init {\n            this.first = "),
        "{fixed}"
    );
    assert!(
        without.contains("\n         {\n            this.first = "),
        "{without}"
    );
}

#[test]
fn delegation_call_arguments_are_in_the_call() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestDelegationArguments",
        KunaFix::KtDelegationCallArguments,
    );
    assert!(
        fixed.contains(
            "class TestDelegationArguments(text: String, size: Int) : DelegationBase(DelegationJava.label(text), size) {\n"
        ),
        "{fixed}"
    );
    assert!(
        fixed.contains("   public constructor(raw: String) : this(DelegationJava.label(raw), 0)\n"),
        "{fixed}"
    );
    assert!(!fixed.contains("var10001"), "{fixed}");
    assert!(
        without.contains("   public constructor(raw: String)  {\n      val var10001: java.lang.String = DelegationJava.label(raw)\n      this(var10001, 0)\n"),
        "{without}"
    );
    assert!(
        without.contains("      super(var10001, size)\n"),
        "{without}"
    );

    let (fixed, without) = fixed_and_without(
        "pkg/TestDelegationOrder",
        KunaFix::KtDelegationCallArguments,
    );
    assert!(
        fixed.contains(
            ": this(DelegationJava.label(first), DelegationJava.label(raw), size.toInt())\n"
        ),
        "{fixed}"
    );
    assert!(
        fixed.contains(": this(raw, DelegationJava.label(raw), n + 1)\n"),
        "{fixed}"
    );
    assert!(
        without.contains("      this(raw, var10002, n + 1)\n"),
        "{without}"
    );
}
