//! Linux pipe effects stay in the same order through emitted C.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;

use std::path::Path;
use std::process::Command;

const SOURCE: &str = include_str!("fixtures/x64_syscall_memory.c");
const SCOPE_SOURCE: &str = include_str!("fixtures/x64_syscall_scope.S");
const SCOPE_DRIVER: &str = include_str!("fixtures/x64_syscall_scope_driver.c");
const PROTOTYPES: &[(&str, &str)] = &[
    ("pointer_before", "int pointer_before(int *p)"),
    ("pointer_expr", "int pointer_expr(int *p, int x)"),
    ("pointer_twice", "int pointer_twice(int *p)"),
    ("global_before", "int global_before(int x)"),
    ("global_twice", "int global_twice(void)"),
    ("global_store", "int global_store(int x)"),
    ("pointer_store", "int pointer_store(int *p, int x)"),
    ("global_reload", "int global_reload(int x)"),
    ("barrier", "int barrier(int *p)"),
    ("plain", "int plain(int *p, int x)"),
    ("return_snapshot", "int return_snapshot(int *p)"),
];

fn compile(args: &[&str], source: &Path, output: &Path) {
    let result = Command::new("cc")
        .args(args)
        .arg(source)
        .args(["-o"])
        .arg(output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn excluded_abis_preserve_the_register_only_syscall_model() {
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    let source = common::scratch_file("x64-syscall-scope", "S");
    let object = common::scratch_file("x64-syscall-scope", "o");
    std::fs::write(&source, SCOPE_SOURCE).unwrap();
    compile(&["-c"], &source, &object);
    let (text, err, rc) = common::run_kuna(&[
        "decompile",
        object.to_str().unwrap(),
        "abi_echo",
        "--assert",
        "prototype abi_echo unsigned long MSABI abi_echo(unsigned long a)",
    ]);
    assert_eq!(rc, 0, "{text}\n{err}");
    assert!(text.contains("return a;"), "{text}");
    let (text, err, rc) = common::run_kuna(&[
        "decompile",
        object.to_str().unwrap(),
        "abi_echo",
        "--option",
        "protoeval",
        "MSABI",
    ]);
    assert_eq!(rc, 0, "{text}\n{err}");
    assert!(text.contains("abi_echo(unsigned long a0)"), "{text}");
    assert!(text.contains("return a0;"), "{text}");
    for policy in [
        "windows",
        "msabi",
        "windows-stdcall",
        "prototype-msabi",
        "sysv-eval-msabi",
    ] {
        for mode in ["auto", "aggressive", "on", "abi", "off"] {
            for name in ["metadata8", "metadata40"] {
                let mut args = vec![
                    "decompile".to_owned(),
                    object.to_str().unwrap().to_owned(),
                    name.to_owned(),
                    "--assert".to_owned(),
                    format!(
                        "prototype {name} unsigned long {}{name}(void)",
                        match policy {
                            "prototype-msabi" => "MSABI ",
                            "sysv-eval-msabi" => "__stdcall ",
                            _ => "",
                        }
                    ),
                ];
                match policy {
                    "windows" | "windows-stdcall" => {
                        args.extend(["--target".into(), "x86:LE:64:default:windows".into()])
                    }
                    "msabi" | "sysv-eval-msabi" => {
                        args.extend(["--option".into(), "protoeval".into(), "MSABI".into()])
                    }
                    _ => {}
                }
                if policy == "windows-stdcall" {
                    args.extend(["--option".into(), "protoeval".into(), "__stdcall".into()]);
                }
                match mode {
                    "aggressive" => args.extend(["--mode".into(), mode.into()]),
                    "auto" => {}
                    _ => args.extend(["--option".into(), "x64syscall".into(), mode.into()]),
                }
                let refs: Vec<_> = args.iter().map(String::as_str).collect();
                let (text, err, rc) = common::run_kuna(&refs);
                assert_eq!(rc, 0, "{policy} {mode}: {text}\n{err}");
                let windows = policy.starts_with("windows");
                let result = if windows {
                    "unsigned int"
                } else {
                    "unsigned long"
                };
                let cast = if windows { "(unsigned int)" } else { "" };
                let body = if mode == "off" {
                    "  syscall();\n  return 1;".to_owned()
                } else if mode == "abi" {
                    let register = if windows {
                        "unsigned long long"
                    } else {
                        "unsigned long"
                    };
                    format!(
                        "  char v1 [16]; // stack - 0x10\n  {register} v2; // r8\n  {register} v3; // r9\n  {register} v4; // r10\n  \n  return {cast}syscall(1,0x11,v1,8,v4,v2,v3);"
                    )
                } else {
                    format!(
                        "  char v1 [16]; // stack - 0x10\n  \n  return {cast}syscall(1,0x11,v1,8);"
                    )
                };
                let expected = format!("{result} {name}(void)\n{{\n{body}\n}}");
                assert_eq!(text.trim_end(), expected, "{policy} {mode}");
                if mode == "off" {
                    continue;
                }
                let printed = common::scratch_file("x64-syscall-excluded", "c");
                std::fs::write(&printed, format!("#include <unistd.h>\n{text}")).unwrap();
                for compiler in ["cc", "clang"] {
                    if Command::new(compiler).arg("--version").output().is_err() {
                        continue;
                    }
                    for optimization in ["-O0", "-O2"] {
                        let output = common::scratch_file("x64-syscall-excluded", "o");
                        let result = Command::new(compiler)
                            .args([optimization, "-Werror", "-c"])
                            .arg(&printed)
                            .arg("-o")
                            .arg(&output)
                            .output()
                            .unwrap();
                        assert!(
                            result.status.success(),
                            "{policy} {mode} {compiler} {optimization}\n{text}\n{}",
                            String::from_utf8_lossy(&result.stderr)
                        );
                    }
                }
            }
        }
    }
    for mode in ["on", "abi"] {
        let (text, err, rc) = common::run_kuna(&[
            "decompile",
            object.to_str().unwrap(),
            "metadata8",
            "--assert",
            "prototype metadata8 unsigned long metadata8(void)",
            "--option",
            "x64syscall",
            mode,
        ]);
        assert_eq!(rc, 0, "{text}\n{err}");
        assert!(text.contains("v1[0] ="), "{mode}: {text}");
        assert!(text.contains("syscall(1,0x11,v1,8"), "{mode}: {text}");
    }
}

#[test]
fn caller_metadata_writes_keep_the_physical_pointer_payload() {
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    let source = common::scratch_file("x64-syscall-metadata", "S");
    let object = common::scratch_file("x64-syscall-metadata", "o");
    let native = common::scratch_file("x64-syscall-metadata-native", "o");
    std::fs::write(&source, SCOPE_SOURCE).unwrap();
    compile(&["-c"], &source, &object);
    compile(
        &[
            "-c",
            "-Dmetadata8=native_metadata8",
            "-Dmetadata40=native_metadata40",
            "-Dabi_echo=native_abi_echo",
        ],
        &source,
        &native,
    );
    let roundtrip = |printed: &Path| {
        for compiler in ["gcc", "clang"] {
            if Command::new(compiler).arg("--version").output().is_err() {
                continue;
            }
            for optimization in ["-O0", "-O2"] {
                let output = common::scratch_file("x64-syscall-metadata", "bin");
                let result = Command::new(compiler)
                    .args([optimization, "-Werror", "-include", "unistd.h"])
                    .arg(printed)
                    .arg(&native)
                    .arg("-o")
                    .arg(&output)
                    .output()
                    .unwrap();
                assert!(
                    result.status.success(),
                    "{compiler} {optimization}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
                assert!(
                    Command::new(&output).status().unwrap().success(),
                    "{compiler} {optimization}: {}",
                    std::fs::read_to_string(printed).unwrap()
                );
            }
        }
    };
    for mode in [None, Some("on")] {
        let mut code = String::new();
        for name in ["metadata8", "metadata40"] {
            let mut args = vec![
                "decompile".to_string(),
                object.to_str().unwrap().to_string(),
                name.to_string(),
                "--assert".to_string(),
                format!("prototype {name} unsigned long {name}(void)"),
            ];
            if let Some(mode) = mode {
                args.extend(["--option".into(), "x64syscall".into(), mode.into()]);
            }
            let (text, err, rc) =
                common::run_kuna(&args.iter().map(String::as_str).collect::<Vec<_>>());
            assert_eq!(rc, 0, "{text}\n{err}");
            assert!(
                text.contains("v1[0] =")
                    && text.contains("__builtin_dwarf_cfa()")
                    && !text.contains("Stack000"),
                "{text}"
            );
            code.push_str(&text);
        }
        let printed = common::scratch_file("x64-syscall-metadata", "c");
        std::fs::write(&printed, code + SCOPE_DRIVER).unwrap();
        roundtrip(&printed);
    }
    let project = common::scratch_file("x64-syscall-metadata-project", "dir");
    let (text, err, rc) = common::run_kuna(&[
        "decompile-project",
        object.to_str().unwrap(),
        "--functions",
        "metadata8,metadata40",
        "--option",
        "x64syscall",
        "on",
        "-o",
        project.to_str().unwrap(),
        "--assert",
        "prototype metadata8 unsigned long metadata8(void)",
        "--assert",
        "prototype metadata40 unsigned long metadata40(void)",
    ]);
    assert_eq!(rc, 0, "{text}\n{err}");
    let c = std::fs::read_dir(&project)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|s| s == "c"))
        .unwrap();
    let mut code = std::fs::read_to_string(&c).unwrap();
    code.push_str(SCOPE_DRIVER);
    std::fs::write(&c, code).unwrap();
    roundtrip(&c);
}

#[test]
fn returned_lengths_precede_the_post_syscall_buffer_read() {
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    let source = common::scratch_file("x64-syscall-lengths", "c");
    std::fs::write(&source, SOURCE).unwrap();
    for optimization in ["-O0", "-O2"] {
        let object = common::scratch_file("x64-syscall-length-object", "o");
        compile(
            &[optimization, "-fcf-protection=none", "-fno-pie", "-c"],
            &source,
            &object,
        );
        let native = common::scratch_file("x64-syscall-length-native", "o");
        let mut args = vec![
            optimization.to_owned(),
            "-c".into(),
            "-Dsink=native_sink".into(),
        ];
        args.extend(
            PROTOTYPES
                .iter()
                .map(|(name, _)| format!("-D{name}=native_{name}")),
        );
        let refs: Vec<_> = args.iter().map(String::as_str).collect();
        compile(&refs, &source, &native);
        for mode in [None, Some("on"), Some("abi")] {
            let emitted = decompile(&object, mode);
            let code = &emitted.iter().find(|f| f.0 == "return_snapshot").unwrap().1;
            let program = format!(
                r#"
#include <stdint.h>
#include <unistd.h>
#include <stdlib.h>
int native_return_snapshot(int *p);
#define syscall(n,fd,p,size,...) read(fd,(void *)(uintptr_t)(p),size)
{code}
static void setup(void) {{
  int a[2],b[2],first=100,second=500;
  if(pipe(a)||pipe(b)) abort();
  if(write(a[1],&first,4)!=4||write(b[1],&second,2)!=2) abort();
  close(a[1]);close(b[1]);dup2(a[0],17);dup2(b[0],18);close(a[0]);close(b[0]);
}}
int main(void) {{
  int p=3; setup();
  if(native_return_snapshot(&p)!=570||p!=500) abort();
  close(17);close(18); p=3; setup();
  if(return_snapshot(&p)!=570||p!=500) abort();
  close(17);close(18); return 0;
}}
"#
            );
            let printed = common::scratch_file("x64-syscall-length-printed", "c");
            let binary = common::scratch_file("x64-syscall-length-pipes", "bin");
            std::fs::write(&printed, &program).unwrap();
            compile(&["-O2", native.to_str().unwrap()], &printed, &binary);
            assert!(
                Command::new(&binary).status().unwrap().success(),
                "{optimization} {mode:?}\n{program}"
            );
        }
    }
}

fn decompile(object: &Path, mode: Option<&str>) -> Vec<(String, String)> {
    let mut args = vec![
        "decompile-all".to_owned(),
        object.to_str().unwrap().to_owned(),
        "--json".to_owned(),
    ];
    if let Some(mode) = mode {
        args.extend(["--option".into(), "x64syscall".into(), mode.into()]);
    }
    for (name, prototype) in PROTOTYPES {
        args.extend(["--assert".into(), format!("prototype {name} {prototype}")]);
    }
    let refs: Vec<_> = args.iter().map(String::as_str).collect();
    let (text, err, rc) = common::run_kuna(&refs);
    assert_eq!(rc, 0, "{text}\n{err}");
    let doc: serde_json::Value = serde_json::from_str(&text).unwrap();
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            assert!(f["error"].is_null(), "{f}");
            (
                f["name"].as_str().unwrap().to_owned(),
                f["code"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

#[test]
fn saved_reads_and_kernel_observed_stores_round_trip_through_linux_pipes() {
    if Command::new("cc").arg("--version").output().is_err() {
        return;
    }
    let source = common::scratch_file("x64-syscall-source", "c");
    let object = common::scratch_file("x64-syscall-object", "o");
    let native = common::scratch_file("x64-syscall-native", "o");
    std::fs::write(&source, SOURCE).unwrap();
    compile(
        &[
            "-O2",
            "-fcf-protection=none",
            "-fno-pie",
            "-fno-asynchronous-unwind-tables",
            "-c",
        ],
        &source,
        &object,
    );
    let mut native_args = vec![
        "-O2".to_owned(),
        "-fcf-protection=none".into(),
        "-c".into(),
        "-Dsink=native_sink".into(),
    ];
    native_args.extend(
        PROTOTYPES
            .iter()
            .map(|(name, _)| format!("-D{name}=native_{name}")),
    );
    let refs: Vec<_> = native_args.iter().map(String::as_str).collect();
    compile(&refs, &source, &native);
    let off = decompile(&object, Some("off"));
    for mode in [None, Some("on"), Some("abi")] {
        let emitted = decompile(&object, mode);
        for name in ["barrier", "plain"] {
            assert_eq!(
                emitted.iter().find(|f| f.0 == name),
                off.iter().find(|f| f.0 == name)
            );
        }
        let global = &emitted.iter().find(|f| f.0 == "global_before").unwrap().1;
        let address = global
            .split("syscall(0,0x11,0x")
            .nth(1)
            .unwrap()
            .split(',')
            .next()
            .unwrap();
        let global_alias = global
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .find(|word| word.starts_with("dat_"))
            .map(|name| format!("#define {name} sink"))
            .unwrap_or_default();
        let declarations = PROTOTYPES
            .iter()
            .map(|(_, p)| p.replacen(" ", " native_", 1) + ";\n")
            .collect::<String>();
        let program = format!(
            r#"
#include <stdint.h>
#include <unistd.h>
#include <stdlib.h>
int sink;
{global_alias}
extern int native_sink;
{declarations}
static long kernel(long n, long fd, uintptr_t address, long size) {{
  void *p = address == 0x{address} ? &sink : (void *)address;
  return n == 0 ? read(fd,p,size) : write(fd,p,size);
}}
#define syscall(n,fd,p,size,...) kernel(n,fd,(uintptr_t)(p),size)
{}
static int setup(int twice) {{
  int in[2], out[2], values[2]={{twice ? 41 : 100,100}};
  if(pipe(in)||pipe(out)) abort();
  if(write(in[1],values,twice ? 8 : 4) < 0) abort();
  close(in[1]); dup2(in[0],17); close(in[0]);
  dup2(out[1],18); close(out[1]);
  return out[0];
}}
static void finish(int out,int written) {{
  close(17);close(18);
  if(written) {{ int seen=0; if(read(out,&seen,4)!=4||seen!=9) abort(); }}
  close(out);
}}
#define CHECK(call,expected,last,twice,written) do {{ \
  int out=setup(twice), result=(call); \
  if(result!=(expected)||(last)) abort(); finish(out,written); \
}} while(0)
int main(void) {{
  int v=3;
  CHECK(native_pointer_before(&v),115,v!=100,0,0); v=3;
  CHECK(pointer_before(&v),115,v!=100,0,0); v=3;
  CHECK(native_pointer_expr(&v,7),121,v!=100,0,0); v=3;
  CHECK(pointer_expr(&v,7),121,v!=100,0,0); v=3;
  CHECK(native_pointer_twice(&v),305,v!=100,1,0); v=3;
  CHECK(pointer_twice(&v),305,v!=100,1,0); v=3;
  native_sink=3; CHECK(native_global_before(7),121,native_sink!=100,0,0);
  sink=3; CHECK(global_before(7),121,sink!=100,0,0);
  native_sink=3; CHECK(native_global_twice(),305,native_sink!=100,1,0);
  sink=3; CHECK(global_twice(),305,sink!=100,1,0);
  native_sink=3; CHECK(native_global_store(2),9,native_sink!=0,0,1);
  sink=3; CHECK(global_store(2),9,sink!=0,0,1);
  CHECK(native_pointer_store(&v,2),9,v!=0,0,1); v=3;
  CHECK(pointer_store(&v,2),9,v!=0,0,1);
  native_sink=3; CHECK(native_global_reload(2),145,native_sink!=100,0,0);
  sink=3; CHECK(global_reload(2),145,sink!=100,0,0);
  return 0;
}}
"#,
            emitted
                .iter()
                .map(|f| f.1.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        );
        let printed = common::scratch_file("x64-syscall-printed", "c");
        let binary = common::scratch_file("x64-syscall-pipes", "bin");
        std::fs::write(&printed, &program).unwrap();
        compile(&["-O2", native.to_str().unwrap()], &printed, &binary);
        assert!(
            Command::new(&binary).status().unwrap().success(),
            "mode {mode:?}\n{program}"
        );
    }
}
