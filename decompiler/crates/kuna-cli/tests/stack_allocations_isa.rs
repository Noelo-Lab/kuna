//! Cross-ISA stack allocation: recompile on the original ABI and compare under QEMU.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::{Object, ObjectSection, ObjectSymbol};
use serde_json::Value;
use std::path::Path;
use std::process::Command;

const CASES: &[&str] = &[
    "one", "two", "branch", "loop", "aligned", "outgoing",
    "vla_block", "vla_branch", "vla_loop_exit", "vla_early",
    "vla_variadic", "vla_escape", "vla_aligned", "vla_product",
    "vla_liveout", "vla_numeric_result", "vla_numeric_pair", "vla_signed_result",
    "vla_signed_branch", "vla_sibling", "vla_sibling_offset", "vla_sequential", "vla_reentry", "vla_pointer_result", "scoped",
];
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
target_test!(
    thumb, "armv7-linux-gnueabihf", "qemu-arm", &["-mthumb"],
    "bl check; movs r7,#1; svc #0", false
);
target_test!(
    ppc32, "powerpc-linux-gnu", "qemu-ppc", &[],
    "bl check; li 0,1; sc", false
);
target_test!(
    ppc64, "powerpc64-linux-gnu", "qemu-ppc64", &[],
    "lis 2,.TOC.@ha; addi 2,2,.TOC.@l; bl check; li 0,1; sc", false
);
target_test!(
    ppc64le, "powerpc64le-linux-gnu", "qemu-ppc64le", &[],
    "lis 2,.TOC.@ha; addi 2,2,.TOC.@l; bl check; li 0,1; sc", false
);
target_test!(
    mips, "mips-linux-gnu", "qemu-mips", &["-mno-abicalls", "-G0"],
    "jal check; move $a0,$v0; li $v0,4001; syscall", false
);
target_test!(
    mipsel, "mipsel-linux-gnu", "qemu-mipsel", &["-mno-abicalls", "-G0"],
    "jal check; move $a0,$v0; li $v0,4001; syscall", false
);
target_test!(
    mips64, "mips64-linux-gnuabi64", "qemu-mips64", &["-mno-abicalls", "-G0"],
    "jal check; move $a0,$v0; li $v0,5058; syscall", false
);
target_test!(
    mips64el, "mips64el-linux-gnuabi64", "qemu-mips64el", &["-mno-abicalls", "-G0"],
    "jal check; move $a0,$v0; li $v0,5058; syscall", false
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
        "powerpc-linux-gnu" | "powerpc64-linux-gnu" | "powerpc64le-linux-gnu" =>
            "mr 12,1; subf 1,3,1; 1: addi 1,1,-16; addic. 3,3,-1; bne 1b; mr 1,12; li 3,0; blr",
        "mips-linux-gnu" | "mipsel-linux-gnu" =>
            "addiu $sp,$sp,-8; sw $fp,4($sp); move $fp,$sp; subu $sp,$sp,$a0; 1: addiu $sp,$sp,-16; addiu $a0,$a0,-1; bnez $a0,1b; move $sp,$fp; lw $fp,4($sp); addiu $sp,$sp,8; move $v0,$zero; jr $ra",
        "mips64-linux-gnuabi64" | "mips64el-linux-gnuabi64" =>
            "daddiu $sp,$sp,-16; sd $fp,8($sp); move $fp,$sp; dsubu $sp,$sp,$a0; 1: daddiu $sp,$sp,-16; addiu $a0,$a0,-1; bnez $a0,1b; move $sp,$fp; ld $fp,8($sp); daddiu $sp,$sp,16; move $v0,$zero; jr $ra",
        _ => unreachable!(),
    }
}

// Match helper declarations to the bundled target C model, using a genuine
// address-width integer type in both the original source and its assertions.
fn address_word(target: &Target) -> &'static str {
    if target.triple.starts_with("sparcv9") { "unsigned long long" }
    else if target.triple.contains("64") { "unsigned long" }
    else { "unsigned int" }
}

