//! End-to-end gate for the Go GC write-barrier fold (`gowritebarrier`,
//! `docs/multilang.md` W2/W5 — the guarded-cold-call idiom).
//!
//! The proof, on a REAL stripped Go build (names from the pclntab alone, the
//! language from the buildinfo section alone):
//! - default (`auto`): a function with guarded pointer stores decompiles with
//!   NO `gcWriteBarrier` anywhere, and the program's own global store is still
//!   in the C;
//! - `option gowritebarrier off`: the same function renders the thunk call.
//!
//! The hermetic matcher logic is pinned by the `kuna_gowritebarrier` unit
//! tests and `tests/stages/kuna-gowritebarrier.xml`; this gate is the
//! detection-driven `auto` path over a genuine Go image (the
//! `verify_go_morestack` pattern, same hermetic build recipe).

use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `true` if a runnable `go` toolchain is on PATH.
fn go_available() -> bool {
    Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// `main.push` writes two heap pointers (`n.next = head; head = n`), so the
/// compiler plants a write-barrier diamond in front of each store group.
const MAIN_GO: &str = r#"package main

import "os"

type Node struct {
	next *Node
	val  int
}

var head *Node

//go:noinline
func push(n *Node) {
	n.next = head
	head = n
}

func main() {
	for i := 0; i < len(os.Args); i++ {
		push(&Node{val: i})
	}
	if head != nil {
		os.Exit(head.val)
	}
}
"#;

/// Build [`MAIN_GO`] stripped (`-ldflags=-s -w`) for linux/amd64, hermetically.
/// A function that stages a Go `(ptr,len)`/pointer return across a write
/// barrier: `a := &Node{}; a.next = n; …; return a, b`. The barrier guards the
/// pointer-field store and the object pointer is also the return value — the
/// shape an earlier phi fold miscompiled (adversarial review: orphaned
/// `runtime.newobject`, lost return). The fold must leave it alone.
const RETURN_GO: &str = r#"package main

import "os"

type Node struct {
	next *Node
	val  int
}

var head *Node

//go:noinline
func mk2(n *Node, v int) (*Node, *Node) {
	a := &Node{val: v}
	a.next = n
	b := &Node{val: v + 1}
	b.next = a
	return a, b
}

func main() {
	for i := 0; i < len(os.Args); i++ {
		a, b := mk2(head, i)
		head = a
		_ = b
	}
	if head != nil {
		os.Exit(head.val)
	}
}
"#;

fn build_go(dir: &std::path::Path, out_bin: &std::path::Path) -> bool {
    build_go_src(dir, out_bin, MAIN_GO)
}

fn build_go_src(dir: &std::path::Path, out_bin: &std::path::Path, src: &str) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.go"), src).is_err() {
        return false;
    }
    Command::new("go")
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(out_bin)
        .arg("main.go")
        .current_dir(dir)
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Bootstrap, optionally flip `gowritebarrier`, commit the analysis facts,
/// then decompile `main.push` and return the console transcript.
fn decompile_push(bin: &str, option: Option<&str>) -> String {
    decompile_fns(bin, option, &["main.push".to_string()])
}

