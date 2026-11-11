//! The fix of ktenum on its kuna-fixes case, in Kotlin and in Java output, with only its own fix
//! turned off (`KunaFixes::without`).

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
