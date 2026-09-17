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

`decompile-all`, 8 binaries x {O0, O2} = 16 slices, `--option indirectonly off`
vs the default. Ten functions change; every hunk is classified in
`corpus-hunk-classification.txt`, and `skeleton-check.txt` is the structural
control: for each of the ten, the sequence of control keywords with their nesting
depth and the ordered sequence of callees are **identical** between the arms, so
no statement moved across a branch or a call.

| slice | function | what changed |
|---|---|---|
| ls -O0 | `sub_1dd61` | call result merged into `stack-0xa8`; one copy removed |
| tar -O0 | `sub_429c6` | RAX merged into `stack-0x208` / `stack-0x210`; two copies removed |
| ls -O2 | `main` | **wrong output fixed** — see §5 |
| sort -O2 | `main` | three temporaries merged into their slots |
| diff -O2 | `sub_6e20` | malloc result merged into `stack-0x80`; one copy removed |
| bzip2 -O2 | `sub_3890` | EAX merged into `stack-0x1428`; **-1** declaration |
| tar -O2 | `main` | a `vN = slot;` / `slot = vN;` round-trip disappears (5 statements); **-1** declaration |
| tar -O2 | `sub_203b0` | two slot merges; **-2** declarations |
| tar -O2 | `argp_parse` | a 16-byte spill round-trip disappears (10 statements); **-2** declarations |
| tar -O2 | `sub_5aa30` | six slot merges; **-5** declarations net (one new register name) |

grep, gzip, find at both levels and sort/bzip2/diff/grep/gzip/find at -O0 are
byte-identical.

No function gains a declaration; the net is **-11** across the corpus.

## 5. The witness: coreutils `ls` -O2 `main`

`ls.c` has, inside `dev_ino_pop`'s caller:

```c
struct dev_ino di = dev_ino_pop ();
struct dev_ino *found = hash_delete (active_dir_set, &di);
```

`di` is a 16-byte struct on the frame at `stack-0x58` / `stack-0x50`; its address
escapes into `hash_delete`, so both halves are address-tied and each one's entry
value reaches nothing but the INDIRECTs of the calls around it — indirect-only.

With the flag dead, the two loads that fill it cannot be merged into the slots, so
they are printed into register locals instead and the struct is never written:

```c
/* option indirectonly off */
      v26 = dat_260f8 + -0x10;
      v18 = *(void **)(dat_260f8 + -0x10);        /* dead: v18 is overwritten 2 lines down */
      v21 = *(unsigned long *)(dat_260f8 + -8);
      dat_260f8 = v26;
      v18 = (void *)sub_11320(dat_263c8,&v22);    /* &v22 == &di, never assigned on this path */
```

`unsigned long v38; // stack - 0x50` — the second half of the struct — is declared
in that arm and **never assigned and never read anywhere in the function**.

With the flag written, both halves are filled before the address is taken:

```c
/* default */
      v26 = dat_260f8 + -0x10;
      v22 = *(void **)(dat_260f8 + -0x10);
      v38 = *(unsigned long *)(dat_260f8 + -8);
      dat_260f8 = v26;
      v18 = (void *)sub_11320(dat_263c8,&v22);
```

The declaration count is the same; what changes is that the emitted C now writes
the memory the machine writes.

## 6. Why merging into an escaped slot stays safe

Two hunks (`bzip2 -O2 sub_3890`, `sort -O2 main`) write an intermediate value into
a frame slot whose address escapes. That is safe for a reason this change does not
touch: a call that may read or write the slot attaches a `CPUI_INDIRECT` to it,
whose output is another Varnode at the same address and therefore a member of the
same addr-tied HighVariable. Its cover overlaps any register live across that
call, and `merge_test_required`'s cover-intersection test rejects the merge. The
flag only lifts the *illegal-input* refusal, which is about the slot's **entry**
value; the cover machinery is unchanged.

## 7. Speed

The scan is one pass over the input def-set plus a bounded worklist per illegal
input, run once per function in an action that was already scheduled. See
`record.json` for the interleaved min-of-15 numbers.
