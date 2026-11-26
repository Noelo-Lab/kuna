//! Final-text regressions for port bugs found by a pass-trace sweep:
//! vendored corpus classes run with the Default set's options plus a few overrides the corpus does
//! not use, compared with the digest of the text pinned for that run (`data/trace_sweep/digests.tsv`).

use std::path::{Path, PathBuf};

use kuna_jvm::kuna_corpus::*;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm")
}

fn regress(classes: &[&str], overrides: &[(&str, &str)], compare: &str, expected: &str) {
    let digests = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/trace_sweep/digests.tsv"),
    )
    .unwrap();
    let digest = digests
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{expected}\t")))
        .unwrap_or_else(|| panic!("no digest for {expected}"));
    let cases = load_cases(&root().join("cases.tsv")).expect("tests/jvm/cases.tsv loads");
    let mut case = cases
        .iter()
        .find(|c| c.id == "default/pkg/TestEnhancedForLoops")
        .expect("a Default-set case")
        .clone();
    case.id = format!("default/{compare}");
    case.class_files = classes
        .iter()
        .map(|c| root().join("classes").join(c))
        .collect();
    for (k, v) in overrides {
        overlay_option(&mut case.options, k, v);
    }
    case.compare = vec![compare.to_string()];
    case.digest = Some(digest.to_string());
    let r = with_engine_stack(move || run_case(&case));
    assert_eq!(
        r.status,
        CaseStatus::Match,
        "{expected}: first difference {:?}, error {:?}",
        r.first_diff,
        r.error
    );
}

/// A `MethodParameters` entry without a name is a `null` name in `VarProcessor.mapVarNames`;
/// `setDebugVarNames` then takes the LVT name (`valueOf(String name)`), and later nameless entries
/// become `nullx`, ... (`TestEnum2(String param1, int nullx)`).
#[test]
fn nameless_method_parameters_keep_their_slot() {
    regress(
        &["java21/ext/TestEnum2.class"],
        &[("decompile-enums", "0")],
        "TestEnum2.java",
        "TestEnum2-denum0",
    );
}
