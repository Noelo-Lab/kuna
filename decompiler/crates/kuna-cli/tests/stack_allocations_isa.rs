//! Cross-ISA stack allocation: recompile on the original ABI and compare under QEMU.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use std::path::Path;
use std::process::Command;

const CASES: &[&str] = &["one", "two", "branch", "loop", "aligned", "outgoing"];
struct Target {
    triple: &'static str,
    emulator: &'static str,
    flags: &'static [&'static str],
    start: &'static str,
    sparc32: bool,
}
macro_rules! target_test {
    ($name:ident,$triple:literal,$emulator:literal,$flags:expr,$start:literal,$sparc32:literal) => {
        #[test]
        fn $name() {
            check(Target {
                triple: $triple,
                emulator: $emulator,
                flags: $flags,
                start: $start,
                sparc32: $sparc32,
            });
        }
    };
}
target_test!(
    x86,
    "i686-linux-gnu",
    "qemu-i386",
    &[],
    "call check; mov %eax,%ebx; mov $1,%eax; int $0x80",
    false
);
target_test!(
    amd64,
    "x86_64-linux-gnu",
    "qemu-x86_64",
    &[],
    "call check; mov %eax,%edi; mov $60,%eax; syscall",
    false
);
target_test!(
    arm,
    "armv7-linux-gnueabihf",
    "qemu-arm",
    &["-marm"],
    "bl check; mov r7,#1; svc #0",
    false
);
target_test!(
    aarch64,
    "aarch64-linux-gnu",
    "qemu-aarch64",
    &[],
    "bl check; mov x8,#93; svc #0",
    false
);
target_test!(
    riscv32,
    "riscv32-linux-gnu",
    "qemu-riscv32",
    &["-march=rv32gc", "-mabi=ilp32d"],
    "call check; li a7,93; ecall",
    false
);
target_test!(
    riscv64,
    "riscv64-linux-gnu",
    "qemu-riscv64",
    &["-march=rv64gc", "-mabi=lp64d"],
    "call check; li a7,93; ecall",
    false
);
target_test!(
    sparc32,
    "sparc-linux-gnu",
    "qemu-sparc32plus",
    &[],
    "call check; nop; mov 1,%g1; ta 0x10",
    true
);
target_test!(
    sparc64,
    "sparcv9-linux-gnu",
    "qemu-sparc64",
    &[],
    "call check; nop; mov 1,%g1; ta 0x6d",
    false
);

fn drifting(target: &Target) -> &'static str {
    match target.triple {
        "i686-linux-gnu" => "push %ebp; mov %esp,%ebp; mov 8(%ebp),%eax; sub %eax,%esp; 1: sub $16,%esp; dec %eax; jne 1b; mov %ebp,%esp; pop %ebp; xor %eax,%eax; ret",
        "x86_64-linux-gnu" => "push %rbp; mov %rsp,%rbp; mov %edi,%eax; sub %rax,%rsp; 1: sub $16,%rsp; dec %eax; jne 1b; mov %rbp,%rsp; pop %rbp; xor %eax,%eax; ret",
        "armv7-linux-gnueabihf" => "push {r4,lr}; mov r4,sp; sub sp,sp,r0; 1: sub sp,sp,#16; subs r0,r0,#1; bne 1b; mov sp,r4; pop {r4,pc}",
        "aarch64-linux-gnu" => "stp x29,x30,[sp,#-16]!; mov x29,sp; sub sp,sp,x0; 1: sub sp,sp,#16; subs w0,w0,#1; b.ne 1b; mov sp,x29; ldp x29,x30,[sp],#16; mov w0,#0; ret",
        "riscv32-linux-gnu" => "addi sp,sp,-16; sw s0,12(sp); mv s0,sp; sub sp,sp,a0; 1: addi sp,sp,-16; addi a0,a0,-1; bnez a0,1b; mv sp,s0; lw s0,12(sp); addi sp,sp,16; li a0,0; ret",
        "riscv64-linux-gnu" => "addi sp,sp,-16; sd s0,8(sp); mv s0,sp; sub sp,sp,a0; 1: addi sp,sp,-16; addi a0,a0,-1; bnez a0,1b; mv sp,s0; ld s0,8(sp); addi sp,sp,16; li a0,0; ret",
        "sparc-linux-gnu" => "save %sp,-96,%sp; sub %sp,%i0,%sp; 1: sub %sp,16,%sp; subcc %i0,1,%i0; bne 1b; nop; ret; restore",
        "sparcv9-linux-gnu" => "save %sp,-176,%sp; sub %sp,%i0,%sp; 1: sub %sp,16,%sp; subcc %i0,1,%i0; bne 1b; nop; ret; restore",
        _ => unreachable!(),
    }
}

