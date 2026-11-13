use crate::common;
use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

const CPP: &str = include_str!("../../kuna-analysis/tests/fixtures/dwarf_namespaces.cpp");
const RUST: &str = include_str!("../../kuna-analysis/tests/fixtures/dwarf_namespaces.rs");
const INPUTS: &[u32] = &[
    0,
    1,
    7,
    12345,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_ffd7,
    0xffff_fff5,
    u32::MAX,
];

fn functions(fixture: &str, language: &str) -> String {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let result = process::required_output(Command::new(binary).args([
        "decompile-all",
        &common::fixture(fixture),
        "--json",
        "--language",
        language,
        "--functions",
        if language == "rust" {
            "direct_left,direct_nested,direct_right,direct_global"
        } else {
            "direct_left,direct_nested,direct_right,direct_global,direct_shadow"
        },
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), if language == "rust" { 4 } else { 5 });
    functions
        .iter()
        .map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn values(binary: &Path, input: u32) -> Vec<u32> {
    let result = process::required_output(Command::new(binary).arg(input.to_string()));
    String::from_utf8(result.stdout)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn compile(compiler: &str, source: &Path, optimization: &str) -> PathBuf {
    let binary = source.with_extension(format!("{compiler}-{optimization}"));
    let mut command = Command::new(compiler);
    if compiler == "rustc" {
        command.args([
            "--edition=2021",
            "--crate-name",
            "namespace_roundtrip",
            "-C",
            &format!("opt-level={optimization}"),
        ]);
    } else {
        command.args(["-std=c++17", &format!("-O{optimization}")]);
    }
    process::required_output(command.arg(source).arg("-o").arg(&binary));
    binary
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn cpp_calls_execute_in_their_original_namespaces() {
    let source = common::scratch_file("dwarf-namespaces", "cpp");
    let body = functions("dwarf_namespaces_cpp_x86_64", "c");
    let helpers = CPP.split("extern \"C\"").next().unwrap();
    let main = r#"
int main(int argc, char **argv) {
    uint32_t x = argc > 1 ? static_cast<uint32_t>(std::strtoul(argv[1], nullptr, 0)) : 0;
    std::printf("%u %u %u %u %u\n", direct_left(x), direct_nested(x), direct_right(x), direct_global(x), direct_shadow(x));
}
"#;
    for compiler in ["clang++", "g++"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["0", "2"] {
            std::fs::write(&source, format!("{helpers}
{body}
{main}")).unwrap();
            let binary = compile(compiler, &source, optimization);
            for &input in INPUTS {
                let native = values(
                    Path::new(&common::fixture("dwarf_namespaces_cpp_x86_64")),
                    input,
                );
                let actual = values(&binary, input);
                assert_eq!(actual, native[..5], "{compiler} O{optimization}, {input}");
            }
        }
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn rust_calls_execute_with_their_crate_and_module_paths() {
    if process::optional_output(Command::new("rustc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("dwarf-namespaces-rust", "rs");
    let body = functions("dwarf_namespaces_rust_x86_64", "rust");
    let helpers = RUST.split("#[no_mangle]").next().unwrap();
    let main = r#"
fn main() {
    let x = std::env::args().nth(1).unwrap().parse::<u32>().unwrap();
    unsafe { println!("{} {} {} {}", direct_left(x), direct_nested(x), direct_right(x), direct_global(x)); }
}

"#;
    std::fs::write(
        &source,
        format!("extern crate self as dwarf_namespaces;\n{helpers}\n{body}\n{main}"),
    )
    .unwrap();
    for optimization in ["0", "2"] {
        let binary = compile("rustc", &source, optimization);
        for &input in INPUTS {
            let native = values(
                Path::new(&common::fixture("dwarf_namespaces_rust_x86_64")),
                input,
            );
            assert_eq!(
                values(&binary, input),
                native[..4],
                "Rust O{optimization}, {input}"
            );
        }
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn qualified_dwarf_definitions_keep_function_values_bound_to_the_same_target() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let fixture = common::fixture("dwarf_function_pointer_x86_64");
    let result = process::required_output(Command::new(kuna).args([
        "decompile-all",
        &fixture,
        "--json",
        "--language",
        "c",
        "--functions",
        "left::target,get_target,main",
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 3);
    let bodies = functions
        .iter()
        .map(|function| {
            assert!(function["error"].is_null(), "{function}");
            function["code"].as_str().unwrap()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bodies.contains("return (unsigned long)left::target;"));
    let source = common::scratch_file("dwarf-function-pointer", "cpp");
    let prefix = "#include <cstdio>\n#include <cstdlib>\n#include <cstdint>\nnamespace left { unsigned int target(unsigned int); }\n#define main recovered_main\n";
    let driver = r#"
#undef main
int main(int argc, char **argv) {
    if (get_target() != reinterpret_cast<uintptr_t>(&left::target)) return 91;
    return recovered_main(argc, argv);
}

"#;
    std::fs::write(&source, format!("{prefix}{bodies}\n{driver}")).unwrap();
    for compiler in ["clang++", "g++"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["0", "2"] {
            let binary = compile(compiler, &source, optimization);
            for &input in INPUTS {
                assert_eq!(
                    values(&binary, input),
                    values(Path::new(&fixture), input),
                    "{compiler} O{optimization}, {input}"
                );
            }
        }
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn rust_raw_namespace_keeps_printable_c_definitions_and_calls() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let fixture = common::fixture("dwarf_raw_namespace_x86_64");
    let result = process::required_output(Command::new(kuna).args([
        "decompile-all",
        &fixture,
        "--json",
        "--language",
        "c",
        "--functions",
        "target,direct_target",
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 2);
    let bodies = functions
        .iter()
        .map(|function| {
            assert!(function["error"].is_null(), "{function}");
            assert_ne!(function["address"], 0);
            function["code"].as_str().unwrap().to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!bodies.contains("::union::"));
    let source = common::scratch_file("dwarf-raw-namespace", "cpp");
    let driver = r#"
int main(int argc, char **argv) {
    unsigned int x = static_cast<unsigned int>(std::strtoul(argv[1], nullptr, 0));
    std::printf("%u\n", direct_target(x));
}
"#;
    std::fs::write(
        &source,
        format!("#include <cstdio>\n#include <cstdlib>\n{bodies}\n{driver}"),
    )
    .unwrap();
    for compiler in ["clang++", "g++"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["0", "2"] {
            let binary = compile(compiler, &source, optimization);
            for &input in INPUTS {
                assert_eq!(values(&binary, input), values(Path::new(&fixture), input));
            }
        }
    }
}
