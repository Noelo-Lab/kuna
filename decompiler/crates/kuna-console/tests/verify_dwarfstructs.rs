//! End-to-end gate for DWARF aggregate-LAYOUT import (`--option dwarfstructs`).
//!
//! The bug: the DIE→`Datatype` mapper interned every `DW_TAG_structure_type` /
//! `union_type` / `class_type` as a NAMED, EMPTY, ZERO-SIZE shell, and never read
//! `DW_AT_byte_size` or walked a single `DW_TAG_member`. That is enough for
//! `struct foo *p` to render and nothing else, and a zero width is not a
//! conservative answer — the ABI classifier acts on it:
//!
//! * `int take_struct(P8 p,int k)` came out `int take_struct(unsigned long,int)`
//!   and shifted the register apart by hand;
//! * `P8 ret_struct(unsigned x)` came out
//!   `P8 *ret_struct(P8 *rethidden,unsigned int x)` — a sizeless return type is
//!   classified as a hidden-return-buffer call, so a PHANTOM first parameter
//!   appears, the real one shifts, and the body does arithmetic on the phantom;
//! * every field access came out as a cast-and-offset.
//!
//! The fixture `dwarfstructs_x86_64` (both sources vendored alongside) is a
//! `-g -O1` C binary carrying one function per shape. Each test is TWO-PASS:
//! `dwarfstructs off` reproduces the bug, the default reproduces the fix.
//!
//! ## The collision case
//!
//! `dwarfstructs_x86_64_collide.c` is a second translation unit defining a
//! DIFFERENT 16-byte `struct Same` next to the 4-byte one in the main unit. The
//! type factory interns by `(name, hash(name))` and refuses a second, different
//! definition of a name it holds, so populating fields makes that collision live
//! — and it is not exotic: `rustc -g` names every enum payload struct bare
//! (`Some`, `Ok`, `Err`), and a five-function Rust witness carries four distinct
//! `Some` DIEs of sizes 16, 24, 16 and 12. Both `Same`s must survive.
//!
//! ## Why the gate is set through the environment, not `set_kuna_option`
//!
//! The layout is installed on the interned type by `DwarfPass` during
//! `load file`, upstream of every `option` command, so the live gate is the
//! process env var (`kuna_dwarfstructs::DWARFSTRUCTS_ENV`) that the CLI exports
//! before the load. The helper below therefore sets it BEFORE
//! `bootstrap_from_object`, serialized by a mutex because the environment is
//! process-global and `cargo test` runs the tests in this binary concurrently.
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

/// Serializes the env-var flip + bootstrap across the concurrently-run tests.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with `dwarfstructs` on/off, decompile each of `funcs`
/// and return the concatenated C. `None` => specs-less skip.
fn decompile(funcs: &[&str], structs: bool) -> Option<String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfstructs_x86_64")
        .to_str()?
        .to_string();

    std::env::set_var(DWARFSTRUCTS_ENV, if structs { "on" } else { "off" });
    let prog = bootstrap_from_object(&path, "", &spec_roots);
    std::env::remove_var(DWARFSTRUCTS_ENV);
    let mut prog = match prog {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_dwarfstructs: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let mut cmds: Vec<String> = Vec::new();
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
    Some(status.optr.clone())
}

/// A struct passed BY VALUE keeps its type instead of degrading to the raw
/// register it arrives in.
#[test]
fn by_value_struct_parameter_keeps_its_type() {
    let (Some(off), Some(on)) =
        (decompile(&["take_struct"], false), decompile(&["take_struct"], true))
    else {
        return;
    };
    assert!(
        off.contains("take_struct(unsigned long p,int4 k)"),
        "gate off should reproduce the bug, got:\n{off}"
    );
    assert!(
        on.contains("take_struct(P8 p,int4 k)") && on.contains("p.a") && on.contains("p.b"),
        "`P8 p` should keep its type and field names, got:\n{on}"
    );
}

/// The headline correctness case: an 8-byte struct return is a REGISTER return,
/// but a zero-size return type is classified as a hidden-return-buffer call — so
/// a phantom `rethidden` parameter appeared and the body did arithmetic on it.
#[test]
fn small_struct_return_loses_its_phantom_sret() {
    let (Some(off), Some(on)) =
        (decompile(&["ret_struct"], false), decompile(&["ret_struct"], true))
    else {
        return;
    };
    assert!(
        off.contains("rethidden") && off.contains("(int4)rethidden"),
        "gate off should reproduce the phantom sret used in arithmetic, got:\n{off}"
    );
    assert!(
        on.contains("P8 ret_struct(uint4 x)") && !on.contains("rethidden"),
        "an 8-byte struct return should be a register return, got:\n{on}"
    );
}