fn compiler(target: &Target) -> Command {
    let mut c = Command::new("clang");
    c.arg(format!("--target={}", target.triple))
        .args(target.flags)
        .args([
            "-ffreestanding",
            "-fno-builtin",
            "-fno-stack-protector",
            "-fno-pic",
        ]);
    c
}
fn compile(target: &Target, source: &Path, output: &Path, opt: &str, generated: bool) {
    let mut cmd = compiler(target);
    cmd.arg(opt).args([
        "-Werror=int-conversion",
        "-Werror=incompatible-pointer-types",
    ]);
    if generated {
        for name in CASES {
            cmd.arg(format!("-Dsample_{name}=regen_sample_{name}"));
        }
    }
    process::required_output(cmd.arg("-c").arg(source).arg("-o").arg(output));
}
fn link(target: &Target, linker: &std::ffi::OsStr, objects: &[&Path], output: &Path) {
    let mut cmd = if target.sparc32 {
        let mut cmd = Command::new(linker);
        cmd.args(["-m", "elf32_sparc", "-e", "_start"]);
        cmd
    } else {
        let mut cmd = compiler(target);
        cmd.args(["-nostdlib", "-static", "-fuse-ld=lld", "-Wl,-e,_start"]);
        cmd
    };
    process::required_output(cmd.args(objects).arg("-o").arg(output));
}
fn check(target: Target) {
    let linker =
        std::env::var_os("KUNA_TEST_SPARC_LD").unwrap_or_else(|| "sparc64-linux-gnu-ld".into());
    for tool in [
        std::ffi::OsStr::new("clang"),
        if target.sparc32 {
            linker.as_os_str()
        } else {
            std::ffi::OsStr::new("ld.lld")
        },
    ] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            assert!(
                std::env::var_os("KUNA_REQUIRE_STACKALLOC_ISA").is_none(),
                "required tool missing: {}",
                tool.to_string_lossy()
            );
            eprintln!(
                "skipping {}: missing {}",
                target.triple,
                tool.to_string_lossy()
            );
            return;
        }
    }
    let emulate =
        process::optional_output(Command::new(target.emulator).arg("--version")).is_some();
    if !emulate {
        assert!(
            std::env::var_os("KUNA_REQUIRE_STACKALLOC_ISA").is_none(),
            "required emulator missing: {}",
            target.emulator
        );
        eprintln!(
            "{}: QEMU unavailable; retaining compile/decompile checks",
            target.triple
        );
    }
    let root = common::scratch_file("stackalloc-isa", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let cases = root.join("cases.c");
    let observer = root.join("observer.c");
    let native_driver = root.join("native-driver.c");
    let driver = root.join("driver.c");
    let start = root.join("start.s");
    std::fs::write(
        &cases,
        include_str!("fixtures/stack_allocations_isa/cases.c"),
    )
    .unwrap();
    std::fs::write(
        &observer,
        include_str!("fixtures/stack_allocations_isa/observer.c"),
    )
    .unwrap();
    std::fs::write(&native_driver, "unsigned sample_one(unsigned);\nint check(void){for(unsigned n=1;n<=513;n++)if(sample_one(n)!=(n==1?18:16))return 1;return 0;}\n").unwrap();
    let descending = drifting(&target);
    let ascending = descending
        .replace("1: sub $16", "1: add $16")
        .replace("1: sub sp,sp,#16", "1: add sp,sp,#16")
        .replace("1: addi sp,sp,-16", "1: addi sp,sp,16")
        .replace("1: sub %sp,16,%sp", "1: add %sp,16,%sp");
    let mut assembly = format!(".text\n.global _start\n_start:\n{}\n", target.start);
    let kind = if target.triple.starts_with("arm") {
        "%"
    } else {
        "@"
    };
    for (name, body) in [
        ("sample_drifting", descending),
        ("sample_rising", ascending.as_str()),
    ] {
        assembly+=&format!(".global {name}\n.type {name},{kind}function\n{name}:\n{body}\n.size {name}, .-{name}\n");
    }
    std::fs::write(&start, assembly.replace(';', "\n")).unwrap();
    let declarations: String = CASES
        .iter()
        .map(|name| {
            format!("unsigned sample_{name}(unsigned);unsigned regen_sample_{name}(unsigned);\n")
        })
        .collect();
    let checks: String = CASES
        .iter()
        .enumerate()
        .map(|(i, name)| {
            format!(
                "observed_calls=0;observed_digest=1;reference=sample_{name}(n);calls=observed_calls;digest=observed_digest;observed_calls=0;observed_digest=1;actual=regen_sample_{name}(n);if(reference!=actual||calls!=observed_calls||digest!=observed_digest)return {};\n",
                i + 1
            )
        })
        .collect();
    std::fs::write(&driver, format!("{declarations}\nextern unsigned observed_calls,observed_digest;\nint check(void){{unsigned reference,actual,calls,digest;for(unsigned n=1;n<=513;n++){{{checks}}}return 0;}}\n")).unwrap();
    let start_obj = root.join("start.o");
    let observe_obj = root.join("observer.o");
    let native_driver_obj = root.join("native-driver.o");
    let driver_obj = root.join("driver.o");
    compile(&target, &start, &start_obj, "-O0", false);
    compile(&target, &observer, &observe_obj, "-O2", false);
    compile(&target, &native_driver, &native_driver_obj, "-O2", false);
    compile(&target, &driver, &driver_obj, "-O2", false);
    for original_opt in ["-O0", "-O2"] {
        let native_obj = root.join("native.o");
        let binary = root.join("sample.elf");
        compile(&target, &cases, &native_obj, original_opt, false);
        link(
            &target,
            &linker,
            &[&start_obj, &native_obj, &observe_obj, &native_driver_obj],
            &binary,
        );
        if emulate {
            process::required_output(Command::new(target.emulator).arg(&binary));
        }
        for negative in ["sample_drifting", "sample_rising", "sample_scoped"] {
            if negative == "sample_scoped" && original_opt == "-O2" {
                continue;
            }
            let mut decline = Command::new(env!("CARGO_BIN_EXE_kuna"));
            decline
                .arg("decompile")
                .arg(&binary)
                .arg(negative)
                .args(["--option", "stackalloc", "on", "--sleighpath"])
                .arg(
                    std::env::var_os("SLEIGHHOME")
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| common::repo_root().join("specs")),
                )
                .args([
                    "--assert",
                    &format!("prototype {negative} unsigned int {negative}(unsigned int n)"),
                ]);
            if target.triple.starts_with("sparc") {
                decline.args(["--option", "noreturn_disc", "off"]);
            }
            let output = process::required_output(&mut decline);
            assert!(
                !String::from_utf8_lossy(&output.stdout).contains("__builtin_alloca("),
                "{}: {negative} must decline unsupported stack growth or lifetime: {decline:?}\n{}",
                target.triple, String::from_utf8_lossy(&output.stdout)
            );
        }

        for option in ["off", "on"] {
            let project = root.join(format!("{}-{option}", original_opt.trim_start_matches('-')));
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
            cmd.arg("decompile-project")
                .arg(&binary)
                .args([
                    "--functions",
                    &CASES
                        .iter()
                        .map(|n| format!("sample_{n}"))
                        .collect::<Vec<_>>()
                        .join(","),
                    "--option",
                    "stackalloc",
                    option,
                    "--assert-strict",
                    "--sleighpath",
                ])
                .arg(
                    std::env::var_os("SLEIGHHOME")
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| common::repo_root().join("specs")),
                )
                .arg("-o")
                .arg(&project);
            for name in CASES {
                cmd.args([
                    "--assert",
                    &format!("prototype sample_{name} unsigned int sample_{name}(unsigned int n)"),
                ]);
            }
            for prototype in [
                "prototype observe unsigned int observe(unsigned char *p,unsigned int n)",
                "prototype observe_aligned unsigned int observe_aligned(unsigned char *p,unsigned int n)",
                "prototype observe_many unsigned int observe_many(unsigned char *p,unsigned int n,unsigned int a,unsigned int b,unsigned int c,unsigned int d,unsigned int e,unsigned int f,unsigned int g,unsigned int h,unsigned int i,unsigned int j)",
            ] { cmd.args(["--assert", prototype]); }
            if target.triple.starts_with("sparc") {
                cmd.args(["--option", "noreturn_disc", "off"]);
            }
            process::required_output(&mut cmd);
            let generated = project.join("sample.elf.c");
            let text = std::fs::read_to_string(&generated).unwrap();
            if option == "off" {
                assert!(
                    !text.contains("__builtin_alloca("),
                    "{} {original_opt}: {text}",
                    target.triple
                );
                continue;
            }
            assert!(
                text.contains("__builtin_alloca("),
                "{} {original_opt}: {text}",
                target.triple
            );
            for generated_opt in ["-O0", "-O2"] {
                let generated_obj = root.join("generated.o");
                let executable = root.join("compare");
                compile(&target, &generated, &generated_obj, generated_opt, true);
                link(
                    &target,
                    &linker,
                    &[
                        &start_obj,
                        &native_obj,
                        &observe_obj,
                        &generated_obj,
                        &driver_obj,
                    ],
                    &executable,
                );
                if !emulate {
                    assert!(
                        std::env::var_os("KUNA_REQUIRE_STACKALLOC_ISA").is_none(),
                        "required emulator missing: {}",
                        target.emulator
                    );
                    continue;
                }
                let result = process::output_with_timeout(
                    Command::new(target.emulator).arg(&executable),
                    std::time::Duration::from_secs(60),
                    std::time::Duration::from_millis(50),
                )
                .expect("cross-ISA comparison timed out");
                assert!(result.status.success(), "{} original={original_opt} generated={generated_opt}; exit={:?}, case order={CASES:?}\n{}",target.triple,result.status.code(),String::from_utf8_lossy(&result.stderr));
            }
        }
    }
    if emulate {
        eprintln!(
            "{}: 12312 native/regenerated comparisons passed ({})",
            target.triple,
            root.display()
        );
    } else {
        eprintln!(
            "{}: four recompilation combinations passed; native execution skipped",
            target.triple
        );
    }
}

