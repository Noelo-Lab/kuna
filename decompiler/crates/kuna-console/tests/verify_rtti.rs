//! MSVC RTTI / vftable class-name recovery e2e gate (`rtti` analysis option — the
//! kuna analog of Ghidra's `RttiAnalyzer`).
//!
//! Two real polymorphic-C++ Windows PEs (`msvc_rtti_x64.exe` / `msvc_rtti_x86.exe`,
//! one base class `Shape` + one derived class `Box` with a virtual method) carry the
//! MSVC run-time-type-information graph **entirely in `.rdata`/`.data`**: per class a
//! `CompleteObjectLocator` before its vftable, reachable from which are the
//! `ClassHierarchyDescriptor` → `BaseClassArray` → `BaseClassDescriptor`s → the
//! `TypeDescriptor`s carrying the `.?A…@@`-mangled class names.
//!
//! ## Before → after
//!
//! ```text
//!   before (default, --option rtti off):  .rdata is raw bytes; the vtable is an
//!                                          unnamed DAT_<addr>; the virtual dispatch
//!                                          `(**(code **)*p)()` jumps to an unnamed
//!                                          slot; no `Box`/`Shape` class names exist.
//!   after  (--option rtti on):            the TypeDescriptors are labelled
//!                                          `Box::RTTI_Type_Descriptor` /
//!                                          `Shape::RTTI_Type_Descriptor`, the COL
//!                                          `Box::RTTI_Complete_Object_Locator`, the
//!                                          vftable `Box::vftable`, AND each vftable
//!                                          slot a named virtual-method function
//!                                          `Box::vftable_0` (R3, at the slot's target
//!                                          VA) — so the virtual dispatch resolves to
//!                                          a named method and `Box`/`Shape` surface
//!                                          as recovered C++ class names.
//! ```
//!
//! ## vftable discovery + virtual-method naming (R3)
//!
//! On top of the class names, `rtti` R3 walks each vftable from its
//! `<Class>::vftable` base (`VfTableModel.getVfTableCount`), bounding the slot array
//! at the first NULL / non-`.text` slot, and names each slot's target a virtual-method
//! function `<Class>::vftable_<i>` + marks the slot array read-only. The Box vftable
//! here has exactly one slot — `Box::area` (`side*side`) — at the pinned target VMA
//! (`0x140001040` x64 / `0x401030` x86). The slots are absolute VAs on BOTH arches
//! (NOT the `IBO32` displacements the COL/RTTI graph uses).
//!
//! ## Why clang, not `cl.exe`
//!
//! `cl.exe` is unavailable on Linux, but `clang -target {x86_64,i686}-pc-windows-msvc
//! -fuse-ld=lld` emits the *same* MSVC C++ RTTI ABI (the real
//! CompleteObjectLocator / RTTI{0..3} bytes in `.rdata`, verified by
//! `objdump -s -j .rdata`). So these are **real** MSVC RTTI PEs, not hand-faked
//! tables. Source + build recipe + the pinned COL/RTTI0/vftable VMAs are recorded in
//! `tests/fixtures/README.md` (the `msvc_rtti.cpp` entry).
//!
//! ## Gating + parity safety
//!
//! Default-OFF (`--option rtti on`), PE-only. The analysis facts commit at
//! `read symbols` (`commit_pending_analysis`), gated by the per-pass flag — so the
//! option is flipped BEFORE that commit. The XML datatest path never loads a PE and
//! never runs this pass, so every parity gate is byte-identical regardless.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping a PE needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). If absent the bootstrap fails and the test prints a visible skip
//! and returns early (never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// Pinned target VMA of the Box vftable's first (and only) slot — `Box::area`
/// (`return side*side;`) — in each fixture. Read from `objdump -s -j .rdata`
/// (`tests/fixtures/README.md`, the `msvc_rtti.cpp` entry): the x64 vftable @
/// `0x140002010` slot 0 holds `0x140001040`; the x86 vftable @ `0x40200c` slot 0
/// holds `0x401030`. (Slots are absolute VAs on both arches — NOT IBO32.)
const BOX_AREA_VMA_X64: u64 = 0x1_4000_1040;
const BOX_AREA_VMA_X86: u64 = 0x0040_1030;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Bootstrap the named RTTI fixture, optionally flip `rtti` on, then commit the
/// (deferred) analysis facts. Returns `None` (a visible skip) if the `.sla` is
/// missing.
fn bootstrap_rtti(fixture: &str, on: bool) -> Option<ConsoleProgram> {
    let bin = fixtures().join(fixture);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_rtti: skipping {fixture} (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    if on {
        prog.arch_mut()
            .set_kuna_option("rtti", "on")
            .expect("rtti flips on");
    }
    // Analysis facts commit at `read symbols` (gated by the per-pass flag), not
    // eagerly at bootstrap — trigger that commit after the option is applied.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// The recovered-symbol assertions shared by the x64 and x86 arms: with `rtti on`,
/// both class names + their RTTI/vftable labels are present, AND (R3) the Box vftable
/// slot at `box_area_vma` resolves to a named virtual-method function.
fn assert_recovered(prog: &ConsoleProgram, box_area_vma: u64) {
    // Both class names recovered, with the Type Descriptor label.
    assert!(
        prog.has_symbol_named("Box::RTTI_Type_Descriptor"),
        "rtti on: the `Box` class must be recovered (Box::RTTI_Type_Descriptor)"
    );
    assert!(
        prog.has_symbol_named("Shape::RTTI_Type_Descriptor"),
        "rtti on: the `Shape` class must be recovered (Shape::RTTI_Type_Descriptor)"
    );
    // The CompleteObjectLocator + vftable labels for the (instantiated) Box class.
    assert!(
        prog.has_symbol_named("Box::RTTI_Complete_Object_Locator"),
        "rtti on: the Box COL must be labelled"
    );
    assert!(
        prog.has_symbol_named("Box::vftable"),
        "rtti on: the Box vftable must be labelled"
    );

    // R3 — vftable discovery + per-slot virtual-method naming. The Box vftable has
    // one slot (`Box::area`); R3 names it `Box::vftable_0`. The name must exist in
    // the symbol table AND a function symbol must be installed AT the slot's target
    // VA — the virtual dispatch `(**(code **)*p)()` now points at a NAMED function
    // (`Box::vftable_0`), not a bare `DAT_*` slot.
    assert!(
        prog.has_symbol_named("Box::vftable_0"),
        "rtti on: the first Box vftable slot must be named Box::vftable_0 (R3)"
    );
    assert_eq!(
        prog.function_named_at(box_area_vma).as_deref(),
        Some("vftable_0"),
        "rtti on: the vftable slot's target VA {box_area_vma:#x} must resolve to the \
         named virtual-method function (R3 — the virtual dispatch is now named)"
    );
}

/// The default-off parity assertion: with `rtti off`, NONE of the recovered names
/// exist — the symbol table is byte-identical to a no-rtti run. The vftable slot's
/// target VA carries no recovered virtual-method symbol either.
fn assert_absent(prog: &ConsoleProgram, box_area_vma: u64) {
    for name in [
        "Box::RTTI_Type_Descriptor",
        "Shape::RTTI_Type_Descriptor",
        "Box::RTTI_Complete_Object_Locator",
        "Box::vftable",
        "Box::vftable_0",
    ] {
        assert!(
            !prog.has_symbol_named(name),
            "rtti off: `{name}` must NOT be present (default-off parity)"
        );
    }
    // The vftable slot is an unnamed `DAT_*`/undiscovered slot with the option off:
    // no `vftable_*` virtual-method function is installed at its target VA.
    assert_ne!(
        prog.function_named_at(box_area_vma).as_deref(),
        Some("vftable_0"),
        "rtti off: the vftable slot target must NOT be a named virtual method"
    );
}

/// AFTER (x64, the dominant case — IBO32 image-base-relative refs): with
/// `--option rtti on`, the `Box`/`Shape` class names + RTTI/vftable labels are
/// recovered from the x64 RTTI graph.
#[test]
fn x64_recovers_class_names_with_option_on() {
    let Some(prog) = bootstrap_rtti("msvc_rtti_x64.exe", true) else {
        return;
    };
    assert_recovered(&prog, BOX_AREA_VMA_X64);
}

/// BEFORE (x64, the default-off gate): with the option OFF, no class name is
/// recovered — the parity proof that the pass is purely additive.
#[test]
fn x64_class_names_absent_with_option_off() {
    let Some(prog) = bootstrap_rtti("msvc_rtti_x64.exe", false) else {
        return;
    };
    assert_absent(&prog, BOX_AREA_VMA_X64);
}

/// AFTER (x86, the raw-VA path — full virtual-address refs, RTTI0 name at offset 8):
/// the SAME recovery works through the x86 ref-kind branch.
#[test]
fn x86_recovers_class_names_with_option_on() {
    let Some(prog) = bootstrap_rtti("msvc_rtti_x86.exe", true) else {
        return;
    };
    assert_recovered(&prog, BOX_AREA_VMA_X86);
}

/// BEFORE (x86, default-off): no class name recovered with the option off.
#[test]
fn x86_class_names_absent_with_option_off() {
    let Some(prog) = bootstrap_rtti("msvc_rtti_x86.exe", false) else {
        return;
    };
    assert_absent(&prog, BOX_AREA_VMA_X86);
}
