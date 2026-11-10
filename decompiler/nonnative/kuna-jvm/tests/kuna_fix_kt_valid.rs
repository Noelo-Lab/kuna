//! The Kotlin-validity fixes of ktvalid on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`), and the pieces that are testable alone.

use std::path::Path;

use kuna_jvm::kotlin::kuna_fix_kt_mapped_java_member::{self, Form};
use kuna_jvm::kotlin::kuna_fix_kt_string_literal_escape::kotlin_text;
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

/// The case's text by default and with `fix` alone off; each `fixed` line is in the first, each `without` line
/// in the second.
fn check(id: &str, fix: KunaFix, fixed: &[&str], without: &[&str]) {
    let on = text_with(id, KunaFixes::DEFAULT);
    let off = text_with(id, KunaFixes::DEFAULT.without(fix));
    for line in fixed {
        assert!(on.contains(line), "{line}\n{on}");
    }
    for line in without {
        assert!(off.contains(line), "{line}\n{off}");
    }
}

#[test]
fn accessor_writes_its_backing_field() {
    check(
        "pkg/TestAccessorBackingField",
        KunaFix::KtAccessorBackingField,
        &[
            "return \"${field}!\"",
            "return field * 2",
            "field = \"$value.\"",
        ],
        &[
            "return this.doubled * 2",
            "TestAccessorBackingField.shared = \"$value.\"",
        ],
    );
}

#[test]
fn string_literals_are_escaped_for_kotlin() {
    check(
        "pkg/TestStringLiteralEscape",
        KunaFix::KtStringLiteralEscape,
        &[
            "return \"\\$this\\$receiver\"",
            "return \"${key}258EAF-E914\"",
            "return \"page\\u000cbreak\"",
        ],
        &["return \"$this$receiver\"", "return \"$key258EAF-E914\""],
    );
}

#[test]
fn abstract_functions_have_no_body() {
    check(
        "pkg/TestAbstractFunctionBody",
        KunaFix::KtAbstractFunctionBody,
        &[
            "public abstract fun dispose()\n",
            "public external fun nativeHash(seed: Long): Int\n",
        ],
        &["public abstract fun dispose() {"],
    );
}

#[test]
fn accessors_and_top_level_properties_have_no_modality() {
    check(
        "pkg/TestMisplacedModality",
        KunaFix::KtMisplacedModality,
        &["      public get() {"],
        &["      public final get() {"],
    );
    check(
        "pkg/TestTopLevelModality",
        KunaFix::KtMisplacedModality,
        &[
            "public val GREETING: String = \"hello\"",
            "\n   public get() {",
        ],
        &[
            "public final val GREETING: String = \"hello\"",
            "public final get() {",
        ],
    );
}

#[test]
fn companion_properties_keep_their_initializers() {
    check(
        "pkg/TestCompanionPropertyInitializer",
        KunaFix::KtCompanionPropertyInitializer,
        &["val FIRST: TestCompanionPropertyInitializer = TestCompanionPropertyInitializer(1)"],
        &["val FIRST: TestCompanionPropertyInitializer\n"],
    );
}

#[test]
fn unstored_properties_start_with_their_default() {
    check(
        "pkg/TestDefaultPropertyInitializer",
        KunaFix::KtDefaultPropertyInitializer,
        &[
            "var count: Int = 0",
            "var closed: Boolean = false",
            "var next: TestDefaultPropertyInitializer? = null",
        ],
        &["var count: Int\n"],
    );
}

#[test]
fn bridged_overrides_are_overrides() {
    check(
        "pkg/TestMissedOverride",
        KunaFix::KtMissedOverride,
        &[
            "public override fun buffer(): pkg.TestMissedOverride.Buffer {",
            "public override val size: Long",
        ],
        &["public open fun buffer(): pkg.TestMissedOverride.Buffer {"],
    );
}

#[test]
fn outer_instances_are_labelled_this() {
    check(
        "pkg/TestOuterThis",
        KunaFix::KtOuterThis,
        &[
            "return this@TestOuterThis",
            "return \"${this@TestOuterThis}/${this.limit}\"",
        ],
        &["return TestOuterThis.this"],
    );
}

#[test]
fn extension_receivers_are_this() {
    check(
        "pkg/TestExtensionReceiver",
        KunaFix::KtExtensionReceiver,
        &["return this * this@TestExtensionReceiver.base + this@TestExtensionReceiver.base"],
        &["`$this$scaled`"],
    );
}

