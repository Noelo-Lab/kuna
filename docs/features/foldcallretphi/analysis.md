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

The collision it forgives is about versions, not order: the operand's other live
version is written by the call itself, and the folded rendering reads the operand
inside the call expression, which is the same point at which that write happens.

Order is the discount's own problem, because the fold it enables moves the call's
evaluation to wherever the expression is printed. Three conditions:

* a high that belongs to a `VariableGroup` declines, because `inflate_test`'s
  second loop reasons about overlapping storage rather than versions;
* an op that itself reads an INDIRECT effect of the call declines, so the folded
  text never names the operand's high as both the call's argument (pre-call) and
  an operand of the reading op (post-call). Both the single use op and the
  statement the expression finally lands in are tested: with an implied chain
  longer than one hop those are different ops, and the landing op's other
  operands are just as exposed;
* the whole span from the call to that landing statement has to be clear — same
  block, no CALL/LOAD/STORE/CALLOTHER, and no write to a global.

### The barrier the opcode set misses

`foldcallret`'s span guard is stated in opcodes, and "writes something the callee
can read" is not an opcode. Heritage promotes a write to a fixed address into a
plain `CPUI_COPY`, which the guard waves through:

```
target: push rbx / mov ebx,ok(%rip) / mov rdi,g(%rip) / call helper
        / movl $42,k(%rip)          <-- a global write; helper returns k
        / and eax,ebx / mov ok(%rip),ebx / pop rbx / ret
```

Before this guard, `--option foldcallretphi on` emitted

```c
  v1 = dat_40403c;
  dat_404038 = 0x2a;
  dat_40403c = v1 & helper(dat_404030);   /* helper now runs after k = 42 */
```

while the binary — and `off`, and plain `origin/main` — compute `v1 & helper(g)`
with the *old* `k`. Compiling both renderings gives `ok=1` against `ok=42`.

`op_writes_tied_storage` declines any op in the span whose output varnode is
address-tied. Built without it and otherwise identical, that binary folds again
and the stage test's assertion #5 fails; with it, `on` is identical to `off`.

A frame slot is in the barrier for the same reason, and the first version of this
guard was wrong to leave it out on the argument that an escaped slot stays a
`CPUI_STORE`. It does not. Given

```
target4: sub rsp,0x20 / movl $1,8(%rsp) / mov ebx,ok(%rip)
         / lea 8(%rsp),%rdi / call helper2      (helper2 returns *p)
         / movl $42,8(%rsp) / and eax,ebx / mov ebx,ok(%rip)
         / lea 8(%rsp),%rdi / call helper2 / mov eax,k(%rip) / ...
```

`print raw` shows the escaped slot promoted exactly like a global —
`0x1020:18: s0xffffffffffffffe0:4 = #0x2a:4`, a `CPUI_COPY`, with the call
carrying its own INDIRECT over the same slot. So the barrier tests
`is_persist() || is_addr_tied()`, and the price is a false positive: a frame slot
the callee could not have reached also declines. (That particular listing folds
through default-on `foldcallret` with no discount involved — `on` and `off` emit
the same thing — so it is GH-657's family, not this option's.)

The frame half of the barrier costs nothing and is worth something. Re-swept with
it, all three metrics report the same numbers to the digit — 1,270 folds located,
718 mapped, 0 hazards, 9 event-set and 40 event-order diffs — and five binaries
change, in one shape each: a local assignment that had moved across a `strlen`
call goes back where the `off` arm puts it (`du`, `diff` ×2, `find`, `grep`,
`tar`: `v11 = '\0'; v10 = strlen((char *)v9);` becomes
`v10 = strlen((char *)v9); v11 = '\0';`). Neither `evalorder.py` nor
`foldmove.py` calls a write to a local an event, so those six reorders were in
nobody's hazard count — the barrier removes them structurally instead.

The same hole is reachable through default-on `foldcallret` alone, when the call
needs no discount at all (`v = f(7); glob = 42; use(v)`): that is GH-657, fixed
separately because it moves default output.

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

`decompile-all` off vs on over 24 binaries -- x86-64 and ARM Cortex-M, at O0,
O2 and O2-noinline. (The corpus's O0 and O2 betaflight images are byte-identical,
so that pair counts twice.)

