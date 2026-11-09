//! The JVM engine's CLI surface: routing, every subcommand's text and JSON,
//! the project export, selection, options, capability errors and exit codes.
//! Every run passes `--option jvm_include_runtime off`, so nothing depends on
//! a JDK being installed.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::Value;

use crate::common;

const NO_JDK: [&str; 3] = ["--option", "jvm_include_runtime", "off"];

fn fixtures() -> PathBuf {
    common::repo_root().join("tests/fixtures/jvm")
}

fn jar() -> String {
    fixtures().join("app.jar").display().to_string()
}

fn classes_dir() -> String {
    fixtures().join("classes").display().to_string()
}

fn main_class() -> String {
    fixtures().join("classes/com/ex/Main.class").display().to_string()
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("spawn kuna")
}

/// `kuna <sub> <args..> --option jvm_include_runtime off`.
fn jvm(args: &[&str]) -> Output {
    let mut all: Vec<&str> = args.to_vec();
    all.extend(NO_JDK);
    kuna(&all)
}

fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("UTF-8 stdout")
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn ok(out: &Output) -> String {
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(out));
    stdout(out)
}

fn json(out: &Output) -> Value {
    serde_json::from_str(&ok(out)).expect("a JSON document")
}

fn keys(v: &Value) -> Vec<&str> {
    v.as_object().expect("object").keys().map(String::as_str).collect()
}

