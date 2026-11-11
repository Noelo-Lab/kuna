//! (GH #1357) Every name a Mach-O or PE symbol table gives one function entry
//! selects it: the inventory keeps one row per address with the other names as
//! aliases, and `decompile` by any of them, in either Mach-O spelling, lifts
//! that entry. A name that is an alias at two addresses stays ambiguous.
//!
//! Fixtures: `macho_alias_names_arm64` (GH #1357's assembly plus two objects'
//! local labels) and `pe_imports.exe` (a MinGW image whose `_fpreset` is also
//! `fpreset`). Integration tests require the built processor specs under
//! `specs/`.

use std::path::PathBuf;
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    let path = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    path.to_str().unwrap().to_string()
}

fn kuna(args: &[&str]) -> Output {
    let specs = repo_root().join("specs");
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(args)
        .args(["--sleighpath", specs.to_str().unwrap()])
        .output()
        .expect("run kuna")
}

/// The address `decompile <image> <name> --json` lifted, or its stderr.
fn selected(image: &str, name: &str) -> Result<u64, String> {
    let out = kuna(&["decompile", image, name, "--json"]);
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("{name}: {e}: {}", String::from_utf8_lossy(&out.stderr)))?;
    match doc["functions"][0]["address"].as_u64() {
        Some(addr) if out.status.success() => Ok(addr),
        _ => Err(format!("{name}: {}", doc["error"])),
    }
}

#[test]
fn every_macho_name_at_an_entry_selects_it() {
    let image = fixture("macho_alias_names_arm64");
    let out = kuna(&["functions", &image]);
    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let rows: Vec<Vec<&str>> = listing.lines().map(|l| l.split('\t').collect()).collect();
    let addrs: Vec<&str> = rows.iter().map(|r| r[0]).collect();
    assert_eq!(addrs, ["0x240", "0x248", "0x250", "0x254", "0x25c", "0x264"], "one row per entry:\n{listing}");
    assert!(listing.contains("0x240\t_audit_export_a\t(sub_240, _audit_export_b)"), "{listing}");
    assert!(listing.contains("0x248\t_audit_local_a\t(sub_248, _audit_local_b)"), "{listing}");
    assert!(!listing.contains("audit_export_a,") && !listing.contains(" audit_export_b"), "{listing}");

    for (name, addr) in [
        ("_audit_export_a", 0x240),
        ("_audit_export_b", 0x240),
        ("audit_export_b", 0x240),
        ("_audit_local_a", 0x248),
        ("_audit_local_b", 0x248),
        ("audit_local_b", 0x248),
        ("_audit_shared", 0x25c),
        ("audit_shared", 0x25c),
    ] {
        assert_eq!(selected(&image, name), Ok(addr), "{name}");
    }
    for name in ["_audit_both", "audit_both"] {
        let err = selected(&image, name).unwrap_err();
        assert!(err.contains("ambiguous") && err.contains("0x254") && err.contains("0x264"), "{err}");
    }

    let out = kuna(&["decompile-all", &image]);
    let c = String::from_utf8_lossy(&out.stdout);
    assert_eq!(c.matches("// Function:").count(), 6, "one body per entry:\n{c}");
}

#[test]
fn a_mingw_pe_alias_selects_its_function() {
    let image = fixture("pe_imports.exe");
    assert_eq!(selected(&image, "fpreset"), selected(&image, "_fpreset"));
    assert_eq!(selected(&image, "_get_invalid_parameter_handler"), Ok(0x140007850));
}
