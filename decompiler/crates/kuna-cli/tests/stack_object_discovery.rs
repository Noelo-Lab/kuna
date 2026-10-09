//! Discovered logical selectors round-trip through assertions and batch workers.
mod common;

use serde_json::Value;
use std::process::Command;
use std::time::Duration;

fn query(enabled: bool, directives: &[String], no_vars: bool) -> Value {
    query_functions(enabled, directives, no_vars, "obj_repeat,obj_reuse")
}

fn query_functions(enabled: bool, directives: &[String], no_vars: bool, functions: &str) -> Value {
    let fixture = common::fixture("lifetimes_objects_x86_64");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile-all",
            &fixture,
            "--functions",
            functions,
            "--jobs",
            "1",
            "--jobs-chunk",
            "1",
            "--mode",
            "reliable",
            "--json",
            "--assert-strict",
            "--assert",
        ])
        .arg(format!("@{fixture}.kuna"))
        .args([
            "--option",
            "stackviews",
            if enabled { "on" } else { "off" },
            "--option",
            "stackalias",
            "off",
            "--option",
            "structdefs",
            "on",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"));
    for directive in directives {
        command.args(["--assert", directive]);
    }
    if no_vars {
        command.arg("--no-vars");
    }
    json_output(&mut command)
}

fn worker_query(jobs: &str, enabled: bool, no_vars: bool) -> Value {
    dwarf_query(jobs, enabled, no_vars, &[])
}

fn dwarf_query(jobs: &str, enabled: bool, no_vars: bool, directives: &[String]) -> Value {
    let fixture = common::fixture("dwarfstructs_x86_64");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile-all",
            &fixture,
            "--functions",
            "main,take_nest",
            "--jobs",
            jobs,
            "--jobs-chunk",
            "1",
            "--mode",
            "reliable",
            "--json",
            "--option",
            "protoorder",
            "off",
            "--option",
            "stackviews",
            if enabled { "on" } else { "off" },
            "--option",
            "stackalias",
            "off",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"));
    if no_vars {
        command.arg("--no-vars");
    }
    if !directives.is_empty() {
        command.arg("--assert-strict");
        for directive in directives {
            command.args(["--assert", directive]);
        }
    }
    json_output(&mut command)
}

fn json_output(command: &mut Command) -> Value {
    let output = common::process::output_with_timeout(
        command,
        Duration::from_secs(30),
        Duration::from_millis(25),
    )
    .expect("stack-object JSON discovery");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn function<'a>(doc: &'a Value, name: &str) -> &'a Value {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"] == name)
        .unwrap()
}

#[test]
fn typed_global_objects_have_stable_selectors_and_replay_names_and_types() {
    let functions = "obj_global_reuse,obj_volatile_reuse,obj_global_shifted,obj_volatile_shifted,obj_indirect_reuse";
    let original = query_functions(true, &[], false, functions);
    let mut directives = vec!["typedef struct SemanticPair { int left; int right; };".to_string()];
    for (name, slot, shift) in [
        ("obj_global_reuse", 0, 0),
        ("obj_volatile_reuse", 2, 0),
        ("obj_global_shifted", 0, 4),
        ("obj_volatile_shifted", 2, 4),
        ("obj_indirect_reuse", 2, 0),
    ] {
        let objects = function(&original, name)["stack_objects"]
            .as_array()
            .unwrap();
        assert_eq!(objects.len(), 2);
        assert_ne!(objects[0]["id"], objects[1]["id"]);
        assert_eq!(
            objects[0]["stack_offset"].as_i64().unwrap() + shift,
            objects[1]["stack_offset"].as_i64().unwrap()
        );
        for (object, label) in objects.iter().zip(["pair_phase", "handle_phase"]) {
            assert_eq!(object["defined"], true);
            assert_eq!(object["size"], 8);
            let uses = object["uses"].as_array().unwrap();
            assert_eq!(uses.len(), 1);
            assert_eq!(uses[0]["slot"], slot);
            directives.push(format!(
                "name {name}::{} {label}",
                object["id"].as_str().unwrap()
            ));
        }
        directives.push(format!("type {name}::pair_phase struct SemanticPair"));
    }
    let replayed = query_functions(true, &directives, false, functions);
    for name in [
        "obj_global_reuse",
        "obj_volatile_reuse",
        "obj_global_shifted",
        "obj_volatile_shifted",
        "obj_indirect_reuse",
    ] {
        let before = function(&original, name)["stack_objects"]
            .as_array()
            .unwrap();
        let row = function(&replayed, name);
        let after = row["stack_objects"].as_array().unwrap();
        for ((before, after), label) in before.iter().zip(after).zip(["pair_phase", "handle_phase"])
        {
            assert_eq!(before["id"], after["id"]);
            assert_eq!(before["uses"], after["uses"]);
            assert_eq!(after["name"], label);
            assert!(row["code"].as_str().unwrap().contains(&format!(".{label}")));
        }
    }
}

