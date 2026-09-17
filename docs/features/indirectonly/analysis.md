# indirectonly — analysis

## 1. The gap

`varnode_flags::indirectonly` has two readers in the merged tree and **no writer**:

| site | what it does with the flag |
|---|---|
| `p6_variables/variable.rs` `HighVariable::has_name` (~:965) | an *unaffected input* whose every member is `indirectonly` is refused a name |
| `p6_variables/merge.rs` `Merge::merge_test_adjacent` (~:623) | a speculative merge involving an **illegal input** is refused *unless* that input is `indirectonly` |

Both are ported verbatim from upstream (`variable.cc:731-760`, `merge.cc:185-196`).
The writer is `Funcdata::markIndirectOnly` / `Funcdata::checkIndirectUse`
(`funcdata_varnode.cc:801-858`), which was never ported:
`ActionMarkIndirectOnly::apply` in `p6_variables/coreaction_cleanup.rs` was a
documented no-op with a `STUB(W7/W8-funcdata)` comment, although the action was
already **scheduled** — `infra/universalaction.rs` puts it one slot ahead of
`ActionMergeAdjacent` and well ahead of `ActionNameVars`, exactly where upstream
puts it. So `is_indirect_only()` answered `false` everywhere and both readers
took the strictly-more-conservative branch.

## 2. What an illegal input is, and why the exception exists

`ActionDirectWrite` marks every Varnode whose value a formal parameter can be
responsible for. An **illegal input** is an input Varnode it never reached:
nothing the caller passes can explain the value. On a normal frame that is the
leftover contents of a stack slot the function overwrites before it reads — a
genuinely uninitialized value.

`merge_test_adjacent` refuses to merge such a variable with anything speculative,
and that refusal is the right default: merging an uninitialized value into a real
variable is the canonical *"a value is printed where it does not hold"* defect.

Upstream's exception is narrow and is what `checkIndirectUse` decides. A reader
that is a plain `CPUI_INDIRECT` is not a use of the value at all — an INDIRECT is
the SSA record that *some other op may have overwritten this storage*. If every
flow out of the input ends in one of those, the leftover value is never read, so
there is nothing to print wrongly and the isolation buys nothing.

`checkIndirectUse` is a worklist over the def-set:

* plain `CPUI_INDIRECT` — accepting, the walk **stops** there;
* `CPUI_INDIRECT` with `isIndirectStore()` — accepted, but the walk **continues**
  through its output, because the value survives the store;
* `CPUI_MULTIEQUAL` — transparent, the walk continues through its output;
* **anything else fails the whole test** — one arithmetic use, one compare, one
  call argument, one COPY anywhere in the def-set and the input stays isolated.

That last clause is the safety property. The unit test
`an_input_with_a_direct_read_is_not_marked` and the stage test's second function
`keep` are both built on it.

## 3. Which of the two readers is actually live

Instrumented build (`KUNA_IO_DEBUG`, throwaway), `decompile-all` over three
coreutils binaries:

| slice | illegal inputs marked |
|---|---|
| fmt -O0 | 23 |
| fmt -O2 | 18 |
| ls -O0 | 140 |
| ls -O2 | 109 |
| sort -O0 | 137 |
| sort -O2 | 108 |

Essentially all of them are `stack` — addr-tied frame slots. Five register inputs
were marked on sort -O0.

The `has_name` reader needs an input that is *unaffected* as well as
indirect-only. On x86-64 ELF the only unaffected inputs on this corpus are RSP
(register 0x20, which is `directwrite` via the spacebase) and the return-address
stack slot (illegal, but **not** indirect-only — the return reads it). So
`has_name`'s `indirectonly` branch never fires here and **no declaration is
removed by that path**. The live consumer is `merge_test_adjacent`.

## 4. Whole-corpus effect

`decompile-all`, default (off) vs `--option indirectonly on`, **34 whole-binary
slices**: the original ls/sort/grep/gzip/diff/bzip2/find/tar at `-O0` and `-O2`,
plus kmod, e2fsck, dpkg, dpkg-query, dash and bash at `-O0`, `-O2` and
`-O2-noinline`. Script `corpus-sweep.sh`, classifier `classify-escape.py`, full
output `corpus-escape-classification.txt`.