fn assert_helper_contracts(cmd: &mut Command, image: &object::File<'_>, word: &str) {
    let local_entries = image.architecture() == object::Architecture::PowerPc64
        && matches!(image.flags(), object::FileFlags::Elf { e_flags, .. }
            if e_flags & object::elf::EF_PPC64_ABI == 2);
    let mut contract = |prototype: &str| {
        if local_entries {
            let (name, declaration) = prototype.strip_prefix("prototype ").unwrap().split_once(' ').unwrap();
            if let Some(symbol) = image.symbols().find(|symbol|
                symbol.is_definition() && symbol.name() == Ok(name))
            {
                if let object::SymbolFlags::Elf { st_other, .. } = symbol.flags() {
                    let encoded = (st_other & object::elf::STO_PPC64_LOCAL_MASK)
                        >> object::elf::STO_PPC64_LOCAL_BIT;
                    let offset = (1_u64 << encoded) & !3;
                    if offset != 0 {
                        // Same-TOC calls use this advertised ELFv2 entry, not the
                        // global entry. Assert the real call target's contract.
                        let address = symbol.address() + offset;
                        cmd.args(["--define-function", &format!("{address:#x}={name}"),
                            "--assert", &format!("prototype {address:#x} {declaration}")]);
                        return;
                    }
                }
            }
        }
        cmd.args(["--assert", prototype]);
    };
    for prototype in [
        "prototype observe unsigned int observe(unsigned char *p,unsigned int n)",
        "prototype observe_aligned unsigned int observe_aligned(unsigned char *p,unsigned int n)",
        "prototype observe_many unsigned int observe_many(unsigned char *p,unsigned int n,unsigned int a,unsigned int b,unsigned int c,unsigned int d,unsigned int e,unsigned int f,unsigned int g,unsigned int h,unsigned int i,unsigned int j)",
        "prototype escape_vla unsigned int escape_vla(unsigned char *p,unsigned int n)",
        "prototype vla_variadic unsigned int vla_variadic(unsigned int count,unsigned int a,unsigned int b,unsigned int c,unsigned int d,unsigned int e,unsigned int f,unsigned int g,unsigned int h,...)",
        "prototype finish_vla unsigned int finish_vla(unsigned int n)",
        "prototype mutate_delimiter void mutate_delimiter(void *p)",
        "prototype vla_register_variadic unsigned int vla_register_variadic(unsigned int count,...)",
        "prototype inspect_expired unsigned int inspect_expired(void *address,unsigned int n)",
        "prototype vla_identity void *vla_identity(void *p)",
        "prototype signed_vla_result int signed_vla_result(unsigned char *p,unsigned int n)",
    ] { contract(prototype); }
    contract(&format!("prototype vla_identity_bits {word} vla_identity_bits(void *p)"));
}

fn compiler(target: &Target) -> Command {
    let mut c = Command::new("clang");
    c.arg(format!("--target={}", target.triple))
        .arg(format!("-DKUNA_ADDRESS_BITS={}", address_word(target)))
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
        cmd.arg("-include")
            .arg(output.parent().unwrap().join("observer.h"));
        for name in CASES {
            cmd.arg(format!("-Dsample_{name}=regen_sample_{name}"));
        }
    }
    process::required_output(cmd.arg("-c").arg(source).arg("-o").arg(output));
}
fn link(target: &Target, linker: &std::ffi::OsStr, objects: &[&Path], output: &Path) {
    let mut cmd = if target.sparc32 || target.triple == "powerpc64-linux-gnu" {
        let mut cmd = Command::new(linker);
        cmd.args(["-m", if target.sparc32 { "elf32_sparc" } else { "elf64ppc" }, "-e", "_start"]);
        cmd
    } else {
        let mut cmd = compiler(target);
        cmd.args(["-nostdlib", "-static", "-fuse-ld=lld", "-Wl,-e,_start"]);
        cmd
    };
    process::required_output(cmd.args(objects).arg("-o").arg(output));
}
fn references(node: &Value, name: &str) -> bool {
    node["referencedDecl"]["name"].as_str() == Some(name)
        || node["inner"].as_array().is_some_and(|children|
            children.iter().any(|child| references(child, name)))
}

