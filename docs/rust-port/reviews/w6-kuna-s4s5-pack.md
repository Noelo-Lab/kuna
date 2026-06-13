# w6-kuna-s4s5-pack
verdict: REJECT
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
round: 1

gate: `cd rust && cargo test -p kuna-decomp`
  -> lib: 1102 passed / 0 failed; catalog_bytecompat: 5/5; corpus_bootstrap:
     4/5 (1 FAILED). The single failure `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
     is ENVIRONMENTAL: it panics at `build_spec_file(x86:LE:64:default) failed:
     Could not find .sla file` because the worktree has no compiled `x86-64.sla`
     (a gitignored build artifact; the main tree has it built). The test file is
     not touched by this diff, exercises sleigh/spec-file resolution, and touches
     NONE of the three in-scope kuna modules. All in-scope tests (including the
     new adversarial suite, 3/3) pass.
clippy: `cargo clippy -p kuna-decomp` clean (no warnings).
port-audit: cpp_blob_sha — all six in-scope blobs match the checklist record
     (no drift): kuna_returnpair.cc 8fc8169…, .hh 26468a9…, kuna_memsetsequence.cc
     3bc1238…, .hh e04810c…, kuna_compareform.cc 3c06286…, .hh cea69df…

## Scope
In-scope C++ (kuna-owned): kuna_returnpair.{cc,hh}, kuna_memsetsequence.{cc,hh},
kuna_compareform.{cc,hh}.  Rust: kuna_returnpair.rs, kuna_memsetsequence.rs,
kuna_compareform.rs (+ their `tests.rs`).  The fspec/dtype/constseq/prefersplit/
space changes in the branch diff belong to OTHER items merged onto this branch
and are out of scope here.

## Hunt list
- Signedness: clean. `restore_lessequal` keeps `intb diff` (i64) as C++ `intb`;
  width-correct `calc_int_min/max`, `calc_uint_max` boundary compares. `form_fill_run`
  low-byte compares are `u8`. (T1/T2 exercise the int-max/int-min/uint-max edges.)
- Integer widths: clean. int4/uintb mappings consistent; `(expect - run_start) as
  int4` matches the C++ `(int4)(expect-runStart)` truncation; `diff as u64` for
  -1 reproduces the C++ `intb`->`uintb` promotion to all-ones.
- Wrapping: F2 (minor). `form_fill_run` (kuna_memsetsequence.rs:235) does plain
  `move_ops[i].offset + move_ops[i].slot as u64` where the C++ `uint8 + uint8`
  wraps; debug Rust panics. Only reachable with offsets near u64::MAX, which the
  real (deferred) array-bounded `collectFillRun` path cannot produce — but it
  diverges from the C++ wrap. `compareform` `val.wrapping_add(diff as u64)` is
  correct. `expect - run_start` cannot underflow (`expect >= run_start` invariant
  once last_idx>=1), so plain `-` there is fine.
- Comparator totality: clean. `move_ops.sort_by(|a,b| a.offset.cmp(&b.offset))` is
  a total order on u64. The C++ uses unstable `std::sort`; Rust `sort_by` is stable
  — they can only differ on DUPLICATE offsets, which the contiguity check
  (`offset != expect` -> break) rejects after the first; Rust is more deterministic,
  not wrong. (Observation, not a finding.)
- Iteration-order provenance: clean. `ActionPresentCompareForm::apply` snapshots
  `data.obank().iter_alive()` (the intrusive `alivelist` link order == C++
  `beginOpAlive()..endOpAlive()`); `restore_lessequal` never inserts/removes ops,
  so the snapshot is order-equivalent. No HashMap/HashSet anywhere in the three files.
- Off-by-one / do-while / reverse iter: clean. `keep_single_return` size>2 boundary
  is exact (T3 pins size 2 vs 3). `form_fill_run` `last_idx < 1` (require >=2 COPYs)
  and `total_bytes < 16` transcribed verbatim; no do-while/reverse-iterator idioms
  in scope.
- Erase-while-iterating: clean. compareform mutates ops in place (opcode + one
  input), never erases. `form_fill_run` `truncate(keep)` faithfully reproduces
  `moveOps.resize(lastIdx+1, ...)` (it only ever shrinks; the C++ fill WriteNode is
  never read after).
- Exception -> Result parity: F1 (major, below) + minor. `parse_return_pair_form`
  maps the C++ NON-throwing error-string return to `KunaError::parse` (documented,
  acceptable idiom). `OptionMemsetRecover::apply`'s `onOrOff` throw is faithfully
  re-wrapped. **`OptionCompareForm::apply`'s `throw ParseError(...)` has NO Rust
  port at all** — see F1.

## Findings
  - F1 (MAJOR): `OptionCompareForm::apply` is not ported anywhere in the crate.
       The C++ file `kuna_compareform.cc` contains the option-apply body that
       parses `original`/`canonical`, sets `glb->present_lessequal`, and throws
       `ParseError("Must specify compareform as 'canonical' or 'original'")` on
       bad input. The two sibling files in THIS pack port their exact analogue —
       `kuna_returnpair.rs::parse_return_pair_form` (OptionReturnPair::apply) and
       `kuna_memsetsequence.rs::parse_memset_recover_form` (OptionMemsetRecover::apply)
       — and `options.rs:232-237` documents that each kuna ArchOption's `apply`
       impl "lives in the `kuna_*.rs` modules" for `w4-kuna-p0-pack` to wire. The
       compareform parse body is silently absent (grep for the option's literal
       error message "Must specify compareform as" returns nothing crate-wide; no
       `parse_compare_form`/`OptionCompareForm`/`CompareForm` setting type exists),
       and it is NOT recorded as a loss. This is an unported in-scope C++ function
       with zero seam dependency (the siblings prove portability now), breaking the
       pack's own convention; the diff does not stand on its own against the C++
       source.
       cpp: decompiler/cpp/kuna_compareform.cc:85-95 (OptionCompareForm::apply)
       rust: rust/crates/kuna-decomp/src/kuna_compareform.rs (no counterpart;
              cf. kuna_returnpair.rs:102 parse_return_pair_form,
              kuna_memsetsequence.rs:397 parse_memset_recover_form)
       Work order: port `OptionCompareForm::apply` as `parse_compare_form(p1) ->
       KunaResult<(CompareForm, String)>` mirroring `parse_return_pair_form`
       (original => present_lessequal=true, canonical => false, else
       `KunaError::parse("Must specify compareform as 'canonical' or 'original'")`,
       confirmation "Comparison presentation set to <p1> form"). OR ledger it as an
       explicit loss with a concrete reason it is deferred while the siblings are not.

  - F2 (MINOR): possible debug-panic where C++ silently wraps.
       `form_fill_run` computes `expect = move_ops[i].offset + move_ops[i].slot as
       u64` with a plain `+`; the C++ `uint8 expect = moveOps[i].offset +
       moveOps[i].slot` wraps on overflow. The merged constseq base uses
       `wrapping_add`/`wrapping_sub` for exactly these uintb-lineage sums (e.g.
       constseq.rs:312). Only reachable via the seam bridge `from_collected` with
       adversarial offsets near u64::MAX; the real (deferred) `collectFillRun`
       confines offsets to within `arrayType->getSize()` so neither C++ nor the
       real Rust path can overflow. Recommend `wrapping_add` for parity with the
       C++ wrap and the constseq base.
       cpp: decompiler/cpp/kuna_memsetsequence.cc:116
       rust: rust/crates/kuna-decomp/src/kuna_memsetsequence.rs:235

## Notes (verified clean, for the record)
- `restore_lessequal` is the exact inverse of `Funcdata::replaceLessequal`
  (funcdata_op.cc:1029-1071): diff signs, operand slots, the four overflow guards
  (signed int_min/int_max, unsigned 0/uint_max), opSetOpcode-before-opSetInput
  ordering, and `clearCanonicalLessequal` only on success — all transcribed and
  round-trip-verified (T1) and edge-decline-verified (T2).
- `lessequal_type_op` installs `binary | booloutput` opflags for both restored
  ops, matching TypeOpIntLessEqual / TypeOpIntSlessEqual (W6 seam, documented).
- The seam-blocked memset halves (`collect_fill_run`, `build_memset`, `transform`,
  the `RuleMemsetCopy::apply_op` body) decline to 0 == option-off behavior, are
  documented stubs, and are correctly ledgered as losses in the structured output.
  The ported, output-determining `form_fill_run` is faithful.

## Adversarial tests added
(rust/crates/kuna-decomp/tests/verify_w6_kuna_s4s5_pack.rs — committed; all pass)
  - t1_signed_roundtrip_at_intmax_boundary_4_and_8 — full replace_lessequal ->
    ActionPresentCompareForm round trip at `V s<= INT_MAX-1`, widths 4 and 8.
  - t2_restore_guards_decline_and_keep_mark — the three genuinely-declining edges
    (unsigned right==0, signed right==INT_MIN, unsigned left==UINT_MAX) leave the
    op untouched AND keep the provenance mark.
  - t3_returnpair_gate_boundary_and_exact_token_parse — keep_single_return size>2
    boundary (2 vs 3, up to usize::MAX), pair-never-truncates, and exact-token /
    no-case-fold parse for both returnpair and memsetrecover.

These tests pass because the *ported* logic is faithful; the REJECT is for the
omission F1 (concrete divergence trace against the C++ oracle, above), not a
numerical bug.

losses: n/a (REJECT — no losses appended)
