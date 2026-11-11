//! The fixes `kt_property_field_flags`, `kt_jvm_field_setter`,
//! `kt_inline_heap_read` and `field_read_reuse` (its `x?.f` case, Java
//! output) on their kuna-fixes cases, each alone off (`KunaFixes::without`)
//! and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes, java: bool) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    if java {
        let name = id.rsplit('/').next().unwrap();
        case.options.retain(|(k, _)| k != "jvm_plugins");
        case.options.push(("jvm_plugins".into(), "0".into()));
        case.compare = vec![format!("{name}.java")];
    }
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const FIELD_FLAGS: &str = "pkg/TestKtPropertyFieldFlags";
const JVM_FIELD: &str = "pkg/TestKtJvmFieldSetter";
const HEAP_READ: &str = "pkg/TestKtInlineHeapRead";
const SAFE_CALL: &str = "pkg/TestKtSafeCallFieldRead";

#[test]
fn a_property_carries_its_field_flags() {
    let fixed = text_with(FIELD_FLAGS, KunaFixes::DEFAULT, false);
    assert_eq!(fixed.matches("@Volatile\n").count(), 5, "{fixed}");
    assert_eq!(fixed.matches("@Transient\n").count(), 2, "{fixed}");
    assert!(fixed.contains("@delegate:Transient\n"), "{fixed}");
    assert!(
        fixed.contains("@JvmSynthetic\n   public final var synthetic"),
        "{fixed}"
    );
    let broken = text_with(
        FIELD_FLAGS,
        KunaFixes::DEFAULT.without(KunaFix::KtPropertyFieldFlags),
        false,
    );
    for annotation in [
        "@Volatile",
        "@Transient",
        "@delegate:Transient",
        "@JvmSynthetic",
    ] {
        assert!(!broken.contains(annotation), "{broken}");
    }
}

#[test]
fn a_jvm_field_var_has_no_setter() {
    let fixed = text_with(JVM_FIELD, KunaFixes::DEFAULT, false);
    assert_eq!(fixed.matches("private set").count(), 4, "{fixed}");
    assert!(
        fixed.contains("@JvmField\n   public final var count: Int = 0\n\n"),
        "{fixed}"
    );
    let broken = text_with(
        JVM_FIELD,
        KunaFixes::DEFAULT.without(KunaFix::KtJvmFieldSetter),
        false,
    );
    assert_eq!(broken.matches("private set").count(), 8, "{broken}");
}

#[test]
fn a_heap_read_with_several_uses_stays_in_its_local() {
    let fixed = text_with(HEAP_READ, KunaFixes::DEFAULT, false);
    for kept in [
        "val e: TestKtInlineHeapRead.Node = this.slots[i]\n",
        "val s: TestKtInlineHeapRead.Node = cache[0]\n",
        "val n: TestKtInlineHeapRead.Node = this.head.next\n",
        "val r: Int = this.limit - this.count\n",
    ] {
        assert!(fixed.contains(kept), "{fixed}");
    }
    assert!(
        fixed.contains("return this.items[0] as? TestKtInlineHeapRead.Node"),
        "{fixed}"
    );
    assert!(
        fixed.contains("if (this.fixed.next != null) this.fixed.next.value else 0"),
        "{fixed}"
    );
    assert!(fixed.contains("return this.slots[i]\n"), "{fixed}");
    let broken = text_with(
        HEAP_READ,
        KunaFixes::DEFAULT.without(KunaFix::KtInlineHeapRead),
        false,
    );
    assert!(
        broken.contains("if (this.slots[i] != null) this.slots[i].value else 0"),
        "{broken}"
    );
    assert!(
        broken.contains("if (this.head.next != null) this.head.next.value else -1"),
        "{broken}"
    );
    assert!(
        broken.contains("if (this.limit - this.count > 0) this.limit - this.count else 0"),
        "{broken}"
    );
}

#[test]
fn a_safe_call_reads_its_field_once_in_java_output() {
    let fixed = text_with(SAFE_CALL, KunaFixes::DEFAULT, true);
    assert!(!fixed.contains("this.stream != null"), "{fixed}");
    assert!(!fixed.contains("this.stream == null"), "{fixed}");
    assert!(!fixed.contains("this.plain != null"), "{fixed}");
    assert!(fixed.contains("this.fixed != null"), "{fixed}");
    let broken = text_with(
        SAFE_CALL,
        KunaFixes::DEFAULT.without(KunaFix::FieldReadReuse),
        true,
    );
    assert!(broken.contains("this.stream != null"), "{broken}");
    assert!(broken.contains("this.stream == null"), "{broken}");
    assert!(broken.contains("this.plain != null"), "{broken}");
}
