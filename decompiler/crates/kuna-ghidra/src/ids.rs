//! The ghidra-mode protocol marshaling ids — ports of the `ElementId`
//! tables in `decompiler/cpp/ghidra_arch.cc:30-48` (the query command
//! elements), `decompiler/cpp/ghidra_process.cc:74` (`ELEM_DOC`), and the
//! version/settings subset of `decompiler/cpp/signature.cc:29-40`.
//!
//! These are PROTOCOL ids: the numbers must match upstream Ghidra exactly
//! for wire compatibility (the Java side dispatches queries on the numeric
//! element id, `DecompileProcess.java` query switch).  They therefore use
//! the upstream numbers, NOT the kuna-invented 4000+ range.
//!
//! Cross-checked against the Java tables (`ElementId.java:371-427,433-440`):
//! every number agrees.  One name skew exists: id 255 is named
//! `"command_getstringdata"` in C++ but `"command_getstring"` in Java —
//! only the number travels on the wire, and kuna ports the C++ name.
//!
//! kuna-decomp already owns the neighboring signature ids 258-260, 263,
//! 265-267, 269 (`kuna-decomp/src/infra/signature.rs`) with upstream
//! numbers; this module adds only the ids the process front-end needs and a
//! repo-wide grep confirms none of them are otherwise taken.

use kuna_base::marshal::{AttributeId, ElementId, IdRegistry};

// ---------------------------------------------------------------------------
// ghidra_process.cc
// ---------------------------------------------------------------------------

/// Marshaling element \<doc> — the decompileAt response document
/// (C++ `ELEM_DOC`, `decompiler/cpp/ghidra_process.cc:74`).
pub const ELEM_DOC: ElementId = ElementId::new("doc", 229);

// ---------------------------------------------------------------------------
// op.cc — the p-code emit vocabulary GhidraTranslate::oneInstruction needs
// ---------------------------------------------------------------------------

/// Marshaling element \<unimpl> — the getPcode response for an instruction
/// the host cannot translate (C++ `ELEM_UNIMPL`, `decompiler/cpp/op.cc:22`).
///
/// kuna's `ELEM_INST` (id 98) / `ELEM_OP` (id 27) live in kuna-decomp /
/// kuna-sleigh; only `ELEM_UNIMPL` is missing, and `GhidraTranslate` compares
/// it by numeric id alone (the packed decoder returns raw ids from
/// `open_element`), so it is defined here rather than pulled from another
/// crate.  Numeric id must match upstream for wire compatibility.
pub const ELEM_UNIMPL: ElementId = ElementId::new("unimpl", 114);

// ---------------------------------------------------------------------------
// ghidra_arch.cc:30-48 — the query command elements (239-257)
// ---------------------------------------------------------------------------

/// Query element \<command_isnameused> (`ghidra_arch.cc:30`).
pub const ELEM_COMMAND_ISNAMEUSED: ElementId = ElementId::new("command_isnameused", 239);
/// Query element \<command_getbytes> (`ghidra_arch.cc:31`).
pub const ELEM_COMMAND_GETBYTES: ElementId = ElementId::new("command_getbytes", 240);
/// Query element \<command_getcallfixup> (`ghidra_arch.cc:32`).
pub const ELEM_COMMAND_GETCALLFIXUP: ElementId = ElementId::new("command_getcallfixup", 241);
/// Query element \<command_getcallmech> (`ghidra_arch.cc:33`).
/// (242 = getcallmech, 243 = getcallotherfixup — verified against both
/// `ghidra_arch.cc` and `ElementId.java:380-385`.)
pub const ELEM_COMMAND_GETCALLMECH: ElementId = ElementId::new("command_getcallmech", 242);
/// Query element \<command_getcallotherfixup> (`ghidra_arch.cc:34`).
pub const ELEM_COMMAND_GETCALLOTHERFIXUP: ElementId =
    ElementId::new("command_getcallotherfixup", 243);
/// Query element \<command_getcodelabel> (`ghidra_arch.cc:35`).
pub const ELEM_COMMAND_GETCODELABEL: ElementId = ElementId::new("command_getcodelabel", 244);
/// Query element \<command_getcomments> (`ghidra_arch.cc:36`).
pub const ELEM_COMMAND_GETCOMMENTS: ElementId = ElementId::new("command_getcomments", 245);
/// Query element \<command_getcpoolref> (`ghidra_arch.cc:37`).
pub const ELEM_COMMAND_GETCPOOLREF: ElementId = ElementId::new("command_getcpoolref", 246);
/// Query element \<command_getdatatype> (`ghidra_arch.cc:38`).
pub const ELEM_COMMAND_GETDATATYPE: ElementId = ElementId::new("command_getdatatype", 247);
/// Query element \<command_getexternalref> (`ghidra_arch.cc:39`).
pub const ELEM_COMMAND_GETEXTERNALREF: ElementId = ElementId::new("command_getexternalref", 248);
/// Query element \<command_getmappedsymbols> (`ghidra_arch.cc:40`).
pub const ELEM_COMMAND_GETMAPPEDSYMBOLS: ElementId =
    ElementId::new("command_getmappedsymbols", 249);
