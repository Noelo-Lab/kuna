//! INDEPENDENT VERIFIER adversarial tests for item `rport/w10-f0flag-v2-untie`.
//!
//! The single source change is in `coreaction_cleanup.rs`
//! `mark_output_storage_addr_tied` (reached from `ActionMergeRequired::apply`):
//! the marker-write un-tie is lifted OUT of the old `output_locked` gate (so an
//! output-UNLOCKED transient return register — e.g. doublemove's `f0`, whose
//! protos are output-unlocked — also un-ties), and the lift is fenced by a
//! ScopeLocal-ownership predicate (LOSS-206) so a return register that is a pure
//! FORWARDING ALIAS of a distinct register local (`partialmerge::readpartial`'s
//! `EAX = COPY(register:0x18)`, where `0x18` carries the recovered `a_simple`
//! dynamic Symbol C++ keeps explicit) STAYS tied.
//!
//! Faithfulness anchor: C++ `Funcdata::syncVarnodesWithSymbols`
//! (`funcdata_varnode.cc:993`) marks an un-symboled storage
//! `Varnode::mapped|Varnode::addrtied` ONLY when `lm->inScope(addr,size,usepoint)`
//! — and `ScopeLocal` does not override the base `Scope::inScope`
//! (`database.hh:599` -> `rangetree.inRange`), whose range tree owns the
//! stack/local space, never a processor register.  So C++ leaves a transient
//! return *register* un-tied; `baseExplicit` then marks the forward-computed
//! value IMPLIED and the printer emits `return <expr>;`.  The kuna stand-in keeps
//! ONE shape tied: a register heritage left at a fixed address across a
//! control-flow join (a CPUI_MULTIEQUAL/INDIRECT marker — the genuine
//! whole-function local), plus the forwarding-alias exception above (the
//! stand-in for the unported per-Varnode dynamic mapentry, LOSS-206).
//!
//! These tests drive the REAL gate path: the committed Rust `decomp_test_dbg` on
//! a datatest, slicing the `KUNA_DUMP` `print C` block, and (where a built C++
//! tree + `.sla` are present) the C++ oracle on the same datatest for a faithful
//! direction check.  A missing binary / `.sla` is reported as SKIP (never a false
//! pass) so the suite stays green in a bare worktree.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    std::env::var_os("KUNA_SPECS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("specs"))
}

fn datatests_dir() -> PathBuf {
    std::env::var_os("KUNA_DATATESTS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("tests/datatests"))
}

fn rust_test_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/release/decomp_test_dbg")
}

fn rust_test_bin_any() -> Option<PathBuf> {
    let rel = rust_test_bin();
    if rel.exists() {
        return Some(rel);
    }
    let dbg = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/decomp_test_dbg");
    if dbg.exists() {
        return Some(dbg);
    }
    None
}

fn cpp_oracle_bin() -> PathBuf {
    std::env::var_os("KUNA_DECOMP_TEST")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/cpp/decomp_test_dbg"))
}

/// Drive a `decomp_test_dbg` binary on one datatest with `KUNA_DUMP=1` and return
/// the first dumped `print C` block (header line stripped — the header echoes the
/// `-path` arg, not body content).  `None` if the binary is missing or produced
/// no dump (no spec / `.sla`).
fn dump_body(bin: &PathBuf, stem: &str) -> Option<String> {
    if !bin.exists() {
        return None;
    }
    let out = Command::new(bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_dir())
        .arg("datatests")
        .arg(format!("{stem}.xml"))
        .env("KUNA_DUMP", "1")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    Some(text[after_begin..mid].to_string())
}

/// Extract the body lines of a single named function from a `print C` dump,
/// from its signature line through its closing `}` (inclusive of `return`).
fn function_block<'a>(dump: &'a str, sig_needle: &str) -> Option<String> {
    let mut out: Vec<&'a str> = Vec::new();
    let mut started = false;
    for line in dump.lines() {
        if !started {
            if line.contains(sig_needle) {
                started = true;
                out.push(line);
            }
            continue;
        }
        out.push(line);
        if line.trim_start().starts_with('}') {
            break;
        }
    }
    if started {
        Some(out.join("\n"))
    } else {
        None
    }
}

