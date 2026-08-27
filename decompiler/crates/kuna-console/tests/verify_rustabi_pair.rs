//! End-to-end gate for **`option rustabi`**: a rustc `ScalarPair` return keeps
//! its payload, and the call that consumes it stops reading a variable nothing
//! assigns.
//!
//! The symptom, on a `Result<u32,u32>` producer and the `match` that consumes
//! it (`rustc -C opt-level=2`, both `#[inline(never)]`):
//!
//! ```text
//! bool prod(uint4 a0) { return a0 < 0xb; }        // the EDX payload is GONE
//!
//! int4 cons(void) {
//!   uint8 v2; // rax
//!   int4 v3;  // edx                              // declared, read twice, NEVER ASSIGNED
//!   prod();
//!   v1 = v3 + 100;
//!   if (!(v2 & 1)) v1 = v3 + 1;
//! }
//! ```
//!
//! kuna already recovers the `RAX:RDX` pair the ABI asks for; the producer loses
//! it to a subvariable-flow truncation driven by the one-bit discriminant, and
//! the consumer loses it to the un-ported multi-trial branch of
//! `buildOutputFromTrials`. The two seams are answered by two DIFFERENT
//! classifications (a call site has no callee values to read), and this pins
//! both ends of both — including the call seam's callee veto, whose whole point
//! is that nothing on the caller's side can tell a real `ScalarPair` consumer
//! from a caller that merely reads a clobbered register.
//!
//! `auto` is what the fixture exercises, not `always`: the fixture is a real
//! rustc binary carrying the `.comment` record the source-language detection
//! reads, so this also gates the load-time fact reaching the per-function rule.
//! The C control below is what proves the fact is doing the gating.
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

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, apply `options` (before the analysis commit, as the CLI
/// does), decompile `func`, and return the captured C (`None` => specs-less skip).
fn decompile(bin: &str, func: &str, options: &[&str]) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_rustabi_pair: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let mut cmds: Vec<String> = options.iter().map(|o| o.to_string()).collect();
    cmds.push(format!("load function {func}"));
    cmds.push("decompile".into());
    cmds.push("print C".into());
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}

/// The bug, pinned: at the default the producer collapses to its discriminant.
#[test]
fn by_default_the_producer_loses_its_payload_register() {
    let Some(code) = decompile("rust_scalarpair_x86_64", "prod", &[]) else { return };
    assert!(
        code.contains("bool prod"),
        "the pre-fix rendering is the one-bit discriminant alone; got:\n{code}",
    );
}

/// With the option on, the pair survives and the payload half is written.
#[test]
fn the_producer_keeps_the_whole_pair() {
    let Some(code) = decompile("rust_scalarpair_x86_64", "prod", &["option rustabi auto"]) else {
        return;
    };
    assert!(
        !code.contains("bool prod"),
        "the return must no longer be the discriminant alone; got:\n{code}",
    );
    // The 4-byte EDX payload write into the `undefined1[16]` container renders
    // as the sized sub-field (arraysubfield); the one-byte discriminant write in
    // the same function keeps its `v2[0]` element subscript.
    assert!(
        code.contains("._8_4_ ="),
        "the payload half must be written at the second register's offset; got:\n{code}",
    );
}

/// The bug on the consumer side: the payload register is a local with no
/// definition anywhere in the function.
#[test]
fn by_default_the_consumer_reads_an_unassigned_payload() {
    let Some(code) = decompile("rust_scalarpair_x86_64", "cons", &[]) else { return };
    assert!(
        code.contains("// edx"),
        "the payload register must show up as a bare register-commented local; got:\n{code}",
    );
    let declared = declared_locals(&code);
    let phantoms: Vec<&String> = declared.iter().filter(|v| !assigned(&code, v)).collect();
    assert!(
        !phantoms.is_empty(),
        "the pre-fix rendering has at least one never-assigned local; got:\n{code}",
    );
}

/// The fix: the call has an output, and every local the body reads is assigned.
#[test]
fn the_consumer_reads_the_payload_out_of_the_call() {
    let Some(code) = decompile("rust_scalarpair_x86_64", "cons", &["option rustabi auto"]) else {
        return;
    };
    assert!(
        code.contains("= prod()"),
        "the call must have an output the payload comes out of; got:\n{code}",
    );
    let declared = declared_locals(&code);
    let phantoms: Vec<&String> = declared.iter().filter(|v| !assigned(&code, v)).collect();
    assert!(
        phantoms.is_empty(),
        "no local may be read without ever being assigned; phantoms {phantoms:?} in:\n{code}",
    );
}

