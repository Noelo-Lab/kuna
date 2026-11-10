//! The fix `overlapping_ranges` on its kuna-fixes case: alone off
//! (`KunaFixes::without`), all five functions fail as in compat mode; on, the
//! `return` path keeps its `catch`, a `finally` copy another path runs stays,
//! and a `try` re-entered at its head stays a catch-all.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == "kuna-fixes/pkg/TestOverlappingRanges")
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

/// The lines of function `name` in `text`, its closing brace excluded.
fn function<'a>(text: &'a str, name: &str) -> Vec<&'a str> {
    let header = format!("   public fun {name}(");
    text.lines()
        .skip_while(|l| !l.starts_with(&header))
        .skip(1)
        .take_while(|l| *l != "   }")
        .collect()
}

/// Whether `lines` hold `try { <call..>; return ..} catch (_: IoFailure) {`.
fn returns_inside_catch(lines: &[&str], call: &str) -> bool {
    lines.windows(4).any(|w| {
        w[0].trim() == "try {"
            && w[1].trim().starts_with(call)
            && w[2].trim().starts_with("return ")
            && w[3].trim().starts_with("} catch (")
            && w[3].ends_with(": TestOverlappingRanges.IoFailure) {")
    })
}

#[test]
fn the_return_path_keeps_its_catch() {
    let fixed = text_with(KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(
        returns_inside_catch(&function(&fixed, "civilized"), "CloseableKt.closeFinally("),
        "{fixed}"
    );
    assert!(
        returns_inside_catch(
            &function(&fixed, "logged"),
            "this.store.close(handle, cause)"
        ),
        "{fixed}"
    );

    let broken = text_with(KunaFixes::DEFAULT.without(KunaFix::OverlappingRanges));
    assert_eq!(count_failed_methods(&broken), 5, "{broken}");
}

#[test]
fn a_shared_finally_copy_stays() {
    let fixed = text_with(KunaFixes::DEFAULT);
    let body = function(&fixed, "sharedTail");
    assert!(
        returns_inside_catch(&body, "this.store.close(handle, cause)"),
        "{fixed}"
    );
    let semaphores = body
        .iter()
        .filter(|l| l.contains("/* VF: Semaphore variable */"))
        .count();
    assert_eq!(semaphores, 2, "{fixed}");
    assert_eq!(
        body[body.len() - 3..]
            .iter()
            .map(|l| l.trim())
            .collect::<Vec<_>>(),
        [
            "this.store.close(handle, null)",
            "this.store.delete(name)",
            "return false"
        ],
        "{fixed}"
    );
}

#[test]
fn a_try_reentered_at_its_head_stays_a_catch_all() {
    let fixed = text_with(KunaFixes::DEFAULT);
    let body = function(&fixed, "countedRead");
    let catch_all = body
        .iter()
        .position(|l| l.trim().starts_with("} catch (") && l.ends_with(": Throwable) {"))
        .expect("a catch-all");
    assert!(
        body[..catch_all].iter().any(|l| l.trim() == "attempts++"),
        "{fixed}"
    );
    assert!(
        body[..catch_all].iter().all(|l| l.trim() != "} finally {"),
        "{fixed}"
    );
}
