//! The kuna option `jvm_javac_layout`: its switch, and the ifs and exits of its
//! two-pass case laid out the way javac placed them, by default only.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_options::{find, resolve, take_form_options, JAVAC_LAYOUT, KUNA_FORM};

fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn text_with(options: &[(&str, &str)]) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == "kuna-fixes/pkg/TestJavacLayout")
        .expect("no case kuna-fixes/pkg/TestJavacLayout");
    case.options.extend(pairs(options));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn switch_is_taken_off_the_options() {
    let rest = pairs(&[("a", "1")]);
    let (forms, left) = take_form_options(&rest, false);
    assert!(forms.contains(JAVAC_LAYOUT));
    assert_eq!(left, rest);
    let (forms, left) = take_form_options(&pairs(&[(JAVAC_LAYOUT, "off"), ("a", "1")]), false);
    assert!(!forms.contains(JAVAC_LAYOUT));
    assert_eq!(left, rest);
    let (forms, _) = take_form_options(&pairs(&[(JAVAC_LAYOUT, "0"), (JAVAC_LAYOUT, "on")]), false);
    assert!(forms.contains(JAVAC_LAYOUT));
    let (forms, _) = take_form_options(&pairs(&[(JAVAC_LAYOUT, "on")]), true);
    assert!(!forms.contains(JAVAC_LAYOUT));
}

#[test]
fn row_is_kunas_own() {
    let row = find(JAVAC_LAYOUT).expect("the jvm_javac_layout row");
    assert!(row.is_kuna());
    assert_eq!(row.change_kind, KUNA_FORM);
    assert_eq!(row.default, "on");
    assert_eq!(
        resolve(JAVAC_LAYOUT, "off").unwrap(),
        (JAVAC_LAYOUT, "0".to_string())
    );
}

#[test]
fn ifs_and_exits_follow_javac() {
    let text = text_with(&[]);
    for shape in [
        "      if (x == null) {\n         return 0;\n      }\n\n      return x.hashCode();\n",
        "      return v < 0 ? -1 : 1;\n",
        "      if (v > 0) {\n         return true;\n      }\n\n      return false;\n",
        "      if (o == null || this.getClass() != o.getClass()) {\n         return false;\n      }\n",
        "      if (index < 0 || index >= this.table.length) {\n",
        "      if (values == null) {\n         return;\n      }\n",
        "         if (v < 0) {\n            continue;\n         }\n",
        "      if (n != 0) {\n         this.total = n;\n      } else {\n",
    ] {
        assert!(text.contains(shape), "{shape}\n---\n{text}");
    }
}

#[test]
fn off_and_compat_keep_vineflowers_structure() {
    for options in [
        &[(JAVAC_LAYOUT, "0")][..],
        &[("jvm_vineflower_compat", "1"), (JAVAC_LAYOUT, "1")][..],
    ] {
        let text = text_with(options);
        for shape in [
            "      return x == null ? 0 : x.hashCode();\n",
            "      return v > 0;\n",
            "      } else if (o != null && this.getClass() == o.getClass()) {\n",
            "      if (values != null) {\n",
            "         if (v >= 0) {\n",
        ] {
            assert!(text.contains(shape), "{options:?}: {shape}\n---\n{text}");
        }
    }
}
