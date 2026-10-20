//! Named byte arrays retain scalar floating storage representations.
use crate::common;
use common::process;
use object::{Object, ObjectSection};
use std::process::Command;

#[test]
fn floating_transfers_through_named_byte_arrays_recompile_and_keep_bits() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let asm = common::scratch_file("array-float-transfer", "s");
    let binary = asm.with_extension("dll");
    std::fs::write(
        &asm,
        r#"
.data
.p2align 3
_state: .space 24
_whole: .space 4
.p2align 3
_dstate: .space 16
.text
.globl _sample4, _sample8, _whole4, _high4
.def _sample4; .scl 2; .type 32; .endef
_sample4:
 mov 4(%esp),%eax
 flds (%eax)
 fld1
 faddp
 fstps _state+4
 flds _state+4
 ret
.def _sample8; .scl 2; .type 32; .endef
_sample8:
 mov 4(%esp),%eax
 fldl (%eax)
 fld1
 faddp
 fstpl _state+16
 fldl _state+16
 ret
.def _whole4; .scl 2; .type 32; .endef
_whole4:
 mov 4(%esp),%eax
 flds (%eax)
 fld1
 faddp
 fstps _whole
 flds _whole
 ret
.def _high4; .scl 2; .type 32; .endef
_high4:
 mov 4(%esp),%eax
 flds (%eax)
 fld1
 faddp
 fstps _dstate+4
 mov 8(%esp),%edx
 mov _dstate+4,%ecx
 mov %ecx,(%edx)
 ret
"#,
    )
    .unwrap();
    let built = Command::new("clang")
        .args([
            "--target=i686-w64-windows-gnu",
            "-shared",
            "-nostdlib",
            "-Wl,--entry,0",
            "-Wl,--export-all-symbols",
        ])
        .arg(&asm)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let bytes = std::fs::read(&binary).unwrap();
    let object = object::File::parse(bytes.as_slice()).unwrap();
    let address = object.section_by_name(".data").unwrap().address();
    let declaration = format!("data 0x{address:x} unsigned char state[24]");
    let whole_declaration = format!("data 0x{:x} unsigned char whole[4]", address + 24);
    let double_declaration = format!("data 0x{:x} double dstate[2]", address + 32);
    let directory = asm.with_extension("project");
    let (out, err, status) = common::run_kuna(&[
        "decompile-project",
        binary.to_str().unwrap(),
        "-o",
        directory.to_str().unwrap(),
        "--functions",
        "_sample4,_sample8,_whole4,_high4",
        "--assert",
        &declaration,
        "--assert",
        &whole_declaration,
        "--assert",
        &double_declaration,
        "--assert",
        "prototype _high4 void __cdecl _high4(float *value, unsigned int *out)",
        "--assert",
        "prototype _whole4 float __cdecl _whole4(float *value)",
        "--assert",
        "prototype _sample4 float __cdecl _sample4(float *value)",
        "--assert",
        "prototype _sample8 double __cdecl _sample8(double *value)",
    ]);
    assert_eq!(status, 0, "{out}\n{err}");
    let stem = binary.file_name().unwrap().to_str().unwrap();
    let generated = directory.join(format!("{stem}.c"));
    let code = std::fs::read_to_string(&generated).unwrap();
    let declarations = asm.with_extension("objects.h");
    std::fs::write(
        &declarations,
        "extern unsigned char state[24];\nextern unsigned char whole[4];\nextern double dstate[2];\n",
    )
    .unwrap();
    let driver = asm.with_extension("driver.c");
    std::fs::write(
        &driver,
        format!(
            r#"
#include "{stem}.h"
#include <string.h>
unsigned char state[24];
unsigned char whole[4];
double dstate[2];
int main(void) {{
    const double values[] = {{0, -0.0, 0.25, -1.75, 123.125, -567.5, 0x1p-30, 0x1p30}};
    for(unsigned i=0;i<sizeof(values)/sizeof(values[0]);i++) {{
        float input4=(float)values[i];
        double input8=values[i];
        float expected4=(float)(1.0L+(long double)input4), actual4=_sample4(&input4);
        float actual_whole=_whole4(&input4);
        if(memcmp(&actual_whole,&expected4,4) || memcmp(whole,&expected4,4)) return 3;
        unsigned high_bits;
        _high4(&input4,&high_bits);
        if(memcmp(&high_bits,&expected4,4) || memcmp((unsigned char *)dstate+4,&expected4,4)) return 4;
        double expected8=(double)(1.0L+(long double)input8), actual8=_sample8(&input8);
        if(memcmp(&actual4,&expected4,4) || memcmp(&state[4],&expected4,4)) return 1;
        if(memcmp(&actual8,&expected8,8) || memcmp(&state[16],&expected8,8)) return 2;
    }}
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            let executable = asm.with_extension("native");
            let built = Command::new(compiler)
                .args([
                    optimization,
                    "-std=gnu11",
                    "-fno-fast-math",
                    "-ffp-contract=off",
                ])
                .arg("-include")
                .arg(&declarations)
                .arg("-I")
                .arg(&directory)
                .arg(&generated)
                .arg(&driver)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                built.status.success(),
                "{compiler} {optimization}: {}\n{code}",
                String::from_utf8_lossy(&built.stderr)
            );
            let result = Command::new(&executable).output().unwrap();
            assert!(
                result.status.success(),
                "{compiler} {optimization}: {result:?}\n{code}"
            );
        }
    }
}
