//! A declared function name answers out of the built-in libc signature tables —
//! `docs/re-needs/both-argument-recovery-options.md`.
//!
//! `libproto` and `libcsigs` match a name the IMAGE carries: its FUNC symbols, its
//! imports. A stripped, statically linked target has neither, so the only evidence
//! a callee is `ptrace` is the operator's `--define-function 0x8049027=ptrace` —
//! and a name that buys no prototype buys almost nothing, because the callee's
//! arity has to come from somewhere and glibc's `ptrace` is a varargs wrapper
//! (`long ptrace(enum __ptrace_request, ...)`, the rest fetched with `va_arg`) that
//! no body analysis can answer for.
//!
//! The two-state proof on `declaredlibcproto_i386` (the reduction of the witness,
//! crackmes.one `5ab77f5c33c5d40ad448c615`):
//!
//!  - **declared, option off**: `ptrace(2,v1,0,0x804a004)` — the spelling and
//!    nothing else.
//!  - **declared, default**: `ptrace(2,v1,NULL,(void *)0x804a004)` — the signature
//!    typed the third and fourth arguments.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The caller, whose loop pushes four `ptrace` arguments per iteration.
const CALLER: u64 = 0x8049000;
/// The varargs wrapper the operator identifies as `ptrace`.
const WRAPPER: u64 = 0x8049027;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with `declaredlibcproto` set to `on`. `None` ⇒ specs-less skip.
fn load(option_on: bool) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin =
        root.join("decompiler/crates/kuna-analysis/tests/fixtures/declaredlibcproto_i386");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_declaredlibcproto: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("declaredlibcproto", if option_on { "on" } else { "off" })
        .expect("the option is settable");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog.arch().manage().get_default_code_space().expect("code space").clone();
    Address::new(Rc::clone(&space), vma)
}

/// Declare the wrapper as `ptrace`, then render the caller.
fn caller_body(option_on: bool) -> Option<String> {
    let mut prog = load(option_on)?;
    let wrapper = code_addr(&prog, WRAPPER);
    let name = prog.declare_function(wrapper, Some("ptrace"), 0).expect("the declaration lands");
    assert_eq!(name, "ptrace");

    let entry = code_addr(&prog, CALLER);
    let declared = prog.declared_extent(CALLER);
    let step = decompile_one(
        prog.arch_mut(),
        "sub_8049000",
        entry,
        declared,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the caller decompiles");
    Some(kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd))
}

/// Default: the declared name carries its signature to every call site.
#[test]
fn a_declared_libc_name_types_the_call_it_names() {
    let Some(body) = caller_body(true) else { return };
    assert!(
        body.contains("ptrace(2,") && body.contains(",NULL,(void *)0x804a004)"),
        "the parked signature must type the pointer arguments, got:\n{body}"
    );
}

/// Option off: the declaration still names the callee and nothing more — the state
/// the need recorded, so the fix cannot be mistaken for something discovery did.
#[test]
fn with_the_option_off_the_name_arrives_without_the_signature() {
    let Some(body) = caller_body(false) else { return };
    assert!(body.contains("ptrace("), "the declared name still renders, got:\n{body}");
    assert!(
        !body.contains("NULL"),
        "no signature is parked, so nothing types the pointer arguments, got:\n{body}"
    );
}

/// A declared name neither table knows is left exactly alone — the lookup must not
/// invent a prototype for `stage1`.
#[test]
fn a_name_the_tables_do_not_know_parks_nothing() {
    let Some(mut prog) = load(true) else { return };
    let wrapper = code_addr(&prog, WRAPPER);
    prog.declare_function(wrapper, Some("stage1"), 0).expect("the declaration lands");

    let entry = code_addr(&prog, CALLER);
    let step = decompile_one(
        prog.arch_mut(),
        "sub_8049000",
        entry,
        0,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the caller decompiles");
    let body = kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd);
    assert!(body.contains("stage1("), "the declared name renders, got:\n{body}");
    assert!(
        !body.contains("NULL"),
        "an unknown name must not acquire a signature, got:\n{body}"
    );
}
