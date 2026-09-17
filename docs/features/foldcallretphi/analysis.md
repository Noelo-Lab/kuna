# foldcallretphi — why an order-safe call result still gets a declaration

## The witness

coreutils `fmt` -O2 (stripped), `main` @0x26a0:

```c
      else {
        v3 = sub_3700(stdin,v7);
        v1 = 1;
        v12 &= v3;
      }
...
  v2 = sub_41f0(stdin);
  if (v2) {
```

`v3` and the `sub_41f0` result are each defined by a call and read exactly once,
in the same basic block, a couple of ops later, with nothing between them that
touches memory. `foldcallret` is on by default and its order-safety predicate
(`kuna_callretfold.rs (call_output_foldable)`) accepts both. They still spill.

## What actually blocks them (instrumented build, `KUNA_DBG_FCR=1`)

The E2 report's hypothesis was that `ActionMergeRequired` runs before
`ActionMarkExplicit`, so a force-merged call output fails `num_instances() > 1`
at `coreaction_cleanup.rs (base_explicit)` even though the `is_call()` arm let it
through. **On these witnesses that is not what happens.** An env-gated trace in
`base_explicit` reports, for every call output in `fmt::main`:

```
FCR call-out def@298a reason=OK ninst=1 tied=false mapped=false protopart=false nodesc=false
FCR call-out def@2a4d reason=OK ninst=1 tied=false mapped=false protopart=false nodesc=false
```

Both pass `base_explicit` entirely. The rejection is one pass later, in
`ActionMarkImplied`:

```
INFLATE-HIT inst[16] b_defcode=CPUI_INDIRECT b_defaddr=298a
CIC inflate-reject def@298a input offc090(stdin) idef=CPUI_INDIRECT@29b8 ininst=46
MI call-out def@298a -> EXPLICIT(cic)
```

`check_implied_cover`'s third arm walks the call's non-constant operands and asks
`Merge::inflate_test` whether the operand's Cover can be inflated to the
candidate's without colliding with another instance of the operand's own
HighVariable. The operand here is the global `stdin` (ram 0xc090), whose high has
46 SSA versions, and the colliding instance is **the output of the
`CPUI_INDIRECT` that the call at 0x298a attaches to `stdin` itself** — a call may
write any global, so every call carries such an INDIRECT for every global it
reads or might touch.

Upstream never reaches this arm with a call output: Ghidra's `base_explicit`
returns `-1` for `op->isCall()` unconditionally, so the arm's call-output half is
live only because of kuna's `foldcallret` escape. It is being asked a question it
was not written for, and it answers with a conflict the fold itself creates.

## The control

Two fixtures differing in one instruction (`tests/stages/kuna-foldcallretphi.xml`
and the control described in its header):

| argument | pass 1 (`foldcallretphi off`) | pass 2 (`on`) |
|---|---|---|
| `mov rdi,[rel g]` (a global) | `dat_1058 = helper(dat_1050);` then `dat_1058 = v1 & dat_1058;` | `dat_1058 = v1 & helper(dat_1050);` |
| `mov edi,7` (a constant) | `dat_1058 = v1 & helper(7);` | identical |

With a constant argument plain `foldcallret` already folds. Passing a global is
the whole difference.

## Census — how often each gate fires

`decompile-all` over six -O2 binaries with the instrumented build, counting call
outputs that reach `base_explicit`:

| binary | call outputs | pass `call_output_foldable` | folded | blocked by the self-INDIRECT inflate | blocked by the cover-crosses-a-call arm | blocked by `num_instances()>1` |
|---|---|---|---|---|---|---|
| fmt | 206 | 99 | 80 | 4 | 4 | 11 |
| ls | 888 | 449 | 392 | 33 | 5 | 19 |
| sort | 687 | 323 | 268 | 11 | 9 | 35 |
| du | 729 | 334 | 284 | 19 | 4 | 27 |
| grep | 963 | 453 | 366 | 33 | 6 | 48 |
| gzip | 423 | 159 | 138 | 17 | 2 | 1 |

(The remaining call outputs are declined by `call_output_foldable` itself: more
than one use, a phi use, or an intervening call — correctly, and untouched here.)

## The change, and why it is sound

Only the first blocked column is taken. `kuna_foldcallretphi.rs
(conflict_is_self_call_effect)` re-walks the instances `inflate_test` just
rejected on and discounts the rejection when *every* colliding instance is an
INDIRECT effect of the call being folded.

The argument rests on `foldcallret`'s predicate, which this change does not
weaken: the call and its use sit in one block with no intervening
CALL/LOAD/STORE/CALLOTHER, so nothing between them can write the operand's
storage. The only writer is the call, and in the folded rendering the operand is
read inside the call expression — the same point at which that write happens.
Two extra conditions close the remaining gaps:

* a high that belongs to a `VariableGroup` declines, because `inflate_test`'s
  second loop reasons about overlapping storage rather than versions;
* a use op that itself reads an INDIRECT effect of the call declines, so the
  folded text never names the operand's high as both the call's argument
  (pre-call) and an operand of the use (post-call).

## The second ceiling, measured and deliberately not taken

