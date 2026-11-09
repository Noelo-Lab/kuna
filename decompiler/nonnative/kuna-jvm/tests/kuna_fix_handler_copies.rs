//! The handler-copy fixes (`handler_copy_offsets`, `nested_handler_copies`,
//! `copied_finally`) on their kuna-fixes cases, in both modes.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack};

/// The text the `<set>/<id>` kuna-fixes case prints.
fn decompile(set: &str, id: &str) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let id = format!("{set}/{id}");
    let case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("no case {id}"));
    with_engine_stack(move || run_case(&case))
        .actual
        .expect("output")
}

/// The `catch (Throwable ...)` blocks of `text` that end without rethrowing.
fn swallowing_catches(text: &str) -> usize {
    let lines: Vec<&str> = text.lines().collect();
    let mut count = 0;
    for (i, line) in lines.iter().enumerate() {
        let body = line.trim_start();
        if !body.starts_with("} catch (Throwable ") {
            continue;
        }
        let close = format!("{}}}", &line[..line.len() - body.len()]);
        let ends = |l: &&&str| {
            l.strip_prefix(close.as_str())
                .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))
        };
        let rethrows = lines[i + 1..]
            .iter()
            .take_while(|l| !ends(l))
            .any(|l| l.trim_start().starts_with("throw "));
        if !rethrows {
            count += 1;
        }
    }
    count
}

#[test]
fn a_finally_copied_by_irreducible_splitting_rethrows() {
    let fixed = decompile("kuna-fixes", "pkg/TestSuspendLoopFinally");
    assert_eq!(count_failed_methods(&fixed), 0);
    assert!(fixed.contains("Irreducible bytecode was duplicated"));
    assert!(fixed.contains("} catch (Throwable "));
    assert_eq!(swallowing_catches(&fixed), 0, "{fixed}");

    let compat = decompile("kuna-fixes-compat", "pkg/TestSuspendLoopFinally");
    assert_eq!(count_failed_methods(&compat), 2);
}

#[test]
fn swallowing_catches_finds_a_catch_without_rethrow() {
    let text = "      try {\n         f();\n      } catch (Throwable e) {\n         g();\n      }\n      try {\n         f();\n      } catch (Throwable e) {\n         g();\n         throw e;\n      } finally {\n         h();\n      }\n";
    assert_eq!(swallowing_catches(text), 1);
}
