# w10-f0flag-v2-untie — INDEPENDENT VERIFIER VERDICT (Round 2)

**VERDICT: ACCEPT_WITH_LOSSES** — the round-1 blocker (`Enum Reading #5`
regression) was a STALE-FORK artifact and is fully resolved by the round-2 repair
(rebase onto rust-port HEAD, which carries the `Enum render` integration). The
regressed set is now EMPTY, +9 real wins, all hard gates pass. The only residual
loss is the LOSS-206 forwarding-alias stand-in (a different local *name*, not a
regression).

Branch `rport/w10-f0flag-v2-untie` @ `62bd8d1` (rebase onto `rust-port`@`ea42fa0`
+ pin Enum Reading #5). Worktree `/tmp/kuna-f0flagv2`; fresh baseline worktree
`/tmp/kuna-f0v2-base` (detached @ `ea42fa0`); both rust `decomp_test_dbg` built
INDEPENDENTLY from source (forced recompile of the changed crate), 148 `.sla` each.
The diff base is the three-dot `rust-port...HEAD` (merge-base `ea42fa0`).

## The change (faithful and generic — CONFIRMED independently)

Single source edit: `rust/crates/kuna-decomp/src/coreaction_cleanup.rs`
`mark_output_storage_addr_tied` (reached from `ActionMergeRequired::apply`).
(a) The marker-write un-tie is lifted OUT of the old `if output_locked &&
!written.is_empty()` gate to `if !written.is_empty() && !is_forwarding_register_alias`
— so an output-UNLOCKED transient return register (doublemove's `f0`) also un-ties.
(b) A new `is_forwarding_register_alias` predicate keeps a register tied when its
sole def is a `CPUI_COPY` of a DISTINCT processor register (LOSS-206 stand-in for
the unported per-Varnode dynamic mapentry that names `partialmerge`'s `a_simple`).

FAITHFULNESS to the C++ addrtied derivation: CONFIRMED. `Funcdata::
syncVarnodesWithSymbols` (funcdata_varnode.cc:993-997) sets `mapped|addrtied` for
an un-symboled storage ONLY when `lm->inScope(addr,size,usepoint)`. Base
`Scope::inScope` (database.hh:599-600) is `rangetree.inRange(addr,size)`, and
`ScopeLocal`/`ScopeInternal` do NOT override it (no inScope in varmap.hh/varmap.cc)
— the local range tree owns the stack/local space, never a processor register, so
C++ leaves a transient return register un-tied. The `output_locked` lift is
justified (doublemove's protos are output-unlocked yet `f0` is a pure transient).

NO SPECIAL-CASING (item 2): PASS. Grepping ALL added non-comment source lines: the
production predicate keys ONLY on generic geometry — `s.get_type() ==
IPTR_PROCESSOR`, single write, `CPUI_COPY` shape, `src.get_addr() != &addr`,
`!is_constant() && !is_persist() && !is_input()`. ZERO float/f0/0x1000/register-
name/function-name/value keying in source (the float/f0/register tokens in the grep
are all *test*-file datatest target names — legitimate). All predicate methods
(`is_persist`/`is_input`/`is_constant`/`get_def`/`get_addr`) and `IPTR_PROCESSOR`
are genuine accessors/variants, not fabricated. Proven generic: float `f0`
(doublemove) and integer `EAX` (modulo2) both un-tie; the same `EAX` in readpartial
stays tied — discriminator is storage geometry, not register identity.

## The round-1 blocker is RESOLVED (re-verified deterministically)

Round 1 REJECTED on `Enum Reading #5` regressing to raw hex `0x100800`. That was
the MISSING `Enum render` feature (the branch was stale-forked from a parent that
predated rust-port `Enum render` integration), not the un-tie. Round 2 carries the
merge of rust-port HEAD; direct `enum.xml`/`ptrenumequal` dump:
- C++ oracle:   `if (ptrequal->flagfield != (FLAG_100000|FLAG_800))`
- baseline:     `if (ptrequal->flagfield != (FLAG_100000|FLAG_800))`  (Success)
- branch:       `if (ptrequal->flagfield != (FLAG_100000|FLAG_800))`  (Success, byte-
  identical to the oracle on the whole `ptrenumequal` block)

The new adversarial test A5 (`enum_reading_5_constant_stays_enum_named_not_raw_hex`)
pins this against the live oracle — round-1's coverage gap is closed.

## Hard gates — all PASS (independent re-run)

- **Full suite, both engines built from source.** Baseline rust-port: **349/675**.
  Branch: **358/675** (+9). C++ oracle in the main tree: **207/207 unit + 675/675
  datatest, PARITY OK** (diff touches no C++/specs/baseline).
- **REGRESSED SET EMPTY.** Per-assertion both-direction `comm` of the named-failure
  sets: newly-passing = exactly {Double precision move #1/#4, If/Switch #3/#5/#6,
  Leading zeros count #1, Mixed float/int #2/#6, ModuloAlt #1}; regressed = ∅.
- **partialmerge :: Partial Merge #3 STILL PASSES** (both base and branch). The
  LOSS-206 forwarding-alias gate works: readpartial renders `v1 = glob1.a + 10;
  return v1;` (tied, explicit) — it does NOT collapse to the forbidden `return
  glob1.a + 10;`. The whole Partial Merge family failure set is identical base<->branch.
- **Legit ties stay tied.** Stack `s0x28` (doublemove second function, still
  `float8 v1; // stack + 0x28`) and global RAM (doublemove #2/#3/#5/#6 PASS on both)
  are unperturbed. boolless ACC unchanged.
- **cargo test --workspace: GREEN (3633 passed / 0 failed)** in debug, run
  independently in the worktree. The 5 adversarial tests in
  `verify_w10_f0flag_v2_untie.rs` all PASS in release where the rust binary + `.sla`
  + C++ oracle resolve (so the gate AND the byte-identical-to-oracle direction
  checks are actually exercised, not SKIPped).
- **clippy --lib (kuna-decomp): clean** (0 warnings/errors).
- **Byte-identity.** boolless, condconst (condconst_conn), switchhide/switchind/
  switchloop/switchreturn: BYTE-IDENTICAL base<->branch. doublemove `loaddouble` and
  the full doublemove C-source: byte-identical to the C++ oracle (only a verbose
  p-code SeqNum differs). readstruct (nestedoffset): now IMPLIED, byte-identical to
  the oracle. The two vN-numbering tests legitimately relocated readstruct->loopcomment
  (loopcomment still renders v1/v2/v3 — faithful, not coverage-weakening).
- **Reserved files UNTOUCHED.** Diff is rust/-only, 4 files (coreaction_cleanup.rs +
  3 test files); `printc.rs` and `dynamic.rs` are NOT in the diff; base_explicit was
  not folded in (optional). B0 unchanged.
- **>=3 adversarial tests: 5 committed**, all pass — (A1) f0 IMPLIED + byte-identical
  to oracle, (A2) partialmerge #3 stays tied, (A3) anti-special-casing (float f0 +
  int EAX un-tie, same EAX stays tied in readpartial), (A4) no per-function token
  leak, (A5) Enum Reading #5 enum-named vs oracle.

## The losses (accepted, faithful)

1. **LOSS-206 stand-in.** readpartial's forwarding-alias return register stays tied
   as an unnamed `v1` where the C++ oracle names a recovered `a_simple`. Same
   explicit/non-collapsed shape, different local name — the per-Varnode dynamic
   mapentry is unported, so keeping the register tied is the faithful minimum that
   preserves Partial Merge #3.
2. **switchmulti** is no longer byte-identical to baseline, but the change is an
   IMPROVEMENT toward the oracle (`uint8 v1; v1 = v3; return v1;` -> `return v3;`)
   with Switch Multi #1-#9 assertions identical base<->branch — beneficial, not a
   regression.

Recorded by the w10-f0flag-v2-untie independent verifier (round 2).
