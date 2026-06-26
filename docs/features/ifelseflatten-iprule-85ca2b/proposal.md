# [PROPOSAL] angr if-else-flatten (`IfElseFlattener`) + an ET_REL no-return boundary prerequisite

**Opportunity:** `test_ifelseflatten_iprule :: flush_rule` (angr 9.2.213), binary
`x86_64/decompiler/iprule.o` (ET_REL `.o`), function `flush_rule` @ `0x1310`.
**Proposed option name:** `ifelseflatten`.
**Scope (decider verdict):** *large* — this is **not** a single option-gated `Action`/`Rule`. It
must go through human go/no-go before any implementation worker is spent. See `analysis.md` for
the full reproduction and root-cause evidence; `angr-vs-kuna.txt` is the captured side-by-side.

## The problem

angr renders `flush_rule` with an early-return as its first statement:

```c
if (ptr->field_0 - 28 < 0)
    return 4294967295;      // -1, single scope, NO else
parse_rtattr(...);
...
```

The `test_ifelseflatten_iprule` assertion is precisely that the *first* `if` is a single-scope
`if (...) return -1;` with no `else`. This is produced by angr's **`IfElseFlattener`** region
simplifier (`angr/analyses/decompiler/region_simplifiers/ifelse.py`): for a `ConditionNode` whose
true-branch is statement-terminating (all endpoints return/abort) and whose false-branch is not,
it drops the `else` and re-parents the false node as a *follower* of the `if` —
`if(c){...return} else {body}` ⇒ `if(c){...return} body`.

kuna has **no** equivalent structured-tree transform, so it emits the un-flattened
`if (cond) { body } ... return -1` form instead.

## Why this target additionally needs a prerequisite fix (the blocker)

`flush_rule` is *also* mis-recovered: kuna **merges the whole neighbouring function**
`iprule_list_flush_or_save` (0x1410, 2059 bytes) into `flush_rule`'s body (15 neighbour-only call
sites leak in — `__isoc99_sscanf`, `matches`, `rtnl_ruledump_req`, `new_json_obj`, …).

Root cause (verified): `flush_rule`'s canary-fail path ends with

```
1408: e8 00 00 00 00   call __stack_chk_fail   ; R_X86_64_PLT32 __stack_chk_fail-0x4
140d: 0f 1f 00         nopl (%rax)              ; padding
1410: <iprule_list_flush_or_save>
```

`__stack_chk_fail` is no-return, but in this ET_REL `.o` the no-return fact is **not propagated to
the relocation-resolved call site**, so kuna assumes the call returns, walks the `nopl` padding,
and decodes straight into the next function. kuna already has the pieces (`relocobjects`, the
known-no-return list normalizes `__stack_chk_fail`→`stack_chk_fail`, `noreturn_disc` /
`noreturn_propagate`), yet enabling all of them leaves the merge unchanged (15 leaks) — and the
full `scripts.pipeline.compare` reproduces it. So **even a perfect `IfElseFlattener` could not
make `flush_rule` match angr**: it would still be a merged blob.

## Proposed plan (multi-step — hence a proposal)

**Step A — boundary prerequisite (analysis/loader tier).** Make the known/discovered no-return
fact reach relocation-resolved call sites in ET_REL objects so `call __stack_chk_fail` terminates
the flow-follower and `flush_rule`'s extent stops at `0x140d`. This is in `kuna-analysis`
(`s1_loader::noreturn` / `s1_noreturn_propagate` + `relocobjects` interaction), *not* an S2
`Action`/`Rule`. This step is what unblocks `flush_rule` as a discrete function.

**Step B — `IfElseFlattener` port (S7/S8 structuring).** Add a gated structured-tree region
simplifier (option `ifelseflatten`, default-OFF) that, for an `if/else` whose `if`-branch is
statement-terminating and whose `else`-branch is not, drops the `else` and re-parents it as a
follower of the `if`. This is a new structuring-tree pass type, touching S7/S8 region code — over
the single-Action bar by itself.

## Validation target recommendation

`flush_rule` should **not** be the firing-test target while Step A is unlanded. The if-else-flatten
capability is real and recurring; use an already-renderable witness for the firing stage test —
the sibling angr testcases `test_ifelseflatten_clientloop` (`clientloop.o`,
`client_request_tun_fwd`) and `test_ifelseflatten_certtool_common` (`certtool-common.o`) — and
verify whether they suffer the same ET_REL merge before picking. Land Step A first if `flush_rule`
must be the witness.

## Speed / risk assessment

- **Step A** changes function extents → affects every ET_REL `.o` with a tail `__stack_chk_fail`
  (or other no-return) call. Risk: shrinking functions is generally correctness-positive but could
  perturb the datatest corpus; must run the full ablation. Default-OFF until proven.
- **Step B** is an emit-time structured-tree rewrite; risk is localized to `if/else`-with-returning
  branch shapes. Default-OFF; only flip to default-ON if the 675-assertion ablation is clean and the
  decompile speed delta is within the +5% budget. Speed impact expected small (one extra
  tree walk).
- **Anchors:** Step B needs a new module + a structuring registration; Step A needs analysis-tier
  wiring. Combined, this exceeds the one-Action / ≤3-anchor / ≤1-module single-feature contract.

## Decision requested

Human go/no-go on: (1) approving the `IfElseFlattener` port as a gated structuring pass, and
(2) whether to also schedule the ET_REL no-return-propagation prerequisite (required for the
`flush_rule` target specifically; not required for clientloop/certtool-common if those render
discretely). On approval, re-dispatch an implementation worker on this branch.
