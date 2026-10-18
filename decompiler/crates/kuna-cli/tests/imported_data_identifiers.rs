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
        ]);
        assert_eq!(status, 0, "{err}");
        assert!(
            !body.contains("counter.3.1") && !body.contains("is_recursive."),
            "{body}"
        );
        let names: BTreeSet<_> = body
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .filter(|name| name.starts_with("kuna_global_"))
            .collect();
        assert_eq!(names.len(), 1, "{body}");
        let normalized = names.first().unwrap();
        let declarations = if compiler == "gcc" {
            assert!(body.contains("counter_3_1"), "{body}");
            format!("unsigned {normalized}=5;\nunsigned counter_3_1=11;")
        } else {
            format!("int {normalized}=0;\nvoid _gfortran_runtime_error_at(const char *where, ...) {{ (void)where; abort(); }}")
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