`num_instances() > 1` does veto 141 otherwise-foldable call outputs across the
same six binaries, which is the E2 hypothesis in its general form. It is not
taken here, because the premise the brief offered for relaxing it — "the extra
instance is only a trim COPY the merge itself inserted" — does not hold on the
corpus. The instance dumps show the opposite: these highs carry 2, 5, 7, 16 or 22
members, e.g.

```
FCR call-out def@4601 reason=OK ninst=22 tied=true mapped=true
    inst[0] def=CPUI_INT_ADD@56dc
    inst[1] def=CPUI_COPY@56d0
    inst[2] def=CPUI_CALL@4544
    inst[3] def=CPUI_CALL@4592
    ...
```

That is a speculative datatype merge reusing one named, mapped local for many
call results. Marking one member implied deletes one of twenty-two assignments to
a variable that is still declared and assigned elsewhere, which is exactly what
upstream's rule forbids. A sound version of that relaxation would have to mark
*every* member implied at once and prove the high then needs no declaration; that
is a separate change with a separate risk surface.

## Corpus sweep

`decompile-all` off vs on over the same six binaries, 1871 functions:

| binary | functions | changed | call-spill statements | declared locals | lines |
|---|---|---|---|---|---|
| fmt | 151 | 2 | 84 → 80 | 369 → 367 | 4069 → 4063 |
| ls | 404 | 4 | 308 → 294 | 1355 → 1353 | 13677 → 13661 |
| sort | 343 | 3 | 267 → 260 | 1153 → 1152 | 11376 → 11368 |
| du | 320 | 6 | 320 → 307 | 1038 → 1033 | 10990 → 10973 |
| grep | 449 | 14 | 385 → 364 | 1659 → 1650 | 18257 → 18228 |
| gzip | 204 | 7 | 195 → 184 | 864 → 860 | 9358 → 9339 |
| **total** | **1871** | **36** | **1559 → 1489 (−70)** | **6438 → 6415 (−23)** | **−95** |

### Hunk classification

Every hunk in the 36 changed functions falls into one of three classes:

1. **The fold** — a `vN = f(...);` statement disappears and `f(...)` appears at
   its single use. 70 sites. Example (`grep sub_7e40`):

   ```diff
   -  unsigned long v1; // rax
   -  v1 = fwrite_unlocked(a0,1,a1,stdout);
   -  if (a1 == v1)
   +  if (a1 == fwrite_unlocked(a0,1,a1,stdout))
        return;
   ```

2. **An un-inline** — a value that used to be printed inside an expression gains
   its own statement, at the position its defining op already occupied. 5 sites,
   all a knock-on of `Merge::mark_implied` re-dirtying operand Covers. Example
   (`du main`): `v = sub_6260(dat,v[0xe],v[0xf]); if (0 < v)` becomes
   `v = v[0xf]; v = v[0xe]; if (0 < (int)sub_6260(dat,v,v))` — the two array
   loads are printed where their LOAD ops are, and the call folds into the `if`.
   This is the conservative direction: un-inlining never moves an evaluation, it
   only stops moving one.

3. **Renumbering** — `vN` indices shift when a declaration disappears.

### No call crossed another call

The decisive check for a pass that moves evaluation points: extract each
function's call sequence in evaluation order (nested calls first, since C
evaluates arguments before the call) and compare off vs on.

```
functions compared=1871 call-order-changed=0
```

Reproduce with `callorder.py`, kept in this bundle.

## Default

Ships **off**, and the reason is worth recording because it is not safety.

With the default flipped on, both parity gates hold: `make test` 675/675 PARITY
OK (0 assertions change) and `make test-stages` PARITY OK. Speed on `fmt`
`decompile-all` is +1.22% (min of 21 interleaved pairs; the per-sample spread,
4.11-6.12 s in both arms, is wider than the delta), inside the +5% budget.

`make test-cli` does not hold. `tests/cli/no-cli-rename-or-prototype-override`
runs `kuna decompile fauxware authenticate --assert 'type v2 char[16]'
--assert 'name v2 credbuf'`, and those directives address a variable by its
auto-generated `vN` name. Folding `v1 = strcmp(pass,sneaky)` into its `if`
removes a declaration, renumbers the rest, and `v2` then designates a different
stack slot: the probe's `char credbuf [16]` lands at -0x10 instead of -0x18 and a
separate `char v2 [8]` read buffer appears. Nothing is unsound about either
listing, but the user's directive no longer names what they meant. That makes
the flip a change about `--assert` ergonomics, not about this pass, so it belongs
in its own PR with those directives re-read.

The option is metric-neutral either way.

It is metric-neutral by construction and by measurement. decbench scores kuna's
`--json variables[]`, which carries arguments, ScopeLocal stack symbols and
`framelayout` slots but never register locals, and every declaration this removes
is a register local. Measured off vs on: `fmt` 737 variables both arms (1
function's array differs), `du` 1723 both (5 differ), `grep` 2524 both (11
differ) -- and in every one of those the `(type, kind, stack_offset)` multiset is
identical, with only the `vN` numbering and the line/address spans moving.
