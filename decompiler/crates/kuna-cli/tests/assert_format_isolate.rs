//! `--assert format` and `--assert isolate` end to end, on both surfaces: the
//! text one (a console script) and the `--json` one (the engine in process).
//!
//! The fixture is `fixtures/assert_format_isolate/assert_format_isolate.c`,
//! built by the command in its first lines.

use crate::common;

fn fixture() -> String {
    common::repo_root()
        .join("decompiler/crates/kuna-cli/tests/fixtures/assert_format_isolate/assert_format_isolate.elf")
        .to_string_lossy()
        .into_owned()
}

/// Decompile `function` as text and as JSON with the given directives;
/// return each surface's C, its report (stderr, the JSON document) and exit
/// code.
fn both(function: &str, directives: &[&str], extra: &[&str]) -> [(String, String, i32); 2] {
    let binary = fixture();
    surfaces(&["decompile", binary.as_str(), function], directives, extra)
}

/// [`both`] for a command line that starts with `head`.
fn surfaces(head: &[&str], directives: &[&str], extra: &[&str]) -> [(String, String, i32); 2] {
    let sleigh = common::repo_root().join("specs").to_string_lossy().into_owned();
    let mut args = head.to_vec();
    args.extend(["--sleighpath", sleigh.as_str()]);
    for directive in directives {
        args.extend(["--assert", directive]);
    }
    args.extend_from_slice(extra);
    let text = common::run_kuna(&args);
    args.push("--json");
    let (doc, _, code) = common::run_kuna(&args);
    let parsed: serde_json::Value = serde_json::from_str(&doc).expect("valid decompile JSON");
    let c = parsed.pointer("/functions/0/code").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
    [text, (c, doc, code)]
}

#[test]
fn octal_prints_the_mask_and_the_mode() {
    for (c, report, code) in both("is_rwx", &["format is_rwx::0x1460 oct", "format 0x1468 oct"], &[]) {
        assert!(c.contains("return (a0 & 0777) == 0755;"), "{c}\n{report}");
        assert!(!report.contains("rejected"), "{report}");
        assert_eq!(code, 0);
    }
}

#[test]
fn decimal_prints_the_mask_and_leaves_the_other_constant() {
    for (c, report, _) in both("is_rwx", &["format 0x1460 dec"], &[]) {
        assert!(c.contains("return (a0 & 511) == 0x1ed;"), "{c}\n{report}");
    }
}

#[test]
fn an_address_that_reads_no_constant_is_rejected() {
    for (c, report, code) in both("is_rwx", &["format 0x146e oct"], &[]) {
        assert!(c.contains("return (a0 & 0x1ff) == 0x1ed;"), "{c}");
        assert!(report.contains("no constant is read at 0x146e"), "{report}");
        assert_eq!(code, 0);
    }
    for (_, _, code) in both("is_rwx", &["format 0x146e oct"], &["--assert-strict"]) {
        assert_eq!(code, 1);
    }
    for (c, report, _) in both("is_rwx", &["format 0x1471 oct"], &[]) {
        assert!(c.contains("return (a0 & 0x1ff) == 0x1ed;"), "{c}");
        assert!(report.contains("no constant is read at 0x1471"), "{report}");
    }
}

/// `int is_blue(int c) { return c == 7; }` at 0x1590 and
/// `long wide(long x) { return x == 0x100000041L; }` at 0x15a0, the same
/// compiler and flags as the fixture, as a raw x86-64 image.
fn raw_image(entry: &str, directives: &[&str]) -> [(String, String, i32); 2] {
    const BYTES: &str = "31c083ff070f94c0c30f1f800000000048b9410000000100000031c04839cf0f94c0c3";
    let binary = common::scratch_file("assert-format-raw", "bin");
    let bytes: Vec<u8> = (0..BYTES.len()).step_by(2).map(|i| u8::from_str_radix(&BYTES[i..i + 2], 16).unwrap()).collect();
    std::fs::write(&binary, bytes).unwrap();
    let binary = binary.to_string_lossy().into_owned();
    let head = ["decompile", binary.as_str(), entry, "--raw-image", "--base", "0x1590", "--target", "x86:LE:64:default:gcc"];
    surfaces(&head, directives, &[])
}

