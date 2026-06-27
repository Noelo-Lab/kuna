//! `s1_pdb` — the PE PDB (Program Database) metadata tier.
//!
//! PDB is **Windows' DWARF**: function names + addresses, full types, params +
//! typed stack locals, and source lines — except the debug info lives in an
//! *external* `.pdb` file, not in the PE itself. The PE carries only a CodeView
//! *fingerprint* (a GUID/signature + age + the `.pdb` path) in its debug
//! directory; that fingerprint is what links a stripped PE back to its `.pdb`.
//!
//! ## Layout (incremental, per `docs/analysis-port-frontier.md`)
//!
//! - [`codeview`] — **PR-P0 (this increment)**: the PE CodeView debug-record
//!   extractor (`PdbInfoCodeView`/`PdbInfoDotNet` port). Decodes the RSDS/NB10
//!   record into [`codeview::CodeViewInfo`]. A pure extractor: no analysis pass,
//!   no `--option`, no `pdb` crate — so it cannot perturb the parity oracles.
//!
//! Future (PR-P1+): the `pdb`-crate-backed `.pdb`-consuming `AnalysisPass` (gated
//! `--option pdb` + `--option pdb_path`, the FID-database external-artifact
//! shape), the `TypeData -> Datatype` mapper, and the symbol/scope walk — all
//! through the proven `s1_dwarf` commit path. The extractor here is what that
//! pass calls to learn *which* `.pdb` to open and *which* GUID/age to gate on.

pub mod codeview;
