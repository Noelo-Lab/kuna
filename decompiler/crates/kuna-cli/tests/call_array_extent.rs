//! Call-escaped frame storage keeps the helper's output and its later reads together.

use crate::common;
use common::process;
use std::process::Command;

fn decompile(entry: &str, name: &str, option: &str, local_type: Option<&str>) -> String {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-call-array-extent.xml"
    ));
    let chunk =
        regex::Regex::new(r#"<bytechunk[^>]*offset="(0x[0-9a-f]+)"[^>]*>([0-9a-f]+)</bytechunk>"#)
            .unwrap();
    let mut bytes = Vec::new();
    for captures in chunk.captures_iter(xml) {
        let offset = usize::from_str_radix(&captures[1][2..], 16).unwrap() - 0x1000;
        let hex = &captures[2];
        bytes.resize(offset, 0);
        bytes.extend(
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()),
        );
    }
    let binary = common::scratch_file("call-array-extent", "bin");
    std::fs::write(&binary, bytes).unwrap();
    let function = format!("function {entry}={name}");
    let proto = format!("prototype {name} unsigned int {name}(unsigned int seed)");
    let mut args = vec![
        "decompile",
        binary.to_str().unwrap(),
        entry,
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        "sparc:BE:32:default:default",
        "--assert",
        &function,
        "--assert",
        &proto,
        "--assert",
        "function 0x1020=fill_words",
        "--assert",
        "prototype fill_words void fill_words(unsigned int *out,unsigned int seed)",
        "--assert",
        "function 0x1074=touch_word",
        "--assert",
        "prototype touch_word void touch_word(unsigned int *out)",
        "--assert",
        "function 0x1154=observe_words",
        "--assert",
        "prototype observe_words unsigned int observe_words(unsigned int *in)",
        "--assert",
        "function 0x1180=copy_words",
        "--assert",
        "prototype copy_words void copy_words(unsigned int *in,unsigned int *out)",
        "--assert",
        "function 0x1300=lower_scalar_output",
        "--assert",
        "function 0x1380=parent_tail_scalar",
        "--assert",
        "function 0x1400=duplicate_arg_output",
        "--assert",
        "function 0x1440=mixed_words",
        "--assert",
        "prototype mixed_words void mixed_words(unsigned int *out,unsigned long long *other)",
        "--option",
        "callarrayextent",
        option,
        "--assert-strict",
        "--json",
    ];
    if let Some(local_type) = local_type {
        args.extend(["--assert", local_type]);
    }
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}");
    let result: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(result["functions"][0]["error"].is_null(), "{stdout}");
    result["functions"][0]["code"].as_str().unwrap().to_owned()
}

#[test]
fn call_array_extent_is_opt_in_and_respects_declared_storage() {
    let off = decompile("0x1000", "escaped_output", "off", None);
    assert!(off.contains("return v2;"), "{off}");
    assert!(off.contains("v1 [2]"), "{off}");
    let on = decompile("0x1000", "escaped_output", "on", None);
    assert!(
        regex::Regex::new(r"return v[0-9]+\[2\];")
            .unwrap()
            .is_match(&on),
        "{on}"
    );
    let declared = decompile(
        "0x1000",
        "escaped_output",
        "on",
        Some("type escaped_output::v1 unsigned int[10]"),
    );
    assert!(declared.contains("v1 [10]"), "{declared}");
    assert!(declared.contains("return v1[2];"), "{declared}");
    let bounded = decompile("0x1048", "bounded_output", "on", None);
    assert!(
        regex::Regex::new(r"v[0-9]+ \[10\]")
            .unwrap()
            .is_match(&bounded),
        "{bounded}"
    );
    assert!(
        regex::Regex::new(r"return v[0-9]+\[2\];")
            .unwrap()
            .is_match(&bounded),
        "{bounded}"
    );
    let separate = decompile("0x1080", "separate_word", "on", None);
    assert!(separate.contains("v1 [2]"), "{separate}");
    assert!(separate.contains("v2 = seed;"), "{separate}");
    assert!(separate.contains("return v2;"), "{separate}");
}

