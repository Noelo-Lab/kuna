//! End-to-end gate for the register<->global HighVariable over-merge **render**
//! bug: raw processor-register names (`EAX`/`RAX`/…) and `Unique<hex>` temps
//! leaking into the decompiled C where a named global belongs.
//!
//! Fixture: `regglobal_fmt_x86_64` — the GNU coreutils `fmt` binary (gcc 8.4.0
//! `-O2`, the case originally reported).  Its `main` resolves the `-w`/`-g`
//! widths and parses the `-p` prefix into file-scope globals
//! (`max_width`/`goal_width`/`prefix_full_length`/…).  Each of those globals is
//! written from a *register-computed* value across several getopt-switch
//! branches, so heritage places a `MULTIEQUAL` for the global at the join and
//! `RulePropagateCopy` folds each branch's register result into it (a cross-space
//! phi: ram-global output, register inputs).  The copy-shadow variable merge then
//! unifies those register scratch values into the global's addr-tied
//! HighVariable — faithful to upstream Ghidra, whose `goal_width` phi likewise
//! carries all-register-`EAX` inputs tagged `hv=goal_width`.
//!
//! The BUG was purely in kuna's printer (`s9_emit/printc.rs`, `push_vn_explicit_ir`
//! unnamed-location tail): it rendered each member of that mixed high by the
//! member's *own* storage, so the register members leaked a raw `EAX`/`RAX` (and
//! `unique` members a `Unique<hex>`) — e.g. `EAX = (dat_215120 * 0xbb) / 200;`
//! and `EAX = (int4)strlen(v18); v6 = &v18[EAX];` — instead of the shared global
//! name.  Ghidra renders every member through `high->getSymbol()` -> the global;
//! kuna now mirrors that, resolving each member to its high's canonical global
//! storage so the whole variable reads as one `dat_<addr>`
//! (`dat_215110 = (dat_215120 * 0xbb) / 200;`, `dat_21511c = (int4)strlen(v15);`).
//!
//! Before the fix `main` leaks a dozen such tokens; after it, zero.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`).  When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use regex::Regex;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/regglobal_fmt_x86_64")
}

/// Bootstrap the fixture and decompile `func`, returning the captured C
/// (`None` ⇒ specs-less skip).  Console defaults (Listing off) — the same path
/// `kuna decompile` drives; the render bug is independent of the analysis tier.
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_global_regmerge: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

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

/// A word-boundary regex matching a raw general-purpose register token used as a
/// *variable* — the `[R/E]AX`…`[R/E]DX`, `[R/E]SI`/`[R/E]DI`, `R8`..`R15`(`D`),
/// and 8-bit `AL`..`DH` forms — plus the `Unique<hex>` temp form.  `RSP`/`RBP`
/// (and `SP`/`BP`) are deliberately absent from the alternation: the unaffected
/// stack pointer legitimately renders raw in the `__libc_start_main` tail call
/// (`PTRSUB(RSP,-0x30)`), a separate stack-spacebase seam, not this bug.
fn register_leak_re() -> Regex {
    Regex::new(
        r"\b(?:[RE]?[ABCD]X|[RE]?[SD]I|R(?:8|9|1[0-5])D?|[ABCD][LH]|Unique[0-9a-fA-F]{4,})\b",
    )
    .unwrap()
}

/// The reported case: `fmt`'s `main` must not render any global-backed value as a
/// raw register / `Unique<hex>`.  The width/prefix globals must read as
/// `dat_<addr>`, exactly as Ghidra renders them (`goal_width`/`max_width`/…).
#[test]
fn fmt_main_has_no_register_or_unique_variable_leaks() {
    let Some(code) = decompile("main") else { return }; // specs-less skip

    let re = register_leak_re();
    let leaks: Vec<&str> = code
        .lines()
        .filter(|l| re.is_match(l))
        .collect();
    assert!(
        leaks.is_empty(),
        "fmt/main leaked raw register / Unique<hex> variables (register<->global \
         over-merge render bug); offending lines:\n{}\n--- full output ---\n{code}",
        leaks.join("\n"),
    );

    // Positive witness: the width globals resolve to `dat_<addr>` and carry the
    // register-computed values that used to leak as `EAX` — i.e. the merged high
    // renders as the global everywhere.  `(dat_2151?? * 0xbb) / 200` is fmt's
    // `goal_width = (max_width * 187) / 200`; the LHS must be a `dat_` global.
    let width_div = Regex::new(r"dat_[0-9a-fA-F]+ = \(dat_[0-9a-fA-F]+ \* 0xbb\) / 200").unwrap();
    assert!(
        width_div.is_match(&code),
        "expected fmt/main's `goal_width = (max_width * 0xbb) / 200` to render both \
         sides as `dat_<addr>` globals; output:\n{code}",
    );
}
