//! The fix `handler_copy_shared_body` on its kuna-fixes case: alone off
//! (`KunaFixes::without`), both suspend functions fail as in compat mode. Both
//! run with fix `kt_suspend_state_machine` off, which rebuilds the functions'
//! state machines into their direct bodies.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == "kuna-fixes/pkg/TestSuspendConditionalFinally")
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

/// The text of the `} finally {` blocks of `text`, each up to its closing brace.
fn finally_blocks(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut blocks = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let body = line.trim_start();
        if body != "} finally {" {
            continue;
        }
        let close = format!("{}}}", &line[..line.len() - body.len()]);
        let block: Vec<&str> = lines[i + 1..]
            .iter()
            .take_while(|l| **l != close)
            .copied()
            .collect();
        blocks.push(block.join("\n"));
    }
    blocks
}

#[test]
fn every_handler_copy_runs_the_whole_finally() {
    let machines = KunaFixes::DEFAULT.without(KunaFix::KtSuspendStateMachine);
    let fixed = text_with(machines);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    let blocks = finally_blocks(&fixed);
    let cancels = blocks
        .iter()
        .filter(|b| b.contains("src.cancel(cause)"))
        .count();
    let closes = blocks.iter().filter(|b| b.contains("src.closeIf(")).count();
    assert!(cancels >= 6 && closes >= 4, "{fixed}");
    assert_eq!(cancels + closes, blocks.len(), "{fixed}");
    for block in blocks.iter().filter(|b| b.contains("src.cancel(cause)")) {
        assert!(
            block.contains("if (consume) {") || block.contains("if (`$continuation`.`Z$0`) {"),
            "{block}"
        );
    }

    let broken = text_with(machines.without(KunaFix::HandlerCopySharedBody));
    assert_eq!(count_failed_methods(&broken), 2, "{broken}");
}

#[test]
fn finally_blocks_ends_each_block_at_its_brace() {
    let text = "   try {\n      f()\n   } finally {\n      if (a) {\n         g()\n      }\n   }\n   h()\n";
    assert_eq!(
        finally_blocks(text),
        vec!["      if (a) {\n         g()\n      }".to_string()]
    );
}
