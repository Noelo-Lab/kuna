//! Execute a late dispatch to a callback and, on MIPS, to the incoming return.
use crate::common::{self, process};
use object::{Object, ObjectSection, ObjectSegment};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

const X86: &str = r#"
.text
.globl dispatch_probe
.type dispatch_probe,@function
dispatch_probe:
    mov (%rsp),%r8
    mov $.Lstep,%eax
.Ldispatch:
    jmp *%rax
.Lstep:
    mov %edi,(%rsi)
    mov %rdx,%rax
    test %edi,%edi
    jne .Ldispatch
    add $8,%rsp
    mov %r8,%rax
    jmp .Ldispatch
.size dispatch_probe,.-dispatch_probe
"#;

// The lifted code reads the original table by its linked address. Keep the
// table bytes at that address in the replay, with all native cases unmapped.
const INDEXED: &str = r#"
.section .original_dispatch,"ax"
.globl indexed_original
.type indexed_original,@function
indexed_original:
    cmp $1,%edi
    ja .Ldone
    mov %edi,%eax
.Lload:
    mov .Ltable(,%rax,8),%rax
.Ldispatch:
    jmp *%rax
.Lfirst:
    mov %edi,(%rsi)
.Ldone:
    ret
.Lsecond:
    mov %edi,(%rsi)
    mov %rdx,%rax
    jmp .Ldispatch
.Lthird:
    movl $23,(%rsi)
    ret
.size indexed_original,.-indexed_original
.section .dispatch_cases,"a"
.balign 8
.Ltable:
    .quad .Lfirst,.Lsecond,.Lthird
"#;

const MIPS: &str = r#"
.set noreorder
.text
.globl dispatch_probe
.type dispatch_probe,@function
dispatch_probe:
    lui $9,%hi(.Lstep)
    addiu $9,$9,%lo(.Lstep)
.Ldispatch:
    jr $9
    nop
.Lstep:
    sw $4,0($5)
    beq $4,$0,.Ldispatch
    move $9,$31
    b .Ldispatch
    move $9,$6
.size dispatch_probe,.-dispatch_probe
"#;

const DRIVER: &str = r#"
extern void dispatch_probe(int, int *, void (*)(void));
static int calls, slots[4] = {12, -1, 0, 34};
static void callback(void) { ++calls; slots[2] = 77; }
int check(void) {
    if (INDEXED_CASE) {
        dispatch_probe(2, slots+1, callback);
        if (calls || slots[1] != -1 || slots[2]) return 1;
    }
    dispatch_probe(0, slots+1, callback);
    if (calls != FIRST_CALLS || slots[1] || slots[2] != (FIRST_CALLS ? 77 : 0)) return 2;
    dispatch_probe(SECOND_INPUT, slots+1, callback);
    return calls != FINAL_CALLS || slots[0] != 12 || slots[1] != FINAL_STORE ||
           slots[2] != (FINAL_CALLS ? 77 : 0) || slots[3] != 34;
}
"#;

// Copy the linked entries, including their relocations' final values. None
// of the original implementation is linked into the replay executable.
fn copy_table(image: &Path, assembly: &Path) -> Vec<u64> {
    let bytes = std::fs::read(image).unwrap();
    let elf = object::File::parse(&*bytes).unwrap();
    let section = elf.section_by_name(".dispatch_cases").unwrap();
    assert_eq!(section.address(), 0x500000);
    let rows: Vec<_> = section
        .data()
        .unwrap()
        .chunks_exact(8)
        .map(|row| u64::from_le_bytes(row.try_into().unwrap()))
        .collect();
    assert_eq!(rows.len(), 3);
    let mut text = String::from(".section .dispatch_cases,\"a\"\n.globl dat_500000\ndat_500000:\n");
    for target in &rows {
        text.push_str(&format!(".quad {target:#x}\n"));
    }
    std::fs::write(assembly, text).unwrap();
    rows
}

