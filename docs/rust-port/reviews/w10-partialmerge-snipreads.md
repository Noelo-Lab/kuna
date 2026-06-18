# Partial Merge snipReads firstuse placement (w10-partialmerge-snipreads) — ACCEPT, +5

Wave (2026-06-18) integrated at rust-port (merge of `worktree-agent-af60ae034cc29e46a` @ `107cacb`,
base `99deca7`/599). Verdict: **ACCEPT**.

## The fix

- **`do_snip_reads_insert_point` (funcdata.rs)** — the input-Varnode trim COPY now lands at the entry
  block START (`bblocks_block_start`), not STOP, faithful to C++ `Merge::snipReads` (merge.cc:454
  `pc = bl->getStart()`). The call-interfere COPY was landing at block-stop (`0x100713`) instead of
  firstuse (`0x100700`), so `DynamicHash::findVarnode` (dynamic.cc:571) couldn't find it — the real bug
  behind #6/#7/#8/#9.
- **`funcdata_merge.rs`** — `bank_symbol`/`bank_symbol_offset` extended with
  `kuna_mapped_symbol_entry`/`_id`/`_offset`: models `HighVariable::getSymbol()` (variable.cc:418-432)
  for a `map addr` access by resolving the covering Symbol from the scope snapshot
  (`ScopeLocal::query_container_for_link` for the stack frame,
  `Architecture::query_container_global` for the global scope), so `Merge::mergeTestRequired`'s symbol
  guard (merge.cc:157-164) rejects the spurious re-fold (#1).

## Effect

599 → **604** (+5). Gained: Partial Merge #1/#6/#7/#8/#9.

## Gate (at integration onto 599)

- datatests `[675, 604]`; regressed-set **EMPTY** (switch / for-loop / Chain-B / Bitfields /
  Stack-string / Piece-Structure / Concat all unmoved).
- `cargo test --workspace --no-fail-fast` (debug): **0 failures**.
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## Residual

Partial Merge #2/#4/#5 — a DISTINCT loss (LOSS-147/D1): the IR is byte-identical to the C++ oracle, but
it's the addrtied return-register **explicit-vs-inline** render decision (`mark_output_storage_addr_tied`
ScopeLocal `inScope` heuristic) — C++ renders `return a_simple + 10`, rust emits
`v1 = a_simple + 10; return v1`. Corpus-wide render-stage gap, own wave.

## No special-casing

Faithful merge.cc:454 + variable.cc:418 transcription; covering-Symbol keyed by storage address only.
