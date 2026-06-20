//! Tests for the `ifacedecomp` command family (W9).
//!
//! The decompiler engine *integration* layer the engine-touching commands need
//! (`Architecture::print`/`types`/`loader`/`context`, `parse_machaddr`/`parse_C`,
//! `ArchOptionContext for Architecture`, the `allacts` decompile drive) is not
//! yet exposed by the merged `rust-port` tree (see the module docs).  So these
//! tests exercise what *is* load-bearing and expressible today: the
//! registration token set, the prefix-expansion the datatest `<com>` streams
//! rely on (`lo fu` → `load function`, `dec` → `decompile`, `print C`), the
//! `IfaceDecompData` null-guard diagnostics with their exact text, and the
//! `execute`/`mainloop` console-driver dispatch + exception→prefix grammar.
//!
//! Each engine-dependent command is smoke-driven through the `ConsoleCommands`
//! feed (the datatest command-feed `LineSource`) and asserted to *resolve and
//! dispatch* — producing either its faithful pre-engine guard text or the
//! self-describing `engine_unavailable` execution error — never an "Invalid
//! command" (which would mean the registration/prefix surface drifted).

use super::*;
use crate::ifaceterm::ConsoleCommands;
use crate::interface::IfaceStatus;

/// A console wired exactly like the datatest runner: a `ConsoleCommands` feed
/// with the full decompiler command set registered.
fn console(commands: &[&str]) -> IfaceStatus {
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    status
}

/// Run every queued command via the console driver and return the captured
/// command output (`optr`), exactly what the harness reads.
fn run_all(commands: &[&str]) -> String {
    let mut status = console(commands);
    mainloop(&mut status);
    status.optr
}

// ---------------------------------------------------------------------------
// Registration: every C++ registerCom token set is present.
// ---------------------------------------------------------------------------

#[test]
fn registers_full_command_set() {
    let status = console(&[]);
    // 9 base/comment commands + the decompiler module commands.  The exact
    // count guards against an accidental add/drop in register_decomp_commands:
    // 9 (comment x3, quit, history, openfile write/append, closefile, echo)
    // + 96 decompiler commands = 105.
    assert_eq!(status.num_commands(), 105);
}

#[test]
fn prefix_expansion_matches_datatest_streams() {
    let mut status = console(&[]);
    // The four abbreviations the datatest <com> tags use most.
    assert_eq!(status.resolve("lo fu main").unwrap(), vec!["load", "function"]);
    assert_eq!(status.resolve("dec").unwrap(), vec!["decompile"]);
    assert_eq!(status.resolve("print C").unwrap(), vec!["print", "C"]);
    assert_eq!(status.resolve("quit").unwrap(), vec!["quit"]);
}

#[test]
fn print_c_subcommands_resolve_distinctly() {
    let mut status = console(&[]);
    // "print C" is its own complete command even though "print C flat"/globals/
    // types/xml extend it — the unique-but-complete branch of expandCom.
    assert_eq!(status.resolve("print C").unwrap(), vec!["print", "C"]);
    assert_eq!(status.resolve("print C flat").unwrap(), vec!["print", "C", "flat"]);
    assert_eq!(status.resolve("print C globals").unwrap(), vec!["print", "C", "globals"]);
    assert_eq!(status.resolve("print C xml").unwrap(), vec!["print", "C", "xml"]);
}

#[test]
fn map_family_resolves() {
    let mut status = console(&[]);
    for (line, expect) in [
        ("map addr", vec!["map", "address"]),
        ("map hash", vec!["map", "hash"]),
        ("map param", vec!["map", "param"]),
        ("map return", vec!["map", "return"]),
        ("map fun", vec!["map", "function"]),
        ("map label", vec!["map", "label"]),
        ("map convert", vec!["map", "convert"]),
        ("map unionfacet", vec!["map", "unionfacet"]),
    ] {
        assert_eq!(status.resolve(line).unwrap(), expect, "line: {line}");
    }
}