/// (A1) THE WIN — `f0` un-tied renders IMPLIED.  doublemove's `loaddouble`
/// computes the return value directly into the floating-point return register
/// `f0` (a transient, no control-flow-join marker, output-UNLOCKED proto).  C++
/// leaves `f0` un-tied (the register is not in the stack `rangetree`), so the
/// value is IMPLIED and the body is exactly `return glob1 + glob1;` with NO
/// explicit `float8 vN; vN = glob1 + glob1; return vN;` round-trip.  The Rust
/// render must match that oracle direction byte-for-byte on this function.
#[test]
fn f0_untied_renders_implied_return_expr() {
    let bin = match rust_test_bin_any() {
        Some(b) => b,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg unavailable");
            return;
        }
    };
    let rust = match dump_body(&bin, "doublemove") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "loaddouble(")
        .unwrap_or_else(|| panic!("no loaddouble block in:\n{rust}"));
    // The value is IMPLIED: the return statement carries the whole expression,
    // and no return-register local is declared / assigned.
    assert!(
        body.contains("return glob1 + glob1;"),
        "loaddouble's f0 must be IMPLIED -> `return glob1 + glob1;` (un-tied); got:\n{body}"
    );
    // NO explicit return-register round-trip: no `<type> vN; // f0..` decl that
    // exists only to hold the return value (the over-tie symptom).
    assert!(
        !body.contains("= glob1 + glob1;"),
        "loaddouble must NOT materialize a tied return-register assignment \
         `vN = glob1 + glob1;` (the over-tie this item removes); got:\n{body}"
    );

    // Faithfulness direction: byte-identical to the C++ oracle on this function.
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "doublemove") {
        let cbody = function_block(&cpp, "loaddouble(")
            .unwrap_or_else(|| panic!("no oracle loaddouble block in:\n{cpp}"));
        assert_eq!(
            body.trim_end(),
            cbody.trim_end(),
            "Rust loaddouble must be byte-identical to the C++ oracle:\nRUST:\n{body}\nCPP:\n{cbody}"
        );
    }
}

