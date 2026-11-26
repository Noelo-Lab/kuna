//! A `--json` function record lists the analysis warnings the engine wrote
//! into its `code` in `warnings`, each at the address the engine recorded it
//! at, while `error` stays `null` and the exit status `0` for a body that
//! decompiled, and the comments stay in `code`.

use crate::common;

use serde_json::{json, Value};

const RAW: [&str; 6] = ["--raw-image", "--target", "AARCH64:LE:64:v8A", "--base", "0x10000", "--json"];

fn document(args: &[&str]) -> Value {
    let (stdout, stderr, code) = common::run_kuna(args);
    assert_eq!(code, 0, "{args:?}: {stdout}\n{stderr}");
    serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

/// The record's warnings, each checked to sit on a line of `code` holding its
/// comment, which spells its message or, abbreviated, its kind.
fn placed(record: &Value) -> Vec<Value> {
    let code = record["code"].as_str().unwrap_or_default();
    let lines: Vec<&str> = code.lines().collect();
    let warnings = record["warnings"].as_array().unwrap().clone();
    for warning in &warnings {
        let line = lines[warning["line"].as_u64().unwrap() as usize - 1];
        let message = warning["message"].as_str().unwrap().lines().next().unwrap();
        assert!(line.contains("//") || line.contains("/*"), "{warning} on {line:?}");
        assert!(line.contains(message) || line.contains(warning["kind"].as_str().unwrap()), "{warning} on {line:?}");
    }
    warnings
}

/// `(kind, message, address)` of every warning, in order: what both warning
/// styles and every language agree on.
fn records(record: &Value) -> Vec<(String, String, Value)> {
    placed(record)
        .iter()
        .map(|w| (w["kind"].as_str().unwrap().into(), w["message"].as_str().unwrap().into(), w["address"].clone()))
        .collect()
}

/// The mnemonic and operands of the instruction at `address`.
fn instruction(image: &str, address: u64) -> (String, String) {
    let at = format!("0x{address:x}");
    let doc = document(&["disassemble", image, &at, "--addr", "--count", "1", "--json"]);
    let row = &doc["instructions"][0];
    assert_eq!(row["address"], address, "{doc}");
    (row["mnemonic"].as_str().unwrap().into(), row["operands"].as_str().unwrap().into())
}

/// A raw AArch64 image at 0x10000: `ret`, `nop`, a word that decodes to
/// nothing (0xffffffff at 0x10008), then `udf #0` (0x1000c), whose exception
/// vector is an indirect call.
#[test]
fn a_bad_instruction_is_a_warning_of_a_record_that_did_not_fail() {
    let path = common::scratch_file("function-warnings", "bin");
    let bytes = [0xc0, 0x03, 0x5f, 0xd6, 0x1f, 0x20, 0x03, 0xd5, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0];
    std::fs::write(&path, bytes).unwrap();
    let image = path.to_str().unwrap();
    let bad = json!([
        {"kind": "bad-instruction", "message": "Control flow encountered bad instruction data",
         "line": 1, "address": 0x10004},
        {"kind": "bad-instruction", "message": "Bad instruction - Truncating control flow here",
         "line": 3, "address": 0x10008},
    ]);

    let one = document(&[&["decompile", image, "0x10004"], &RAW[..]].concat());
    let record = &one["functions"][0];
    assert_eq!(one["error"], Value::Null);
    assert_eq!(record["error"], Value::Null);
    assert!(
        record["code"].as_str().unwrap().contains("halt_baddata(); // warn: Bad instruction - Truncating control flow here"),
        "{record}"
    );
    assert_eq!(placed(record), bad.as_array().unwrap().clone());

    let mut all = vec!["decompile-all", image];
    all.extend(RAW);
    all.extend(["--entry", "0x10000", "--entry", "0x10004", "--entry", "0x1000c"]);
    let doc = document(&all);
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 3, "{doc}");
    assert_eq!(functions[0]["warnings"], json!([]), "the clean `ret`: {}", functions[0]);
    assert_eq!(functions[1]["warnings"], bad);
    assert_eq!(
        functions[2]["warnings"],
        json!([{"kind": "jump-as-call", "message": "Treating indirect jump as call", "line": 3, "address": 0x1000c}])
    );

    let serial = document(&[&all[..], &["--option", "protoorder", "off"]].concat());
    assert_eq!(serial["functions"], doc["functions"], "the callee-first result store keeps them");

    all.extend(["--option", "warnstyle", "banner"]);
    let banner = document(&all);
    for (inline, banner) in functions.iter().zip(banner["functions"].as_array().unwrap()) {
        assert_eq!(records(inline), records(banner), "{banner}");
    }
    assert!(banner["functions"][1]["code"].as_str().unwrap().starts_with("/* WARNING: Control flow"));

    for language in ["go", "rust"] {
        for entry in ["0x10004", "0x1000c"] {
            let doc = document(&[&["decompile", image, entry, "--language", language], &RAW[..]].concat());
            let c = document(&[&["decompile", image, entry], &RAW[..]].concat());
            assert_eq!(records(&doc["functions"][0]), records(&c["functions"][0]), "{language} {entry}: {doc}");
        }
    }
    std::fs::remove_file(&path).unwrap();
}