fn storage_contract(
    node: &Value, scope: Option<&Value>, depth: usize,
    scoped_ends: &mut Vec<(u64, u64, bool)>, observers: &mut Vec<u64>,
) -> bool {
    let compound = node["kind"] == "CompoundStmt";
    let scope = if compound { Some(node) } else { scope };
    let depth = depth + usize::from(compound);
    if node["kind"] == "CallExpr" && references(node, "finish_vla") {
        observers.push(node["range"]["begin"]["offset"].as_u64().unwrap());
    }
    if node["kind"] == "VarDecl" && node["type"]["qualType"].as_str().is_some_and(|ty|
        ty.split('[').skip(1).filter_map(|bound| bound.split_once(']').map(|(bound, _)| bound))
            // Clang canonicalizes constant array bounds to decimal integers.
            .any(|bound| bound.parse::<u64>().is_err()))
    {
        let range = &scope.expect("runtime storage outside a function body")["range"];
        scoped_ends.push((range["begin"]["offset"].as_u64().unwrap(),
            range["end"]["offset"].as_u64().unwrap(), depth > 1));
    }
    let mut alloca = node["kind"] == "CallExpr" && references(node, "__builtin_alloca");
    if let Some(children) = node["inner"].as_array() {
        for child in children {
            alloca |= storage_contract(child, scope, depth, scoped_ends, observers);
        }
    }
    alloca
}

fn assignments<'a>(node: &'a Value, out: &mut Vec<(&'a str, u64, &'a Value)>) {
    if node["kind"] == "BinaryOperator" && node["opcode"] == "=" {
        if let (Some(id), Some(offset), Some(rhs)) = (
            node["inner"][0]["referencedDecl"]["id"].as_str(),
            node["range"]["begin"]["offset"].as_u64(),
            node["inner"].as_array().and_then(|children| children.get(1)),
        ) { out.push((id, offset, rhs)); }
    }
    if let Some(children) = node["inner"].as_array() {
        for child in children { assignments(child, out); }
    }
}

fn captured_address(
    node: &Value, before: u64, start: u64, end: u64,
    definitions: &[(&str, u64, &Value)],
) -> Option<bool> {
    if node["kind"] == "DeclRefExpr" && node["referencedDecl"]["kind"] == "VarDecl" {
        let id = node["referencedDecl"]["id"].as_str()?;
        let ty = node["referencedDecl"]["type"]["qualType"].as_str()?;
        let definition = definitions.iter()
            .filter(|(variable, offset, _)| *variable == id && *offset < before)
            .max_by_key(|(_, offset, _)| *offset);
        if ty.contains('*') && definition.is_none_or(|(_, offset, _)| *offset <= end) {
            return None;
        }
        return match definition {
            Some((_, offset, expression)) if *offset > end =>
                captured_address(expression, *offset, start, end, definitions),
            Some((_, offset, _)) => Some(*offset > start && !ty.contains('*')),
            None => Some(false),
        };
    }
    let mut captured = false;
    if let Some(children) = node["inner"].as_array() {
        for child in children {
            captured |= captured_address(child, before, start, end, definitions)?;
        }
    }
    Some(captured)
}

fn check_expired_address(node: &Value, body: &Value, start: u64, end: u64) -> bool {
    if node["kind"] == "CallExpr" && references(node, "inspect_expired") {
        let offset = node["range"]["begin"]["offset"].as_u64().unwrap();
        assert!(offset > end, "the expired-address observer must run after the VLA closes");
        let mut definitions = Vec::new();
        assignments(body, &mut definitions);
        let argument = &node["inner"][1];
        assert_eq!(captured_address(argument, offset, start, end, &definitions), Some(true),
            "an expired pointer must be recreated from integer bits saved before the close: {argument}");
        return true;
    }
    node["inner"].as_array().is_some_and(|children| {
        let mut found = false;
        for child in children { found |= check_expired_address(child, body, start, end); }
        found
    })
}

