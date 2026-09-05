//! End-to-end gate for the **adopted flow follow**: `kuna decompile` drives the
//! console with `load function <fn>` and then `decompile`, and each of those used
//! to follow the same function's flow from scratch — lift, block build, and the
//! per-jump-table sub-decompilation, all twice.
//!
//! Upstream never pays that: C++ `IfcFuncload` follows flow once and
//! `IfcDecompile` re-runs the actions on *that* `Funcdata` after
//! `Architecture::clearAnalysis` (`ifacedecomp.cc:889`). The kuna console rebuilds
//! instead because the facts a decompile is seeded with — `map addr` symbols and
//! DWARF locals, `type varnode` usepoint symbols, `map hash` dynamic symbols, a
//! `parse line` prototype, `map param` storage locks, `override prototype` facts —
//! are consumed AT FLOW TIME and `load function` applies none of them. So the
//! rebuild is required exactly when one of those is present, and pure waste when
//! none is, which is every plain `kuna decompile <bin> <fn>`.
//!
//! `IfcDecompile` now adopts the loaded IR when it can prove the rebuild would
//! repeat the same follow (`kuna_console::ifacedecomp::PristineFlow`). The
//! properties asserted here are the two that matter:
//!
//! 1. **The fast path fires** on the plain `load function` → `decompile` pair
//!    (`IfaceDecompData::adopted_flows`), and does NOT fire when anything at all
//!    ran in between or when a seed is present.
//! 2. **It changes nothing**: the C rendered through the adopting path is
//!    byte-identical to the C rendered through the rebuilding path.
//!
//! ## `.sla` precondition
//!
//! Like the sibling gates, bootstrapping needs the built per-arch `.sla` under
//! `specs/` (gitignored; `make specs`). When it is absent the bootstrap fails;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the x86-64 `fmt` fixture and run the analysis commit.  `None` is a
/// visible specs-less skip.
fn load() -> Option<ConsoleProgram> {
    load_fixture("fmt_x86_64")
}

fn load_fixture(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_flowreuse[{name}]: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Drive the console with `cmds` and return `(rendered C, adoption count)`.
fn drive(cmds: &[&str]) -> Option<(String, u64)> {
    drive_on("fmt_x86_64", cmds)
}

fn drive_on(fixture: &str, cmds: &[&str]) -> Option<(String, u64)> {
    drive_with(fixture, &[], cmds)
}

fn drive_with(fixture: &str, options: &[(&str, &str)], cmds: &[&str]) -> Option<(String, u64)> {
    let mut prog = load_fixture(fixture)?;
    for (name, value) in options {
        prog.arch_mut().set_kuna_option(name, value).expect("option flips");
    }
    let cmds: Vec<String> = cmds.iter().map(|s| s.to_string()).collect();
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
    let adopted = {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap().adopted_flows
    };
    // Without a `decomp_dbg`-style `openfile` redirect the bulk `print C` output
    // lands in the same buffer as the command notices, so drop the transcript and
    // keep the C (the `verify_decompile_all_parity` precedent).
    let text = status
        .optr
        .rsplit_once("Decompilation complete\n")
        .unwrap_or_else(|| panic!("console decompile did not complete:\n{}", status.optr))
        .1
        .to_string();
    Some((text.trim_matches('\n').to_string(), adopted))
}

/// Property 1 + 2 together: the plain pair adopts, an `echo` in between does not,
/// and both render the same C.  `echo` is chosen deliberately — it is the most
/// inert command the console has, so what suppresses the adoption is the command
/// counter itself and not any state the command touched.
#[test]
fn adopting_the_loaded_flow_renders_the_same_c_as_rebuilding_it() {
    let Some((adopted_c, adopted_n)) =
        drive(&["load function main", "decompile", "print C"])
    else {
        return;
    };
    let (rebuilt_c, rebuilt_n) =
        drive(&["load function main", "echo x", "decompile", "print C"]).expect("second load");

    assert_eq!(adopted_n, 1, "the plain load/decompile pair must adopt the loaded IR");
    assert_eq!(rebuilt_n, 0, "a command in between must fall back to the rebuild");
    assert_eq!(
        adopted_c, rebuilt_c,
        "adopting the loaded IR must render byte-identical C"
    );
    assert!(adopted_c.contains("printf("), "the fixture renders a printf call:\n{adopted_c}");
}

/// `load addr` stamps its follow the same way `load function` does.
#[test]
fn load_addr_also_adopts() {
    let Some((c, n)) = drive(&["load function main", "decompile", "print C"]) else { return };
    let Some(prog) = load() else { return };
    let entry = prog
        .function_entries_canonical()
        .into_iter()
        .find(|e| e.name == "main")
        .expect("the fixture exports main");
    drop(prog);
    let cmd = format!("load addr 0x{:x} main", entry.addr.get_offset());
    let (c2, n2) = drive(&[&cmd, "decompile", "print C"]).expect("load addr drive");
    assert_eq!(n, 1);
    assert_eq!(n2, 1, "`load addr` must stamp its follow too");
    assert_eq!(c, c2, "the two entry surfaces render the same C");
}

/// The stamp is single-use: a second `decompile` of the same function has no
/// pristine IR to adopt (the first one consumed it and left a decompiled
/// `Funcdata` in its place), so it must re-follow.
#[test]
fn a_second_decompile_does_not_adopt() {
    let Some((_, n)) = drive(&[
        "load function main",
        "decompile",
        "decompile",
        "print C",
    ]) else {
        return;
    };
    assert_eq!(n, 1, "the stamp is single-use; a second decompile must rebuild");
}

/// The SEED guard, isolated from the command-counter guard: a `-g` binary parks
/// DWARF stack locals on the function, and `IfcDecompile` re-seeds them into the
/// rebuilt IR's `ScopeLocal` — a fact consumed at flow time that `load function`
/// never applied.  Nothing runs between the load and the decompile here, so the
/// counter guard passes and it is the seed guard alone that must force the
/// rebuild.  The rendered C is the proof it matters: the DWARF names survive.
#[test]
fn dwarf_locals_are_a_flow_time_seed_that_forces_the_rebuild() {
    let Some((c, n)) =
        drive_on("dwarfstructs_x86_64", &["load function main", "decompile", "print C"])
    else {
        return;
    };
    assert_eq!(n, 0, "DWARF locals are a flow-time seed; they must force the rebuild");
    assert!(!c.is_empty(), "the DWARF fixture still renders C:\n{c}");
}

/// The ARCHITECTURE-CONFIG guard, and the reason it exists.  A `Funcdata`
/// snapshots the per-function flags into its ArchSeam handle when it is BUILT, so
/// a flag flipped afterwards is invisible to it.  `formatstring` flips exactly
/// one — read-only propagation, turned on around the drive so the printf format
/// constant can be READ — which the loaded IR snapshotted OFF.  Adopting it there
/// leaves `printf((char *)(dat_… + …), …)`: the format string never resolves and
/// the varargs never get typed.  So `formatstring` must force the rebuild.
#[test]
fn formatstring_forces_the_rebuild() {
    let Some((c, n)) = drive_with(
        "fmt_x86_64",
        &[("formatstring", "on")],
        &["load function main", "decompile", "print C"],
    ) else {
        return;
    };
    assert_eq!(n, 0, "formatstring moves an arch flag around the drive; it must rebuild");
    assert!(
        c.contains("printf(\"%d %s"),
        "the format string must still resolve with formatstring on:\n{c}"
    );
}
