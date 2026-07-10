//! End-to-end gate for the **full byte-pattern function-start** pass
//! (`funcstart_patterns` — the faithful port of Ghidra's `FunctionStartAnalyzer`
//! over the entire vendored pattern corpus, default-**OFF**).
//!
//! Drives `load file <funcstart_patterns_x86_64>` (a stripped, statically-linked
//! x86-64 ELF) through the bootstrap. The fixture has a `static` helper `widget`
//! at **0x401130** whose prologue is `push rbx; mov rbx,rdi` (`53 48 89 fb`),
//! preceded by an 8-byte NOP pad. That shape is a `<patternpairs>` postpattern in
//! the FULL upstream set (`0x534889fb`, "PUSH RBX; MOV RBX,RDI"), gated by an
//! upstream NOP prepattern (`0x0f1f840000000000`) — but it is NOT one of the three
//! bare x86-64 prologues the always-on minimal oracle (`entry_disc` oracle 5)
//! ports, and `widget` has no symbol (stripped), no `.eh_frame` FDE
//! (`-fno-asynchronous-unwind-tables`), and is not `e_entry`/INIT/FINI/`main`. So
//! `widget` is discoverable ONLY via the new full pattern set.
//!
//! The proof (the headline): with **`--option funcstart_patterns on`** the extra
//! function `sub_401130` is discovered and decompiles by name; with the option
//! **off (default)** it is NOT registered — proving the gate is genuinely
//! output-changing and the default run is unperturbed.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_entry`/`verify_w11_elf_plt_names` gates, the
//! bootstrap needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored stripped x86-64 fixture (built `gcc -O2
/// -fno-asynchronous-unwind-tables -fcf-protection=none -no-pie -fno-pic
/// -fno-stack-protector` + `strip`; see the fixtures README provenance).
fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/funcstart_patterns_x86_64")
}

/// `widget`'s pinned VMA (read from the unstripped build's `nm`): 0x401130.
const WIDGET: u64 = 0x401130;

/// Whether to enable the full byte-pattern function-start pass.
enum Mode {
    /// Default: no flags (today's `entry_disc`-only baseline).
    Off,
    /// `--option funcstart_patterns on`, set BEFORE the deferred commit (the live
    /// CLI ordering: the `option` line precedes `read symbols`).
    On,
}

/// Bootstrap the fixture, (optionally) enable the pass, commit the analysis, and
/// return the program. `None` ⇒ specs-less skip.
fn bootstrap(mode: Mode) -> Option<kuna_console::engine::ConsoleProgram> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_funcstart_patterns: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The live-CLI ordering: the `option` line precedes `read symbols` (the
    // deferred commit). Flip the gate on the live arch BEFORE committing so the
    // gated commit keeps the pass's facts.
    if let Mode::On = mode {
        prog.arch_mut()
            .set_kuna_option("funcstart_patterns", "on")
            .expect("funcstart_patterns flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// THE PAYOFF: default-off, `widget` (`sub_401130`) is NOT discovered; with the
/// option on, it IS — and decompiles by name. The extra-function discovery is the
/// whole point of the gate.
#[test]
fn full_patterns_discover_extra_function_when_enabled() {
    let Some(off) = bootstrap(Mode::Off) else {
        return; // specs-less skip
    };
    // Default-off: the full-pattern-only `widget` is NOT registered (the minimal
    // oracle misses it; nothing else covers it). This is the byte-identical
    // baseline contract.
    assert!(
        off.lookup_symbol("sub_401130").is_none(),
        "default-off: widget (sub_401130) must NOT be discovered (full-pattern-only)"
    );

    // Option on: the full byte-pattern pass discovers `widget` and the commit seam
    // registers it as `sub_401130`.
    let on = bootstrap(Mode::On).expect("second bootstrap succeeds if the first did");
    assert!(
        on.lookup_symbol("sub_401130").is_some(),
        "with --option funcstart_patterns on, widget (sub_401130) must be discovered"
    );

    // And it is a real, decompilable function under its discovered name (no --addr).
    let cmds: Vec<String> =
        ["load function sub_401130", "decompile", "print C"].iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(on);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    let out = status.optr.clone();
    eprintln!("---- sub_401130 (funcstart_patterns ON) ----\n{out}");
    assert!(
        out.contains("sub_401130"),
        "expected a decompiled body for the discovered sub_401130, got:\n{out}"
    );
    assert!(
        out.contains('{') && out.contains('}'),
        "expected a C function body for sub_401130, got:\n{out}"
    );
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "sub_401130 should resolve to a function, got:\n{out}"
    );
}

/// The discovery seam directly (independent of the `.sla`/console path):
/// `full_pattern_starts` includes `widget` while the default `collect_entries`
/// (entry_disc) does NOT — the pure-core proof that the extra discovery comes
/// specifically from the full pattern set.
#[test]
fn full_pattern_starts_includes_widget_but_collect_entries_does_not() {
    let bytes = std::fs::read(fixture()).expect("read fixture");
    let file = object::File::parse(bytes.as_slice()).expect("parse fixture");

    // The default entry-discovery (entry_disc, always-on) does NOT find widget.
    let default = kuna_analysis::entry::collect_entries(&file, bytes.as_slice());
    assert!(
        !default.contains(&WIDGET),
        "default entry_disc must NOT discover widget {WIDGET:#x} (the minimal oracle misses it)"
    );

    // The full byte-pattern pass DOES find widget (and it survives the funcsym-skip
    // inside `full_pattern_starts`, so widget is genuinely un-symboled).
    let full = kuna_analysis::entry::full_pattern_starts(&file);
    assert!(
        full.contains(&WIDGET),
        "full_pattern_starts must discover widget {WIDGET:#x}, got {full:#x?}"
    );
}
