//! Execute a late dispatch to a callback and, on MIPS, to the incoming return.
use crate::common::{self, process};
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
// table and its case symbols at the same addresses in the replay executable.
const INDEXED: &str = r#"
.section .original_dispatch,"ax"
.globl indexed_original
.type indexed_original,@function
indexed_original:
    cmp $1,%edi
    ja .Ldone
    mov %edi,%eax
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
.size indexed_original,.-indexed_original
.section .dispatch_cases,"a"
.balign 8
.Ltable:
    .quad .Lfirst,.Lsecond
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
    dispatch_probe(0, slots+1, callback);
    if (calls || slots[1] || slots[2]) return 1;
    dispatch_probe(42, slots+1, callback);
    return calls != 1 || slots[0] != 12 || slots[1] != 42 || slots[2] != 77 || slots[3] != 34;
}
"#;

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
    for (machine, mips, indexed) in [
        (X86, false, false),
        (MIPS, true, false),
        (INDEXED, false, true),
    ] {
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
        let checks = if indexed {
            DRIVER.replace("42", "1")
        } else {
            DRIVER.to_owned()
        };
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
                    cmd.arg(&body);
                }
            }
            cmd.arg(format!("-fuse-ld={}", linker.display()))
                .args([
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
        std::fs::write(&driver, &checks).unwrap();
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
                format!("#include <stdbool.h>\n#include <stddef.h>\n{code}"),
            )
            .unwrap();
            for level in ["-O0", "-O2"] {
                link(&emitted, &replay, level);
                run_image(&replay, emulator);
            }
        }
        for file in [body, start, driver, image, emitted, replay] {
            std::fs::remove_file(file).unwrap();
        }
    }
}
