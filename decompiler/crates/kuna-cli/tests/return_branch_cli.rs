//! A constant jump through the return register keeps the code it reaches, on a
//! linked image, behind an instruction that emits no p-code, and on a language
//! whose compiler spec names no return address.
use crate::common;
use common::process;

use std::path::{Path, PathBuf};
use std::process::Command;

/// The hand-written original: the local continuation's address is a link-time
/// constant, which is what the entry prefix must prove.
const BOUNCE: &str = r#"	.text
	.globl bounce
	.type bounce, @function
bounce:
LEADING	move	$t0, $ra
	lui	$ra, %hi(bounce_cont)
	ori	$ra, $ra, %lo(bounce_cont)
	jr	$ra
	nop
bounce_cont:
	sw	$a0, 0($a1)
	move	$ra, $t0
	jr	$ra
	nop
	.size bounce, .-bounce
"#;

/// `probe` returns 0 only when the store the jump reaches has run.
const PROBE: &str = r#"extern void bounce(int value, int *slot);
static int slot[2];
int probe(void) { bounce(42, slot); return slot[0] == 42 ? 0 : 3; }
"#;

fn start(exit_syscall: u32) -> String {
    format!(
        r#"	.text
	.globl _start
_start:
	jal	probe
	nop
	move	$a0, $v0
	li	$v0, {exit_syscall}
	syscall
"#
    )
}

#[derive(Clone, Copy)]
struct Abi {
    name: &'static str,
    triple: &'static str,
    arch: &'static str,
    abi: &'static str,
    exit_syscall: u32,
    qemu: &'static [&'static str],
}

/// N64 Octeon is the second ABI on purpose: its compiler spec names no return
/// address, so the proof must fall back to the language's own `ra` register.
const ABIS: [Abi; 2] = [
    Abi {
        name: "mips32",
        triple: "mips-linux-gnu",
        arch: "mips32r2",
        abi: "32",
        exit_syscall: 4001,
        qemu: &["qemu-mips", "qemu-mips-static"],
    },
    Abi {
        name: "octeon-n64",
        triple: "mips64-linux-gnuabi64",
        arch: "octeon",
        abi: "64",
        exit_syscall: 5058,
        qemu: &["qemu-mips64", "qemu-mips64-static"],
    },
];

fn cross_tool(variable: &str, defaults: &[&str]) -> Option<String> {
    if let Some(value) = std::env::var_os(variable) {
        let value = value.into_string().expect("UTF-8 tool path");
        process::required_output(Command::new(&value).arg("--version"));
        return Some(value);
    }
    defaults.iter().find_map(|tool| {
        process::optional_output(Command::new(tool).arg("--version")).map(|_| (*tool).to_owned())
    })
}

/// `-fuse-ld=` takes a known name or a path, never an arbitrary binary's name.
fn linker_path(name: &str) -> Option<PathBuf> {
    if name.contains(std::path::MAIN_SEPARATOR) {
        return Some(PathBuf::from(name));
    }
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

fn kuna() -> Command {
    Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    )
}

fn decompile(image: &Path, options: &[&str]) -> String {
    let mut command = kuna();
    command
        .args(["decompile"])
        .arg(image)
        .args(["bounce", "--sleighpath"])
        .arg(common::repo_root().join("specs"))
        .args([
            "--assert-strict",
            "--assert",
            "prototype bounce void bounce(int value, int *slot)",
        ])
        .args(options);
    String::from_utf8(process::required_output(&mut command).stdout).unwrap()
}

/// Link `sources` into one freestanding image for `abi`.
fn link(abi: Abi, cc: &str, ld: &Path, level: &str, sources: &[&Path], out: &Path) {
    let mut command = Command::new(cc);
    command
        .arg(format!("--target={}", abi.triple))
        .args([
            format!("-march={}", abi.arch),
            format!("-mabi={}", abi.abi),
            format!("-fuse-ld={}", ld.display()),
        ])
        .args([
            "-msoft-float",
            "-mno-abicalls",
            "-fno-pic",
            "-G0",
            "-ffreestanding",
            "-nostdlib",
            "-static",
            "-Wl,-e,_start",
            "-w",
            level,
        ])
        .args(sources)
        .arg("-o")
        .arg(out);
    process::required_output(&mut command);
}

