//! (GH #1349) A Mach-O dylib stripped of its symbol table (`strip -N`) keeps
//! its export trie, and the trie's names are the image's own: `functions` lists
//! them, `decompile` selects by them in either spelling, and a call to one reads
//! by name, as on the unstripped dylib.
//!
//! Fixtures: `macho_trie_exports_arm64` (chained fixups, `LC_DYLD_EXPORTS_TRIE`),
//! `macho_trie_exports_info_arm64` (the classic `LC_DYLD_INFO_ONLY` trie and no
//! `LC_FUNCTION_STARTS`) and `macho_trie_partial_arm64` (an `nlist` that keeps
//! one of three exports). Integration tests require the built processor specs
//! under `specs/`.

use std::path::PathBuf;
use std::process::{Command, Output};

const FIXTURES: [&str; 2] = ["macho_trie_exports_arm64", "macho_trie_exports_info_arm64"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
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

fn fixture(name: &str) -> String {
    let path = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    path.to_str().unwrap().to_string()
}

#[test]
fn trie_names_list_and_select_on_a_stripped_dylib() {
    for name in FIXTURES {
        let image = fixture(name);
        let out = kuna(&["functions", &image]);
        let listing = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{name}: {}", String::from_utf8_lossy(&out.stderr));
        let rows: Vec<(&str, &str)> = listing
            .lines()
            .filter_map(|l| {
                let mut cols = l.split('\t');
                Some((cols.next()?, cols.next()?))
            })
            .collect();
        assert_eq!(
            rows,
            [("0x328", "_audit_exported_function"), ("0x340", "_audit_caller")],
            "{name}:\n{listing}"
        );

        for selector in ["audit_exported_function", "_audit_exported_function"] {
            let out = kuna(&["decompile", &image, selector]);
            let c = String::from_utf8_lossy(&out.stdout);
            assert!(out.status.success(), "{name} {selector}: {}", String::from_utf8_lossy(&out.stderr));
            assert!(c.contains("return a0 + 7;"), "{name} {selector}:\n{c}");
        }

        let out = kuna(&["decompile", &image, "audit_caller"]);
        let c = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{name}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(
            c.contains("return _audit_exported_function(a0) + _audit_exported_data;"),
            "{name}: the callee and the global read by name:\n{c}"
        );

        let out = kuna(&["decompile", &image, "audit_exported_data"]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(!out.status.success(), "{name}: a data export is not a function");
        assert!(stderr.contains("it names data at 0x4000 in __data"), "{name}:\n{stderr}");
    }
}

/// A symbol table that keeps one of three exports: the trie names the entry it
/// lost, and the second name at the entry it kept selects that entry.
#[test]
fn trie_names_fill_a_partial_symbol_table() {
    let image = fixture("macho_trie_partial_arm64");
    let out = kuna(&["functions", &image]);
    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(listing.contains("0x240\t_audit_export_a\t(sub_240, _audit_export_b)"), "{listing}");
    assert!(listing.contains("0x248\t_audit_caller\t(sub_248)"), "{listing}");
    for selector in ["_audit_export_b", "audit_export_b"] {
        let out = kuna(&["decompile", &image, selector]);
        let c = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{selector}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(c.contains("return a0 + 7;"), "{selector}:\n{c}");
    }
}