/// Query element \<command_getnamespacepath> (`ghidra_arch.cc:41`).
pub const ELEM_COMMAND_GETNAMESPACEPATH: ElementId =
    ElementId::new("command_getnamespacepath", 250);
/// Query element \<command_getpcode> (`ghidra_arch.cc:42`).
pub const ELEM_COMMAND_GETPCODE: ElementId = ElementId::new("command_getpcode", 251);
/// Query element \<command_getpcodeexecutable> (`ghidra_arch.cc:43`).
pub const ELEM_COMMAND_GETPCODEEXECUTABLE: ElementId =
    ElementId::new("command_getpcodeexecutable", 252);
/// Query element \<command_getregister> (`ghidra_arch.cc:44`).
pub const ELEM_COMMAND_GETREGISTER: ElementId = ElementId::new("command_getregister", 253);
/// Query element \<command_getregistername> (`ghidra_arch.cc:45`).
pub const ELEM_COMMAND_GETREGISTERNAME: ElementId = ElementId::new("command_getregistername", 254);
/// Query element \<command_getstringdata> (`ghidra_arch.cc:46`; the Java
/// table names id 255 `"command_getstring"` — same number, C++ name kept).
pub const ELEM_COMMAND_GETSTRINGDATA: ElementId = ElementId::new("command_getstringdata", 255);
/// Query element \<command_gettrackedregisters> (`ghidra_arch.cc:47`).
pub const ELEM_COMMAND_GETTRACKEDREGISTERS: ElementId =
    ElementId::new("command_gettrackedregisters", 256);
/// Query element \<command_getuseropname> (`ghidra_arch.cc:48`).
pub const ELEM_COMMAND_GETUSEROPNAME: ElementId = ElementId::new("command_getuseropname", 257);

// ---------------------------------------------------------------------------
// signature.cc:29-40 — the version/settings subset
// ---------------------------------------------------------------------------
// The signature commands themselves are not implemented in phase 1 (the
// unknown-command "Bad command" response is the documented degradation
// path), but the ids are pinned here so the phase-2+ getSignatureSettings
// wire-glue reuses them.  kuna-decomp/src/infra/signature.rs owns the other
// eight ids of the 258-269 block with matching upstream numbers.

/// Marshaling element \<major> (C++ `ELEM_MAJOR`, `signature.cc:33`).
pub const ELEM_MAJOR: ElementId = ElementId::new("major", 261);
/// Marshaling element \<minor> (C++ `ELEM_MINOR`, `signature.cc:34`).
pub const ELEM_MINOR: ElementId = ElementId::new("minor", 262);
/// Marshaling element \<settings> (C++ `ELEM_SETTINGS`, `signature.cc:36`).
pub const ELEM_SETTINGS: ElementId = ElementId::new("settings", 264);
/// Marshaling element \<sigsettings> (C++ `ELEM_SIGSETTINGS`,
/// `signature.cc:39`).
pub const ELEM_SIGSETTINGS: ElementId = ElementId::new("sigsettings", 268);

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------
// Every attribute the query encodings need is already a kuna-base id
// (kuna-base/src/marshal.rs: ATTRIB_NAME=14, ATTRIB_INDEX=10, ATTRIB_TYPE=22,
// ATTRIB_ID=9, ATTRIB_SIZE=19, ATTRIB_CONTENT=1, ATTRIB_OFFSET=16,
// ATTRIB_BIGENDIAN=3, ATTRIB_SPACE=20; kuna-base/src/address.rs:
// ATTRIB_FIRST=27, ATTRIB_LAST=28; kuna-sleigh/src/translate.rs:
// ATTRIB_UNIQBASE=46) — with ONE exception:

/// Marshaling attribute "maxsize" (upstream `fspec.cc:27`, id 120), used by
/// the getStringData query.  kuna's port of this id lives in
/// `kuna-decomp/src/s4_calls/modelrules.rs:99`, which phase 1 deliberately
/// does not depend on; the constant is duplicated here with the same
/// upstream number.  TODO(phase-2): drop this duplicate once kuna-ghidra
/// links against kuna-decomp for the engine bridge.
pub const ATTRIB_MAXSIZE: AttributeId = AttributeId::new("maxsize", 120);

