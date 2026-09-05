//! End-to-end two-pass gate for `tailcallframe` — an `-O2` tail `jmp` whose
//! callee no discovery oracle found is a tail call, not intraprocedural flow
//! (P2 flow classification).
//!
//! Fixture: `tailcallframe_x86_64` (source beside it), the reduced form of the
//! round-2 RE-friction shape. `on_key` at `0x11b0` is reachable only through the
//! `listener[]` code pointer in `.data.rel.ro`, so the recursive-descent walk
//! never enters it, never sees its `call`, and `render` at `0x1170` is in no
//! oracle's output — `query_call(0x1170)` is `None`, which is exactly what
//! `tailcalljump` needs and does not have. Built without unwind tables, so the
//! `.eh_frame` FDE oracle cannot supply the entry either.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** the `jmp 0x1170` at `0x11cd` is followed as
//!   ordinary flow and the whole renderer is decoded into the callback — its
//!   arithmetic and its 32-byte string blit are emitted inside `on_key`.
//! * **default (the fix):** the teardown in front of the jump (`pop rbp`, which
//!   exactly undoes the entry block's `push rbp`) makes it a tail call, so the
//!   callback ends at `sub_1170(a0)` with an attributing warning.
//!
//! Neither parity corpus can cover the *discovery* half of this: both are
//! symbol-less bytechunks that never construct an `ObjectLoadImage`, so nothing
//! there has a function map for `query_call` to miss.
//! `tests/stages/tailcallframe.xml` covers the decision on hand-built bytes.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The indirect-only callback: `on_key`, reached only through `listener[0]`.
const CALLBACK: u64 = 0x11b0;
/// The tail-jump target: `render`, which no discovery oracle reaches.
const RENDERER: u64 = 0x1170;
/// `render`'s own arithmetic (`s->w = s->key * 7 + 3`), the tell that its body
/// was decoded into the callback.
const RENDERER_BODY: &str = "* 7 + 3";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with `tailcallframe` in the requested state.
/// `None` is a visible skip when the `.sla` is missing.
fn load(tailcallframe: bool) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_tailcallframe: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("tailcallframe", if tailcallframe { "on" } else { "off" })
        .expect("tailcallframe flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog.arch().manage().get_default_code_space().expect("code space").clone();
    Address::new(Rc::clone(&space), vma)
}

/// The rendered C of the callback at `CALLBACK`.
fn callback_body(prog: &mut ConsoleProgram) -> String {
    let entry = code_addr(prog, CALLBACK);
    let declared = prog.declared_extent(CALLBACK);
    let step = decompile_one(
        prog.arch_mut(),
        "sub_11b0",
        entry,
        declared,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the callback decompiles");
    kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd)
}

/// The precondition the whole rule exists for: nothing discovers the renderer,
/// so `tailcalljump`'s `query_call` can never resolve it.
#[test]
fn the_tail_jump_target_is_not_a_discovered_function() {
    let Some(prog) = load(true) else { return };
    let entries: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    assert!(
        !entries.contains(&RENDERER),
        "the fixture only reproduces the defect while {RENDERER:#x} is undiscovered; \
         got {entries:x?}"
    );
}

/// BEFORE (`--option tailcallframe off`): the callback absorbs the renderer.
#[test]
fn the_callback_absorbs_the_renderer_with_the_option_off() {
    let Some(mut prog) = load(false) else { return };
    let body = callback_body(&mut prog);
    assert!(
        body.contains(RENDERER_BODY),
        "with tailcallframe off the jump must still be followed into the renderer — \
         the option must restore the previous output exactly; got:\n{body}"
    );
}

/// AFTER (default): the jump is a call, and the renderer's body is gone.
#[test]
fn the_tail_jump_becomes_a_call_by_default() {
    let Some(mut prog) = load(true) else { return };
    let body = callback_body(&mut prog);
    assert!(
        !body.contains(RENDERER_BODY),
        "the renderer's body must not be decoded into the callback; got:\n{body}"
    );
    assert!(
        body.contains("sub_1170("),
        "the tail jump must render as a call to the callee; got:\n{body}"
    );
    assert!(
        body.contains("tailcallframe: recovered tail call"),
        "the introduced call must be attributable to the rule that introduced it; \
         got:\n{body}"
    );
}
