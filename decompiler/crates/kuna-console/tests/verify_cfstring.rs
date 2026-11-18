//! `cfstrings` end-to-end gate — render a Mach-O Core Foundation constant string
//! (`__cfstring`) as `CFSTR("...")` of the string it holds, instead of the entry.
//!
//! Fixture: `tests/fixtures/macho_cfstring` — a self-contained arm64 Mach-O built
//! from `macho_cfstring.c` with `zig cc -target aarch64-macos -O0`, its STABS
//! debug map stripped. It hand-builds
//! three `__builtin_CFString` constants in `__DATA_CONST,__cfstring` (8-bit,
//! `flags` 0x7c8) and a getter returning each one's address. The second string is
//! longer than a 32-byte entry; the third entry sits right behind it. The char
//! pointers are classic `LC_DYLD_INFO` pointers into `__TEXT,__cstring`. The build
//! recipe + the pinned VMAs are in `tests/fixtures/README.md`.
//!
//! ## The three-state proof (the `verify_objc` posture)
//!
//! - **`--option cfstrings off`**: each getter returns the
//!   `__cfstring` entry by its data symbol — the CFString struct the printer
//!   cannot follow on its own.
//! - **`--option cfstrings on`** (default): each returns `CFSTR("...")` and is
//!   typed `CFStringRef` — the entry's char pointer resolved, the string seeded
//!   into the `StringManager`, the entry typed an opaque `struct __CFString` over
//!   its own 32 bytes only, so a long string never hides the next entry.
//! - **`--option cfstrings annotate`**: the same, followed by a comment naming the
//!   entry address.
//!
//! Both run on the real-Mach-O path, so the XML datatest oracles never reach this.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

/// `(getter VMA, entry rendered when off, entry address, string rendered when on)`.
/// The second string is longer than one 32-byte entry, and the third entry sits
/// right behind it, so it is only reachable when the second entry's type stays
/// inside its own entry.
const ENTRIES: [(u64, &str, u64, &str); 3] = [
    (0x100000538, "&_kGreeting", 0x100004008, "cfstring hello"),
    (0x100000544, "&_kLong", 0x100004028, "a constant string longer than one entry"),
    (0x100000550, "&_kAfter", 0x100004048, "after the long one"),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

#[derive(Clone, Copy)]
enum Mode {
    Off,
    On,
    Annotate,
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the fixture, (optionally) enable `cfstrings`, commit, then decompile
/// `_get_greeting` and return its rendered C.
fn run(mode: Mode, vma: u64) -> String {
    run_with(mode, vma, None)
}

/// [`run`], with a data symbol of type `ty` named `planted` already mapped at
/// the first entry before the commit, as a debug or analysis symbol would be.
fn run_with(mode: Mode, vma: u64, planted: Option<(&str, kuna_decomp::dtype::type_metatype)>) -> String {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixtures().join("macho_cfstring");
    assert!(bin.exists(), "missing fixture {bin:?}");

    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");

    let value = match mode {
        Mode::Off => "off",
        Mode::On => "on",
        Mode::Annotate => "annotate",
    };
    prog.arch_mut().set_kuna_option("cfstrings", value).expect("cfstrings takes the value");
    if let Some((name, meta)) = planted {
        let arch = prog.arch_mut();
        let space = arch.manage().get_default_code_space().expect("a default code space").clone();
        let entry = Address::new(space, ENTRIES[0].2);
        let ty = arch.types().get_base(8, meta).expect("an 8-byte base type");
        let global = arch.symboltab.get_global_scope().expect("a global scope");
        arch.symboltab.add_symbol_mapped(global, name, ty, &entry, &Address::new_invalid()).expect("map the symbol");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let name = name_at(&prog, vma);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, addr, kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the _get_greeting funcdata builds")
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
    status.optr.clone()
}

/// The three-state proof: a `__cfstring` reference is the entry (its data
/// symbol) when `cfstrings` is off, `CFSTR("...")` of its string when it is on, and
/// that plus a comment naming the entry address under `annotate`, for every
/// entry, including the one behind an entry whose string is longer than it.
#[test]
fn cfstring_renders_as_literal_only_when_on() {
    for (vma, symbol, entry, text) in ENTRIES {
        let off = run(Mode::Off, vma);
        let on = run(Mode::On, vma);
        let annotated = run(Mode::Annotate, vma);
        assert!(
            off.contains(&format!("return {symbol};")),
            "cfstrings off must render the __cfstring entry {symbol}:\n{off}"
        );
        assert!(!off.contains(text), "cfstrings off must not render {text:?}:\n{off}");
        assert!(
            on.contains(&format!("return CFSTR(\"{text}\");")),
            "cfstrings on must render CFSTR({text:?}):\n{on}"
        );
        assert!(!on.contains("cfstring@"), "cfstrings on must not annotate:\n{on}");
        assert!(
            annotated.contains(&format!("return CFSTR(\"{text}\") /* cfstring@0x{entry:x} */;")),
            "cfstrings annotate must name the entry 0x{entry:x}:\n{annotated}"
        );
        assert!(
            on.contains("CFStringRef ") && !on.contains("char *"),
            "cfstrings on types the constant a CFStringRef:\n{on}"
        );
        assert!(
            annotated.contains("CFStringRef "),
            "cfstrings annotate types the constant a CFStringRef:\n{annotated}"
        );
        assert!(!off.contains("CFStringRef"), "cfstrings off leaves the type alone:\n{off}");
    }
}

/// A symbol already on a `__cfstring` entry (debug info, another pass) keeps
/// its name, but the entry still reads as its string: the literal is
/// registered whatever sits there.
#[test]
fn cfstring_under_an_existing_symbol_still_renders() {
    use kuna_decomp::dtype::type_metatype;
    let (vma, _, _, text) = ENTRIES[0];
    for meta in [type_metatype::TYPE_UNKNOWN, type_metatype::TYPE_INT] {
        let on = run_with(Mode::On, vma, Some(("kGreetingDebug", meta)));
        assert!(on.contains(&format!("CFSTR(\"{text}\")")), "an entry under a symbol must render CFSTR({text:?}):\n{on}");
    }
}
