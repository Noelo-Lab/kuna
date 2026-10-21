//! Runtime-sized stack buffers retain storage across ordinary calls.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

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
fn elf64_scoped_vla_roundtrip() {
    check_case(None, "-O0", Case::Scoped);
}
#[test]
fn pe32_scoped_vla() {
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
        Case::Scoped => "int total=0;for(int i=0;i<3;i++){char data[n+i];data[0]=7;total+=observe(data,n+i);}return total;",
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
    let naked = if matches!(case, Case::Probe) {
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
    let text = std::fs::read_to_string(&generated).unwrap();
    let allocations = if matches!(case, Case::Two | Case::Conditional) {
        2
    } else {
        1
    };
    assert_eq!(
        text.matches("__builtin_alloca(").count(),
        allocations,
        "{text}"
    );
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
            Case::Two => "n==1?36:28",
            Case::Repeated | Case::Scoped => "n==1?50:48",
            _ => "n==1?18:16",
        };
        let limit = if matches!(case, Case::Probe) {
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