#[test]
fn nested_annotations_have_no_at_sign() {
    check(
        "pkg/TestNestedAnnotationArgument",
        KunaFix::KtNestedAnnotationArgument,
        &["replaceWith = ReplaceWith(expression = \"size\""],
        &["replaceWith = @ReplaceWith(expression = \"size\""],
    );
}

#[test]
fn throws_names_classes_in_kotlin() {
    check(
        "pkg/TestThrowsClassName",
        KunaFix::KtThrowsClassName,
        &["@Throws(TestThrowsClassName.Failure::class, IllegalStateException::class, Throwable::class)"],
        &["@Throws(pkg/TestThrowsClassName$Failure::class"],
    );
}

#[test]
fn thrown_values_drop_the_throwable_cast() {
    check(
        "pkg/TestThrowableCast",
        KunaFix::KtThrowableCast,
        &["throw IllegalArgumentException(\"bad: $n\")\n", "throw t\n"],
        &["throw IllegalArgumentException(\"bad: $n\") as Throwable"],
    );
}

#[test]
fn mapped_members_use_kotlin_names() {
    check(
        "pkg/TestMappedJavaMember",
        KunaFix::KtMappedJavaMember,
        &[
            "return s.length + cs.length + sb.length",
            "arr.size + objs.size",
            "x.javaClass",
        ],
        &["return s.length() + cs.length() + sb.length()"],
    );
}

#[test]
fn mapped_types_use_kotlin_names() {
    check(
        "pkg/TestMappedJavaType",
        KunaFix::KtMappedJavaType,
        &["var out: String = \"\"", "} catch (var3: Throwable) {"],
        &[
            "var out: java.lang.String = \"\"",
            "} catch (var3: java.lang.Throwable) {",
        ],
    );
}

#[test]
fn objects_are_named_not_their_instance_field() {
    check(
        "pkg/TestObjectInstance",
        KunaFix::KtObjectInstance,
        &["val s: IntArray = TestObjectInstance.take()"],
        &["val s: IntArray = TestObjectInstance.INSTANCE.take()"],
    );
}

#[test]
fn super_accessor_calls_are_on_super() {
    check(
        "pkg/TestSuperPropertyAccess",
        KunaFix::KtSuperPropertyAccess,
        &[
            "return super.isActive && this.count < 10",
            "super.count = value * 2",
        ],
        &["return this.isActive && this.count < 10"],
    );
}

#[test]
fn setter_bodies_name_the_header_parameter() {
    check(
        "pkg/TestSetterParameterName",
        KunaFix::KtSetterParameterName,
        &["      public set(value) {\n         field = value\n"],
        &["field = var1"],
    );
}

#[test]
fn java_class_constants_are_kotlin_literals() {
    check(
        "pkg/TestJavaClassLiteral",
        KunaFix::KtJavaClassLiteral,
        &["TestJavaClassLiteral::class.java.getName()"],
        &["TestJavaClassLiteral.class.getName()"],
    );
}

#[test]
fn kotlin_text_escapes_template_starts_only() {
    assert_eq!(kotlin_text("$this$x", false), "\\$this\\$x");
    assert_eq!(kotlin_text("${a}", false), "\\${a}");
    assert_eq!(kotlin_text("a $ b $", false), "a $ b $");
    assert_eq!(kotlin_text("cost $5", false), "cost $5");
    assert_eq!(kotlin_text("page\\fbreak", false), "page\\u000cbreak");
    assert_eq!(kotlin_text("back\\\\fslash", false), "back\\\\fslash");
    assert_eq!(kotlin_text("a $ b", true), "a \\$ b");
}

/// The forms of members of the JDK classes the fix lists (no decompiler context is needed for them).
#[test]
fn mapped_member_forms() {
    let form = kuna_fix_kt_mapped_java_member::form;
    assert_eq!(
        form("java/lang/String", "length", "()I"),
        Some(Form::Property("length"))
    );
    assert_eq!(
        form("java/lang/CharSequence", "charAt", "(I)C"),
        Some(Form::Index)
    );
    assert_eq!(
        form("java/util/ArrayList", "size", "()I"),
        Some(Form::Property("size"))
    );
    assert_eq!(
        form("java/util/HashMap", "entrySet", "()Ljava/util/Set;"),
        Some(Form::Property("entries"))
    );
    assert_eq!(
        form("java/lang/Number", "intValue", "()I"),
        Some(Form::Renamed("toInt"))
    );
    assert_eq!(
        form("java/util/List", "remove", "(I)Ljava/lang/Object;"),
        Some(Form::Renamed("removeAt"))
    );
    assert_eq!(
        form("pkg/Anything", "getClass", "()Ljava/lang/Class;"),
        Some(Form::Property("javaClass"))
    );
}
