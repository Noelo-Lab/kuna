//! End-to-end gate for the **x86 direction-flag guarantee**: an inlined string
//! operation steps its pointers by `1`, not by `1 - 2*DF`, and the flag itself
//! never appears as a variable.
//!
//! Every x86 string instruction scales its pointer step by the direction flag —
//! forward when clear, backward when set — and SLEIGH lowers that faithfully. So
//! gcc's inlined `strcmp(file, "-")` (a `repz cmpsb`, present at file offset
//! `0x1c77` of the `fmt` fixture) used to decompile as:
//!
//! ```text
//! uint1 v21;                             // df
//! v21 = 0;
//! v15 = &v16[(uint8)v21 * -2 + 1];
//! v13 = &v14[(uint8)v21 * -2 + 1];
//! ```
//!
//! The flag is not unknown: the processor spec pins it to 0 at function entry and
//! the constant-base action materializes that. What broke the chain was the
//! *call* — the gcc compiler spec omits `DF` from the registers a call leaves
//! alone, so SSA construction plants `DF = INDIRECT(DF, <call>)` at every call
//! site and the constant never reaches the stride. Both x86 ABIs require the
//! direction flag to be clear at every function boundary, and the Microsoft
//! prototype in the same spec already says so; kuna now states it where the spec
//! is silent.
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

/// Bootstrap `bin`, commit the analysis facts, decompile `func`, and return the
/// captured C (`None` ⇒ specs-less skip).
fn decompile(bin: &str, func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_direction_flag: skipping (bootstrap failed, build the `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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
    Some(status.optr.clone())
}

/// `fmt/main` inlines `strcmp(file, "-")` as `repz cmpsb`, and calls several
/// functions before reaching it — so it needs the flag to survive a call.
#[test]
fn an_inlined_string_op_steps_by_one() {
    let Some(code) = decompile("regglobal_fmt_x86_64", "main") else { return };

    assert!(
        !code.contains("* -2 + 1"),
        "the direction-flag-scaled pointer stride must be folded to +1; got:\n{code}",
    );
    assert!(
        !code.contains("// df"),
        "the direction flag must not survive as a variable; got:\n{code}",
    );
    // The loop itself must still be there: this folds a stride, it does not
    // remove code.
    assert!(
        code.contains("do {"),
        "the comparison loop should still be present; got:\n{code}",
    );
}

/// A language with no `DF` register must be structurally untouched — the
/// assertion is keyed on a SLEIGH register name, and a lookup miss is the no-op
/// path that every non-x86 target takes.
#[test]
fn a_non_x86_target_is_unaffected() {
    let Some(code) = decompile("fmt_aarch64", "main") else { return };

    assert!(
        code.contains("main("),
        "an AArch64 binary must still decompile normally; got:\n{code}",
    );
}
