//! Incoming parameters and reused frame objects have separate value identities.
mod common;

use serde_json::Value;
use std::process::Command;
use std::time::Duration;

fn query(enabled: bool, directives: &[String]) -> Value {
    query_for(
        "lifetimes_parameter_x86_64",
        "param_reuse",
        enabled,
        directives,
    )
}

fn query_for(fixture_name: &str, function: &str, enabled: bool, directives: &[String]) -> Value {
    let fixture = common::fixture(fixture_name);
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile",
            &fixture,
            function,
            "--mode",
            "reliable",
            "--json",
        ])
        .args(["--option", "structdefs", "on"])
        .args(["--option", "protoorder", "off", "--option", "stackviews"])
        .arg(if enabled { "on" } else { "off" })
        .args(["--assert-strict", "--assert"])
        .arg(format!("@{fixture}.kuna"))
        .arg("--sleighpath")
        .arg(common::repo_root().join("specs"));
    for directive in directives {
        command.args(["--assert", directive]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(15),
        Duration::from_millis(25),
    )
    .expect("bounded parameter-reuse decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = document["functions"][0].clone();
    assert!(function["error"].is_null(), "{function}");
    function
}

fn execute_source(label: &str, code: &str) {
    let source = common::scratch_file(label, "c");
    std::fs::write(&source, code).unwrap();
    for compiler in ["gcc", "clang"] {
        if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
            common::process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", level, "-fstrict-aliasing"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}

fn execute_views(function: &str, code: &str, expected: [i32; 2]) {
    assert!(!code.contains("&prepare_only"), "{code}");
    assert!(!code.contains("prepare_only.bytes"), "{code}");
    let mut context = String::from(
        "#include <stdint.h>\n#include <stdbool.h>\n\
         typedef int8_t int1; typedef uint8_t uint1;\n\
         typedef int16_t int2; typedef uint16_t uint2;\n\
         typedef int32_t int4; typedef uint32_t uint4;\n\
         typedef int64_t int8; typedef uint64_t uint8;\n\
         typedef uint8_t undefined1;\n\
         struct LifetimePair; struct LifetimeIncomingPair; struct LifetimeHandle;\n\
         int4 datum = 55; void *escaped_pointer;\n\
         int4 read_pair(const struct LifetimePair *);\n\
         int4 read_incoming_pair(const struct LifetimeIncomingPair *);\n\
         int4 read_handle(const struct LifetimeHandle *);\n\
         int4 read_saved_handle(void);\n",
    );
    for (name, fields) in [
        ("LifetimePair", "int4 x, y;"),
        ("LifetimeIncomingPair", "bool x; uint1 padding[3]; int4 y;"),
        ("LifetimeHandle", "int4 *value;"),
    ] {
        if !code.contains(&format!("struct {name} {{")) {
            context.push_str(&format!("struct {name} {{ {fields} }};\n"));
        }
    }
    let helpers = "\n\
        __attribute__((noinline)) int4 read_pair(const struct LifetimePair *p) { return p->x + p->y; }\n\
        __attribute__((noinline)) int4 read_incoming_pair(const struct LifetimeIncomingPair *p) { return p->x + p->y; }\n\
        __attribute__((noinline)) int4 read_handle(const struct LifetimeHandle *p) { return *p->value; }\n\
        __attribute__((noinline)) int4 read_saved_handle(void) { return **(int4 **)escaped_pointer; }\n";
    execute_source(
        function,
        &format!(
            "{context}\n{code}\n{helpers}\nint main(void) {{ return \
         {function}(1,2,3,4,5,6,false)!={} || {function}(1,2,3,4,5,6,true)!={}; }}\n",
            expected[0], expected[1],
        ),
    );
}

const VIEW_CASES: [(&str, [i32; 2]); 4] = [
    ("param_pair", [67, 99]),
    ("param_incoming", [62, 63]),
    ("param_mutation", [63, 63]),
    ("param_escaped", [62, 63]),
];
const VIEW_FIXTURE: &str = "lifetimes_parameter_views_x86_64";

#[test]
fn shared_parameter_views_preserve_input_bytes_stores_and_escaped_addresses() {
    common::process::required_output(&mut Command::new(common::fixture(VIEW_FIXTURE)));
    for (function, expected) in VIEW_CASES {
        let before = query_for(VIEW_FIXTURE, function, false, &[]);
        assert!(
            before["code"].as_str().unwrap().contains("&prepare_only"),
            "{before}"
        );
        let after = query_for(VIEW_FIXTURE, function, true, &[]);
        let objects = after["stack_objects"].as_array().unwrap();
        assert!(objects
            .iter()
            .all(|object| object["stack_offset"] == 8 && object["size"] == 8));
        assert_eq!(objects[0]["defined"], function == "param_pair");
        execute_views(function, after["code"].as_str().unwrap(), expected);
    }
}

#[test]
fn shared_parameter_view_names_and_types_keep_the_same_memory() {
    for (function, expected) in VIEW_CASES {
        let baseline = query_for(VIEW_FIXTURE, function, true, &[]);
        let names: Vec<_> = baseline["stack_objects"]
            .as_array()
            .unwrap()
            .iter()
            .zip(["first_pair", "replacement_handle"])
            .map(|(object, name)| {
                format!("name {function}::{} {name}", object["id"].as_str().unwrap())
            })
            .collect();
        let mut reversed = names.clone();
        reversed.reverse();
        let mut typed = names.clone();
        let fields = if function == "param_pair" {
            "int left; int right;"
        } else {
            "bool left; unsigned char padding[3]; int right;"
        };
        typed.push(format!("typedef struct SemanticParameter {{ {fields} }};"));
        typed.push(format!(
            "type {function}::first_pair struct SemanticParameter"
        ));
        for directives in [names, reversed, typed] {
            let after = query_for(VIEW_FIXTURE, function, true, &directives);
            let code = after["code"].as_str().unwrap();
            assert!(code.contains("first_pair"), "{code}");
            execute_views(function, code, expected);
        }
    }
}

fn execute(code: &str) {
    assert!(code.contains("if (!prepare_only)"), "{code}");
    assert!(!code.contains("&prepare_only"), "{code}");
    execute_source(
        "stack-parameter-lifetimes",
        &format!(
            "#include <stdbool.h>\ntypedef int int4; typedef unsigned int uint4;\n\
             __attribute__((noinline)) int4 read_key(const int4 *key) {{ return *key; }}\n\
             {code}\n\
             int main(void) {{ return param_reuse(1,2,3,4,5,6,false)!=21 || \
             param_reuse(1,2,3,4,5,6,true)!=99; }}\n"
        ),
    );
}

const VARIANT_FIXTURE: &str = "lifetimes_parameter_variants_x86_64";
const VARIANT_CASES: [(&str, &[(i64, i64, bool)]); 5] = [
    ("param_scalar_escaped", &[(8, 4, true)]),
    ("param_equal_float", &[(8, 8, true)]),
    ("param_multiple", &[(8, 16, false), (8, 8, true)]),
    ("param_multiple_interior", &[(8, 16, false), (16, 8, true)]),
    ("param_word_partial", &[(12, 4, false), (8, 8, true)]),
];

fn execute_variant(function: &str, code: &str) {
    let mut context = String::from(
        "#include <stdint.h>\n#include <stdbool.h>\n\
         typedef int8_t int1; typedef uint8_t uint1;\n\
         typedef int16_t int2; typedef uint16_t uint2;\n\
         typedef int32_t int4; typedef uint32_t uint4;\n\
         typedef int64_t int8; typedef uint64_t uint8;\n\
         typedef double float8; typedef float float4; typedef uint8_t undefined1;\n\
         struct LifetimeHandle; struct LifetimeParameterPair;\n\
         int4 datum = 55; void *escaped_pointer;\n\
         int4 read_word(const int4 *); int4 read_saved_word(void);\n\
         int4 read_float_handle(const struct LifetimeHandle *);\n\
         int4 read_parameter_pair(const struct LifetimeParameterPair *);\n\
         int4 read_parameter_handle(const struct LifetimeHandle *);\n",
    );
    for (name, fields) in [
        ("LifetimeHandle", "int4 *value;"),
        ("LifetimeParameterPair", "bool x; uint1 padding[7]; int8 y;"),
    ] {
        if !code.contains(&format!("struct {name} {{")) {
            context.push_str(&format!("struct {name} {{ {fields} }};\n"));
        }
    }
    let helpers = "\n\
        __attribute__((noinline)) int4 read_word(const int4 *p) { return *p; }\n\
        __attribute__((noinline)) int4 read_saved_word(void) { return *(int4 *)escaped_pointer; }\n\
        __attribute__((noinline)) int4 read_float_handle(const struct LifetimeHandle *p) { return *p->value; }\n\
        __attribute__((noinline)) int4 read_parameter_handle(const struct LifetimeHandle *p) { return *p->value; }\n\
        __attribute__((noinline)) int4 read_parameter_pair(const struct LifetimeParameterPair *p) { return p->x + p->y; }\n";
    let (arguments, expected) = match function {
        "param_scalar_escaped" => (["1,2,3,4,5,6,false", "1,2,3,4,5,6,true"], [21, 99]),
        "param_equal_float" => (["0,0,0,0,0,0,0,0,13.0", "0,0,0,0,0,0,0,0,17.0"], [68, 72]),
        "param_multiple" | "param_multiple_interior" => {
            (["1,2,3,4,5,6,false,13", "1,2,3,4,5,6,true,17"], [68, 73])
        }
        "param_word_partial" => (
            ["1,2,3,4,5,6,0xd00000001LL", "1,2,3,4,5,6,0x1100000001LL"],
            [68, 72],
        ),
        _ => unreachable!(),
    };
    execute_source(
        function,
        &format!(
            "{context}\n{code}\n{helpers}\nint main(void) {{ return \
             {function}({})!={} || {function}({})!={}; }}\n",
            arguments[0], expected[0], arguments[1], expected[1],
        ),
    );
}

#[test]
fn parameter_backing_preserves_equal_width_multiple_and_interior_values() {
    common::process::required_output(&mut Command::new(common::fixture(VARIANT_FIXTURE)));
    for (function, expected) in VARIANT_CASES {
        let before = query_for(VARIANT_FIXTURE, function, false, &[]);
        assert!(before["stack_objects"]
            .as_array()
            .is_none_or(|objects| objects.is_empty()));
        let after = query_for(VARIANT_FIXTURE, function, true, &[]);
        let objects = after["stack_objects"].as_array().unwrap();
        assert_eq!(objects.len(), expected.len(), "{after}");
        for (object, &(offset, size, defined)) in objects.iter().zip(expected) {
            assert_eq!(object["stack_offset"], offset, "{object}");
            assert_eq!(object["size"], size, "{object}");
            assert_eq!(object["defined"], defined, "{object}");
        }
        execute_variant(function, after["code"].as_str().unwrap());
    }
}

#[test]
fn parameter_backing_preserves_logical_names_types_and_replay_order() {
    for (function, _) in VARIANT_CASES {
        let baseline = query_for(VARIANT_FIXTURE, function, true, &[]);
        let names: Vec<_> = baseline["stack_objects"]
            .as_array()
            .unwrap()
            .iter()
            .zip(["first_view", "second_view"])
            .map(|(object, name)| {
                format!("name {function}::{} {name}", object["id"].as_str().unwrap())
            })
            .collect();
        let mut reversed = names.clone();
        reversed.reverse();
        let mut typed = names.clone();
        let datatype = match function {
            "param_scalar_escaped" | "param_word_partial" => "unsigned int",
            "param_equal_float" => {
                typed.push("typedef struct SemanticHandle { int *resource; };".to_string());
                "struct SemanticHandle"
            }
            _ => {
                typed.push("typedef struct SemanticParameters { bool flag; unsigned char padding[7]; long long value; };".to_string());
                "struct SemanticParameters"
            }
        };
        typed.push(format!("type {function}::first_view {datatype}"));
        for directives in [names, reversed, typed] {
            let after = query_for(VARIANT_FIXTURE, function, true, &directives);
            let code = after["code"].as_str().unwrap();
            assert!(code.contains("first_view"), "{code}");
            execute_variant(function, code);
        }
    }
}

#[test]
fn formal_names_and_types_survive_shared_storage_replay() {
    for (function, old_name, new_name, datatype) in [
        ("param_multiple", "a", "unused_source", "int"),
        ("param_scalar_escaped", "prepare_only", "source_flag", "bool"),
        ("param_equal_float", "prepared_value", "source_value", "double"),
        ("param_multiple", "prepare_only", "source_flag", "bool"),
        ("param_multiple_interior", "carried_value", "source_value", "unsigned long long"),
        ("param_word_partial", "incoming_word", "source_bits", "unsigned long long"),
    ] {
        let baseline = query_for(VARIANT_FIXTURE, function, true, &[]);
        let object = baseline["stack_objects"][0]["id"].as_str().unwrap();
        let name = format!("name {function}::{old_name} {new_name}");
        let retype = format!("type {function}::{old_name} {datatype}");
        let logical = format!("name {function}::{object} observed_storage");
        for directives in [
            vec![name.clone()],
            vec![retype.clone(), name.clone()],
            vec![logical.clone(), name.clone(), retype.clone()],
        ] {
            let after = query_for(VARIANT_FIXTURE, function, true, &directives);
            let code = after["code"].as_str().unwrap();
            let header = code.split_once("\n{").unwrap().0;
            let signature = header.lines().last().unwrap();
            assert!(signature.contains(new_name), "{code}");
            let parameters = signature.split_once('(').unwrap().1;
            assert!(
                !parameters.split(',').any(|parameter| {
                    parameter.trim_end_matches(')').split_whitespace().last() == Some(old_name)
                }),
                "{code}"
            );
            execute_variant(function, code);
        }
    }
}

#[test]
fn inferred_register_parameter_rename_keeps_the_argument() {
    let fixture = common::fixture("lifetimes_stack_x86_64");
    let (document, stderr, _) = common::run_kuna(&[
        "decompile", &fixture, "converted_view", "--json", "--mode", "reliable",
        "--option", "protoorder", "off", "--option", "structdefs", "on",
        "--option", "stackviews", "on", "--assert-strict",
        "--assert", "prototype read_float int read_float(const float *value)",
        "--assert", "prototype read_integer int read_integer(const int *value)",
        "--assert", "name converted_view::a0 source_value",
    ]);
    let parsed: Value = serde_json::from_str(&document).unwrap_or_else(|e| panic!("{e}: {stderr}"));
    let code = parsed["functions"][0]["code"].as_str().unwrap();
    assert!(code.contains("converted_view(int4 source_value)"), "{code}");
    execute_source("inferred-parameter-replay", &format!(
        "#include <stdint.h>\ntypedef int32_t int4; typedef uint32_t uint4;\n\
         typedef uint16_t uint2; typedef uint8_t uint1; typedef float float4;\n\
         int4 read_float(const float *); int4 read_integer(const int4 *);\n\
         {code}\n\
         __attribute__((noinline)) int4 read_float(const float *p) {{ return (int4)*p; }}\n\
         __attribute__((noinline)) int4 read_integer(const int4 *p) {{ return *p; }}\n\
         int main(void) {{ return converted_view(7)!=16 || converted_view(-7)!=2; }}\n"
    ));
}

#[test]
fn incoming_parameter_names_swap_without_changing_storage() {
    let first = "name param_multiple::prepare_only carried_value".to_string();
    let second = "name param_multiple::carried_value prepare_only".to_string();
    for directives in [vec![first.clone(), second.clone()], vec![second, first]] {
        let after = query_for(VARIANT_FIXTURE, "param_multiple", true, &directives);
        let code = after["code"].as_str().unwrap();
        assert!(code.contains("bool carried_value,int8 prepare_only)"), "{code}");
        execute_variant("param_multiple", code);
    }
}

#[test]
fn narrow_input_is_read_before_full_width_storage_reuse() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_parameter_x86_64",
    )));
    let before = query(false, &[]);
    assert!(
        before["code"].as_str().unwrap().contains("&prepare_only"),
        "{before}"
    );
    let after = query(true, &[]);
    let objects = after["stack_objects"].as_array().unwrap();
    assert_eq!(objects.len(), 2);
    assert!(objects.iter().all(|object| object["defined"] == true
        && object["stack_offset"] == 8
        && object["size"] == 4));
    execute(after["code"].as_str().unwrap());
}

#[test]
fn names_and_types_survive_replay_without_replacing_the_input() {
    let baseline = query(true, &[]);
    let word = baseline["variables"]
        .as_array()
        .unwrap()
        .iter()
        .find(|variable| {
            variable["kind"] == "stack" && variable["stack_offset"] == 8 && variable["size"] == 4
        })
        .unwrap()["name"]
        .as_str()
        .unwrap();
    let objects = baseline["stack_objects"].as_array().unwrap();
    let first = objects[0]["id"].as_str().unwrap();
    let second = objects[1]["id"].as_str().unwrap();
    let logical = vec![
        format!("name param_reuse::{first} first_key"),
        format!("name param_reuse::{second} second_key"),
        "type param_reuse::first_key int".to_string(),
    ];
    let mut reversed = logical.clone();
    reversed.swap(0, 1);
    for directives in [
        vec![format!("name param_reuse::{word} key_storage")],
        vec![format!("type param_reuse::{word} unsigned int key_storage")],
        logical,
        reversed,
    ] {
        let after = query(true, &directives);
        execute(after["code"].as_str().unwrap());
    }
}
