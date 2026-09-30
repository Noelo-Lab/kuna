//! A veneer that jumps through a literal to another function is a tail call
//! when the callee's prototype is declared or the veneer's exact extent leaves
//! the callee outside it (GH-781).  A callee with no prototype inside an open
//! extent keeps its copied-in body, and a target that is not a function keeps
//! the out-of-bounds halt.

#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;
mod common;

const HELPER: [u32; 2] = [0xe2800001, 0xe12fff1e];

fn words(code: &[u32]) -> Vec<u8> {
    code.iter().flat_map(|w| w.to_le_bytes()).collect()
}

fn ldr_bx_veneer() -> Vec<u8> {
    words(&[0xe59f1000, 0xe12fff11, 0x10010, 0, HELPER[0], HELPER[1]])
}

fn decompile(code: &[u8], isa: &str, asserts: &[&str], extra: &[&str]) -> String {
    let path = common::scratch_file("arm-veneer-tailcall", "elf");
    std::fs::write(&path, arm_images::elf(code, &[], &[])).unwrap();
    let specs = common::repo_root().join("specs");
    let mut args = vec![
        "decompile",
        path.to_str().unwrap(),
        "0x10000",
        "--mode",
        "reliable",
        "--isa",
        isa,
        "--assert-strict",
        "--json",
        "--sleighpath",
        specs.to_str().unwrap(),
    ];
    for assertion in asserts {
        args.extend(["--assert", assertion]);
    }
    args.extend_from_slice(extra);
    let (stdout, stderr, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{stdout}\n{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let function = &doc["functions"][0];
    assert!(function["error"].is_null(), "{stdout}");
    function["code"].as_str().unwrap().to_owned()
}

fn assert_tail_call(c: &str, callee: &str) {
    assert!(
        c.contains(&format!("return {callee}(value);")),
        "no tail call to {callee}:\n{c}"
    );
    assert!(
        !c.contains("halt_missing"),
        "the transfer became a halt:\n{c}"
    );
    assert!(
        !c.contains("switch"),
        "the single target is still a switch:\n{c}"
    );
}

const PROTOS: [&str; 2] = [
    "prototype 0x10000 unsigned int veneer(unsigned int value)",
    "prototype 0x10010 unsigned int arm_helper(unsigned int value)",
];

#[test]
fn exact_bound_arm_veneer_tail_calls_the_declared_helper() {
    for bound in ["function 0x10000-0x10008=veneer", "function 0x10000=veneer"] {
        let c = decompile(
            &ldr_bx_veneer(),
            "arm",
            &[
                bound,
                "function 0x10010-0x10018=arm_helper",
                "readonly 0x10008+4",
                PROTOS[0],
                PROTOS[1],
            ],
            &[],
        );
        assert_tail_call(&c, "arm_helper");
    }
}

#[test]
fn exact_bound_ldr_pc_veneer_tail_calls_the_declared_helper() {
    let image = words(&[0xe51ff004, 0x10010, 0, 0, HELPER[0], HELPER[1]]);
    let c = decompile(
        &image,
        "arm",
        &[
            "function 0x10000-0x10004=veneer",
            "function 0x10010-0x10018=arm_helper",
            "readonly 0x10004+4",
            PROTOS[0],
            PROTOS[1],
        ],
        &[],
    );
    assert_tail_call(&c, "arm_helper");
}

#[test]
fn exact_bound_thumb_veneer_tail_calls_the_declared_helper() {
    let image: Vec<u8> = [0x4900u16, 0x4708]
        .iter()
        .flat_map(|h| h.to_le_bytes())
        .chain(0x10009u32.to_le_bytes())
        .chain([0x3001u16, 0x4770].iter().flat_map(|h| h.to_le_bytes()))
        .collect();
    let c = decompile(
        &image,
        "thumb",
        &[
            "function 0x10000-0x10004=veneer",
            "function 0x10008-0x1000c=thumb_helper",
            "readonly 0x10004+4",
            PROTOS[0],
            "prototype 0x10008 unsigned int thumb_helper(unsigned int value)",
        ],
        &[],
    );
    assert_tail_call(&c, "thumb_helper");
}

#[test]
fn target_that_is_not_a_function_keeps_the_out_of_bounds_halt() {
    let c = decompile(
        &ldr_bx_veneer(),
        "arm",
        &[
            "function 0x10000-0x10008=veneer",
            "readonly 0x10008+4",
            PROTOS[0],
        ],
        &[],
    );
    assert!(c.contains("halt_missing"), "{c}");
    assert!(c.contains("flows to r0x00010010"), "{c}");
    assert!(!c.contains("// tail-call"), "{c}");
}

#[test]
fn tailcalljump_direct_restores_the_flow_into_the_target() {
    let c = decompile(
        &ldr_bx_veneer(),
        "arm",
        &[
            "function 0x10000-0x10008=veneer",
            "function 0x10010-0x10018=arm_helper",
            "readonly 0x10008+4",
            PROTOS[0],
            PROTOS[1],
        ],
        &["--option", "tailcalljump", "direct"],
    );
    assert!(c.contains("halt_missing"), "{c}");
    assert!(!c.contains("arm_helper("), "{c}");
}

#[test]
fn bounded_veneer_without_prototypes_does_not_pass_its_target_register() {
    let c = decompile(
        &ldr_bx_veneer(),
        "arm",
        &[
            "function 0x10000-0x10008=veneer",
            "function 0x10010-0x10018=arm_helper",
            "readonly 0x10008+4",
        ],
        &[],
    );
    assert!(c.contains("arm_helper(a0); // tail-call"), "{c}");
    assert!(!c.contains("halt_missing"), "{c}");
}

#[test]
fn open_veneer_to_a_helper_without_a_prototype_keeps_its_body() {
    let c = decompile(
        &ldr_bx_veneer(),
        "arm",
        &[
            "function 0x10000=veneer",
            "function 0x10010-0x10018=arm_helper",
            "readonly 0x10008+4",
        ],
        &[],
    );
    assert!(c.contains("return a0 + 1;"), "{c}");
    assert!(!c.contains("// tail-call"), "{c}");
}