#[test]
fn arm_unasserted_call_before_allocation() {
    let target = Target {
        triple: "armv7-linux-gnueabihf",
        emulator: "qemu-arm",
        flags: &["-mthumb"],
        start: "bl check; mov r7,#1; svc #0",
        sparc32: false,
    };
    for tool in ["clang", "ld.lld"] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            assert!(
                std::env::var_os("KUNA_REQUIRE_STACKALLOC_ISA").is_none(),
                "required tool missing: {tool}"
            );
            return;
        }
    }
    let emulate =
        process::optional_output(Command::new(target.emulator).arg("--version")).is_some();
    assert!(
        emulate || std::env::var_os("KUNA_REQUIRE_STACKALLOC_ISA").is_none(),
        "required emulator missing: {}",
        target.emulator
    );
    let root = common::scratch_file("stackalloc-before-call", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let native = root.join("native.o");
    let start = root.join("start.s");
    let start_obj = root.join("start.o");
    let driver = root.join("driver.c");
    let driver_obj = root.join("driver.o");
    let binary = root.join("sample.elf");
    std::fs::write(
        &source,
        include_str!("fixtures/stack_allocations_isa/spill_before_vla.c"),
    )
    .unwrap();
    std::fs::write(
        &start,
        ".syntax unified\n.thumb\n.text\n.global _start\n.type _start,%function\n.thumb_func\n_start:\nbl check\nmov r7,#1\nsvc #0\n",
    )
    .unwrap();
    std::fs::write(
        &driver,
        "unsigned sample(unsigned);\nextern volatile unsigned call_count,ordered_digest;\nint check(void){for(unsigned n=1;n<=513;n++){call_count=0;ordered_digest=1;sample(n);if(call_count!=2||ordered_digest!=(33u+n*3u+123u)*33u+(n&255u)+n)return 1;}return 0;}\n",
    )
    .unwrap();
    compile(&target, &source, &native, "-O2", false);
    compile(&target, &start, &start_obj, "-O0", false);
    compile(&target, &driver, &driver_obj, "-O2", false);
    link(
        &target,
        std::ffi::OsStr::new("ld.lld"),
        &[&start_obj, &native, &driver_obj],
        &binary,
    );
    if emulate {
        process::required_output(Command::new(target.emulator).arg(&binary));
    }
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile")
            .arg(&binary)
            .arg("sample")
            .args(["--json", "--option", "stackalloc", "on", "--sleighpath"])
            .arg(
                std::env::var_os("SLEIGHHOME")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| common::repo_root().join("specs")),
            ),
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"] == "sample")
        .unwrap();
    assert!(function["error"].is_null(), "{function}");
    let code = function["code"].as_str().unwrap();
    assert!(code.contains("__builtin_alloca("), "{code}");
    let signature = code
        .lines()
        .find(|line| line.contains("sample("))
        .unwrap()
        .split("//")
        .next()
        .unwrap()
        .trim();
    let parameters = signature
        .split_once("sample(")
        .unwrap()
        .1
        .split_once(')')
        .unwrap()
        .0;
    let arguments = parameters
        .split(',')
        .enumerate()
        .map(|(index, _)| if index == 0 { "n" } else { "0" })
        .collect::<Vec<_>>()
        .join(",");
    let generated = root.join("generated.c");
    let generated_obj = root.join("generated.o");
    // The real three-argument declaration is a C consumer contract, not an
    // analysis assertion: a phantom fourth/fifth argument must fail to compile.
    std::fs::write(
        &generated,
        format!(
            "unsigned pre_three(unsigned,unsigned,unsigned);\nunsigned observe(unsigned char *,unsigned);\n{}\n",
            code.replacen("sample(", "regen_sample(", 1)
        ),
    )
    .unwrap();
    std::fs::write(
        &driver,
        format!(
            "{signature};\nunsigned sample(unsigned);\nextern volatile unsigned call_count,ordered_digest;\nint check(void){{for(unsigned n=1;n<=513;n++){{call_count=0;ordered_digest=1;sample(n);unsigned calls=call_count,digest=ordered_digest;call_count=0;ordered_digest=1;regen_sample({arguments});if(calls!=call_count||digest!=ordered_digest)return 1;}}return 0;}}\n",
            signature = signature.replacen("sample(", "regen_sample(", 1)
        ),
    )
    .unwrap();
    compile(&target, &driver, &driver_obj, "-O2", false);
    for optimization in ["-O0", "-O2"] {
        compile(&target, &generated, &generated_obj, optimization, false);
        let executable = root.join(format!("compare-{}", optimization.trim_start_matches('-')));
        link(
            &target,
            std::ffi::OsStr::new("ld.lld"),
            &[&start_obj, &native, &generated_obj, &driver_obj],
            &executable,
        );
        if emulate {
            process::required_output(Command::new(target.emulator).arg(&executable));
        }
    }
    if emulate {
        eprintln!(
            "{}: 1026 native/regenerated pre-allocation call-effect comparisons passed ({})",
            target.triple,
            root.display()
        );
    }
}
