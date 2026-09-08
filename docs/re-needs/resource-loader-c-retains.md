---
need_id: resource-loader-c-retains
title: Resource-loader C retains return-address stores and drops API arguments
track: quality
status: open
severity: blocker
probe_id: p-ea0ebaae9eee
acceptance_id: a-38be14c225c2
hypothesis_status: overturned
credibility: 0.45
instances: 3
challenges: [6442366033c5d43938912a85]
rounds: [7]
first_seen_round: 7
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Readable stack dataflow and Windows API arguments in 0x401ba0.

> **Resource-loader C retains return-address stores and drops API arguments** (blocker, `?`)
> With both argument recovery options enabled, emits LoadLibraryExW() and EnumResourceTypesW(), plus assignments of CALL fall-through addresses. Kuna disassembly confirms 0x401baf follows CALL 0x40e660 at 0x401baa. Address-targeted prototypes restore argument count but leave extensive stack artifacts.

> **Argument recovery drops the resolved checker call's input** (major, `?`)
> The indirect branch passes the buffer, but the resolved branch emits check_serial(). Both recovery options are explicitly enabled. An explicit checker prototype restores the argument.

> **GetTickCount64 receives a fabricated argument** (minor, `?`)
> The first call is GetTickCount64(CONCAT44(dat_c,argc)); the second has no arguments. An explicit API prototype fixes this. Also encountered already-filed main-returns-invented-cookie; not refiled.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_401ba0",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "=\\s*(?:\\([^;\\n]*\\)\\s*)?0x401baf\\s*;",
      "LoadLibraryExW\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/Cube.exe",
    "binary_sha256": "6b80dcaa066884e4f7b71373f85aabcdb824d27593c1e4adf113a80e47062aeb",
    "binary_size": 193024,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_401ba0",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "=\\s*(?:\\([^;\\n]*\\)\\s*)?0x401baf\\s*;",
      "LoadLibraryExW\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/Cube.exe",
    "binary_sha256": "6b80dcaa066884e4f7b71373f85aabcdb824d27593c1e4adf113a80e47062aeb",
    "binary_size": 193024,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Incorrect stack-pointer propagation or callee stack effects; the responsible helper is unconfirmed.
