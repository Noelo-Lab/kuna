# structsynth `nest` plan

1. Measure the opportunity before designing (temporary diagnostic, removed):
   which pointer fields of an accepted parameter are loaded and dereferenced, and
   which parameter rule each would hit. Done: 274 fields over 12 builds, 10 pass
   the strict rules.
2. `structsynth nest`, a value of the existing option (not a new option): the
   parameter pass unchanged, plus `kuna_structsynth/nest.rs` over the merged
   evidence of each pointer field's loads, depth-bounded at two records.
3. Self-reference by claim subset after the standalone rules; minted around the
   shell in `ledger::mint`, keyed `SELF`; the shell resolved at the LOAD/STORE
   value-type choke points (`resolve_self_pointer`).
4. `--jobs`: a request with a nested or self field is non-portable, so the
   existing one-ordered-worker fallback names it; no protocol change.
5. Instrument: fix `structscore.header_layouts` so a pointer to a later or the
   enclosing struct counts as a pointee.
6. Evidence: nesting census vs DWARF, layout P/R, TRex, 444-slice typesweep,
   whole-corpus hunks on 15 binaries incl. ARM32, `offsetof` round trip and a
   whole-project syntax check, interleaved min-of-15 speed.
7. Default: flip `param` -> `nest` only if the default-flip procedure passes on
   this base; otherwise ship opt-in and commit `default-on-evaluation.md`.
