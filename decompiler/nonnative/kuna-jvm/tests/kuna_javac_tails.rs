//! The kuna option `jvm_javac_layout` for code after loops, try-catch and
//! `synchronized` blocks, `while` loops, `||` jumps and early exits in
//! protected blocks: the `TestJavacTails` two-pass case laid out the way
//! javac placed it, by default only.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_options::JAVAC_LAYOUT;

fn text_with(options: &[(&str, &str)]) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == "kuna-fixes/pkg/TestJavacTails")
        .expect("no case kuna-fixes/pkg/TestJavacTails");
    case.options
        .extend(options.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const JAVAC: [&str; 9] = [
    "         if (sub == 0) {\n            break;\n         }\n\n         out.append(sub);\n      }\n\n      d.close(el);\n",
    "         if (attribId == 0) {\n            break;\n         }\n",
    "            if (pos == chars.length) {\n               break label21;\n            }\n",
    "      TestJavacTails.Node node = this.first;\n\n      while (node != null) {\n",
    "         throw new IllegalStateException(\"not a number: \" + text, e);\n      }\n\n      return builder;\n",
    "         }\n      }\n\n      return taken;\n",
    "         if (w.isEmpty() || w.startsWith(\"#\")) {\n            continue;\n",
    "         if (this.name.equals(newName)) {\n            return;\n         }\n",
    "         if (client == null) {\n            return;\n         }\n",
];

#[test]
fn tails_follow_javac() {
    let text = text_with(&[]);
    for shape in JAVAC {
        assert!(text.contains(shape), "{shape}\n---\n{text}");
    }
}

#[test]
fn off_and_compat_keep_vineflowers_tails() {
    for options in [
        &[(JAVAC_LAYOUT, "0")][..],
        &[("jvm_vineflower_compat", "1"), (JAVAC_LAYOUT, "1")][..],
    ] {
        let text = text_with(options);
        for shape in JAVAC {
            assert!(!text.contains(shape), "{options:?}: {shape}\n---\n{text}");
        }
        for shape in [
            "            d.close(el);\n            return;\n",
            "      for (TestJavacTails.Node node = this.first; node != null; node = node.next) {\n",
        ] {
            assert!(text.contains(shape), "{options:?}: {shape}\n---\n{text}");
        }
    }
}
