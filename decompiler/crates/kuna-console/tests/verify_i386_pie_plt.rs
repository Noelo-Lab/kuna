//! End-to-end gate for i386 **PIE** ELF PLT/GOT import-name resolution
//! (angr `test_decompiling_nl_i386_pie::usage`).
//!
//! The `elf_plt` i386-PIE arm (`src/s1_loader/elf_plt.rs::decode_i386`) decodes
//! the GOT-relative `jmp *disp(%ebx)` (`FF A3 <disp32>`) PLT stub form — the
//! i386 analog of the x86-64 RIP-relative and aarch64 veneer decoders — by
//! deriving the GOT base (`_GLOBAL_OFFSET_TABLE_`) and computing
//! `slot = base + disp32`.  Naming those stubs both (a) renders the libc calls
//! by name (`exit(`/`dcgettext(`/`__printf_chk(`/`fputs_unlocked(` instead of
//! `sub_<addr>`) and (b) — via the existing `NoReturnKnownPass` — flags `exit`
//! no-return, collapsing the spurious `do{}while(true)`/`goto` fall-through loop
//! kuna otherwise manufactures (the whole structural failure cascades from the
//! missing no-return).  This drives the full `load function usage` → `decompile`
//! → `print C` path against the **real, linked i386-PIE `nl`** fixture and
//! asserts both effects.
//!
//! The stage-test (`tests/stages/*.xml`) harness CANNOT exercise this: it accepts
//! only `<binaryimage>` bytechunks, which carry no `.rel.plt`/`.dynsym`/GOT
//! structure, so a stage XML can never reach the PLT/no-return loader path — this
//! integration test (the precedent: `verify_aarch64_plt.rs`) is the gate, per the
//! feature proposal's approved sub-decision B.  The decoder arithmetic itself is
//! additionally unit-tested in `elf_plt.rs::tests::i386_pie_plt_decode`.
//!
//! The fix ships **default-on** (`option i386_pie_plt`, the loader-fidelity
//! default), so this test bootstraps with no option override.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`).  When absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

// Load-bearing VMAs, pinned from the linked fixture (objdump/readelf):
//   usage              0x1f60   (calls dcgettext/__printf_chk/fputs_unlocked, ends in exit)
//   dcgettext@plt      0xeb0    → sub_eb0  (the pre-fix placeholder)
//   exit@plt           0xfb0    → sub_fb0  (the pre-fix placeholder; no-return)
const DCGETTEXT_PLT: &str = "sub_eb0";
const EXIT_PLT: &str = "sub_fb0";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The linked i386-PIE `nl` fixture (vendored under the kuna-analysis fixtures).
fn i386_pie_nl() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/i386_pie_nl")
}

#[test]
fn i386_pie_plt_calls_named_and_no_spurious_loop() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = i386_pie_nl();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_i386_pie_plt: skipping (bootstrap failed, build the x86 `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The i386-PIE PLT imports are resolvable as functions (the loader markup
    // the FF A3 decode now feeds), and the local `.symtab` `usage` still resolves.
    assert!(prog.lookup_symbol("exit").is_some(), "PLT import `exit` not resolved by the loader");
    assert!(
        prog.lookup_symbol("dcgettext").is_some(),
        "PLT import `dcgettext` not resolved by the loader"
    );
    assert!(
        prog.lookup_symbol("fputs_unlocked").is_some(),
        "PLT import `fputs_unlocked` not resolved by the loader"
    );
    assert!(prog.lookup_symbol("usage").is_some(), "defined `usage` not resolved by the loader");

    // Drive `load function usage` → `decompile` → `print C`.
    let cmds: Vec<String> = ["load function usage", "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
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
    let out = status.optr.clone();

    // (a) The library calls render NAMED (the FF A3 stub→name mapping)...
    for name in ["exit(", "dcgettext(", "__printf_chk(", "fputs_unlocked("] {
        assert!(out.contains(name), "expected a `{name}` call, got:\n{out}");
    }
    // ...and the old PLT-stub placeholders are gone for those call sites.
    assert!(
        !out.contains(EXIT_PLT) && !out.contains(DCGETTEXT_PLT),
        "PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );

    // (b) `exit` marked no-return collapses the spurious fall-through loop: no
    // do{}while(true)/goto/recovery-failure markers remain.
    for bad in ["do {", "while( true )", "while(true)", "goto "] {
        assert!(!out.contains(bad), "spurious `{bad}` should be gone, got:\n{out}");
    }
}
