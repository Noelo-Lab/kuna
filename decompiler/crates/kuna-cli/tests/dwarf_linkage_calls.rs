//! Exact DWARF linkage names carry signatures across same-file ELF PLT calls.
use crate::common;
use common::process;
use std::process::Command;

fn roundtrip(compiler: &str, extension: &str, source: &str, functions: &[&str], driver: &str) {
    if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
        return;
    }
    let path = common::scratch_file("dwarf-linkage", extension);
    let library = path.with_extension("so");
    std::fs::write(&path, source).unwrap();
    let output = Command::new(compiler)
        .args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&path)
        .arg("-o")
        .arg(&library)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut code = String::new();
    for name in functions {
        let (body, err, status) = common::run_kuna(&["decompile", library.to_str().unwrap(), name]);
        assert_eq!(status, 0, "{err}");
        assert!(!body.contains("shifted();"), "{body}");
        code.push_str(&body.replacen(&format!(" {name}("), &format!(" regenerated_{name}("), 1));
    }
    let harness_extension = if extension == "cpp" { "cpp" } else { "c" };
    let harness = path.with_file_name(format!(
        "{}-roundtrip.{harness_extension}",
        path.file_stem().unwrap().to_str().unwrap()
    ));
    let executable = path.with_extension("exe");
    let (declarations, main) = driver.split_once("// GENERATED\n").unwrap();
    std::fs::write(&harness, format!("{declarations}\n{code}\n{main}")).unwrap();
    let compilers = if extension == "cpp" {
        ["g++", "clang++"]
    } else {
        ["gcc", "clang"]
    };
    let mut ran = false;
    for cc in compilers {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        ran = true;
        for opt in ["-O0", "-O2"] {
            let mut command = Command::new(cc);
            command.args([opt, "-Werror=uninitialized"]);
            if extension != "cpp" {
                command.arg("-Werror=incompatible-pointer-types");
            }
            let output = command
                .arg(&harness)
                .arg(&library)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{cc} {opt}: {}\n{code}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{cc} {opt}\n{code}"
            );
        }
    }
    assert!(ran);
    for path in [path, library, harness, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn ordinary_c_definition_keeps_its_pointer_contract_at_the_plt() {
    roundtrip(
        "gcc",
        "c",
        r#"
#include <stdint.h>
#include <string.h>
void set_float(float *value) { float replacement = 3.5f; memcpy(value, &replacement, 4); }
uint64_t sample(uint64_t initial) {
    uint64_t bits = initial;
    set_float((float *)&bits);
    return bits;
}
"#,
        &["sample"],
        r#"
#include <stdint.h>
extern void set_float(float *);
extern uint64_t sample(uint64_t);
// GENERATED
int main(void) {
    uint64_t state = 0x7385192327;
    for (unsigned i = 0; i < 256; ++i) {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        if (sample(state) != regenerated_sample(state)) return 1;
    }
    return 0;
}
"#,
    );
}

#[test]
fn asm_named_float_call_keeps_its_argument_and_result() {
    roundtrip(
        "gcc",
        "c",
        r#"
float shifted(float x) __asm__("exact_shifted");
float shifted(float x) { return x + 2.0f; }
void caller(float x, float *out) { *out = shifted(x); }
"#,
        &["caller"],
        r#"
extern float exact_shifted(float);
extern void caller(float, float *);
// GENERATED
int main(void) {
    float values[] = {-31.5f, -2.0f, 0.0f, 0.125f, 64.0f};
    for (unsigned i = 0; i < 5; ++i) {
        float expected, actual;
        caller(values[i], &expected);
        regenerated_caller(values[i], &actual);
        if (actual != expected) return 1;
    }
    return 0;
}
"#,
    );
}

#[test]
fn specification_linked_overloads_keep_distinct_float_and_integer_abis() {
    roundtrip(
        "g++",
        "cpp",
        r#"
namespace samples {
float shifted(float x) { return x + 2.0f; }
int shifted(int x) { return x - 3; }
}
extern "C" void float_caller(float x, float *out) { *out = samples::shifted(x); }
extern "C" void int_caller(int x, int *out) { *out = samples::shifted(x); }
"#,
        &["float_caller", "int_caller"],
        r#"
namespace samples { float shifted(float); int shifted(int); }
extern "C" void float_caller(float, float *);
extern "C" void int_caller(int, int *);
// GENERATED
int main(void) {
    for (int i = -4; i < 5; ++i) {
        float expected_float, actual_float;
        int expected_int, actual_int;
        float_caller(i * 0.25f, &expected_float);
        regenerated_float_caller(i * 0.25f, &actual_float);
        int_caller(i, &expected_int);
        regenerated_int_caller(i, &actual_int);
        if (actual_float != expected_float || actual_int != expected_int) return 1;
    }
    return 0;
}
"#,
    );
}

#[test]
fn indirect_function_resolver_does_not_retype_its_plt_entry() {
    if !cfg!(target_arch = "x86_64")
        || process::optional_output(Command::new("gcc").arg("--version")).is_none()
    {
        return;
    }
    let source = common::scratch_file("linkage-ifunc", "c");
    let library = source.with_extension("so");
    std::fs::write(
        &source,
        "static int impl(int a, int b, int c) { return a + b * c; }\n\
         void *resolve(void) __asm__(\"dispatch\");\n\
         void *resolve(void) { return (void *)impl; }\n\
         __asm__(\".type dispatch, %gnu_indirect_function\");\n\
         extern int dispatch_call(int, int, int) __asm__(\"dispatch\");\n\
         int caller(int x, int y) { return dispatch_call(x, y, 7) + 1; }\n",
    )
    .unwrap();
    let built = Command::new("gcc")
        .args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&source)
        .arg("-o")
        .arg(&library)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let (code, err, status) = common::run_kuna(&["decompile", library.to_str().unwrap(), "caller"]);
    assert_eq!(status, 0, "{err}");
    assert!(code.contains("dispatch(x,y,7)"), "{code}");
    for path in [source, library] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn local_linker_name_collision_does_not_retype_external_call() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let local = common::scratch_file("linkage-local", "c");
    let imported = common::scratch_file("linkage-import", "c");
    let library = local.with_extension("so");
    std::fs::write(&local, "static __attribute__((noinline,used)) float local_value(float x) __asm__(\"shared_name\");\nstatic float local_value(float x) { return x+1.0f; }\nfloat local_wrapper(float x) { return local_value(x); }\n").unwrap();
    std::fs::write(
        &imported,
        "extern int shared_name(int);\nint imported_wrapper(int x) { return shared_name(x); }\n",
    )
    .unwrap();
    let built = Command::new("clang")
        .args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&local)
        .arg(&imported)
        .arg("-o")
        .arg(&library)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let (code, err, status) =
        common::run_kuna(&["decompile", library.to_str().unwrap(), "imported_wrapper"]);
    assert_eq!(status, 0, "{err}");
    assert!(code.contains("shared_name(x)"), "{code}");
    assert!(!code.contains("float"), "{code}");
    for path in [local, imported, library] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn fortran_c_binding_definition_keeps_value_and_reference_arguments_at_the_plt() {
    roundtrip(
        "gfortran", "f90",
        r#"
subroutine offset_sample(value, output) bind(C)
  use iso_c_binding
  real(c_float), value :: value
  real(c_float) :: output
  output = value + 7.0_c_float
end subroutine
subroutine sample(value, output) bind(C)
  use iso_c_binding
  real(c_float), value :: value
  real(c_float) :: output
  interface
    subroutine offset_sample(value, output) bind(C)
      use iso_c_binding
      real(c_float), value :: value
      real(c_float) :: output
    end subroutine
  end interface
  call offset_sample(value, output)
end subroutine
"#,
        &["sample"],
        r#"
#include <string.h>
extern void sample(float, float *);
extern void offset_sample(float, float *);
// GENERATED
int main(void) {
    for (int i=-100;i<=100;++i) {
        float value=(float)i*0.125f, expected, actual;
        sample(value,&expected);
        regenerated_sample(value,&actual);
        if (memcmp(&actual,&expected,sizeof(actual))) return 1;
    }
    return 0;
}
"#,
    );
}