#[test]
fn a_constant_printed_as_an_enum_name_is_rejected() {
    let enumerated = ["function 0x1590=is_blue", "typedef enum color {RED=1,GREEN=2,BLUE=7};", "prototype is_blue int is_blue(color c)"];
    let mut directives = enumerated.to_vec();
    directives.push("format 0x1592 dec");
    for (c, report, _) in raw_image("0x1590", &directives) {
        assert!(c.contains("return c == BLUE;"), "{c}\n{report}");
        assert!(report.contains("the constant read at 0x1592 prints as a name of color, not as a number"), "{report}");
    }
}

#[test]
fn a_character_literal_does_not_take_a_wider_value() {
    let wide = ["function 0x15a0=wide", "prototype wide long wide(long x)"];
    let mut directives = wide.to_vec();
    directives.push("format 0x15ac char");
    for (c, report, _) in raw_image("0x15a0", &directives) {
        assert!(c.contains("x == 0x100000041"), "{c}\n{report}");
        assert!(report.contains("a character literal cannot hold 0x100000041"), "{report}");
    }
    directives.pop();
    directives.push("format 0x15ac dec");
    for (c, report, _) in raw_image("0x15a0", &directives) {
        assert!(c.contains("x == 4294967361"), "{c}\n{report}");
    }
}

#[test]
fn a_value_picks_one_of_several_constants() {
    for (c, report, _) in both("scale", &["format 0x1480 oct"], &[]) {
        assert!(c.contains("return a0 * 8 + 7;"), "{c}");
        assert!(report.contains("reads several constants (0x7, 0x8); name one by its value"), "{report}");
    }
    for (c, report, _) in both("scale", &["format 0x1480 oct 7", "format 0x1480 hex 0x8"], &[]) {
        assert!(c.contains("return a0 * 0x8 + 07;"), "{c}\n{report}");
    }
    for (_, report, _) in both("scale", &["format 0x1480 oct 9"], &[]) {
        assert!(report.contains("no constant read at 0x1480 holds that value (it reads 0x7, 0x8)"), "{report}");
    }
}

#[test]
fn a_file_of_directives_formats_and_isolates() {
    let path = common::scratch_file("assert-format-isolate", "kuna");
    std::fs::write(&path, "# the multiplier, in octal\nformat reuse::0x14a6 oct\nisolate reuse::v1 # two variables\n").unwrap();
    let file = format!("@{}", path.display());
    for (c, report, _) in both("reuse", &[file.as_str()], &[]) {
        assert!(c.contains("v2 = v1 * 03 + 1;"), "{c}\n{report}");
    }
}

#[test]
fn an_isolated_local_is_not_merged_with_its_successor() {
    for (c, _, _) in both("reuse", &[], &[]) {
        assert!(c.contains("v1 = v1 * 3 + 1;"), "{c}");
    }
    for (c, report, _) in both("reuse", &["isolate v1"], &[]) {
        assert!(c.contains("v2 = v1 * 3 + 1;") && c.contains("k(v2);"), "{c}\n{report}");
        assert!(!report.contains("rejected"), "{report}");
    }
    for (c, report, _) in both("reuse", &["name v1 first", "isolate first"], &[]) {
        assert!(c.contains("v2 = first * 3 + 1;"), "{c}\n{report}");
    }
}

#[test]
fn an_isolated_parameter_is_not_reused_for_a_later_value() {
    for (c, _, _) in both("triple", &[], &[]) {
        assert!(c.contains("a0 *= 3;"), "{c}");
    }
    for (c, report, _) in both("triple", &["isolate a0"], &[]) {
        assert!(c.contains("v2 = a0 * 3;") && c.contains("return v2;"), "{c}\n{report}");
    }
}

#[test]
fn a_name_no_local_has_and_another_function_are_rejected() {
    for (_, report, _) in both("reuse", &["isolate v9"], &[]) {
        assert!(report.contains("No symbol named: v9"), "{report}");
    }
    for (_, report, _) in both("reuse", &["isolate triple::a0", "format is_rwx::0x1460 oct"], &[]) {
        assert_eq!(report.matches("rejected").count(), 2, "{report}");
    }
}
