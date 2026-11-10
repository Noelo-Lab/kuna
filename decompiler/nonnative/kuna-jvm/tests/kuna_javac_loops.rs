//! The kuna option `jvm_javac_layout` in loop bodies: the ifs, exits and loops
//! of the `TestJavacLoops` two-pass case laid out the way javac placed them,
//! by default only.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_options::JAVAC_LAYOUT;

fn text_with(options: &[(&str, &str)]) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == "kuna-fixes/pkg/TestJavacLoops")
        .expect("no case kuna-fixes/pkg/TestJavacLoops");
    case.options
        .extend(options.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const JAVAC: [&str; 7] = [
    "         if (row != null) {\n            this.total += row.length;\n         } else {\n            return -1;\n",
    "         } else if (v > 0) {\n            this.skipped += v;\n         } else {\n",
    "            this.skipped++;\n            continue;\n         }\n\n         this.total += v;\n",
    "         if (row == null) {\n            continue;\n         }\n\n         if (row.length == 0) {\n",
    "      for (i = 0; i < n; i++) {\n         if (input[i] == key) {\n            break;\n",
    "         if (!source.more()) {\n            return current;\n         }\n",
    "         throw new IllegalStateException(\"zero\", e);\n      }\n\n      return n;\n",
];

#[test]
fn loop_bodies_follow_javac() {
    let text = text_with(&[]);
    for shape in JAVAC {
        assert!(text.contains(shape), "{shape}\n---\n{text}");
    }
}

#[test]
fn off_and_compat_keep_vineflowers_loops() {
    for options in [
        &[(JAVAC_LAYOUT, "0")][..],
        &[("jvm_vineflower_compat", "1"), (JAVAC_LAYOUT, "1")][..],
    ] {
        let text = text_with(options);
        for shape in JAVAC {
            assert!(!text.contains(shape), "{options:?}: {shape}\n---\n{text}");
        }
        for shape in [
            "         if (row != null && row.length != 0 && row[0] == key) {\n",
            "      while (i < n && input[i] != key) {\n",
            "      } while (source.more());\n",
        ] {
            assert!(text.contains(shape), "{options:?}: {shape}\n---\n{text}");
        }
    }
}
