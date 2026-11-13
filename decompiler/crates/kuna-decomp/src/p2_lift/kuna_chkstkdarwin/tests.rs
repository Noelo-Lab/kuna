//! Unit tests for the Darwin stack-probe gates: the probe's two symbol spellings
//! and the AArch64-only language test.

use super::*;

#[test]
fn both_probe_spellings_match() {
    assert!(is_probe("__chkstk_darwin"));
    assert!(is_probe("___chkstk_darwin"));
}

#[test]
fn other_probes_and_prefixes_do_not_match() {
    for name in ["__chkstk", "_chkstk_darwin", "chkstk_darwin", "__chkstk_darwin_probe", "___chkstk_darwin2", ""] {
        assert!(!is_probe(name), "{name} must not match");
    }
}

#[test]
fn only_aarch64_languages_apply() {
    assert!(applies_to("AARCH64:LE:64:v8A:default"));
    assert!(applies_to("AARCH64:LE:64:AppleSilicon:default"));
    assert!(applies_to("AARCH64:LE:32:ilp32"));
    assert!(!applies_to("x86:LE:64:default:gcc"));
    assert!(!applies_to("ARM:LE:32:v8"));
    assert!(!applies_to(""));
}
