//! Driving the decompiler console in-process, so each `decompile` hands its
//! finished `Funcdata` to the check before the next command replaces it.

use std::rc::Rc;

use kuna_base::xml::DocumentStorage;
use kuna_console::engine::{bootstrap_program, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_console_commands, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::{quote_filename, IfaceStatus};
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

/// A command that failed in a script that keeps going: its index and message.
pub type Failure = (usize, String);

/// The wall-clock budget of a capped decompile: without a rule some actions
/// never settle.
const CAPPED_BUDGET: std::time::Duration = std::time::Duration::from_secs(5);

/// Rule applications counted, or one rule's limited, in one decompile: the
/// `decompile`-th one [`run`] reports (`kuna_rulecap`). The run stops after it.
pub struct Cap {
    pub decompile: usize,
    pub mode: kuna_decomp::kuna_rulecap::Mode,
    pub result: Option<kuna_decomp::kuna_rulecap::Record>,
}

/// Run `commands`, calling `on_decompile` after every `decompile` that
/// succeeds, however the line abbreviates it, except one after a failed
/// `load`: the function it would decompile is the one loaded before. `conf` preloads a program (the datatest `<binaryimage>`). The first
/// failing command ends the run unless `keep_going`, which records it and goes
/// on. Returns the console, so the caller can read the program the script
/// loaded, and the failures it went past.
pub fn run(
    commands: Vec<String>,
    conf: Option<ConsoleProgram>,
    spec_roots: &[String],
    keep_going: bool,
    cap: Option<&mut Cap>,
    mut on_decompile: impl FnMut(&ConsoleProgram, &Funcdata),
) -> Result<(IfaceStatus, Vec<Failure>), String> {
    run_steps(commands, conf, spec_roots, keep_going, cap, |step| {
        if let Step::Decompiled { prog, fd, .. } = step {
            on_decompile(prog, fd);
        }
    })
}

/// What [`run_steps`] reports as it goes, with the index of the command.
pub enum Step<'a> {
    Decompiled { line: usize, prog: &'a ConsoleProgram, fd: &'a Funcdata },
    Failed { line: usize, msg: &'a str },
}

/// [`run`], reporting each failure as it happens too.
pub fn run_steps(
    commands: Vec<String>,
    conf: Option<ConsoleProgram>,
    spec_roots: &[String],
    keep_going: bool,
    mut cap: Option<&mut Cap>,
    mut on_step: impl FnMut(Step<'_>),
) -> Result<(IfaceStatus, Vec<Failure>), String> {
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
    let mut failures = Vec::new();
    let mut stale = false;
    let mut decompiles = 0usize;
    let result = loop {
        if console.is_stream_finished() {
            break Ok(());
        }
        let capped = cap.as_deref().is_some_and(|c| c.decompile == decompiles);
        if let Some(c) = cap.as_deref().filter(|_| capped) {
            if let Some(prog) = dcp_mut(&mut console).and_then(|d| d.conf.as_mut()) {
                prog.arch_mut().kuna_fn_budget = Some(CAPPED_BUDGET);
            }
            kuna_decomp::kuna_rulecap::arm(c.mode.clone());
        }
        execute(&mut console);
        let applied = capped.then(kuna_decomp::kuna_rulecap::disarm);
        let line = commands.get(idx).map(|s| s.trim().to_string()).unwrap_or_default();
        idx += 1;
        let command = console.last_command.first().cloned().unwrap_or_default();
        if console.is_in_error() {
            if command == "load" {
                stale = true;
            }
            let said: Vec<&str> = console
                .optr
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && *l != "Aborting process")
                .collect();
            let msg = format!("`{line}` failed: {}", said.last().copied().unwrap_or("no message"));
            if !keep_going {
                break Err(msg);
            }
            on_step(Step::Failed { line: idx - 1, msg: &msg });
            failures.push((idx - 1, msg));
            console.inerror = false;
            console.done = false;
        } else if command == "load" {
            stale = false;
        } else if command == "decompile" && !stale {
            if let Some(d) = dcp(&console) {
                if let (Some(prog), Some(fd)) = (d.conf.as_ref(), d.fd.as_ref()) {
                    if let (Some(c), Some(a)) = (cap.as_deref_mut(), applied) {
                        c.result = Some(a);
                    }
                    on_step(Step::Decompiled { line: idx - 1, prog, fd });
                    decompiles += 1;
                }
            }
            if capped {
                break Ok(());
            }
        }
        console.optr.clear();
        if let Some(f) = console.fileoptr.as_mut() {
            f.contents.clear();
        }
    };
    kuna_decomp::kuna_rulecap::disarm();
    result.map(|()| (console, failures))
}

/// [`run`], keeping only whether it succeeded.
pub fn drive(
    commands: Vec<String>,
    conf: Option<ConsoleProgram>,
    spec_roots: &[String],
    on_decompile: impl FnMut(&ConsoleProgram, &Funcdata),
) -> Result<(), String> {
    run(commands, conf, spec_roots, false, None, on_decompile).map(|_| ())
}

pub fn load_lines(path: &str, options: &[(String, String)]) -> Vec<String> {
    let mut lines = vec![format!("load file {}", quote_filename(path)), "option readonly on".to_string()];
    let named = |name: &str| options.iter().any(|(n, _)| n == name);
    lines.extend(
        driver_defaults(non_x86_64(path))
            .into_iter()
            .filter(|(n, _)| !named(n))
            .map(|(n, v)| format!("option {n} {v}")),
    );
    lines.extend(options.iter().map(|(n, v)| format!("option {n} {v}")));
    lines.push("read symbols".to_string());
    lines
}

/// The options `kuna decompile` turns on for an object file unless named, so
/// the function checked is the one it prints: the Listing that call-graph
/// no-return propagation reads, and off x86-64 the discovery passes. The
/// instruction-budget truncation it also turns on is left off: a truncated
/// body is not the function.
fn driver_defaults(non_x86_64: bool) -> Vec<(&'static str, &'static str)> {
    let mut on = vec![("listing", "on")];
    if non_x86_64 {
        on.extend([("funcstart_patterns", "on"), ("aif", "on")]);
    }
    on
}

/// Whether an ELF, PE or Mach-O header names a machine other than x86-64.
/// Anything else is taken as x86-64, as `kuna` takes an image it cannot parse.
fn non_x86_64(path: &str) -> bool {
    let Ok(bytes) = std::fs::read(path) else { return false };
    let u16_at = |o: usize| bytes.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32_at = |o: usize| bytes.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if bytes.starts_with(b"\x7fELF") {
        let machine = if bytes.get(5) == Some(&2) { u16_at(18).map(u16::swap_bytes) } else { u16_at(18) };
        return machine.is_some_and(|m| m != 62);
    }
    if bytes.starts_with(b"MZ") {
        return u32_at(0x3c).and_then(|pe| u16_at(pe as usize + 4)).is_some_and(|m| m != 0x8664);
    }
    match u32_at(0) {
        Some(0xfeed_facf | 0xfeed_face) => u32_at(4).is_some_and(|cpu| cpu != 0x0100_0007),
        _ => false,
    }
}

/// The entry address of every function with code in an object file.
pub fn function_entries(path: &str, options: &[(String, String)], spec_roots: &[String]) -> Result<Vec<u64>, String> {
    Ok(function_extents(path, options, spec_roots)?.into_iter().map(|(at, _)| at).collect())
}

/// [`function_entries`] with each function's byte extent, 0 where unknown.
pub fn function_extents(path: &str, options: &[(String, String)], spec_roots: &[String]) -> Result<Vec<(u64, u64)>, String> {
    let (console, _) = run(load_lines(path, options), None, spec_roots, false, None, |_, _| {})?;
    let prog = dcp(&console).and_then(|d| d.conf.as_ref()).ok_or("no program loaded")?;
    Ok(prog
        .function_entries_executable()
        .into_iter()
        .filter(|e| prog.entry_bytes_mapped(&e.addr))
        .map(|e| (e.addr.get_offset(), e.size))
        .collect())
}

/// Load an object file and hand its program to `f`.
pub fn with_program<R>(
    path: &str,
    options: &[(String, String)],
    spec_roots: &[String],
    f: impl FnOnce(&ConsoleProgram) -> R,
) -> Result<R, String> {
    let (console, _) = run(load_lines(path, options), None, spec_roots, false, None, |_, _| {})?;
    let prog = dcp(&console).and_then(|d| d.conf.as_ref()).ok_or("no program loaded")?;
    Ok(f(prog))
}

/// The console script that decompiles `functions` from an object file. A
/// function is a name, or an address written `0x...`.
pub fn binary_script(path: &str, functions: &[String], options: &[(String, String)]) -> Vec<String> {
    let mut lines = load_lines(path, options);
    for f in functions {
        let verb = if f.starts_with("0x") { "addr" } else { "function" };
        lines.push(format!("load {verb} {f}"));
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