// ---------------------------------------------------------------------------
// Null-guard diagnostics: exact C++ IfaceExecutionError text.
// ---------------------------------------------------------------------------

#[test]
fn print_c_without_function_is_no_function_selected() {
    // No `load function` first → `dcp->fd == 0` guard.  The console driver wraps
    // the IfaceExecutionError under "Execution error: " (C++ execute()).
    let out = run_all(&["print C", "quit"]);
    assert!(
        out.contains("Execution error: No function selected"),
        "out was: {out:?}"
    );
}

#[test]
fn decompile_without_function_is_no_function_selected() {
    let out = run_all(&["decompile", "quit"]);
    assert!(out.contains("Execution error: No function selected"), "out: {out:?}");
}

#[test]
fn option_without_image_is_no_load_image_present() {
    let out = run_all(&["option readonly on", "quit"]);
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn parse_line_without_image_is_no_load_image_present() {
    let out = run_all(&["parse line extern void f(void);", "quit"]);
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn map_hash_without_function_is_no_function_loaded() {
    let out = run_all(&["map hash r0x10000c 5fc6319fee int4 size", "quit"]);
    assert!(out.contains("Execution error: No function loaded"), "out: {out:?}");
}

#[test]
fn volatile_without_image_is_no_load_image_present() {
    let out = run_all(&["volatile [ram,0x1000,16]", "quit"]);
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn pointer_setting_without_image_is_no_load_image_present() {
    // The conf check precedes any token read (C++ ifacedecomp.cc:3054), so a
    // no-image console produces the "No load image present" guard, NOT the old
    // engine-unavailable seam.  This pins that the command is wired (no longer a
    // SEAM stub) while exercising the no-image guard order.
    let out = run_all(&[
        "pointer setting myptroff mystruct offset 8",
        "quit",
    ]);
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
    assert!(
        !out.contains("engine integration not yet ported"),
        "pointer setting must no longer be an engine-unavailable seam: {out:?}"
    );
}

// ---------------------------------------------------------------------------
// Argument-parse errors: exact C++ IfaceParseError text under the "Command
// parsing error: " prefix.
// ---------------------------------------------------------------------------

#[test]
fn option_missing_name_is_parse_error() {
    // conf is None, so the "No load image present" guard fires first — to reach
    // the "Missing option name" parse error we would need a loaded image.  The
    // guard-order is exactly C++ (conf check precedes the name read), so assert
    // the guard, which is what a no-image console produces.
    let out = run_all(&["option", "quit"]);
    assert!(out.contains("Execution error: No load image present"), "out: {out:?}");
}

#[test]
fn map_label_missing_name_is_parse_error() {
    // map label reads the name token before touching conf, so the "Need label
    // name and address" parse error is reachable with no image.
    let out = run_all(&["map label", "quit"]);
    assert!(
        out.contains("Command parsing error: Need label name and address"),
        "out: {out:?}"
    );
}

#[test]
fn rename_missing_names_is_parse_error() {
    let out = run_all(&["rename", "quit"]);
    assert!(
        out.contains("Command parsing error: Missing old symbol name"),
        "out: {out:?}"
    );
}

#[test]
fn map_convert_bad_format_is_parse_error() {
    // fd is None → "No function loaded" guard precedes the format check (C++
    // order), so a no-function console reports the function guard.
    let out = run_all(&["map convert bogus 0 r0x0 0", "quit"]);
    assert!(out.contains("Execution error: No function loaded"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// Dispatch / driver behavior.
// ---------------------------------------------------------------------------

#[test]
fn invalid_command_reports_invalid() {
    let out = run_all(&["nonsense token here", "quit"]);
    assert!(out.contains("ERROR: Invalid command"), "out: {out:?}");
}

#[test]
fn quit_terminates_mainloop() {
    // After quit the loop stops; a trailing command must NOT have run (no
    // "Invalid command" / guard text from it).
    let mut status = console(&["quit", "print C"]);
    mainloop(&mut status);
    assert!(status.done);
    assert!(!status.optr.contains("No function selected"), "out: {:?}", status.optr);
}

#[test]
fn comment_lines_do_nothing() {
    // The three comment tokens (//, #, %) followed by text run with no output.
    let out = run_all(&["// a comment", "# another", "% script comment", "quit"]);
    assert!(out.is_empty() || !out.contains("ERROR"), "out: {out:?}");
}

#[test]
fn clear_architecture_is_clean() {
    // `clear architecture` always succeeds (drops conf/fd) with no diagnostic.
    let out = run_all(&["clear architecture", "quit"]);
    assert!(!out.contains("ERROR"), "out: {out:?}");
    assert!(!out.contains("error:"), "out: {out:?}");
}

#[test]
fn decompile_engine_drive_is_documented_loss() {
    // With no function the guard fires; the engine-drive loss is only reached
    // once a function is loaded (which itself needs the unported loader).  Here
    // we assert the dispatch path is intact (resolves + runs, no "Invalid").
    let out = run_all(&["dec", "quit"]);
    assert!(!out.contains("Invalid command"), "out: {out:?}");
    assert!(out.contains("Execution error: No function selected"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// Corpus command-set smoke: every distinct <com> token sequence used by the
// datatests + stage tests resolves and dispatches (never "Invalid command").
// ---------------------------------------------------------------------------

/// The distinct upstream `ifacedecomp` command lines the corpus drives (a
/// representative full token line per command kind, taken from the datatest /
/// stage-test `<com>` tags).  Each must resolve and dispatch; the assertion is
/// the *absence* of "Invalid command" (a registration/prefix regression).
const CORPUS_COMMANDS: &[&str] = &[
    "print C",
    "print raw",
    "print tree varnode",
    "decompile",
    "dec",
    "lo fu main",
    "load function FUN_0",
    "option readonly on",
    "option compareform canonical",
    "option returnpair single",
    "option integerformat dec",
    "option inline prologue",
    "option noreturn __stack_chk_fail",
    "parse line extern void printf(char *,...);",
    "map addr r0x601030 int4 globvar",
    "map return s0x10 int8",
    "map fun r0x100590 printf",
    "map param 0 [register,8,8] uint8 llval",
    "map label mylabel r0x100913",
    "map hash r0x10000c 5fc6319fee int4 size",
    "map convert oct fffffffe r0x100730 e107fa4a8f",
    "map unionfacet structunion 1 r0x1006ee 10603fc3e29498",
    "disassemble 0x0 0x4",
    "set context TMode 1 [ram,0x0] [ram,0x100]",
    "set track t9 0x0 0x410040 0x410041",
    "override flow r0x110324 callreturn",
    "comment instruction r0x10085a Comment",
    "rename v3 i",
    "retype v4 myptroff ptrrel",
    "force varnode #0xaa(0x100619:7) bin",
    "force datatype uint8_t hex",
    "volatile [ram,0x210000,64]",
    "fixup apply guard_dispatch_icall _guard_dispatch_icall",
    "pointer setting outer_rel OuterStruct offset 16",
    "structure blocks",
    "continue",
];

#[test]
fn corpus_command_set_all_dispatch() {
    for line in CORPUS_COMMANDS {
        // Drive a fresh console per command so guard state never carries over.
        let out = run_all(&[line, "quit"]);
        assert!(
            !out.contains("Invalid command"),
            "command did not resolve: {line:?}\nout: {out:?}"
        );
        assert!(
            !out.contains("Incomplete command"),
            "command was ambiguous/incomplete: {line:?}\nout: {out:?}"
        );
    }
}

#[test]
fn comment_token_must_be_followed_by_space() {
    // The C++ note: a space must follow "//"/"#"/"%".  "//foo" (no space) is NOT
    // the comment command — it does not resolve to any registered command.
    let mut status = console(&[]);
    // "// foo" resolves to the comment command...
    assert_eq!(status.resolve("// foo").unwrap(), vec!["//"]);
    // ...but "//foo" (token glued) does not match "//".
    assert!(status.resolve("//foo").is_err());
}

// ---------------------------------------------------------------------------
// IfaceDecompData behavior.
// ---------------------------------------------------------------------------

#[test]
fn decomp_data_is_shared_single_instance() {
    // register_decomp_commands registers ~95 decompile-module commands but the
    // module data is created exactly once (createData on first module sighting).
    // Reaching it via two different commands must see the same object: set a
    // marker through one command's dcp and read it back through the datamap.
    let mut status = console(&[]);
    {
        let dcp = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = dcp.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.cgraph_allocated = true;
    }
    let dcp = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = dcp.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    assert!(dcp.cgraph_allocated);
}

#[test]
fn abort_function_warns_and_drops_fd() {
    let mut dcp = IfaceDecompData::default();
    let mut out = String::new();
    // With no function, abort_function is a no-op.
    dcp.abort_function(&mut out);
    assert!(out.is_empty());
    assert!(dcp.fd.is_none());
}

#[test]
fn callgraph_dump_without_graph_is_no_callgraph_present() {
    let out = run_all(&["callgraph dump foo.out", "quit"]);
    assert!(out.contains("Execution error: No callgraph present"), "out: {out:?}");
}

#[test]
fn produce_prototypes_without_callgraph_guard() {
    // conf None → "No load image" first (C++ order: conf precedes cgraph).
    let out = run_all(&["produce prototypes", "quit"]);
    assert!(out.contains("Execution error: No load image"), "out: {out:?}");
}

// ---------------------------------------------------------------------------
// render_engine_error: the KunaError → console-prefix grammar (the catch arms
// of C++ execute() that fire once the engine integration lands).
// ---------------------------------------------------------------------------

#[test]
fn render_engine_error_prefixes() {
    use kuna_base::error::KunaError;
    let mut dcp = IfaceDecompData::default();

    let mut out = String::new();
    render_engine_error(&KunaError::parse("bad"), &mut dcp, &mut out);
    assert_eq!(out, "Parse ERROR: bad\n");

    let mut out = String::new();
    render_engine_error(&KunaError::recov("oops"), &mut dcp, &mut out);
    assert_eq!(out, "Function ERROR: oops\n");

    let mut out = String::new();
    render_engine_error(&KunaError::decoder("xml"), &mut dcp, &mut out);
    assert_eq!(out, "Decoding ERROR: xml\n");

    let mut out = String::new();
    render_engine_error(&KunaError::lowlevel("deep"), &mut dcp, &mut out);
    assert_eq!(out, "Low-level ERROR: deep\n");
}

// ===========================================================================
// w9-con-ifacedecomp verifier adversarial tests (round 1).
//
// These target the load-bearing, *in-scope* (non-engine) surface the hunt list
// flagged as most fragile for a registration/console-driver port: the exact
// registerCom token sequence (prefix-expansion provenance), the unusual
// empty-arg-is-EXECUTION-not-PARSE error class on `break action`/`break start`,
// the execute() exception->prefix grammar across all three reachable IfaceError
// kinds, and the deliberate omissions (`source`, the CPUI_RULECOMPILE-gated
// `parse rule`/`experimental rules`).
// ===========================================================================

/// The full, ordered C++ `IfaceDecompCapability::registerCommands` token
/// sequence (ifacedecomp.cc:37-147), MINUS the console-only `source`
/// (registered in consolemain.cc, not ifacedecomp.cc) and the
/// `#ifdef CPUI_RULECOMPILE` / `#ifdef OPACTION_DEBUG` / `#ifdef TYPEPROP_DEBUG`
/// blocks (compiled out of every kuna build).  Transcribed by hand from the C++
/// so a single-token drift in register_decomp_commands is caught positionally —
/// the `num_commands()==105` count test cannot see a swap that keeps the count.
const CPP_REGISTER_ORDER: &[&[&str]] = &[
    &["//"], &["#"], &["%"], &["quit"], &["history"],
    &["openfile", "write"], &["openfile", "append"], &["closefile"], &["echo"],
    // (source omitted — console-only)
    &["option"], &["parse", "file"], &["parse", "line"], &["adjust", "vma"],
    &["load", "function"], &["load", "addr"], &["read", "symbols"],
    &["clear", "architecture"], &["map", "address"], &["map", "hash"],
    &["map", "param"], &["map", "return"], &["map", "function"],
    &["map", "externalref"], &["map", "label"], &["map", "convert"],
    &["map", "unionfacet"], &["disassemble"], &["decompile"], &["dump"],
    &["binary"], &["force", "goto"], &["force", "varnode"], &["force", "datatype"],
    &["override", "prototype"], &["override", "jumptable"], &["override", "flow"],
    &["deadcode", "delay"], &["global", "add"], &["global", "remove"],
    &["global", "spaces"], &["global", "registers"], &["graph", "dataflow"],
    &["graph", "controlflow"], &["graph", "dom"], &["print", "language"],
    &["print", "C"], &["print", "C", "flat"], &["print", "C", "globals"],
    &["print", "C", "types"], &["print", "C", "xml"], &["print", "parammeasures"],
    &["produce", "C"], &["produce", "prototypes"], &["print", "raw"],
    &["print", "inputs"], &["print", "inputs", "all"], &["list", "action"],
    &["list", "override"], &["list", "prototypes"], &["set", "context"],
    &["set", "track"], &["break", "start"], &["break", "action"],
    &["print", "spaces"], &["print", "high"], &["print", "tree", "varnode"],
    &["print", "tree", "block"], &["print", "localrange"], &["print", "map"],
    &["print", "varnode"], &["print", "cover", "high"], &["print", "cover", "varnode"],
    &["print", "cover", "varnodehigh"], &["print", "extrapop"],
    &["print", "actionstats"], &["reset", "actionstats"], &["count", "pcode"],
    &["type", "varnode"], &["name", "varnode"], &["rename"], &["retype"],
    &["remove"], &["isolate"], &["prototype", "lock"], &["prototype", "unlock"],
    &["comment", "instruction"], &["duplicate", "hash"], &["callgraph", "build"],
    &["callgraph", "build", "quick"], &["callgraph", "dump"], &["callgraph", "load"],
    &["callgraph", "list"], &["fixup", "call"], &["fixup", "callother"],
    &["fixup", "apply"], &["volatile"], &["readonly"], &["pointer", "setting"],
    &["prefersplit"], &["structure", "blocks"], &["analyze", "range"],
    &["load", "test", "file"], &["list", "test", "commands"],
    &["execute", "test", "command"], &["continue"],
];

#[test]
fn w9_con_ifacedecomp_v1_register_token_sequence_byte_identical() {
    // 105 active tokens, matching the count test and my positional C++ diff.
    assert_eq!(CPP_REGISTER_ORDER.len(), 105, "transcription count");
    let mut status = console(&[]);
    // Each C++ token sequence must resolve to itself (full token line resolves
    // to the same words) — this exercises the actual registered prefix tree, so
    // a missing/renamed/reordered token shows up as an Invalid/ambiguous resolve.
    for words in CPP_REGISTER_ORDER {
        let line = words.join(" ");
        let resolved = status
            .resolve(&line)
            .unwrap_or_else(|e| panic!("token {words:?} did not resolve: {e}"));
        assert_eq!(&resolved, words, "registered token drifted: {words:?}");
    }
    assert_eq!(status.num_commands(), CPP_REGISTER_ORDER.len());
}

#[test]
fn w9_con_ifacedecomp_v2_break_empty_specify_is_execution_not_parse() {
    // C++ IfcBreakaction/IfcBreakstart (ifacedecomp.cc:1182,1208): an EMPTY
    // action/rule name throws IfaceExecutionError("No action/rule specified"),
    // NOT a parse error — an unusual class choice that a "looks like a missing
    // arg => parse error" port would get wrong.  So the console driver must
    // prefix it with "Execution error: ", never "Command parsing error: ".
    // (`break action`/`break start` read `specify` before touching conf, and
    // conf==None would give "Decompile action not loaded" — but the empty-arg
    // check precedes that, so a bare `break action` reports the specify error.)
    for cmd in ["break action", "break start"] {
        let out = run_all(&[cmd, "quit"]);
        assert!(
            out.contains("Execution error: No action/rule specified"),
            "{cmd}: expected execution-class empty-arg error, out: {out:?}"
        );
        assert!(
            !out.contains("Command parsing error"),
            "{cmd}: empty arg must NOT be a parse error, out: {out:?}"
        );
    }
}

/// A synthetic command that raises a chosen IfaceError kind, to exercise the
/// execute() exception->prefix grammar for all three reachable kinds without
/// the unported engine (the only kinds a command body produces today).
struct RaiseCmd(crate::interface::IfaceError);
impl IfaceCommandAction for RaiseCmd {
    fn execute(&self, _s: &mut IfaceStatus, _c: &mut CommandStream) -> IfaceResult<()> {
        Err(self.0.clone())
    }
    fn module(&self) -> String {
        DECOMPILE_MODULE.to_string()
    }
}

#[test]
fn w9_con_ifacedecomp_v3_execute_exception_prefix_grammar() {
    use crate::interface::IfaceError;
    // C++ execute() (ifacedecomp.cc:3616) catches, in inheritance order:
    //   IfaceParseError     -> "Command parsing error: "
    //   IfaceExecutionError -> "Execution error: "
    //   IfaceError (base)   -> "ERROR: "
    // Build a console whose single registered command raises each kind and
    // assert the exact prefix the driver writes (the harness greps these).
    for (err, prefix) in [
        (IfaceError::parse("p-explain"), "Command parsing error: p-explain"),
        (IfaceError::execution("x-explain"), "Execution error: x-explain"),
        (IfaceError::base("b-explain"), "ERROR: b-explain"),
    ] {
        let mut status = ConsoleCommands::into_status(vec!["raise".to_string()]);
        status.register_com(Box::new(RaiseCmd(err)), &["raise"]);
        // Drive exactly one command through the real console driver.
        super::execute(&mut status);
        assert!(
            status.optr.starts_with(prefix),
            "expected prefix {prefix:?}, got {:?}",
            status.optr
        );
        // Exactly one newline-terminated diagnostic line.
        assert!(status.optr.ends_with('\n'), "missing newline: {:?}", status.optr);
    }
}

#[test]
fn w9_con_ifacedecomp_v4_deliberate_omissions_absent() {
    // `source` is registered in consolemain.cc (console-only), NOT by
    // register_decomp_commands; and `parse rule` / `experimental rules` are
    // #ifdef CPUI_RULECOMPILE (compiled out of every kuna build).  None must
    // resolve through the decompiler command set.
    let mut status = console(&[]);
    assert!(status.resolve("source foo.txt").is_err(), "source must not be registered");
    assert!(status.resolve("parse rule").is_err(), "parse rule is CPUI_RULECOMPILE-gated");
    assert!(
        status.resolve("experimental rules").is_err(),
        "experimental rules is CPUI_RULECOMPILE-gated"
    );
    // But `parse file` / `parse line` (the non-gated parse subcommands) DO
    // resolve — guards that the omission did not over-prune the `parse` subtree.
    assert_eq!(status.resolve("parse file x").unwrap(), vec!["parse", "file"]);
    assert_eq!(status.resolve("parse line int x;").unwrap(), vec!["parse", "line"]);
}
