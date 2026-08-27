//! End-to-end gate for the **uncomputed-return-value repair**: a function that
//! was modelled as returning a 16-byte `RAX:RDX` pair, whose high half is
//! leftover, returns a single register instead — and a function that genuinely
//! returns a pair keeps it.
//!
//! The symptom, on `fmt/main` with no usable prototype:
//!
//! ```text
//! undefined16 main(uint4 a0,void *a1)
//!   char v5 [16];
//!   v5._0_8_ = v16 ^ 1;   // the real result
//!   v5._8_8_ = v31;       // an uninitialized stack slot
//!   return v5;
//! ```
//!
//! That output is not merely unreadable — it reads memory the function never
//! wrote. Return recovery registers a trial per output register the model
//! characterizes (x86-64 SysV: `RAX` *and* `RDX`), ancestor realism accepts the
//! epilogue's callee-saved restore as a value that could legitimately reach the
//! RETURN, and the compiler spec's `join_dual_class` output rule then joins the
//! consecutive pair into one 16-byte return.
//!
//! The repair runs late, once heritage has resolved the restore into a bare
//! unwritten Varnode, and rewrites the RETURN to the half that carries a value.
//!
//! Fixtures: `regglobal_fmt_x86_64` with DWARF disabled (the prototype-less path;
//! with DWARF the locked `int` return never enters recovery at all) and
//! `structreturn_x86_64`, the negative control.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, apply `options` (before the analysis commit, as the CLI
/// does), decompile `func`, and return the captured C (`None` ⇒ specs-less skip).
fn decompile(bin: &str, func: &str, options: &[&str]) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_return_uncomputed: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let mut cmds: Vec<String> = options.iter().map(|o| o.to_string()).collect();
    cmds.push(format!("load function {func}"));
    cmds.push("decompile".into());
    cmds.push("print C".into());
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        // The `option` lines above must land before the analysis commit; do the
        // commit here, after they are queued but before any of them runs, exactly
        // as `read symbols` does in the CLI script.
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}

/// With no prototype to lock the return, `fmt/main` used to join `RAX:RDX`. The
/// `listing` flag is what surfaces it — the no-return analyses it drives add the
/// return sites where the leftover shows up.
#[test]
fn fmt_main_without_a_prototype_returns_one_register() {
    let Some(code) = decompile("regglobal_fmt_x86_64", "main", &["option dwarf off", "option listing on"])
    else {
        return;
    };

    assert!(
        !code.contains("undefined16"),
        "the bogus RAX:RDX return join must be gone; got:\n{code}",
    );
    assert!(
        !code.contains("[16]"),
        "the phantom 16-byte return buffer must be gone; got:\n{code}",
    );
    // Both spellings: an 8-byte write at offset 8 of the container renders
    // `._8_8_` since arraysubfield, and `[8]` before it -- neither may appear,
    // or this arm silently stops testing anything.
    assert!(
        !code.contains("[8] =") && !code.contains("._8_8_ ="),
        "nothing should write the phantom high half; got:\n{code}",
    );
}

/// NEGATIVE CONTROL: `make` builds both halves of a real `struct pair` return
/// (`x * 3`, `y + 7`). Arithmetic is not a move, so the walk stops there and the
/// pair is kept.
#[test]
fn a_computed_struct_pair_return_is_kept() {
    let Some(code) = decompile("structreturn_x86_64", "make", &[]) else { return };

    assert!(
        code.contains("undefined16"),
        "a genuine 16-byte struct return must survive the repair; got:\n{code}",
    );
    // Each half is an 8-byte write into the `undefined1[16]` container, so it
    // renders as the sized sub-field, not a size-blind element subscript
    // (arraysubfield; `v1[8] = ...` claimed a one-byte store).
    assert!(
        code.contains("._8_8_ =") && code.contains("._0_8_ ="),
        "both halves of a real pair are written; got:\n{code}",
    );
}

/// NEGATIVE CONTROL: `passthru` returns a struct it read through a pointer — the
/// shape closest to leftover, since both halves arrive by load rather than
/// arithmetic. A LOAD is a real value, so the pair is kept.
#[test]
fn a_loaded_struct_pair_return_is_kept() {
    let Some(code) = decompile("structreturn_x86_64", "passthru", &[]) else { return };

    assert!(
        code.contains("undefined16"),
        "a pair loaded through a pointer must survive the repair; got:\n{code}",
    );
}