/// `nop; nop; ret` at 0x10000, cut after its first instruction.
#[test]
fn a_flow_cut_by_the_budget_or_a_declared_extent_is_a_warning() {
    let path = common::scratch_file("function-warnings-nops", "bin");
    std::fs::write(&path, [0x1f, 0x20, 0x03, 0xd5, 0x1f, 0x20, 0x03, 0xd5, 0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    let image = path.to_str().unwrap();
    let cases: [(&[&str], [(&str, &str); 2]); 2] = [
        (
            &["--option", "maxinstruction", "1"],
            [("instruction-limit", "Exceeded the 1 instruction budget: "), ("instruction-limit", "Too many instructions -- ")],
        ),
        (
            &["--define-function", "0x10000-0x10004"],
            [("out-of-bounds", "Function flows out of bounds"), ("out-of-bounds", "Function flow out of bounds: ")],
        ),
    ];
    for (cut, expected) in cases {
        let doc = document(&[&["decompile", image, "0x10000"], &RAW[..], cut].concat());
        let record = &doc["functions"][0];
        assert_eq!((&doc["error"], &record["error"]), (&Value::Null, &Value::Null), "{record}");
        let warnings = placed(record);
        assert_eq!(warnings.len(), 2, "{record}");
        for ((warning, (kind, text)), (line, address)) in
            warnings.iter().zip(expected).zip([(1, 0x10000), (3, 0x10004)])
        {
            assert_eq!(warning["kind"], kind, "{record}");
            assert!(warning["message"].as_str().unwrap().starts_with(text), "{record}");
            assert_eq!((&warning["line"], &warning["address"]), (&json!(line), &json!(address)), "{record}");
        }
    }
    std::fs::remove_file(&path).unwrap();
}

/// Each function's body falls through into the next function at its last
/// instruction, which is where the drained `funcboundflow` warning is recorded.
/// The `sub esp,4` inside `register_tm_clones` is declared an entry: discovery
/// no longer plants one there.
#[test]
fn an_empty_body_s_drained_warning_is_a_body_warning_in_every_language() {
    let cases = [
        ("function_boundary_return_x86_64", "ordinary_first", 0x401016, vec![], vec![]),
        ("i386_pie_nl", "register_tm_clones", 0x18f0, vec![("injected", 0x18d0)], vec!["--define-function", "0x18f2"]),
    ];
    for (fixture, name, last, header, declare) in &cases {
        let image = common::fixture(fixture);
        let listing = document(&[&["disassemble", &image, name, "--json"][..], &declare[..]].concat());
        assert_eq!(listing["instructions"].as_array().unwrap().last().unwrap()["address"], *last, "{listing}");
        let mut expected: Vec<(String, Value)> = vec![("funcboundflow".into(), json!(last))];
        expected.extend(header.iter().map(|(kind, address)| (kind.to_string(), json!(address))));
        expected.sort_by_key(|(kind, _)| kind.clone());
        for language in ["c", "go", "rust"] {
            for style in ["inline", "banner"] {
                let args = ["decompile", &image, name, "--json", "--language", language];
                let doc = document(&[&args[..], &["--option", "warnstyle", style], &declare[..]].concat());
                let record = &doc["functions"][0];
                let mut found: Vec<(String, Value)> =
                    records(record).into_iter().map(|(kind, _, address)| (kind, address)).collect();
                found.sort_by_key(|(kind, _)| kind.clone());
                assert_eq!(found, expected, "{language} {style}: {record}");
            }
        }
    }

    let go = |fixture: &str, name: &str, declare: &[&str]| {
        let image = common::fixture(fixture);
        document(&[&["decompile", &image, name, "--json", "--language", "go"][..], declare].concat())["functions"][0].clone()
    };
    let alone = go("function_boundary_return_x86_64", "ordinary_first", &[]);
    assert!(alone["code"].as_str().unwrap().starts_with("func ordinary_first() { // warn: funcboundflow: "), "{alone}");
    let both = go("i386_pie_nl", "register_tm_clones", &["--define-function", "0x18f2"]);
    assert!(
        both["code"].as_str().unwrap().starts_with("func register_tm_clones() { // injected // warn: funcboundflow: "),
        "{both}"
    );
    let at: Vec<(Value, Value)> = placed(&both).iter().map(|w| (w["line"].clone(), w["address"].clone())).collect();
    assert_eq!(at, [(json!(1), json!(0x18d0)), (json!(1), json!(0x18f0))], "{both}");
}

/// `audit_undecodable` opens with `JZ 0x401033`, a branch into the middle of
/// the next instruction, whose bytes decode to nothing.
#[test]
fn a_storage_or_asserted_comment_beside_a_bad_instruction_is_not_a_warning() {
    let image = common::fixture("followoverlap_x86_64");
    assert_eq!(instruction(&image, 0x401030), ("JZ".into(), "0x401033".into()));
    let bad: Vec<(String, String, Value)> = [
        ("Control flow encountered bad instruction data", 0x401030),
        ("Bad instruction - Truncating control flow here", 0x401033),
    ]
    .map(|(message, address)| ("bad-instruction".into(), message.into(), json!(address)))
    .into();
    for style in ["inline", "banner"] {
        let doc = document(&[
            "decompile", &image, "audit_undecodable", "--json", "--option", "warnstyle", style,
            "--assert", "comment 0x401032 WARNING: hello there",
        ]);
        assert_eq!(doc["assertions"][0]["status"], "applied", "{doc}");
        let record = &doc["functions"][0];
        let code = record["code"].as_str().unwrap();
        assert!(code.contains("bool v1; // zf") && code.contains("/* WARNING: hello there */"), "{style}: {record}");
        assert_eq!(records(record), bad, "{style}: {record}");
    }
}

#[test]
fn a_function_s_original_name_comment_is_not_a_warning() {
    let image = common::fixture("adrnext_x86_64");
    for name in ["no-return", "inlined: lea_next", "warn: Bad instruction - Truncating control flow here"] {
        let define = format!("0x201120={name}");
        for language in ["rust", "go", "c"] {
            let args = ["decompile", &image, "0x201120", "--addr", "--define-function", &define, "--json"];
            let doc = document(&[&args[..], &["--language", language]].concat());
            let record = &doc["functions"][0];
            let code = record["code"].as_str().unwrap();
            assert_eq!(code.starts_with(&format!("// {name}\n")), language != "c", "{language}: {code}");
            assert_eq!(record["warnings"], json!([]), "{language}: {record}");
        }
    }
}

#[test]
fn both_warning_styles_report_the_same_warnings() {
    let pe = common::fixture("explicit_branch_assertion_pe_i386.exe");
    let mut args = vec!["decompile-all", &pe, "--json", "--omit", "line_mappings,variables,types"];
    args.extend(["--addr", "0x418764", "--addr", "0x406de0", "--addr", "0x44a0c6", "--option", "tailform", "off"]);
    let inline = document(&args);
    let workers = document(&[&args[..], &["--jobs", "2"]].concat());
    assert_eq!(workers["functions"], inline["functions"], "a --jobs worker's records carry them");
    args.extend(["--option", "warnstyle", "banner"]);
    let banner = document(&args);
    let mut seen = Vec::new();
    for (inline, banner) in inline["functions"].as_array().unwrap().iter().zip(banner["functions"].as_array().unwrap()) {
        assert_eq!(records(inline), records(banner), "{}", inline["name"]);
        seen.extend(records(inline).into_iter().map(|(kind, _, address)| (kind, address)));
    }
    for kind in ["bad-instruction", "int3-pad", "calltrampoline", "branch-flip", "funcboundflow"] {
        assert!(seen.iter().any(|(k, _)| k == kind), "{kind} missing from {seen:?}");
    }
    assert!(seen.contains(&("calltrampoline".into(), json!(0x406df0))), "{seen:?}");
    assert_eq!(instruction(&pe, 0x406df0), ("CALL".into(), "0x44a677".into()));
    assert!(seen.contains(&("branch-flip".into(), json!(0x406e9f))), "{seen:?}");
    assert_eq!(instruction(&pe, 0x406e9f).0, "JZ");
    let joined = placed(&inline["functions"][1]);
    let line49: Vec<&Value> = joined.iter().filter(|w| w["line"] == 49).map(|w| &w["address"]).collect();
    assert_eq!(line49, [&json!(0x406eba), &json!(0x406ed9)], "{joined:?}");

    let stripped = common::fixture("pe_imports_stripped.exe");
    let doc = document(&["decompile", &stripped, "0x140001b30", "--json"]);
    let message = "returndup: duplicated 5 shared bare-epilogue return(s) for early-return recovery in sub_140001b30";
    assert_eq!(
        placed(&doc["functions"][0])[0],
        json!({"kind": "return-dupe", "message": message, "line": 1, "address": 0x140001b30u64})
    );
}

/// `maxlinewidth` breaks plain text only: a `--json` record keeps one statement
/// per line, so its warnings and line mappings still address its code.
#[test]
fn a_line_width_leaves_json_records_on_their_lines() {
    let pe = common::fixture("explicit_branch_assertion_pe_i386.exe");
    let args = ["decompile-all", &pe, "--json", "--addr", "0x418764", "--addr", "0x406de0", "--addr", "0x44a0c6"];
    let plain = document(&args);
    let narrow = document(&[&args[..], &["--option", "maxlinewidth", "40"]].concat());
    assert_eq!(narrow["functions"], plain["functions"]);
    assert!(narrow["functions"].as_array().unwrap().iter().any(|f| !placed(f).is_empty()), "{narrow}");

    let (unbroken, _, code) = common::run_kuna(&["decompile", &pe, "0x406de0"]);
    assert_eq!(code, 0);
    let (broken, stderr, code) = common::run_kuna(&["decompile", &pe, "0x406de0", "--option", "maxlinewidth", "40"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(broken.lines().count() > unbroken.lines().count(), "{broken}");
    let words = |text: &str| text.split_whitespace().collect::<String>();
    assert_eq!(words(&broken), words(&unbroken));
}