fn function_body<'a>(ast: &'a Value, name: &str) -> &'a Value {
    ast["inner"].as_array().unwrap().iter()
        .filter(|node| node["kind"] == "FunctionDecl" && node["name"] == name)
        .filter_map(|node| node["inner"].as_array())
        .flat_map(|children| children.iter())
        .find(|node| node["kind"] == "CompoundStmt")
        .unwrap_or_else(|| panic!("missing generated body {name}"))
}


fn loop_contains_scope(node: &Value, start: u64, end: u64) -> bool {
    let is_loop = matches!(node["kind"].as_str(), Some("ForStmt" | "WhileStmt" | "DoStmt"));
    if is_loop && node["range"]["begin"]["offset"].as_u64().is_some_and(|begin| begin < start)
        && node["range"]["end"]["offset"].as_u64().is_some_and(|finish| end < finish)
    { return true; }
    node["inner"].as_array().is_some_and(|children|
        children.iter().any(|child| loop_contains_scope(child, start, end)))
}

fn check_storage_contract(target: &Target, source: &Path, observer_header: &Path) {
    let output = process::required_output(compiler(target)
        .args(["-Xclang", "-ast-dump=json", "-fsyntax-only", "-include"])
        .arg(observer_header).arg(source));
    let ast: Value = serde_json::from_slice(&output.stdout).unwrap();
    for name in CASES {
        let function_name = format!("sample_{name}");
        let body = function_body(&ast, &function_name);
        let mut scoped_ends = Vec::new();
        let mut observers = Vec::new();
        let alloca = storage_contract(body, None, 0, &mut scoped_ends, &mut observers);
        if (name.starts_with("vla_") || *name == "scoped") && *name != "vla_early" {
            assert!(!scoped_ends.is_empty(),
                "{}: {function_name} lost its block lifetime", target.triple);
        } else {
            assert!(alloca || !scoped_ends.is_empty(),
                "{}: {function_name} did not recover runtime storage", target.triple);
        }
        if matches!(*name, "vla_sequential" | "vla_sibling" | "vla_sibling_offset") {
            assert_eq!(scoped_ends.len(), 2, "{}: sequential objects lost their separate lifetimes", target.triple);
        }
        for (start, end, nested) in scoped_ends {
            if (name.starts_with("vla_") || *name == "scoped") && *name != "vla_early" {
                assert!(nested, "{}: {function_name} lost its nested lifetime", target.triple);
                if *name == "scoped" {
                    assert!(loop_contains_scope(body, start, end),
                        "{}: {function_name} kept its VLA live across iterations", target.triple);
                } else {
                    assert!(observers.iter().any(|&call| call > end),
                        "{}: {function_name} kept its VLA live through the later observer", target.triple);
                }
            }
            if matches!(*name, "vla_liveout" | "vla_numeric_result" | "vla_numeric_pair" | "vla_pointer_result") {
                assert!(check_expired_address(body, body, start, end));
            }
        }
    }
}