- Arity propagation may miss calls synthesized by indirect-target resolution.
- An unlocked import prototype admits an incoming register as a speculative argument.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/Cube.exe --backend ida` — Server exited with status 1 before registering. No reference pseudocode was obtained; reference superiority is unestablished.

## Instances

- `?` (round ?, tester ?)
- `?` (round ?, tester ?)
- `?` (round ?, tester ?)

## Decision log

- filed by cluster.py from 3 observation(s)
- round 7 REFUTER: hypothesis **overturned** (was inconclusive). REFUTED IN-TICK 2026-09-07 ~10:35Z by the captain. VERDICT: OVERTURNED -- THE SYMPTOM IS REAL AND REPRODUCES EXACTLY, BUT FILED HYPOTHESIS #1 ('incorrect stack-pointer propagation or callee stack effects') IS DISPROVEN, AND THE DEFECT IS IN A DEFAULT-ON P4 OPTION THAT LEAVES DEBRIS WHEN ITS OWN TRIALS LOSE. Also: THE TWO ACCEPTANCE CLAUSES HAVE DIFFERENT CAUSES, so the cheap fix flips one and fails the probe. Read all of this before designing.

REPRO (arena .kuna-repipe/arena/7/6442366033c5d43938912a85/target/Cube.exe, PE32 i386):
  kuna decompile $B sub_401ba0 --option calleearity on --option varargstackargs on
  -> 47 declarations, v41 = 0x401baf; sub_40e660(); ... v36 = 0x401be3; LoadLibraryExW(); ... v33 = (int *)0x401c00; EnumResourceTypesW();
Both probe clauses reproduce at HEAD.

1. THE STACK-POINTER MODEL IS EXACT -- HYPOTHESIS #1 CANNOT STAND. Disassembly at the site: 0x401ba4-0x401ba9 PUSH EAX x6 (a frame allocation written as pushes), 0x401baa CALL 0x40e660, 0x401baf PUSH EDX, 0x401bb0 CALL 0x40e520, 0x401bb5 POP EDX. In the emitted C the CALL's return-address push and the following PUSH EDX land in THE SAME slot: 'v41 = 0x401baf;' then 'v41 = v5;' (v41 // stack - 0x24). That reuse is only possible if ESP is tracked exactly across the call, i.e. the callee's pop of the return address is modelled correctly. Every stack local carries a constant '// stack - 0xNN' offset; no drift, no unbalance warning on stderr or in the body. There is no stack-pointer or extrapop bug here. Do not send a builder to p2_lift stack normalization.

2. WHAT ACTUALLY PRODUCES THE RETURN-ADDRESS STORES: **callsitestackargs** (default ON, P4/active-input-trial-scoring, tier=core). One A/B settles it -- 'kuna decompile $B sub_401ba0 --option callsitestackargs off' collapses the function from 47 declarations to 8 AND REMOVES EVERY RETURN-ADDRESS STORE; the body becomes a clean 'sub_40e660(); sub_40e520(); ... LoadLibraryExW();'. So the pass that exists to promote outgoing stack stores into call arguments is what MATERIALIZES those stack slots as mapped locals; its trials are then scored INACTIVE, no argument is promoted, and the mapped slots survive -- including the slot the CALL's own return-address push writes. All of the cost, none of the benefit. THE DEBRIS IS THE RESIDUE OF A LOST TRIAL, not a lifting error.
Corroborating evidence that the arguments really are recovered as far as the slots: at the EnumResourceTypesW site the three outgoing values sit in consecutive slots immediately before the call -- 'v36 = 0; v35 = 0x402033; v34 = v37;' (0x402033 is the enum callback) -- and only then 'v33 = (int *)0x401c00;' (the return address) and the argument-less call. The values are right there and correctly ordered; nothing promotes them.

3. THE OTHER TWO OPTIONS IN THE PROBE ARE VACUOUS -- ignore any sentence in the observation that leans on them. Output md5 with 'varargstackargs off' is IDENTICAL to the default (these call sites are not variadic, and the option is default-on anyway), and 'stackalias on' is likewise identical. Control: 'compareform canonical' DOES move the output, so --option reaches this path and the null results are real null results, not a plumbing failure.

4. THE ARGUMENT LOSS AND THE STORE DEBRIS ARE **TWO DIFFERENT CAUSES**, AND THIS IS THE ACCEPTANCE TRAP (a-38be14c225c2). The acceptance demands BOTH /=\s*(...)?0x401baf\s*;/ absent AND /LoadLibraryExW\(\s*\)/ absent. 'callsitestackargs off' flips the FIRST clause and leaves the SECOND standing verbatim ('LoadLibraryExW();' is still argument-less in the clean output). So any fix that merely narrows or gates callsitestackargs FAILS ITS OWN PROBE. To flip clause 2 the call must actually acquire arguments, and for an IMPORT that is a KNOWLEDGE problem, not a recovery one: kuna-analysis/src/loader/kuna_peimportcall.rs binds the IAT slot to the import's NAME (and its no-return effect) but there is no Win32 API signature source anywhere in the tree -- LoadLibraryExW arrives with an empty prototype, so trial scoring is the only thing standing between the pushes and the call. That is why the tester found that an address-targeted prototype restores the arguments: a LOCKED prototype bypasses trial scoring entirely. See also [[kuna-proto-park-key]] for the import-thunk/IAT name-vs-address parking trap if a declared-prototype route is taken.

5. RECOMMENDED SHAPE (advisory): treat it as ONE need with two halves, because the acceptance already forces both. Half A -- when a call-site stack-argument trial is scored inactive, do not leave its slot mapped (or re-run the deadcode fixpoint over the abandoned trial's slots), which removes the return-address stores AND ~39 spurious declarations. Half B -- give the promoted-name import a prototype so the trials win at all. Half A alone is measurable and safe; half B is the one that flips the second clause. If the builder cannot do both, this should come back as a PROPOSAL rather than a half-fix that cannot close.

SEVERITY/SCOPE: severity blocker stands (47 declarations vs 8 is the difference between readable and unreadable, and it fires on every PE32 call site in the image). SCOPE should probably move from small to MEDIUM given half B. The other two instances in this bucket are NOT this mechanism and were deliberately kept merged at T_DEDUP -- 'argument recovery drops the resolved checker call's input' and 'GetTickCount64 receives a fabricated argument' are the same p4_calls plane (too few / too many at a call site) but a fix for one need not move the others; their own probes are preserved in rounds/7/gate.json results[].observation.

## Captain note (round 10, re-dispatch after a failed merge)

PR #520 carries the whole feature but its merge bailed: CI step `make test-stages`
returned 703/704, `REGRESSED (1): data:PEIMPORTCALL #5: on ends bail at the bound
no-return import`. `make test` passed 675/675. Main was never touched.

Two things were measured before this re-dispatch; do not re-derive them.

1. The first builder's green parity gates were an artifact of a STALE HELPER.
   `make test` (Makefile:46) and `make test-stages` (Makefile:52) guard the helper with
   `test -x $(BINDIR)/decomp_test_dbg || $(MAKE) binaries` -- an existence test, never a
   freshness test -- and `kuna test --datatests` FORKS that binary. The worktree's copy
   predated the feature commit by ~30 minutes, so both gates measured a pre-win32sigs
   decompiler. `decomp_test_dbg` is built by **kuna-harness**, not kuna-console:
   `cargo build --release -p kuna-console` does NOT relink it. Run `make binaries` (or
   `cargo build --release -p kuna-harness`) BEFORE trusting any parity result. The
   worktree was RECREATED for this attempt, so worker.sh's initial `make binaries` has
   already built a helper matching the branch tip -- but rebuild it again after any Rust
   change you make.

2. With the fresh helper: `make test` is 675/675 PARITY OK, so the DIV-141 default-ON
   evidence bar genuinely holds and the feature does not need re-scoping. The single
   stages assertion is the only real defect.

Root cause of that assertion, measured on the same fixture and binary:

```
kuna decompile .../pe_noreturn_import.exe bail --option win32sigs off
  ExitProcess(); // no-return
kuna decompile .../pe_noreturn_import.exe bail            # default, win32sigs on
  ExitProcess(a0); // no-return
```

`tests/stages/ghdec-peimportcall.xml` asserts the regex `ExitProcess\(\); // no-return`
(min=1 max=1), an empty-parens spelling frozen in by DIV-57 (#254) when kuna had no
Win32 prototypes. ExitProcess really does take one UINT, so the arg-bearing output is
correct and the regex is what is stale; what the test exists to protect -- peimportcall
binding the call to its import, and the `// no-return` bail surviving -- is unchanged.

Expected fix: update that one assertion to the arg-bearing spelling and re-record
`docs/baseline-stages.json`. That is an expectation update for an intentional, measured
change on a kuna-owned test, not a re-pin of `docs/baseline.json` (untouched, green).
Own the justification in the PR. Then re-run `--merge` on the SAME branch; do not open a
new one. Note `make check-spec` and `kuna catalog --check` were never evidenced on disk
by the first builder -- run all four gates yourself.
