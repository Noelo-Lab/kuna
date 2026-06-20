# Partial Merge cover-trim — build_single_read_cover addRefPoint (w10-partialmerge) — ACCEPT, +1

Wave (2026-06-18) integrated at rust-port (merge of `worktree-agent-ab9b28b723b0b24d9` @ `da8989b`,
base `34de25e`/588). Verdict: **ACCEPT** (substrate wave, +1; full 9-cluster gated downstream).

## The fix

LOSS-229's "merge cover-trim re-insertion" locus was right but the mechanism was a **dropped
`addRefPoint`**, not a missing feature:
- **`build_single_read_cover` (funcdata.rs:2588)** — added the dropped `single.add_ref_point_for(...)`
  (C++ `Merge::eliminateIntersect`, merge.cc:502-505). Without it the single-read cover was
  degenerate (start==stop at the def), no cover intersection was ever found, and the firstuse COPY was
  never re-inserted. With it, rust's raw pcode for `readpartial` now byte-matches the C++ oracle (the
  `u…:4 = r…:4` firstuse COPY is present and read by the `+ 0xa` INT_ADD).
- **`build_copy_trim_op` (funcdata.rs:2486)** — cover-trim output inherits the `mapped` bit from a
  mapped input (markexplicit isMapped arm, coreaction.cc:3148).
- **`attempt_dynamic_mapping` (funcdata.rs:3308/3445)** + `variable.rs:409`
  (`set_kuna_dynamic_symbol`/`kuna_dynamic_symbol`) + `funcdata_merge.rs:402` (`bank_symbol` wired to
  the high's dynamic/equate symbol for `Merge::mergeTestRequired`) + `coreaction_cleanup.rs:516`
  (`ActionMergeRequired` re-runs idempotent dynamic mapping after cover-separation).

## Effect

588 → **589** (+1). Gained: Partial Merge #10 (`return b_between`).

## Gate (at integration onto 588)

- datatests `[675, 589]`; regressed-set **EMPTY** (switch 37/37, for-loop 25/25, Bitfields 60/60,
  Stack-string 12/12 all unmoved).
- `cargo test --workspace --no-fail-fast` (debug): **3682 / 0**.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## Residual (next-locus for Partial Merge #1/2/4/5/6/7/8/9)

The dynamic temp (`a_simple`) is folded back into the field high (`glob1.a`) by `ActionMergeCopy`'s
`mergeOpcode(CPUI_COPY)` (merge.cc:326). Rust merges because at merge time the `map addr` global field
HighVariable carries no Symbol, so `mergeTestRequired`'s symbol guard (merge.cc:157-164) never fires.
The lever: bind the global Symbol to the field HighVariable before the merge group (C++
`high->getSymbol()` for globals; `seams.rs::query_container_global` already resolves the covering
entry) and surface it through `bank_symbol`, so the symbol guard rejects the re-fold. Deferred to keep
this wave's blast radius limited to the proven cover-trim substrate (binding global symbols onto highs
touches every global access).

## No special-casing

Faithful merge.cc:502 transcription; no address/hash/offset/name hardcoding.
