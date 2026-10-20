//! Local frame views preserve physical writes and escaped addresses.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use object::{Object, ObjectSection, ObjectSymbol, SectionKind};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn compilers() -> Vec<&'static str> {
    ["gcc", "clang"]
        .into_iter()
        .filter(|compiler| {
            common::process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect()
}

fn escaped_publications(code: &str) -> usize {
    code.lines()
        .filter(|line| line.trim_start().starts_with("escaped_pointer ="))
        .count()
}

fn decompile(image: &Path, function: &str, enabled: bool, contracts: &[String]) -> String {
    decompile_with_views(image, function, enabled, contracts, false)
}

fn decompile_with_views(
    image: &Path,
    function: &str,
    enabled: bool,
    contracts: &[String],
    full_views: bool,
) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile")
        .arg(image)
        .arg(function)
        .args([
            "--target",
            "x86:LE:64:default:gcc",
            "--mode",
            "reliable",
            "--json",
            "--assert-strict",
            "--option",
            "structdefs",
            "on",
            "--option",
            "stackviews",
            if full_views { "on" } else { "off" },
            "--option",
            "stackalias",
            "off",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"));
    if !enabled {
        command.args(["--option", "localstackviews", "off"]);
    }
    for contract in contracts {
        command.args(["--assert", contract]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(25),
    )
    .expect("bounded local-view decompilation");
    assert!(
        output.status.success(),
        "{function}, localstackviews={enabled}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(document["functions"][0]["error"].is_null(), "{document}");
    let code = document["functions"][0]["code"].as_str().unwrap();
    if enabled {
        assert!(!code.contains("WARNING:"), "{code}");
    }
    code.to_owned()
}

fn input_contracts(image: &Path) -> Vec<String> {
    let bytes = std::fs::read(image).unwrap();
    let object = object::File::parse(bytes.as_slice()).unwrap();
    let address = |name: &str| {
        object
            .symbols()
            .find(|symbol| symbol.name().ok() == Some(name))
            .unwrap_or_else(|| panic!("missing ELF symbol {name}"))
            .address()
    };
    let mut contracts = vec![
        "typedef struct LifetimePair { int x; int y; };".to_owned(),
        "typedef struct LifetimeHandle { int *value; };".to_owned(),
        format!("data 0x{:x} int datum", address("datum")),
    ];
    for (name, signature) in [
        ("local_constant", "int local_constant(void)"),
        ("local_register", "int local_register(int a,int b)"),
        ("local_partial", "int local_partial(void)"),
        ("local_branch", "int local_branch(int condition)"),
        (
            "read_pair",
            "int read_pair(const struct LifetimePair *pair)",
        ),
        (
            "read_handle",
            "int read_handle(const struct LifetimeHandle *handle)",
        ),
    ] {
        contracts.push(format!("prototype 0x{:x} {signature}", address(name)));
    }
    contracts
}

const PRELUDE: &str = r#"
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
typedef int8_t int1; typedef uint8_t uint1;
typedef int16_t int2; typedef uint16_t uint2;
typedef int32_t int4; typedef uint32_t uint4;
typedef int64_t int8; typedef uint64_t uint8;
typedef uint8_t undefined1; typedef uint16_t undefined2;
typedef uint32_t undefined4; typedef uint64_t undefined8;
#define CONCAT11(hi,lo) ((uint2)(((uint2)(uint1)(hi) << 8) | (uint1)(lo)))
#define CONCAT22(hi,lo) ((uint4)(((uint4)(uint2)(hi) << 16) | (uint2)(lo)))
#define CONCAT44(hi,lo) ((uint8)(((uint8)(uint4)(hi) << 32) | (uint4)(lo)))
"#;

fn readonly_pool_words(image: &Path, code: &str) -> String {
    let globals = code
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter_map(|word| {
            let address = u64::from_str_radix(word.strip_prefix("dat_")?, 16).ok()?;
            Some((word, address))
        })
        .collect::<BTreeSet<_>>();
    if globals.is_empty() {
        return String::new();
    }
    let bytes = std::fs::read(image).unwrap();
    let object = object::File::parse(bytes.as_slice()).unwrap();
    assert!(
        object.is_little_endian(),
        "x86 fixture must be little-endian"
    );
    let mut definitions = String::new();
    for (name, address) in globals {
        let end = address.checked_add(8).expect("constant-pool extent");
        let section = object
            .sections()
            .find(|section| {
                section.kind() == SectionKind::ReadOnlyData
                    && section.address() <= address
                    && section
                        .address()
                        .checked_add(section.size())
                        .is_some_and(|section_end| end <= section_end)
            })
            .unwrap_or_else(|| panic!("no readonly eight-byte constant holds {name}"));
        let data = section.data_range(address, 8).unwrap().unwrap();
        let bits = u64::from_le_bytes(data.try_into().unwrap());
        definitions.push_str(&format!(
            "static const uint8 {name} = UINT64_C(0x{bits:016x});\n"
        ));
    }
    definitions
}

fn execute_emitted(
    compilers: &[&str],
    image: &Path,
    function: &str,
    cases: &[(&str, i32)],
    code: &str,
) {
    let header = fixture("local_stack_views.h");
    let mut context = format!(
        "{PRELUDE}\n#define LOCAL_STACK_VIEWS_FORWARD_TYPES 1\n#include {:?}\n",
        header
    );
    for (name, fields) in [
        ("LifetimePair", "int x; int y;"),
        ("LifetimeHandle", "int *value;"),
    ] {
        if !code.contains(&format!("struct {name} {{")) {
            context.push_str(&format!("struct {name} {{ {fields} }};\n"));
        }
    }
    context.push_str(&readonly_pool_words(image, code));
    let checks = cases
        .iter()
        .map(|(arguments, expected)| {
            format!(
                "poison_stack(); result = {function}({arguments}); \
                 if (result != {expected}) {{ \
                 fprintf(stderr, \"{function}({arguments}): expected {expected}, got %d\\n\", result); \
                 return 1; }}\n"
            )
        })
        .collect::<String>();
    let source = common::scratch_file(function, "c");
    std::fs::write(
        &source,
        format!(
            "{context}\n{code}\n\
             static __attribute__((noinline)) void poison_stack(void) {{ \
             volatile unsigned char bytes[1024]; \
             for (unsigned int i = 0; i < sizeof(bytes); ++i) bytes[i] = 0xa5; }}\n\
             int main(void) {{ int result; {checks} return 0; }}\n"
        ),
    )
    .unwrap();
    for compiler in compilers {
        for optimization in ["-O0", "-O2"] {
            let executable = common::scratch_file(function, "elf");
            common::process::required_output(
                Command::new(compiler)
                    .args([
                        "-std=c11",
                        optimization,
                        "-fstrict-aliasing",
                        "-no-pie",
                        "-fno-stack-protector",
                        "-Werror=int-conversion",
                        "-Werror=incompatible-pointer-types",
                        "-Werror=implicit-function-declaration",
                    ])
                    .arg(&source)
                    .arg(fixture("local_stack_views_helpers.c"))
                    .arg("-o")
                    .arg(&executable),
            );
            let output = common::process::output_with_timeout(
                &mut Command::new(&executable),
                Duration::from_secs(5),
                Duration::from_millis(25),
            )
            .expect("bounded emitted local-view execution");
            assert!(
                output.status.success(),
                "{compiler} {optimization}: {}\n{code}",
                String::from_utf8_lossy(&output.stderr)
            );
            std::fs::remove_file(executable).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
}

#[test]
fn optimized_pair_handle_lifetimes_preserve_constant_register_partial_and_branch_writes() {
    let compilers = compilers();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping local-view native execution");
        return;
    }
    let mut inputs: Vec<_> = compilers
        .iter()
        .map(|compiler| (*compiler, false))
        .collect();
    if compilers.contains(&"gcc") {
        inputs.push(("gcc", true));
    }
    for (input_compiler, slp) in inputs {
        let image = common::scratch_file("local-stack-views-input", "elf");
        let mut build = Command::new(input_compiler);
        if slp {
            build.arg("-ftree-slp-vectorize");
        }
        common::process::required_output(
            build
                .args([
                    "-O2",
                    "-no-pie",
                    "-fno-stack-protector",
                    "-fcf-protection=none",
                ])
                .arg(fixture("local_stack_views.c"))
                .arg(fixture("local_stack_views_helpers.c"))
                .arg("-o")
                .arg(&image),
        );
        common::process::required_output(&mut Command::new(&image));
        let contracts = input_contracts(&image);
        for (function, cases) in [
            ("local_constant", &[("", 67)][..]),
            ("local_register", &[("3,4", 62), ("-3,8", 60)][..]),
            ("local_partial", &[("", 92)][..]),
            ("local_branch", &[("0", 67), ("1", 162), ("-1", 162)][..]),
        ] {
            let enabled = decompile(&image, function, true, &contracts);
            assert!(enabled.contains("union stack_views_"), "{enabled}");
            execute_emitted(&compilers, &image, function, cases, &enabled);
            let disabled = decompile(&image, function, false, &contracts);
            assert!(!disabled.contains("union stack_views_"), "{disabled}");
            assert!(disabled.contains("read_pair("), "{disabled}");
            assert!(disabled.contains("read_handle("), "{disabled}");
        }
        std::fs::remove_file(image).unwrap();
    }
}

#[test]
fn permanent_stack_lifetimes_keep_escaped_pointers_and_simultaneous_views() {
    let compilers = compilers();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping local-view native execution");
        return;
    }
    let image = PathBuf::from(common::fixture("lifetimes_stack_x86_64"));
    common::process::required_output(&mut Command::new(&image));
    let contracts = vec![format!("@{}.kuna", image.display())];
    for (function, expected) in [
        ("stack_reuse", 67),
        ("escaped_alias", 7),
        ("escaped_object_reuse", 122),
        ("simultaneous_views", 56),
    ] {
        let enabled = decompile(&image, function, true, &contracts);
        if function != "escaped_alias" {
            assert!(enabled.contains("union stack_views_"), "{enabled}");
        }
        if matches!(function, "escaped_alias" | "escaped_object_reuse") {
            assert_eq!(escaped_publications(&enabled), 1, "{enabled}");
        }
        execute_emitted(&compilers, &image, function, &[("", expected)], &enabled);
        let disabled = decompile(&image, function, false, &contracts);
        assert!(!disabled.contains("union stack_views_"), "{disabled}");
    }
}

#[test]
fn shifted_declared_objects_decline_without_splitting_prior_storage() {
    let image = PathBuf::from(common::fixture("lifetimes_objects_x86_64"));
    common::process::required_output(&mut Command::new(&image));
    let contracts = vec![format!("@{}.kuna", image.display())];
    for function in ["obj_global_shifted", "obj_volatile_shifted"] {
        let enabled = decompile(&image, function, true, &contracts);
        let disabled = decompile(&image, function, false, &contracts);
        assert_eq!(enabled, disabled, "{function}");
        assert!(!enabled.contains("union stack_views_"), "{enabled}");
    }
}

#[test]
fn qualified_nominal_types_decline_automatic_views() {
    let image = PathBuf::from(common::fixture("dwarfvariants_x86_64"));
    let enabled = decompile(&image, "fx::_start", true, &[]);
    let disabled = decompile(&image, "fx::_start", false, &[]);
    assert_eq!(enabled, disabled);
    assert!(!enabled.contains("union stack_views_"), "{enabled}");
}

#[test]
fn escaped_scalar_writes_reach_every_observer_and_readback() {
    let compilers = compilers();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping local-view native execution");
        return;
    }
    let image = PathBuf::from(common::fixture("lifetimes_stack_x86_64"));
    common::process::required_output(&mut Command::new(&image));
    let contracts = vec![format!("@{}.kuna", image.display())];
    for (function, cases, publications) in [
        ("escaped_indirect_write", &[("", 7)][..], 1),
        ("escaped_indirect_partial", &[("", 0x1122aa44)][..], 1),
        ("callee_updates_saved", &[("", 7)][..], 1),
        ("escaped_two_reads", &[("", 12)][..], 1),
        ("escaped_partial", &[("", 0x9900)][..], 1),
        ("escaped_loop", &[("", 3)][..], 1),
        ("escaped_branch", &[("0", 7), ("1", 5), ("-1", 5)][..], 1),
        ("callback_rewrite", &[("set_seven", 7)][..], 0),
    ] {
        let enabled = decompile(&image, function, true, &contracts);
        assert_eq!(escaped_publications(&enabled), publications, "{enabled}");
        execute_emitted(&compilers, &image, function, cases, &enabled);
        let disabled = decompile(&image, function, false, &contracts);
        assert!(!disabled.contains("union stack_views_"), "{disabled}");
        if function == "escaped_loop" {
            let full = decompile_with_views(&image, function, true, &contracts, true);
            let prior = decompile_with_views(&image, function, false, &contracts, true);
            assert_eq!(full, prior);
            execute_emitted(&compilers, &image, function, cases, &full);
        }
    }
}

#[test]
fn escaped_ranges_preserve_valid_accesses_and_decline_crossing_reads_and_stores() {
    let compilers = compilers();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping local-view native execution");
        return;
    }
    let cases = [
        (0x1122334455667788_u64, 0x8899aabbccddeeff_u64),
        (0xfedcba9876543210_u64, 0x0123456789abcdef_u64),
    ];
    let functions = [
        "local_range_valid",
        "local_range_cross_word",
        "local_range_cross_partial",
        "local_range_vector",
        "local_range_read_valid",
        "local_range_read_cross",
    ];
    let checks = |function: &str| {
        cases
            .iter()
            .map(|&(initial, replacement)| {
                let expected = match function {
                    "local_range_read_valid" => initial >> 32,
                    "local_range_read_cross" => (initial >> 32) | ((replacement & 0xffffffff) << 32),
                    "local_range_cross_partial" => {
                        (initial & 0x00ffffffffffffff) | ((replacement & 0xff) << 56)
                    }
                    _ => (initial & 0xffffffff) | ((replacement & 0xffffffff) << 32),
                };
                format!(
                    "if ({function}(0x{initial:x}ULL,0x{replacement:x}ULL) != 0x{expected:x}ULL) return 1;\n"
                )
            })
            .collect::<String>()
    };
    let driver = common::scratch_file("local-stack-views-ranges", "c");
    let declarations = functions
        .iter()
        .map(|function| {
            format!(
                "extern unsigned long long {function}(unsigned long long initial,unsigned long long replacement);\n"
            )
        })
        .collect::<String>();
    let native_checks = functions
        .iter()
        .map(|function| checks(function))
        .collect::<String>();
    std::fs::write(
        &driver,
        format!("{declarations}\nint main(void) {{ {native_checks} return 0; }}\n"),
    )
    .unwrap();
    let image = common::scratch_file("local-stack-views-ranges", "elf");
    for (index, compiler) in compilers.iter().enumerate() {
        for optimization in ["-O0", "-O2"] {
            let canonical = index == 0 && optimization == "-O2";
            let built = if canonical {
                image.clone()
            } else {
                common::scratch_file("local-stack-views-ranges-native", "elf")
            };
            common::process::required_output(
                Command::new(compiler)
                    .args([
                        "-std=c11",
                        optimization,
                        "-no-pie",
                        "-fno-stack-protector",
                        "-fstrict-aliasing",
                        "-Werror=int-conversion",
                        "-Werror=incompatible-pointer-types",
                        "-Werror=implicit-function-declaration",
                    ])
                    .arg(&driver)
                    .arg(fixture("local_stack_views_ranges.S"))
                    .arg(fixture("local_stack_views_helpers.c"))
                    .arg("-o")
                    .arg(&built),
            );
            common::process::required_output(&mut Command::new(&built));
            if !canonical {
                std::fs::remove_file(built).unwrap();
            }
        }
    }
    let bytes = std::fs::read(&image).unwrap();
    let object = object::File::parse(bytes.as_slice()).unwrap();
    let address = |name: &str| {
        object
            .symbols()
            .find(|symbol| symbol.name().ok() == Some(name))
            .unwrap_or_else(|| panic!("missing ELF symbol {name}"))
            .address()
    };
    let mut contracts = vec![format!(
        "data 0x{:x} int *escaped_pointer",
        address("escaped_pointer")
    )];
    contracts.extend([
        "typedef struct LifetimePair { int x; int y; };".to_string(),
        "typedef struct LifetimeHandle { int *value; };".to_string(),
        format!("prototype 0x{:x} unsigned long long read_range_pair(const struct LifetimePair *pair)", address("read_range_pair")),
        format!("prototype 0x{:x} unsigned long long read_range_handle(const struct LifetimeHandle *handle)", address("read_range_handle")),
    ]);
    for function in functions {
        contracts.push(format!(
            "prototype 0x{:x} unsigned long long {function}(unsigned long long initial,unsigned long long replacement)",
            address(function)
        ));
    }
    for function in functions {
        let enabled = decompile(&image, function, true, &contracts);
        if matches!(function, "local_range_valid" | "local_range_read_valid") {
            let wrapper = format!(
                "{enabled}\nint local_range_check(void) {{ {} return 0; }}\n",
                checks(function)
            );
            execute_emitted(
                &compilers,
                &image,
                "local_range_check",
                &[("", 0)],
                &wrapper,
            );
        }
        let disabled = decompile(&image, function, false, &contracts);
        assert!(!disabled.contains("union stack_views_"), "{disabled}");
        if !matches!(function, "local_range_valid" | "local_range_read_valid") {
            assert_eq!(enabled, disabled, "{function}");
            assert!(!enabled.contains("union stack_views_"), "{enabled}");
        }
        if function == "local_range_read_cross" {
            let full = decompile_with_views(&image, function, true, &contracts, true);
            let prior = decompile_with_views(&image, function, false, &contracts, true);
            assert_eq!(full, prior);
        }
    }
    std::fs::remove_file(image).unwrap();
    std::fs::remove_file(driver).unwrap();
}