#[test]
fn discovered_selectors_replay_asserted_names() {
    let original = query(true, &[], false);
    let repeated = &function(&original, "obj_repeat")["stack_objects"];
    assert_eq!(repeated.as_array().unwrap().len(), 1);
    assert_eq!(repeated[0]["uses"].as_array().unwrap().len(), 2);
    let reused = function(&original, "obj_reuse")["stack_objects"]
        .as_array()
        .unwrap();
    assert_eq!(reused.len(), 2);
    assert_ne!(reused[0]["id"], reused[1]["id"]);
    assert_eq!(reused[0]["stack_offset"], reused[1]["stack_offset"]);
    let directives: Vec<_> = reused
        .iter()
        .zip(["initial", "replacement"])
        .map(|(object, name)| format!("name obj_reuse::{} {name}", object["id"].as_str().unwrap()))
        .collect();
    for directives in [directives.clone(), directives.into_iter().rev().collect()] {
        let doc = query(true, &directives, false);
        let row = function(&doc, "obj_reuse");
        let objects = row["stack_objects"].as_array().unwrap();
        for ((object, original), name) in objects.iter().zip(reused).zip(["initial", "replacement"])
        {
            assert_eq!(object["id"], original["id"]);
            assert_eq!(object["name"], name);
            assert_eq!(object["stack_offset"], original["stack_offset"]);
            assert_eq!(object["size"], 8);
            assert_eq!(object["defined"], true);
            assert_eq!(object["uses"], original["uses"]);
            let use_ = &object["uses"][0];
            assert!(use_["address"].as_u64().unwrap() >= 0x401000);
            assert_eq!(use_["slot"], 1);
            assert!(use_["type"].as_str().unwrap().contains("LifetimePair"));
            assert!(row["code"].as_str().unwrap().contains(&format!(".{name}")));
        }
        assert_eq!(function(&doc, "obj_repeat")["stack_objects"], *repeated);
    }
}

#[test]
fn worker_transport_preserves_discovered_object_metadata() {
    let serial = worker_query("1", true, false);
    let pooled = worker_query("2", true, false);
    assert!(!function(&serial, "main")["stack_objects"]
        .as_array()
        .unwrap()
        .is_empty());
    for name in ["main", "take_nest"] {
        assert_eq!(
            function(&serial, name)["stack_objects"],
            function(&pooled, name)["stack_objects"]
        );
    }
}

#[test]
fn discovered_objects_bind_names_through_existing_dwarf_layouts() {
    let original = dwarf_query("1", true, false, &[]);
    let objects = function(&original, "main")["stack_objects"]
        .as_array()
        .unwrap();
    let selected: Vec<_> = objects
        .iter()
        .filter(|object| object["uses"][0]["type"] == "Big24" || object["uses"][0]["type"] == "U4")
        .collect();
    assert_eq!(selected.len(), 3);
    let names = ["returned", "first_union", "second_union"];
    let directives: Vec<_> = selected
        .iter()
        .zip(names)
        .map(|(object, name)| format!("name main::{} {name}", object["id"].as_str().unwrap()))
        .collect();
    for directives in [directives.clone(), directives.into_iter().rev().collect()] {
        let doc = dwarf_query("1", true, false, &directives);
        let row = function(&doc, "main");
        let recovered = row["stack_objects"].as_array().unwrap();
        for (original, name) in selected.iter().zip(names) {
            let object = recovered
                .iter()
                .find(|o| o["id"] == original["id"])
                .unwrap();
            assert_eq!(object["name"], name);
            assert_eq!(object["uses"], original["uses"]);
            assert_eq!(object["stack_offset"], original["stack_offset"]);
            assert_eq!(object["size"], original["size"]);
            assert!(row["code"]
                .as_str()
                .unwrap()
                .contains(&format!("{name}_view")));
        }
        assert!(row["code"].as_str().unwrap().contains("Big24 b;"));
    }
}

#[test]
fn disabled_recovery_and_no_vars_keep_the_default_json_shape() {
    for (enabled, no_vars) in [(false, false), (true, true)] {
        for doc in [
            query(enabled, &[], no_vars),
            worker_query("2", enabled, no_vars),
        ] {
            for function in doc["functions"].as_array().unwrap() {
                assert!(function.get("stack_objects").is_none());
                if no_vars {
                    assert_eq!(function["variables"].as_array().unwrap().len(), 0);
                }
            }
        }
    }
}
