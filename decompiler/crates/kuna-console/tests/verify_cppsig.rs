//! End-to-end gate for the demangled-C++-signature arm (`--option cppsig`).
//!
//! The gap: `kuna_analysis::demangle` reduced a mangled symbol to its qualified
//! NAME and threw the rest of the declaration away — its own header recorded the
//! signature application as "a deferred follow-up", and `demangle_raw` (the full
//! c++filt form) had no production caller at all. So on a STRIPPED C++ shared
//! library, where `.dynsym` still carries the whole exported API, every member
//! function decompiled as `f(int8 *a0,unsigned long a1)`.
//!
//! ## Why a Rust integration test and not a `tests/stages/*.xml` case
//!
//! The XML datatest bootstrap builds its program from a `<binaryimage>` element
//! and never runs the analyzer tier — it produces no `AnalysisOutput` at all, so
//! no `kuna_analysis` pass (this one included) can fire there. The same reason
//! `verify_cppproto.rs` / `verify_cppcallnames.rs` / `verify_fdeinterior.rs`
//! exist. The two-pass discipline is kept: every test below decompiles the same
//! function under two gate settings and asserts the before AND the after.
//!
//! The fixture `cppsig_x86_64.so` (source vendored alongside) is `strip
//! --strip-all`ped, so there is no DWARF and no `.symtab` — the mangled `.dynsym`
//! names are the only signature source in the file, which is exactly the
//! situation the feature exists for.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the stripped C++ fixture with `cppsig` set to `mode`, commit the
/// analysis facts under that gate, decompile `func` and return the captured C.
/// `None` => specs-less skip.
fn decompile(func: &str, mode: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cppsig_x86_64.so")
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_cppsig: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The gate is read at the analysis COMMIT boundary (the producing pass runs
    // at `load file`, upstream of any `option` command), so it must be set here.
    prog.arch_mut().set_kuna_option("cppsig", mode).expect("cppsig is a registered option");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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

/// The headline, and the default: a **destructor** cannot be static, so the
/// mangling entails an implicit object parameter. `_ZN3sig7AccountD1Ev`.
#[test]
fn destructor_recovers_the_class_typed_this_at_the_default() {
    let Some(off) = decompile("sig::Account::~Account", "off") else { return };
    assert!(
        off.contains("sig::Account::~Account(unsigned int *a0)"),
        "gate off must reproduce the untyped `a0` signature, got:\n{off}"
    );

    let on = decompile("sig::Account::~Account", "proven").expect("second pass bootstraps");
    assert!(
        on.contains("sig::Account::~Account(Account *this)"),
        "the default (`proven`) must recover `Account *this`, got:\n{on}"
    );
}

/// A **constructor** (`C1`/`C2`), likewise entailed, plus its declared parameter.
#[test]
fn constructor_recovers_this_and_the_declared_parameter() {
    let Some(off) = decompile("sig::Account::Account", "off") else { return };
    assert!(
        off.contains("sig::Account::Account(unsigned int *a0,unsigned int a1)"),
        "gate off signature, got:\n{off}"
    );

    let on = decompile("sig::Account::Account", "proven").expect("second pass bootstraps");
    assert!(
        on.contains("sig::Account::Account(Account *this,int4 a1)"),
        "`proven` must recover `Account *this` plus the declared `int`, got:\n{on}"
    );
}

/// A **`const` member** (`_ZNK`): a cv qualifier can only attach to an implicit
/// object parameter, so a static member can never carry one.
///
/// This one also pins the return-type contract: `balance()` returns `int`, and
/// the recovered signature must NOT flatten it to `void`. Itanium encodes no
/// return type for an ordinary function, so the prototype locks the INPUT half
/// only and kuna's own return recovery still runs (upstream's
/// `DemangledFunction.resolveReturnType` returns null for the same reason).
#[test]
fn const_member_is_proven_and_keeps_its_recovered_return_type() {
    let Some(off) = decompile("sig::Account::balance", "off") else { return };
    assert!(
        off.contains("sig::Account::balance(unsigned int *a0)"),
        "gate off signature, got:\n{off}"
    );

    let on = decompile("sig::Account::balance", "proven").expect("second pass bootstraps");
    assert!(
        on.contains("sig::Account::balance(Account *this)"),
        "a `const` member's `this` is entailed by the mangling, got:\n{on}"
    );
    assert!(
        on.contains("unsigned int sig::Account::balance"),
        "the recovered return type must survive the input-only lock, got:\n{on}"
    );
}

/// A **plain member function** is NOT proven: Itanium mangles
/// `sig::Account::deposit(Ledger*, int)` and the static `sig::Account::rate(int)`
/// with the same nested-name shape. The default leaves it alone; `inferred`
/// decides it from class evidence and gets it right.
#[test]
fn plain_member_needs_inferred_and_then_types_every_parameter() {
    let Some(default) = decompile("sig::Account::deposit", "proven") else { return };
    assert!(
        default.contains("sig::Account::deposit(int4 *a0,unsigned int a1,int4 a2)"),
        "the default must NOT guess a `this` on an ambiguous nested name, got:\n{default}"
    );

    let inferred = decompile("sig::Account::deposit", "inferred").expect("second pass bootstraps");
    assert!(
        inferred.contains("sig::Account::deposit(Account *this,Ledger *a1,int4 a2)"),
        "`inferred` must recover `this` AND both declared parameter types, got:\n{inferred}"
    );
}

/// A **namespaced free function** has no `this`, and getting that wrong would
/// shift every parameter. `inferred` reads `sig` as a namespace (no constructor,
/// destructor, cv-member or `_ZTV`/`_ZTI`/`_ZTS` symbol is filed under it) and
/// applies the declared type at position 0.
#[test]
fn namespaced_free_function_gets_no_this() {
    let Some(off) = decompile("sig::combine", "off") else { return };
    assert!(
        off.contains("sig::combine(unsigned int a0,int4 a1)"),
        "gate off signature, got:\n{off}"
    );

    let inferred = decompile("sig::combine", "inferred").expect("second pass bootstraps");
    assert!(
        inferred.contains("sig::combine(Account *a0,int4 a1)"),
        "a namespaced free function must keep its parameters in place, got:\n{inferred}"
    );
    assert!(
        !inferred.contains("sig::combine(Account *this"),
        "no implicit object parameter may be invented here, got:\n{inferred}"
    );
}

/// DWARF beats the demangler. On a `-g` binary BOTH `cppproto` (#264) and this
/// arm reach the same function; the mangled symbol carries a DECLARATION, DWARF
/// carries ground truth, so the commit applies this one FIRST and lets the DWARF
/// prototype overwrite it. The witness is the parameter NAME: a mangled symbol
/// has none, so `amount` surviving proves which source won.
#[test]
fn dwarf_wins_over_the_demangled_signature() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cppproto_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    for mode in ["off", "proven", "inferred"] {
        let Ok(mut prog) = bootstrap_from_object(&path, "", &spec_roots) else { return };
        prog.arch_mut().set_kuna_option("cppsig", mode).unwrap();
        prog.arch_mut().set_kuna_option("cppproto", "on").unwrap();
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        let cmds: Vec<String> = [
            "load function Account::deposit".into(),
            "decompile".into(),
            "print C".into(),
        ]
        .to_vec();
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
        assert!(
            status.optr.contains("Account::deposit(Account *this,int4 amount)"),
            "DWARF must win at cppsig={mode} (the parameter name proves it), got:\n{}",
            status.optr
        );
    }
}

/// The measured cost of `inferred`, pinned so it cannot drift silently: a
/// **static member function** carries no `this`, and nothing in the Itanium
/// mangling says so. `proven` therefore refuses it; `inferred` gets it WRONG.
///
/// On google/leveldb this is the whole 7.2% of `inferred`'s error (precision
/// 0.9278 vs 1.0000 for `proven`), which is why `proven` is the default.
#[test]
fn static_member_is_refused_by_proven_and_mis_typed_by_inferred() {
    let Some(default) = decompile("sig::Account::rate", "proven") else { return };
    assert!(
        default.contains("sig::Account::rate(int4 a0)"),
        "`proven` must refuse a static member outright, got:\n{default}"
    );

    let inferred = decompile("sig::Account::rate", "inferred").expect("second pass bootstraps");
    assert!(
        inferred.contains("sig::Account::rate(Account *this,int4 a1)"),
        "`inferred`'s known cost: a spurious `this` on a static member, got:\n{inferred}"
    );
}
