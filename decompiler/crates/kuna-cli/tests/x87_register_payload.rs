//! Explicit x87 prototypes allocate payload bytes in floating registers.
#![cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::common;
use common::process;
use std::process::Command;

const ASSEMBLY: &str = r#"
.text
.globl sample
.type sample,@function
sample: fldt 4(%esp); ret
.size sample,.-sample
.globl sample_choice
.type sample_choice,@function
sample_choice:
 cmpl $0,4(%esp); je 1f
 fldt 8(%esp); ret
1: fldt 20(%esp); ret
.size sample_choice,.-sample_choice
.section .note.GNU-stack,"",@progbits
"#;

#[test]
fn explicit_extended_prototypes_round_trip_through_native_i386_calls() {
    for tool in ["as", "ld", "gcc"] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            return;
        }
    }
    let source = common::scratch_file("explicit-x87-return", "s");
    let object = source.with_extension("o");
    let library = source.with_extension("so");
    std::fs::write(&source, ASSEMBLY).unwrap();
    for (tool, flags, input, output) in [
        ("as", vec!["--32"], &source, &object),
        ("ld", vec!["-m", "elf_i386", "-shared"], &object, &library),
    ] {
        let result = Command::new(tool)
            .args(flags)
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let specs = common::repo_root().join("specs");
    let mut code = String::new();
    for (name, prototype) in [
        ("sample", "prototype sample long double sample(long double input)"),
        ("sample_choice", "prototype sample_choice long double sample_choice(int choice,long double left,long double right)"),
    ] {
        let (body, error, status) = common::run_kuna(&[
            "decompile",
            library.to_str().unwrap(),
            name,
            "--sleighpath",
            specs.to_str().unwrap(),
            "--assert",
            prototype,
            "--assert-strict",
        ]);
        assert_eq!(status, 0, "{error}");
        assert!(
            !body.contains("undefined10") && !body.contains("rethidden"),
            "{body}"
        );
        code.push_str(&body.replacen(&format!(" {name}("), &format!(" regenerated_{name}("), 1));
        code.push('\n');
    }
    let c = source.with_extension("c");
    std::fs::write(&c, format!(r#"
typedef long double float10;
extern long double sample(long double);
extern long double sample_choice(int, long double, long double);
{code}
_Static_assert(sizeof(long double) == 12, "native i386 storage");
void *memcpy(void *destination, const void *source, unsigned count) {{
    volatile unsigned char *out = destination;
    const volatile unsigned char *in = source;
    while (count--) *out++ = *in++;
    return destination;
}}
static int differs(long double left, long double right) {{
    const unsigned char *a = (const unsigned char *)&left, *b = (const unsigned char *)&right;
    for (unsigned i = 0; i < 10; ++i) if (a[i] != b[i]) return 1;
    return 0;
}}
static int test(void) {{
    const long double values[] = {{0.0L, -0.0L, 1.0L, -3.0L, 0x1.0000000000000002p0L,
        0x1p3000L, 0x1p-3000L, __builtin_infl(), -__builtin_infl(), __builtin_nanl("37")}};
    for (unsigned i = 0; i < sizeof(values)/sizeof(values[0]); ++i) {{
        long double left = values[i], right = values[(i+1) % (sizeof(values)/sizeof(values[0]))];
        if (differs(sample(left), regenerated_sample(left))) return 1;
        for (int choice = 0; choice < 2; ++choice)
            if (differs(sample_choice(choice,left,right), regenerated_sample_choice(choice,left,right))) return 2;
    }}
    return 0;
}}
__attribute__((noreturn,force_align_arg_pointer)) void _start(void) {{
    int result = test();
    __asm__ volatile("int $0x80" : : "a"(1), "b"(result) : "memory");
    __builtin_unreachable();
}}
"#)).unwrap();
    let executable = source.with_extension("exe");
    let emulator = process::optional_output(Command::new("qemu-i386").arg("--version")).is_some();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let result = Command::new(compiler)
                .args([
                    "-m32",
                    "-static",
                    "-nostdlib",
                    "-fno-pie",
                    "-no-pie",
                    "-ffreestanding",
                    "-fno-stack-protector",
                    "-Werror=implicit-function-declaration",
                    level,
                ])
                .arg(&c)
                .arg(&object)
                .args(["-Wl,-e,_start", "-o"])
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{compiler} {level}: {}\n{code}",
                String::from_utf8_lossy(&result.stderr)
            );
            let mut run = if emulator {
                let mut run = Command::new("qemu-i386");
                run.arg(&executable);
                run
            } else {
                Command::new(&executable)
            };
            assert!(
                run.status().unwrap().success(),
                "{compiler} {level}: native x87 result mismatch\n{code}"
            );
        }
    }
}
