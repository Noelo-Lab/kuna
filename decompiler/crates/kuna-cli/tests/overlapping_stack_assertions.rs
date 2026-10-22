//! Independently retyped frame locals must not become disjoint C objects.
#![cfg(target_os = "linux")]
use crate::common;
use common::process;
use object::Object;
use std::process::Command;

#[test]
fn overlapping_stack_retypes_are_rejected_in_either_order() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("overlapping-stack-assertions", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let binary = root.join("sample.dll");
    std::fs::write(
        &source,
        r#"
#define API __declspec(dllexport) __attribute__((noinline))
typedef struct Span {unsigned count;const unsigned char *data;unsigned stride;} Span;
API unsigned digest(void *opaque) {
 Span *s=opaque;unsigned sum=0;
 for(unsigned i=0;i<s->count;++i)sum=sum*17+s->data[i*s->stride];
 return sum;
}
API unsigned run_span(const unsigned char *input,unsigned n) {
 Span span;span.count=n;span.data=input;span.stride=3;
 unsigned first=digest(&span);
 span.count=n/2;span.data=input+1;span.stride=2;
 return first^digest(&span);
}
"#,
    )
    .unwrap();
    process::required_output(
        Command::new("clang")
            .args([
                "--target=i686-w64-windows-gnu",
                "-O0",
                "-shared",
                "-nostdlib",
                "-Wl,--entry,0",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let bytes = std::fs::read(&binary).unwrap();
    let image = object::File::parse(bytes.as_slice()).unwrap();
    let exports = image.exports().unwrap();
    let address = |name: &[u8]| exports.iter().find(|e| e.name() == name).unwrap().address();
    let run = address(b"run_span");
    let digest = address(b"digest");
    let declarations = [
        "typedef struct Span {unsigned int count;const unsigned char *data;unsigned int stride;} Span;".to_owned(),
        format!("prototype 0x{digest:x} unsigned int __cdecl digest(void *opaque)"),
        format!("prototype 0x{run:x} unsigned int __cdecl run_span(const unsigned char *input,unsigned int n)"),
    ];
    let root_type = "type _run_span::v2 Span".to_owned();
    let tail_type = "type _run_span::v4 unsigned char[4]".to_owned();
    let build_command = |label: &str, assertions: &[&str]| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command
            .arg("decompile-project")
            .arg(&binary)
            .args([
                "--addr",
                &format!("0x{run:x}"),
                "--option",
                "stackviews",
                "on",
                "--assert-strict",
                "--sleighpath",
            ])
            .arg(common::repo_root().join("specs"))
            .arg("-o")
            .arg(root.join(label));
        for declaration in &declarations {
            command.args(["--assert", declaration]);
        }
        for assertion in assertions {
            command.args(["--assert", assertion]);
        }
        command
    };
    for (label, assertions) in [
        ("tail-first", [&*tail_type, &*root_type]),
        ("root-first", [&*root_type, &*tail_type]),
    ] {
        let output = build_command(label, &assertions).output().unwrap();
        let diagnostic = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.status.success(),
            "overlapping retypes silently accepted:\n{diagnostic}"
        );
        assert!(
            diagnostic.contains("Stack storage overlaps the independently retyped local"),
            "{diagnostic}"
        );
    }
    // A single complete layout owns all three fields and remains supported.
    process::required_output(&mut build_command("one-layout", &[&root_type]));
    let generated = root.join("one-layout/sample.dll.c");
    let code = std::fs::read_to_string(&generated).unwrap();
    assert!(code.contains("v2.stride = 3;"), "{code}");
    assert!(code.contains("v2.stride = 2;"), "{code}");
    process::required_output(
        Command::new("clang")
            .args([
                "--target=i686-w64-windows-gnu",
                "-ffreestanding",
                "-nostdlibinc",
                "-Werror",
                "-c",
            ])
            .arg(&generated)
            .arg("-o")
            .arg(root.join("generated.o")),
    );
    // A separate adjacent local and repeated statements on one object are valid.
    let adjacent = "type _run_span::v1 unsigned int".to_owned();
    process::required_output(&mut build_command("adjacent", &[&root_type, &adjacent]));
    process::required_output(&mut build_command("repeat", &[&root_type, &root_type]));
    let _ = std::fs::remove_dir_all(root);
}
