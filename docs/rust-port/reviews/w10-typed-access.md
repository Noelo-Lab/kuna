# w10-typed-access
verdict: REJECT
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 1

gate: `git diff rust-port..rport/w10-typed-access -- rust/` -> **EMPTY** (no
      rust/ files changed); `git rev-list --count rust-port..rport/w10-typed-access`
      -> **0** unique commits; branch reflog -> `branch: Created from rust-port`
      (no further entries). `cargo test --workspace` -> all green (0 failed),
      but the green is **rust-port's**, not this item's — the branch contributes
      no source. C++ oracle untouched.

## Why REJECT: the branch is empty — there is no port to verify

`rport/w10-typed-access` (HEAD `cd44e73`) is the **merge-base** of `rust-port`;
`rust-port` (`229c1c6`) is strictly *ahead* of it by one PROGRESS-only commit.

Hard evidence (all reproducible from the MAIN tree):

- `git rev-list --count rust-port..rport/w10-typed-access` = **0**
  (zero commits unique to the branch).
- `git diff rust-port..rport/w10-typed-access -- rust/` = **empty**
  (no `rust/` file added, modified, or deleted).
- `git diff rust-port..rport/w10-typed-access` = only `PROGRESS.md`, and that
  is `rust-port` being *ahead* (it deletes 31 lines the branch never had),
  not the branch contributing anything.
- `git reflog show rport/w10-typed-access` = a single entry
  `cd44e73 ... branch: Created from rust-port`. The branch was created and
  **never received a commit**.
- The worktree `/tmp/w10-typed-access` is `nothing to commit, working tree
  clean` — no uncommitted or untracked port either.
- No `tests/verify_w10_typed_access.rs` existed on the branch (the porter never
  produced one; this verdict adds one — see below).

The in-scope typed-access machinery the item names — union field lookup /
`TypeField` / bitfield (`type.cc`), SUBPIECE/PIECE/INT_AND `propagateType` and
`resolveInFlow` (`typeop.cc`), `ScoreUnionFields`/`resolveInFlow`
(`unionresolve.cc`), and the `.name`/`->name`/bitfield/`(floatN)`/union-facet
rendering (`printc.cc`) — **already exists and was already verified** in the
W6/W8 items it was lifted from:
`w6-s5-type-2`, `w6-s5-typeop`, `w6-s5-unionresolve`, `w6-s5-bitfield`,
`w8-s9-printc` (all `verified` in `docs/rust-port/checklist.json`). Those
sources are present at the base commit
(`rust/crates/kuna-decomp/src/{unionresolve.rs,dtype.rs,coreaction_infertypes.rs,printc.rs}`).
This branch adds **nothing on top of them**.

There is therefore no closed seam, no transcription to assess for faithfulness,
no special-casing to find, and — critically — **no new datatest pass that this
item may claim.** Any "+N parity" credited to the W10 fleet (the rust-port
PROGRESS commit `229c1c6` says the fleet reached 59/549) must come from the
*other* w10 branches that actually have commits (e.g. `w10-jts-chain`); it
cannot be attributed to `w10-typed-access`, which is inert.

A submission that delivers zero lines cannot be ACCEPTed (nothing faithful was
produced) and cannot be ACCEPT-WITH-LOSSES (a loss is an accepted divergence in
*real* code; there is no code). It is REJECTed and returned to `todo` with the
work order: actually port the typed-access seam (or, if the capability is in
fact fully subsumed by the verified W6/W8 items, close the item as a no-op with
that justification recorded — but that decision belongs to the porter/human,
not to a silently-empty branch presented for review).

## The mandatory hunt list

Not applicable — there is no diff. Every entry is vacuously clean because the
branch introduces no signed/unsigned comparison, no integer-width mapping, no
wrapping arithmetic, no comparator, no iteration-order-bearing loop, no
do-while/reverse-iterator, no erase-while-iterating, and no exception→Result
seam. (The pre-existing in-scope code these would apply to was reviewed under
the W6/W8 verdicts.)

