//! Go DWARF contracts reach definitions, callers and executable output.

use crate::common;
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn fixture(name: &str) -> PathBuf {
    root().join("tests/fixtures/go-dwarf-signatures").join(name)
}
fn decompile(binary: &Path, names: &[&str], enabled: bool, options: &[&str]) -> Vec<String> {
    let mut program = bootstrap_from_object(binary.to_str().unwrap(), "",
        &[root().join("specs").display().to_string()]).unwrap();
    program.arch_mut().set_kuna_option("godwarfsig", if enabled { "on" } else { "off" }).unwrap();
    program.commit_pending_analysis().unwrap();
    let mut commands = vec!["option outlang go".to_string()];
    commands.extend(options.iter().map(|s| s.to_string()));
    let prefix = commands.len();
    for name in names {
        commands.extend([format!("load function {name}"), "decompile".into(), "print C".into()]);
    }
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut()
        .downcast_mut::<IfaceDecompData>().unwrap().conf = Some(program);
    for _ in 0..prefix {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    names.iter().map(|name| {
        execute(&mut status);
        assert!(!status.inerror, "{name}: {}", status.optr);
        execute(&mut status);
        assert!(!status.inerror, "{name}: {}", status.optr);
        let start = status.optr.len();
        execute(&mut status);
        assert!(!status.inerror, "{name}: {}", status.optr);
        status.optr[start..].trim().to_string()
    }).collect()
}
fn run(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(output.status.success(), "{command:?}:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
}
fn go_available() -> bool {
    match Command::new("go").arg("version").output() {
        Ok(output) => { assert!(output.status.success()); eprintln!("{}", String::from_utf8_lossy(&output.stdout)); true }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Go toolchain absent; external execution check skipped"); false
        }
        Err(error) => panic!("go: {error}"),
    }
}
const SUPPORTED: &[&str] = &[
    "main.fromWord", "main.narrow", "main.extend", "main.boolNot", "main.mix",
    "main.store", "main.add", "main.scale", "main.callIdentity", "main.noResult",
    "main.resultOnly", "main.abstractIdentity",
];

#[test]
fn compressed_and_cross_unit_contracts_agree() {
    let expected = [
        "func fromWord(p uint64) *int32", "func narrow(a uint64) uint32",
        "func extend(a int8) int64", "func boolNot(x bool) bool",
        "func mix(x float64, n int64, y float64) float64", "func store(p *int32, v int32)",
        "func add(a int64, b int64) int64", "func scale(p *main.point, k int64)",
        "func callIdentity(p uint64) *int32", "func noResult(a int32)",
        "func resultOnly() int64", "func abstractIdentity(p uint64) *int32",
    ];
    let mut previous = None;
    for image in ["uncompressed", "compressed"] {
        let on = decompile(&fixture(image), SUPPORTED, true, &[]);
        let off = decompile(&fixture(image), &SUPPORTED[..2], false, &[]);
        for (body, signature) in on.iter().zip(expected) {
            assert!(body.contains(signature), "{body}");
        }
        assert!(on[8].contains("return fromWord(p)"), "{}", on[8]);
        assert!(on[7].contains("(*p).x") && on[7].contains("(*p).y"), "{}", on[7]);
        assert!(!off[0].contains("func fromWord(p uint64) *int32"), "{}", off[0]);
        assert!(!off[1].contains("func narrow(a uint64) uint32"), "{}", off[1]);
        if let Some(previous) = previous { assert_eq!(on, previous); }
        previous = Some(on);
    }
}

#[test]
fn rejected_signatures_preserve_inference() {
    let names = ["main.badReg", "main.missingFlag", "main.multiResult", "main.trampoline",
        "main.worker.func1", "main.badType", "main.tooWide", "main.wrongProducer",
        "main.badResultReg", "main.stackInput", "main.tooManyIntegers", "main.tooManyFloats"];
    for image in ["uncompressed", "compressed"] {
        let off = decompile(&fixture(image), &names, false, &[]);
        let on = decompile(&fixture(image), &names, true, &[]);
        assert_eq!(off, on, "{image}");
    }
    let binary = root().join("tests/fixtures/go-output/output-c");
    let names = ["add", "narrow", "from_word", "record"];
    assert_eq!(decompile(&binary, &names, false, &[]), decompile(&binary, &names, true, &[]));
}

#[test]
fn explicit_prototype_overrides_recovered_signature() {
    let code = decompile(&fixture("compressed"), &["main.fromWord"], true,
        &["map prototype main.fromWord uint8 fromWord(uint8 explicit_name);"]);
    assert!(code[0].contains("func fromWord(explicit_name uint64) uint64"), "{}", code[0]);
    assert!(code[0].contains("return explicit_name"), "{}", code[0]);
}

#[test]
fn recovered_fixture_go_executes() {
    if !go_available() { return; }
    let names = SUPPORTED.iter().copied().filter(|name| *name != "main.scale").collect::<Vec<_>>();
    for image in ["uncompressed", "compressed"] {
        let code = decompile(&fixture(image), &names, true, &[]);
        let file = common::scratch_file("godwarfsig", "go");
        let check = r#"
var dat_40302a int32
func main() {
  var x int32
  store(&x,-17)
  if x != -17 || narrow(0x123456789)!=0x23456789 || extend(-128)!=-128 { panic("widths") }
  if boolNot(true) || !boolNot(false) || mix(1.5,7,-0.25)!=8.25 { panic("bool/float") }
  if add(-17,29)!=12 || resultOnly()!=42 { panic("integer results") }
  p:=uint64(uintptr(unsafe.Pointer(&x)))
  if fromWord(p)!=&x || callIdentity(p)!=&x || abstractIdentity(p)!=&x { panic("pointer results") }
  noResult(45)
  if dat_40302a!=45 { panic("void result") }
}
"#;
        std::fs::write(&file, format!("package main\nimport \"unsafe\"\n{}\n{check}", code.join("\n"))).unwrap();
        run(Command::new("go").arg("run").arg(file));
    }
}

#[test]
fn native_go_contracts_compile_and_execute_without_assertions() {
    if !go_available() { return; }
    let binary = common::scratch_file("godwarfsig-native", "elf");
    run(Command::new("go").args(["build", "-trimpath", "-buildvcs=false", "-o"]).arg(&binary)
        .arg(root().join("tests/fixtures/go-output/output.go")));
    let names = ["main.add", "main.choose", "main.sumFor", "main.shortCalls", "main.loopCalls",
        "main.dispatch", "main.readAt", "main.writeAt", "main.fromWord", "main.toWord",
        "main.retype", "main.narrow", "main.extend", "main.number"];
    let code = decompile(&binary, &names, true, &[]);
    let file = common::scratch_file("godwarfsig-native", "go");
    let check = r#"
func main() {
  xs := [4]int32{1,2,3,4}
  writeAt(&xs[0],2,17)
  if add(2,3)!=5 || choose(11,2)!=3 || choose(-1,2)!=1 || choose(2,3)!=6 { panic("arithmetic") }
  if sumFor(9)!=16 || shortCalls(2,3)!=7 || shortCalls(0,7)!=2 || loopCalls(9,-1)!=30 { panic("loops") }
  if dispatch(0,2)!=10 || dispatch(1,2)!=4 || dispatch(2,9)!=5 || dispatch(8,2)!=-2 { panic("switch") }
  if readAt(&xs[0],2)!=17 || fromWord(toWord(&xs[0]))!=&xs[0] || retype(nil)!=nil { panic("pointers") }
  if narrow(0x123456789)!=0x23456789 || extend(-3)!=-3 || number(7)!=7 { panic("conversions") }
}
"#;
    std::fs::write(&file, format!("package main\nimport \"unsafe\"\n{}\n{check}", code.join("\n"))).unwrap();
    run(Command::new("go").arg("run").arg(file));
}

#[test]
fn empty_entry_ranges_require_the_exact_register_and_ret_body() {
    let binary = root().join("tests/fixtures/go-dwarf-entry-ranges/entry-ranges");
    let mut program = bootstrap_from_object(binary.to_str().unwrap(), "",
        &[root().join("specs").display().to_string()]).unwrap();
    program.arch_mut().set_kuna_option("godwarfsig", "on").unwrap();
    program.commit_pending_analysis().unwrap();
    for name in ["identity", "wrongRegister", "laterRange", "notReturn", "longBody",
        "duplicateRange", "badBase", "resultRange"] {
        let address = program.lookup_symbol(&format!("main.{name}")).unwrap();
        let present = program.arch().symboltab.function_proto_pieces_across_scopes(&address).is_some();
        assert_eq!(present, name == "identity", "{name}");
    }
    let on = decompile(&binary, &["main.identity"], true, &[]);
    let off = decompile(&binary, &["main.identity"], false, &[]);
    assert!(on[0].contains("func identity(p uint64) *int32"), "{}", on[0]);
    assert!(off[0].contains("func identity()"), "{}", off[0]);
    if go_available() {
        let file = common::scratch_file("godwarfsig-entry", "go");
        std::fs::write(&file, format!("package main\nimport \"unsafe\"\n{}\nfunc main() {{ var x int32; if identity(uint64(uintptr(unsafe.Pointer(&x))))!=&x {{ panic(\"identity\") }} }}\n", on[0])).unwrap();
        run(Command::new("go").arg("run").arg(file));
    }
}

#[test]
fn uncompressed_go_results_never_become_c_inputs() {
    let names = ["main.identity", "main.wrongRegister", "main.laterRange", "main.notReturn",
        "main.longBody", "main.duplicateRange", "main.badBase", "main.resultRange"];
    for enabled in [false, true] {
        for cpp in [false, true] {
            let mut previous = None;
            for image in ["entry-ranges", "entry-ranges-plain"] {
                let binary = root().join("tests/fixtures/go-dwarf-entry-ranges").join(image);
                let mut program = bootstrap_from_object(binary.to_str().unwrap(), "",
                    &[root().join("specs").display().to_string()]).unwrap();
                program.arch_mut().set_kuna_option("godwarfsig", if enabled { "on" } else { "off" }).unwrap();
                program.arch_mut().set_kuna_option("cppproto", if cpp { "on" } else { "off" }).unwrap();
                program.commit_pending_analysis().unwrap();
                for name in names {
                    let address = program.lookup_symbol(name).unwrap();
                    let present = program.arch().symboltab.function_proto_pieces_across_scopes(&address).is_some();
                    assert_eq!(present, enabled && name == "main.identity", "{image} {name}, Go={enabled}, C++={cpp}");
                }
                let options = [if cpp { "option cppproto on" } else { "option cppproto off" }];
                let code = decompile(&binary, &[names[0], names[1], names[7]], enabled, &options);
                if let Some(previous) = previous { assert_eq!(code, previous); }
                previous = Some(code);
            }
        }
    }
}

#[test]
fn a_different_calling_convention_refuses_the_contract() {
    let mut program = bootstrap_from_object(fixture("compressed").to_str().unwrap(), "",
        &[root().join("specs").display().to_string()]).unwrap();
    program.arch_mut().set_kuna_option("godwarfsig", "on").unwrap();
    let model = program.arch().get_model("abi0").unwrap().clone();
    program.arch_mut().set_default_model_rc(model);
    program.commit_pending_analysis().unwrap();
    let address = program.lookup_symbol("main.fromWord").unwrap();
    assert!(program.arch().symboltab.function_proto_pieces_across_scopes(&address).is_none());
}
