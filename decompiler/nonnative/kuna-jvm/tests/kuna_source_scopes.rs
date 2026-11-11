//! The kuna option `jvm_source_locals`, its declaration scopes and increments:
//! the `TestDeclarationScopes` and `TestIncrementStatements` two-pass cases
//! keep the source's declarations and increment statements by default only.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_options::SOURCE_LOCALS;

fn text_with(id: &str, options: &[(&str, &str)]) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("no case {id}"));
    case.options
        .extend(options.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const SCOPES: [&str; 6] = [
    "      int tmplength;\n      int mask1;\n      int mask2;\n      int value1;\n      int value2;\n      int sbit = 0;\n",
    "         mask2 = b[sbit];\n",
    "      TestDeclarationScopes.Digest digest;\n      try {\n         digest = source.open(base);\n",
    "      String r;\n      try {\n",
    "      for (node = node.parent(); node != null && !parents.contains(node); node = node.parent()) {\n",
    "         for (sbit = 0; sbit < limit - size; sbit++) {\n",
];

const INCREMENTS: [&str; 4] = [
    "      for (int i = 0; i < n; i++) {\n         keys[i] = i;\n",
    "      this.toRemove = this.index;\n      this.index++;\n",
    "      m--;\n      int common = (int)(m & bits);\n",
    "      pos++;\n      if (pos > this.verified) {\n",
];

const FOLDED: [&str; 3] = ["values[i++]", "int y = ++i;", "base + \"_\" + ++counter"];

#[test]
fn declarations_follow_the_slots() {
    let text = text_with("kuna-fixes/pkg/TestDeclarationScopes", &[]);
    for shape in SCOPES {
        assert!(text.contains(shape), "{shape}\n---\n{text}");
    }
    assert!(text.contains("         String name = (String)names.get(i);\n         Supplier<Integer> len"), "{text}");
}

#[test]
fn increments_stay_statements() {
    let text = text_with("kuna-fixes/pkg/TestIncrementStatements", &[]);
    for shape in INCREMENTS.iter().chain(&FOLDED) {
        assert!(text.contains(shape), "{shape}\n---\n{text}");
    }
}

#[test]
fn off_and_compat_keep_vineflowers_scopes() {
    for options in [
        &[(SOURCE_LOCALS, "0")][..],
        &[("jvm_vineflower_compat", "1"), (SOURCE_LOCALS, "1")][..],
    ] {
        let scopes = text_with("kuna-fixes/pkg/TestDeclarationScopes", options);
        for shape in SCOPES {
            assert!(!scopes.contains(shape), "{options:?}: {shape}\n---\n{scopes}");
        }
        let increments = text_with("kuna-fixes/pkg/TestIncrementStatements", options);
        for shape in INCREMENTS {
            assert!(!increments.contains(shape), "{options:?}: {shape}\n---\n{increments}");
        }
        for shape in FOLDED {
            assert!(increments.contains(shape), "{options:?}: {shape}\n---\n{increments}");
        }
    }
}
