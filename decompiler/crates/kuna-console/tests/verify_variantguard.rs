//! End-to-end gate for DWARF discriminant-guarded variant selection
//! (`--option variantguard`).
//!
//! `dwarfvariants` (DIV-87) imports a Rust tagged enum's `DW_TAG_variant_part`,
//! installs it as `struct { tag; union payload; }`, and then refuses to NAME a
//! variant wherever two of them overlay one byte range — because a union member
//! selects itself by OFFSET and the discriminant is never consulted, so `Ok.__0`
//! and `Err.__0` (both at offset 8, both 8 bytes) tie in `ScoreUnionFields` and
//! the winner is trial order. Every `Result<T,E>` is that case, and its payload
//! is spelled `field_0x8` on every access.
//!
//! `variantguard` is the dominating-guard analysis that side table was recorded
//! for: it seeds from the DWARF discriminant (a branch testing the tag bytes, or
//! a constant store over them), propagates the answer over the block graph, and
//! reaches a value the compiler HOISTED above the branch through a backward walk
//! over def-use. The printer then spells the `DW_TAG_variant` name where — and
//! only where — that proof exists.
//!
//! ## The tests that matter are the REFUSALS
//!
//! `the_arms_are_not_swapped` is the one a reviewer checks first, because naming
//! `Ok` on the `Err` arm is exactly the failure DIV-87's suppression rule exists
//! to prevent. After it come the three refusals:
//! `a_branchless_discriminant_names_nothing` (the tag is computed, so nothing is
//! proved), `a_payload_read_that_can_reach_both_variants_stays_unnamed` (the
//! read is not under any guard), and
//! `an_already_forced_layout_is_never_reconsidered` (`Option` / disjoint
//! payloads, which `dwarfvariants` already names — this pass never even looks at
//! those layouts).
//!
//! ## Why this file and not only a datatest
//!
//! `tests/stages/kuna-variantguard.xml` is the stage-tier companion and covers
//! the same ground through `load file`; this is the cross-crate gate, and it can
//! assert on the whole rendered function rather than on a regex.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV;
use kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV;

