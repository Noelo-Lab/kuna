//! A real caller can make an isolated raw Thumb wrapper return its callee's result.

use crate::common;
use serde_json::Value;

const RAW: [&str; 9] = [
    "--raw-image", "--base", "0x6100", "--target", "ARM:LE:32:v7:default", "--isa", "thumb",
    "--json", "--no-vars",
];
const WRAPPER: &str = "0x6110";
const FORWARD: &[u16] = &[0xb510, 0xf000, 0xf815, 0xbf00, 0xbd10]; // push; bl provider; nop; pop

const PROVIDER: &[u16] = &[
    0xb510, 0x2900, 0xd002, // push; cmp r1,#0; beq constant path
    0xf000, 0xf80b, 0xe002, // bl increment; b shared epilogue
    0xf000, 0xf810, 0x2011, 0xbd10, // bl store; movs r0,#17; pop {r4,pc}
];

fn image(wrapper: &[u16], caller: &[u16]) -> tempfile::TempPath {
    image_with_provider(wrapper, caller, PROVIDER)
}

fn image_with_provider(wrapper: &[u16], caller: &[u16], provider: &[u16]) -> tempfile::TempPath {
    let mut words = [0xbf00u16; 0x3a]; // nop padding
    let mut put = |at, code: &[u16]| words[at..at + code.len()].copy_from_slice(code);
    put(0x00, &[0xb510, 0xf000, 0xf80d, 0xbd10]); // push; bl reader; pop
    put(0x08, wrapper);
    put(0x10, caller);
    put(0x20, provider);
    put(0x30, &[0x3003, 0x4770]); // increment: adds r0,#3; bx lr
    put(0x38, &[0x6001, 0x4770]); // store: str r1,[r0]; bx lr
    let bytes: Vec<_> = words.into_iter().flat_map(u16::to_le_bytes).collect();
    let file = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(file.path(), bytes).unwrap();
    file.into_temp_path()
}

// At 0x6120: push; bl wrapper; cmp r0,#7; beq; return 31 or 37.
const READER: &[u16] = &[0xb510, 0xf7ff, 0xfff5, 0x2807, 0xd001, 0x201f, 0xbd10, 0x2025, 0xbd10];

fn run(file: &std::path::Path, command: &str, args: &[&str]) -> Value {
    let args = [&[command, file.to_str().unwrap()][..], args, &RAW].concat();
    let (out, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}\n{out}");
    serde_json::from_str(&out).unwrap_or_else(|e| panic!("{e}: {out}"))
}

fn function_at(doc: &Value, address: u64) -> &Value {
    let function = doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["address"] == address)
        .unwrap_or_else(|| panic!("no function at {address:#x}: {doc}"));
    assert!(function["error"].is_null(), "{function}");
    function
}

fn wrapper(doc: &Value) -> &str {
    function_at(doc, 0x6110)["code"].as_str().unwrap()
}

#[test]
fn consumed_thumb_call_result_reaches_the_return_in_single_and_batch() {
    let file = image(FORWARD, READER);
    let single = run(&file, "decompile", &[WRAPPER]);
    assert_eq!(single["functions"].as_array().unwrap().len(), 1, "{single}");
    let batch = run(&file, "decompile-all", &["--entry", "0x6100"]);
    let narrowed = run(&file, "decompile-all", &["--addr", WRAPPER]);
    for doc in [&single, &batch, &narrowed] {
        let text = wrapper(doc);
        assert!(text.contains("return sub_6140(") && !text.contains("void sub_6110("), "{text}");
    }
}

