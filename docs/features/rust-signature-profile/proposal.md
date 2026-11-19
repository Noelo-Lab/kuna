# Rust signature comparison on a function summary

Add `--rust-signature-manifest PATH` to `kuna functions --summary --json`.
The manifest supplies an explicit candidate set, corpus identity and pin,
and an inline and no-inline FLIRT bank for each candidate. Verify each
listed file against its SHA-256 before analysis. Compare each bank once against
the accepted function entries of a linked x86-64 ELF image.

Expose `compiler_provenance` and `rust_signature_profile` in the requested
JSON report. Keep evidence for each bank and give distinct outcomes for a
unique winner, ties, missing banks, and unsupported inputs. Require a unique
winner with a complete candidate set and nonzero match evidence. Use the match results exclusively as diagnostic evidence in the report.

Verify the three pinned optimization and compiler cases; also test tied scores,
missing banks, malformed manifests and unmodified-image validation. Replay
ordinary function summaries byte for byte. Measure the time spent on an
explicitly requested comparison.
