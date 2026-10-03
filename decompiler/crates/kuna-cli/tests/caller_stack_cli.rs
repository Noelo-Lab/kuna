//! Rebuilt code must address the physical caller area, including after O2.
mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

const FUNCTIONS: [&str; 5] = [
    "caller_first",
    "caller_adjacent",
    "caller_frame",
    "caller_write_first",
    "caller_write_adjacent",
];

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn build_object() -> PathBuf {
    let path = common::scratch_file("caller-stack", "o");
    let output = Command::new("cc")
        .args(["-c"])
        .arg(fixture("caller_stack_x86_64.S"))
        .arg("-o")
        .arg(&path)
        .output()
        .expect("assemble caller-stack fixture");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    path
}

fn prototype(name: &str) -> String {
    format!(
        "prototype {name} {} {name}(void)",
        if name.contains("write") {
            "unsigned long"
        } else {
            "char *"
        }
    )
}

fn decompile(path: &Path, name: &str) -> String {
    let (text, err, rc) = common::run_kuna(&[
        "decompile",
        path.to_str().unwrap(),
        name,
        "--option",
        "x64syscall",
        "on",
        "--assert",
        &prototype(name),
    ]);
    assert_eq!(rc, 0, "{text}\n{err}");
    assert!(
        text.contains("__builtin_dwarf_cfa()") && text.contains("__attribute__((noinline))"),
        "{text}"
    );
    assert!(!text.contains("Stack000"), "{text}");
    text
}

fn roundtrip(source: &Path, original: Option<&Path>) {
    for compiler in ["gcc", "clang"] {
        if Command::new(compiler).arg("--version").output().is_err() {
            continue;
        }
        for opt in ["-O0", "-O2"] {
            let exe = common::scratch_file("caller-stack-roundtrip", "bin");
            let mut command = Command::new(compiler);
            command
                .args([opt, "-Werror", "-include", "unistd.h"])
                .arg(source);
            if let Some(object) = original {
                command.arg(object);
            }
            let output = command
                .arg("-o")
                .arg(&exe)
                .output()
                .expect("compile caller-stack roundtrip");
            assert!(
                output.status.success(),
                "{compiler} {opt}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let output = Command::new(exe)
                .output()
                .expect("run caller-stack roundtrip");
            assert!(output.status.success(), "{compiler} {opt}: {output:?}");
            assert_eq!(
                String::from_utf8_lossy(&output.stdout).trim(),
                "caller addresses and pipe payloads match"
            );
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn incoming_addresses_and_kernel_payloads_roundtrip() {
    let object = build_object();
    let driver = std::fs::read_to_string(fixture("caller_stack_driver.c")).unwrap();
    let native = common::scratch_file("caller-stack-native", "c");
    std::fs::write(&native, &driver).unwrap();
    roundtrip(&native, Some(&object));
    let printed = common::scratch_file("caller-stack-printed", "c");
    let mut source = "#include <unistd.h>\n".to_string();
    for function in FUNCTIONS {
        source.push_str(&decompile(&object, function));
    }
    source.push_str(&driver);
    std::fs::write(&printed, source).unwrap();
    roundtrip(&printed, None);
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn project_preserves_caller_storage_and_prototypes() {
    let object = build_object();
    let project = common::scratch_file("caller-stack-project", "dir");
    let mut args = vec![
        "decompile-project".to_string(),
        object.to_str().unwrap().to_string(),
        "--functions".to_string(),
        FUNCTIONS.join(","),
        "--option".to_string(),
        "x64syscall".to_string(),
        "on".to_string(),
        "-o".to_string(),
        project.to_str().unwrap().to_string(),
    ];
    for function in FUNCTIONS {
        args.extend(["--assert".to_string(), prototype(function)]);
    }
    let (text, err, rc) = common::run_kuna(&args.iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(rc, 0, "{text}\n{err}");
    let c = std::fs::read_dir(&project)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|s| s == "c"))
        .expect("project C");
    let h = c.with_extension("h");
    let header = std::fs::read_to_string(&h).unwrap();
    assert!(
        header.contains("caller_first(void)") && !header.contains("Stack000"),
        "{header}"
    );
    let mut source = std::fs::read_to_string(&c).unwrap();
    source.push_str(&std::fs::read_to_string(fixture("caller_stack_driver.c")).unwrap());
    std::fs::write(&c, source).unwrap();
    roundtrip(&c, None);
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn locals_return_address_and_named_parameter_keep_existing_forms() {
    let object = build_object();
    for name in ["caller_below", "caller_local", "caller_named"] {
        let proto = if name == "caller_named" {
            "prototype caller_named char *caller_named(long a, long b, long c, long d, long e, long f, long named)".to_string()
        } else {
            prototype(name)
        };
        let (text, err, rc) = common::run_kuna(&[
            "decompile",
            object.to_str().unwrap(),
            name,
            "--assert",
            &proto,
        ]);
        assert_eq!(rc, 0, "{text}\n{err}");
        assert!(
            !text.contains("__builtin_dwarf_cfa") && !text.contains("noinline"),
            "{text}"
        );
        if name == "caller_named" {
            assert!(text.contains("&named"), "{text}");
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn typed_object_pointers_keep_the_caller_address() {
    let object = build_object();
    for ty in ["int", "struct Payload"] {
        let proto = format!("prototype caller_first {ty} *caller_first(void)");
        let (text, err, rc) = common::run_kuna(&[
            "decompile",
            object.to_str().unwrap(),
            "caller_first",
            "--assert-strict",
            "--assert",
            "typedef struct Payload { long value; char tag; };",
            "--assert",
            &proto,
        ]);
        assert_eq!(rc, 0, "{text}\n{err}");
        let source = common::scratch_file("caller-stack-typed", "c");
        std::fs::write(&source, format!(
            "#include <stdlib.h>\ntypedef struct Payload {{ long value; char tag; }} Payload;\n{text}\nint main(void) {{ void *sp; __asm__ volatile(\"mov %%rsp,%0\" : \"=r\"(sp)); if ((void *)caller_first() != sp) abort(); }}\n"
        )).unwrap();
        for compiler in ["gcc", "clang"] {
            if Command::new(compiler).arg("--version").output().is_err() {
                continue;
            }
            for opt in ["-O0", "-O2"] {
                let exe = common::scratch_file("caller-stack-typed", "bin");
                let output = Command::new(compiler)
                    .args([opt, "-Werror"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .expect("compile typed caller address");
                assert!(
                    output.status.success(),
                    "{compiler} {opt}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let output = Command::new(exe)
                    .output()
                    .expect("run typed caller address");
                assert!(output.status.success(), "{compiler} {opt}: {output:?}");
            }
        }
    }
}