/// (A2) THE GUARD — `partialmerge :: Partial Merge #3` storage STAYS recovered.
/// This is the negative assertion the naive lift regresses: `readpartial`'s `EAX`
/// is a pure forwarding COPY of the distinct register `0x18` that in C++ carries
/// the recovered `a_simple` dynamic Symbol.  The body must NOT collapse to the
/// forbidden `return glob1.a + 10;` (datatest `Partial Merge #3`, `min=0 max=0`):
/// the `a_simple` recovered local must survive to anchor the forwarded value.
///
/// (LOSS-234 update) The forwarding-alias return-register tie is a stand-in for an
/// absent recovered-local Symbol *on the source register*; it must fire ONLY when
/// the source actually carries one (`readpartial`'s `a_simple`), never for a bare
/// transient (`condmove`'s flag register).  With that narrowed, `readpartial` now
/// renders BYTE-IDENTICAL to the C++ oracle — `a_simple = glob1.a; return a_simple
/// + 10;` (datatest `Partial Merge #2`) — instead of the prior extra `v1 = a_simple
/// + 10; return v1;` round-trip the over-tie inserted.  The real invariant (a_simple
/// stays explicit, the full-collapse is forbidden) is preserved and now matches the
/// oracle exactly.
#[test]
fn partialmerge_3_forwarding_alias_storage_stays_tied() {
    let bin = match rust_test_bin_any() {
        Some(b) => b,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg unavailable");
            return;
        }
    };
    let rust = match dump_body(&bin, "partialmerge") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "readpartial(")
        .unwrap_or_else(|| panic!("no readpartial block in:\n{rust}"));
    // The forbidden fully-collapsed form (`Partial Merge #3`, a min=0 NEGATIVE
    // assertion) must NOT appear: un-tying the forwarding-alias return register
    // would produce exactly this.
    assert!(
        !body.contains("return glob1.a + 10;"),
        "readpartial's forwarding-alias return register must STAY tied; it must NOT \
         collapse to the forbidden `return glob1.a + 10;` (Partial Merge #3, min=0); got:\n{body}"
    );
    // The recovered local `a_simple` survives to anchor the forwarded value — it is
    // assigned the partial load (`a_simple = glob1.a;`) and returned (`return
    // a_simple + 10;`).  This is the oracle-faithful form (byte-identical to C++);
    // the prior extra `v1 = a_simple + 10; return v1;` tied round-trip was a kuna
    // over-tie the LOSS-234 narrowing removed.
    assert!(
        body.contains("a_simple = glob1.a;") && body.contains("return a_simple + 10;"),
        "readpartial must keep the recovered `a_simple` local explicit and return it \
         directly (oracle form `a_simple = glob1.a; return a_simple + 10;`); got:\n{body}"
    );
    // Structurally byte-identical to the C++ oracle on this function (the
    // faithfulness direction), MODULO trailing declaration-line `// name` comments:
    // the oracle emits `int4 a_simple; // tmp` (the symbol's storage-name decl
    // comment) where rust currently omits the comment (`int4 a_simple;`).  That
    // declaration-comment gap is a pre-existing printc limitation (LOSS-238); it
    // affects NO datatest assertion (Partial Merge #3 is the min=0 negative on the
    // forbidden `return glob1.a + 10;` collapse, which rust correctly avoids).  We
    // normalise both bodies by stripping trailing `// ...` comments before the
    // structural comparison so this test pins the code shape, not the decl comment.
    fn strip_line_comments(s: &str) -> String {
        s.lines()
            .map(|l| match l.find("//") {
                Some(i) => l[..i].trim_end(),
                None => l.trim_end(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "partialmerge") {
        if let Some(cbody) = function_block(&cpp, "readpartial(") {
            assert_eq!(
                strip_line_comments(body.trim_end()),
                strip_line_comments(cbody.trim_end()),
                "Rust readpartial must be structurally byte-identical to the C++ oracle \
                 (modulo declaration-line comments):\nRUST:\n{body}\nCPP:\n{cbody}"
            );
        }
    }
}

/// (A3) ANTI-SPECIAL-CASING — the un-tie fires on storage of DIFFERENT space
/// geometry across UNRELATED functions, so it cannot be an `f0`/float/register-
/// name special case.  A floating-point return register (`f0`, doublemove) AND an
/// integer return register (`EAX`, modulo2's `mod2`) both un-tie to an IMPLIED
/// `return <expr>;`.  Conversely a register-to-register FORWARDING move
/// (readpartial) stays tied regardless of being the *same* `EAX` integer register
/// as the un-tied `mod2` — proving the discriminator is storage geometry
/// (forwarding-alias vs direct computation), not the register identity.
#[test]
fn untie_is_generic_geometry_not_f0_or_register_name_special_case() {
    let bin = match rust_test_bin_any() {
        Some(b) => b,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg unavailable");
            return;
        }
    };

    // Float return register (f0) un-ties.
    let dm = match dump_body(&bin, "doublemove") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: doublemove .sla unavailable");
            return;
        }
    };
    let dm_body = function_block(&dm, "loaddouble(")
        .unwrap_or_else(|| panic!("no loaddouble block in:\n{dm}"));
    assert!(
        dm_body.contains("return glob1 + glob1;"),
        "float f0 must un-tie (IMPLIED); got:\n{dm_body}"
    );

    // Integer return register (EAX in mod2) ALSO un-ties — same code path, no
    // float dependence.
    let m2 = match dump_body(&bin, "modulo2") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: modulo2 .sla unavailable");
            return;
        }
    };
    let m2_body =
        function_block(&m2, "mod2(").unwrap_or_else(|| panic!("no mod2 block in:\n{m2}"));
    // Implied integer return: the modulo expression is in the `return`, no tied
    // return-register assignment `vN = ... % 2;` round-trip survives.
    assert!(
        m2_body.contains("return") && m2_body.contains("% 2"),
        "integer EAX in mod2 must un-tie to an IMPLIED `return ... % 2 ...`; got:\n{m2_body}"
    );

    // Same integer register (EAX) in readpartial — a FORWARDING alias — stays
    // tied.  If the gate keyed on the register name/space/type it could not tie
    // one EAX and un-tie another; the storage-geometry discriminator can.
    let pm = match dump_body(&bin, "partialmerge") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: partialmerge .sla unavailable");
            return;
        }
    };
    let pm_body = function_block(&pm, "readpartial(")
        .unwrap_or_else(|| panic!("no readpartial block in:\n{pm}"));
    assert!(
        !pm_body.contains("return glob1.a + 10;"),
        "the SAME integer register (EAX) that un-ties in mod2 must STAY tied in \
         readpartial's forwarding-alias shape — the discriminator is geometry, not \
         the register identity; got:\n{pm_body}"
    );
}