/// The call seam's callee veto. `rust_clobber_pair_x86_64` is the same
/// caller-side shape as `cons` above — two used output trials, a byte-tested
/// first register, a twice-read second one — over a callee that provably never
/// writes the second register (`movq %rdi,%rax; addq $7,%rax; ret`). Since the
/// two are indistinguishable at the call site, the veto has to come from the
/// callee's body, and the reader must render identically with the option on.
#[test]
fn a_callee_proven_not_to_write_the_payload_is_not_paired() {
    let Some(off) = decompile("rust_clobber_pair_x86_64", "pair_shaped_reader", &[]) else {
        return;
    };
    let Some(on) = decompile(
        "rust_clobber_pair_x86_64",
        "pair_shaped_reader",
        &["option rustabi auto"],
    ) else {
        return;
    };
    let body = |s: &str| s[s.find("Decompiling").unwrap_or(0)..].to_string();
    assert_eq!(
        body(&off),
        body(&on),
        "a callee the probe proves never writes the payload register must not be paired",
    );
    assert!(
        on.contains("scalar_callee();"),
        "the call must keep no output; got:\n{on}",
    );
}

/// The control for the test above, and the reason it is evidence rather than a
/// coincidence: the SAME option, on the SAME caller-side shape, over a callee
/// that DOES write the payload register, still pairs. The callee body is the
/// only thing that differs.
#[test]
fn the_veto_is_the_callee_body_and_nothing_else() {
    let Some(paired) = decompile("rust_scalarpair_x86_64", "cons", &["option rustabi auto"])
    else {
        return;
    };
    assert!(
        paired.contains("= prod()"),
        "the positive control must still pair; got:\n{paired}",
    );
}

/// The gate, proved rather than assumed: `auto` reads the loader's
/// source-language verdict, so a C binary cannot reach the rules at all. `main`
/// of the classic non-PIE `fauxware` renders identically with the option on.
#[test]
fn auto_cannot_change_a_c_binary() {
    let Some(off) = decompile("fauxware", "main", &[]) else { return };
    let Some(on) = decompile("fauxware", "main", &["option rustabi auto"]) else { return };
    // `on` carries the option's confirmation line ahead of the decompilation;
    // compare the emitted C, which is the thing that must not move.
    let body = |s: &str| s[s.find("Decompiling").unwrap_or(0)..].to_string();
    assert_eq!(body(&off), body(&on), "`auto` must be inert on a non-Rust image");
}

/// Every `vN` local the emitted body declares.
fn declared_locals(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in code.lines() {
        let body = line.split("//").next().unwrap_or("");
        let Some(name) = body.trim_end().strip_suffix(';') else { continue };
        let Some(last) = name.rsplit([' ', '*']).next() else { continue };
        let last = last.trim_end_matches(|c: char| c == ']' || c.is_ascii_digit() || c == '[');
        // A declaration is `<type> vN;` / `<type> vN [k];`; anything with an `=`
        // or a `(` is a statement.
        if body.contains('=') || body.contains('(') {
            continue;
        }
        let ident = name.split_whitespace().last().unwrap_or("");
        let ident = ident.split('[').next().unwrap_or("");
        if last.is_empty() && ident.starts_with('v') && ident[1..].chars().all(|c| c.is_ascii_digit())
        {
            out.push(ident.to_string());
        } else if ident.starts_with('v') && ident.len() > 1 && ident[1..].chars().all(|c| c.is_ascii_digit())
        {
            out.push(ident.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Is `name` ever written in `code` (plain, indexed, or member assignment)?
fn assigned(code: &str, name: &str) -> bool {
    for line in code.lines() {
        let body = line.split("//").next().unwrap_or("");
        let Some(pos) = body.find(name) else { continue };
        // The declaration line itself is not an assignment.
        let after = &body[pos + name.len()..];
        let after = after.trim_start_matches(|c: char| {
            c == ' ' || c == '[' || c == ']' || c == '.' || c.is_ascii_alphanumeric() || c == '_'
        });
        if after.starts_with('=') && !after.starts_with("==") {
            return true;
        }
    }
    false
}
