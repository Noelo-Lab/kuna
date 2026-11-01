//! Execute recovered Go register results and byte-array views without editing their bodies.

use crate::common;
use kuna_console::engine::{bootstrap_from_file, bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn decompile(binary: &Path, name: &str, enabled: bool) -> String {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program
        .arch_mut()
        .set_kuna_option("godwarfsig", if enabled { "on" } else { "off" })
        .unwrap();
    program.commit_pending_analysis().unwrap();
    let commands = vec![
        "option outlang go".to_string(),
        format!("load function {name}"),
        "decompile".into(),
        "print C".into(),
    ];
    print(program, commands)
}

fn print(program: ConsoleProgram, commands: Vec<String>) -> String {
    let prefix = commands.len() - 1;
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..prefix {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    let start = status.optr.len();
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    status.optr[start..].trim().to_string()
}

fn go_available() -> bool {
    match Command::new("go").arg("version").output() {
        Ok(output) => {
            assert!(output.status.success());
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Go toolchain absent; external execution check skipped");
            false
        }
        Err(error) => panic!("go: {error}"),
    }
}

fn run(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn execute_body(body: &str, name: &str) {
    let source = common::scratch_file("gobyteview", "go");
    let check = r#"
func main() {
  for _, x := range []int64{0,1,-1,-17,0x7fffffffffffffff,-0x8000000000000000} {
    result := recovered(x)
    if binary.LittleEndian.Uint64(result[:8]) != uint64(x) ||
       binary.LittleEndian.Uint64(result[8:]) != uint64(x)+1 { panic("result words") }
  }
}
"#;
    std::fs::write(&source, format!("package main\nimport (\"encoding/binary\"; \"unsafe\")\n{body}\nvar recovered = {name}\n{check}")).unwrap();
    run(Command::new("go").arg("run").arg(source));
}

#[test]
fn compressed_and_plain_register_results_preserve_both_words() {
    if !go_available() {
        return;
    }
    for image in ["compressed", "uncompressed"] {
        let binary = root()
            .join("tests/fixtures/go-dwarf-signatures")
            .join(image);
        for enabled in [false, true] {
            execute_body(
                &decompile(&binary, "main.multiResult", enabled),
                "multiResult",
            );
        }
    }
}

#[test]
fn native_go_result_carrier_executes_without_a_signature_assertion() {
    if !go_available() {
        return;
    }
    let source = common::scratch_file("gobyteview-native", "go");
    let binary = common::scratch_file("gobyteview-native", "elf");
    std::fs::write(&source, "package main\n//go:noinline\nfunc pair(x int64)(int64,int64){return x,x+1}\nvar a,b int64\nfunc main(){a,b=pair(41)}\n").unwrap();
    run(Command::new("go")
        .args(["build", "-trimpath", "-buildvcs=false", "-o"])
        .arg(&binary)
        .arg(&source));
    execute_body(&decompile(&binary, "main.pair", true), "pair");
}

#[test]
fn unaligned_reads_and_writes_keep_the_nested_byte_array_path() {
    if !go_available() {
        return;
    }
    for signed in [false, true] {
        let fixture = common::scratch_file("gobyteview-read", "xml");
        std::fs::write(&fixture, r#"<decompilertest><binaryimage arch="x86:LE:64:default:gcc">
<bytechunk space="ram" offset="0x100000" readonly="true">4883ec1866897c240189742407488d3c24e8110000000fb74424018b5424074831d04883c418c3c3</bytechunk>
<symbol space="ram" offset="0x100000" name="read_views"/>
<symbol space="ram" offset="0x100027" name="consume"/>
</binaryimage></decompilertest>"#).unwrap();
        let program = bootstrap_from_file(
            fixture.to_str().unwrap(),
            "",
            &[root().join("specs").display().to_string()],
        )
        .unwrap();
        let commands = [
            "option outlang go",
            "parse line typedef struct Wrap { unsigned char payload[16]; } Wrap;",
            "parse line extern void consume(Wrap *p);",
            if signed {
                "parse line extern unsigned long read_views(short a,int b);"
            } else {
                "parse line extern unsigned long read_views(unsigned short a,unsigned int b);"
            },
            "lo fu read_views",
            "map address s0xffffffffffffffe8 Wrap frame",
            "decompile",
            "print C",
        ];
        let body = print(program, commands.iter().map(|s| s.to_string()).collect());
        assert!(body.contains("unsafe.Pointer(&frame.payload)"), "{body}");
        assert!(!body.contains("._"), "{body}");
        let source = common::scratch_file("gobyteview-read-execute", "go");
        let support = r#"
type Wrap struct { payload [16]byte }
var wantA uint16
var wantB uint32
func consume(p *Wrap) {
  if binary.LittleEndian.Uint16(p.payload[1:]) != wantA ||
     binary.LittleEndian.Uint32(p.payload[7:]) != wantB { panic("stored words") }
  for _, i := range []int{0,3,4,5,6,11,12,13,14,15} {
    if p.payload[i] != 0 { panic("store escaped its width") }
  }
}
func ZEXT28(x uint16) uint64 { return uint64(x) }
func ZEXT48(x uint32) uint64 { return uint64(x) }
func main() {
  for _, a := range []uint16{0,1,0x1234,0x8000,0xffff} {
    for _, b := range []uint32{0,1,0x12345678,0x80000000,0xffffffff} {
      wantA, wantB = a,b
      if read_views(a,b) != uint64(a)^uint64(b) { panic("read words") }
    }
  }
}
"#;
        let support = if signed {
            support.replace("read_views(a,b)", "read_views(int16(a),int32(b))")
        } else {
            support.to_string()
        };
        std::fs::write(
            &source,
            format!("package main\nimport (\"encoding/binary\"; \"unsafe\")\n{body}\n{support}"),
        )
        .unwrap();
        run(Command::new("go").arg("run").arg(source));
    }
}
