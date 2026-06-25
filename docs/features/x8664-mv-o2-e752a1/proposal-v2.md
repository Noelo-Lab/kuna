# [PROPOSAL v2] noreturn-by-body inference — the real fix for `mv -O2 main` case recovery

**Supersedes step-1 (`jumptable-augment`) of `proposal.md`.** Implementation of step-1
revealed it is mis-shaped for this function (full evidence: `findings.md`). This v2 proposes
the mechanism that actually closes the case-recovery sub-gap, validated by ablation. It needs
human go/no-go because it is a **new analysis pass** (Hard rule 7: new pass *type*, P0/S1
analysis tier) rather than the previously-approved S2 jump-table Action.

- Opportunity: `test_decompiling_x8664_mv_O2::main`
- Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/mv_-O2`, `main` @ `0x402b40`, x86_64
- angr 9.2.213 reference. Reproduction in `findings.md`.

## The problem (sub-gap #3, restated correctly)

kuna recovers the `getopt` switch but renders 10 of 13 cases; the long-only options
`--version` (`-0x83`) and `--help` (`-0x82`) survive as a residual `cmp %eax` cascade folded
inside the `default:` arm. Root cause: kuna treats the program-local `usage()` as a
**returning** function, so the jump-table default block `usage(1)` keeps a fall-through into
the residual comparisons, shadowing the native jump-table cases and disconnecting the cascade
variable from the switch value (it reads `usage`'s modeled return, not `getopt`'s).

`usage()` is in fact **no-return** (every path in its body reaches `call exit@plt`). angr
infers this and renders the clean 13-case switch. kuna has no body-based no-return inference:
`noreturn_known` (static list) does not know the local `usage`, and `noreturn_disc`
(fall-through evidence) is defeated because GCC laid the residual comparisons physically after
the `usage` call sites.

## Proposed feature — `noreturn_body`

A new, **default-OFF**, Listing-gated analysis pass: the kuna analog of angr's no-return
propagation / the body-based half of Ghidra's no-return discovery.

> For each function `F` in the Listing whose body contains **no reachable `RET`** and **all**
> of whose terminating basic blocks end in a call to an already-no-return function
> (`noreturn_known` ∪ already-discovered), mark `F` no-return (`set_function_no_return`).
> Iterate to a fixpoint (a wrapper whose last act is a call to a discovered-no-return is
> itself no-return).

This reuses the entire existing consumption path — once the flag is set (pre-flow, as the
`option noreturn usage true` ablation demonstrates), kuna already re-splits the block at the
no-return call, prunes the dead cascade, and the native jump-table cases surface. **No S2 /
jump-table / structurer change is required.**

### Validated effect

`option noreturn usage true` (set after load) → `main`: **12 cases (from 10), 17 gotos (from
24)**, with `case -0x83: version_etc(); exit(0);` and `case -0x82: usage(0);` recovered —
i.e. exactly the win step-1 targeted. (Remaining angr delta — the `while(true)` getopt loop
and tail `if/else` condensing — are sub-gaps #1/#2/#4, separate structuring steps, unchanged
by this feature.)

## Why this is the right shape (and its scope)

- **Builds on existing infrastructure:** `set_function_no_return` + the no-return flow
  consumption already exist and already work; `noreturn_disc` already walks the Listing. This
  adds a sibling discovery *signal*, not new infrastructure.
- **One module + standard anchors:** new pass module, one `architecture` gate flag (default
  off), pass registration, one `options.rs` + `stages.toml` `settableTable` row. No S7/S8
  structuring code. Does not touch the jump-table recovery.
- **Corpus-safe:** like `noreturn_disc`, it is a no-op when the Listing is absent. The 675
  XML datatests and the stage corpus carry no Listing, so all three parity gates stay
  **byte-identical** with the option default-OFF *and* default-ON. The only behavior change is
  on real-ELF targets with `option listing on`.

### Open risks for the implementer to resolve

1. **Body availability.** The pass must analyze callee bodies (e.g. `usage` @0x34a0) via the
   Listing's decoded instructions / intra-function CFG. Confirm the Listing exposes enough to
   compute "no reachable RET ∧ all leaves end in a no-return call" without a full decompile of
   each callee (cost). `noreturn_disc`'s Listing access is the model to follow.
2. **Soundness of "no reachable RET".** Tail-calls, `longjmp`, signal handlers, and indirect
   calls complicate the leaf classification. Start conservative (only mark when every leaf is
   a *direct* call to an established no-return and there is provably no `RET`), accept false
   negatives.
3. **Speed.** Adds a Listing pass; measure against the +5% budget on `mv -O2 main` and keep
   opt-in if it regresses (it is default-OFF regardless until an ablation is clean).

## Recommended option

`noreturn_body` · `change_kind = analysis-enablement` · `source_decompiler = angr`
(inspiration: angr no-return propagation; Ghidra `FindNoReturnFunctionsAnalyzer` body half).
Default-OFF; pairs with `option listing on`.

## Recommendation

Approve `noreturn_body` as the replacement for step-1. It is the mechanism that actually
closes sub-gap #3, is corpus-safe, and reuses existing flow consumption. Steps #1/#2/#4 of
the original proposal (the `while(true)` loop + switch-break + tail condensing — the S7/S8
SAILR structuring work) remain deferred and unchanged.
