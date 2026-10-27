//! Runtime-sized stack buffers retain storage across ordinary calls.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn pe32_unoptimized_stack_allocation() {
    check(Some("i686-w64-windows-gnu"), "-O0", false);
}
#[test]
fn pe32_optimized_stack_allocation() {
    check(Some("i686-w64-windows-gnu"), "-O2", false);
}
#[test]
fn pe64_unoptimized_stack_allocation() {
    check(Some("x86_64-w64-windows-gnu"), "-O0", false);
}
#[test]
fn pe64_optimized_stack_allocation() {
    check(Some("x86_64-w64-windows-gnu"), "-O2", false);
}
#[test]
fn elf64_unoptimized_stack_allocation_roundtrip() {
    check(None, "-O0", false);
}
#[test]
fn elf64_optimized_stack_allocation_roundtrip() {
    check(None, "-O2", false);
}
#[test]
fn elf64_two_stack_allocations_roundtrip() {
    check(None, "-O0", true);
}
#[test]
fn pe32_two_stack_allocations() {
    check(Some("i686-w64-windows-gnu"), "-O0", true);
}

#[derive(Clone, Copy)]
enum Case {
    One,
    Two,
    Conditional,
    Repeated,
    Scoped,
    Probe,
    ProbeTwo,
    Drifting,
    Restored,
    Displaced,
    SteppedProbe,
    MergedSteppedProbe,
}

#[test]
fn elf64_conditional_allocation_roundtrip() {
    check_case(None, "-O0", Case::Conditional);
}
#[test]
fn pe32_conditional_allocation() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::Conditional);
}
#[test]
fn elf64_loop_allocation_roundtrip() {
    check_case(None, "-O0", Case::Repeated);
}
#[test]
fn pe32_loop_allocation() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::Repeated);
}
#[test]
fn elf64_scoped_vla_is_declined() {
    check_case(None, "-O0", Case::Scoped);
}
#[test]
fn pe32_scoped_vla_is_declined() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::Scoped);
}
#[test]
fn elf64_probe_loop_roundtrip() {
    check_case(None, "-O0", Case::Probe);
}
#[test]
fn pe32_probe_loop() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::Probe);
}

#[test]
fn elf64_two_probe_allocations_roundtrip() {
    check_case(None, "-O0", Case::ProbeTwo);
}
#[test]
fn pe32_two_probe_allocations() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::ProbeTwo);
}

#[test]
fn elf64_unknown_outgoing_stack_is_declined() {
    check_case(None, "-O0", Case::Drifting);
}

#[test]
fn elf64_displaced_allocation_address_roundtrip() {
    check_case(None, "-O0", Case::Displaced);
}

#[test]
fn elf64_stepped_stack_probe_is_declined() {
    check_case(None, "-O0", Case::SteppedProbe);
}

#[test]
fn elf64_reconverging_stack_probe_is_declined() {
    check_case(None, "-O0", Case::MergedSteppedProbe);
}

#[test]
fn pe32_restored_frame_allocation() {
    check_case(Some("i686-w64-windows-gnu"), "-O0", Case::Restored);
}

#[test]
fn elf64_restored_frame_allocation_roundtrip() {
    check_case(None, "-O0", Case::Restored);
}

