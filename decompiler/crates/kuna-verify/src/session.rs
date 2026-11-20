//! Driving the decompiler console in-process, so each `decompile` hands its
//! finished `Funcdata` to the check before the next command replaces it.

use std::rc::Rc;

use kuna_base::xml::DocumentStorage;
use kuna_console::engine::{bootstrap_program, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_console_commands, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::IfaceStatus;
use kuna_console::kuna_console::register_kuna_commands;
use kuna_decomp::funcdata::Funcdata;

fn dcp(console: &IfaceStatus) -> Option<&IfaceDecompData> {
    console
        .get_data(DECOMPILE_MODULE)
        .and_then(|d| d.as_any().downcast_ref::<IfaceDecompData>())
}

fn dcp_mut(console: &mut IfaceStatus) -> Option<&mut IfaceDecompData> {
    console
        .get_data_mut(DECOMPILE_MODULE)
        .and_then(|d| d.as_any_mut().downcast_mut::<IfaceDecompData>())
}

/// Run `commands`, calling `on_decompile` after every `decompile` that
/// succeeds. `conf` preloads a program (the datatest `<binaryimage>`).
pub fn drive(
    commands: Vec<String>,
    conf: Option<ConsoleProgram>,
    spec_roots: &[String],
    mut on_decompile: impl FnMut(&ConsoleProgram, &Funcdata),
) -> Result<(), String> {
    let mut console = ConsoleCommands::into_status(commands.clone());
    register_decomp_commands(&mut console);
    register_kuna_commands(&mut console);
    register_console_commands(&mut console);
    console.set_error_is_done(true);
    if let Some(d) = dcp_mut(&mut console) {
        d.spec_roots = spec_roots.to_vec();
        if conf.is_some() {
            d.conf = conf;
        }
    }
    let mut idx = 0usize;
    while !console.is_stream_finished() {
        execute(&mut console);
        let line = commands.get(idx).map(|s| s.trim().to_string()).unwrap_or_default();
        idx += 1;
        if console.is_in_error() {
            let said: Vec<&str> = console
                .optr
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && *l != "Aborting process")
                .collect();
            return Err(format!("`{line}` failed: {}", said.last().copied().unwrap_or("no message")));
        }
        if line.split_whitespace().next() == Some("decompile") {
            if let Some(d) = dcp(&console) {
                if let (Some(prog), Some(fd)) = (d.conf.as_ref(), d.fd.as_ref()) {
                    on_decompile(prog, fd);
                }
            }
        }
        console.optr.clear();
        if let Some(f) = console.fileoptr.as_mut() {
            f.contents.clear();
        }
    }
    Ok(())
}

/// The console script that decompiles `functions` from an object file.
pub fn binary_script(path: &str, functions: &[String], options: &[(String, String)]) -> Vec<String> {
    let mut lines = vec![format!("load file {path}"), "option readonly on".to_string()];
    lines.extend(options.iter().map(|(n, v)| format!("option {n} {v}")));
    lines.push("read symbols".to_string());
    for f in functions {
        lines.push(format!("load function {f}"));
        lines.push("decompile".to_string());
    }
    lines
}

/// A datatest's program and script.
pub fn load_datatest(path: &str, spec_roots: &[String]) -> Result<(ConsoleProgram, Vec<String>), String> {
    let mut store = DocumentStorage::new();
    let doc = store.open_document(path).map_err(|e| e.explain().to_string())?;
    let root = doc.get_root().clone();
    if root.get_name() != "decompilertest" {
        return Err(format!("{path}: not a <decompilertest>"));
    }
    let mut prog = None;
    let mut commands = Vec::new();
    for child in root.get_children() {
        match child.get_name() {
            "binaryimage" => {
                let arch = child
                    .get_attribute_value("arch")
                    .map_err(|e| format!("{path}: <binaryimage>: {e}"))?;
                let arch = String::from_utf8_lossy(arch).into_owned();
                prog = Some(
                    bootstrap_program(Rc::clone(child), &arch, spec_roots)
                        .map_err(|e| format!("{path}: {}", e.explain()))?,
                );
            }
            "script" => {
                for com in child.get_children() {
                    let text: String = com
                        .get_content()
                        .iter()
                        .filter(|&&c| c != b'\r')
                        .map(|&c| if c == b'\n' { ' ' } else { c as char })
                        .collect();
                    commands.push(text);
                }
            }
            _ => {}
        }
    }
    let prog = prog.ok_or_else(|| format!("{path}: no <binaryimage>"))?;
    Ok((prog, commands))
}
