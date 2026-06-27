//! s1_objc end-to-end gate — THE HEADLINE: recover an Objective-C method name in
//! a (locally-)stripped Mach-O from the `__objc_*` metadata, on **x86-64 and
//! arm64**.
//!
//! Fixtures: `tests/fixtures/macho_objc` (x86-64) + `tests/fixtures/macho_objc_arm64`
//! (arm64) — both self-contained Mach-Os built from `macho_objc.m` (a root class
//! `Greeter` with `-(int)greet:(int)n` → `n*3+7`). A **root class**
//! (`objc_root_class`) needs no macOS SDK / Foundation, so they build with bare
//! `clang` + the rustup `ld64.lld` (the exact `macho_imports` recipe). Linked with
//! `-x` (strip local symbols), so the IMP `-[Greeter greet:]` has NO leftover
//! symbol — only the `__objc_*` metadata can recover the name. The arm64 slice is
//! built with `-fixup_chains`, so it carries a real `LC_DYLD_CHAINED_FIXUPS` —
//! naming its IMP exercises the PR-O0 chained-fixup resolver + the PR-O2 arm64
//! walk together. The build recipe + the pinned VMAs are in `tests/fixtures/README.md`.
//!
//! ## The two-state proof (the `verify_fid` posture)
//!
//!  - **`--option objc off`** (default): the IMP at `0x100000640` carries an engine
//!    `sub_<addr>` placeholder — proving the name is NOT a leftover symbol (it was
//!    stripped) and that the ObjC pass's value is what's visible.
//!  - **`--option objc on`**: the SAME function is now **`-[Greeter greet:]`** —
//!    recovered purely by the ObjC metadata walk (classlist → class_t → class_ro_t
//!    → method_list_t → IMP). Only the ObjC pass could do this.
//!
//! The proof is read two ways: (1) the symbol-table name at the IMP VMA (the rename
//! ground truth) and (2) the decompiled C (the name renders in the header). Both run
//! on the real-Mach-O path (loading an actual Mach-O), so the XML datatest 675/158
//! oracles never reach this.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

/// The pinned VMA of the IMP for `-[Greeter greet:]` in the x86-64 fixture
/// (`__text` base; PIE imagebase `0x100000000`). See `tests/fixtures/README.md`.
const GREET_IMP_VMA: u64 = 0x100000640;

/// The pinned VMA of the IMP in the **arm64** (chained-fixup) fixture. Differs
/// from the x86-64 VMA because the arm64 slice's `__text` layout differs.
const GREET_IMP_VMA_ARM64: u64 = 0x1000005f0;

/// The recovered Objective-C method name the pass installs.
const GREET_NAME: &str = "-[Greeter greet:]";

/// One fixture under test: the binary name + the IMP VMA to resolve.
#[derive(Clone, Copy)]
struct Fixture {
    bin: &'static str,
    imp_vma: u64,
}

const X86_64: Fixture = Fixture { bin: "macho_objc", imp_vma: GREET_IMP_VMA };
const ARM64: Fixture = Fixture { bin: "macho_objc_arm64", imp_vma: GREET_IMP_VMA_ARM64 };

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Whether to enable the ObjC metadata pass.
enum Mode {
    /// Default: no flag (the today baseline — the stripped placeholder).
    Off,
    /// `--option objc on`, set BEFORE the deferred commit (the live-CLI ordering).
    On,
}

/// The result of a run: the symbol-table name at the IMP VMA (the rename ground
/// truth) + the decompiled C of that function.
struct Run {
    name: String,
    body: String,
}