/// One console session: bootstrap, flip the option, decompile every function
/// in `names`, return the combined transcript.
fn decompile_fns(bin: &str, option: Option<&str>, names: &[String]) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    if let Some(v) = option {
        prog.arch_mut().set_kuna_option("gowritebarrier", v).expect("gowritebarrier applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let mut cmds: Vec<String> = Vec::new();
    for n in names {
        cmds.push(format!("load function {n}"));
        cmds.push("decompile".to_string());
        cmds.push("print C".to_string());
    }
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The auto path end to end: detection (buildinfo) → `source_lang` fact →
/// the fold; and `off` restores the barrier for auditing.
#[test]
fn stripped_go_pointer_store_folds_its_write_barrier_under_auto() {
    if !go_available() {
        eprintln!("verify_go_writebarrier: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_wb_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    let kept = decompile_push(bin, Some("off"));
    assert!(
        kept.contains("gcWriteBarrier"),
        "with gowritebarrier off the thunk call must render; transcript:\n{kept}"
    );

    let folded = decompile_push(bin, None);
    assert!(
        !folded.contains("gcWriteBarrier") && !folded.contains("writeBarrier"),
        "under the auto default a Go binary's write barrier must fold; transcript:\n{folded}"
    );
    // The fold deletes GC bookkeeping, never the program's store: the global
    // head-pointer assignment (a `dat_<addr> =` store) survives.
    assert!(
        folded.contains("dat_") && folded.contains(" = "),
        "the program's own stores survive the fold; transcript:\n{folded}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Rename-insensitive program-store counter for the preservation oracle: the
/// number of memory-write statements (`*p = …`, `p[i] = …`, `s->f = …`)
/// outside barrier arms (`if` blocks whose body calls `gcWriteBarrier`) and
/// not through the unmodeled thunk register (locals declared `// …r11…`).
/// The fold legitimately re-renders ADDRESS EXPRESSIONS once the join phis
/// collapse (struct fields appear, builtins form, call results bind to locals
/// instead of slots), so the invariant is the count, never the text shape.
fn program_store_count(code: &str) -> usize {
    let lines: Vec<&str> = code.lines().collect();
    // Barrier-arm exclusion: brace-balanced `if` blocks mentioning the thunk.
    let mut excluded = vec![false; lines.len()];
    for i in 0..lines.len() {
        let bare = lines[i].split("//").next().unwrap_or("").trim_end();
        if bare.contains("if (") && bare.ends_with('{') {
            let mut depth = 0i32;
            let mut body = Vec::new();
            for (j, l) in lines.iter().enumerate().skip(i) {
                depth += l.matches('{').count() as i32 - l.matches('}').count() as i32;
                body.push(j);
                if depth <= 0 && j > i {
                    break;
                }
            }
            if body.iter().any(|&k| lines[k].contains("gcWriteBarrier")) {
                for k in body {
                    excluded[k] = true;
                }
            }
        }
    }
    // Phantom thunk-register locals (`unsigned long *v5; // r11`).
    let mut bufvars: Vec<String> = Vec::new();
    for l in &lines {
        if let Some((decl, comment)) = l.split_once("//") {
            if comment.contains("r11") && decl.trim_end().ends_with(';') {
                if let Some(tok) = decl
                    .trim_end()
                    .trim_end_matches(';')
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .filter(|t| !t.is_empty())
                    .last()
                {
                    bufvars.push(tok.to_string());
                }
            }
        }
    }
    let mut n = 0;
    for (i, l) in lines.iter().enumerate() {
        if excluded[i] {
            continue;
        }
        let bare = l.split("//").next().unwrap_or("").trim();
        if !bare.ends_with(';') {
            continue;
        }
        let Some((lhs, _)) = bare.split_once(" = ") else { continue };
        if !lhs.contains('*') && !lhs.contains('[') && !lhs.contains("->") {
            continue;
        }
        if bufvars.iter().any(|b| lhs.contains(b.as_str())) {
            continue;
        }
        n += 1;
    }
    n
}

/// The post-SSA arm end to end, with the store-preservation oracle: every
/// runtime function whose `off` rendering carries a barrier keeps at least as
/// many program stores under the default fold, and most of them fold clean.
#[test]
fn phi_fed_barrier_arms_fold_and_preserve_every_program_store() {
    if !go_available() {
        eprintln!("verify_go_writebarrier: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_wbphi_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_wbphi");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    // Known multi-op / shared-arm carriers, present across go1.22..1.27 (the
    // phi matcher's residual class under the pre-SSA fold alone).
    let witnesses: Vec<String> = [
        "runtime.makechan",
        "runtime.send",
        "runtime.convTstring",
        "runtime.acquireSudog",
        "runtime.(*mheap).allocMSpanLocked",
        "runtime.setProfLabel",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let off = decompile_fns(bin, Some("off"), &witnesses);
    let on = decompile_fns(bin, None, &witnesses);
    let split = |t: &str| -> Vec<String> {
        // Per-function transcripts separated by the console's decompile echo.
        t.split("Decompiling ").map(|s| s.to_string()).collect()
    };
    let (offs, ons) = (split(&off), split(&on));
    assert_eq!(offs.len(), ons.len(), "same function count both ways");
    let mut folded = 0;
    let mut carried = 0;
    for (fo, fn_) in offs.iter().zip(ons.iter()).skip(1) {
        if !fo.contains("gcWriteBarrier") {
            continue;
        }
        carried += 1;
        let before = program_store_count(fo);
        let after = program_store_count(fn_);
        assert!(
            after >= before,
            "program stores lost ({before} -> {after}); off:
{fo}
== on:
{fn_}"
        );
        if !fn_.contains("gcWriteBarrier") {
            folded += 1;
        }
    }
    assert!(carried >= 3, "fixture must exercise the residual class (got {carried})");
    assert!(folded * 2 >= carried, "most residual arms fold ({folded}/{carried})");

    let _ = std::fs::remove_dir_all(&dir);
}

/// Regression (adversarial review, 2026-11): the phi fold must not touch a
/// write barrier whose updated object pointer threads the function's return —
/// folding it orphaned the `runtime.newobject` binding and dropped the return.
/// On the staged-return fixture, `main.mk`'s barrier is refused, so `auto`
/// equals `off`: the return and the allocation survive.
#[test]
fn staged_return_across_barrier_is_not_folded() {
    if !go_available() {
        eprintln!("verify_go_writebarrier: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_wbret_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_wbret");
    if !build_go_src(&dir, &out_bin, RETURN_GO) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the staged-return fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");
    let names = ["main.mk2".to_string()];

    let off = decompile_fns(bin, Some("off"), &names);
    let on = decompile_fns(bin, None, &names);

    // The refusal keeps the barrier, so the two renderings agree on the
    // load-bearing facts: the allocation is captured and a value is returned,
    // never a bare `return;` with the object dropped.
    for (tag, c) in [("off", &off), ("auto", &on)] {
        let body = c[c.find('{').unwrap_or(0)..].to_string();
        let bare_only = body.contains("return;")
            && !body.lines().any(|l| {
                let t = l.trim();
                t.starts_with("return ") && t != "return;"
            });
        assert!(
            !bare_only,
            "main.mk2 ({tag}) must return its value, not a bare return; transcript:\n{c}"
        );
        assert!(
            c.contains("gcWriteBarrier"),
            "main.mk2 ({tag}) keeps its return-threading barrier (fold must refuse it); transcript:\n{c}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
