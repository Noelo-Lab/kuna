# protoscc — recursive functions state their recovered types

## The gap

`protoorder` (#669) decompiles a whole binary callees-first and lets each callee
state the parameter types its own recovery found, which its callers take as a
vote. Inside a call-graph cycle there is no callee-first order, so
`callee_first_plan` parked nothing for a member of a recursive component — a
function that calls itself, or a set of functions that call each other. The type
chain therefore stops at the first recursion, and much of coreutils' and gnulib's
character-level work bottoms out in recursive functions: `quotearg_buffer_restyled`
calls itself, `copy_internal` and `copy_dir` call each other.

cp -O0, stripped (`decompile-all`, main at 2888a2a94):

```
void sub_a6db(unsigned long a0,unsigned long a1,long a2)        # emit_verbose
void * sub_185de(int a0,unsigned long a1,unsigned long a2,struct_0 *a3)   # quotearg_n_options
```

`sub_17254` (`quotearg_buffer_restyled`) is recovered with `char *a2`; its callers
never hear about it. With `--option protoorder cycles` (this branch's default):

```
void sub_a6db(char *a0,char *a1,char *a2)
void * sub_185de(int a0,char *a1,unsigned long a2,struct_0 *a3)
```

The ranked-lever census on `origin/feat/charptr-on`
(`docs/features/charptr/next-levers.md`) put this at +36..+46 perfect functions and
+163..+188 `char *` true positives with one function worse, from an env-gated
experiment that simply let cycle members park in address order.

## The design question

A member of a cycle decompiled first states types it recovered without its
partners' statements. Three answers were built and measured on the same base:

| arm | order inside a component | second round | perfect | aggregate | improved / worse | bash -O2 cost |
|---|---|---|---|---|---|---|
| main (`types`) | — | — | 1,353 | 3669.96 | | |
| experiment `all` (ce008ce2b) | address | no | +46 | +43.72 | 160 / 1 | |
| `cycles`, one pass | depth-first from the entered members | no | 1,399 (+46) | 3714.21 (+44.26) | 162 / 1 | -0.9% min, +0.3% median (interleaved min-of-15) |
| `cycles` + re-pass | same | members that called a later partner, once | 1,399 (+46) | 3714.60 (+44.65) | 167 / 1 | +33..+37% (single runs, 108.9 s / 112.4 s -> 149.5 s) |

The second round buys 0.39 aggregate and five more improved functions (none
reaching a perfect score) and costs about +35% on bash -O2, whose parser and command
executor are the two largest members of one component and each decompiled twice
(12 s and 11 s per decompile on this box). It is not taken.

The order that is taken: a depth-first walk over the component's own edges that
emits each member after the partners it reaches, started from the members
something outside the component calls. The member the component's callers read is
then decompiled after the partners it reaches, so what it states includes theirs.

## What does not change

- Neither value locks, so no call gains an argument; the one argument a call can
  lose is the trailing clobbered one `argclobber` drops (below), which under
  `cycles` includes a call to a recursive callee (`rtarget(a0,5)`). Over 15
  binaries both values render the same 90,420 call arguments and 18,556
  `variables[]` argument rows (`callsite.json`).
- `lock` still declines a recursive component: a parked prototype is declared, so a
  first-decompiled member would lock its partners' calls to a list recovered
  without them.
- Every refusal of the vote (frame objects, float/composite pointees, storage,
  MIPS/ARM float-register rules, declared prototypes, `--assert`) is the same code.
- `--jobs N` takes no callee-first order at all (a worker cannot see another
  worker's callees), so it states nothing under either value; the pool's
  serial-replay contract is still "`--jobs 1 --option protoorder off`".

## The structsynth convergence sweep

The sweep decompiles again every result that names a structure a later, larger
layout superseded (`converge_callee_first`). Under `types` a recursive function
states nothing, so no redo can read a stale statement of its own. Under `cycles`
it could: tar -O2 `make_hol` (`sub_3b0c0`) was redone onto `struct_59 *a0` and read
its own first-pass statement at its recursive call, so it passed
`(struct_54 *)*v22`, the superseded record, while its own prototype takes
`struct_59 *`. A cycle member redone before a partner has the same
exposure to the partner's stale statement.

Two rules close it. `seed_protoorder_types` never offers a function its own
statement, and the sweep first forgets every stated list that names a superseded
structure (`forget_statements_naming`); a redone callee states again before its
redone callers. Rebuilding the table in plan order, so that a redo reads exactly
what its first decompile read, was tried first and rejected: under `types` it moved
tar -O0 `sub_3854d`, whose callees the call graph plans after it, from
`sub_34a6f("-f")` to `sub_34a6f(0x8746d)`. The rule taken keeps `types`
byte-identical to main.

Checked on 26 binaries (the 15 below plus mv, dash, gzip, bzip2, scp and e2fsck at
-O0 and ginstall, dash, tar, xmlwf and crazyflie at -O2): no result names a
superseded structure after the sweep under either value, no call argument is cast
to a `struct_N *` other than the callee's own parameter where `types` has none
(`r3c-stale.txt`), and `types` is byte-identical to main on all 26. Regression
fixture `protoorder_cyclestruct_x86_64`.

## argclobber

`argclobber` (#689) drops a clobbered trailing argument only when the callee's
stated list accounts for every surviving argument and its body neither reads nor
forwards the register. A recursive callee now states a list, so it becomes
evidence there too. The body walk (`resolve_forward_transfer`) is what answers for
a member whose list is short because it hands a register on to a partner: it
follows the partner's body, and a cycle it re-enters is denied, so a register not
written before the recursive call can still reach a read and the drop declines.

Checked on:

- every existing counterexample (the seven `docs/features/argclobber/ce-*`
  builds, and the four committed `argclobber_*` fixtures): byte-identical output
  under both values;
- `ce-rec-forward.c` (the two-parameter forwarding thunk made self-recursive),
  `ce-rec-mutual.c` (a two-member cycle handing its registers to the thunk) and
  `ce-rec-pair.s` (a member that never names rdx, a partner that reads it first):
  under `cycles` the short lists are stated (`state sub_40102b params=2`) and every
  caller keeps its third argument;
- the fixture `protoorder_cycles_x86_64`: `rtarget` writes rdx (`cqo`) before
  calling itself, so its caller's clobbered argument is dropped (`rtarget(a0,5)`);
  `rkeep` forwards rdx into its own recursion and `rkeep(a0,5,v3)` stays.

Across the 15-binary corpus `argclobber` changes the same one function under both
values (fmt -O2 `main`).
