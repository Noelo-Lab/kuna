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

`decompile-all` off vs on over 22 binaries -- x86-64 and ARM Cortex-M, at O0,
O2 and O2-noinline. (The corpus's O0 and O2 betaflight images are byte-identical,
so that pair counts twice.)

| binary | functions | changed | declared locals | statements |
|---|---|---|---|---|
| O2 fmt | 151 | 2 | 369 → 367 | 2999 → 2995 |
| O2 ls | 404 | 4 | 1355 → 1353 | 10399 → 10386 |
| O2 sort | 343 | 3 | 1153 → 1152 | 8625 → 8620 |
| O2 du | 320 | 6 | 1038 → 1033 | 8450 → 8439 |
| O2 grep | 449 | 14 | 1657 → 1648 | 14503 → 14484 |
| O2 gzip | 204 | 6 | 864 → 860 | 7528 → 7515 |
| O2 bzip2 | 114 | 3 | 610 → 607 | 8623 → 8622 |
| O2 diff | 398 | 13 | 1592 → 1583 | 12480 → 12464 |
| O2 find | 658 | 31 | 2223 → 2207 | 15965 → 15941 |
| O0 fmt | 191 | 3 | 303 → 300 | 2040 → 2036 |
| O0 du | 441 | 13 | 992 → 980 | 6518 → 6490 |
| O0 grep | 642 | 27 | 1648 → 1636 | 10294 → 10260 |
| O2-noinline fmt | 201 | 3 | 329 → 327 | 2832 → 2826 |
| O2-noinline ls | 598 | 10 | 1211 → 1206 | 9744 → 9728 |
| O2 betaflight (ARM) | 5797 | 171 | 11085 → 10988 | 96871 → 96694 |
| O2 cleanflight (ARM) | 2558 | 62 | 4627 → 4595 | 43197 → 43139 |
| O2 chibios (ARM) | 398 | 0 | 748 → 748 | 6685 → 6685 |
| O2 crazyflie (ARM) | 1971 | 54 | 6729 → 6696 | 52913 → 52858 |
| O2 nuttx (ARM) | 792 | 14 | 2488 → 2482 | 18765 → 18755 |
| O0 betaflight (ARM) | 5797 | 171 | 11085 → 10988 | 96871 → 96694 |
| O0 chibios (ARM) | 772 | 4 | 686 → 683 | 6591 → 6588 |
| O2-noinline betaflight (ARM) | 6388 | 208 | 11537 → 11410 | 97253 → 97027 |
| **total** | **29587** | **822** | **64329 → 63849 (−480)** | **−900** |

### Hunk classification

Every hunk in the 822 changed functions falls into one of three classes:

1. **The fold** — a `vN = f(...);` statement disappears and `f(...)` appears at
   its single use. 705 sites. Example (`grep sub_7e40`):

   ```diff
   -  unsigned long v1; // rax
   -  v1 = fwrite_unlocked(a0,1,a1,stdout);
   -  if (a1 == v1)
   +  if (a1 == fwrite_unlocked(a0,1,a1,stdout))
        return;
   ```

2. **An un-inline** — a value that used to be printed inside an expression gains
   its own statement, at the position its defining op already occupied, a
   knock-on of `Merge::mark_implied` re-dirtying operand Covers. Example
   (`du main`): `v = sub_6260(dat,v[0xe],v[0xf]); if (0 < v)` becomes
   `v = v[0xf]; v = v[0xe]; if (0 < (int)sub_6260(dat,v,v))` — the two array
   loads are printed where their LOAD ops are, and the call folds into the `if`.
   This is the conservative direction: un-inlining never moves an evaluation, it
   only stops moving one.

3. **Renumbering** — `vN` indices shift when a declaration disappears.

### What the folded calls moved past

A pass that moves evaluation points has to be checked against more than
call-to-call order, because a call reordered against a memory read is invisible
to that. Three metrics, all in this bundle, over 22 binaries -- x86-64 and ARM
Cortex-M, at O0, O2 and O2-noinline, 29,587 functions, 822 changed:

* `foldmove.py` -- for each folded call, which statements stood between the
  deleted spill and the statement the call expression landed in. 705 folds
  located; 696 land on the very next statement and 8 on the one after (a
  register or constant assignment in between). One landing the text matcher
  could not map (crazyflie `sub_8035ebc`) was read by hand: adjacent. **No fold
  crosses a call, a memory access or a control-flow boundary.**
* `evalorder.py` -- the whole ordered stream of calls and memory touches per
  function. **No function gains or loses a call anywhere.** Eight functions
  differ in memory-touch count and 36 in event order; all 44 were read, and
  every one is an adjacent fold, a compound assignment merged into its own
  statement, or an operand hoisted to its own statement immediately before the
  call, which is the conservative direction.
* `callorder.py` -- 13 functions differ, all inside those 44: the script orders
  a multi-clause condition innermost-first and a fold re-nests it. The calls
  keep their order in every one.

### The cross-block escape this guards against

The first version of the discount fired on the order-safety of the *use*, and
that is not where the call is printed. `call_output_foldable` guards the span
from the call to its single use; when the use op's own output is implied, the
expression keeps travelling and lands wherever that implied value is finally
consumed, which can be a later block behind a branch. On
`betaflight_STM32F405.elf` `sub_8051ac4` the call `sub_80515b4(dat_200181a4)`
came out *after* `if (dat_200019cc & 1)`, while the binary calls at `0x8051b00`
and first reads `0x200019cc` at `0x8051b0e` -- a call moved past two global reads
it may itself write.

`print_point` follows the implied chain to the statement the expression really
lands in, and `print_point_is_order_safe` re-runs the span guard over that whole
distance. A build with that guard bypassed reproduces the hunk exactly; with it,
the function is byte-identical to `--option foldcallretphi off`.

## Default

Ships **off**, for two reasons.

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

The second reason is the pass itself: it changes where a call is evaluated, and
that is sound only while the guards above hold. They have a corpus sweep behind
them and no default-on mileage, so a flip should re-establish the claim with its
own evidence rather than inherit it from here.

The option is metric-neutral either way.

It is metric-neutral by construction and by measurement. decbench scores kuna's
`--json variables[]`, which carries arguments, ScopeLocal stack symbols and
`framelayout` slots but never register locals, and every declaration this removes
is a register local. Measured off vs on: `fmt` 737 variables both arms (1
function's array differs), `du` 1723 both (5 differ), `grep` 2524 both (11
differ) -- and in every one of those the `(type, kind, stack_offset)` multiset is
identical, with only the `vN` numbering and the line/address spans moving.
