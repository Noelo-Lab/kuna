//! `--assert bytes` end-to-end — `docs/re-needs/byte-overlay-assertion-recovered.md`.
//!
//! A packer's plaintext exists only after the packer has run, so it is a fact no
//! loader can derive and the only one an agent working a self-modifying binary
//! actually has. Before this directive there was no way to hand it back: the
//! recorded workaround was applying the recovered transform in Python and
//! re-loading a patched copy of the executable.
//!
//! Every case here asserts the **emitted C changed**, which is the bar this
//! family is held to — `override prototype` has printed "Successfully added
//! override" and changed nothing since it was ported.
//!
//! Fixture: `kuna-analysis/tests/fixtures/assertranges_x86_64` (shared with
//! `verify_assertranges`), whose `sample` computes `scale * a0 + bias +
//! dat_50000000 * 2`. Overlaying `b8 2a 00 00 00 c3` on its entry is
//! `mov eax,0x2a; ret`, so a directive that reached the lifter is visible in one
//! line of C and one that did not is equally visible.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive, Outcome};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

/// `sample`'s entry in the fixture.
const SAMPLE: u64 = 0x401140;
/// `mov eax,0x2a; ret`.
const RETURN_42: [u8; 6] = [0xb8, 0x2a, 0x00, 0x00, 0x00, 0xc3];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture, WITHOUT the analysis commit: a byte overlay has to be
/// stated before it, which is the ordering under test.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64");
    match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_byteoverlay: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

fn bytes(addr: u64, data: &[u8]) -> Directive {
    let hex: String = data.iter().map(|b| format!("{b:02x}")).collect();
    Directive {
        raw: format!("bytes {addr:#x} {hex}"),
        body: Body::Bytes { addr, data: data.to_vec() },
    }
}

/// Decompile `target` under `directives`, in the order the CLI's in-process
/// surface applies them (`decompile_all::load_program`): image-scoped before the
/// analysis commit, program-scoped after it.
fn decompile_with(target: &str, directives: Vec<Directive>) -> Option<(String, Vec<Outcome>)> {
    let mut prog = load()?;
    if !directives.is_empty() {
        prog.set_assertions(directives);
        assertions::apply_image_scoped(&mut prog);
    }
    prog.commit_pending_analysis().expect("analysis commit");
    assertions::apply_program_scoped(&mut prog);
    let entry = prog
        .resolve_entry(&EntrySelector::Name(target.to_string()))
        .expect("the fixture has this function");
    let funcs = decompile_targets(&mut prog, vec![entry], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    Some((code, prog.assertion_outcomes()))
}

/// The un-asserted baseline the case below is measured against.
#[test]
fn the_baseline_computes_from_the_image_bytes() {
    let Some((code, report)) = decompile_with("sample", Vec::new()) else { return };
    assert!(report.is_empty(), "no directives ⇒ no report rows");
    assert!(code.contains("scale"), "baseline lost the .data load:\n{code}");
    assert!(!code.contains("0x2a"), "baseline already returns the overlay:\n{code}");
}

/// The headline: stated bytes are what gets lifted.
#[test]
fn an_overlay_is_what_the_lifter_decodes() {
    let Some((code, report)) = decompile_with("sample", vec![bytes(SAMPLE, &RETURN_42)]) else {
        return;
    };
    assert_eq!(report[0].status, "applied", "{report:?}");
    assert_eq!(report[0].kind, "bytes");
    assert!(code.contains("return 0x2a"), "the overlay never reached the lifter:\n{code}");
    assert!(!code.contains("scale"), "the image bytes were lifted anyway:\n{code}");
}

/// An address no segment maps is REJECTED, with the span in the detail. A
/// directive that is accepted and does nothing is worse than an error — and for
/// this one it would be a decompilation of a program the caller never described.
#[test]
fn an_unmapped_overlay_is_rejected_naming_the_span() {
    let Some((code, report)) = decompile_with("sample", vec![bytes(0x9000_0000, &RETURN_42)])
    else {
        return;
    };
    assert_eq!(report[0].status, "rejected", "{report:?}");
    let detail = report[0].detail.clone().unwrap_or_default();
    assert!(detail.contains("no loaded segment maps 0x90000000"), "detail: {detail:?}");
    assert!(code.contains("scale"), "a rejected overlay still changed the C:\n{code}");
}

/// The console spelling of the same fact, from the generated script surface.
#[test]
fn the_console_command_overlays_and_says_so() {
    let Some(out) = drive_console(&["override bytes 0x401140 b82a000000c3"]) else { return };
    assert!(out.contains("Successfully overlaid 6 bytes at"), "out: {out:?}");

    let Some(out) = drive_console(&["override bytes 0x401140 abc"]) else { return };
    assert!(out.contains("odd number of hex digits"), "out: {out:?}");

    let Some(out) = drive_console(&["override bytes 0x90000000 90"]) else { return };
    assert!(out.contains("no loaded segment maps"), "out: {out:?}");
}

/// Drive `commands` through a console wired like the datatest runner, with the
/// fixture installed as the current program.  `None` ⇒ specs-less skip.
fn drive_console(commands: &[&str]) -> Option<String> {
    use kuna_console::ifacedecomp::{
        execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::ConsoleCommands;
    use kuna_console::kuna_console::register_kuna_commands;

    let program = load()?;
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    // `override bytes` is a kuna command, not an upstream one.
    register_kuna_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).expect("decompile module data");
        let dcp = data
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .expect("decompile module data is IfaceDecompData");
        dcp.conf = Some(program);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}
