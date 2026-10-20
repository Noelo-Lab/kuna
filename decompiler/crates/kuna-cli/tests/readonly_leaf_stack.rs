//! Conditional stack stores survive a complete register-only helper.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#".text
.globl frame_choice
.type frame_choice,@function
frame_choice:
 push %ebp
 mov %esp,%ebp
 sub $24,%esp
 mov 8(%ebp),%eax
 test %eax,%eax
 jne 1f
 movl $17,-8(%ebp)
 jmp 2f
1:
 push $3f
 push %ebp
 mov %esp,%ebp
 call frame_helper
3:
 pop %ebp
 add $4,%esp
 mov %eax,-8(%ebp)
2:
 mov -8(%ebp),%eax
 leave
 ret
.size frame_choice,.-frame_choice
.globl frame_helper
.type frame_helper,@function
frame_helper:
 mov $29,%eax
 ret
.size frame_helper,.-frame_helper

"#;

#[test]
fn conditional_store_remains_associated_with_its_return_load() {
    for tool in ["as", "ld", "gcc"] {
        if process::optional_output(Command::new(tool).arg("--version")).is_none() {
            return;
        }
    }
    let source = common::scratch_file("readonly-leaf", "s");
    let object = source.with_extension("o");
    let binary = source.with_extension("elf");
    std::fs::write(&source, SOURCE).unwrap();
    let output = Command::new("as")
        .arg("--32")
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new("ld")
        .args(["-m", "elf_i386", "-e", "frame_choice"])
        .arg(&object)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (body, err, status) =
        common::run_kuna(&["decompile", binary.to_str().unwrap(), "frame_choice"]);
    assert_eq!(status, 0, "{err}");
    assert!(body.contains("0x11"), "{body}");
    let (before, err, status) = common::run_kuna(&[
        "decompile",
        binary.to_str().unwrap(),
        "frame_choice",
        "--option",
        "calleepreserves",
        "off",
    ]);
    assert_eq!(status, 0, "{err}");
    assert!(
        !before.contains("0x11"),
        "control unexpectedly retains the conditional store: {before}"
    );
    let code = format!("unsigned char Stackfffffffc;\nunsigned int frame_helper(void *frame, unsigned int continuation) {{ (void)frame; (void)continuation; return 29; }}\n{body}\nint main(void) {{ return frame_choice(0)!=17 || frame_choice(1)!=29 || frame_choice(-1)!=29; }}\n");
    let regenerated = source.with_extension("c");
    let executable = source.with_extension("exe");
    std::fs::write(&regenerated, &code).unwrap();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let output = Command::new(compiler)
                .args(["-std=c11", "-Wall", "-Werror", level])
                .arg(&regenerated)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}\n{code}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{compiler} {level}\n{code}"
            );
        }
    }
    for helper in [
        "movl $29,4(%esp)\nmov $29,%eax\nret",
        "mov 4(%esp),%ecx\nmov (%ecx),%eax\nret",
        "ret",
    ] {
        std::fs::write(&source, SOURCE.replace("mov $29,%eax\n ret", helper)).unwrap();
        assert!(Command::new("as")
            .arg("--32")
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .status()
            .unwrap()
            .success());
        assert!(Command::new("ld")
            .args(["-m", "elf_i386", "-e", "frame_choice"])
            .arg(&object)
            .arg("-o")
            .arg(&binary)
            .status()
            .unwrap()
            .success());
        let (body, err, status) =
            common::run_kuna(&["decompile", binary.to_str().unwrap(), "frame_choice"]);
        assert_eq!(status, 0, "{err}");
        assert!(
            !body.contains("0x11"),
            "helper must retain conservative memory effects: {body}"
        );
    }
    for path in [source, object, binary, regenerated, executable] {
        std::fs::remove_file(path).unwrap();
    }
}