fn check(target: Option<&str>, optimization: &str, twice: bool) {
    check_case(
        target,
        optimization,
        if twice { Case::Two } else { Case::One },
    );
}
fn check_case(target: Option<&str>, optimization: &str, case: Case) {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("dynamic-stack-allocation", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join(if target.is_some() {
        "sample.dll"
    } else {
        "sample.so"
    });
    let body = match case {
        Case::Two => "char *first=__builtin_alloca((unsigned)n);char *second=__builtin_alloca((unsigned)n);first[0]=3;second[0]=7;return observe(first,n)+observe(second,n);",
        Case::One => "char *data=__builtin_alloca((unsigned)n);data[0]=7;return observe(data,n);",
        Case::Conditional => "char *data;volatile int choice=n&1;if(choice)data=__builtin_alloca((unsigned)n);else data=__builtin_alloca((unsigned)n+16);data[0]=7;return observe(data,n);",
        Case::Repeated => "int total=0;for(int i=0;i<3;i++){char *data=__builtin_alloca((unsigned)n+i);data[0]=7;total+=observe(data,n+i);}return total;",
        Case::Scoped => "int total=0;for(int i=0;i<20000;i++){char data[n+4000];data[0]=7;total+=observe(data,n+4000);}return total;",
        Case::Restored if target.is_some() => r#"__asm__(
"pushl %ebp\nmovl %esp,%ebp\npushl %ebx\npushl %edi\nsubl $8,%esp\nmovl 8(%ebp),%ebx\nmovl $31,-12(%ebp)\n"
"movl %ebx,%eax\naddl $15,%eax\nandl $-16,%eax\nsubl %eax,%esp\nmovl %esp,%edi\nmovb $7,(%edi)\n"
"pushl $0x13579\npushl %ebp\nmovl %esp,%ebp\npushl %ebx\npushl %edi\ncalll _observe\naddl $8,%esp\npopl %ebp\naddl $4,%esp\n"
"movl %eax,-16(%ebp)\nleal -16(%ebp),%edi\nmovl %ebx,%eax\naddl $15,%eax\nandl $-16,%eax\nsubl %eax,%edi\nmovl %edi,%esp\nmovb $7,(%edi)\npushl $0x2468a\npushl %ebp\nmovl %esp,%ebp\npushl %ebx\npushl %edi\ncalll _observe\naddl $8,%esp\npopl %ebp\naddl $4,%esp\naddl -16(%ebp),%eax\naddl -12(%ebp),%eax\nleal -8(%ebp),%esp\npopl %edi\npopl %ebx\npopl %ebp\nretl\n");"#,
        Case::Restored => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\npushq %rbx\nsubq $24,%rsp\nmovl %edi,%ebx\nmovl $31,-16(%rbp)\n"
"movl %ebx,%eax\naddq $15,%rax\nandq $-16,%rax\nsubq %rax,%rsp\nmovq %rsp,%rdi\nmovb $7,(%rdi)\nmovl %ebx,%esi\n"
"pushq $0x13579\npushq %rbp\nmovq %rsp,%rbp\ncall observe\npopq %rbp\naddq $8,%rsp\nmovl %eax,-24(%rbp)\n"
"leaq -32(%rbp),%r10\nmovl %ebx,%eax\naddq $15,%rax\nandq $-16,%rax\nsubq %rax,%r10\nmovq %r10,%rsp\nmovq %r10,%rdi\nmovb $7,(%rdi)\nmovl %ebx,%esi\n"
"pushq $0x2468a\npushq %rbp\nmovq %rsp,%rbp\ncall observe\npopq %rbp\naddq $8,%rsp\naddl -24(%rbp),%eax\naddl -16(%rbp),%eax\n"
"leaq -8(%rbp),%rsp\npopq %rbx\npopq %rbp\nretq\n");"#,
        Case::Displaced => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\nmovl %edi,%esi\nmovl %edi,%eax\naddq $31,%rax\nandq $-16,%rax\n"
"movq %rsp,%r10\nmovq %r10,%rdi\nsubq %rax,%rdi\nmovq %rdi,%rsp\nnegq %rax\nmovb $7,16(%r10,%rax,1)\n"
"addq $16,%rdi\ncall observe\nmovq %rbp,%rsp\npopq %rbp\nretq\n");"#,
        Case::SteppedProbe => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\nmovl %edi,%esi\nmovslq %edi,%rax\naddq $15,%rax\nmovq %rax,%rcx\nandq $-4096,%rax\nandq $-16,%rcx\n"