/// A 24-byte return really IS an sret — the fix must not delete a hidden return
/// pointer that the ABI genuinely has, only type what it points at.
#[test]
fn large_struct_return_keeps_its_real_sret() {
    let (Some(off), Some(on)) = (decompile(&["ret_big"], false), decompile(&["ret_big"], true))
    else {
        return;
    };
    assert!(off.contains("rethidden"), "the 24-byte sret is real off, got:\n{off}");
    assert!(
        on.contains("rethidden") && on.contains("rethidden->x") && on.contains("rethidden->z"),
        "the sret must survive and be typed, got:\n{on}"
    );
}

/// A union member is named rather than reconstructed from a shift of the whole
/// word.
#[test]
fn union_members_resolve() {
    let (Some(off), Some(on)) =
        (decompile(&["take_union"], false), decompile(&["take_union"], true))
    else {
        return;
    };
    assert!(
        off.contains("take_union(unsigned int u)"),
        "gate off should reproduce the bug, got:\n{off}"
    );
    assert!(
        on.contains("take_union(U4 u)") && on.contains("u.i"),
        "the union parameter should keep its type and member, got:\n{on}"
    );
}

/// A union keeps EVERY member, not just its first: all of them sit at offset 0,
/// so any offset-keyed deduplication silently collapses `u->f` and `u->c[2]` into
/// reads of `u->i`. (That is a bug this file caught before it shipped.)
#[test]
fn union_keeps_every_member_not_just_the_first() {
    let funcs = ["union_second", "union_third"];
    let (Some(off), Some(on)) = (decompile(&funcs, false), decompile(&funcs, true)) else {
        return;
    };
    assert!(
        off.contains("*(float4 *)u") && off.contains("*(char *)((int8)u + 2)"),
        "gate off should reproduce the raw reads, got:\n{off}"
    );
    assert!(
        on.contains("u->f") && on.contains("u->c[2]"),
        "the second and third union members must both resolve, got:\n{on}"
    );
}

/// A struct whose member is itself a struct resolves both levels.
#[test]
fn nested_struct_members_resolve() {
    let (Some(off), Some(on)) =
        (decompile(&["take_nest"], false), decompile(&["take_nest"], true))
    else {
        return;
    };
    assert!(
        off.contains("*(int4 *)((int8)n + 4)"),
        "gate off should reproduce the cast-and-offset, got:\n{off}"
    );
    assert!(
        on.contains("n->tag") && on.contains(").a") && on.contains("n->tail"),
        "the nested struct's members should render by name, got:\n{on}"
    );
}

/// A bitfield member renders by name instead of as a mask and shift of the word
/// that holds it.
#[test]
fn bitfield_members_resolve() {
    let (Some(off), Some(on)) =
        (decompile(&["take_bits"], false), decompile(&["take_bits"], true))
    else {
        return;
    };
    assert!(
        off.contains("& 7") && off.contains(">> 3"),
        "gate off should reproduce the mask-and-shift, got:\n{off}"
    );
    assert!(
        on.contains("b->lo") && on.contains("b->mid") && on.contains("b->hi"),
        "the three bitfields should render by name, got:\n{on}"
    );
}

/// A self-referential struct must TERMINATE and still name its own pointer
/// member — the recursion the shell-before-members ordering exists to survive.
#[test]
fn self_referential_struct_terminates() {
    let Some(on) = decompile(&["walk_list"], true) else { return };
    assert!(
        on.contains("n->val") && on.contains("n->next"),
        "a linked-list node should resolve both its members, got:\n{on}"
    );
}

/// Two same-named, DIFFERENT-sized aggregates in one binary both survive: the
/// second is interned under a size-suffixed name rather than inheriting the
/// first's layout or collapsing the whole parameter to `void *`.
#[test]
fn same_name_different_size_aggregates_both_survive() {
    let funcs = ["read_same_small", "read_same_big"];
    let (Some(off), Some(on)) = (decompile(&funcs, false), decompile(&funcs, true)) else {
        return;
    };
    assert!(
        off.contains("*(int4 *)((int8)s + 8)"),
        "gate off should reproduce the cast-and-offset, got:\n{off}"
    );
    assert!(
        on.contains("read_same_small(Same *s)") && on.contains("s->only"),
        "the 4-byte Same should keep its single member, got:\n{on}"
    );
    assert!(
        on.contains("Same_16 *s") && on.contains("s->first") && on.contains("s->third"),
        "the 16-byte Same should get its own name and members (not void *), got:\n{on}"
    );
}
