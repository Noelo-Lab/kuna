---
need_id: flow-call-override-retain
title: Flow-call override cannot retain the pushed continuation
track: tooling
status: open
severity: blocker
probe_id: p-b8e2aeb8fe0c
acceptance_id: a-d34213fb6c8c
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5c33c5d40ad448c681]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Specify both the RET-based call and its continuation at 0x401770, preserving the connected loader dataflow.

> **Flow-call override cannot retain the pushed continuation** (blocker, `5ab77f5c33c5d40ad448c681`)
> Reported flow 0x40176f call as applied, but emitted only LoadLibraryA(s_40151e). The pushed continuation is the next instruction, 0x401770. Defining it separately and overriding RET 0x401785 exposes GetProcAddress, but loses cross-fragment state. No continuation-address control was found in the manual or catalog. Enabling calleearity and varargstackargs still left that fragment's GetProcAddress arguments absent.

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
    "0x401757",
    "--addr",
    "--assert",
    "flow 0x40176f call",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "LoadLibraryA("
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "GetProcAddress"
      }
    ]
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
    "0x401757",
    "--addr",
    "--assert",
    "flow 0x40176f call",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "GetProcAddress"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/bm3.zip.__x/bm3.exe",
    "binary_sha256": "8116e2cb1bbe102beba05abe99c6e41f913e8fad7867c1640fcb002492b3fe79",
    "binary_size": 26624,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The override may change the operation without extending the original return-terminated worklist.

## Refutation

**The filed hypothesis is wrong; the symptom stands.** The override does NOT drop
the pushed continuation. `print raw` under `--assert 'flow 0x40176f call'` shows
`Basic Block 0 0x00401757-0x00401785` — flow resumed at 0x401770, the pushed
continuation, exactly as it should, and ran on to 0x401785.

What is missing is that 0x401785 is *another* link of the same
`push <cont>; push <target>; ret` idiom. It is still a return, so it ends the
flow, and the whole 22-link body behind it is dead and prints as nothing. Two
overrides in one command (`--assert 'flow 0x40176f call' --assert 'flow 0x401785
call'`) already produced `GetProcAddress(LoadLibraryA(s_40151e),s_40152b)` before
this change — the gap was that nothing walks the chain, and nothing in the output
says the next link exists.

## Reference

- `ida-decompile load ./target/bm3.zip.__x/bm3.exe` — The reference server exited before registering. This is an infrastructure failure, not evidence that IDA also fails to recover the continuation.

## Instances

- `5ab77f5c33c5d40ad448c681` (round 9, tester t-r9-5ab77f5c)

## Decision log

- round 9 CAPTAIN, at dispatch of `b-r9-flow-call-overri` (04:04Z). Three corrections to the sibling-contract block in that builder's prompt, which was composed 16s before this was written and is stale in one row.

1. `prototype-parser-rejects-previously` IS NOT RUNNING. It merged as PR #501 at 03:58Z and released every lease. `decompiler/crates/kuna-cli/src/assertdecl.rs` and `decompiler/crates/kuna-console/src/grammar` are FREE -- take them, do not report `blocked` on them. The live lease table is authoritative over the prompt text; check it (`python3 -m scripts.repipe.status --json`) before believing any row. Rebase onto `origin/main` at `3e11d193` or later: the worktree was cut from LOCAL main `8df2a52e`, which is two commits behind and does not contain #501 or #492.

2. HARD CONSTRAINT -- NO NEW OPTION THIS ROUND. `b-r9-bcryptgetpropert` holds `counter:catalog`, `counter:div`, `counter:stages-corpus`, `file:docs/options.md` and `file:phases.toml`. You may not add a settableTable row, edit docs/options.md, or bump a catalog/DIV/stages count; two option-adding PRs collide on those hard-coded counts. The intended shape here is a correctness fix, not a feature: the user typed `--assert 'flow 0x40176f call'` and kuna answered `applied`, so making that answer true needs no flag. If your diagnosis genuinely requires an option, stop and report blocked rather than racing for the counters.

3. THE SAME GAP, MEASURED ON A DIFFERENT BINARY. Refuting the sibling need `beria-call-fragments-produce` last tick produced what is probably this defect from another challenge. `kuna decompile .kuna-repipe/arena/9/5ab77f5f33c5d40ad448c834/target/crackme1.zip.__x/crkme.exe 0x40d120 --addr --assert 'flow 0x40d1ae branch'` drops the body from 76 lines to 42: the whole post-override continuation vanishes and the function degenerates to `v3 = sub_44ac6b(); return v3;`, losing a [0x40f068] structure walk, a resolver dispatch through `*(void**)&v5[0xd4]` and an `*v4 != 0xcc` int3 test. Adding `--option tailcalljump off --option tailcallframe off` restores 68 correct lines; those two flags alone, without the override, change nothing. Read that as two halves: applying a flow override does not extend the decode worklist with the continuation, AND tailcalljump/tailcallframe (both default ON) can convert the resumed edge into a tail call that ends the function. Your `call` form at 0x40176f -> continuation 0x401770 looks like the same mechanism. Advisory, not binding -- verify before building on it, same as the filed `## Hypothesis`.

