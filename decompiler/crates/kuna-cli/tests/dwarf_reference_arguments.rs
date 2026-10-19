//! Independently authored Fortran references and VALUE arguments round-trip through C.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#"
subroutine offset_sample(sample, bias, output)
  implicit none
  real :: sample, bias, output
  output = sample + bias
end subroutine

subroutine mixed_sample(sample, bias, output)
  implicit none
  real, value :: sample
  real :: bias, output
  output = sample - bias
end subroutine

subroutine integer_sample(sample, output)
  implicit none
  integer :: sample, output
  output = sample
end subroutine
"#;

#[test]
fn dwarf_dereferenced_parameters_keep_their_address_abi() {
    reference_parameters_round_trip(&["-O0"]);
}

#[test]
fn optimized_reference_parameters_round_trip_without_changing_value_arguments() {
    reference_parameters_round_trip(&["-O2", "-O3"]);
}

#[test]
fn c_values_pointers_and_cpp_references_keep_their_existing_signatures() {
    for (compiler, extension, source, functions) in [
        ("gcc", "c", "float scalar(float value) { return value; }\nfloat pointee(float *value) { return *value; }",
         &[("scalar", "float scalar(float value)"), ("pointee", "float pointee(float *value)")][..]),
        ("g++", "cpp", "extern \"C\" float reference(float &value) { return value; }",
         &[("reference", "float reference(float *value)")][..]),
    ] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() { continue; }
        let path = common::scratch_file("dwarf-reference-control", extension);
        let object = path.with_extension("so");
        std::fs::write(&path, source).unwrap();
        for level in ["-O0", "-O2"] {
            let output = Command::new(compiler).args(["-g", "-shared", "-fPIC", level])
                .arg(&path).arg("-o").arg(&object).output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            for &(function, signature) in functions {
                let (body, err, status) = common::run_kuna(&["decompile", object.to_str().unwrap(), function]);
                assert_eq!(status, 0, "{err}");
                assert!(body.starts_with(signature), "{compiler} {level}: {body}");
            }
        }
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(object).unwrap();
    }
}

#[test]
fn pe_reference_arguments_keep_their_stack_slots() {
    let compiler = "i686-w64-mingw32-gfortran";
    if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("dwarf-reference-pe", "f90");
    let binary = source.with_extension("dll");
    std::fs::write(&source, "subroutine many_refs(a,b,c,d,e,f,g,h,i,j,k,l)\nreal :: a,b,c,d,e,f,g,h,i,j,k,l\na=b+c+d+e+f+g+h+i+j+k+l\nl=a\nend\n").unwrap();
    let output = Command::new(compiler)
        .args(["-g", "-O0", "-shared"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (body, err, status) =
        common::run_kuna(&["decompile", binary.to_str().unwrap(), "many_refs"]);
    assert_eq!(status, 0, "{err}");
    let signature = body.lines().next().unwrap();
    assert_eq!(signature.matches("float *").count(), 12, "{body}");
    assert!(body.contains("*l = *a;"), "{body}");
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}

#[test]
fn large_reference_array_does_not_abort_parameter_allocation() {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("large-reference", "f90");
    let binary = source.with_extension("so");
    std::fs::write(
        &source,
        "subroutine first_sample(a,out)\nreal a(512),out\nout=a(1)\nend\n",
    )
    .unwrap();
    let output = Command::new("gfortran")
        .args(["-O0", "-g", "-shared", "-fPIC"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (body, err, status) =
        common::run_kuna(&["decompile", binary.to_str().unwrap(), "first_sample"]);
    assert_eq!(status, 0, "{err}");
    assert!(body.contains("(*a)"), "{body}");
    assert!(body.contains("*out = (*a)[0];"), "{body}");
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}

fn reference_parameters_round_trip(optimizations: &[&str]) {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        eprintln!("gfortran is not installed");
        return;
    }
    let source = common::scratch_file("dwarf-reference", "f90");
    let object = source.with_extension("so");
    std::fs::write(&source, format!("{SOURCE}\nsubroutine integer_value(sample, bias, output)\ninteger, value :: sample, bias\ninteger :: output\noutput = sample + bias\nend\nsubroutine array_sample(samples, bias, output)\nreal :: samples(4), bias, output\noutput = samples(2) + bias\nend\n")).unwrap();
    for optimization in optimizations {
        let compiled = Command::new("gfortran")
            .args([*optimization, "-g", "-shared", "-fPIC"])
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .unwrap();
        assert!(
            compiled.status.success(),
            "{}",
            String::from_utf8_lossy(&compiled.stderr)
        );
        let mut code = String::new();
        for name in [
            "offset_sample",
            "mixed_sample",
            "integer_sample",
            "integer_value",
            "array_sample",
        ] {
            let (body, err, status) =
                common::run_kuna(&["decompile", object.to_str().unwrap(), name]);
            assert_eq!(status, 0, "{err}");
            assert!(body.lines().next().unwrap().contains('*'), "{body}");
            code.push_str(&body);
        }
        let driver = r#"
#include <string.h>
extern void offset_sample_(float *, float *, float *);
extern void mixed_sample_(float, float *, float *);
extern void integer_sample_(int *, int *);
extern void integer_value_(int, int, int *);
extern void array_sample_(float *, float *, float *);
int main(void) {
    float values[] = {-17.5f, -0.0f, 0.25f, 1024.0f};
    for (unsigned i = 0; i < 4; ++i) {
      for (unsigned j = 0; j < 4; ++j) {
        float input = values[i], bias = values[j], expected, actual;
        offset_sample_(&input, &bias, &expected);
        offset_sample(&input, &bias, &actual);
        if (memcmp(&actual, &expected, sizeof(actual))) return 1;
        mixed_sample_(input, &bias, &expected);
        mixed_sample(input, &bias, &actual);
        if (memcmp(&actual, &expected, sizeof(actual))) return 2;
        float samples[4] = {values[j], input, bias, values[i]};
        array_sample_(samples, &bias, &expected);
        array_sample(&samples, &bias, &actual);
        if (memcmp(&actual, &expected, sizeof(actual))) return 5;
      }
    }
    int input = -123456, expected, actual;
    integer_sample_(&input, &expected);
    integer_sample(&input, &actual);
    if (actual != expected) return 3;
    for (int i = -100; i <= 100; ++i) {
        integer_value_(i, 73, &expected);
        integer_value(i, 73, &actual);
        if (actual != expected) return 4;
    }
    return 0;
}
"#;
        let c = source.with_extension("c");
        let exe = source.with_extension("exe");
        std::fs::write(&c, format!("{code}\n{driver}")).unwrap();
        let mut ran = false;
        for compiler in ["gcc", "clang"] {
            if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            ran = true;
            for level in ["-O0", "-O2"] {
                let result = Command::new(compiler)
                    .args(["-std=c11", "-Werror=incompatible-pointer-types", level])
                    .arg(&c)
                    .arg(&object)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "{compiler} {level}: {}\n{code}",
                    String::from_utf8_lossy(&result.stderr)
                );
                assert!(
                    Command::new(&exe).status().unwrap().success(),
                    "{compiler} {level}\n{code}"
                );
            }
        }
        assert!(ran, "the round-trip requires GCC or Clang");
    }
    for path in [
        source.clone(),
        object.clone(),
        source.with_extension("c"),
        source.with_extension("exe"),
    ] {
        std::fs::remove_file(path).unwrap();
    }
}
