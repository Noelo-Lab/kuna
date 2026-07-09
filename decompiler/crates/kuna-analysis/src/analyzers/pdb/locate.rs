//! Locate + fingerprint-gate the external `.pdb` for a PE.
//!
//! PDB is the lone **external-file** case in the metadata tier: the PE carries only
//! a CodeView *fingerprint* (a GUID/sig + age + the `.pdb` path — see
//! [`crate::pdb::codeview`]), never the debug info itself. So PDB recovery is
//! conditional on the user *supplying* the `.pdb` — the exact shape of the FID
//! database (which needs a `.fid` DB supplied out of band), and modeled on the
//! `fid` `kuna_fid_db` env-var precedent.
//!
//! ## Tier-1 (the MVP, this PR)
//!
//! The `.pdb` path is read from the **`kuna_pdb_path` environment variable** — the
//! exact `fid::FID_DB_ENV` (`kuna_fid_db`) precedent. This is the simplest
//! locate strategy and keeps the LLM control surface to a SINGLE new on|off
//! settable (`--option pdb`): the path source lives off the per-decompilation
//! `--option` surface (the same posture FID's DB source takes). Tier-2 (same-dir /
//! local-store sidecar) and tier-3 (symbol-server download) are deferred follow-ons
//! (design §4.2).
//!
//! ## The fingerprint gate (the FID full-hash-match discipline)
//!
//! A supplied `.pdb` is applied **only** when its own `pdb_information().guid/age`
//! matches the PE's CodeView record. A MISMATCH (a stale / wrong / unrelated `.pdb`)
//! or an ABSENT/unreadable file yields `None` → the pass emits nothing. This is the
//! exact "never apply the wrong external knowledge" discipline FID's full-hash
//! bucket-collapse enforces: a wrong PDB would rename functions to *someone else's*
//! names, far worse than leaving them `FUN_*`.

use std::path::PathBuf;

use crate::pdb::codeview::CodeViewInfo;

/// The environment variable naming the external `.pdb` file to apply. Mirrors the
/// `fid` `kuna_fid_db` env gate exactly (the FID-database external-artifact
/// precedent): the `.pdb` source is kept OFF the per-decompilation `--option`
/// surface in tier-1 (so the catalog gains a single on|off `pdb` settable, not a
/// path-valued one). Absent / empty / unreadable ⇒ the pass is inert.
pub const PDB_PATH_ENV: &str = "kuna_pdb_path";

/// Resolve the tier-1 `.pdb` path from [`PDB_PATH_ENV`]. `None` (the inert path)
/// when the variable is unset or empty. The fingerprint gate ([`fingerprint_ok`])
/// is applied separately, after the `.pdb` is opened.
pub fn pdb_path_from_env() -> Option<PathBuf> {
    let p = std::env::var_os(PDB_PATH_ENV)?;
    if p.is_empty() {
        return None;
    }
    Some(PathBuf::from(p))
}

/// The fingerprint gate: does a supplied `.pdb`'s own `{guid, age}` match the PE's
/// CodeView record [`cv`]?
///
/// Compares (1) the **age** (exact `u32` equality) and (2) the **GUID** in its
/// canonical Microsoft text form. The CodeView record's
/// [`CodeViewInfo::guid_string`] renders the raw 16 mixed-endian bytes
/// (`data1/2/3` little-endian, `data4` byte-order) to `XXXXXXXX-XXXX-XXXX-XXXX-…`;
/// the `pdb` crate's `Uuid` renders the SAME canonical hyphenated form via
/// `Display` (the PDB info stream stores the GUID in the identical on-disk
/// mixed-endian layout). Both are upper-cased before comparison so the match is
/// case-insensitive (the `uuid` crate lower-cases; `guid_string` upper-cases).
///
/// An **NB10** CodeView record (a `u32` signature, no GUID — [`CodeViewInfo::Nb10`])
/// has no GUID to compare; the modern `RSDS`/`pdb`-crate path is GUID-keyed, so a
/// `pdb` opened against an NB10 record cannot be fingerprint-matched at this tier
/// and is rejected (returns `false`). NB10 binaries are rare in practice; tier-1
/// targets the modern RSDS form the fixture uses.
///
/// `pdb_guid` is the `.pdb`'s `pdb_information().guid` rendered to its canonical
/// string (the caller passes `info.guid.to_string()`); `pdb_age` is
/// `pdb_information().age`.
pub fn fingerprint_ok(cv: &CodeViewInfo, pdb_guid: &str, pdb_age: u32) -> bool {
    // The age must match exactly (a rebuilt PDB bumps the age even if the GUID is
    // unchanged in some toolchains — but here both must agree with the record).
    if cv.age() != pdb_age {
        return false;
    }
    // The GUID must match (case-insensitively, canonical hyphenated form). NB10 has
    // no GUID ⇒ `guid_string()` is `None` ⇒ reject (no GUID-keyed match possible).
    match cv.guid_string() {
        Some(record_guid) => record_guid.eq_ignore_ascii_case(pdb_guid),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rsds() -> CodeViewInfo {
        // The pdb_min.exe / pdb_prog GUID raw bytes (Microsoft mixed-endian); its
        // canonical text form is "61AC3963-FF48-9024-4C4C-44205044422E".
        CodeViewInfo::Rsds {
            guid: [
                0x63, 0x39, 0xAC, 0x61, 0x48, 0xFF, 0x24, 0x90, 0x4C, 0x4C, 0x44, 0x20, 0x50, 0x44,
                0x42, 0x2E,
            ],
            age: 1,
            pdb_path: "x.pdb".to_string(),
        }
    }

    #[test]
    fn matching_guid_and_age_passes() {
        let cv = rsds();
        // The canonical render (uppercase) and a lowercased variant both match.
        assert!(fingerprint_ok(&cv, "61AC3963-FF48-9024-4C4C-44205044422E", 1));
        assert!(fingerprint_ok(&cv, "61ac3963-ff48-9024-4c4c-44205044422e", 1));
    }

    #[test]
    fn mismatched_guid_fails() {
        let cv = rsds();
        // A different GUID (one nibble off) is rejected — the stale-PDB guard.
        assert!(!fingerprint_ok(&cv, "00000000-FF48-9024-4C4C-44205044422E", 1));
    }

    #[test]
    fn mismatched_age_fails() {
        let cv = rsds();
        // Right GUID, wrong age (a rebuilt PDB) is rejected.
        assert!(!fingerprint_ok(&cv, "61AC3963-FF48-9024-4C4C-44205044422E", 2));
    }

    #[test]
    fn nb10_record_has_no_guid_to_match() {
        // An NB10 record carries a u32 signature, not a GUID — the GUID-keyed gate
        // cannot match it, so it is rejected (tier-1 targets RSDS).
        let cv = CodeViewInfo::Nb10 { signature: 0xDEAD_BEEF, age: 1, pdb_path: "x.pdb".into() };
        assert!(!fingerprint_ok(&cv, "61AC3963-FF48-9024-4C4C-44205044422E", 1));
    }

    #[test]
    fn env_unset_or_empty_is_none() {
        // (Serial-ish: tolerate whatever the ambient env holds by setting + clearing.)
        std::env::remove_var(PDB_PATH_ENV);
        assert!(pdb_path_from_env().is_none());
        std::env::set_var(PDB_PATH_ENV, "");
        assert!(pdb_path_from_env().is_none());
        std::env::set_var(PDB_PATH_ENV, "/some/where.pdb");
        assert_eq!(pdb_path_from_env(), Some(PathBuf::from("/some/where.pdb")));
        std::env::remove_var(PDB_PATH_ENV);
    }
}