"movq %rsp,%rdx\nsubq %rax,%rdx\n1: cmpq %rdx,%rsp\nje 2f\nsubq $4096,%rsp\norq $0,4088(%rsp)\njmp 1b\n"
"2: andl $4095,%ecx\nsubq %rcx,%rsp\nmovq %rsp,%rdi\nmovb $7,(%rdi)\ncall observe\nmovq %rbp,%rsp\npopq %rbp\nretq\n");"#,
        Case::MergedSteppedProbe => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\nmovl %edi,%esi\nmovl %edi,%eax\naddq $15,%rax\nmovq %rax,%rcx\nandq $-4096,%rax\nandq $-16,%rcx\n"
"movq %rsp,%rdx\nsubq %rax,%rdx\nxorl %r8d,%r8d\n1: xorl $1,%r8d\ntestl %r8d,%r8d\njnz 2f\nsubq $4096,%rsp\norq $0,4088(%rsp)\n"
"2: cmpq %rdx,%rsp\njne 1b\nandl $4095,%ecx\nsubq %rcx,%rsp\nmovq %rsp,%rdi\nmovb $7,(%rdi)\ncall observe\nmovq %rbp,%rsp\npopq %rbp\nretq\n");"#,
        Case::Drifting => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\nmovl %edi,%esi\nmovl %edi,%eax\naddq $15,%rax\nandq $-16,%rax\nsubq %rax,%rsp\nmovq %rsp,%rdi\nmovb $7,(%rdi)\nmovl %esi,%ecx\n"
"1: subq $16,%rsp\ndecl %ecx\njnz 1b\nmovq %rsp,%rdi\nmovb $7,(%rdi)\ncall observe\nmovq %rbp,%rsp\npopq %rbp\nretq\n");"#,
        Case::ProbeTwo if target.is_some() => r#"__asm__(
"pushl %ebp\nmovl %esp,%ebp\npushl %ebx\npushl %esi\npushl %edi\nmovl 8(%ebp),%ebx\n"
"movl %esp,%edi\nmovl %ebx,%eax\naddl $15,%eax\nandl $-16,%eax\n"
"1: cmpl $4096,%eax\njbe 2f\nsubl $4096,%edi\nsubl $4096,%eax\ntestl %eax,(%edi)\njmp 1b\n"
"2: subl %eax,%edi\ntestl %eax,(%edi)\nmovl %edi,%esp\nmovl %edi,%esi\n"
"movl %esp,%edi\nmovl %ebx,%eax\naddl $15,%eax\nandl $-16,%eax\n"
"3: cmpl $4096,%eax\njbe 4f\nsubl $4096,%edi\nsubl $4096,%eax\ntestl %eax,(%edi)\njmp 3b\n"
"4: subl %eax,%edi\ntestl %eax,(%edi)\nmovl %edi,%esp\nmovb $3,(%esi)\nmovb $7,(%edi)\n"
"pushl %ebx\npushl %esi\ncalll _observe\naddl $8,%esp\nmovl %eax,%esi\npushl %ebx\npushl %edi\ncalll _observe\naddl $8,%esp\naddl %esi,%eax\n"
"leal -12(%ebp),%esp\npopl %edi\npopl %esi\npopl %ebx\npopl %ebp\nretl\n");"#,
        Case::ProbeTwo => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\npushq %rbx\npushq %r12\npushq %r13\nsubq $8,%rsp\nmovl %edi,%ebx\n"
"movq %rsp,%r10\nmovl %ebx,%eax\naddq $15,%rax\nandq $-16,%rax\n"
"1: cmpq $4096,%rax\njbe 2f\nsubq $4096,%r10\nsubq $4096,%rax\ntestl %eax,(%r10)\njmp 1b\n"
"2: subq %rax,%r10\ntestl %eax,(%r10)\nmovq %r10,%rsp\nmovq %r10,%r12\n"
"movq %rsp,%r10\nmovl %ebx,%eax\naddq $15,%rax\nandq $-16,%rax\n"
"3: cmpq $4096,%rax\njbe 4f\nsubq $4096,%r10\nsubq $4096,%rax\ntestl %eax,(%r10)\njmp 3b\n"
"4: subq %rax,%r10\ntestl %eax,(%r10)\nmovq %r10,%rsp\nmovq %r10,%r13\nmovb $3,(%r12)\nmovb $7,(%r13)\n"
"movq %r12,%rdi\nmovl %ebx,%esi\ncall observe\nmovl %eax,%r12d\nmovq %r13,%rdi\nmovl %ebx,%esi\ncall observe\naddl %r12d,%eax\n"
"leaq -24(%rbp),%rsp\npopq %r13\npopq %r12\npopq %rbx\npopq %rbp\nretq\n");"#,
        Case::Probe if target.is_some() => r#"__asm__(
