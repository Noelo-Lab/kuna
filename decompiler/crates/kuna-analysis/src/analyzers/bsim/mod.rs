//! BSim — function-similarity signatures over the decompiler's feature vectors.
//!
//! FID (the `fid` analyzer beside this one) answers "is this the *same* known
//! library function?" by exact fingerprint. BSim answers the question FID cannot:
//! "which functions in this corpus *look like* this one?". A decompiled function
//! becomes a weighted feature vector ([`lsh`]); two vectors are compared with a
//! cosine similarity plus a significance score that penalises chance agreement.
//!
//! # Layout
//!
//! - [`lshweights`] — the TF/IDF weight tables and the corpus-frequency lookup,
//!   loaded from bundled stock data or an external `lshweights_*.xml` file.
//! - [`lsh`] — the sparse weighted vector, its comparison, and the significance
//!   formula.
//!
//! # Origin (upstream Ghidra)
//!
//! The feature list itself is the decompiler's signature module, already ported
//! as [`kuna_decomp::signature`] (`GraphSigManager`). What this module adds is the
//! `generic/lsh` half: `Ghidra/Framework/Generic/src/main/java/generic/lsh/`.
//!
//! # Faithfulness
//!
//! Similarity and significance are BSim's own numbers, not a re-derivation: the
//! quantities are ports of `LSHCosineVector.compare` and
//! `LSHVectorFactory.calculateSignificance`, so a score kuna reports means what
//! Ghidra's BSim means by it.

pub mod corpus;
pub mod lsh;
pub mod lshweights;
pub mod query;
pub mod signature;
