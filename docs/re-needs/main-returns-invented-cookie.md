---
need_id: main-returns-invented-cookie
title: Main returns an invented cookie-check result instead of preserved zero
track: quality
status: open
severity: major
probe_id: p-38e3d04ecf77
acceptance_id: a-3eab4d19627d
hypothesis_status: overturned
credibility: 1.0
instances: 2
challenges: [69b7492049fa49a2a260218f, 69d6e9ba49fa49a2a26023a7]
rounds: [6]
first_seen_round: 6
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover main's zero return across the MSVC cookie checker.

> **Main returns an invented cookie-check result instead of preserved zero** (major, `69b7492049fa49a2a260218f`)
> Main ends by returning the cookie-check call. Disassembly sets EAX to zero at 0x1400061fb before the call; the checker's successful path preserves RAX. Decompiling the checker itself yields void. Asserting a void prototype makes main return an unassigned local. msvcstackguard on leaves the original defect unchanged; calleepreserves is already on.

> **Preserved random seed becomes the return value of a void wipe helper** (major, `69d6e9ba49fa49a2a26023a7`)
> Assigns wipe_random(...) & 0xffffff | 0x1000000 to the secret. The helper never writes RAX and independently decompiles as void. calleepreserves on does not help; declaring the helper void instead leaves an uninitialized EAX local.

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
    "0x140001000",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "return\\s+sub_[0-9a-fA-F]+\\([^;]*\\);\\s*\\}"
    ]
  },
  "target": {
    "binary_rel": "pe_cookiecheck_x86_64.exe",
    "binary_sha256": "97095d9bf909a03e82924dfa826988c27ef3aac9b1c90b64bc4f71f4e3582f12",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_cookiecheck_x86_64.exe",
    "selector": "0x140001000",
    "selector_kind": "addr"
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
    "0x140001000",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "return\\s+sub_[0-9a-fA-F]+\\([^;]*\\);\\s*\\}"
    ],
    "stdout_matches": [
      "return\\s+(?:0|0x0)\\s*;"
    ]
  },
  "target": {
    "binary_rel": "pe_cookiecheck_x86_64.exe",
    "binary_sha256": "97095d9bf909a03e82924dfa826988c27ef3aac9b1c90b64bc4f71f4e3582f12",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_cookiecheck_x86_64.exe",
    "selector": "0x140001000",
    "selector_kind": "addr"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The call-effect summary may decline the checker because its failure path tail-jumps, losing the successful path's preserved RAX.
- Return recovery treats preserved RAX as a call output rather than retaining its pre-call value.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/almost` — Server exited status 1 before registering. No comparative decompilation obtained.

## Instances

- `69b7492049fa49a2a260218f` (round 6, tester t-r6-69b74920)
- `69d6e9ba49fa49a2a26023a7` (round 6, tester t-r6-69d6e9ba)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 6 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED -- both filed hypotheses point a builder at the wrong layer, and I have a control that proves it. Symptom reproduced verbatim on the witness (vm.exe 0x140005b30): 'return sub_140007d90(v12);'. H1 said the call-effect summary declines the checker because its failure path tail-jumps. The tail jump is REAL -- 0x140007da9 JMP 0x1400081d8, __report_gsfailure -- but it is not the operative cause, and calleepreserves is not the pass in play. H2 said return recovery treats preserved RAX as a call output; that describes the OUTPUT but names the wrong pass. THE CONTROL THAT DECIDES IT: declare the checker void and the bug does not go away, it changes shape -- 'kuna decompile vm.exe 0x140005b30 --addr --define-function 0x140007d90=cookiechk --assert "prototype cookiechk void cookiechk(long long c)"' emits 'cookiechk(v30 ^ v15); return v2;' where v2 is an UNINITIALISED local. So the pre-call 'XOR EAX,EAX' at 0x1400061fb never reaches the RET at 0x140006217 whatever the callee returns. Any fix built on the callee's return model therefore CANNOT produce 'return 0;' -- it can only trade an invented call result for an invented local, which is what the tester already saw. THE ACTUAL MECHANISM, read from the spec and the source: x86-64-win.cspec lists RAX (and XMM0) in <killedbycall> for the default __fastcall proto (lines 125-128, repeated at 205-208), so guardCalls plants an INDIRECT creation of RAX at the call and the pre-call zero is dead. calleepreserves is exactly the pass that downgrades killedbycall to unaffected from the callee's decoded body -- and it CANNOT help here BY DESIGN: its own module header (p4_calls/kuna_calleepreserves.rs, 'What it will not do') states 'The return register is not affected... guardCalls's own output-active branch re-promotes the range to killedbycall when it is the call's return storage'. That is why the tester's 'calleepreserves is already on' changed nothing; I re-ran with it off as a control and the output is byte-identical. A second, independent block sits behind it: the same probe declares itself incomplete at an unresolved branch, and this callee's failure path leaves its extent by a direct JMP -- so even without the return-register carve-out the summary may decline. H1's premise is thus true but doubly non-operative. WHAT A CORRECT FIX HAS TO DO (for the builder): keep the caller's own pre-call RAX definition alive across a call the callee is PROVEN not to write, i.e. relax the return-storage re-promotion in the output-active branch when the callee-body probe proves no write AND the call has no recovered output -- not touch return recovery (H2) and not touch the tail-jump handling (H1). This is a behaviour change on the most-used register on the platform: it must ship behind its own option, and the risk to size is real -- every void MSVC helper call in every function is a candidate. The acceptance probe is SOUND: it demands both the absence of 'return sub_...(...)' and the presence of 'return 0;', so neither the void-prototype dodge nor a bare local rename can fake it. The SYMPTOM stands. NOTE ON THE SECOND INSTANCE (69d6e9ba, 'preserved random seed becomes the return value of a void wipe helper'): I did NOT re-run it -- it is consistent with the same killedbycall/return-storage mechanism on a non-cookie helper, but that is inference, not measurement.
- round 8 CAPTAIN (build track, after builder b-r8-main-returns-inv stopped short of merge): THE BUILDER'S STOP-POINT DIAGNOSIS IS WRONG AND MUST NOT BE ACTED ON. Its handoff note (docs/features/main-returns-invented-cookie/record.json.note on branch feat/re-main-returns-invented-cookie, commit 01438935) says make rust-test is red because promote_compare "regains a `reg = N; return reg;` round-trip", blames all_writes_const_copy in mark_output_storage_addr_tied, and proposes two fixes. Both are phantoms. The builder grepped its own test output and truncated at the panic LOCATION without ever reading the panic MESSAGE, then attributed the failure to the wrong assertion. verify_w10_proto_unlock.rs:202 is the ORACLE assertion (cpp.contains("xunknown4 promote_compare(char *"), message "oracle promote_compare signature drifted"); the round-trip assertion the builder blamed is at line 192 and it PASSES. MEASURED CONTROL, both release decomp_test_dbg binaries driven on tests/datatests/promotecompare.xml with KUNA_DUMP=1: main at bfacf0c6 and the branch at 01438935 emit BYTE-IDENTICAL C -- "unsigned int promote_compare(char *a0)", "return 1;", "return 0;", and NO constant-assign-to-local round-trip line in either. The branch does not move this test. WHY IT IS RED AT ALL: cpp_oracle_bin() resolves KUNA_DECOMP_TEST, which worker.sh points at the worktree's OWN kuna decomp_test_dbg, so kuna is graded against itself and can never print the C++ spelling "xunknown4" (kuna prints "unsigned int" by DIV-6 realtypes, in every build, patched or not). The proof needs no /proc read: if KUNA_DECOMP_TEST were unset, cpp_oracle_bin() would resolve decompiler/cpp/decomp_test_dbg, which does not exist since the C++ tree was removed, dump_body would return None, and the whole oracle block would be skipped -- so merely REACHING line 202 proves the oracle was a kuna binary. In CI the var is unset and the target is skipped. FOR THE RE-DISPATCHED BUILDER: resume on branch feat/re-main-returns-invented-cookie at 01438935 (RESUME_BRANCH), do NOT re-implement -- the option, its three evidence gates, phases.toml, options.rs, all counters, docs/options.md, the DIV-133 row, spec prose, the two-pass stage test with its endbr64/ret negative control, the vendored pe_cookiecheck_x86_64.exe fixture + generator and the promoted tests/cli probe are all already committed, and the other four gates are green (make test 675/675, make test-stages 683/683, make test-cli 74/74, make check-spec, catalog --check). The ONLY outstanding work is: (1) run the FULL make rust-test once as `env -u KUNA_DECOMP_TEST make rust-test` -- commit 01438935 fixed three of the four red targets (kuna-decomp --lib, catalog_bytecompat, options_md_fresh) but the full suite was never re-run after that commit, so those three are unconfirmed as a set; (2) push and open/merge the PR. Budget this as a short session, not a fresh implementation.