/// Register every ghidra-mode protocol id on the given registry (the
/// explicit stand-in for the C++ static-object registration; follow-up to
/// [`IdRegistry::with_base_ids`] + `register_translate_ids`, the same
/// per-module pattern as `kuna-console`'s `build_registry`).
pub fn register_ghidra_ids(registry: &mut IdRegistry) {
    registry.register_attribute(&ATTRIB_MAXSIZE);
    registry.register_element(&ELEM_DOC);
    registry.register_element(&ELEM_COMMAND_ISNAMEUSED);
    registry.register_element(&ELEM_COMMAND_GETBYTES);
    registry.register_element(&ELEM_COMMAND_GETCALLFIXUP);
    registry.register_element(&ELEM_COMMAND_GETCALLMECH);
    registry.register_element(&ELEM_COMMAND_GETCALLOTHERFIXUP);
    registry.register_element(&ELEM_COMMAND_GETCODELABEL);
    registry.register_element(&ELEM_COMMAND_GETCOMMENTS);
    registry.register_element(&ELEM_COMMAND_GETCPOOLREF);
    registry.register_element(&ELEM_COMMAND_GETDATATYPE);
    registry.register_element(&ELEM_COMMAND_GETEXTERNALREF);
    registry.register_element(&ELEM_COMMAND_GETMAPPEDSYMBOLS);
    registry.register_element(&ELEM_COMMAND_GETNAMESPACEPATH);
    registry.register_element(&ELEM_COMMAND_GETPCODE);
    registry.register_element(&ELEM_COMMAND_GETPCODEEXECUTABLE);
    registry.register_element(&ELEM_COMMAND_GETREGISTER);
    registry.register_element(&ELEM_COMMAND_GETREGISTERNAME);
    registry.register_element(&ELEM_COMMAND_GETSTRINGDATA);
    registry.register_element(&ELEM_COMMAND_GETTRACKEDREGISTERS);
    registry.register_element(&ELEM_COMMAND_GETUSEROPNAME);
    registry.register_element(&ELEM_MAJOR);
    registry.register_element(&ELEM_MINOR);
    registry.register_element(&ELEM_SETTINGS);
    registry.register_element(&ELEM_SIGSETTINGS);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The verified upstream numbers (`ghidra_arch.cc:30-48` ==
    /// `ElementId.java:371-427`; `signature.cc:29-40` == `ElementId.java:433-440`).
    #[test]
    fn test_protocol_id_numbers_match_upstream() {
        assert_eq!(ELEM_DOC.get_id(), 229);
        assert_eq!(ELEM_UNIMPL.get_id(), 114);
        assert_eq!(ELEM_COMMAND_ISNAMEUSED.get_id(), 239);
        assert_eq!(ELEM_COMMAND_GETBYTES.get_id(), 240);
        assert_eq!(ELEM_COMMAND_GETCALLFIXUP.get_id(), 241);
        assert_eq!(ELEM_COMMAND_GETCALLMECH.get_id(), 242);
        assert_eq!(ELEM_COMMAND_GETCALLOTHERFIXUP.get_id(), 243);
        assert_eq!(ELEM_COMMAND_GETCODELABEL.get_id(), 244);
        assert_eq!(ELEM_COMMAND_GETCOMMENTS.get_id(), 245);
        assert_eq!(ELEM_COMMAND_GETCPOOLREF.get_id(), 246);
        assert_eq!(ELEM_COMMAND_GETDATATYPE.get_id(), 247);
        assert_eq!(ELEM_COMMAND_GETEXTERNALREF.get_id(), 248);
        assert_eq!(ELEM_COMMAND_GETMAPPEDSYMBOLS.get_id(), 249);
        assert_eq!(ELEM_COMMAND_GETNAMESPACEPATH.get_id(), 250);
        assert_eq!(ELEM_COMMAND_GETPCODE.get_id(), 251);
        assert_eq!(ELEM_COMMAND_GETPCODEEXECUTABLE.get_id(), 252);
        assert_eq!(ELEM_COMMAND_GETREGISTER.get_id(), 253);
        assert_eq!(ELEM_COMMAND_GETREGISTERNAME.get_id(), 254);
        assert_eq!(ELEM_COMMAND_GETSTRINGDATA.get_id(), 255);
        assert_eq!(ELEM_COMMAND_GETTRACKEDREGISTERS.get_id(), 256);
        assert_eq!(ELEM_COMMAND_GETUSEROPNAME.get_id(), 257);
        assert_eq!(ELEM_MAJOR.get_id(), 261);
        assert_eq!(ELEM_MINOR.get_id(), 262);
        assert_eq!(ELEM_SETTINGS.get_id(), 264);
        assert_eq!(ELEM_SIGSETTINGS.get_id(), 268);
        assert_eq!(ATTRIB_MAXSIZE.get_id(), 120);
    }
}