4. DO NOT CLOSE THIS BY GREP. The acceptance clause (`code` contains `GetProcAddress`) is a positive witness, which is the right shape and better than most of this round's needs -- but your PROBE also asserts `contains LoadLibraryA(`. A fix that re-roots or truncates the function can satisfy the acceptance while emitting different wrong code. Diff full before/after output at 0x401757; do not grep alone. If the same change also fixes the 0x40d120 case above, add it as a second test and say so in the PR -- one mechanism closing two blockers is worth the paragraph.
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). CAPTAIN REFUTATION (round 9, in-tick, release kuna @ local main 8df2a52e; binary .kuna-repipe/arena/9/5ab77f5c33c5d40ad448c681/target/bm3.zip.__x/bm3.exe).

VERDICT: the filed hypothesis -- 'the override may change the operation without extending the original return-terminated worklist' -- is UPHELD almost verbatim, and it is now located in the source. But the need's own framing ('no continuation-address control was found') is WRONG in a way that matters: the control exists, is named exactly for this, and does not work.

1. THE OBFUSCATION, FROM THE DISASSEMBLY. This is push/ret, twice. Fragment 1 ends 0x40176f RET; fragment 2 is 0x401772 PUSH 0x40152b (arg) / 0x401779 PUSH EAX (arg, the module handle) / 0x40177a PUSH 0x401786 (the CONTINUATION) / 0x40177f PUSH [0x401102] (the TARGET, GetProcAddress) / 0x401785 RET. The RET pops the pushed target and jumps to it; the callee returns to the pushed 0x401786, which does MOV [0x40151a],EAX -- the store of the resolved proc address. So 'the pushed continuation is the next instruction 0x401770' is right, and the whole rest of the function hangs off it.

2. THE FOUR-WORD CONTROL TABLE. Same command, only the flow word varies, entry 0x401757:
  flow 0x40176f call        -> LoadLibraryA(s_40151e);            [4 lines, no GetProcAddress]
  flow 0x40176f callreturn  -> LoadLibraryA(s_40151e);            [4 lines, BYTE-IDENTICAL to call]
  flow 0x40176f branch      -> LoadLibraryA(s_40151e); // jump-as-call
  flow 0x40176f return      -> REFUSED ('Could not apply flowoverride'), body unchanged
  default (no override)     -> void sub_401757(void) { return; }  [nothing at all]
The override is genuinely applied and genuinely helps -- LoadLibraryA appears only with it. It just stops there.

3. THE FINDING THE TESTER MISSED, AND IT REFRAMES THE NEED. 'callreturn' IS a documented flow word (assertdecl.rs:311 accepts branch|call|callreturn|return) and its name promises precisely 'a call that returns'. It produces output byte-identical to 'call'. Worse, it is semantically the OPPOSITE of what is wanted: funcdata_op.rs:2365 shows CALL_RETURN rewrites the op to CALLIND and then INSERTS AN ARTIFICIAL RETURN AFTER IT, i.e. 'call, then end the function'. So kuna ships a continuation-shaped control that guarantees no continuation. That is the sharpest statement of this need: not 'no control was found' but 'the control that exists is inert here, and the one named for it does the opposite'.

4. WHERE IT IS, IN TWO SITES. (a) Funcdata::override_flow (substrate/funcdata_op.rs:2329) only REWRITES THE OPCODE of an already-decoded dead op -- RETURN -> CALLIND for call/callreturn. It touches no worklist. (b) The call site is INSIDE the flow walk (p2_lift/flow.rs:1511), and the very next statement is xref_control_flow(firstop, startbasic, &mut isfallthru), whose dispatch (flow.rs:1129) matches on the CURRENT, post-rewrite opcode -- so the CALLIND arm IS reached -- and flow.rs:1522 pushes curaddr+step onto self.addrlist only when isfallthru comes back true. Empirically 0x401770 is never decoded, so isfallthru is false for the override-created CALLIND. THAT is the one fact to establish first: does the CPUI_CALLIND arm fail to set isfallthru, or is it set and then cleared (no-return inference on an indirect call whose target is a pushed constant, or the is_funcbound_fallthru halt at flow.rs:1531)? Do not write a fix before answering it -- the two answers have different fixes.

5. THE MACHINERY IS PROVEN TO WORK ONCE THE ADDRESS IS REACHED. Decompiling the second fragment DIRECTLY -- entry 0x401772, --assert 'flow 0x401785 call' -- yields 'dat_40151a = GetProcAddress(); if (!dat_40151a) return;'. So RET-as-call recovery, the import name and the store all already work. The single missing thing is the fall-through edge that would have brought the walk to 0x401772 in the first place. Note GetProcAddress() prints with NO arguments even there, because EAX is undefined at a fragment entry -- which is the tester's 'loses cross-fragment state', and it is a SECOND, downstream defect that fixing the edge should also resolve. Check it: after the fix, the arguments should appear at 0x401757 because EAX now carries LoadLibraryA's result.

6. THE CROSS-BINARY LEAD I FILED AT DISPATCH IS PARTLY REFUTED -- DO NOT SPEND TIME ON IT. In the Decision log entry above I pointed at the beria sibling, where tailcalljump/tailcallframe off was needed alongside the override. Measured here: --option tailcalljump off --option tailcallframe off changes NOTHING on this binary, with or without the override (all four runs identical). The two needs share the symptom 'the continuation after an override is lost' but NOT the second half of the mechanism. Treat them as related, not identical; a fix here need not touch tail-call recovery.

7. SCOPE. Unchanged at small, and no new option: this is a correctness fix to a control the user explicitly asked for and kuna reported 'applied'. Reachable without touching phases.toml, which matters this round because those counters are leased.