/// Serializes the env-var flip + bootstrap across the concurrently-run tests.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap `fixture` with the DWARF import pinned ON, decompile each of
/// `funcs` with `variantguard` set to `guard`, and return the concatenated C.
///
/// `variantguard` is a plain `Architecture` flag, not a load-time env gate, so
/// the `option` command goes AFTER the load — unlike its `dwarfvariants` parent.
/// `None` => specs-less skip.
fn decompile_in(fixture: &str, funcs: &[&str], guard: bool) -> Option<String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fixture)
        .to_str()?
        .to_string();

    std::env::set_var(DWARFSTRUCTS_ENV, "on");
    std::env::set_var(DWARFVARIANTS_ENV, "on");
    let prog = bootstrap_from_object(&path, "", &spec_roots);
    std::env::remove_var(DWARFVARIANTS_ENV);
    std::env::remove_var(DWARFSTRUCTS_ENV);
    let mut prog = match prog {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_variantguard: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let mut cmds: Vec<String> =
        vec![format!("option variantguard {}", if guard { "on" } else { "off" })];
    for f in funcs {
        cmds.push(format!("load function {f}"));
        cmds.push("decompile".into());
        cmds.push("print C".into());
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
    // Drop the `option` command's own confirmation line: it names the arm, so an
    // off-vs-on comparison of the rendered C would differ on it alone.
    Some(
        status
            .optr
            .lines()
            .filter(|l| !l.starts_with("DWARF discriminant-guarded variant selection"))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

/// Run one function of the overlay fixture both ways; `None` => specs-less skip.
fn ab_overlay(func: &str) -> Option<(String, String)> {
    let off = decompile_in("dwarfvariants_overlay_x86_64", &[func], false)?;
    let on = decompile_in("dwarfvariants_overlay_x86_64", &[func], true)?;
    Some((off, on))
}

/// As [`ab_overlay`], over the SHAPES fixture.
fn ab_shapes(func: &str) -> Option<(String, String)> {
    let off = decompile_in("dwarfvariants_x86_64", &[func], false)?;
    let on = decompile_in("dwarfvariants_x86_64", &[func], true)?;
    Some((off, on))
}

/// THE headline. `use16` is `match r16(x) { Ok(v) => v, Err(e) => e + 100 }`.
/// Off, both arms render the same offset spelling. On, each names its own
/// variant.
#[test]
fn a_match_on_a_result_names_both_arms() {
    let Some((off, on)) = ab_overlay("use16") else { return };
    assert_eq!(
        off.matches("v2.payload.field_0x8").count(),
        2,
        "gate off should spell both arms by offset, got:\n{off}"
    );
    assert!(!off.contains(".Ok"), "gate off names no variant:\n{off}");
    assert!(!off.contains(".Err"), "gate off names no variant:\n{off}");
    assert!(on.contains("v2.payload.Ok"), "gate on should name Ok, got:\n{on}");
    assert!(on.contains("v2.payload.Err"), "gate on should name Err, got:\n{on}");
    assert!(
        !on.contains("payload.field_0x8"),
        "no arm keeps the offset spelling once both are proved:\n{on}"
    );
}

/// **The check a reviewer makes first.** Naming `Ok` where the source says `Err`
/// is the exact failure DIV-87's suppression rule exists to prevent, and it is
/// invisible unless the arms are read against the source:
///
/// ```ignore
/// match r16(x) { Ok(v) => v, Err(e) => e + 100 }
/// ```
///
/// `Ok` has `DW_AT_discr_value` 0, so `Ok` must be the arm guarded by
/// `!(v2.tag & 1)` and must carry NO `+ 100`; `Err` is the value rustc hoisted
/// above the branch and it is the one that gets `+ 100`.
#[test]
fn the_arms_are_not_swapped() {
    let Some((_off, on)) = ab_overlay("use16") else { return };
    assert!(
        on.contains("(int8)v2.payload.Err + 100"),
        "the `+ 100` arm is Err in the source, got:\n{on}"
    );
    assert!(
        !on.contains("v2.payload.Ok + 100") && !on.contains("(int8)v2.payload.Ok"),
        "Ok must never carry the Err arm's arithmetic:\n{on}"
    );
    // The `Ok` read is the one under the discriminant-0 guard.
    let guarded = on
        .lines()
        .skip_while(|l| !l.contains("if (!(v2.tag & 1))"))
        .nth(1)
        .unwrap_or("")
        .to_string();
    assert!(
        guarded.contains("v2.payload.Ok"),
        "the tag == 0 arm must be Ok, got `{guarded}` in:\n{on}"
    );
}

/// A REFUSAL. rustc at `-C opt-level=1` computes a `Result`'s discriminant
/// branchlessly (`dst->tag = (x < 0xb)`), so no constant ever reaches the tag
/// and no branch tests it: nothing is proved and the payload keeps the offset
/// spelling in both arms.
#[test]
fn a_branchless_discriminant_names_nothing() {
    let Some((off, on)) = ab_overlay("put_res") else { return };
    assert!(
        off.contains("(dst->payload).field_0x8.__0 = v1;"),
        "gate off writes the payload by offset, got:\n{off}"
    );
    assert_eq!(off, on, "a computed discriminant is not a seed:\n{on}");
}

/// A REFUSAL, and the one that makes "no false positives" non-vacuous: an
/// `Option` has exactly one payload-carrying variant, so `dwarfvariants` already
/// forces `Some` and this pass never considers the layout at all.
#[test]
fn an_already_forced_layout_is_never_reconsidered() {
    let Some((off, on)) = ab_overlay("put_opt") else { return };
    assert!(off.contains("(dst->payload).Some.__0 ="), "Option keeps Some:\n{off}");
    assert_eq!(off, on, "an already-named layout is out of scope:\n{on}");
    let Some((off3, on3)) = ab_shapes("ret_three") else { return };
    assert!(off3.contains("(rethidden->payload).A.__0 = x;"), "disjoint payloads keep A:\n{off3}");
    assert_eq!(off3, on3, "disjoint payload ranges are out of scope:\n{on3}");
}

/// The PRODUCER side. `enum Multi { P { a, b }, Q(u64) }` overlays its two
/// payloads at offset 8, so `dwarfvariants` spells both facets by offset; each
/// arm here stores a LITERAL discriminant, which is a seed.
///
/// Note what does NOT change: the FIELD inside the facet keeps `field_0x8`.
/// That is `dwarfvariants`'s separate per-field suppression rule, which this
/// pass does not touch.
#[test]
fn a_literal_tag_store_names_the_variant_it_builds() {
    let Some((off, on)) = ab_shapes("ret_multi") else { return };
    assert!(
        off.contains("(rethidden->payload).field_0x4.a = x;"),
        "gate off spells the facet by offset, got:\n{off}"
    );
    assert!(on.contains("(rethidden->payload).P.a = x;"), "tag 0 builds P, got:\n{on}");
    assert!(
        on.contains("(rethidden->payload).Q.field_0x8 = (uint8)x;"),
        "tag 1 builds Q, got:\n{on}"
    );
    assert!(!on.contains("payload).field_0x4"), "no facet keeps the offset spelling:\n{on}");
}

/// A REFUSAL that is measured rather than asserted: `r16` builds the `Result`
/// itself, and its discriminant is `(x < 0xb)` — computed, never a literal — so
/// the constructor's own payload writes stay on the offset spelling even though
/// the CONSUMER of the same type in the same binary is fully named.
#[test]
fn a_payload_read_that_can_reach_both_variants_stays_unnamed() {
    let Some((off, on)) = ab_overlay("r16") else { return };
    assert!(
        off.contains("v1.payload.field_0x8.__0 = v2.field_0x8.__0;"),
        "gate off spells the constructor's store by offset, got:\n{off}"
    );
    assert_eq!(off, on, "an unguarded write proves nothing:\n{on}");
}