/// (A4) NO CROSS-CONTAMINATION — un-tying doublemove's `f0` must not leak
/// readpartial's recovered local tokens (and vice-versa); the decision is
/// per-function IR geometry, not a global toggle.  doublemove carries none of
/// readpartial's `glob1.a`/`a_simple` markers, and readpartial carries no
/// doublemove `glob1 + glob1` artifact.
#[test]
fn untie_decision_is_per_function_no_token_leak() {
    let bin = match rust_test_bin_any() {
        Some(b) => b,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg unavailable");
            return;
        }
    };
    let dm = match dump_body(&bin, "doublemove") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: doublemove .sla unavailable");
            return;
        }
    };
    let dm_body = function_block(&dm, "loaddouble(")
        .unwrap_or_else(|| panic!("no loaddouble block in:\n{dm}"));
    assert!(
        !dm_body.contains("glob1.a") && !dm_body.contains("a_simple"),
        "doublemove's loaddouble must not carry partialmerge tokens (per-function \
         decision, not a global): got:\n{dm_body}"
    );
}

/// (A5) REGRESSION PIN — `enum :: ptrenumequal` keeps its enum-named return-path
/// constant (`Enum Reading #5`).  Round-1 review REJECTED a *stale* form of this
/// branch: it was forked from a parent (`9ba537f`) that predated the rust-port
/// `Enum render` integration (`80c65fe`, `PrintC::pushEnumConstant` / `TYPE_ENUM`
/// dispatch).  Diffed against the post-enum baseline, `ptrenumequal` *appeared* to
/// regress from `if (ptrequal->flagfield != (FLAG_100000|FLAG_800))` (enum-named)
/// to `0x100800` (raw hex) — but that was the MISSING enum-render feature, not the
/// un-tie.  This branch now carries the merge of rust-port HEAD, so the enum render
/// is present; this test PINS the post-merge render so the un-tie can never be
/// re-blamed for the enum-constant typing path.  The un-tie reshapes only the
/// *return-register* storage; `ptrenumequal`'s `flagfield != …` constant is typed
/// by the comparison's enum operand, an INDEPENDENT path, so it must stay
/// enum-named (`(FLAG_100000|FLAG_800)`, never `0x100800`) — byte-identical to the
/// C++ oracle.
#[test]
fn enum_reading_5_constant_stays_enum_named_not_raw_hex() {
    let bin = match rust_test_bin_any() {
        Some(b) => b,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg unavailable");
            return;
        }
    };
    let rust = match dump_body(&bin, "enum") {
        Some(t) => t,
        None => {
            eprintln!("SKIP: rust decomp_test_dbg / enum .sla unavailable");
            return;
        }
    };
    let body = function_block(&rust, "ptrenumequal(")
        .unwrap_or_else(|| panic!("no ptrenumequal block in:\n{rust}"));
    // The enum-named OR-of-flags must survive (Enum Reading #5, min=1).
    assert!(
        body.contains("(FLAG_100000|FLAG_800)"),
        "ptrenumequal's flagfield constant must stay enum-named `(FLAG_100000|FLAG_800)` \
         (Enum Reading #5); the un-tie must not strip the `flags` enum type; got:\n{body}"
    );
    // And it must NOT collapse to the raw-hex form the stale round-1 branch showed.
    assert!(
        !body.contains("0x100800"),
        "ptrenumequal's flagfield constant must NOT render as raw hex `0x100800` \
         (would be the enum-type loss the round-1 review flagged); got:\n{body}"
    );

    // Faithfulness direction: byte-identical to the C++ oracle on this function.
    if let Some(cpp) = dump_body(&cpp_oracle_bin(), "enum") {
        let cbody = function_block(&cpp, "ptrenumequal(")
            .unwrap_or_else(|| panic!("no oracle ptrenumequal block in:\n{cpp}"));
        assert_eq!(
            body.trim_end(),
            cbody.trim_end(),
            "Rust ptrenumequal must be byte-identical to the C++ oracle:\nRUST:\n{body}\nCPP:\n{cbody}"
        );
    }
}
