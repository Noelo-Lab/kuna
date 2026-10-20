//! Imported punctuation must not turn a global variable into a C expression.
use crate::common;
use common::process;
use std::collections::BTreeSet;
use std::process::Command;

#[test]
#[cfg(target_os = "linux")]
fn imported_data_names_compile_and_keep_independent_global_state() {
    for (compiler, extension, source, reference_type, negative) in [
        (
            "gcc",
            "c",
            r#"
static unsigned first __asm__("counter.3.1") = 5;
static unsigned counter_3_1 = 11;
unsigned sample(unsigned amount) { first += amount; counter_3_1 += 2; return first + counter_3_1; }
"#,
            "unsigned",
            false,
        ),
        (
            "gfortran",
            "f90",
            r#"
function sample(amount) result(value) bind(C)
  use iso_c_binding
  integer(c_int), value :: amount
  integer(c_int) :: value
  value = amount + 37
end function sample
"#,
            "int",
            true,
        ),
    ] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        let input = common::scratch_file("imported-data-identifiers", extension);
        let binary = input.with_extension("so");
        std::fs::write(&input, source).unwrap();
        let mut command = Command::new(compiler);
        command.args(["-O0", "-shared", "-fPIC"]);
        if compiler == "gfortran" {
            command.args(["-g", "-fcheck=all"]);
        }
        let built = command.arg(&input).arg("-o").arg(&binary).output().unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let (body, err, status) = common::run_kuna(&[
            "decompile",
            binary.to_str().unwrap(),
            "sample",
            "--option",
            "globalref",
            "off",
            "--option",
            "symbolnamechars",
            "ident",
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(
            !body.contains("counter.3.1") && !body.contains("is_recursive."),
            "{body}"
        );
        let names: BTreeSet<_> = body
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .filter(|name| {
                if compiler == "gcc" {
                    name.starts_with("counter_3_1_")
                } else {
                    name.starts_with("is_recursive_")
                }
            })
            .collect();
        assert_eq!(names.len(), 1, "{body}");
        let normalized = names.first().unwrap();
        if compiler == "gcc" {
            assert_eq!(*normalized, "counter_3_1_1");
            let (default_body, error, status) = common::run_kuna(&[
                "decompile",
                binary.to_str().unwrap(),
                "sample",
                "--option",
                "globalref",
                "off",
            ]);
            assert_eq!(status, 0, "{error}");
            assert!(default_body.contains("counter.3.1"), "{default_body}");
            let debug_binary = input.with_extension("debug.so");
            let built = Command::new("gcc")
                .args(["-O2", "-g", "-shared", "-fPIC"])
                .arg(&input)
                .arg("-o")
                .arg(&debug_binary)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{}",
                String::from_utf8_lossy(&built.stderr)
            );
            for lookup_binary in [&binary, &debug_binary] {
                for mode in ["safe", "ident"] {
                    for spelling in ["counter.3.1", "counter_3_1"] {
                        let (_, error, status) = common::run_kuna(&[
                            "xrefs",
                            lookup_binary.to_str().unwrap(),
                            "--to",
                            spelling,
                            "--option",
                            "symbolnamechars",
                            mode,
                            "--json",
                        ]);
                        assert_eq!(status, 0, "{mode} {spelling}: {error}");
                    }
                    let mut addresses = Vec::new();
                    for (spelling, expected) in
                        [("counter.3.1", "05000000"), ("counter_3_1", "0b000000")]
                    {
                        let (text, error, status) = common::run_kuna(&[
                            "read",
                            lookup_binary.to_str().unwrap(),
                            spelling,
                            "--bytes",
                            "4",
                            "--option",
                            "symbolnamechars",
                            mode,
                            "--json",
                        ]);
                        assert_eq!(status, 0, "{mode} {spelling}: {error}");
                        let document: serde_json::Value = serde_json::from_str(&text).unwrap();
                        assert_eq!(
                            document["hex"].as_str().unwrap().replace(' ', ""),
                            expected,
                            "{text}"
                        );
                        addresses.push(document["start"].clone());
                    }
                    assert_ne!(addresses[0], addresses[1]);
                }
            }
            std::fs::remove_file(debug_binary).unwrap();
        }

        let declarations = if compiler == "gcc" {
            assert!(body.contains("counter_3_1"), "{body}");
            format!("unsigned {normalized}=5;\nunsigned counter_3_1=11;")
        } else {
            format!("int {normalized}=0;\nvoid gfortran_runtime_error_at(const char *where, ...) {{ (void)where; abort(); }}")
        };
        let regenerated = input.with_extension("regenerated.c");
        let executable = input.with_extension("exe");
        let amount = if negative { "i%101-50" } else { "i%16" };
        std::fs::write(&regenerated,format!(r#"
#include <dlfcn.h>
#include <stdlib.h>
{declarations}
{body}
int main(int argc, char **argv) {{
    if (argc!=2) return 2;
    void *module=dlopen(argv[1], RTLD_NOW | RTLD_LOCAL);
    if (!module) return 3;
    {reference_type} (*original)({reference_type})=({reference_type} (*)({reference_type}))dlsym(module,"sample");
    if (!original) return 4;
    for (int i=0;i<100;++i) {{
        int amount={amount};
        if (({reference_type})sample(amount)!=original(({reference_type})amount)) return 1;
    }}
    dlclose(module);
    return 0;
}}
"#)).unwrap();
        let mut ran = false;
        for cc in ["gcc", "clang"] {
            if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                continue;
            }
            ran = true;
            for opt in ["-O0", "-O2"] {
                let built = Command::new(cc)
                    .args(["-std=c11", "-Wall", "-Werror", opt])
                    .arg(&regenerated)
                    .arg("-ldl")
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .unwrap();
                assert!(
                    built.status.success(),
                    "{}\n{body}",
                    String::from_utf8_lossy(&built.stderr)
                );
                assert!(
                    Command::new(&executable)
                        .arg(&binary)
                        .status()
                        .unwrap()
                        .success(),
                    "{compiler}: {cc} {opt}\n{body}"
                );
            }
        }
        assert!(
            ran,
            "GCC or Clang is required for the regenerated-code check"
        );
        for path in [input, binary, regenerated, executable] {
            std::fs::remove_file(path).unwrap();
        }
    }
}

#[test]
#[cfg(target_os = "linux")]
fn shared_data_spelling_resolves_to_its_lowest_address_in_both_modes() {
    use object::{Object, ObjectSymbol};
    if process::optional_output(Command::new("gcc").arg("--version")).is_none() {
        return;
    }
    let binary = common::scratch_file("shared-data-spelling", "so");
    let mut sources = Vec::new();
    let mut command = Command::new("gcc");
    command.args(["-O0", "-shared", "-fPIC"]);
    for (index, value) in [11, 22, 33].into_iter().enumerate() {
        let source = common::scratch_file("shared-data-spelling", "c");
        std::fs::write(
            &source,
            format!("static int c __asm__(\"ctr.0\") = {value};\nint f{index}(int x) {{ return c += x; }}\n"),
        )
        .unwrap();
        command.arg(&source);
        sources.push(source);
    }
    let built = command.arg("-o").arg(&binary).output().unwrap();
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let image = std::fs::read(&binary).unwrap();
    let lowest = object::File::parse(&*image)
        .unwrap()
        .symbols()
        .filter(|symbol| symbol.name() == Ok("ctr.0"))
        .map(|symbol| symbol.address())
        .min()
        .unwrap();
    for mode in ["safe", "ident"] {
        let (text, error, status) = common::run_kuna(&[
            "read",
            binary.to_str().unwrap(),
            "ctr.0",
            "--bytes",
            "4",
            "--option",
            "symbolnamechars",
            mode,
            "--json",
        ]);
        assert_eq!(status, 0, "{mode}: {error}");
        let document: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(document["start"].as_u64(), Some(lowest), "{mode}: {text}");
        let (text, error, status) = common::run_kuna(&[
            "xrefs",
            binary.to_str().unwrap(),
            "--to",
            "ctr.0",
            "--option",
            "symbolnamechars",
            mode,
            "--json",
        ]);
        assert_eq!(status, 0, "{mode}: {error}");
        let document: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(document["target"]["address"].as_u64(), Some(lowest), "{mode}: {text}");
    }
    for path in sources.into_iter().chain([binary]) {
        std::fs::remove_file(path).unwrap();
    }
}