fn run(qemu: &str, image: &Path) -> i32 {
    let output = process::output_with_timeout(
        &mut Command::new(qemu).arg(image),
        std::time::Duration::from_secs(30),
        std::time::Duration::from_millis(10),
    )
    .expect("the probe terminates");
    output.status.code().expect("the probe exits")
}

fn source(stem: &str, text: &str) -> PathBuf {
    let path = common::scratch_file(stem, "s");
    std::fs::write(&path, text).unwrap();
    path
}

/// For each ABI and for a leading instruction with and without p-code: the
/// original image stores through the jump, the default output stores too and
/// replays identically, and `returnbranch off` drops the store, as the
/// instruction specification's RETURN requires.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn a_constant_jump_through_the_return_register_keeps_its_store() {
    let Some(cc) = cross_tool("KUNA_RETURNBRANCH_MIPS_CC", &["clang", "clang-21", "clang-14"])
    else {
        eprintln!("returnbranch replay: no cross compiler");
        return;
    };
    let ld = cross_tool("KUNA_RETURNBRANCH_MIPS_LD", &["ld.lld", "ld.lld-21", "ld.lld-14"])
        .as_deref()
        .and_then(linker_path);
    let Some(ld) = ld else {
        eprintln!("returnbranch replay: no cross linker");
        return;
    };
    let probe = {
        let path = common::scratch_file("returnbranch-probe", "c");
        std::fs::write(&path, PROBE).unwrap();
        path
    };
    for abi in ABIS {
        let Some(qemu) = cross_tool("KUNA_RETURNBRANCH_MIPS_QEMU", abi.qemu) else {
            eprintln!("returnbranch replay: no {} emulator", abi.name);
            continue;
        };
        let entry = source(&format!("returnbranch-start-{}", abi.name), &start(abi.exit_syscall));
        for leading in ["", "\tnop\n"] {
            let label = format!("{}{}", abi.name, if leading.is_empty() { "" } else { "-nop" });
            let bounce = source(
                &format!("returnbranch-bounce-{label}"),
                &BOUNCE.replace("LEADING", leading),
            );
            let original = common::scratch_file(&format!("returnbranch-{label}"), "elf");
            link(abi, &cc, &ld, "-O1", &[&bounce, &entry, &probe], &original);
            assert_eq!(run(&qemu, &original), 0, "{label}: the original lost its store");

            let printed = decompile(&original, &[]);
            assert!(printed.contains("*slot = value;"), "{label}: {printed}");
            let refused = decompile(&original, &["--option", "returnbranch", "off"]);
            assert!(!refused.contains("*slot = value;"), "{label}: {refused}");

            let emitted = common::scratch_file(&format!("returnbranch-{label}-emitted"), "c");
            std::fs::write(
                &emitted,
                printed.lines().filter(|l| !l.starts_with("// ")).collect::<Vec<_>>().join("\n"),
            )
            .unwrap();
            for level in ["-O0", "-O2"] {
                let replay = common::scratch_file(&format!("returnbranch-{label}{level}"), "elf");
                link(abi, &cc, &ld, level, &[&emitted, &entry, &probe], &replay);
                assert_eq!(run(&qemu, &replay), 0, "{label} {level}: {printed}");
                std::fs::remove_file(replay).unwrap();
            }
            for path in [bounce, original, emitted] {
                std::fs::remove_file(path).unwrap();
            }
        }
        std::fs::remove_file(entry).unwrap();
    }
    std::fs::remove_file(probe).unwrap();
}
