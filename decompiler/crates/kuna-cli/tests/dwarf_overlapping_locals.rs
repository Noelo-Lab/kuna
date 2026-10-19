//! Shared debug-local views must retain writes when regenerated C is executed.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#"
function sample_f(initial) result(answer)
  use iso_c_binding
  implicit none
  integer(c_int64_t), intent(in) :: initial
  integer(c_int64_t) :: answer, bits
  real(c_float) :: alias
  equivalence(bits, alias)
  bits = initial
  call put_alias(alias)
  answer = bits
end function
subroutine put_alias(alias)
  use iso_c_binding
  implicit none
  real(c_float), intent(out) :: alias
  alias = 3.5_c_float
end subroutine
function interior_f(initial) result(answer)
  use iso_c_binding
  implicit none
  integer(c_int64_t), intent(in) :: initial
  integer(c_int64_t) :: answer, wide
  integer(c_int8_t) :: bytes(12)
  real(c_float) :: narrow
  equivalence(bytes(1), wide), (bytes(3), narrow)
  bytes = 0
  wide = initial
  call put_alias(narrow)
  answer = wide
end function
function transitive_f(initial) result(answer)
  use iso_c_binding
  implicit none
  integer(c_int64_t), intent(in) :: initial
  integer(c_int64_t) :: answer
  integer(c_int32_t) :: a(2), b(2), c(2)
  equivalence(a(2), b(1)), (b(2), c(1))
  a(1) = int(iand(initial, 65535_c_int64_t), c_int32_t)
  a(2) = 23
  b(2) = 31
  c(2) = 47
  call put_word(c(1))
  answer = int(a(1), c_int64_t) + int(a(2), c_int64_t) &
         + int(b(2), c_int64_t) + int(c(2), c_int64_t)
end function
subroutine put_word(word)
  use iso_c_binding
  implicit none
  integer(c_int32_t), intent(out) :: word
  word = 19
end subroutine
"#;

#[test]
fn overlapping_debug_views_round_trip_call_writes() {
    if process::optional_output(Command::new("gfortran").arg("--version")).is_none() {
        eprintln!("gfortran is not installed");
        return;
    }
    let source = common::scratch_file("debug-overlap", "f90");
    let original = source.with_extension("so");
    std::fs::write(&source, SOURCE).unwrap();
    let built = Command::new("gfortran")
        .args(["-O0", "-g", "-shared", "-fPIC", "-Wl,-Bsymbolic-functions"])
        .arg(&source)
        .arg("-o")
        .arg(&original)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let project = source.with_extension("project");
    let specs = common::repo_root().join("specs");
    let (_, error, status) = common::run_kuna(&[
        "decompile-project",
        original.to_str().unwrap(),
        "-o",
        project.to_str().unwrap(),
        "--functions",
        "sample_f_,interior_f_,transitive_f_",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--assert",
        "prototype sample_f_ long sample_f_(long *initial)",
        "--assert",
        "prototype interior_f_ long interior_f_(long *initial)",
        "--assert",
        "prototype transitive_f_ long transitive_f_(long *initial)",
        "--assert",
        "prototype put_alias_ void put_alias_(float *value)",
        "--assert",
        "prototype put_word_ void put_word_(int *value)",
        "--option",
        "stackviews",
        "on",
        "--assert-strict",
    ]);
    assert_eq!(status, 0, "{error}");
    let generated = project.join(format!(
        "{}.c",
        original.file_name().unwrap().to_str().unwrap()
    ));
    let header = std::fs::read_to_string(generated.with_extension("h")).unwrap();
    let mut layout_checks = String::new();
    let mut slice = None;
    for line in header.lines() {
        if line.starts_with("struct dwarf_stack_") && line.contains("_slice_") && line.contains('{')
        {
            slice = Some((line.split_whitespace().nth(1).unwrap().to_owned(), None));
        } else if let Some((name, offset)) = &mut slice {
            if let Some(padding) = line.split("_pad0[").nth(1) {
                *offset = Some(padding.split(']').next().unwrap().parse::<usize>().unwrap());
            }
            if line.starts_with('}') {
                let offset = offset.expect("interior slice has explicit prefix padding");
                layout_checks.push_str(&format!(
                    "_Static_assert(offsetof({name}, value) == {offset}, \"view offset\");\n\
                     _Static_assert(sizeof({name}) == {offset} + sizeof((({name} *)0)->value), \"view extent\");\n"
                ));
                slice = None;
            }
        }
    }
    let driver = source.with_extension("c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &driver,
        format!(
            r#"
#include <stdint.h>
#include <string.h>
#include <stddef.h>
extern void put_alias_(float *);
extern void put_word_(int *);
#define sample_f regenerated_sample
#define interior_f regenerated_interior
#define transitive_f regenerated_transitive
#define sample_f_ regenerated_sample
#define interior_f_ regenerated_interior
#define transitive_f_ regenerated_transitive
#include "{}"
#undef sample_f
#undef interior_f
#undef transitive_f
#undef sample_f_
#undef interior_f_
#undef transitive_f_
{}
extern long sample_f_(long *);
extern long interior_f_(long *);
extern long transitive_f_(long *);
int main(void) {{
    uint64_t state = 0x65721c7833;
    for (unsigned i = 0; i < 256; ++i) {{
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        long input; memcpy(&input, &state, sizeof input);
        long expected = sample_f_(&input), actual = regenerated_sample(&input);
        if (actual != expected) return 1;
        if (interior_f_(&input) != regenerated_interior(&input)) return 2;
        if (transitive_f_(&input) != regenerated_transitive(&input)) return 3;
    }}
    return 0;
}}
"#,
            generated.display(),
            layout_checks
        ),
    )
    .unwrap();
    let mut ran = false;
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        ran = true;
        for level in ["-O0", "-O2"] {
            let built = Command::new(compiler)
                .args(["-std=gnu11", "-Werror=incompatible-pointer-types", level])
                .arg(&driver)
                .arg(&original)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{compiler} {level}: {}",
                String::from_utf8_lossy(&built.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{compiler} {level}: overlapping writes were lost"
            );
        }
    }
    assert!(ran, "the round trip requires GCC or Clang");
}
