//! A function whose result a caller reads returns it in `decompile-all` and in
//! a single-function `kuna decompile` when it also stores the value, tests it,
//! or hands a value made from it back in another register, and in
//! `decompile-all` a function whose epilogue zeroes every other return register
//! (`-fzero-call-used-regs`) returns the one its callers read. The fixture
//! holds gcc and clang -O2 builds of each; the printed C is compiled against it
//! and compared with it. A function nothing reads stays `void`.
use crate::common;
use common::process;
use std::process::Command;

const ASM: &str = include_str!("fixtures/stored_return_values.S");

/// Each function that returns: its name, its parameter list, and the arguments
/// the driver calls it with.
const RETURNING: &[(&str, &str, &str)] = &[
    ("triple_store", "int", "(a)"),
    ("load_store", "int *", "(&x)"),
    ("load_store_clang", "int *", "(&x)"),
    ("triple_test", "int", "(a)"),
    ("triple_test_clang", "int", "(a)"),
    ("triple_out", "int, int *", "(a, &x)"),
    ("put", "int *, int", "(&x, 3)"),
];

/// A `main` that calls each of `cases` and its emitted copy (`emitted_<name>`)
/// for 40 inputs and fails when a result or a global differs.
fn driver(cases: &[&(&str, &str, &str)]) -> String {
    let mut out = String::from(
        r#"
extern int gi, gj;
#define SAME(f, args)                                                          \
    do {                                                                       \
        int x = a;                                                             \
        gi = gj = 5;                                                           \
        int r = f args;                                                        \
        int si = gi, sj = gj, sx = x;                                          \
        x = a;                                                                 \
        gi = gj = 5;                                                           \
        if (r != emitted_##f args || si != gi || sj != gj || sx != x)          \
            return 1;                                                          \
    } while (0)
"#,
    );
    for (name, params, _) in cases {
        out.push_str(&format!("int {name}({params});\nint emitted_{name}({params});\n"));
    }
    out.push_str("int main(void) {\n    for (int a = -20; a < 20; a++) {\n");
    for (name, _, args) in cases {
        out.push_str(&format!("        SAME({name}, {args});\n"));
    }
    out.push_str("    }\n    return 0;\n}\n");
    out
}

/// Assemble the fixture with clang, or `None` when clang is absent.
fn fixture() -> Option<std::path::PathBuf> {
    process::optional_output(Command::new("clang").arg("--version"))?;
    let asm = common::scratch_file("stored-return-values", "S");
    let elf = asm.with_extension("elf");
    std::fs::write(&asm, ASM).unwrap();
    process::required_output(
        Command::new("clang")
            .args(["-nostdlib", "-no-pie", "-Wl,--build-id=none", "-Wl,-e,reader"])
            .arg(&asm)
            .arg("-o")
            .arg(&elf),
    );
    Some(asm)
}

/// Check that each of `cases` prints as returning `int` in `code`, then compile
/// the printed functions against the fixture and run the comparison driver.
fn round_trip(asm: &std::path::Path, cases: &[&(&str, &str, &str)], code: impl Fn(&str) -> String) {
    let mut emitted = String::from("extern int gi, gj;\nint check(int *);\n");
    for (name, _, _) in cases {
        let text = code(name);
        assert!(text.contains(&format!("int {name}(")), "{text}");
        assert!(!text.contains("void") && text.contains("return "), "{text}");
        let mut renamed = text.clone();
        for (other, _, _) in RETURNING {
            renamed = renamed.replace(&format!(" {other}("), &format!(" emitted_{other}("));
        }
        emitted.push_str(&renamed);
        emitted.push('\n');
    }
    let source = asm.with_extension("emitted.c");
    std::fs::write(&source, format!("{emitted}\n{}", driver(cases))).unwrap();
    for level in ["-O0", "-O2"] {
        let exe = asm.with_extension("emitted");
        process::required_output(
            Command::new("clang")
                .arg(level)
                .arg("-no-pie")
                .arg(&source)
                .arg(asm)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(&exe));
    }
}

fn kuna() -> std::ffi::OsString {
    std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into())
}

fn decompile_all(elf: &std::path::Path) -> Vec<(String, String)> {
    let output = process::required_output(
        Command::new(kuna()).args(["decompile-all", elf.to_str().unwrap(), "--json"]),
    );
    let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            assert!(f["error"].is_null(), "{f}");
            (
                f["name"].as_str().unwrap().to_owned(),
                f["code"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn a_read_result_is_returned_when_it_is_also_stored_or_tested() {
    let Some(asm) = fixture() else { return };
    let functions = decompile_all(&asm.with_extension("elf"));
    let code = |name: &str| {
        functions
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, c)| c.clone())
            .unwrap_or_else(|| panic!("no {name} in {functions:?}"))
    };
    let void = code("triple_void");
    assert!(void.contains("void triple_void("), "{void}");
    round_trip(&asm, &RETURNING.iter().collect::<Vec<_>>(), code);
}

/// `kuna decompile` of one function reads its callers' code after each call
/// (`callerreads`). `put`'s zeroing epilogue is left out: alone, it still
/// returns the `xmm0` its epilogue zeroes.
#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn a_single_function_decompile_returns_what_its_callers_read() {
    let Some(asm) = fixture() else { return };
    let elf = asm.with_extension("elf");
    let decompile = |name: &str, extra: &[&str]| {
        let output = process::required_output(
            Command::new(kuna()).arg("decompile").arg(&elf).arg(name).args(extra),
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let off = decompile("triple_store", &["--option", "callerreads", "off"]);
    assert!(off.contains("void triple_store("), "{off}");
    let void = decompile("triple_void", &[]);
    assert!(void.contains("void triple_void("), "{void}");
    let cases: Vec<_> = RETURNING.iter().filter(|(name, _, _)| *name != "put").collect();
    round_trip(&asm, &cases, |name| decompile(name, &[]));
}
