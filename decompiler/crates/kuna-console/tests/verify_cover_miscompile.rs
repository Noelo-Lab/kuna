//! End-to-end gate: the decompiler must never emit C that **computes a different
//! value than the binary**.
//!
//! Two P6 Cover-extension defects each produced a silent miscompilation; this test
//! pins the VALUE-carrying statement for both, against the `covercopy_x86_64`
//! fixture (`+.c`, a non-PIE `gcc -O0` x86-64 ELF built for exactly these shapes).
//!
//! 1. **Dropped restore** (`Merge::checkCopyPair`, `merge.cc:1121`).
//!    `lookup_service` has three `return name;` guards sharing one `-O0` epilogue,
//!    and the middle path clobbers the return register with `lookup()`'s result
//!    before reloading `name`.  The port built `checkCopyPair`'s dominance range
//!    from the dominant COPY's def point alone, omitting
//!    `range.addRefPoint(subOp, subOp->getIn(0))`, so the intervening
//!    `v = lookup(...)` write was never seen inside the range and
//!    `markRedundantCopies` marked the reload non-printing.  The emitted C then
//!    returned the NULL from the failed lookup instead of the parameter.
//!
//! 2. **Over-merge** (`Merge::markImplied`, `merge.cc:1595-1605`, plus the
//!    `Varnode::setFlags` -> `high->coverDirty()` forward, `varnode.cc:377-378`).
//!    `two_selects` has two `cond ? step : 0` phis whose reads are both inlined
//!    into one call argument printed after both writes.  Without dirtying the
//!    operands' Covers on implied-marking, `Cover::rebuild`'s forward walk through
//!    implied consumers never runs, the two phis look cover-disjoint, and the
//!    speculative merge folds them into one variable — so the emitted C subtracts
//!    the second select's value twice.

use std::path::PathBuf;
use std::process::Command;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/covercopy_x86_64")
}

/// Bootstrap the fixture and decompile `func`, returning the printed C.
fn decompile(func: &str) -> String {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str().expect("UTF-8 fixture path").to_string();
    let prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");

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
    status.optr.clone()
}

#[test]
fn returned_parameter_survives_a_failed_lookup() {
    let c = decompile("lookup_service");
    let declaration = c.lines().find(|line| line.contains("lookup_service("))
        .expect("lookup_service declaration");
    let code = &c[c.find(declaration).unwrap()..];
    let source = std::env::temp_dir().join(format!("kuna-returned-parameter-{}.c", std::process::id()));
    std::fs::write(&source, format!(r#"
#include <stdint.h>
#include <stdlib.h>
typedef int32_t int4;
struct svc {{ char *name; }};
char service[] = "http";
__attribute__((noinline)) struct svc *lookup(int port) {{
    static struct svc s = {{service}};
    return port == 80 ? &s : 0;
}}
int4 is_digit_c(int4 c) {{ return c >= '0' && c <= '9'; }}
{code}
int main(void) {{
    char alpha[] = "alpha", zero[] = "0", missing[] = "99999", found[] = "80";
    char *names[] = {{alpha, zero, missing, found}};
    for (unsigned i = 0; i < 4; ++i) {{
        int4 warnings = 0;
        if (lookup_service(names[i], &warnings) != (i == 3 ? service : names[i])) return 1;
        if (warnings != (i == 2)) return 2;
    }}
    return 0;
}}
"#)).unwrap();
    let mut checked = false;
    for compiler in ["gcc", "clang"] {
        let available = match Command::new(compiler).arg("--version").output() {
            Ok(output) => {
                assert!(output.status.success(), "{compiler} --version failed");
                true
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(error) => panic!("cannot run {compiler}: {error}"),
        };
        if !available {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let executable = source.with_extension(format!("{compiler}{}", &level[1..]));
            let output = Command::new(compiler)
                .args(["-std=c11", level, "-Werror=int-conversion", "-Werror=incompatible-pointer-types"])
                .arg(&source).arg("-o").arg(&executable).output().unwrap();
            assert!(output.status.success(), "{}\n{code}", String::from_utf8_lossy(&output.stderr));
            assert!(Command::new(&executable).status().unwrap().success(), "{code}");
            std::fs::remove_file(executable).unwrap();
            checked = true;
        }
    }
    std::fs::remove_file(source).unwrap();
    if !checked {
        eprintln!("no C compiler available; skipping emitted-C execution");
    }
}

#[test]
fn independent_selects_do_not_share_one_variable() {
    let c = decompile("two_selects");
    eprintln!("---- two_selects ----\n{c}");

    // Sanity: both selects must still render as assignment diamonds (this is the
    // shape the over-merge needs; if a later pass folds them to `?:` the test
    // would pass vacuously).
    let step_writes: Vec<&str> = c
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.ends_with("= g_step;"))
        .collect();
    assert_eq!(
        step_writes.len(),
        2,
        "expected two `vN = g_step;` select arms (the shape this bug needs), got {step_writes:?}\n--- C ---\n{c}"
    );

    // The bug: both phis merged into ONE HighVariable, so the call argument read
    // the same variable twice and the first select's value was lost.
    let lhs: std::collections::BTreeSet<&str> =
        step_writes.iter().map(|l| l.split(" =").next().unwrap()).collect();
    assert_eq!(
        lhs.len(),
        2,
        "the two independent selects share ONE variable — the emitted call argument \
         subtracts the second select's value twice and drops the first \
         (Merge::markImplied cover-dirty, merge.cc:1595-1605)\n--- C ---\n{c}"
    );

    // And the call argument must read both of them.
    let call = c
        .lines()
        .find(|l| l.contains("emit("))
        .expect("two_selects must call emit(...)");
    for v in &lhs {
        assert!(
            call.contains(&format!("- {v}")),
            "the emit(...) argument does not subtract `{v}`: {call}\n--- C ---\n{c}"
        );
    }
}