#[test]
fn thumb_call_result_inference_requires_the_result_to_reach_the_read() {
    for caller in [
        // No read; overwrite before the read; another call before the read.
        &[0xb510, 0xf7ff, 0xfff5, 0xbd10][..],
        &[0xb510, 0xf7ff, 0xfff5, 0x2006, 0x2807, 0xd001, 0x201f, 0xbd10, 0x2025, 0xbd10],
        &[0xb510, 0xf7ff, 0xfff5, 0xf000, 0xf823, 0x2807, 0xd001, 0x201f, 0xbd10, 0x2025, 0xbd10],
    ] {
        let doc = run(&image(FORWARD, caller), "decompile", &[WRAPPER]);
        let text = wrapper(&doc);
        assert!(text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");
    }

    // The wrapper itself overwrites r0 with 43 after the call.
    let file = image(&[0xb510, 0xf000, 0xf815, 0x202b, 0xbd10], READER);
    let doc = run(&file, "decompile", &[WRAPPER]);
    let text = wrapper(&doc);
    assert!(text.contains("sub_6140(") && !text.contains("return sub_6140("), "{text}");
    assert!(text.contains("return 0x2b;") || text.contains("return 43;"), "{text}");
}

#[test]
fn a_predicated_call_cannot_borrow_a_read_from_its_skipped_path() {
    // r4 is preserved by the call. Only its nonzero path reads r0, but BLEQ
    // takes place only when r4 is zero.
    let mut reader = [
        0xb510, 0x2c00, 0xbf08, 0xf7ff, 0xfff3, // push; cmp r4,#0; it eq; bleq wrapper
        0x2c00, 0xd003, 0x2807, 0xd003, // cmp r4,#0; beq .zero; cmp r0,#7; beq .equal
        0x201f, 0xbd10, 0x2025, 0xbd10, 0x2029, 0xbd10, // return 31, 37 (.zero), or 41 (.equal)
    ];
    let doc = run(&image(FORWARD, &reader), "decompile-all", &["--entry", "0x6100"]);
    let text = wrapper(&doc);
    assert!(text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");

    // Remove only the IT instruction. A real path from the now-unconditional
    // call to the same r0 read supplies evidence.
    reader[2] = 0xbf00;
    let doc = run(&image(FORWARD, &reader), "decompile-all", &["--entry", "0x6100"]);
    let text = wrapper(&doc);
    assert!(text.contains("return sub_6140(") && !text.contains("void sub_6110("), "{text}");
}

#[test]
fn caller_read_must_fit_inside_the_callers_declared_extent() {
    let reader = [0xb510, 0xf7ff, 0xfff5, 0xf1b0, 0x0f07, 0xd001, 0x201f, 0xbd10, 0x2025, 0xbd10];
    let file = image(FORWARD, &reader);
    for end in ["0x6126", "0x6128", "0x612a"] {
        let bound = format!("0x6120-{end}");
        let doc = run(&file, "decompile", &[WRAPPER, "--define-function", &bound]);
        let text = wrapper(&doc);
        // The cmp.w at 0x6126 occupies four bytes; it must fit in the bound.
        assert_eq!(text.contains("return sub_6140("), end == "0x612a", "{bound}: {text}");
    }
}

#[test]
fn caller_bound_follows_control_flow_past_an_intervening_entry() {
    // Caller at 0x6120 branches around a separate bx lr at 0x6124.
    // Only that caller reaches the BL at 0x6128 and cmp at 0x612c.
    let reader = [0xe002, 0xbf00, 0x4770, 0xbf00, 0xf7ff, 0xfff2, 0x2807, 0xd001, 0x201f, 0x4770, 0x2025, 0x4770];
    let file = image(FORWARD, &reader);
    for end in ["0x612c", "0x612e"] {
        let bound = format!("0x6120-{end}");
        let doc = run(&file, "decompile-all", &["--addr", WRAPPER, "--entry", "0x6124", "--define-function", &bound]);
        let text = wrapper(&doc);
        assert_eq!(text.contains("return sub_6140("), end == "0x612e", "{bound}: {text}");
    }
}

#[test]
fn a_nested_return_proof_obeys_the_callees_declared_extent() {
    // At 0x6140: b 0x6144; nop; movs r0,#37; bx lr.
    let file = image_with_provider(FORWARD, READER, &[0xe000, 0xbf00, 0x2025, 0x4770]);
    let doc = run(&file, "decompile", &[WRAPPER, "--define-function", "0x6140-0x6148"]);
    let text = wrapper(&doc);
    assert!(text.contains("return sub_6140(") && !text.contains("void sub_6110("), "{text}");

    let bound = ["--define-function", "0x6140-0x6142"];
    let single = run(&file, "decompile", &[&[WRAPPER][..], &bound].concat());
    let batch = run(&file, "decompile-all", &[&["--addr", WRAPPER][..], &bound].concat());
    for doc in [&single, &batch] {
        let text = wrapper(doc);
        assert!(text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");
    }
    let direct = run(&file, "decompile", &[&["0x6140"][..], &bound].concat());
    let callee = function_at(&direct, 0x6140);
    assert!(callee["warnings"].as_array().unwrap().iter().any(|w| w["kind"] == "out-of-bounds"), "{callee}");
}

#[test]
fn a_nested_return_proof_declines_unreplayed_callee_directives() {
    let file = image(FORWARD, READER);
    let doc = run(
        &file,
        "decompile",
        &[WRAPPER, "--assert", "callproto sub_6140::0x6146 void increment(void)"],
    );
    let text = wrapper(&doc);
    assert!(text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");
}

#[test]
fn an_unresolved_thumb_jump_is_not_a_value_return() {
    // The provider has a computed arm and an unresolved BX arm, so the
    // outer wrapper cannot use its isolated result as proof.
    let file = image_with_provider(FORWARD, READER, &[0x2900, 0xd002, 0x4710, 0xbf00, 0xbf00, 0x2011, 0x4770]);
    let doc = run(&file, "decompile", &[WRAPPER]);
    let text = wrapper(&doc);
    let provider = run(&file, "decompile", &["0x6140"]);
    let record = function_at(&provider, 0x6140);
    assert!(text.contains("void sub_6110("), "{text}");
    assert!(record["warnings"].as_array().unwrap().iter().any(|w| {
        w["kind"] == "jump-as-call" && w["address"] == 0x6144
    }), "{record}");

    // The same transfer explicitly resolved to a function with a declared
    // value is a valid tail call, not a failed jump-table conversion.
    let file = image(&[0x2900, 0xd002, 0x4710, 0xbf00, 0xbf00, 0x2011, 0x4770], READER);
    let resolved = run(
        &file,
        "decompile",
        &[
            WRAPPER, "--assert-strict",
            "--define-function", "0x6140=sub_6140",
            "--assert", "prototype 0x6140 int sub_6140(int)",
            "--assert", "callee 0x6114 0x6140",
        ],
    );
    let text = wrapper(&resolved);
    assert!(!text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");

    // A BLX with a genuine continuation and normal epilogue is not a
    // synthetic tail transfer either.
    let indirect_call = image(
        &[0xb510, 0x2900, 0xd001, 0x4790, 0xbd10, 0x2011, 0xbd10],
        READER,
    );
    let doc = run(&indirect_call, "decompile", &[WRAPPER]);
    let text = wrapper(&doc);
    assert!(!text.contains("void sub_6110("), "{text}");
}

#[test]
fn thumb_call_result_inference_does_not_promote_void_or_override_a_prototype() {
    for code in [
        // bl store; pop
        &[0xb510, 0xf000, 0xf82d, 0xbf00, 0xbd10][..],
        // bl provider; bl store; pop
        &[0xb510, 0xf000, 0xf815, 0xf000, 0xf82b, 0xbd10],
    ] {
        let doc = run(&image(code, READER), "decompile", &[WRAPPER]);
        let text = wrapper(&doc);
        assert!(text.contains("void sub_6110(") && text.contains("sub_6170("), "{text}");
    }

    let file = image(FORWARD, READER);
    for extra in [
        &["--option", "protoorder", "off"][..],
        &["--assert-strict", "--assert", "prototype 0x6110 void sub_6110(void)"][..],
        &["--assert-strict", "--assert", "prototype 0x6140 void sub_6140(void)"][..],
    ] {
        let doc = run(&file, "decompile", &[&[WRAPPER][..], extra].concat());
        let text = wrapper(&doc);
        assert!(text.contains("void sub_6110(") && text.contains("sub_6140("), "{text}");
    }
}
