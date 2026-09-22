# passthrough — default-on evaluation

Verdict: **hold at default off (opt-in).** The failing criterion is return
fabrication: the tail-return arm gives 23 functions a return value DWARF says
they do not have, and `type_match` cannot see a return type. The argument arm
passes every criterion measured.

All numbers are commit `7da4afecf`, one binary, `--option passthrough on` against
the shipped default, over whole-binary `decompile-all` (the option is inert
anywhere a callee was not decompiled first).

## What passes

- **make test**: PARITY OK 675/675. **make test-stages**: PARITY OK 1299/1299.
  **make test-cli**: 221/221. The console decompiles no callee first, so the
  datatest and stage corpora cannot move under either default.
- **444-slice typesweep** (pinned decbench 625e892, `DECBENCH_NO_CACHE=1`):
  PERFECT 1,353 -> 1,377 (**+24**), **0** moved off perfect; aggregate
  3,669.96 -> 3,728.72 (+58.76); improved 102, worsened 6; control 10,469
  byte-identical-variables functions, 0 scored differently. The 6 worsened are
  three functions in two slices each, and each is a type the callee's own
  recovery hands up: `print_group_list` (groups and id O2) `username` and
  `print_stuff` (id O2) `pw_name` go `char *` -> `long`, because the forwarded
  parameter now also reaches the mgetgroups copy kuna recovers as `long a0`,
  and `copy_reg` (cp/ginstall/mv O2-noinline) `dst_dirfd` goes `int` ->
  `unsigned long` the same way.
- **Arguments, against DWARF** (26 binaries: O0/O2/O2-noinline coreutils, gzip,
  grep, tar, diffutils, bzip2, findutils, shadow; betaflight, crazyflie, nuttx,
  u-boot): 259 functions gain 506 parameters, **454 confirmed, 0 contradicted**,
  52 with no DWARF twin (u-boot 20, betaflight 12). No function loses one.
- **Call sites** (`docs/decbench/typecampaign/final/callsite_metrics.py`):
  **0 arguments deleted**, 724 added at 427 sites; phantom arguments in the
  changed functions 1 -> 0 (817 -> 816 over whole bodies); 0 new over-arity
  groups, 77 fixed; no call site count changes.
- **Lost arguments** (`lostargs.py`, a call against a callee whose own arity is
  unchanged, or a function's own list): 0 and 0.
- **Whole-corpus diff**: 1,179 functions change, every one a gained parameter
  or tail-call return, calls gaining arguments, or types propagating from them
  (`record.json` `corpus_diff`). The option-off arm is byte-identical, text and
  `--json`.
- **Speed** (interleaved min-of-15, off vs on, whole-binary `decompile-all`):
  fmt -2.78%, ls -0.28%, sort -0.11% (re-run alone; +7.50% the first time,
  measured while this lane's workspace suite was building), bash +3.97%; worst
  +3.97%, within +5%.

## What fails

The tail-return arm hands a wrapper the return value its callee is recovered
with, and when that recovery is wrong the wrapper inherits the error: 281 gained
returns agree with DWARF, **23 do not** (16 gnulib `version_etc_ar` copies and 2
`version_etc_hook`, whose callee `version_etc_arn` is `void` but ends in a call
whose `rax` kuna reads as its result; 2 tar `print_total_stats`; 3 on ARM), 14
have no twin. `type_match` scores `variables[]` and never a return type, so a
default flip would add those 23 with nothing on the metric to offset them, and
preset membership (`auto` picks `aggressive` under 500 KiB) would reach every
small binary.

What would let it flip: a gate on the return arm that refuses a callee whose own
return is a value it did not compute (a call result it hands back), or shipping
the argument arm alone by default.

## History

The first measured cut also fabricated ~55 register parameters on ARM firmware
and could delete call arguments (a claimed register that reached the call through
an earlier call scored "killed by call" and `forceNoUse` dropped every argument
after it; an argument supplied while scoring turned off the `calleearityfwd`
sibling rescue). Claiming only at a call no other call precedes, leaving claimed
trials unscored, and extending the settled list last removed all three.
