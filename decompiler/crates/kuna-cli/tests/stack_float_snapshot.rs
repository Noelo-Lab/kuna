//! Cached integer snapshots keep float bit patterns at stack-view float uses.
mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const FUNCTIONS: &str = "flip_first_and_add,flip_second_and_add,integer_to_float";
const ASSERTIONS: &[&str] = &[
    "typedef struct FloatPair { float x; float y; };",
    "prototype copy_point void copy_point(FloatPair *destination, const FloatPair *source)",
    "prototype observe_word uint4 observe_word(const uint4 *source)",
    "prototype flip_first_and_add float4 flip_first_and_add(const FloatPair *source, float4 bias, int4 flip)",
    "prototype flip_second_and_add float4 flip_second_and_add(const FloatPair *source, float4 bias, int4 flip)",
    "prototype integer_to_float float4 integer_to_float(int4 value)",
];

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn required(command: &mut Command) {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("cannot run {command:?}: {error}"));
    assert!(
        output.status.success(),
        "{command:?} failed ({}):\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn build_image(compiler: &str) -> PathBuf {
    let image = common::scratch_file("stack-float-snapshot", "elf");
    required(
        Command::new(compiler)
            .args([
                "-nostdlib",
                "-no-pie",
                "-Wl,-e,flip_first_and_add",
                "-Wl,-Ttext=0x401000",
            ])
            .arg(fixture("stack_float_snapshot.S"))
            .arg("-o")
            .arg(&image),
    );
    image
}

fn decompile_all(image: &Path, stackviews: bool) -> serde_json::Value {
    let root = common::repo_root();
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile-all",
            image.to_str().unwrap(),
            "--functions",
            FUNCTIONS,
            "--mode",
            "reliable",
            "--json",
            "--assert-strict",
            "--option",
            "stackviews",
            if stackviews { "on" } else { "off" },
            "--option",
            "stackalias",
            "off",
            "--option",
            "structdefs",
            "on",
            "--sleighpath",
        ])
        .arg(specs);
    for assertion in ASSERTIONS {
        command.args(["--assert", assertion]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(50),
    )
    .expect("bounded stack-float decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let rejected: Vec<_> = document["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|assertion| assertion["status"] != "applied")
        .map(|assertion| assertion["detail"].as_str().unwrap_or("no detail"))
        .collect();
    assert!(rejected.is_empty(), "{rejected:?}");
    document
}

fn code(document: &serde_json::Value, name: &str) -> String {
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"] == name)
        .unwrap_or_else(|| panic!("missing {name}: {document}"))["code"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn emitted_declarations(code: &str, name: &str) -> Vec<String> {
    let end = emitted_function_start(code, name);
    let declarations = &code[..end];
    let mut start = 0;
    let mut depth = 0;
    let mut result = Vec::new();
    for (index, character) in declarations.char_indices() {
        match character {
            '{' => depth += 1,
            '}' => depth -= 1,
            ';' if depth == 0 => {
                let declaration = declarations[start..=index].trim();
                if !declaration.is_empty() {
                    result.push(declaration.to_owned());
                }
                start = index + 1;
            }
            _ => {}
        }
    }
    assert_eq!(
        depth, 0,
        "unbalanced emitted type declarations: {declarations}"
    );
    assert!(declarations[start..].trim().is_empty());
    result
}

fn emitted_function_start(code: &str, name: &str) -> usize {
    let signature = format!("float4 {name}(");
    code.find(&signature)
        .unwrap_or_else(|| panic!("missing emitted definition for {name}: {code}"))
}

fn emitted_translation_unit(first: &str, second: &str, conversion: &str) -> String {
    let mut declarations = Vec::new();
    for (code, name) in [
        (first, "flip_first_and_add"),
        (second, "flip_second_and_add"),
        (conversion, "integer_to_float"),
    ] {
        for declaration in emitted_declarations(code, name) {
            if !declarations.contains(&declaration) {
                declarations.push(declaration);
            }
        }
    }
    let declarations = declarations.join("\n\n");
    let functions = [
        (first, "flip_first_and_add"),
        (second, "flip_second_and_add"),
        (conversion, "integer_to_float"),
    ]
    .into_iter()
    .map(|(code, name)| &code[emitted_function_start(code, name)..])
    .collect::<Vec<_>>()
    .join("\n");
    let driver = include_str!("fixtures/stack_float_snapshot_driver.c");
    driver
        .replace("/* KUNA_EMITTED_TYPES */", &declarations)
        .replace(
            "/* KUNA_EMITTED_CODE */",
            &format!(
                "static const uint4 dat_402000 = UINT32_C(0x80000000);\nstatic const uint4 dat_402010 = UINT32_C(0x40e00000);\n{functions}"
            ),
        )
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn stack_float_snapshots_keep_bits_at_both_point_lanes() {
    let mut compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| Command::new(compiler).arg("--version").output().is_ok())
        .collect();
    if compilers.is_empty() {
        if Command::new("cc").arg("--version").output().is_err() {
            eprintln!("no C compiler available; skipping native stack-float control");
            return;
        }
        compilers.push("cc");
    }
    let image = build_image(compilers[0]);
    let off = decompile_all(&image, false);
    let on = decompile_all(&image, true);
    let first_off = code(&off, "flip_first_and_add");
    let second_off = code(&off, "flip_second_and_add");
    let first = code(&on, "flip_first_and_add");
    let second = code(&on, "flip_second_and_add");
    let conversion = code(&on, "integer_to_float");
    let emitted = emitted_translation_unit(&first, &second, &conversion);

    assert!(first_off.contains("return v1.x + bias;"), "{first_off}");
    assert!(second_off.contains("return v1.y + bias;"), "{second_off}");
    for (body, lane) in [(&first, "view_uint4"), (&second, "view_uint4_at4.value")] {
        assert!(body.contains("uint4 v2;"), "{body}");
        assert!(body.contains("^ dat_402000"), "{body}");
        assert!(body.contains(lane), "{body}");
        assert!(
            body.contains(".from = v2 }).to + bias;"),
            "float consumer must reinterpret the cached word once: {body}"
        );
        assert!(!body.contains("return v2 + bias;"), "{body}");
    }
    assert!(conversion.contains("(float4)value"), "{conversion}");
    assert!(!conversion.contains("union"), "{conversion}");
    assert!(
        emitted.contains("union stack_views_401000_m10 {"),
        "{emitted}"
    );
    assert!(
        emitted.contains("union stack_views_401058_m10 {"),
        "{emitted}"
    );
    assert!(
        emitted.contains("stack_slice_401058_m10_at4_uint4 view_uint4_at4;"),
        "{emitted}"
    );
    assert_eq!(
        emitted.matches("\nstruct FloatPair {").count(),
        1,
        "{emitted}"
    );

    let native = common::scratch_file("stack-float-native", "bin");
    required(
        Command::new(compilers[0])
            .args(["-std=c11", "-O0", "-fstrict-aliasing", "-DKUNA_NATIVE"])
            .arg(fixture("stack_float_snapshot_driver.c"))
            .arg(fixture("stack_float_snapshot.S"))
            .arg("-o")
            .arg(&native),
    );
    common::process::required_output(&mut Command::new(native));

    let printed = common::scratch_file("stack-float-printed", "c");
    std::fs::write(&printed, emitted).unwrap();
    for compiler in compilers {
        for optimization in ["-O0", "-O2"] {
            let binary = common::scratch_file("stack-float-roundtrip", "bin");
            required(
                Command::new(compiler)
                    .args(["-std=c11", optimization, "-fstrict-aliasing"])
                    .arg(&printed)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}
