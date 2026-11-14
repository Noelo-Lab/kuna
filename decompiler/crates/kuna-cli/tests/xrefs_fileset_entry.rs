//! `kuna xrefs` and the other surfaces that read the image themselves, over a
//! non-default `MH_FILESET` entry chosen with `--option kernelcache` (GH #1551):
//! they read the entry the engine loads, and an address another entry holds
//! says so instead of answering a silent zero.
//!
//! Fixture: `kernelcache/fileset_calls.macho` (`gen_fileset_calls.py`), whose
//! kext calls `pmgr_wait_pending` from five `bl` sites. Integration tests
//! require the built processor specs under `specs/`.

use crate::common;

const PMGR: &[&str] = &["--option", "kernelcache", "com.example.driver.PMGR"];
const WAIT: u64 = 0xfffffe00_07007840;
const KERNEL_ADD: u64 = 0xfffffe00_07007000;

fn kuna(sub: &str, args: &[&str], entry: &[&str]) -> (String, String) {
    let fixture = common::fixture("kernelcache/fileset_calls.macho");
    let mut argv = vec![sub, fixture.as_str()];
    argv.extend_from_slice(args);
    argv.extend_from_slice(entry);
    let (stdout, stderr, rc) = common::run_kuna(&argv);
    assert_eq!(rc, 0, "{argv:?}\n{stderr}\n{stdout}");
    (stdout, stderr)
}

fn json(sub: &str, args: &[&str], entry: &[&str]) -> serde_json::Value {
    let mut args = args.to_vec();
    args.push("--json");
    serde_json::from_str(&kuna(sub, &args, entry).0).unwrap()
}

fn sites(doc: &serde_json::Value) -> Vec<(u64, String, u64)> {
    doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let from = r["from_function"]["name"].as_str().unwrap_or("-").to_owned();
            (r["from_address"].as_u64().unwrap(), from, r["to_address"].as_u64().unwrap())
        })
        .collect()
}

#[test]
fn the_selected_entry_is_walked_by_address_and_by_name() {
    let set = "pmgr_set_perf_state";
    let callers = [(0x7808, set), (0x7810, set), (0x7814, set), (0x781c, set), (0x7868, "pmgr_idle_enter")]
        .map(|(at, from)| (0xfffffe00_07000000 | at, from.to_owned(), WAIT))
        .to_vec();
    for target in ["0xfffffe0007007840", "pmgr_wait_pending", "sub_fffffe0007007840"] {
        let doc = json("xrefs", &["--to", target], PMGR);
        assert_eq!(doc["count"], 5, "{target}: {doc}");
        assert_eq!(sites(&doc), callers, "{target}");
        let coverage = &doc["coverage"];
        assert_eq!(coverage["decoded"], true, "{coverage}");
        assert_eq!(coverage["section"], "__TEXT_EXEC,__text", "{coverage}");
        assert_eq!(coverage["notes"], serde_json::json!([]), "{coverage}");
    }
    let (text, _) = kuna("xrefs", &["--to", "pmgr_wait_pending"], PMGR);
    assert!(text.starts_with("# 5 references to pmgr_wait_pending @ 0xfffffe0007007840\n"), "{text}");
    assert!(text.contains("pmgr_idle_enter+0x8\tbl 0xfffffe0007007840"), "{text}");

    let doc = json("xrefs", &["--from", "pmgr_set_perf_state"], PMGR);
    let targets: Vec<u64> = sites(&doc).iter().map(|s| s.2).collect();
    assert_eq!(targets, [KERNEL_ADD, WAIT, WAIT, WAIT, WAIT], "{doc}");
    let (text, _) = kuna("xrefs", &["--from", "0xfffffe0007007800"], PMGR);
    assert!(text.starts_with("# 5 references from pmgr_set_perf_state @ 0xfffffe0007007800\n"), "{text}");
}

