---
need_id: ppc64-localentry-splits-function
title: PPC64 ELFv2 local entry points split every function into an 8-byte named husk plus an anonymous body
track: quality
status: closed
severity: major
probe_id: p-a15704a28a56
acceptance_id: a-fb2b9c0f7f5b
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: ppclocalentry
touches: [decompiler/crates/kuna-analysis/src, decompiler/crates/kuna-decomp/src/p2_lift/kuna_funcboundflow.rs]
scope: small
regression_of: null
pr: "390"
closed_in_round: 2
closing_pr: "390"
reject_reason: null
---

## Symptom

Decompile or list any function in a PPC64 ELFv2 image and you get two functions where there is one: the
named symbol truncated to its 8-byte global-entry prologue, and the whole real body filed under an
anonymous `sub_<hex>` 8 bytes later.

Carried forward by the captain from PR #377's collateral sweep (builder `b-r2-analysis-generat`), which
found the defect while ablating the discovery bundle, explicitly declined to close it, and asked for a
need of its own. Captain reproduced it on a **repo-vendored** fixture at merged main `6d315ffe`:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le __do_global_ctors_aux
void __do_global_ctors_aux(void)
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}
```

The body is not lost, it is misfiled. `kuna functions --json` on the same image:

```
{"name": "__do_global_ctors_aux", "address_hex": "0x940", "size": 8}
{"name": "sub_948",               "address_hex": "0x948", "size": 112}
```

and the same 8/anonymous pairing for `deregister_tm_clones`/`sub_6f8`, `__do_global_dtors_aux`/`sub_7c8`
and `frame_dummy`/`sub_8b8` -- i.e. every ELFv2 function in the image that has a distinct local entry.
`--option funcboundflow off` recovers the correct loop in full, which is what pins the two halves of the
mechanism together. The builder measured **8 such truncations over 33 non-x86-64 fixtures**, so this is
not one fixture's accident.

This is live TODAY on default `kuna decompile` / `kuna functions` / `kuna decompile-all` for PPC64 --
no option, no mode and no bundle injection is needed to see it. PR #377 only dodged it by making the
discovery bundle a retry-on-miss rather than an up-front injection.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "__do_global_ctors_aux"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le",
    "binary_sha256": "b096b4098c64e6a8be6386d85fd2c1386fa646295247bcedabad2efea40e9a28",
    "binary_size": 70144,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le",
    "selector": "__do_global_ctors_aux",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "funcboundflow: fall-through reached the next function entry"
    ],
    "stdout_absent": [
      "while"
    ]
  },
  "notes": "The husk as shipped: kuna emits __do_global_ctors_aux with an empty body because a spurious entry sits 8 bytes into it (the PPC64 ELFv2 local entry point) and funcboundflow truncates the flow there."
}
```

Replayed at `6d315ffe` (merged main, round 2): **PASSES** -- exit 0, the funcboundflow truncation warning
present, no loop emitted.

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "__do_global_ctors_aux"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le",
    "binary_sha256": "b096b4098c64e6a8be6386d85fd2c1386fa646295247bcedabad2efea40e9a28",
    "binary_size": 70144,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le",
    "selector": "__do_global_ctors_aux",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "while"
    ],
    "stdout_absent": [
      "funcboundflow: fall-through reached the next function entry"
    ]
  },
  "notes": "Fixed: the real loop body is emitted for __do_global_ctors_aux -- no entry inside the function truncates it."
}
```

Replayed at `6d315ffe`: **FAILS** -- `stdout_matches[0]` (`while`) false and `stdout_absent[0]` false,
i.e. the husk and its warning are exactly what comes out. Two-arm gate holds.

Note the target is `binary_source: in-repo` against a fixture already committed at
`decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le`, so unlike every dataset-backed need in this
backlog **this acceptance is promotable into `tests/cli/` verbatim** when it closes.

## Hypothesis

**Advisory -- the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were
overturned while the symptom stood in all 8.

- Two facts compose. (1) On PPC64 ELFv2 a caller inside the same module branches to the **local** entry
  point, `st_value + 8`, so a call-target-seeded discovery files 0x948 as a function of its own. kuna has
  the information to know better and does not use it: `readelf -sW` shows these symbols carry
  `st_size == 0` and `[<localentry>: 8]` in `st_other`, and nothing in `kuna-analysis` consumes that
  ELFv2 annotation. (2) `funcboundflow` (`p2_lift/kuna_funcboundflow.rs`) then does exactly what it says
  on the tin -- a fall-through reaching a known function entry is truncated -- and the "known function
  entry" it stops at is 8 bytes into the function being decompiled.
- The narrow fix is to fold a local-entry address back into its global entry at discovery time, so
  0x948 is never an entry; the alternative (teach `funcboundflow` to ignore an entry that is its own
  function's local entry) leaves the bogus row in `kuna functions`.
- Not verified by the captain: whether the +8 entries come from call targets, from a prologue matcher, or
  from the ELF reader. Ablating `funcstart_patterns`, `aif` and `listing` individually does **not** remove
  them, so it is none of those three alone -- and note `--option listing off` did not drop the symbol names
  either, so the CLI's injected bundle appears to win over a user's `off` on these three; do not trust
  a `--option ... off` ablation on this path without checking it took effect.

## Refutation

_pending -- T_REFUTE has not run on this need (it was captain-filed at B_DONE, not cluster-filed)._

## Reference

_none recorded_

## Instances

- PR #377 collateral sweep, round 2 (builder `b-r2-analysis-generat`): 8 truncations over 33 non-x86-64
  fixtures when the discovery bundle is injected up front; `plt_ppc64le __do_global_ctors_aux` went from
  a correct loop to an empty husk.
- Captain replay at `6d315ffe`, round 2: reproduces with NO bundle injection, on the default option set.

## Decision log

- filed by the captain at round 2 B_DONE, carried forward from PR #377's decision log so it would not die
  in a note. The builder found it, judged it out of scope for a CLI-selector fix, and said so.
- scope set to **small** on the theory that one mechanism (fold local entry -> global entry) closes it;
  T_TRIAGE should re-scope to large if the fix turns out to need an option, since an entry-set change is
  a judgment call that can move emitted C on every PPC64 image. A strict husk-is-wrong-output fix needs no
  flag; a discovery-policy change does.
- credibility 0.9: two independent observations (the builder's 33-fixture sweep and the captain's replay),
  a vendored in-repo witness, and a mechanism confirmed at least half-way by `--option funcboundflow off`
  restoring the correct body.
- closed: acceptance a-fb2b9c0f7f5b now PASSES at da48cdc7e518
- hypothesis **upheld**, and the one part the captain flagged as unverified is now answered: the +8
  entries come from CALL targets in the Listing walk's worklist (`listing off`, `funcstart_patterns off`
  and `aif off` each leave them; `listing off` + `fast_funcdisc off` removes them), and
  `register_tm_clones` carries the same `st_other` yet is NOT split because its local entry is reached by
  a tail `b`, not a `bl`. The shipped fix is the hypothesis's "narrow fix": fold the local entry back into
  its global entry at discovery time.
- closing PR #390, merge commit `da48cdc7`, option `ppclocalentry` (default **on**), DIV-107. All four
  gates + `catalog --check` + `tests/cli` green on merged main at `da48cdc7` (675/675, 603/603, 5,341
  rust tests, 19/19 clitests).