/// Resolve the symbol-table name at the fixture's IMP VMA (the engine placeholder,
/// or the ObjC-installed method name).
fn name_at_imp(prog: &ConsoleProgram, imp_vma: u64) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), imp_vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the fixture, (optionally) enable the ObjC pass, then resolve +
/// decompile the IMP at the fixture's IMP VMA. `None` ⇒ specs-less skip.
fn run(fx: Fixture, mode: Mode) -> Option<Run> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixtures().join(fx.bin);
    assert!(bin.exists(), "missing fixture {bin:?}");

    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_objc: skipping (bootstrap failed, build `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // Live-CLI ordering: the `option` line precedes the deferred commit. Flip the
    // flag on the live arch BEFORE committing so the gated objc facts are applied.
    if let Mode::On = mode {
        prog.arch_mut().set_kuna_option("objc", "on").expect("objc flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    // The rename ground truth: the symbol-table name at the IMP after the commit.
    let name = name_at_imp(&prog, fx.imp_vma);

    // Decompile the IMP and render its body through the full console pipeline
    // (`decompile` → `print C`, the same path `verify_macho_imports` drives), naming
    // the function with whatever the symbol table now holds (the engine
    // `sub_<addr>` placeholder when objc is off, the recovered `-[Greeter greet:]`
    // when it is on). We seed `dcp.fd` by building the funcdata in-process
    // (`build_and_follow_flow`) — rather than the console `load function`/`load addr`
    // grammar — because an ObjC method name carries a space the console tokenizer
    // would split; `IfcDecompile` then rebuilds the IR from that funcdata's
    // name/entry, so the rendered header carries the recovered name.
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), fx.imp_vma);
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, addr, kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the IMP funcdata builds")
    };

    let cmds: Vec<String> = ["decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(Run { name, body: status.optr.clone() })
}

/// The shared two-state proof for one fixture: the IMP is a generic `sub_<addr>`
/// placeholder by default, and becomes `-[Greeter greet:]` only when the ObjC pass
/// is on — recovered purely by the `__objc_*` metadata walk. Returns `false` on a
/// specs-less skip.
fn assert_two_state_recovery(fx: Fixture) -> bool {
    let Some(off) = run(fx, Mode::Off) else {
        return false; // specs-less skip
    };
    let on = run(fx, Mode::On).expect("second bootstrap succeeds if the first did");

    eprintln!(
        "==== {} 0x{:x} symbol  OFF: {:>20}   ON: {:>20} ====",
        fx.bin, fx.imp_vma, off.name, on.name
    );
    eprintln!("---- objc OFF (default) ----\n{}", off.body);
    eprintln!("---- objc ON ----\n{}", on.body);

    // 1. objc OFF (default): the IMP's name is a generic engine placeholder
    //    (`sub_*`/`FUN_*`/`func_*`) — NOT the ObjC method name (it was stripped).
    assert!(
        is_placeholder(&off.name),
        "[{}] default (objc off) must leave 0x{:x} a generic placeholder, got `{}`",
        fx.bin, fx.imp_vma, off.name
    );
    assert_ne!(
        off.name, GREET_NAME,
        "[{}] default (objc off) must NOT name the stripped IMP {GREET_NAME} (proves it is not a leftover symbol)",
        fx.bin
    );
    assert!(
        !off.body.contains(GREET_NAME),
        "[{}] default (objc off) body must not render {GREET_NAME}:\n{}",
        fx.bin, off.body
    );

    // 2. objc ON: the SAME function is now `-[Greeter greet:]`, recovered from the
    //    metadata — in the symbol table AND in the rendered body.
    assert_eq!(
        on.name, GREET_NAME,
        "[{}] objc on must rename 0x{:x} to {GREET_NAME} from the __objc_* metadata (symbol table)",
        fx.bin, fx.imp_vma
    );
    assert!(
        on.body.contains(GREET_NAME),
        "[{}] objc on body must render the recovered method name {GREET_NAME}:\n{}",
        fx.bin, on.body
    );

    // 3. The symbol name changed — the ObjC pass performed the rename.
    assert_ne!(
        off.name, on.name,
        "[{}] objc must change the IMP's name (the placeholder -> {GREET_NAME} rename)",
        fx.bin
    );
    true
}

/// THE HEADLINE (x86-64): the IMP at `0x100000640` in the stripped Mach-O is a
/// generic `sub_<addr>` placeholder by default, and becomes `-[Greeter greet:]`
/// only when the ObjC pass is on — recovered purely by the `__objc_*` metadata.
#[test]
fn objc_recovers_stripped_imp_method_name() {
    assert_two_state_recovery(X86_64);
}

/// THE arm64 HEADLINE (PR-O0 + PR-O2): the SAME recovery on a **real
/// `LC_DYLD_CHAINED_FIXUPS`** arm64 Mach-O. Proving `-[Greeter greet:]` here proves
/// the chained-fixup resolver rebased the `__objc_classlist`/`class_t` slots (whose
/// raw words are packed fixup entries) AND that the arm64 ObjC walk then names the
/// IMP. Without the resolver the classlist would dereference to garbage and the
/// name would never recover.
#[test]
fn objc_recovers_arm64_chained_fixup_imp_method_name() {
    assert_two_state_recovery(ARM64);
}

/// The default path is byte-identical regardless of how it is reached: with NO objc
/// flag the IMP stays a `sub_<addr>` placeholder (the pass never fires by default).
#[test]
fn objc_off_is_the_today_baseline() {
    let Some(off) = run(X86_64, Mode::Off) else {
        return; // specs-less skip
    };
    assert!(
        is_placeholder(&off.name),
        "default name must be a placeholder, got `{}`",
        off.name
    );
    assert_ne!(off.name, GREET_NAME, "default must not recover the stripped method name");
    assert!(!off.body.trim().is_empty(), "expected a non-empty body for the IMP function");
}

/// Is `name` an engine-generated placeholder (`sub_*`/`func_*`/`FUN_*`/`LAB_*`)?
fn is_placeholder(name: &str) -> bool {
    name.starts_with("sub_")
        || name.starts_with("func_")
        || name.starts_with("FUN_")
        || name.starts_with("LAB_")
}
