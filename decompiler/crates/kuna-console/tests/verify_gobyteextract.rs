//! Execute Go callers that consume every word of a recovered array result.

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

fn print(program: ConsoleProgram, name: &str) -> String {
    print_commands(
        program,
        vec![
            "option outlang go".into(),
            format!("lo fu {name}"),
            "decompile".into(),
            "print C".into(),
        ],
    )
}

fn print_commands(program: ConsoleProgram, commands: Vec<String>) -> String {
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

fn native_body(binary: &Path, name: &str) -> String {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program
        .arch_mut()
        .set_kuna_option("godwarfsig", "on")
        .unwrap();
    program.commit_pending_analysis().unwrap();
    print(program, name)
}

#[test]
fn native_caller_and_callee_execute_with_both_result_words() {
    if !go_available() {
        return;
    }
    let source = common::scratch_file("gobyteextract-native", "go");
    let binary = common::scratch_file("gobyteextract-native", "elf");
    std::fs::write(&source, "package main\n//go:noinline\nfunc pair(x int64)(int64,int64){return x,x+1}\n//go:noinline\nfunc combine(x int64)int64{a,b:=pair(x);return a*3+b}\nvar result int64\nfunc main(){result=combine(41)}\n").unwrap();
    run(Command::new("go")
        .args(["build", "-trimpath", "-buildvcs=false", "-o"])
        .arg(&binary)
        .arg(&source));
    let pair = native_body(&binary, "main.pair");
    let combine = native_body(&binary, "main.combine");
    assert_eq!(combine.matches("pair(").count(), 1, "{combine}");
    assert!(
        !combine.contains("SUB168") && !combine.contains("._"),
        "{combine}"
    );
    let support = "func main(){for _,x:=range []int64{0,1,-1,-17,0x7fffffffffffffff,-0x8000000000000000}{if combine(x)!=x*4+1{panic(\"caller result\")}}}";
    let output = common::scratch_file("gobyteextract-execute", "go");
    std::fs::write(
        &output,
        format!("package main\nimport \"unsafe\"\n{pair}\n{combine}\n{support}\n"),
    )
    .unwrap();
    run(Command::new("go").arg("run").arg(output));
}

#[test]
fn unsigned_caller_evaluates_its_pair_once_and_preserves_signed_bits() {
    if !go_available() {
        return;
    }
    let fixture = root().join("tests/stages/ghangr-go-byte-extract.xml");
    let program = bootstrap_from_file(
        fixture.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    let body = print(program, "combine");
    assert!(!body.contains("SUB168") && !body.contains("._"), "{body}");
    let support = r#"
var calls int
func pair(x uint64) [16]byte {
  calls++
  var result [16]byte
  binary.LittleEndian.PutUint64(result[:8],x)
  binary.LittleEndian.PutUint64(result[8:],x+1)
  return result
}
func main() {
  for _,x:=range []uint64{0,1,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffef,0xffffffffffffffff} {
    before:=calls
    if combine(x)!=int64(x*4+1) || calls!=before+1 {panic("single call and result bits")}
  }
}
"#;
    let output = common::scratch_file("gobyteextract-counted", "go");
    std::fs::write(
        &output,
        format!("package main\nimport (\"encoding/binary\";\"unsafe\")\n{body}\n{support}\n"),
    )
    .unwrap();
    run(Command::new("go").arg("run").arg(output));
}

#[test]
fn inferred_unsigned_stores_use_the_rendered_pointee_type() {
    if !go_available() {
        return;
    }
    let fixture = root().join("tests/stages/ghangr-go-byte-extract.xml");
    let program = bootstrap_from_file(
        fixture.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    let body = print_commands(
        program,
        [
            "option outlang go",
            "parse line extern void write_pair(unsigned long ctx);",
            "lo fu write_pair",
            "decompile",
            "print C",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect(),
    );
    assert!(!body.contains("SUB168") && !body.contains("._"), "{body}");
    let support = r#"
var calls int
var context [2]uint64
var storage [4]uint64
func pair(x uint64) [16]byte {
  calls++
  var result [16]byte
  binary.LittleEndian.PutUint64(result[:8],x)
  binary.LittleEndian.PutUint64(result[8:],x+1)
  return result
}
func main() {
  for _,x:=range []uint64{0,1,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffef,0xffffffffffffffff} {
    storage=[4]uint64{0x1234,0,0,0x5678}
    context=[2]uint64{x,uint64(uintptr(unsafe.Pointer(&storage[1])))}
    before:=calls
    write_pair(uint64(uintptr(unsafe.Pointer(&context))))
    if storage[1]!=x || storage[2]!=x+1 || storage[0]!=0x1234 || storage[3]!=0x5678 || calls!=before+1 {panic("stored result words")}
  }
}
"#;
    let output = common::scratch_file("gobyteextract-store", "go");
    std::fs::write(
        &output,
        format!("package main\nimport (\"encoding/binary\";\"unsafe\")\n{body}\n{support}\n"),
    )
    .unwrap();
    run(Command::new("go").arg("run").arg(output));
}