#[test]
fn emitted_call_output_matches_the_stored_word() {
    for (entry, name) in [("0x1000", "escaped_output"), ("0x1048", "bounded_output")] {
        let code = decompile(entry, name, "on", None);
        let program = format!(
            r#"
void fill_words(unsigned int *out, unsigned int seed) {{
    for (unsigned int i = 0; i < 10; ++i) out[i] = seed + i;
}}
void touch_word(unsigned int *out) {{ *out = 0; }}
{code}
int main(void) {{
    unsigned int seeds[] = {{0, 1, 2047, 2048, 0xffffffffu, 0x80000000u}};
    for (unsigned int i = 0; i < sizeof(seeds) / sizeof(seeds[0]); ++i)
        if ({name}(seeds[i]) != seeds[i] + 2u) return 1;
    return 0;
}}
"#
        );
        let source = common::scratch_file("call-array-extent", "c");
        std::fs::write(&source, program).unwrap();
        for compiler in ["gcc", "clang"] {
            if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            for optimization in ["-O0", "-O2"] {
                let executable = common::scratch_file("call-array-extent", "exe");
                let compiled = Command::new(compiler)
                    .args([optimization, "-std=c11"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    compiled.status.success(),
                    "{compiler} {optimization}: {}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let run = Command::new(&executable).output().unwrap();
                assert!(
                    run.status.success(),
                    "{name}, {compiler} {optimization}: {}",
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
}

#[test]
fn overlapping_output_preserves_the_initializer_and_call_result() {
    for (entry, name, off_words, factor, addend) in [("0x1100", "overlapping_output", 11, 11, 2)] {
        let off = decompile(entry, name, "off", None);
        assert!(
            regex::Regex::new(&format!(r"v[0-9]+ \[{off_words}\]"))
                .unwrap()
                .is_match(&off),
            "{off}"
        );
        let code = decompile(entry, name, "on", None);
        assert!(
            regex::Regex::new(r"v[0-9]+ \[23\]")
                .unwrap()
                .is_match(&code),
            "{code}"
        );
        let program = format!(
            r#"
unsigned int observe_words(unsigned int *in) {{
    unsigned int sum = 0;
    for (unsigned int i = 0; i < 10; ++i) sum += in[i];
    return sum;
}}
void copy_words(unsigned int *in, unsigned int *out) {{
    for (unsigned int i = 0; i < 10; ++i) out[i] = in[i] + i;
}}
{code}
int main(void) {{
    unsigned int seeds[] = {{0, 1, 2047, 2048, 0xffffffffu, 0x80000000u}};
    for (unsigned int i = 0; i < sizeof(seeds) / sizeof(seeds[0]); ++i)
        if ({name}(seeds[i]) != {factor}u * seeds[i] + {addend}u) return 1;
    return 0;
}}
"#
        );
        let source = common::scratch_file("overlapping-call-output", "c");
        std::fs::write(&source, program).unwrap();
        for compiler in ["gcc", "clang"] {
            if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            for optimization in ["-O0", "-O2"] {
                let executable = common::scratch_file("overlapping-call-output", "exe");
                let compiled = Command::new(compiler)
                    .args([optimization, "-std=c11", "-fsanitize=address"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    compiled.status.success(),
                    "{}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let run = Command::new(&executable)
                    .env("ASAN_OPTIONS", "detect_leaks=0")
                    .output()
                    .unwrap();
                assert!(
                    run.status.success(),
                    "{compiler} {optimization}: {}",
                    String::from_utf8_lossy(&run.stderr)
                );
            }
        }
    }
}

#[test]
fn loop_carried_pointer_declines_the_direct_address_transform() {
    let off = decompile("0x1200", "looping_output", "off", None);
    let on = decompile("0x1200", "looping_output", "on", None);
    assert!(off.contains("v2 [13]"), "{off}");
    assert_eq!(on, off);
}

#[test]
fn adjacent_caller_values_remain_separate() {
    let off = decompile("0x1300", "lower_scalar_output", "off", None);
    let on = decompile("0x1300", "lower_scalar_output", "on", None);
    assert_ne!(on, off);
    assert!(on.contains("stack - 0x34"), "{on}");
    assert!(
        regex::Regex::new(r"v[0-9]+ \[12\]").unwrap().is_match(&on),
        "{on}"
    );
    assert!(
        regex::Regex::new(r"return v[0-9]+ \+ v[0-9]+\[2\];")
            .unwrap()
            .is_match(&on),
        "{on}"
    );

    for (entry, name) in [
        ("0x1380", "parent_tail_scalar"),
        ("0x1400", "duplicate_arg_output"),
    ] {
        let off = decompile(entry, name, "off", None);
        let on = decompile(entry, name, "on", None);
        assert_eq!(on, off, "{name}");
    }
}
