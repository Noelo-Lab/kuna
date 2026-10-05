//! A function whose result a caller reads returns it in `decompile-all` when it
//! also stores the value, tests it, or hands a value made from it back in
//! another register, and a function whose epilogue zeroes every other return
//! register (`-fzero-call-used-regs`) returns the one its callers read. The
//! fixture holds gcc and clang -O2 builds of each; the printed C is compiled
//! against it and compared with it. A function nothing reads stays `void`.
mod common;
use common::process;
use std::process::Command;

const ASM: &str = include_str!("fixtures/stored_return_values.S");

const RETURNING: &[&str] = &[
    "triple_store",
    "load_store",
    "load_store_clang",
    "triple_test",
    "triple_test_clang",
    "triple_out",
    "put",
];

const DRIVER: &str = r#"
extern int gi, gj;
int triple_store(int);
int load_store(int *);
int load_store_clang(int *);
int triple_test(int);
int triple_test_clang(int);
int triple_out(int, int *);
int put(int *, int);
int emitted_triple_store(int);
int emitted_load_store(int *);
int emitted_load_store_clang(int *);
int emitted_triple_test(int);
int emitted_triple_test_clang(int);
int emitted_triple_out(int, int *);
int emitted_put(int *, int);
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
int main(void) {
    for (int a = -20; a < 20; a++) {
        SAME(triple_store, (a));
        SAME(load_store, (&x));
        SAME(load_store_clang, (&x));
        SAME(triple_test, (a));
        SAME(triple_test_clang, (a));
        SAME(triple_out, (a, &x));
        SAME(put, (&x, 3));
    }
    return 0;
}
"#;

fn decompile_all(elf: &std::path::Path) -> Vec<(String, String)> {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(
        Command::new(binary).args(["decompile-all", elf.to_str().unwrap(), "--json"]),
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
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
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
    let functions = decompile_all(&elf);
    let code = |name: &str| {
        functions
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, c)| c.clone())
            .unwrap_or_else(|| panic!("no {name} in {functions:?}"))
    };
    let mut emitted = String::from("extern int gi, gj;\nint check(int *);\n");
    for name in RETURNING {
        let text = code(name);
        assert!(text.contains(&format!("int {name}(")), "{text}");
        assert!(!text.contains("void") && text.contains("return "), "{text}");
        let mut renamed = text.clone();
        for other in RETURNING {
            renamed = renamed.replace(&format!(" {other}("), &format!(" emitted_{other}("));
        }
        emitted.push_str(&renamed);
        emitted.push('\n');
    }
    let void = code("triple_void");
    assert!(void.contains("void triple_void("), "{void}");

    let source = asm.with_extension("emitted.c");
    std::fs::write(&source, format!("{emitted}\n{DRIVER}")).unwrap();
    for level in ["-O0", "-O2"] {
        let exe = asm.with_extension("emitted");
        process::required_output(
            Command::new("clang")
                .arg(level)
                .arg("-no-pie")
                .arg(&source)
                .arg(&asm)
                .arg("-o")
                .arg(&exe),
        );
        process::required_output(&mut Command::new(&exe));
    }
}