fn check(target: Target) {
    let ppc64_be = target.triple == "powerpc64-linux-gnu";
    let linker = std::env::var_os(if ppc64_be { "KUNA_TEST_PPC64_LD" } else { "KUNA_TEST_SPARC_LD" })
        .unwrap_or_else(|| if ppc64_be { "powerpc64-linux-gnu-ld.bfd".into() } else { "sparc64-linux-gnu-ld".into() });
    for tool in [
        std::ffi::OsStr::new("clang"),
        if target.sparc32 || ppc64_be {
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
    std::fs::write(
        root.join("observer.h"),
        include_str!("fixtures/stack_allocations_isa/observer.h"),
    )
    .unwrap();
    std::fs::write(&native_driver, concat!(
        "unsigned sample_one(unsigned),sample_vla_sequential(unsigned),sample_vla_reentry(unsigned);\n",
        "int check(void){for(unsigned n=0;n<=513;n++){",
        "if(n&&sample_one(n)!=(n==1?18u:16u))return 1;",
        "unsigned tail=(2*n)&255u;",
        "if(sample_vla_sequential(n)!=(n==0?18u:16u)+((n&63)==0?18u:16u)+tail)return 2;",
        "if(sample_vla_reentry(n)!=48u+(n==0?2u:0u)+tail)return 3;",
        "}return 0;}\n",
    )).unwrap();
    let descending = drifting(&target);
    let ascending = descending
        .replace("1: sub $16", "1: add $16")
        .replace("1: sub sp,sp,#16", "1: add sp,sp,#16")
        .replace("1: addi sp,sp,-16", "1: addi sp,sp,16")
        .replace("1: addi 1,1,-16", "1: addi 1,1,16")
        .replace("1: sub %sp,16,%sp", "1: add %sp,16,%sp")
        .replace("1: addiu $sp,$sp,-16", "1: addiu $sp,$sp,16")
        .replace("1: daddiu $sp,$sp,-16", "1: daddiu $sp,$sp,16");
    let mut assembly = if target.triple == "powerpc64-linux-gnu" {
        format!(".abiversion 1\n.section .opd,\"aw\"\n.align 3\n.global _start\n.type _start,@function\n_start:\n.quad .L_start,.TOC.@tocbase,0\n.text\n.L_start:\n{}\n", target.start)
    } else {
        format!(".text\n.global _start\n_start:\n{}\n", target.start)
    };
    if target.triple == "powerpc64le-linux-gnu" {
        assembly.insert_str(0, ".abiversion 2\n");
    }
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
            let nonzero = if *name == "scoped" { "n == 0 || n == 127 || n == 513" }
                else if name.starts_with("vla_") { "1" } else { "n != 0" };
            format!(
                "if({nonzero}){{observed_calls=0;observed_digest=1;reference=sample_{name}(n);calls=observed_calls;digest=observed_digest;observed_calls=0;observed_digest=1;actual=regen_sample_{name}(n);if(reference!=actual||calls!=observed_calls||digest!=observed_digest)return {};}}\n",
                i + 1
            )
        })
        .collect();
    std::fs::write(&driver, format!("{declarations}\nextern unsigned observed_calls,observed_digest;\nint check(void){{unsigned reference,actual,calls,digest;for(unsigned n=0;n<=513;n++){{{checks}}}return 0;}}\n")).unwrap();
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
        let bytes = std::fs::read(&binary).unwrap();
        let image = object::File::parse(bytes.as_slice()).unwrap();
        let word = address_word(&target);
        let escaped_address = image.symbols()
            .find(|symbol| symbol.name() == Ok("escaped_vla_address")).unwrap().address();
        for name in ["drifting", "rising"] {
            let negative = format!("sample_{name}");
            let mut decline = Command::new(env!("CARGO_BIN_EXE_kuna"));
            decline
                .arg("decompile")
                .arg(&binary)
                .arg(&negative)
                .args(["--json", "--option", "stackalloc", "on", "--sleighpath"])
                .arg(
                    std::env::var_os("SLEIGHHOME")
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| common::repo_root().join("specs")),
                )
                .args(["--assert",
                    &format!("prototype {negative} unsigned int {negative}(unsigned int n)")]);
            assert_helper_contracts(&mut decline, &image, word);
            if target.triple.starts_with("sparc") {
                decline.args(["--option", "noreturn_disc", "off"]);
            }
            // ppc64le's ELFv2 baseline is POWER8, whose `isel` the default
            // PowerPC:LE:64 language does not decode.
            if target.triple == "powerpc64le-linux-gnu" {
                decline.args(["--target", "PowerPC:LE:64:A2ALT"]);
            }
            let output = process::required_output(&mut decline);
            let document: Value = serde_json::from_slice(&output.stdout).unwrap();
            let function = document["functions"].as_array().unwrap().iter()
                .find(|function| function["name"] == negative)
                .unwrap_or_else(|| panic!("{}: missing {negative}: {document}", target.triple));
            assert!(function["error"].is_null(), "{function}");
            let code = function["code"].as_str().unwrap();
            let declined_source = root.join(format!("{negative}-{}.c", original_opt.trim_start_matches('-')));
            std::fs::write(&declined_source, code).unwrap();
            // A refusal may retain raw frame references that do not compile.
            // Inspect the recovery AST only for materialized runtime storage.
            let parsed = compiler(&target)
                .args(["-ferror-limit=0", "-Xclang", "-ast-dump=json", "-fsyntax-only", "-include"])
                .arg(root.join("observer.h")).arg(&declined_source).output().unwrap();
            let ast: Value = serde_json::from_slice(&parsed.stdout)
                .unwrap_or_else(|error| panic!("{negative}: {error}\n{}", String::from_utf8_lossy(&parsed.stderr)));
            let mut arrays = Vec::new();
            let mut observers = Vec::new();
            let alloca = storage_contract(function_body(&ast, &negative), None, 0, &mut arrays, &mut observers);
            assert!(!alloca && arrays.is_empty(),
                "{}: {negative} materialized unsupported runtime storage\n{code}", target.triple);
        }

        for (option, regional, fold) in [
            ("off", "off", "off"), ("on", "off", "off"), ("on", "on", "off"),
            ("on", "off", "on"), ("on", "on", "on"),
        ] {
            let project = root.join(format!("{}-{option}-{regional}-{fold}", original_opt.trim_start_matches('-')));
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
            cmd.args(["--option", "regionstructure", regional]);
            cmd.args(["--option", "foldcallret", fold]);
            cmd.args(["--assert", &format!(
                "data {escaped_address:#x} unsigned char *escaped_vla_address"
            )]);
            // The generated C links without the image's data: constants a
            // compiler loads from .rodata (an x86 vector load of four
            // arguments) must fold, not print as undefined globals.
            if let Some(rodata) = image.section_by_name(".rodata").filter(|s| s.size() != 0) {
                cmd.args(["--assert", &format!("readonly {:#x}+{:#x}", rodata.address(), rodata.size())]);
            }
            for name in CASES {
                cmd.args([
                    "--assert",
                    &format!("prototype sample_{name} unsigned int sample_{name}(unsigned int n)"),
                ]);
            }
            assert_helper_contracts(&mut cmd, &image, word);
            if target.triple.starts_with("sparc") {
                cmd.args(["--option", "noreturn_disc", "off"]);
            }
            // ppc64le's ELFv2 baseline is POWER8, whose `isel` the default
            // PowerPC:LE:64 language does not decode.
            if target.triple == "powerpc64le-linux-gnu" {
                cmd.args(["--target", "PowerPC:LE:64:A2ALT"]);
            }
            process::required_output(&mut cmd);
            let generated = project.join("sample.elf.c");
            if option == "off" {
                let parsed = compiler(&target)
                    .args(["-ferror-limit=0", "-Xclang", "-ast-dump=json", "-fsyntax-only", "-include"])
                    .arg(root.join("observer.h")).arg(&generated).output().unwrap();
                let ast: Value = serde_json::from_slice(&parsed.stdout)
                    .unwrap_or_else(|error| panic!("{error}\n{}", String::from_utf8_lossy(&parsed.stderr)));
                for name in CASES {
                    let function = format!("sample_{name}");
                    let mut arrays = Vec::new();
                    let mut observers = Vec::new();
                    let alloca = storage_contract(function_body(&ast, &function), None, 0,
                        &mut arrays, &mut observers);
                    assert!(!alloca && arrays.is_empty(),
                        "{} {original_opt}: {function} recovered storage with the option off", target.triple);
                }
                continue;
            }
            check_storage_contract(&target, &generated, &root.join("observer.h"));
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
            "{}: native/regenerated VLA and allocation comparisons passed ({})",
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
