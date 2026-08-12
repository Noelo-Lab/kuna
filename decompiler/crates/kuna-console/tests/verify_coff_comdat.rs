//! COFF **function-level-linking** gate: an object whose functions each live in
//! their own COMDAT `.text` section loads *every* function, not just the first.
//!
//! ## The defect this pins
//!
//! A COFF object stores every section at VMA 0 — the section-relative base a
//! linker later assigns.  Unlike an ELF `ET_REL` (no program headers at all), a
//! COFF object *does* answer `file.segments()`, so the loader's mapped-image path
//! accepted it and mapped all of them on top of each other at address 0.  With
//! one `.text` per object that is survivable (the historical `coff_obj.obj`
//! fixture, `verify_coff_object`); with MSVC function-level linking (`/Gy`, and
//! clang's `-ffunction-sections`) — the default for real game/engine builds — it
//! means:
//!
//! ```text
//!   before:  kuna functions combat.obj  →  1 function  (all 13 collide at 0x0)
//!   after:   kuna functions combat.obj  →  13 functions at distinct VMAs
//! ```
//!
//! Every function but the first was unreachable: its symbol was deduped away by
//! address and its bytes were shadowed by whichever section sorted first.
//!
//! ## The fixture
//!
//! `coff_comdat_i386.c` (vendored beside the `.obj`):
//!
//! ```c
//!   int g_counter;
//!   extern int ext_helper(int);
//!   int alpha(int x) { return x * 3 + 1; }
//!   int beta(int x)  { return alpha(x) + g_counter; }
//!   int gamma_(int x){ return ext_helper(x) + beta(x); }
//! ```
//!
//! built `clang -target i386-pc-windows-msvc -O1 -ffunction-sections
//! -fdata-sections -c` — three separate `.text` sections all at VMA 0, a `.bss`
//! for `g_counter`, an undefined `ext_helper` external, and both COFF relocation
//! shapes with their addends stored *in place*: `DIR32` (absolute, the
//! `g_counter` load) and `REL32` (PC-relative, the `ext_helper` call).  It also
//! carries the empty `.text`/`.data`/`.bss` placeholders and an
//! `IMAGE_SCN_LNK_REMOVE` `.llvm_addrsig` section, so the layout's
//! "memory-resident only" and "skip empty" filters are exercised too.
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, the decompile half needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`).  If it is absent the
//! bootstrap fails and that half prints a visible skip.  The loader half needs no
//! `.sla` and always runs.

use std::collections::BTreeMap;
use std::path::PathBuf;

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/coff_comdat_i386.obj")
}

/// The loader's function symbols, keyed by name.
fn func_syms() -> BTreeMap<String, u64> {
    let path = fixture();
    assert!(path.exists(), "missing fixture {path:?}");
    let bytes = std::fs::read(&path).unwrap();
    let image = ObjectLoadImage::from_bytes(path.to_str().unwrap(), &bytes)
        .expect("COFF object must load");
    image.func_symbols().into_iter().map(|(a, n)| (n, a)).collect()
}

/// The "before" half, stated structurally: the object really does stack every
/// `.text` section at VMA 0, so the mapped-image path *cannot* tell them apart —
/// which is why a COFF object needs the synthetic layout.
#[test]
fn coff_comdat_sections_all_claim_vma_zero() {
    use object::read::{Object, ObjectSection};

    let bytes = std::fs::read(fixture()).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let text: Vec<_> = file
        .sections()
        .filter(|s| s.name() == Ok(".text") && s.size() > 0)
        .map(|s| s.address())
        .collect();
    assert!(text.len() >= 3, "fixture must carry ≥3 non-empty COMDAT .text sections");
    assert!(
        text.iter().all(|a| *a == 0),
        "COFF sections are section-relative: every one must report VMA 0, got {text:?}"
    );
}

/// The headline: all three COMDAT functions survive the load, at *distinct*
/// rebased addresses.  Before the layout, `beta`/`gamma_` deduped away against
/// `alpha`'s address 0 and only one function existed.
#[test]
fn coff_comdat_loads_every_function() {
    let syms = func_syms();
    for name in ["_alpha", "_beta", "_gamma_"] {
        assert!(syms.contains_key(name), "COMDAT function {name} missing; loader saw {syms:?}");
    }
    let addrs: Vec<u64> = ["_alpha", "_beta", "_gamma_"].iter().map(|n| syms[*n]).collect();
    assert!(
        addrs.iter().all(|a| *a >= kuna_analysis::loader::reloc_object::RELOC_BASE),
        "every function must be rebased above RELOC_BASE, got {addrs:x?}"
    );
    let mut sorted = addrs.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 3, "the three functions must occupy distinct VMAs, got {addrs:x?}");
}

/// Bootstrap the object, returning `None` (a visible skip) when the `.sla` is
/// absent.
fn boot() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    match bootstrap_from_object(fixture().to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_coff_comdat: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// Run `load function <name>` → `decompile` → `print C` and return the C.
fn decompile_func(prog: ConsoleProgram, name: &str) -> String {
    let cmds: Vec<String> = [format!("load function {name}"), "decompile".into(), "print C".into()]
        .into_iter()
        .collect();
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
    status.optr.clone()
}

/// A function from the *third* COMDAT section decompiles its real body — the
/// end-to-end proof that the rebased bytes, not just the rebased symbol, reach
/// the lifter.  `beta` is `alpha(x) + g_counter`, which clang inlines to
/// `x * 3 + 1 + g_counter`; the `g_counter` reference exercises the in-place
/// (`DIR32`) addend the COFF relocation carries in the patched field itself.
#[test]
fn coff_comdat_second_section_decompiles() {
    let Some(prog) = boot() else { return };
    let out = decompile_func(prog, "_beta");
    assert!(out.contains("_beta"), "expected the function to be named `_beta`, got:\n{out}");
    assert!(
        out.contains("* 3 +"),
        "`_beta` should decompile its inlined multiply-add body, got:\n{out}"
    );
    assert!(
        !out.contains("Unable to load"),
        "the rebased bytes must be mapped for a non-first COMDAT section, got:\n{out}"
    );
}