fn assert_native_cases_unmapped(image: &Path, rows: &[u64]) {
    let bytes = std::fs::read(image).unwrap();
    let elf = object::File::parse(&*bytes).unwrap();
    for target in rows {
        assert!(
            elf.segments()
                .all(|seg| !(seg.address()..seg.address() + seg.size()).contains(target)),
            "replay maps original case {target:#x}"
        );
    }
}

fn run_image(image: &Path, emulator: Option<&str>) {
    let mut command = match emulator {
        Some(tool) => {
            let mut cmd = Command::new(tool);
            cmd.arg(image);
            cmd
        }
        None => Command::new(image),
    };
    let output = process::output_with_timeout(
        &mut command,
        Duration::from_secs(5),
        Duration::from_millis(10),
    )
    .expect("dispatch did not return");
    assert!(
        output.status.success(),
        "{command:?} exited with {}",
        output.status
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn a_reentered_dispatch_retains_its_indirect_exit() {
    let installed =
        |tool: &&str| process::optional_output(Command::new(tool).arg("--version")).is_some();
    let Some(clang) = ["clang", "clang-21"].into_iter().find(installed) else {
        return;
    };
    let Some(linker) = ["ld.lld", "ld.lld-21"].into_iter().find(installed) else {
        return;
    };
    let linker = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join(linker))
        .find(|file| file.is_file())
        .unwrap();
    let disjoint = X86.replace("mov (%rsp),%r8", "mov %edi,-4(%rsp)\n    mov (%rsp),%r8");
    let pushed = X86.replace(
        "mov (%rsp),%r8",
        "push %rbx\n    mov 8(%rsp),%r8\n    pop %rbx",
    );
    let overwritten = X86
        .replace(
            "mov (%rsp),%r8",
            "mov (%rsp),%r9\n    mov %rdx,(%rsp)\n    mov (%rsp),%r8\n    mov %r9,(%rsp)",
        )
        .replace("add $8,%rsp", "");
    let from_table = INDEXED.replace(
        "mov %rdx,%rax\n    jmp .Ldispatch",
        "mov $2,%eax\n    jmp .Lload",
    );
    // This branch checks the output pointer, not the new table index.
    let unrelated_guard = from_table.replace(
        "jmp .Lload",
        "test %rsi,%rsi\n    jne .Lload\n    ret",
    );
    let fused_load = from_table.replace(
        "mov .Ltable(,%rax,8),%rax\n.Ldispatch:\n    jmp *%rax",
        "jmp *.Ltable(,%rax,8)",
    );
    let changed_base = from_table
        .replace("mov %edi,%eax", "mov $.Ltable,%ecx\n    mov %edi,%eax")
        .replace(".Ltable(,%rax,8)", "(%rcx,%rax,8)")
        .replace("mov $2,%eax", "mov $.Ltable+16,%ecx\n    mov $0,%eax");
    let from_register = INDEXED.replace("mov %rdx,%rax", "mov $.Lthird,%eax");
    // Input zero and the second input exercise different exits. Check the
    // callback count after each transfer, the final store and both canaries.
    for (machine, mips, indexed, first_calls, second_input, final_calls, final_store) in [
        (X86, false, false, 0, 42, 1, 42),
        (&disjoint, false, false, 0, 42, 1, 42),
        (&pushed, false, false, 0, 42, 1, 42),
        (&overwritten, false, false, 1, 42, 2, 42),
        (MIPS, true, false, 0, 42, 1, 42),
        (INDEXED, false, true, 0, 1, 1, 1),
        (&from_table, false, true, 0, 1, 0, 23),
        (&unrelated_guard, false, true, 0, 1, 0, 23),
        (&from_register, false, true, 0, 1, 0, 23),
        (&changed_base, false, true, 0, 1, 0, 23),
        (&fused_load, false, true, 0, 1, 0, 23),
    ] {
        let checks = format!(
            "#define INDEXED_CASE {}\n#define FIRST_CALLS {first_calls}\n#define SECOND_INPUT {second_input}\n#define FINAL_CALLS {final_calls}\n#define FINAL_STORE {final_store}\n{DRIVER}",
            u8::from(indexed)
        );
        if mips && !installed(&"qemu-mips64") {
            continue;
        }
        let emulator = mips.then_some("qemu-mips64");
        let entry = if mips {
            ".set noreorder\n.text\n.globl _start\n_start: jal check\nnop\nmove $4,$2\nli $2,5058\nsyscall\n"
        } else {
            ".text\n.globl _start\n_start: call check\nmov %eax,%edi\nmov $60,%eax\nsyscall\n"
        };
        let body = common::scratch_file("dispatch-body", "S");
        let start = common::scratch_file("dispatch-start", "S");
        let driver = common::scratch_file("dispatch-driver", "c");
        let image = common::scratch_file("dispatch-original", "elf");
        let emitted = common::scratch_file("dispatch-emitted", "c");
        let replay = common::scratch_file("dispatch-replay", "elf");
        let fixture_name = if indexed {
            "indexed_original"
        } else {
            "dispatch_probe"
        };
        std::fs::write(&body, machine).unwrap();
        std::fs::write(&start, entry).unwrap();
        let table = common::scratch_file("dispatch-table", "S");
        std::fs::write(&driver, checks.replace("dispatch_probe", fixture_name)).unwrap();

        let link = |source: &Path, out: &Path, level: &str| {
            let mut cmd = Command::new(clang);
            if mips {
                cmd.args([
                    "--target=mips64-linux-gnuabi64",
                    "-march=mips64r2",
                    "-mno-abicalls",
                    "-G0",
                ]);
            }
            if indexed {
                cmd.args([
                    "-Wl,--section-start=.original_dispatch=0x480000",
                    "-Wl,--section-start=.dispatch_cases=0x500000",
                ]);
                if source != body {
                    cmd.arg(&table);
                }
            }
            cmd.arg(format!("-fuse-ld={}", linker.display()))
                .args([
                    "-Werror=uninitialized",
                    "-ffreestanding",
                    "-fno-pic",
                    "-nostdlib",
                    "-static",
                    "-Wl,-Ttext=0x400000",
                    "-Wl,-e,_start",
                    level,
                ])
                .arg(source)
                .arg(&start)
                .arg(&driver)
                .arg("-o")
                .arg(out);
            process::required_output(&mut cmd);
        };
        link(&body, &image, "-O1");
        run_image(&image, emulator);
        let rows = indexed.then(|| copy_table(&image, &table));
        std::fs::write(&driver, checks).unwrap();
        for mode in [None, Some("off"), Some("on")] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
            cmd.arg("decompile")
                .arg(&image)
                .args([fixture_name, "--sleighpath"])
                .arg(common::repo_root().join("specs"))
                .args([
                    "--assert-strict",
                    "--assert",
                    &format!("prototype {fixture_name} void dispatch_probe(int value,int *slot,void (*next)(void))"),
                ]);
            if let Some(mode) = mode {
                cmd.args(["--option", "constselectjump", mode]);
            }
            let code = String::from_utf8(process::required_output(&mut cmd).stdout).unwrap();
            let code = code.replace("indexed_original(", "dispatch_probe(");
            std::fs::write(
                &emitted,
                format!("#include <stdbool.h>\n#include <stddef.h>\nextern const unsigned long dat_500000[];\n{code}"),
            )
            .unwrap();
            for level in ["-O0", "-O2"] {
                link(&emitted, &replay, level);
                if let Some(rows) = &rows {
                    assert_native_cases_unmapped(&replay, rows);
                }
                run_image(&replay, emulator);
            }
        }
        if indexed {
            std::fs::remove_file(table).unwrap();
        }
        for file in [body, start, driver, image, emitted, replay] {
            std::fs::remove_file(file).unwrap();
        }
    }
}
