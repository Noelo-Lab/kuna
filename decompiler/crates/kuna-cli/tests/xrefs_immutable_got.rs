//! Exact relative GOT targets join the public call graph; mutable words do not.

use crate::common;
use object::{Object, ObjectSymbol};
use serde_json::Value;

fn symbols() -> std::collections::BTreeMap<String, u64> {
    let bytes = std::fs::read(common::fixture("immutable_got_calls_x86_64")).unwrap();
    object::File::parse(bytes.as_slice())
        .unwrap()
        .symbols()
        .filter_map(|symbol| Some((symbol.name().ok()?.to_owned(), symbol.address())))
        .collect()
}

fn query(command: &str, args: &[&str]) -> Value {
    let image = common::fixture("immutable_got_calls_x86_64");
    let mut argv = vec![command, &image];
    argv.extend(args);
    let (stdout, stderr, status) = common::run_kuna(&argv);
    assert_eq!(status, 0, "{stderr}\n{stdout}");
    serde_json::from_str(&stdout).unwrap()
}

#[test]
fn immutable_memory_call_reports_the_callee_and_its_separate_slot_read() {
    let names = symbols();
    let from = query("xrefs", &["--from", "through_got", "--json"]);
    let rows = from["xrefs"].as_array().unwrap();
    let calls: Vec<_> = rows.iter().filter(|r| r["kind"] == "call").collect();
    assert_eq!(calls.len(), 1, "{from}");
    assert_eq!(calls[0]["to_address"], names["callback"]);
    assert_eq!(calls[0]["to_function"]["address"], names["callback"]);
    assert!(
        rows.iter()
            .any(|r| r["kind"] == "read" && r["to_address"] == names["callback_slot"]),
        "{from}"
    );
    let to = query("xrefs", &["--to", "callback", "--json"]);
    assert!(
        to["xrefs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["kind"] == "call" && r["from_function"]["address"] == names["through_got"]),
        "{to}"
    );
    let reachable = query("functions", &["--reachable-from", "through_got", "--json"]);
    assert!(
        reachable["functions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["address"] == names["callback"]),
        "{reachable}"
    );
    let graph = query("decompile-graph", &["--functions", ""]);
    let edges: Vec<_> = graph["edges"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|edge| edge["callerAddress"] == names["through_got"])
        .collect();
    assert_eq!(edges.len(), 1, "{graph}");
    assert_eq!(edges[0]["calleeAddress"], names["callback"]);
    assert_eq!(edges[0]["kind"], "call");
    let caller = graph["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["address"] == names["through_got"])
        .unwrap();
    assert_eq!(
        caller["hasIndirectCalls"], true,
        "physical CALLIND remains recorded"
    );
}

#[test]
fn mutable_and_register_carried_calls_keep_their_existing_slot_edges() {
    let names = symbols();
    for (caller, slot) in [
        ("through_mutable", "mutable_slot"),
        ("through_carried", "callback_slot"),
    ] {
        let doc = query("xrefs", &["--from", caller, "--json"]);
        let calls: Vec<_> = doc["xrefs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["kind"] == "call")
            .collect();
        assert_eq!(calls.len(), 1, "{doc}");
        assert_eq!(calls[0]["to_address"], names[slot]);
        let reachable = query("functions", &["--reachable-from", caller, "--json"]);
        assert!(
            !reachable["functions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|f| f["address"] == names["callback"]),
            "{caller}: {reachable}"
        );
    }
}

#[test]
fn an_explicit_callee_overrides_the_immutable_slot_and_the_last_statement_wins() {
    let site = symbols()["through_got"] + 4;
    let first = format!("callee through_got::{site:#x} callback");
    let last = format!("callee through_got::{site:#x} alternate");
    for restated in [false, true] {
        let mut args = vec!["through_got", "--json"];
        if restated {
            args.extend(["--assert", first.as_str()]);
        }
        args.extend(["--assert", last.as_str()]);
        let doc = query("decompile", &args);
        let code = doc["functions"][0]["code"].as_str().unwrap();
        assert!(code.contains("alternate("), "{doc}");
        assert!(!code.contains("callback("), "{doc}");
        let outcomes = doc["assertions"].as_array().unwrap();
        assert_eq!(outcomes.last().unwrap()["status"], "applied", "{doc}");
        if restated {
            assert!(
                doc.to_string()
                    .contains("a later callee states this call site again"),
                "{doc}"
            );
        }
    }
}