fn scratch_dir(stem: &str) -> PathBuf {
    let dir = common::scratch_file(stem, "d");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

const METHODS: &str = "\
com.ex.Main.<init>(I)V
com.ex.Main.main([Ljava/lang/String;)V
com.ex.Main.check(Ljava/lang/String;)Z
com.ex.Main.add(II)I
com.ex.Main.add(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
com.ex.Main$1.<init>()V
com.ex.Main$1.run()V
com.ex.Main$Checker.<init>()V
com.ex.Main$Checker.verify(Ljava/lang/String;)Z
com.ex.util.Helper.<init>()V
com.ex.util.Helper.twice(I)I
com.ex.util.Helper.loop(I)I
";

#[test]
fn a_jar_and_a_directory_list_the_same_methods() {
    assert_eq!(ok(&jvm(&["functions", &jar()])), METHODS);
    assert_eq!(ok(&jvm(&["functions", &classes_dir()])), METHODS);
}

#[test]
fn a_class_file_brings_its_nested_classes_unless_told_not_to() {
    let listed = ok(&jvm(&["functions", &main_class()]));
    assert!(listed.contains("com.ex.Main$Checker.verify(Ljava/lang/String;)Z"), "{listed}");
    assert!(!listed.contains("com.ex.util.Helper"), "{listed}");
    let alone = ok(&jvm(&["functions", &main_class(), "--option", "jvm_siblings", "off"]));
    assert!(alone.contains("com.ex.Main.check(Ljava/lang/String;)Z"), "{alone}");
    assert!(!alone.contains("Checker"), "{alone}");
}

#[test]
fn a_fat_macho_still_takes_the_native_path() {
    let fat = common::fixture("macho_fat");
    let head = std::fs::read(&fat).unwrap();
    assert_eq!(&head[..4], &[0xCA, 0xFE, 0xBA, 0xBE], "the fixture shares the class-file magic");
    let out = kuna(&["functions", &fat, "--json"]);
    let text = stdout(&out) + &stderr(&out);
    assert!(!text.contains("\"engine\": \"jvm\"") && !text.contains("JVM"), "{text}");
}

#[test]
fn raw_image_inputs_stay_native() {
    let out = kuna(&["decompile", &jar(), "0x0", "--raw-image"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("--raw-image requires --target"), "{}", stderr(&out));
}

#[test]
fn decompiling_a_class_prints_its_compilation_unit() {
    let unit = ok(&jvm(&["decompile", &jar(), "com.ex.Main"]));
    assert!(unit.starts_with("package com.ex;\n\nimport com.ex.util.Helper;\n"), "{unit}");
    assert!(unit.contains("   public static boolean check(String key) {\n"), "{unit}");
    assert!(unit.contains("   static class Checker {\n"), "{unit}");
    for spelling in ["com/ex/Main", "Main", "Main.class"] {
        assert_eq!(ok(&jvm(&["decompile", &jar(), spelling])), unit, "{spelling}");
    }
    assert_eq!(ok(&jvm(&["decompile", &main_class()])), unit, "a lone class file needs no target");
}

#[test]
fn a_nested_class_prints_its_own_part() {
    let nested = ok(&jvm(&["decompile", &jar(), "com.ex.Main.Checker"]));
    assert!(nested.starts_with("static class Checker {\n   static boolean verify(String key) {\n"), "{nested}");
    assert!(nested.ends_with("}\n"), "{nested}");
    assert_eq!(ok(&jvm(&["decompile", &jar(), "com.ex.Main$Checker"])), nested);
}

#[test]
fn a_method_prints_its_slice_without_indentation() {
    let check = ok(&jvm(&["decompile", &jar(), "com.ex.Main.check"]));
    assert_eq!(
        check,
        "public static boolean check(String key) {\n   return key.length() == 16 && Main.Checker.verify(key);\n}\n"
    );
    assert_eq!(ok(&jvm(&["decompile", &jar(), "com.ex.Main#check"])), check);
    assert_eq!(ok(&jvm(&["decompile", &jar(), "com/ex/Main.check(Ljava/lang/String;)Z"])), check);
    assert_eq!(ok(&jvm(&["decompile", &main_class(), "check"])), check);
}

#[test]
fn overloads_print_together_unless_a_descriptor_picks_one() {
    let out = jvm(&["decompile", &jar(), "com.ex.Main.add"]);
    let both = ok(&out);
    assert!(both.contains("public int add(int a, int b) {") && both.contains("public String add(String a, String b) {"), "{both}");
    assert!(stderr(&out).contains("2 overloads"), "{}", stderr(&out));
    let one = ok(&jvm(&["decompile", &jar(), "com.ex.Main.add(II)I"]));
    assert_eq!(one, "public int add(int a, int b) {\n   return this.base + a + b;\n}\n");
}

#[test]
fn misses_name_the_nearest_class_or_method() {
    let out = jvm(&["decompile", &jar(), "com.ex.Mian"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("did you mean \"com.ex.Main\"?"), "{}", stderr(&out));
    let out = jvm(&["decompile", &jar(), "com.ex.Main.chek"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("did you mean \"com.ex.Main.check\"?"), "{}", stderr(&out));
    let out = jvm(&["decompile", &jar(), "com.ex.Main.add(J)J"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("its overloads are com.ex.Main.add(II)I"), "{}", stderr(&out));
    let out = jvm(&["decompile", &jar()]);
    assert_eq!(out.status.code(), Some(2), "a jar needs a target");
}

#[test]
fn decompile_json_is_the_native_document_with_java_records() {
    let doc = json(&jvm(&["decompile", &jar(), "com.ex.Main.check", "--json"]));
    assert_eq!(
        keys(&doc),
        ["binary", "engine", "language", "omitted", "count", "error", "runtime", "functions", "classes", "assertions"]
    );
    assert_eq!(doc["engine"], "jvm");
    assert_eq!(doc["language"], "java");
    assert_eq!(doc["omitted"], serde_json::json!(["variables", "types", "stack_objects"]));
    assert_eq!(doc["count"], 1);
    assert!(doc["error"].is_null() && doc["runtime"].is_null());
    assert_eq!(doc["classes"], serde_json::json!([]));
    assert_eq!(doc["assertions"], serde_json::json!([]));
    let record = &doc["functions"][0];
    assert_eq!(
        keys(record),
        [
            "name", "address", "address_hex", "aliases", "object_location", "size", "code", "error",
            "unstructured_gotos", "line_mappings", "class", "method", "descriptor", "access", "path", "line"
        ]
    );
    assert_eq!(record["name"], "com.ex.Main.check");
    assert!(record["address"].is_null() && record["address_hex"].is_null() && record["object_location"].is_null());
    assert_eq!(record["size"], 22, "the bytecode length");
    assert_eq!(record["descriptor"], "(Ljava/lang/String;)Z");
    assert_eq!(record["access"], serde_json::json!(["public", "static"]));
    assert_eq!(record["path"], "com/ex/Main.java");
    assert!(record["code"].as_str().unwrap().starts_with("public static boolean check(String key) {\n"));
    let line = record["line"].as_u64().unwrap() as usize;
    let unit = ok(&jvm(&["decompile", &jar(), "com.ex.Main"]));
    assert!(unit.lines().nth(line - 1).unwrap().contains("check(String key)"), "line {line}");
    let mappings = record["line_mappings"].as_array().unwrap();
    assert!(!mappings.is_empty());
    for m in mappings {
        assert!(m["line_number"].as_u64().unwrap() >= 1);
        assert!(m["addresses"].as_array().unwrap().iter().all(|a| a.as_u64().unwrap() < 22));
    }
}

#[test]
fn decompile_json_of_a_class_carries_the_unit_and_its_methods() {
    let doc = json(&jvm(&["decompile", &jar(), "com.ex.Main", "--json", "--omit", "line_mappings"]));
    assert_eq!(doc["omitted"], serde_json::json!(["line_mappings", "variables", "types", "stack_objects"]));
    let classes = doc["classes"].as_array().unwrap();
    assert_eq!(classes.len(), 1);
    assert_eq!(keys(&classes[0]), ["name", "path", "size", "code", "error"]);
    assert_eq!(classes[0]["name"], "com.ex.Main");
    assert_eq!(classes[0]["code"].as_str().unwrap(), ok(&jvm(&["decompile", &jar(), "com.ex.Main"])));
    let names: Vec<&str> = doc["functions"].as_array().unwrap().iter().map(|f| f["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"com.ex.Main$Checker.verify") && names.contains(&"com.ex.Main$1.run"), "{names:?}");
    assert!(doc["functions"][0].get("line_mappings").is_none());
}

#[test]
fn decompile_all_text_is_every_unit_under_a_class_header() {
    let text = ok(&jvm(&["decompile-all", &jar()]));
    assert!(text.starts_with("// Class: com.ex.Main  (com/ex/Main.java)\npackage com.ex;\n"), "{text}");
    assert!(text.contains("\n// Class: com.ex.util.Helper  (com/ex/util/Helper.java)\npackage com.ex.util;\n"), "{text}");
}

#[test]
fn decompile_all_triage_narrows_to_method_slices() {
    let text = ok(&jvm(&["decompile-all", &jar(), "--filter", "loop|verify"]));
    assert!(text.starts_with("// Method: com.ex.Main$Checker.verify(Ljava/lang/String;)Z  (com/ex/Main.java:"), "{text}");
    assert!(text.contains("// Method: com.ex.util.Helper.loop(I)I  (com/ex/util/Helper.java:"), "{text}");
    assert!(!text.contains("// Class:"), "{text}");
    let doc = json(&jvm(&["decompile-all", &jar(), "--json", "--sort", "size", "--limit", "1"]));
    assert_eq!(doc["count"], 1);
    assert_eq!(doc["total"], 12, "every method of the class files");
    assert_eq!(doc["functions"][0]["name"], "com.ex.Main.main");
    assert_eq!(doc["classes"], serde_json::json!([]));
}

#[test]
fn decompile_all_json_lists_methods_and_units() {
    let doc = json(&jvm(&["decompile-all", &jar(), "--json"]));
    assert!(doc.get("total").is_none(), "no triage, no total");
    let classes: Vec<&str> = doc["classes"].as_array().unwrap().iter().map(|c| c["name"].as_str().unwrap()).collect();
    assert_eq!(classes, ["com.ex.Main", "com.ex.util.Helper"]);
    assert_eq!(doc["count"].as_u64().unwrap() as usize, doc["functions"].as_array().unwrap().len());
    let only = json(&jvm(&["decompile-all", &jar(), "--json", "--functions", "com.ex.util.Helper", "--omit", "classes"]));
    assert!(only.get("classes").is_none());
    assert!(only["omitted"].as_array().unwrap().contains(&Value::from("classes")));
    let names: Vec<&str> = only["functions"].as_array().unwrap().iter().map(|f| f["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["com.ex.util.Helper.<init>", "com.ex.util.Helper.twice", "com.ex.util.Helper.loop"]);
}

#[test]
fn worker_count_does_not_change_a_byte() {
    let dir = common::repo_root().join("tests/jvm/classes/java17").display().to_string();
    let one = ok(&jvm(&["decompile-all", &dir, "--json", "--jobs", "1"]));
    let eight = ok(&jvm(&["decompile-all", &dir, "--json", "--jobs", "8"]));
    assert!(one == eight, "--jobs 1 and --jobs 8 differ");
    assert!(one.matches("\"name\": ").count() > 38);
}

#[test]
fn decompile_project_writes_a_java_source_tree() {
    let out_dir = scratch_dir("jvm-project");
    let out = jvm(&["decompile-project", &jar(), "-o", out_dir.to_str().unwrap()]);
    let summary = ok(&out);
    let main = std::fs::read_to_string(out_dir.join("com/ex/Main.java")).unwrap();
    let helper = std::fs::read_to_string(out_dir.join("com/ex/util/Helper.java")).unwrap();
    assert_eq!(main, ok(&jvm(&["decompile", &jar(), "com.ex.Main"])));
    assert!(helper.starts_with("package com.ex.util;\n"), "{helper}");
    let expected = format!(
        "wrote {}: 2 .java files ({} bytes); classes: 2 ok, 0 failed; methods: 10 ok, 0 failed\n",
        out_dir.display(),
        main.len() + helper.len()
    );
    assert_eq!(summary, expected);
    let files: Vec<PathBuf> = walk(&out_dir);
    assert_eq!(files.len(), 2, "{files:?}");
}

#[test]
fn decompile_project_defaults_to_input_dot_kuna() {
    let dir = scratch_dir("jvm-project-default");
    let copy = dir.join("app.jar");
    std::fs::copy(jar(), &copy).unwrap();
    ok(&jvm(&["decompile-project", copy.to_str().unwrap()]));
    assert!(dir.join("app.jar.kuna/com/ex/Main.java").is_file());
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else {
            out.push(path);
        }
    }
    out.sort();
    out
}

#[test]
fn functions_json_is_the_inventory() {
    let doc = json(&jvm(&["functions", &jar(), "--json"]));
    assert_eq!(keys(&doc), ["binary", "engine", "count", "total", "error", "functions", "classes"]);
    assert_eq!(doc["count"], 12);
    assert_eq!(doc["total"], 12);
    let check = &doc["functions"][2];
    assert_eq!(
        keys(check),
        ["name", "address", "address_hex", "aliases", "object_location", "size", "class", "method", "descriptor", "access"]
    );
    assert_eq!(check["name"], "com.ex.Main.check");
    assert_eq!(check["size"], 22);
    let classes = doc["classes"].as_array().unwrap();
    assert_eq!(classes.len(), 4);
    assert_eq!(keys(&classes[0]), ["name", "outer", "source_file", "size", "access"]);
    let checker = classes.iter().find(|c| c["name"] == "com.ex.Main$Checker").unwrap();
    assert_eq!(checker["outer"], "com.ex.Main");
    assert_eq!(checker["access"], serde_json::json!(["static"]));
    let biggest = json(&jvm(&["functions", &jar(), "--json", "--sort", "size", "--limit", "1"]));
    assert_eq!(biggest["functions"][0]["name"], "com.ex.Main.main");
    assert_eq!(biggest["total"], 12);
    let filtered = ok(&jvm(&["functions", &jar(), "--filter", "Helper", "--min-size", "5"]));
    assert_eq!(filtered, "com.ex.util.Helper.<init>()V\ncom.ex.util.Helper.loop(I)I\n", "twice is 4 bytes of bytecode");
}

#[test]
fn jvm_options_resolve_by_name_and_alias() {
    let default = ok(&jvm(&["decompile", &jar(), "com.ex.util.Helper.loop"]));
    assert!(default.contains("total--;"), "{default}");
    for spelling in [["jvm_decompile_finally", "off"], ["decompile-finally", "0"], ["fdi", "false"]] {
        ok(&jvm(&["decompile", &jar(), "com.ex.Main", "--option", spelling[0], spelling[1]]));
    }
    let indented = ok(&jvm(&["decompile", &jar(), "com.ex.Main", "--option", "jvm_indent_string", "\t"]));
    assert!(indented.contains("\n\tpublic static boolean check(String key) {\n"), "{indented}");
}

#[test]
fn bad_options_and_native_flags_are_usage_errors() {
    let cases: &[(&[&str], &str)] = &[
        (&["--option", "loweredswitch", "off"], "a native-engine option"),
        (&["--option", "jvm_no_such_option", "on"], "unknown JVM option"),
        (&["--option", "jvm_decompile_finaly", "on"], "did you mean jvm_decompile_finally?"),
        (&["--option", "jvm_decompile_finally", "maybe"], "invalid value"),
        (&["--option", "thread-count", "4"], "set by kuna itself"),
        (&["--isa", "arm"], "--isa is a native-engine flag"),
        (&["--define-function", "0x10"], "native-engine flag"),
        (&["--mode", "bogus"], "unknown --mode"),
        (&["--language", "c"], "decompiles to Java"),
        (&["--no-such-flag"], "unknown option --no-such-flag"),
    ];
    let jar = jar();
    for (extra, message) in cases {
        let mut args = vec!["decompile", jar.as_str(), "com.ex.Main"];
        args.extend_from_slice(extra);
        let out = jvm(&args);
        assert_eq!(out.status.code(), Some(2), "{extra:?}: {}", stderr(&out));
        assert!(stderr(&out).contains(message), "{extra:?}: {}", stderr(&out));
    }
    ok(&jvm(&["decompile", &jar, "com.ex.Main", "--mode", "fast"]));
}

#[test]
fn a_jvm_option_on_a_native_input_points_at_the_jvm_catalog() {
    let elf = common::fixture("arraycoverwidth_x86_64");
    let out = kuna(&["decompile", &elf, "vm", "--option", "jvm_decompile_finally", "off"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("kuna catalog --engine jvm"), "{}", stderr(&out));
}

#[test]
fn a_named_jdk_that_is_missing_is_an_error() {
    let out = kuna(&["decompile", &jar(), "com.ex.Main", "--option", "jvm_include_runtime", "/no/such/jdk"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("--option jvm_include_runtime /no/such/jdk"), "{}", stderr(&out));
}

#[test]
fn the_other_subcommands_answer_with_capability_errors() {
    for (sub, what) in [
        ("strings", "string inventory"),
        ("xrefs", "cross-reference index"),
        ("disassemble", "bytecode listing"),
        ("read", "bytecode listing"),
        ("decompile-graph", "program graph"),
        ("crypto", "crypto-constant scan"),
        ("raw-pcode", "p-code"),
        ("unpack", "nothing to unpack"),
    ] {
        let out = kuna(&[sub, &jar()]);
        assert_eq!(out.status.code(), Some(1), "{sub}");
        let err = stderr(&out);
        assert!(err.contains("JVM input (jar)") && err.contains(what) && err.contains("kuna docs jvm"), "{sub}: {err}");
        assert!(out.stdout.is_empty(), "{sub}");
    }
    let out = kuna(&["functions", &jar(), "--summary"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("no call graph yet"), "{}", stderr(&out));
}

#[test]
fn dex_bytecode_is_refused_cleanly() {
    let dex = common::scratch_file("classes", "dex");
    std::fs::write(&dex, b"dex\n035\0\0\0\0\0\0\0\0\0").unwrap();
    let out = kuna(&["decompile-all", dex.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("DEX bytecode is not JVM bytecode"), "{}", stderr(&out));
}

#[test]
fn a_directory_without_classes_says_so() {
    let empty = scratch_dir("jvm-empty");
    let out = kuna(&["functions", empty.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("no .class files under"), "{}", stderr(&out));
}

#[test]
fn help_on_a_jvm_input_is_the_jvm_usage() {
    for sub in ["decompile", "decompile-all", "decompile-project", "functions"] {
        let out = kuna(&[sub, &jar(), "--help"]);
        assert_eq!(out.status.code(), Some(0));
        let err = stderr(&out);
        assert!(err.starts_with(&format!("usage: kuna {sub} <class|jar|dir>")), "{sub}: {err}");
    }
}

#[test]
fn catalog_lists_jvm_rows_on_request() {
    let native: Vec<Value> = serde_json::from_str(&ok(&kuna(&["catalog", "--json"]))).unwrap();
    assert!(native.iter().all(|r| r.get("engine").is_none()), "the default catalog is unchanged");
    let rows: Vec<Value> = serde_json::from_str(&ok(&kuna(&["catalog", "--engine", "jvm", "--json"]))).unwrap();
    assert!(!rows.is_empty());
    for row in &rows {
        assert_eq!(row["engine"], "jvm");
        assert!(row["option"].as_str().unwrap().starts_with("jvm_"));
        assert!(!row["symptoms"].as_array().unwrap().is_empty());
        assert!(["transform", "analysis", "core"].contains(&row["tier"].as_str().unwrap()));
    }
    let finally = rows.iter().find(|r| r["option"] == "jvm_decompile_finally").unwrap();
    assert_eq!(finally["aliases"], serde_json::json!(["decompile-finally", "fdi"]));
    assert_eq!(&keys(finally)[..3], ["option", "values", "default"]);
    let all: Vec<Value> = serde_json::from_str(&ok(&kuna(&["catalog", "--engine", "all", "--json"]))).unwrap();
    assert_eq!(all.len(), native.len() + rows.len());
    assert!(all[..native.len()].iter().all(|r| r["engine"] == "native"));
    let one: Vec<Value> = serde_json::from_str(&ok(&kuna(&["catalog", "--json", "--option", "fdi"]))).unwrap();
    assert_eq!(one.len(), 1);
    assert_eq!(one[0]["option"], "jvm_decompile_finally");
    let transforms: Vec<Value> =
        serde_json::from_str(&ok(&kuna(&["catalog", "--engine", "jvm", "--json", "--tier", "transform"]))).unwrap();
    assert!(!transforms.is_empty() && transforms.len() < rows.len());
    let text = ok(&kuna(&["catalog", "--option", "jvm_decompile_finally"]));
    assert!(text.contains("aliases: decompile-finally, fdi"), "{text}");
    let check = ok(&kuna(&["catalog", "--check"]));
    assert!(check.contains("JVM engine options"), "{check}");
    assert_eq!(kuna(&["catalog", "--engine", "bogus"]).status.code(), Some(2));
    assert_eq!(kuna(&["catalog", "--markdown", "--engine", "all"]).status.code(), Some(2));
    let miss = kuna(&["catalog", "--engine", "jvm", "--option", "loweredswitch"]);
    assert_eq!(miss.status.code(), Some(1));
}

#[test]
fn docs_embed_the_jvm_topics() {
    for topic in ["jvm", "options-jvm"] {
        let body = ok(&kuna(&["docs", topic]));
        let file = std::fs::read_to_string(common::repo_root().join(format!("docs/{topic}.md"))).unwrap();
        assert!(body == file, "stale embedded {topic}");
    }
}

#[test]
fn test_jvm_runs_corpus_cases_against_the_baseline() {
    let baseline = common::repo_root().join("tests/jvm/baseline.txt").display().to_string();
    let out = kuna(&["test", "--jvm", "--name", "default/pkg/TestEnhancedForLoops", "--baseline", &baseline]);
    let text = ok(&out);
    assert!(text.starts_with("jvm: 1/1 cases matched"), "{text}");
    assert!(text.trim_end().ends_with("PARITY OK"), "{text}");
    let doc: Value = serde_json::from_str(&ok(&kuna(&["test", "--jvm", "--name", "TestEnhancedForLoops", "--json"]))).unwrap();
    assert_eq!(doc["passing"], serde_json::json!(["jvm:default/pkg/TestEnhancedForLoops"]));
    assert_eq!(doc["jvm_footer"], serde_json::json!([1, 1]));
    let miss = kuna(&["test", "--jvm", "--name", "no-such-case"]);
    assert_eq!(miss.status.code(), Some(2));
    assert_eq!(kuna(&["test", "--jvm", "--datatests"]).status.code(), Some(2));
}

#[test]
fn a_reader_that_walks_away_is_not_a_panic() {
    let dir = common::repo_root().join("tests/jvm/classes/java8").display().to_string();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", &dir])
        .args(NO_JDK)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let out = child.wait_with_output().unwrap();
    assert!(!stderr(&out).contains("panicked"), "{}", stderr(&out));
    assert_eq!(out.status.code(), Some(0));
}
