//! (kuna `reglocals`) A local the C declares outside the stack frame is a
//! `kind: "register"` row of `decompile-all --json`'s `variables`, under its
//! printed name and type and with the instructions that use it; the emitted C
//! does not move.
use crate::common;

use serde_json::Value;
use std::process::Command;

fn run(reglocals: &str, jobs: &str) -> Value {
    run_on("aliasoverlap_x86_64", "inside,indexed", reglocals, jobs)
}

fn run_on(fixture: &str, functions: &str, reglocals: &str, jobs: &str) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            functions,
            "--jobs",
            jobs,
            "--json",
            "--option",
            "reglocals",
            reglocals,
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"))
        .output()
        .expect("spawn kuna");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).expect("decompile-all --json")
}

fn function<'a>(doc: &'a Value, name: &str) -> &'a Value {
    doc["functions"]
        .as_array()
        .expect("functions")
        .iter()
        .find(|f| f["name"] == name)
        .unwrap_or_else(|| panic!("no {name} in {doc}"))
}

fn rows<'a>(f: &'a Value, kind: &str) -> Vec<&'a Value> {
    f["variables"].as_array().expect("variables").iter().filter(|v| v["kind"] == kind).collect()
}

#[test]
fn a_register_local_is_reported_under_its_declaration_with_its_uses() {
    let doc = run("on", "1");
    let f = function(&doc, "inside");
    let code = f["code"].as_str().expect("code");
    let lines: Vec<&str> = code.lines().collect();
    let registers = rows(f, "register");
    assert_eq!(registers.len(), 1, "{f}");
    let v = registers[0];
    let (name, ty) = (v["name"].as_str().unwrap(), v["type"].as_str().unwrap());
    assert!(
        lines.iter().any(|l| l.trim_start().strip_prefix(ty).is_some_and(|rest| rest.trim_start().starts_with(&format!("{name};")))),
        "no `{ty} {name};` declaration in:\n{code}"
    );
    assert!(v["stack_offset"].is_null() && v["arg_index"].is_null(), "{v}");
    assert_eq!(v["size"], 4, "{v}");

    let uses = v["line_numbers"].as_array().unwrap();
    assert!(!uses.is_empty(), "{v}");
    for line in uses {
        let text = lines[line.as_u64().unwrap() as usize - 1];
        assert!(text.contains(name), "line {line} `{text}` does not use {name}");
    }
    let (start, size) = (f["address"].as_u64().unwrap(), f["size"].as_u64().unwrap());
    let addresses = v["addresses"].as_array().unwrap();
    assert!(!addresses.is_empty(), "{v}");
    assert!(
        addresses.iter().all(|a| (start..start + size).contains(&a.as_u64().unwrap())),
        "{v} outside {start:#x}+{size:#x}"
    );
}

#[test]
fn reglocals_moves_only_the_variables_array() {
    let (on, off) = (run("on", "1"), run("off", "1"));
    for name in ["inside", "indexed"] {
        let (a, b) = (function(&on, name), function(&off, name));
        assert_eq!(a["code"], b["code"], "{name}: the emitted C moved");
        assert!(rows(b, "register").is_empty(), "{b}");
        assert!(!rows(a, "register").is_empty(), "{a}");
        let frame: Vec<&Value> = a["variables"].as_array().unwrap().iter().filter(|v| v["kind"] != "register").collect();
        let before: Vec<&Value> = b["variables"].as_array().unwrap().iter().collect();
        assert_eq!(frame, before, "{name}: a parameter or frame row moved");
    }
}

#[test]
fn a_worker_process_reports_the_same_register_rows() {
    let (serial, workers) = (run("on", "1"), run("on", "2"));
    for name in ["inside", "indexed"] {
        assert_eq!(function(&serial, name)["variables"], function(&workers, name)["variables"], "{name}");
    }
}

#[test]
fn a_register_array_is_spelled_like_a_frame_array() {
    let doc = run_on("argclobber_pair_x86_64", "sub_401010", "on", "1");
    let f = function(&doc, "sub_401010");
    let code = f["code"].as_str().expect("code");
    assert!(code.contains("char v1 [16]; // rdx:rax"), "{code}");
    let registers = rows(f, "register");
    assert_eq!(registers.len(), 1, "{f}");
    assert_eq!(registers[0]["type"], "char[16]", "{f}");
    assert_eq!(registers[0]["size"], 16, "{f}");
}

#[test]
fn a_declaration_the_c_places_on_the_stack_is_not_a_register_row() {
    let doc = run_on("arraycoverwidth_x86_64", "vm", "on", "1");
    let f = function(&doc, "vm");
    let code = f["code"].as_str().expect("code");
    assert!(code.contains("char v1 [16]; // stack - 0x38"), "{code}");
    assert!(rows(f, "register").is_empty(), "{f}");
    assert!(rows(f, "stack").iter().any(|v| v["stack_offset"] == -0x38), "{f}");
}
