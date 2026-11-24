//! The final IR keeps p-code's size rules: a bitwise or arithmetic op's output and
//! inputs share one size, and a compare reads two inputs of one size.
//!
//! `RuleExpandLoad` widens a narrow LOAD tested only as `(V & C) == D` into a LOAD of
//! the whole field and shifts both constants. It used to keep the AND's narrow
//! output, so `ptrenumlow` and `ptrenumhigh` (`tests/datatests/enum.xml`) and `t_bit`
//! (`tests/stages/kuna-narrowload.xml`) ended with `AL:1 = u:4 & #0x3000:4` and a
//! compare of that byte against a 4-byte constant. The printed C was right, but the
//! IR was not well formed.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_file;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::IfaceStatus;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The setup commands of a datatest script: every `<com>` before its first `lo fu`.
fn setup_commands(xml: &str) -> Vec<String> {
    let mut out = Vec::new();
    for piece in xml.split("<com>").skip(1) {
        let com = piece.split("</com>").next().unwrap();
        let com = com.replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&");
        if com.starts_with("lo fu") {
            break;
        }
        out.push(com);
    }
    out
}

fn dcp(status: &mut IfaceStatus) -> &mut IfaceDecompData {
    status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap()
}

fn size_violations(fd: &Funcdata) -> Vec<String> {
    use OpCode::*;
    let mut bad = Vec::new();
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id).unwrap();
        let size = |vn| fd.vbank().get(vn).unwrap().get_size();
        let ins: Vec<_> = (0..op.num_input()).map(|i| size(op.get_in(i).unwrap())).collect();
        let out = op.get_out().map(size);
        let ok = match op.code() {
            CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_XOR | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_MULT
            | CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM => {
                ins.iter().all(|&s| Some(s) == out)
            }
            CPUI_INT_NEGATE | CPUI_INT_2COMP | CPUI_COPY => Some(ins[0]) == out,
            CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_SLESS | CPUI_INT_LESSEQUAL
            | CPUI_INT_SLESSEQUAL | CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
                ins[0] == ins[1] && out == Some(1)
            }
            _ => true,
        };
        if !ok {
            bad.push(format!("{:?} out {out:?} ins {ins:?}", op.code()));
        }
    }
    bad
}

fn check(corpus_file: &str, functions: &[&str]) {
    let root = repo_root();
    let path = root.join(corpus_file);
    let prog = bootstrap_from_file(path.to_str().unwrap(), "", &[root.join("specs").to_str().unwrap().to_string()])
        .expect("bootstrap the corpus image with built processor specs");
    let mut cmds = setup_commands(&std::fs::read_to_string(&path).unwrap());
    let setup = cmds.len();
    for f in functions {
        cmds.push(format!("lo fu {f}"));
        cmds.push("decompile".into());
    }
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    dcp(&mut status).conf = Some(prog);
    for _ in 0..setup {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    for f in functions {
        execute(&mut status);
        execute(&mut status);
        assert!(!status.inerror, "{f}: {}", status.optr);
        let bad = size_violations(dcp(&mut status).fd.as_ref().unwrap());
        assert!(bad.is_empty(), "{f}: {bad:#?}");
    }
}

#[test]
fn expanded_enum_flag_tests_keep_op_sizes() {
    check("tests/datatests/enum.xml", &["ptrenumlow", "ptrenumhigh"]);
}

#[test]
fn expanded_record_byte_test_keeps_op_sizes() {
    check("tests/stages/kuna-narrowload.xml", &["t_bit"]);
}
