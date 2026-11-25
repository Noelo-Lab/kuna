//! Replay original computed-dispatch effects, independent of printed spellings.
use crate::common;
use std::process::Command;

fn decompile(
    image: &std::path::Path,
    abi: &str,
    base: u64,
    wide: bool,
    option: Option<(&str, &str)>,
) -> String {
    let target = if abi == "n32" {
        "MIPS:BE:64:octeon-32addr:default"
    } else {
        "MIPS:BE:64:octeon:default"
    };
    let return_type = if wide { "unsigned long long" } else { "void" };
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile")
        .arg(image)
        .arg(format!("0x{base:x}"))
        .args(["--raw-image", "--base"])
        .arg(format!("0x{base:x}"))
        .args(["--target", target, "--sleighpath"])
        .arg(common::repo_root().join("specs"))
        .args(["--assert-strict", "--assert"])
        .arg(format!(
            "prototype 0x{base:x} {return_type} bounce(unsigned int value, unsigned int *slot)"
        ));
    if let Some((name, value)) = option {
        command.args(["--option", name, value]);
    }
    String::from_utf8(common::process::required_output(&mut command).stdout).unwrap()
}

/// Does the emitted C keep every store and canary (and a wide fixture's
/// return) for 10,000 inputs at O0 and O2 under UBSan? A compile error, a
/// failed check or a loop that never returns is `false`. A wide fixture's
/// 32-bit ADDIU wraps before its sign extension; defined C for that word
/// arithmetic is emission's concern, so those compile with `-fwrapv`.
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn replay_passes(code: &str, steps: usize, wide: bool, join: bool, return_shift: u64, cap: u64) -> bool {
    let source = common::scratch_file("indirect-dispatch", "c");
    let driver = source.with_extension("driver.c");
    let executable = source.with_extension("exe");
    std::fs::write(
        &source,
        format!("#include <stdint.h>\n#include <stdbool.h>\n#define CONCAT44(hi,lo) (((uint64_t)(uint32_t)(hi)<<32)|(uint32_t)(lo))\n{code}"),
    )
    .unwrap();
    let declaration = code
        .split_once('{')
        .expect("function body")
        .0
        .split("//")
        .next()
        .unwrap()
        .trim();
    let name = declaration
        .split_once('(')
        .unwrap()
        .0
        .split_whitespace()
        .last()
        .unwrap();
    let checks = (0..steps)
        .map(|i| {
            if join && i == 1 {
                format!("a[{}]!=x+((x&1u)?1u:2u)", i + 1)
            } else {
                format!("a[{}]!=x+{}u", i + 1, i)
            }
        })
        .collect::<Vec<_>>()
        .join(" || ");
    let invoke = if wide {
        format!("uint64_t result={name}(x,&a[1]);")
    } else {
        format!("{name}(x,&a[1]);")
    };
    let returned = if wide {
        format!(" || result!=(((uint64_t)(int64_t)(int32_t)(x+1u)<<{return_shift})|((uint32_t)(x+1u)^0x12345678u))")
    } else {
        String::new()
    };
    std::fs::write(&driver,format!("#include <stdint.h>\n{declaration};\nint main(void) {{ for(uint32_t i=0;i<10000;i++) {{uint32_t edges[7]={{0,1,73,0xffffffffu,0x7fffffffu,0x80000000u,0xfffffffeu}};uint32_t x=i<7?edges[i]:i*2654435761u; uint32_t a[{}]={{12}};a[{}]=34;{invoke}if(a[0]!=12 || a[{}]!=34 || {checks}{returned})return 1;}}return 0;}}\n",steps+2,steps+1,steps+1)).unwrap();
    let mut passed = true;
    for optimization in ["-O0", "-O2"] {
        let mut compile = Command::new("cc");
        compile.args([
            "-std=c11",
            optimization,
            "-fno-lto",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-Wno-parentheses",
            "-fsanitize=undefined",
            "-fno-sanitize-recover=undefined",
            "-Wuninitialized",
        ]);
        if wide {
            compile.arg("-fwrapv");
        }
        let built = compile.arg(&source).arg(&driver).arg("-o").arg(&executable).output().unwrap();
        if !built.status.success() {
            passed = false;
            break;
        }
        let output = common::process::output_with_timeout(
            &mut Command::new(&executable),
            std::time::Duration::from_secs(cap),
            std::time::Duration::from_millis(10),
        );
        if !output.is_some_and(|o| o.status.success()) {
            passed = false;
            break;
        }
    }
    for file in [source, driver, executable] {
        let _ = std::fs::remove_file(file);
    }
    passed
}

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn reentered_dispatches_preserve_each_effect_and_the_callers_return() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let root = common::repo_root().join("tests/fixtures/indirect-dispatch");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("provenance.json")).unwrap()).unwrap();
    let fixtures = manifest["fixtures"].as_array().unwrap();
    assert_eq!(fixtures.len(), 32);
    for fixture in fixtures {
        let name = fixture["binary"].as_str().unwrap();
        let image = root.join(name);
        let abi = fixture["abi"].as_str().unwrap();
        let base = fixture["base"].as_u64().unwrap();
        let wide = fixture["wide_return"].as_bool().unwrap();
        let join = fixture["join"].as_bool().unwrap();
        let return_shift = fixture["return_shift"].as_u64().unwrap();
        let steps = fixture["steps"].as_u64().unwrap() as usize;
        let code = decompile(&image, abi, base, wide, None);
        assert!(replay_passes(&code, steps, wide, join, return_shift, 10), "{name}\n{code}");
        if steps > 1 {
            let upstream = decompile(&image, abi, base, wide, Some(("constselectjump", "off")));
            assert!(
                !replay_passes(&upstream, steps, wide, join, return_shift, 1),
                "constselectjump off must lose a later step: {name}\n{upstream}"
            );
        }
        let off = decompile(&image, abi, base, wide, Some(("returnbranch", "off")));
        if fixture["register"] == 9 {
            assert_eq!(code, off, "ordinary-register control: {name}");
        } else {
            assert!(
                !off.contains("*slot = value;"),
                "off must falsify the computed-return witness: {name}\n{off}"
            );
        }
        if name.ends_with("r31-2.bin") {
            let legacy = decompile(&image, abi, base, wide, Some(("regionstructure", "off")));
            assert!(replay_passes(&legacy, steps, wide, join, return_shift, 10), "{name}\n{legacy}");
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn dispatcher_return_updates_survive_narrowing_and_foreign_register_copies() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let root = common::repo_root().join("tests/fixtures/return-value-dispatch");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("provenance.json")).unwrap()).unwrap();
    for fixture in manifest["fixtures"].as_array().unwrap() {
        let narrow = fixture["kind"] == "narrow";
        let prototype = if narrow { "unsigned int bounce(void)" }
            else { "unsigned int bounce(unsigned int value)" };
        let target = if fixture["abi"] == "n32" { "MIPS:BE:64:octeon-32addr:default" }
            else { "MIPS:BE:64:octeon:default" };
        let (code, stderr, rc) = common::run_kuna(&[
            "decompile", root.join(fixture["binary"].as_str().unwrap()).to_str().unwrap(),
            "0x400000", "--raw-image", "--base", "0x400000", "--target", target,
            "--assert-strict", "--assert", &format!("prototype 0x400000 {prototype}"),
        ]);
        assert_eq!(rc, 0, "{stderr}");
        let declaration = code.split_once('{').unwrap().0.trim();
        let name = declaration.split_once('(').unwrap().0.split_whitespace().last().unwrap();
        let source = common::scratch_file("dispatch-value", "c");
        let driver = source.with_extension("driver.c");
        let executable = source.with_extension("exe");
        std::fs::write(&source, format!("#include <stdbool.h>\n#include <stdint.h>\n{code}")).unwrap();
        let invocation = if narrow { format!("{name}()!=3u") } else { format!("{name}(x)!=x+7u") };
        std::fs::write(&driver, format!("#include <stdint.h>\n{declaration};\nint main(void){{for(uint32_t i=0;i<10000;i++){{uint32_t x=i*2654435761u;(void)x;if({invocation})return 1;}}return 0;}}\n")).unwrap();
        for opt in ["-O0", "-O2"] {
            common::process::required_output(Command::new("cc")
                .args(["-std=c11", opt, "-Wall", "-Wextra", "-Werror", "-Wuninitialized",
                    "-fno-lto", "-fsanitize=undefined", "-fno-sanitize-recover=undefined"])
                .arg(&source).arg(&driver).arg("-o").arg(&executable));
            let result = common::process::output_with_timeout(&mut Command::new(&executable),
                std::time::Duration::from_secs(5), std::time::Duration::from_millis(10))
                .expect("dispatcher must return");
            assert!(result.status.success(), "{code}\n{}", String::from_utf8_lossy(&result.stderr));
        }
        for path in [source, driver, executable] { std::fs::remove_file(path).unwrap(); }
    }
}
