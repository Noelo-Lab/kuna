//! A call with an explicit resume address must not print a second call.
use std::fs;

use kuna_base::address::Address;
use kuna_console::engine::bootstrap_from_file;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_slacomp::slgh_compile::SleighCompile;

const SLEIGH: &str = r#"
define endian=little;
define alignment=2;
define space ram type=ram_space size=4 default;
define space register type=register_space size=4;
define register offset=0 size=4 [ pc sp r0 ];
define register offset=12 size=1 [ flag ];
define token opword(16) opc=(0,7) imm=(8,15);
target: addr is imm [ addr = 0x1000 + imm; ] { export *:4 addr; }
resume: addr is epsilon [ addr = inst_start + 4; ] { export *:4 addr; }
helper: addr is epsilon [ addr = 0x1020; ] { export *:4 addr; }
:call_resume target is opc=1 & target & resume {
    local link:4 = inst_start + 4;
    sp = sp - 4; *:4 sp = link; call target; goto resume;
}
:callind_resume is opc=2 & resume {
    local link:4 = inst_start + 4;
    sp = sp - 4; *:4 sp = link; call [r0]; goto resume;
}
:jmp target is opc=3 & target { goto target; }
:mov imm is opc=4 & imm { r0 = imm; }
:ret is opc=5 { local link:4 = *:4 sp; sp = sp + 4; return [link]; }
:call target is opc=6 & target { sp = sp - 4; *:4 sp = inst_next; call target; }
:call_then_test target is opc=7 & target & helper {
    sp = sp - 4; *:4 sp = inst_next; call helper; if (flag) goto target;
}
"#;

const LANGUAGE: &str = r#"<language_definitions>
  <language processor="TestResume" endian="little" size="32" variant="default" version="1.0"
            slafile="test.sla" processorspec="test.pspec" id="TestResume:LE:32:default">
    <description>Test instruction-level call continuation</description>
    <compiler name="default" spec="test.cspec" id="default"/>
  </language>
</language_definitions>"#;

const COMPILER: &str = r#"<compiler_spec>
  <global><range space="ram"/></global>
  <stackpointer register="sp" space="ram"/>
  <returnaddress><varnode space="stack" offset="0" size="4"/></returnaddress>
  <default_proto><prototype name="__cdecl" extrapop="4" stackshift="4">
    <input>
      <pentry minsize="1" maxsize="4"><register name="r0"/></pentry>
      <pentry minsize="1" maxsize="1"><register name="flag"/></pentry>
    </input>
    <output><pentry minsize="1" maxsize="4"><register name="r0"/></pentry></output>
    <unaffected><register name="sp"/><register name="flag"/></unaffected>
  </prototype></default_proto>
</compiler_spec>"#;

const IMAGE: &str = r#"<binaryimage arch="TestResume:LE:32:default:default">
  <!-- CALL helper; skipped RET; resume at MOV 42; RET -->
  <bytechunk space="ram" offset="0x1000" readonly="true">01200500042a0500</bytechunk>
  <bytechunk space="ram" offset="0x1020" readonly="true">0500</bytechunk>
  <!-- Ordinary CALL helper followed by a separate JMP tail_target. -->
  <bytechunk space="ram" offset="0x1040" readonly="true">062003a0</bytechunk>
  <!-- CALLIND r0; skipped RET; resume at MOV 17; RET -->
  <bytechunk space="ram" offset="0x1060" readonly="true">0200050004110500</bytechunk>
  <!-- CALL helper; if flag tail-jump to tail_target, else MOV 7; RET. -->
  <bytechunk space="ram" offset="0x1080" readonly="true">07a004070500</bytechunk>
  <bytechunk space="ram" offset="0x10a0" readonly="true">04630500</bytechunk>
</binaryimage>"#;

fn emit(function: &str) -> String {
    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    let spec = dir.join("test.slaspec");
    let sla = dir.join("test.sla");
    let image = dir.join("image.xml");
    fs::write(&spec, SLEIGH).unwrap();
    fs::write(dir.join("test.ldefs"), LANGUAGE).unwrap();
    fs::write(dir.join("test.cspec"), COMPILER).unwrap();
    fs::write(
        dir.join("test.pspec"),
        r#"<processor_spec><programcounter register="pc"/></processor_spec>"#,
    )
    .unwrap();
    fs::write(&image, IMAGE).unwrap();
    let code = SleighCompile::new()
        .run_compilation(spec.to_str().unwrap(), sla.to_str().unwrap())
        .unwrap();
    assert_eq!(code, 0);
    let mut prog =
        bootstrap_from_file(image.to_str().unwrap(), "", &[dir.to_str().unwrap().into()]).unwrap();
    let space = prog.arch().manage().get_default_code_space().unwrap().clone();
    for (offset, name, size) in [
        (0x1000, "from_direct", 0),
        (0x1004, "direct_resume", 0),
        (0x1020, "helper", 0),
        (0x1040, "separate_jump", 0),
        (0x1060, "from_indirect", 0),
        (0x1064, "indirect_resume", 0),
        (0x1080, "conditional_jump", 6),
        (0x10a0, "tail_target", 0),
    ] {
        prog.declare_function(Address::new(space.clone(), offset), Some(name), size)
            .unwrap();
    }

    let commands = [
        "parse line extern int from_direct(void);",
        "parse line extern void helper(void);",
        "parse line extern int separate_jump(void);",
        "parse line extern int from_indirect(void (*callee)(void));",
        "parse line extern int conditional_jump(int unused, unsigned char flag);",
        "parse line extern int tail_target(void);",
    ];
    let mut commands: Vec<_> = commands.into_iter().map(str::to_owned).collect();
    commands.extend([format!("load function {function}"), "decompile".into(), "print C".into()]);
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(prog);
    for _ in 0..count {
        execute(&mut status);
    }
    let c = status.optr;
    assert!(!c.contains("ERROR:") && !c.contains("Execution error:"), "{c}");
    c
}

#[test]
fn direct_call_reaches_its_explicit_resume() {
    let c = emit("from_direct");
    assert_eq!(c.matches("helper();").count(), 1, "{c}");
    assert!(!c.contains("direct_resume("), "{c}");
    assert!(c.contains("return 0x2a;"), "{c}");
}

#[test]
fn indirect_call_reaches_its_explicit_resume() {
    let c = emit("from_indirect");
    assert_eq!(c.matches("(*callee)();").count(), 1, "{c}");
    assert!(!c.contains("indirect_resume("), "{c}");
    assert!(c.contains("return 0x11;"), "{c}");
}

#[test]
fn separate_jump_after_call_remains_a_tail_call() {
    let c = emit("separate_jump");
    assert_eq!(c.matches("helper();").count(), 1, "{c}");
    assert!(c.contains("return tail_target();"), "{c}");
}

#[test]
fn same_instruction_conditional_tail_still_tests_its_condition() {
    let c = emit("conditional_jump");
    assert_eq!(c.matches("helper();").count(), 1, "{c}");
    assert!(c.contains("if (") && c.contains("return tail_target();"), "{c}");
    assert!(c.contains("return 7;"), "{c}");
}