- Signedness: n/a (no diff)
- Integer widths: n/a (no diff)
- Wrapping: n/a (no diff)
- Comparator totality: n/a (no diff)
- Iteration-order provenance: n/a (no diff)
- Off-by-one / do-while / reverse iterators: n/a (no diff)
- Erase-while-iterating: n/a (no diff)
- Exception → Result partial-state parity: n/a (no diff)

## No-special-casing audit

`git diff rust-port..rport/w10-typed-access -- rust/` is empty, so there is
nothing to grep for function-name / address / constant / magic-number /
divisor / type-name hardcoding. Vacuously clean.

## Mechanical pass

- `git diff … -- rust/` for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  uncommented `as`: **nothing to scan** (empty diff).
- C++ oracle: `git diff … -- decompiler/ specs/` empty -> **C++ untouched**,
  207/207 + 675/675 PARITY OK preserved by construction.
- `cargo test --workspace` (run in MAIN tree, == rust-port + this verdict's new
  test): **all green, 0 failed** across every binary (including the 8 new tests
  below). This confirms NO REGRESSION — but it is rust-port's pass set, not a
  gain from this item.
- `cargo clippy -p kuna-decomp --test verify_w10_typed_access`: **clean**.
  NOTE (pre-existing, NOT a finding against this item): `cargo clippy -p
  kuna-decomp --lib --tests` surfaces one clippy `error` at
  `rust/crates/kuna-decomp/src/heritage.rs:3122` (a `... || true` tautology in
  a W5 heritage **unit test**) — it is present on `rust-port` itself, is in
  heritage (out of this item's scope), and is test code. Flagged here for the
  W5 owner; it does not bear on this empty branch.

## Adversarial tests (added with this verdict)

Since the branch delivers nothing, these tests pin the **C++ oracle semantics
of the in-scope public typed-access scorers as a standing parity fence** the
eventual real port must not break. All assert constants verbatim from
`decompiler/cpp/unionresolve.cc:931-977` (`ScoreUnionFields::scoreTruncation`)
and `scoreLockedType`. File:
`rust/crates/kuna-decomp/tests/verify_w10_typed_access.rs` (8 tests, all pass):

- `w10_typed_access_union_field_size_match_scores_plus10`
  (union size+offset match → +10, returns None: cpp:935-948)
- `w10_typed_access_union_no_match_scores_minus10` (cpp:938)
- `w10_typed_access_union_match_self_base_bonus_plus5`
  (`result.getBase()==unionDt` → +5; a *different* base gets no bonus —
  mutation-style cross-check, cpp:944-945)
- `w10_typed_access_scalar_exact_match_scores_plus10` (cpp:951-953)
- `w10_typed_access_scalar_wider_int_covers_scores_plus1`
  (`size >= vn_size + curOff` → +1, cpp:954-958)
- `w10_typed_access_scalar_unreachable_scores_minus10`
  (no sub-type reaches the truncation → ct==0 → -10, cpp:970-973)
- `w10_typed_access_locked_type_pointer_identity_seeds_plus5`
  (`lockType==ct` Rc-identity seed is exactly +5 over a structurally-equal
  distinct type)
- `w10_typed_access_locked_type_metatype_mismatch_is_lower`
  (totality + metatype-mismatch ordering)

These pass against the pre-existing (W6-verified) machinery, which is the
concrete trace this REJECT carries: the typed-access scorers are already
correct and already in-tree — this branch added nothing to them.

## findings

- F1 (blocker): the deliverable is empty. `rport/w10-typed-access` has 0
  commits beyond its merge-base with `rust-port` and an empty `rust/` diff; the
  reflog shows it was created and never committed to. No seam closed, no test
  added, no datatest pass attributable.
       cpp: n/a (no transcription was performed)
       rust: n/a (no rust/ file changed; diff is empty)

## losses

LOSS-166 (MAIN-tree losses.md) — records the empty-branch fact for traceability
of the W10 fleet accounting. This is NOT an accepted divergence (the verdict is
REJECT); it documents that this item contributed 0 to the 59/549 number.
