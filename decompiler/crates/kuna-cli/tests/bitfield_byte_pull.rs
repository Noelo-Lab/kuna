use crate::common;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

fn fixture(debug: bool) -> PathBuf {
    common::repo_root()
        .join("tests/fixtures/bitfield-byte-pull")
        .join(if debug {
            "review-dwarf4.o"
        } else {
            "review-no-debug.o"
        })
}

fn decompile(name: &str, signature: &str, debug: bool) -> String {
    let image = fixture(debug);
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile",
        image.to_str().unwrap(),
        name,
        "--json",
        "--max-fn-seconds",
        "2",
    ]);
    if !debug {
        command.args([
            "--option",
            "ctypes",
            "on",
            "--assert-strict",
            "--assert",
            "typedef struct A { unsigned lo:3, hi:5, wide:16, top:8; } A;",
            "--assert",
            "typedef struct W { unsigned short w:16; } W;",
            "--assert",
            "typedef struct M { unsigned short w:16; double d; } M;",
        ]);
        command.args(["--assert", &format!("prototype {name} {signature}")]);
        if name == "a_call" {
            command.args(["--assert", "prototype change void change(A *p)"]);
        }
    }
    let output = common::process::required_output(&mut command);
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = &document["functions"][0];
    assert!(function["error"].is_null(), "{document}");
    function["code"].as_str().unwrap().to_owned()
}

fn execute(code: &str, checks: &str) {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("bitfield-pull", "c");
    let program = common::scratch_file("bitfield-pull", "exe");
    let original = common::repo_root().join("tests/fixtures/bitfield-byte-pull/review.c");
    let names = [
        "a_save", "hi_save", "a_alias", "a_call", "a_phi", "a_swap", "w_byval", "a_byval",
        "m_byval",
    ];
    let aliases = names
        .iter()
        .map(|n| format!("#define {n} native_{n}\n"))
        .collect::<String>();
    let unaliases = names
        .iter()
        .map(|n| format!("#undef {n}\n"))
        .collect::<String>();
    std::fs::write(
        &source,
        format!(
            "#include <assert.h>\n#include <stdint.h>\n{aliases}#include \"{}\"\n{unaliases}\n\
         typedef uint8_t undefined1; typedef uint16_t undefined2;\n\
         typedef uint32_t undefined4; typedef uint64_t undefined8;\n\
         {code}\nint main(void) {{ {checks} return 0; }}\n",
            original.display(),
        ),
    )
    .unwrap();
    common::process::required_output(
        Command::new("cc")
            .args(common::CC_GCC15_DEMOTE)
            .arg("-O2")
            .arg(&source)
            .arg("-o")
            .arg(&program),
    );
    common::process::required_output(&mut Command::new(&program));
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(program).unwrap();
}

#[test]
fn bitfield_loads_keep_their_values_across_stores_aliases_and_calls() {
    for debug in [true, false] {
        let functions = [
            ("a_save", "unsigned a_save(A *p, unsigned v)"),
            ("hi_save", "unsigned hi_save(A *p, unsigned v)"),
            ("a_alias", "unsigned a_alias(A *p, unsigned char *q)"),
            ("a_call", "unsigned a_call(A *p)"),
            ("a_swap", "unsigned a_swap(A *p)"),
        ];
        let code = functions
            .iter()
            .map(|(name, signature)| decompile(name, signature, debug))
            .collect::<Vec<_>>()
            .join("\n");
        let mut checks = String::new();
        for (name, native_args, recovered_args) in [
            ("a_save", "&a, seed * 319", "&b, seed * 319"),
            ("hi_save", "&a, seed * 319", "&b, seed * 319"),
            (
                "a_alias",
                "&a, (unsigned char *)&a + 1",
                "&b, (unsigned char *)&b + 1",
            ),
            ("a_call", "&a", "&b"),
            ("a_swap", "&a", "&b"),
        ] {
            checks.push_str(&format!(
                "for (unsigned seed = 0; seed < 256; ++seed) {{\n\
                 A a = {{seed & 7, (seed * 7) & 31, (seed * 257 + 0x1234) & 65535, (seed * 73 + 19) & 255}}, b = a;\n\
                 unsigned expected = native_{name}({native_args});\n\
                 unsigned actual = {name}({recovered_args});\n\
                 assert(expected == actual);\n\
                 assert(a.lo == b.lo && a.hi == b.hi && a.wide == b.wide && a.top == b.top);\n}}\n",
            ));
        }
        execute(&code, &checks);
    }
}

#[test]
fn bitfield_phi_uses_finish_and_preserve_values() {
    for debug in [true, false] {
        let code = decompile("a_phi", "unsigned a_phi(A *p, int k)", debug);
        execute(
            &code,
            "for (unsigned seed = 0; seed < 256; ++seed) {\n\
            A a = {seed & 7, (seed * 7) & 31, (seed * 257) & 65535, seed};\n\
            for (int k = -2; k < 6; ++k) assert(native_a_phi(&a, k) == a_phi(&a, k));\n}",
        );
    }
}

#[test]
fn whole_field_arguments_have_a_defined_value() {
    for debug in [true, false] {
        let code = [
            ("w_byval", "unsigned w_byval(W w)"),
            ("a_byval", "unsigned a_byval(A a)"),
            ("m_byval", "double m_byval(M m)"),
        ]
        .iter()
        .map(|(name, signature)| decompile(name, signature, debug))
        .collect::<Vec<_>>()
        .join("\n");
        execute(
            &code,
            "for (unsigned value = 0; value < 65536; ++value) {\n\
            W w = {value}; assert(native_w_byval(w) == w_byval(w));\n\
            A a = {value & 7, (value * 7) & 31, value, value & 255};\n\
            assert(native_a_byval(a) == a_byval(a));\n\
            M m = {value, (double)value * 0.25 - 8192.0};\n\
            assert(native_m_byval(m) == m_byval(m));\n}",
        );
    }
}
