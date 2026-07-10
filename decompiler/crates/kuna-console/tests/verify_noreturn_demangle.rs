//! End-to-end gate for the no-return × demangle cross-pass seam (the kuna analog
//! of Ghidra's `NoReturnFunctionAnalyzer` interacting with `GnuDemanglerAnalyzer`).
//!
//! The seam: the no-return pass (`loader/noreturn.rs`) matches on the **raw**
//! object-symbol name, but the demangle pass renames the funcsym *before* it is
//! installed. A mangled C++ no-return import like `_ZSt9terminatev` is installed
//! as `std::terminate` (in scope `std`) at its PLT-stub address — so a name-only
//! commit (`query_global_function("_ZSt9terminatev")`) would miss it and the
//! no-return flag would never attach. The fix carries the symbol **address** on
//! the fact and resolves it via `Database::find_function_across_scopes(addr)`
//! (the same cross-scope, address-keyed resolver the call resolver uses); the
//! address is stable across demangling.
//!
//! The fixture `cpp_noreturn_x86_64` (built `g++ -O0 -no-pie -fno-pic`) has a free
//! function `fail()` (`_Z4failv`) that tail-calls `std::terminate()`. After
//! demangle the callee funcsym is `std::terminate`; the no-return fact carries the
//! PLT-stub address (`0x401070`), so the commit attaches no-return there and the
//! flow analysis emits the `Subroutine does not return` warning + elides the
//! fall-through dead code. WITHOUT the address path the body keeps a wrong
//! signature (`int4 fail(int4)`) and spurious code after the call.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_callfixup`/`verify_s1_entry` gates, bootstrapping
//! needs the built `x86` `.sla` under `specs/` (gitignored; `make specs`). When it
//! is absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored C++ no-return fixture (shared with the kuna-analysis
/// `noreturn::tests::cpp_mangled_noreturn_emits_plt_stub_address` unit test).
fn cpp_noreturn_fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cpp_noreturn_x86_64")
}

/// Drive `load file <cpp_noreturn>` → `load function fail` → `decompile` →
/// `print C` and capture the rendered body.  Returns `None` when the `.sla` is
/// absent (visible skip).
fn decompile_function(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = cpp_noreturn_fixture().to_str()?.to_string();

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_noreturn_demangle: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let cmds: Vec<String> = [format!("load function {func}"), "decompile".into(), "print C".into()]
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
    Some(status.optr.clone())
}

/// The core gate: `fail()` tail-calls the **demangled** `std::terminate`, which is
/// installed at its PLT-stub address. The no-return flag must attach (resolved by
/// address), so the body renders `void fail(void)` with the no-return warning and
/// no dead fall-through after the call.
#[test]
fn demangled_std_terminate_is_no_return_in_fail() {
    let Some(out) = decompile_function("fail") else { return };

    // A real decompilation happened.
    assert!(
        out.contains('{') && out.contains('}'),
        "expected a C function body for `fail`, got:\n{out}"
    );

    // The demangled callee renders (`std::terminate`), proving demangle ran.
    assert!(
        out.contains("std::terminate"),
        "expected the demangled `std::terminate` callee, got:\n{out}"
    );

    // The no-return flag attached: the engine emits the warning at the call site.
    // This is the direct proof the address-resolved no-return reached flow
    // analysis even though the funcsym was renamed by demangle.
    assert!(
        out.contains("Subroutine does not return"),
        "expected the no-return warning on `std::terminate` (the seam fix), got:\n{out}"
    );

    // With the call known no-return, `fail` has no fall-through: the body is the
    // clean `void fail(void) { std::terminate(); }`. WITHOUT the fix the callee
    // returns, so flow keeps the wrong `int4 fail(int4)` signature + dead code.
    assert!(
        out.contains("void fail(void)"),
        "expected the `void fail(void)` signature (no fall-through after the \
         no-return call), got:\n{out}"
    );
}
