//! (kuna `gotypes`) The Go runtime-type harvest — walk a Go binary's own
//! `moduledata → typelinks → _type` metadata and intern NAMED struct shells
//! (`main.Point`, `os.File`, …) into the type factory, phase 1 of the
//! `docs/multilang.md` W3 Go provider. The pass itself lives in
//! `kuna-analysis::analyzers::gotypes`; this module holds the bound it shares.

/// The most typelink entries one image's walk will process. Real Go binaries
/// carry 1,500–4,000 (`vet`, the 6 MB tool, has ~3,400); the ceiling only
/// exists so a corrupt length cannot make the load path spin. Generous rather
/// than tight: every skipped entry is a named type the user never sees.
pub const GOTYPES_MAX_TYPELINKS: usize = 20_000;