| binary | functions | changed | declared locals | statements |
|---|---|---|---|---|
| O2 fmt | 151 | 2 | 369 → 367 | 2999 → 2995 |
| O2 ls | 404 | 4 | 1355 → 1353 | 10399 → 10386 |
| O2 sort | 343 | 3 | 1153 → 1152 | 8625 → 8620 |
| O2 du | 320 | 6 | 1038 → 1033 | 8450 → 8439 |
| O2 grep | 449 | 14 | 1659 → 1650 | 14503 → 14485 |
| O2 gzip | 204 | 6 | 864 → 860 | 7528 → 7515 |
| O2 bzip2 | 114 | 3 | 610 → 607 | 8623 → 8622 |
| O2 diff | 398 | 13 | 1592 → 1583 | 12480 → 12464 |
| O2 find | 658 | 31 | 2223 → 2207 | 15965 → 15941 |
| O2 tar | 1125 | 69 | 5325 → 5300 | 46089 → 45994 |
| O0 fmt | 191 | 3 | 303 → 300 | 2040 → 2036 |
| O0 du | 441 | 13 | 992 → 980 | 6518 → 6490 |
| O0 grep | 642 | 26 | 1648 → 1636 | 10294 → 10261 |
| O0 tar | 1570 | 120 | 5203 → 5131 | 34812 → 34652 |
| O2-noinline fmt | 201 | 3 | 329 → 327 | 2832 → 2826 |
| O2-noinline ls | 598 | 10 | 1211 → 1206 | 9744 → 9728 |
| O2 betaflight (ARM) | 5797 | 171 | 11085 → 10988 | 96871 → 96694 |
| O2 cleanflight (ARM) | 2558 | 61 | 4627 → 4596 | 43197 → 43141 |
| O2 chibios (ARM) | 398 | 0 | 748 → 748 | 6685 → 6685 |
| O2 crazyflie (ARM) | 1971 | 53 | 6729 → 6697 | 52913 → 52862 |
| O2 nuttx (ARM) | 792 | 14 | 2488 → 2482 | 18765 → 18755 |
| O0 betaflight (ARM) | 5797 | 171 | 11085 → 10988 | 96871 → 96694 |
| O0 chibios (ARM) | 772 | 4 | 686 → 683 | 6591 → 6588 |
| O2-noinline betaflight (ARM) | 6388 | 208 | 11537 → 11411 | 97253 → 97028 |
| **total** | **32282** | **1008** | **74859 → 74285 (-574)** | **621047 → 619901 (-1146)** |

### Hunk classification

Every hunk in the 1008 changed functions falls into one of three classes:

1. **The fold** — a `vN = f(...);` statement disappears and `f(...)` appears at
   its single use. 1270 sites. Example (`grep sub_7e40`):

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
to that. Three metrics, all in this bundle, over 24 binaries -- x86-64 and ARM
Cortex-M, at O0, O2 and O2-noinline, 32,282 functions, 1,008 changed. Both of the
first two flag the global-write counterexample above when it is fed to them, so
they are calibrated against the defect class, not just against nothing:

* `evalorder.py` -- the whole ordered stream of calls and memory touches per
  function, which needs no statement mapping and so covers **every** changed
  function. **No function gains or loses a call anywhere.** Nine functions differ
  in memory-touch count and 40 in event order; all 49 were read by hand (41
  distinct functions -- the betaflight images repeat), and every one is an
  adjacent fold, a load hoisted to its own statement immediately before the call,
  or a decompiler-artifact intermediate store to a global disappearing
  (`dat_x = f(); dat_x ^= 1;` becoming `dat_x = f() ^ 1;`, where the binary
  stores once). All three are the conservative direction.
* `foldmove.py` -- how far each folded call actually travelled: the statements
  between the deleted spill and the statement the call expression landed in.
  This is a text matcher, so it reports its own coverage. 1,270 removed spills
  located, of which it maps 718: **711 land on the very next statement and 7 on
  the one after** (a register or constant assignment in between), and **none
  crosses a call, a memory access or a control-flow boundary.** The other 552
  (43%) are three named buckets it refuses to guess at: 167 where the ON body
  carries no line with the call text (an argument was un-inlined, so the call
  prints differently), 112 where no OFF statement maps onto the ON line inside
  the diff hunk, and 273 where the call text is not unique in the function, so
  which statement is the landing would be a guess. That last bucket is why the
  script no longer classifies them: an older version picked a landing 8 and 74
  statements away in two `tar` functions whose spill text occurs twice and
  reported 65 hazards from them, all false. `evalorder.py` covers the 552 without
  needing a mapping.
* `callorder.py` -- 13 functions differ, all inside those 49: the script orders
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
`decompile-all` is unchanged at the minimum (21 interleaved pairs: 4.200 s off,
4.200 s on; median 4.750 s vs 4.810 s, +1.26%, on a box at load 22 where the
per-sample spread, 4.20-5.93 s in both arms, is wider than the delta), inside
the +5% budget.

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