#[test]
fn an_address_another_entry_holds_names_that_entry() {
    let doc = json("xrefs", &["--to", "0xfffffe0007007000"], PMGR);
    assert_eq!(sites(&doc), [(0xfffffe00_07007820, "pmgr_set_perf_state".to_owned(), KERNEL_ADD)], "{doc}");
    let coverage = &doc["coverage"];
    assert_eq!(coverage["decoded"], false, "{coverage}");
    assert_eq!(coverage["section"], serde_json::Value::Null, "{coverage}");
    let note = coverage["notes"][0].as_str().unwrap_or_default();
    assert!(
        note.contains("is in the MH_FILESET entry com.apple.kernel, not in com.example.driver.PMGR")
            && note.contains("only references made from com.example.driver.PMGR are listed")
            && note.contains("`--option kernelcache com.apple.kernel`"),
        "{coverage}"
    );

    let (text, stderr) = kuna("xrefs", &["--from", "0xfffffe0007007008"], PMGR);
    assert!(text.starts_with("# 0 references from 0xfffffe0007007008"), "{text}");
    assert!(stderr.contains("note: 0xfffffe0007007008 is in the MH_FILESET entry com.apple.kernel"), "{stderr}");

    let doc = json("xrefs", &["--to", "0xfffffe0007007840"], &[]);
    assert_eq!(doc["count"], 0, "{doc}");
    let note = doc["coverage"]["notes"][0].as_str().unwrap_or_default();
    assert!(note.contains("is in the MH_FILESET entry com.example.driver.PMGR, not in com.apple.kernel"), "{doc}");

    let doc = json("xrefs", &["--to", "kernel_add"], &[]);
    assert_eq!(sites(&doc), [(0xfffffe00_07007014, "kernel_main".to_owned(), KERNEL_ADD)], "{doc}");
    assert_eq!(doc["coverage"]["decoded"], true, "{doc}");
    assert_eq!(doc["coverage"]["notes"], serde_json::json!([]), "{doc}");
}

#[test]
fn image_scans_read_the_selected_entry() {
    let texts = |entry: &[&str]| -> Vec<String> {
        let doc = json("strings", &[], entry);
        doc["strings"].as_array().unwrap().iter().map(|s| s["text"].as_str().unwrap().to_owned()).collect()
    };
    assert_eq!(texts(PMGR), ["AppleExamplePMGR"]);
    assert_eq!(texts(&[]), ["xnu-example-kernel"]);

    let (graph, _) = kuna("decompile-graph", &[], PMGR);
    let graph: serde_json::Value = serde_json::from_str(&graph).unwrap();
    let edges: Vec<(u64, u64)> = graph["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e["callerAddress"].as_u64().unwrap(), e["calleeAddress"].as_u64().unwrap()))
        .collect();
    assert!(edges.contains(&(0xfffffe00_07007860, WAIT)), "pmgr_idle_enter -> pmgr_wait_pending: {edges:?}");
}

/// `kernelcache off` reads the bare collection on every surface. Its code is in
/// segments no section describes, so finding no function there is an error.
#[test]
fn kernelcache_off_finding_no_functions_is_an_error() {
    let fixture = common::fixture("kernelcache/fileset_calls.macho");
    let (_, stderr, rc) = common::run_kuna(&["functions", &fixture, "--option", "kernelcache", "off"]);
    assert_eq!(rc, 1, "{stderr}");
    assert!(stderr.contains("no functions discovered"), "{stderr}");
}

/// The dyld shared cache's image choice travels the same way.
#[test]
fn a_dyldimage_option_selects_the_image_xrefs_reads() {
    let cache = common::fixture("dyld_cache_arm64e/dyld_shared_cache_arm64e");
    let (stdout, stderr, rc) = common::run_kuna(&[
        "xrefs",
        &cache,
        "--from",
        "_caller_entry",
        "--option",
        "dyldimage",
        "libcaller.dylib",
    ]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains("call\tcallee_add\t@0x18000180c"), "{stdout}");
}
