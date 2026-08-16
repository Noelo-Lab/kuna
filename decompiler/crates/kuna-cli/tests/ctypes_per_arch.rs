//! CLI end-to-end gate for `option ctypes`: the emitted C carries the target's
//! own C type names, and the SAME core type spells differently on targets with
//! different data models.
//!
//! The per-architecture claim is what needs a gate. A table that is right for
//! x86-64 Linux and wrong for i386 would pass any single-target test, and the
//! specific trap is the 8-byte integer: `int8` must spell `long` under LP64 (where
//! `long` is 8 bytes) and `long long` under ILP32 (where it is 4). Both fixtures
//! are decompiled here in one gate so the two spellings are compared directly
//! rather than asserted apart.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the command fails to build an architecture; the
//! test prints that and returns early (a specs-less CI is a visible skip, never
//! a false green).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// Every Ghidra core-type name that can reach the emitted C. `ctypes on` must
/// leave none of them behind; that is the whole contract.
const GHIDRA_TYPE_NAMES: &[&str] = &[
    "uint1", "uint2", "uint4", "uint8", "int1", "int2", "int4", "int8", "float4", "float8",
    "float10", "float16", "xunknown1", "xunknown2", "xunknown4", "xunknown8", "wchar2", "wchar4",
];

/// `kuna decompile-all <fixture> --option ctypes <on|off>`; `None` on a
/// missing-`.sla` skip.
fn decompile_all(fixture_name: &str, ctypes: &str) -> Option<String> {
    let sp = specs();
    let bin = fixture(fixture_name);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", &bin, "--sleighpath", &sp, "--option", "ctypes", ctypes])
        .output()
        .expect("failed to spawn the kuna binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        if stderr.contains("could not build an architecture") {
            eprintln!("ctypes_per_arch: skipping (no .sla; build with `make specs`)");
            return None;
        }
        panic!("kuna decompile-all failed on {fixture_name}: {stderr}");
    }
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Count whole-word occurrences of `needle` in `hay`, so `int` does not match
/// inside `print` and `long` does not match inside `long long`'s second word.
fn word_count(hay: &str, needle: &str) -> usize {
    let bytes = hay.as_bytes();
    let mut n = 0;
    let mut from = 0;
    while let Some(i) = hay[from..].find(needle) {
        let s = from + i;
        let e = s + needle.len();
        let before_ok = s == 0 || !(bytes[s - 1] as char).is_ascii_alphanumeric() && bytes[s - 1] != b'_';
        let after_ok =
            e == bytes.len() || !(bytes[e] as char).is_ascii_alphanumeric() && bytes[e] != b'_';
        if before_ok && after_ok {
            n += 1;
        }
        from = e;
    }
    n
}

/// The headline contract: with `ctypes on`, no Ghidra core-type name survives
/// into the emitted C, on any target. Asserted over four data models so a table
/// that happens to be complete for one cannot pass.
#[test]
fn no_ghidra_type_name_survives_on_any_target() {
    for name in ["fmt_x86_64", "i386_pie_nl", "fmt_arm", "fmt_aarch64"] {
        let Some(c) = decompile_all(name, "on") else { return };
        for ghidra in GHIDRA_TYPE_NAMES {
            assert_eq!(
                word_count(&c, ghidra),
                0,
                "{name}: `{ghidra}` survived into the emitted C with ctypes on"
            );
        }
    }
}

/// The per-architecture claim, compared directly rather than asserted apart: the
/// same 8-byte core type spells `long` under LP64 and `long long` under ILP32.
/// Getting this backwards is the single most likely way a size->name table is
/// wrong, and it is invisible on either target alone.
#[test]
fn eight_byte_integers_spell_per_data_model() {
    let Some(lp64) = decompile_all("fmt_x86_64", "on") else { return };
    let Some(ilp32) = decompile_all("i386_pie_nl", "on") else { return };

    // x86-64 System V is LP64: `long` is 8 bytes, so the 8-byte core types land
    // there and `long long` is never reached.
    assert!(
        word_count(&lp64, "unsigned long") > 0,
        "LP64 must spell an 8-byte unsigned `unsigned long`"
    );
    assert_eq!(
        word_count(&lp64, "long long"),
        0,
        "LP64 must NOT reach `long long` -- `long` is already 8 bytes there"
    );

    // i386 is ILP32: `long` is 4, so the same 8-byte types must reach `long long`.
    assert!(
        word_count(&ilp32, "long long") > 0,
        "ILP32 must spell an 8-byte integer `long long`, not `long`"
    );
}

/// `ctypes off` restores the Ghidra vocabulary exactly, so the option is a real
/// gate and the parity corpus (which runs at the shipped default of off) is
/// unaffected.
#[test]
fn off_keeps_the_ghidra_vocabulary() {
    let Some(off) = decompile_all("i386_pie_nl", "off") else { return };
    assert!(word_count(&off, "int4") > 0, "ctypes off must keep the core-type names");
    assert!(word_count(&off, "int8") > 0, "ctypes off must keep the 8-byte core-type names");
}

/// The spelling must be *consistent*, not merely valid: the defect this feature
/// fixes is that one function rendered `unsigned int` (via the older `realtypes`
/// relabel, which covers only residual TYPE_UNKNOWN) beside `int4` (a genuine
/// TYPE_INT core type). With `ctypes on` both vocabularies collapse onto one.
#[test]
fn the_two_vocabularies_no_longer_coexist() {
    let Some(on) = decompile_all("i386_pie_nl", "on") else { return };
    let Some(off) = decompile_all("i386_pie_nl", "off") else { return };
    // Off: both a C spelling (from realtypes) and a Ghidra spelling are present --
    // the reported mixture.
    assert!(word_count(&off, "unsigned int") > 0 && word_count(&off, "uint4") > 0);
    // On: only the C spelling remains.
    assert!(word_count(&on, "unsigned int") > 0);
    assert_eq!(word_count(&on, "uint4"), 0);
}