"pushl %ebp\nmovl %esp,%ebp\npushl %ebx\npushl %esi\npushl %edi\n"
"movl 8(%ebp),%edx\nmovl %esp,%edi\nmovl %edx,%eax\naddl $15,%eax\nandl $-16,%eax\n"
"cmpl $4096,%eax\njbe 2f\n1: subl $4096,%edi\nsubl $4096,%eax\ntestl %eax,(%edi)\ncmpl $4096,%eax\nja 1b\n"
"2: subl %eax,%edi\ntestl %eax,(%edi)\nmovl %edi,%esp\nmovb $7,(%edi)\n"
"pushl %edx\npushl %edi\ncalll _observe\naddl $8,%esp\n"
"leal -12(%ebp),%esp\npopl %edi\npopl %esi\npopl %ebx\npopl %ebp\nretl\n");"#,
        Case::Probe => r#"__asm__(
"pushq %rbp\nmovq %rsp,%rbp\npushq %rbx\npushq %r12\npushq %r13\nsubq $8,%rsp\n"
"movl %edi,%edx\nmovq %rsp,%r10\nmovl %edx,%eax\naddq $15,%rax\nandq $-16,%rax\n"
"cmpq $4096,%rax\njbe 2f\n1: subq $4096,%r10\nsubq $4096,%rax\ntestl %eax,(%r10)\ncmpq $4096,%rax\nja 1b\n"
"2: subq %rax,%r10\ntestl %eax,(%r10)\nmovq %r10,%rsp\nmovb $7,(%r10)\nmovq %r10,%rdi\nmovl %edx,%esi\ncall observe\n"
"leaq -24(%rbp),%rsp\npopq %r13\npopq %r12\npopq %rbx\npopq %rbp\nretq\n");"#,
    };
    let naked = if matches!(
        case,
        Case::Probe
            | Case::ProbeTwo
            | Case::Drifting
            | Case::Restored
            | Case::Displaced
            | Case::SteppedProbe
            | Case::MergedSteppedProbe
    ) {
        "__attribute__((naked)) "
    } else {
        ""
    };
    std::fs::write(&source, format!(r#"
#ifdef _WIN32
#define API __declspec(dllexport) __attribute__((noinline))
#else
#define API __attribute__((visibility("default"),noinline))
#endif
API int observe(char *data,int n) {{data[n-1]=9;return (unsigned char)data[0]+(unsigned char)data[n-1];}}
API {naked}int allocate(int n) {{{body}}}
"#)).unwrap();
    let mut build = Command::new("clang");
    build.args([optimization, "-shared", "-nostdlib"]);
    if let Some(target) = target {
        build
            .arg(format!("--target={target}"))
            .args(["-mno-stack-arg-probe", "-Wl,--entry,0"]);
    } else {
        build.args(["-fPIC", "-Wl,-Bsymbolic"]);
    }
    process::required_output(build.arg(&source).arg("-o").arg(&binary));
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let allocate = address(b"allocate");
    let observe = address(b"observe");
    let convention = if target.is_some() { "__cdecl " } else { "" };
    let project = root.join("project");
    let mut decompile = Command::new(env!("CARGO_BIN_EXE_kuna"));
    decompile
        .arg("decompile-project")
        .arg(&binary)
        .args([
            "--addr",
            &format!("0x{allocate:x}"),
            "--assert-strict",
            "--option",
            "stackalloc",
            "on",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"))
        .args([
            "--assert",
            &format!("prototype 0x{allocate:x} int {convention}allocate(int n)"),
            "--assert",
            &format!("prototype 0x{observe:x} int {convention}observe(char *data,int n)"),
        ])
        .arg("-o")
        .arg(&project);
    process::required_output(&mut decompile);
    let generated = project.join(if target.is_some() {
        "sample.dll.c"
    } else {
        "sample.so.c"
    });
    if matches!(case, Case::Restored) {
        map_restored_frame(
            &binary,
            allocate,
            observe,
            &root,
            &generated,
            target.is_some(),
        );
    }
    let text = std::fs::read_to_string(&generated).unwrap();
    let declined = matches!(
        case,
        Case::Drifting | Case::SteppedProbe | Case::MergedSteppedProbe | Case::Scoped
    );
    let allocations = if declined {
        0
    } else if matches!(
        case,
        Case::Two | Case::Conditional | Case::ProbeTwo | Case::Restored
    ) {
        2
    } else {
        1
    };
    let actual = text.matches("__builtin_alloca(").count();
    if matches!(case, Case::Restored) {
        // These lifetimes do not overlap: sharing proved identical storage is valid.
        assert!((1..=2).contains(&actual), "{text}");
    } else {
        assert_eq!(actual, allocations, "{text}");
    }
    if declined {
        return;
    }
    assert!(!text.contains("Stackffff"), "{text}");
    if let Some(target) = target {
        for opt in ["-O0", "-O2"] {
            process::required_output(
                Command::new("clang")
                    .arg(format!("--target={target}"))
                    .args(["-std=gnu11", "-Werror", opt, "-c"])
                    .arg(&generated)
                    .arg("-o")
                    .arg(root.join("generated.o")),
            );
        }
    } else {
        let driver = root.join("driver.c");
        let expected = match case {
            Case::Two | Case::ProbeTwo => "n==1?36:28",
            Case::Restored => "n==1?67:63",
            Case::Repeated | Case::Scoped => "n==1?50:48",
            _ => "n==1?18:16",
        };
        let limit = if matches!(case, Case::Probe | Case::ProbeTwo | Case::Drifting) {
            16384
        } else {
            4096
        };
        std::fs::write(&driver, format!(r#"
int allocate(int);int regenerated_allocate(int);int observe(char*,int);
int regenerated_observe(char *p,int n) {{return observe(p,n);}}
int main(void) {{for(int n=1;n<={limit};n++) {{int reference=allocate(n);if(reference!=({expected})||regenerated_allocate(n)!=reference)return 1;}}return 0;}}
"#)).unwrap();
        let reference = root.join("reference.o");
        process::required_output(
            Command::new("clang")
                .arg(optimization)
                .arg("-c")
                .arg(&source)
                .arg("-o")
                .arg(&reference),
        );
        for opt in ["-O0", "-O2"] {
            let object = root.join("generated.o");
            process::required_output(
                Command::new("clang")
                    .args([
                        "-std=gnu11",
                        "-Werror",
                        opt,
                        "-Dallocate=regenerated_allocate",
                        "-Dobserve=regenerated_observe",
                        "-c",
                    ])
                    .arg(&generated)
                    .arg("-o")
                    .arg(&object),
            );
            let executable = root.join("check");
            process::required_output(
                Command::new("clang")
                    .arg(opt)
                    .arg(&driver)
                    .arg(&object)
                    .arg(&reference)
                    .arg("-o")
                    .arg(&executable),
            );
            process::required_output(&mut Command::new(&executable));
        }
    }
}

fn map_restored_frame(
    binary: &std::path::Path,
    allocate: u64,
    observe: u64,
    root: &std::path::Path,
    generated: &std::path::Path,
    windows: bool,
) {
    let mapped = root.join("mapped.c");
    let engine = std::env::var_os("KUNA_DECOMP_DBG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_BIN_EXE_kuna"))
                .parent()
                .unwrap()
                .join("decomp_dbg")
        });
    // Map the complete fixed footprint declared by the authored assembly.
    let (storage, size) = if windows {
        ("s0xffffffec", 20)
    } else {
        ("s0xffffffffffffffd8", 40)
    };
    let convention = if windows { "__cdecl " } else { "" };
    let script=format!("load file \"{}\"\nread symbols\noption ctypes on\noption partialaccess on\noption stackalloc on\nmap prototype 0x{allocate:x} int {convention}allocate(int n);\nmap prototype 0x{observe:x} int {convention}observe(char *data,int n);\nload addr 0x{allocate:x}\nmap address {storage} unsigned char frame[{size}]\ndecompile\nopenfile write \"{}\"\nprint C\nclosefile\nquit\n",binary.display(),mapped.display());
    let mut child = Command::new(engine)
        .arg("-s")
        .arg(common::repo_root().join("specs"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!text.contains("Execution error:"), "{text}");
    std::fs::write(
        generated,
        format!(
            "#include \"{}\"\n{}",
            if windows {
                "sample.dll.h"
            } else {
                "sample.so.h"
            },
            std::fs::read_to_string(mapped).unwrap()
        ),
    )
    .unwrap();
}

#[test]
fn elf64_unasserted_allocation_keeps_call_arity() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("unasserted-stack-allocation", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let object = root.join("sample.o");
    let binary = root.join("sample.so");
    std::fs::write(
        &source,
        r#"volatile unsigned observed_calls,observed_value;
__attribute__((noinline)) int observe(char *p,int n) {
    volatile unsigned char clobber[8192];
    for(unsigned i=0;i<sizeof clobber;i++)clobber[i]=(unsigned char)(i+(unsigned)n);
    p[n-1]=9;
    int value=(unsigned char)p[0]+(unsigned char)p[n-1];
    observed_calls++;
    observed_value=(unsigned)value;
    return value;
}
int allocate(int n){char*p=__builtin_alloca((unsigned)n);p[0]=7;return observe(p,n);}
"#,
    )
    .unwrap();
    process::required_output(
        Command::new("clang")
            .args(["-O2", "-fPIC", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object),
    );
    process::required_output(
        Command::new("clang")
            .args(["-shared", "-nostdlib", "-Wl,-Bsymbolic"])
            .arg(&object)
            .arg("-o")
            .arg(&binary),
    );
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile")
            .arg(&binary)
            .arg("allocate")
            .args(["--json", "--option", "stackalloc", "on", "--sleighpath"])
            .arg(common::repo_root().join("specs")),
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"] == "allocate")
        .unwrap();
    assert!(function["error"].is_null(), "{function}");
    let code = function["code"].as_str().unwrap();
    assert!(code.contains("__builtin_alloca("), "{code}");
    let consumer = root.join("consumer.c");
    std::fs::write(
        &consumer,
        format!(
            "#include <stddef.h>\n#include <stdint.h>\nint observe(char *,int);\nint allocate(int);\nextern volatile unsigned observed_calls,observed_value;\n{}\nint main(void){{for(unsigned n=1;n<=513;n++){{observed_calls=0;observed_value=0;int reference=allocate((int)n);unsigned calls=observed_calls,value=observed_value;observed_calls=0;observed_value=0;regen_allocate(n);if(reference!=(n==1?18:16)||calls!=1||calls!=observed_calls||value!=observed_value)return 1;}}return 0;}}\n",
            code.replacen("allocate(", "regen_allocate(", 1)
        ),
    )
    .unwrap();
    for optimization in ["-O0", "-O2"] {
        let executable = root.join(optimization.trim_start_matches('-'));
        process::required_output(
            Command::new("clang")
                .arg(optimization)
                .arg(&consumer)
                .arg(&object)
                .arg("-o")
                .arg(&executable),
        );
        process::required_output(&mut Command::new(&executable));
    }
    eprintln!(
        "1026 native/regenerated stack-clobbering helper comparisons passed ({})",
        root.display()
    );
}
