//! `kuna xrefs --to <name>` on a shared library that calls its own export through
//! `name@plt` (issue #1031): the name answers for the export, so the calls that
//! reach it through the stub and its GOT slot are its callers.

use crate::common;
use serde_json::Value;

const BODY: u64 = 0x1120;
const STUB: u64 = 0x1050;
const SLOT: u64 = 0x4018;

fn xrefs_to(spec: &str) -> (Value, String) {
    let lib = common::fixture("selfplt_x86_64.so");
    let (out, err, code) = common::run_kuna(&["xrefs", &lib, "--to", spec, "--json"]);
    assert_eq!(code, 0, "{err}");
    (serde_json::from_str(&out).expect("xrefs JSON"), err)
}

fn aliases(doc: &Value) -> Vec<u64> {
    doc["target"]["aliases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["address"].as_u64().unwrap())
        .collect()
}

#[test]
fn a_name_counts_the_calls_through_the_exports_own_plt_stub() {
    let (doc, err) = xrefs_to("bump");
    assert!(!err.contains("warning"), "{err}");
    assert_eq!(doc["target"]["address"].as_u64(), Some(BODY));
    assert_eq!(aliases(&doc), vec![STUB, SLOT]);
    let sites: Vec<(u64, &str, u64)> = doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["from_address"].as_u64().unwrap(),
                r["kind"].as_str().unwrap(),
                r["to_address"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        sites,
        vec![
            (0x1137, "call", STUB),
            (0x1140, "jump", STUB),
            (0x1158, "call", STUB)
        ]
    );
    assert_eq!(doc["count"].as_u64(), Some(3));
}

#[test]
fn an_address_still_answers_for_that_address() {
    let (body, err) = xrefs_to("0x1120");
    assert!(err.is_empty(), "{err}");
    assert_eq!((body["count"].as_u64(), aliases(&body)), (Some(0), vec![]));
    let (stub, err) = xrefs_to("0x1050");
    assert!(err.is_empty(), "{err}");
    assert_eq!(
        (stub["count"].as_u64(), aliases(&stub)),
        (Some(3), vec![SLOT])
    );
}
