---
need_id: address-taken-key-parameter
title: Address-taken key parameter is redeclared as a local
track: quality
status: open
severity: blocker
probe_id: p-dcdd99149caa
acceptance_id: a-7fae5b7d7391
hypothesis_status: upheld-symptom-overturned-location
credibility: 0.85
instances: 1
challenges: [64f1f7fad931496abf909535]
rounds: [5]
first_seen_round: 5
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p9_emit]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover how the runtime transformation uses its incoming four-byte key.

> **Address-taken key parameter is redeclared as a local** (major, `64f1f7fad931496abf909535`)
> The signature declares unsigned int a0 and the body declares int4 a0 again, then passes &a0. Disassembly uses LEA ECX,[EBP+8], the incoming parameter. calleearity and varargstackargs do not change this. The whole-binary run also hit already-filed block-processing-panics-out.

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
    "0x401571"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?s)\\([^\\)\\n]*\\b(a[0-9]+)\\b[^\\)\\n]*\\)\\s*\\{[^}]*\\n\\s*\\w+\\s+\\1\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/illusion.exe",
    "binary_sha256": "8093e4a899faa8283b1573f571834fffbee872c7e40da384bf36e6edb9b88a8e",
    "binary_size": 1292288,
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
    "0x401571"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "(?s)\\([^\\)\\n]*\\b(a[0-9]+)\\b[^\\)\\n]*\\)\\s*\\{[^}]*\\n\\s*\\w+\\s+\\1\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/illusion.exe",
    "binary_sha256": "8093e4a899faa8283b1573f571834fffbee872c7e40da384bf36e6edb9b88a8e",
    "binary_size": 1292288,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Address-taken parameter storage may be emitted again as a local symbol.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x401571 --binary ./target/illusion.exe --backend ida --json` — No comparison obtained: IDA startup exited 1 before registering; decompile reported no running server.

## Instances

- `64f1f7fad931496abf909535` (round 5, tester t-r5-64f1f7fa)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 5) on the filed binary .kuna-repipe/arena/5/64f1f7fad931496abf909535/target/illusion.exe with the release kuna at 38c461d1.

VERDICT: UPHELD, AND THE FILED SEVERITY IS TOO LOW. The hypothesis ("address-taken parameter storage may be emitted again as a local symbol") is exactly right, and I pinned it to one storage location by address. THE OUTPUT IS NOT MERELY UGLY, IT IS INVALID C: a parameter and a local with THE SAME NAME in THE SAME SCOPE and TWO DIFFERENT TYPES.

REPRO, one command, no options, no assertions:
  kuna decompile <bin> 0x401571
  void sub_401571(unsigned int a0,int4 a1,int4 a2)
  {
    int4 a0;                                <- redeclares the parameter, different type
    ...
    v1 = CryptDeriveKey(v4,&a0,4,0);        <- the only use of either
  }
A C compiler rejects that outright, so `decompile-project` output for this function cannot recompile.

THE PROOF THAT IT IS ONE STORAGE LOCATION, FROM kuna's OWN LINE MAPPING — do not re-derive this:
  the `&a0` line maps to 0x401641, and 0x401641 disassembles to `LEA ECX,[EBP + 0x8]`.
  [EBP+0x8] on this i386 cdecl frame IS the first incoming parameter (return address at EBP+4; the body reads the other two at [EBP+0xc] and [EBP+0x10], which is where `v6 = a2 - a1` comes from at 0x401577-0x40157d).
So `&a0` genuinely takes the address of the parameter's own slot. There is no second object anywhere on the frame. The tester's LEA observation is correct and I confirmed it independently.

THE TWO VIEWS DISAGREE, AND A FIX MUST MOVE BOTH — this is the part nobody filed:
  `--json` .functions[0].variables lists param_1 (kind arg, arg_index 0, stack_offset 4, type "unsigned int") with line_numbers [] and addresses [] — i.e. AN UNUSED PARAMETER.
  There is NO entry for the printed `int4 a0` local at all; the JSON variable list has no symbol at that offset besides param_1.
So an agent reading the JSON sees a parameter that is never used; an agent reading the C sees a local that shadows it and a `&a0` that resolves to the wrong one of the two. Both are wrong, differently, from one cause. An acceptance that only greps the C will not notice if the JSON stays broken.

TYPE DISAGREEMENT PROVES THEY ARE TWO SYMBOLS, NOT ONE PRINTED TWICE: signature `unsigned int`, body `int4`. Whatever creates the body symbol did not consult the parameter symbol, so this is a symbol-mapping defect (the address-taken varnode gets its own address-tied local symbol over storage that is already claimed by an input parameter), not a printer defect. Look in the P6 variable/scope-restructure plane before you look at P9 emission.

THE NAIVE FIX IS WRONG AND IT WILL PASS THE ACCEPTANCE. The acceptance regex only demands that the parameter not be redeclared, so RENAMING the body symbol (`int4 a0_1;`) closes the need, compiles, and ASSERTS A FALSEHOOD: two distinct objects where the machine has one, with `&a0_1` no longer visibly related to the parameter the caller passed. The correct fix MERGES: the address-taken use must resolve to the parameter symbol itself, so the declaration disappears and the call reads `CryptDeriveKey(v4,&a0,4,0)` with `a0` being the parameter. Write the stage test to demand the merged form (parameter used, no shadow, and the JSON variable carrying the use), not merely the absence of a duplicate line.

NOT COVERED BY ANY EXISTING OPTION: nothing in docs/options.md addresses an address-taken parameter's storage getting a second symbol (`loadguardrange` is about stack arrays and explicitly concerns the never-address-taken case).

SCOPE: leave at `small` only if the merge turns out to be one mapping decision; if the fix has to reach into how input parameters are entered into the local scope, it is `medium`. Have the builder answer that in design before it writes code.
round 5 TRIAGE (captain): SEVERITY major -> blocker, TOUCHES -> p6_variables. The refuter proved the output is INVALID C -- a parameter and a local with the same name in the same scope and two different types -- so decompile-project output for this function cannot recompile; that is a blocker by the same standard every other invalid-C need in this backlog was filed under. SCOPE STAYS small BUT IS CONDITIONAL: small only if the address-taken use can be resolved onto the existing parameter symbol as one mapping decision. If the fix has to change how input parameters are entered into the local scope, the builder must STOP and escalate to a [PROPOSAL] rather than widen in place -- answer that in design, before code. And the acceptance is rename-passable: a stage test must demand the MERGED form (the parameter used, no shadow declaration, and the JSON variable carrying the use), not merely the absence of a duplicate line.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- round 10 BUILDER: the SYMPTOM stands exactly as filed and refuted; the LOCATION does not. Instrumented `PrintC::emit_local_var_decls` on the witness: the declared high named `a0` carries `kuna_ref_symbol = Some(SymbolId(10v5))`, whose ScopeLocal category is 0 = FUNCTION_PARAMETER -- there is ONE symbol at the parameter's storage and the `&a0` reference is already bound to it, so P6 mapped it correctly. (The `unsigned int` vs `int4` disagreement is the FuncProto parameter's type against the reference high's own data type, not two symbols.) The defect is in P9: kuna's printer walks HighVariables and recognises a parameter by asking whether a `function_parameter` Symbol contains the STORAGE of one of the high's Varnodes, and an `&parameter` reference's only Varnode is the PTRSUB offset CONSTANT. Closed by option `paramrefdecl` (P9, default on, DIV-143), which makes the same decision on symbol IDENTITY and so removes the declaration rather than renaming it -- the merged form the refuter demanded. The `--json` half was a second, separate query (`variable_storage_varrefs` matches a parameter's own storage, and the name fallback misses because the JSON spells it `param_1` while the high is `a0`); added as a third fallback consulted only when the other two find nothing, so `param_1` now carries the `&a0` line. Regression probe promoted to `tests/cli/address-taken-key-parameter.json` over a newly vendored 4,532-byte i386 ELF (`paramrefdecl_i386`), since the acceptance target is a dataset binary.

## Round 10 — salvage brief (captain, 21:52Z)

The work is DONE and unmerged. A previous builder implemented `paramrefdecl` (DIV-143),
pushed `feat/re-address-taken-key-parameter`, and opened **PR #527**, then ran out of budget
before merging. Do not re-implement it. Read the branch first (`git log --oneline
origin/main..feat/re-address-taken-key-parameter`), then finish it:

1. **Rebase onto `origin/main`.** PR #527 is OPEN, not draft, and `mergeable: CONFLICTING`.
   The branch was cut at `84a32105`; `origin/main` is now `135aa456`, four commits ahead
   (#522, #523, #525, #526).
2. **`make rust-test` to completion.** The previous builder died mid-suite at 431 suites
   green with no result line, so the workspace gate is UNCONFIRMED. This is the one gate you
   cannot inherit. The other three plus `make test-cli` were reported green
   (675/675, 707/707, 101/101, check-spec strict, `catalog --check` OK) — re-run them after
   the rebase, since the rebase is what can break them.
3. **Re-derive the counts the rebase invalidates**, do not hand-merge them: the catalog
   counts, the `tests/stages` corpus file count in `decompiler/crates/kuna-base/src/xml.rs`,
   and `docs/baseline-stages.json`.
4. **DIV-143 is still free** on `origin/main` (highest row there is DIV-142), so no renumber
   is expected — but check the row is present exactly once after the rebase.
5. Then merge under the `merge` lease.

Acceptance `a-7fae5b7d7391` was reported PASS on the branch; confirm it again after the
rebase before merging.