| | |
|---|---|
| slices | 34 (15 change anything) |
| changed functions | 27 |
| declarations | **-19** net, none gained |
| ESCAPE-CANDIDATE | 18 |
| ESCAPE-CANDIDATE-WEAK | 6 |
| SAFE | 3 |

An ESCAPE-CANDIDATE is a statement the ON arm adds that assigns a frame slot,
where the slot sits inside an object whose address escapes and a call reaches that
address after the write. WEAK is the same with the address escaping only before
the write. In 10 of the 18 the slot is one the OFF arm never writes anywhere.

**24 of the 27 changed functions are in that ambiguous class**, and that is the
result: no subset of these hunks is certified safe by the emitted C alone. The
first round reported **0 BUG hunks** over 16 slices; that claim is withdrawn. Its
classifier had no escape predicate at all, and `bzip2 -O2 sub_3890` — inside that
same original 16-slice set — is confirmed wrong output.

## 5. The two directions, and why the text cannot tell them apart

The candidates split by what the **machine** does at the merge point, and only
the disassembly says which. Six are settled — two wrong, four right — by reading
the frame displacement out of `objdump` (the printed `// stack - X` is the rbp
displacement minus 8, confirmed on the stage fixture where `stack - 0xc8` is
`-0xc0(%rbp)`):

**FIX** — the machine really does store into the slot, and the OFF arm had
dropped the store. `ls -O2 main` is the witness. `ls.c` has

```c
struct dev_ino di = dev_ino_pop ();
struct dev_ino *found = hash_delete (active_dir_set, &di);
```

and the binary fills both halves before taking the address:

```
591a: mov rdx,QWORD PTR [rax-0x10]
591e: mov rax,QWORD PTR [rax-0x8]
5922: mov QWORD PTR [rsp+0x40],rdx      <- the store
5927: mov QWORD PTR [rsp+0x48],rax      <- the store
592c: call 11320                        <- hash_delete(..., &di)
```

With the flag dead, kuna prints the two loads into register locals and never into
the slot, so `unsigned long v38; // stack - 0x50` is declared and **never assigned
and never read anywhere in the function**, and `hash_delete` is handed memory the
emitted C never wrote. With the flag set, both halves are filled.

`tar -O0 sub_429c6`, `dpkg -O0 sub_2019c` and `e2fsck -O0 sub_1680a` are the same
shape at `-O0`, where the variable lives on the frame and the compiler really does
write it: `43076..43085`, `2021c`/`20307`, and `16b08`/`16b7c` are the stores the
OFF arm loses.

**BUG** — the machine only ever *loads* the slot, and the merge fabricates a
store. This is the `-O2` shape: the value lives in a register for its whole
lifetime and the frame slot it came from is never written back. `bzip2 -O2 sub_3890` is the clearest: the block at `[rsp+0x18..0x2c]` is a
set of out-params filled by callees through `lea`'d pointers, and every single
access to it in the function is a `lea` or a `mov reg,[rsp+X]` — there is no store
to any of those slots anywhere. The ON arm nevertheless emits

```c
S[stack-0x1428] &= 0xff;
...
S[stack-0x1428] = (unsigned int)*X;
```

`kmod -O2-noinline sub_97b0` is the same: `989d: mov r14d,DWORD PTR [rsp+0x28]` is
the only reference to that slot in the whole function and it is a load, while
`97c9: lea r13,[rsp+0x20]` is the object handed to `9964: call e270`.

On the page the two look identical — both are "a slot the ON arm writes and the
OFF arm does not". No local test over the emitted C separates them, which is why
the classifier reports candidates rather than verdicts, and why the sweep cannot
be turned into a safety proof.

`counterexample.md` reduces the BUG direction to a 40-line `walk.S`, compiles both
emitted bodies against the same library and shows them disagreeing (`a_done sees
n=3` versus `a_done sees n=-1`).

## 6. Why the merge phalanx does not catch it

An earlier draft of this document argued the opposite — that a call which may read
or write the slot attaches a `CPUI_INDIRECT` whose output joins the addr-tied
HighVariable, so its cover overlaps any register live across that call and
`merge_test_required`'s cover-intersection test rejects the merge. **That argument
is false.** `print raw` on `walk` shows why:

```
0x1150:12e:  s0x...ffd0:4(0x1150:12e) = s0x...ffd0:4(i) [] i0x1150:18(free)    ; INDIRECT at a_init
0x1173:131:  s0x...ffd0:4(0x1173:131) = s0x...ffd0:4(0x1167:130) [] ...        ; INDIRECT at a_push
0x117d:33:   R14D(0x117d:33) = s0x...ffd0:4(0x1173:131) + #0xffffffff:4        ; the load, the LAST read
0x118e:12a:  s0x...ffc8(0x118e:12a) = s0x...ffc8(0x1183:12c) [] i0x118e:45     ; INDIRECT at sink, for -0x38 ONLY
0x11a0:5f:   call fa_done(free)(RSP(0x113d:157))                               ; no INDIRECT at all
```

A `CPUI_INDIRECT` is attached only where the storage is still **live in the SSA**.
`stack-0x30`'s last read is the load at `0x117d`, so heritage stops guarding it:
no INDIRECT at `call sink`, none at `call a_done`, cover ends at `0x117d`, and the
intersection test has nothing to intersect. The neighbouring slot at `-0x38` is
still live — it is dereferenced inside the loop — and *does* get an INDIRECT at
`sink`, which is exactly why only the dead-after-read half is vulnerable.

There is also no p-code for a callee *reading* an escaped slot: INDIRECT models a
possible write. The information the merge would need is not in the IR at this
point, so no local test can recover it.

This is upstream behaviour, not a porting gap. Stock **Ghidra 12.1.2** on the same
`walk.so` emits `local_30 = local_30 + -1;` — the identical fabricated store
(`counterexample.md` §5). kuna with the option **on** reproduces Ghidra exactly;
kuna's shipped default declines to.

## 7. Speed and type_match

With the option off the action takes one `if` and returns, so the shipped default
costs nothing. Measured against a separately built origin/main (83830e86),
interleaved, min-of-11, `decompile-all` on `fmt` -O2: main 4092.9 ms, this
branch's default 4064.2 ms, **-0.70%** — noise, and the default arm's output is
byte-identical to that build over six whole binaries (`off-equals-main.txt`).

The cost of the option itself is one pass over the input def-set plus a bounded
worklist per illegal input, in an action that was already scheduled. Interleaved
off/on, min-of-15 per function and min-of-11 whole-binary:

| target | delta |
|---|---|
| `date` -O0 `main` | +0.31% |
| `fmt` -O2 `main` | -0.46% |
| `ls` -O2 `0x8ae0` | -0.78% |
| `fmt` -O2 whole binary | -0.64% |
| `ls` -O2 whole binary | -3.18% |

(The batched-median form in `scripts.pipeline.timeit` is contention-biased on this
box: it read +16.8% on a loaded machine and +3.4% on a quiet one for the same
build, which is why the interleaved minima are the numbers of record.)

`typesweep` over the campaign's 444 slices is exactly neutral between the arms:
959 perfect in both, aggregate 3037.05 in both, 0 functions moved in either
direction (`typesweep-report.md`). decbench scores the JSON `variables[]` surface,
which this change does not touch — 10739 of 10748 functions have byte-identical
`variables` between the arms.

## 8. What ships

**Default off.** The port is faithful — `check_indirect_use` and
`mark_indirect_only` match `funcdata_varnode.cc:801-858` opcode for opcode, and
`merge_test_adjacent`'s lifted refusal matches `merge.cc:189-211` — and with
`--option indirectonly on` kuna reproduces Ghidra's partitioning, including on the
counterexample. But the behaviour itself is wrong on a shape that occurs in the
corpus, and off is the arm that does not fabricate a store, so off is what ships.

Turn it on to reproduce Ghidra variable partitioning exactly, or for the
`ls -O2 main` class of fix where the OFF arm drops a store the machine makes.

Pinned:

* `tests/stages/kuna-indirectonly-escape.xml` — the counterexample as a
  bytechunk. Its first pass runs the **default**, so flipping the shipped default
  to on turns the test red.
* `tests/stages/kuna-indirectonly.xml` — the sound direction, both arms explicit.
* `kuna_indirectonly/tests.rs` — eight unit tests over `check_indirect_use`,
  including the direct-read counterexample that must not be marked.

Closing the hole instead of shipping off would mean diverging from upstream in
`merge_test_adjacent`, and the discriminator it would need — "does the machine
store to this slot here?" — is not available to a HighVariable-level test. That is
a separate feature, not a default change.
