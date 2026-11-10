//! The kuna option `jvm_source_locals`: its row, the compat switch forcing it
//! off, and its two-pass case with the option alone turned off.

use std::path::Path;

use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::Fernflower;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::kuna_corpus::{load_cases, reset_engine_thread, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::VINEFLOWER_COMPAT;
use kuna_jvm::kuna_options::{find, form_enabled, resolve, KUNA_FORM, SOURCE_LOCALS};

/// Whether the context `Fernflower::new` installs for `options` has the option on.
fn engine_has_option(options: &[(&str, &str)]) -> bool {
    let options: Vec<(String, String)> = options
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    with_engine_stack(move || {
        reset_engine_thread();
        let mut fernflower = Fernflower::new(&options, <dyn IFernflowerLogger>::NO_OP());
        let on = form_enabled(SOURCE_LOCALS);
        let leaked = DecompilerContext::get_property(SOURCE_LOCALS).is_some();
        assert!(!leaked, "the option stays out of Vineflower's option map");
        fernflower.clear_context();
        reset_engine_thread();
        on
    })
}

#[test]
fn the_row_is_a_kuna_form_option_on_by_default() {
    let row = find(SOURCE_LOCALS).expect("jvm_source_locals row");
    assert!(row.is_kuna());
    assert_eq!((row.values, row.default), ("on|off", "on"));
    assert_eq!(row.change_kind, KUNA_FORM);
    assert_eq!(
        resolve(SOURCE_LOCALS, "off").unwrap(),
        (SOURCE_LOCALS, "0".to_string())
    );
}

#[test]
fn compat_turns_the_option_off() {
    assert!(engine_has_option(&[]));
    assert!(engine_has_option(&[(SOURCE_LOCALS, "1")]));
    assert!(!engine_has_option(&[(SOURCE_LOCALS, "0")]));
    assert!(!engine_has_option(&[(VINEFLOWER_COMPAT, "1")]));
    assert!(!engine_has_option(&[
        (VINEFLOWER_COMPAT, "1"),
        (SOURCE_LOCALS, "1")
    ]));
}

fn text_with(id: &str, options: &[(&str, &str)]) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"));
    case.options
        .extend(options.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn named_locals_stay_and_declarations_follow_the_slots() {
    let kept = text_with("pkg/TestSourceLocals", &[]);
    for line in [
        "result = 31 * result + this.names.length;\n      return result;",
        "List<T> typed = (List<T>)list;\n      return typed;",
        "String res = c ? a : b;\n      return res;",
        "arr[0] = a;",
        "for (String s : part) {",
        "while (it.hasNext()) {",
        "int nRead;\n      long total = 0L;",
        "return this.names.length;",
        "return this.id;",
    ] {
        assert!(kept.contains(line), "{line}\n{kept}");
    }

    let folded = text_with("pkg/TestSourceLocals", &[(SOURCE_LOCALS, "0")]);
    for line in [
        "return 31 * result + this.names.length;",
        "return (List<T>)list;",
        "return c ? a : b;",
        "return new String[]{a, b};",
        "for (String s : src.subList(1, 3)) {",
        "long total = 0L;\n\n      int nRead;",
    ] {
        assert!(folded.contains(line), "{line}\n{folded}");
    }
}
