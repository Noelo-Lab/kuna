# callpush — analysis

## What the `(unsigned long *)&v` residue is

The round-I cast census left `(unsigned long *)&v` at 1,252 casts on the 4,815 functions kuna
and IDA both emit (castbench `main-b3878d32e`, and the same 1,252 on `main-c960fb18d` after
`globalref`), against IDA's 3, and read it as "an object read at another width". A census of every
pointer cast whose operand is an `&` expression, by what the object behind the `&` is
(`addrcensus3.py`, beside this file), says otherwise:

| what the `&` operand is | all widths | `(unsigned long *)` | after `callpush` | IDA at the same sites |
|---|---:|---:|---:|---|
| the stack pointer after an `alloca` (a `char *` copied from a stack array, `// rsp`) | 1,070 | 990 | 142 | nothing: IDA prints `alloca(n)` and no push |
| a pointer local indexed at another width (`*(unsigned int *)&v38[6]`, `v38` an `unsigned long *`) | 992 | 130 | 122 | the same count as `*(_DWORD *)(a1 + 24)` on an integer-typed base |
| a pointer parameter indexed at another width (`*(long *)&a0[0x10]`, `a0` an `int *`) | 325 | 37 | 37 | the same: `*(long long *)(a1 + 64)` |
| a member of a sized, fieldless libc shell (`*(char **)&stdout->field_0x28`; `stat` 49, `tm` 18, `obstack` 13, `re_pattern_buffer` 7) | 593 | 87 | 87 | `stdout->_IO_write_ptr`; kuna's `libctypes glibc` value names these and stays opt-in for its own reasons |
| a global indexed at another width | 228 | 4 | 4 | `dword_18124` |
| a stack local declared at another width (`(int *)&v11` into `getopt_long`/`waitpid`, `unsigned int v11`) | 63 | 0 | 0 | `&v11` with `int v11` |
| a stack buffer read as words (`*(int *)&v17[i * 4]`, `char v17[...]`) | 27 | 1 | 1 | `v17[i]` on `int v17[...]` |
| a `struct_N` member at another width | 51 | 3 | 3 | |
| an index expression the census could not attribute | 6 | 0 | 0 | |
| total | 3,355 | 1,252 | 396 | |

The eight pointer-local casts that also drop are pushes through an alloca alias the census's
root tracing does not follow. Only the stack-local, stack-buffer and `struct_N` rows are objects
declared at another width than their uses, 141 casts of which 4 are `(unsigned long *)`. The pointer-index rows are a shape difference, not a count gap:
IDA types the same bases as integers and pays one cast per access too (`diff_2_files`: kuna
`*(long *)&a0[0x10]`, IDA `*(long long *)(a1 + 64)`); the fix there is a record for the base,
which is `structsynth`'s job. The largest row is not a type at all.

## The alloca row

The brief's witness, 155 copies of `*(unsigned long *)&v28[...]` in one function, is cp -O0
`copy_internal` (0xabda), and `v28` is a `char *` holding the stack pointer after an `alloca`.
Of the 1,070 casts in that row, in 22 functions:

* 844 are a call's own return-address push, `*(unsigned long *)&v28[-8] = 0xbc79;` before every
  call (0xbc79 is the address after the `call`);
* 109 are stores of stack-passed arguments for a call through the moved stack pointer
  (`*(unsigned long *)&v28[-0x10] = (unsigned long)v48;`), real arguments the call does not
  show, a separate defect;
* 96 are `-fstack-clash-protection` probe touches, `*(unsigned long *)&v6[-8] = *(unsigned long *)&v6[-8];`;
* 21 are the alloca's own pointer arithmetic.

An x86 `call` lifts as a stack-pointer decrement, a STORE of the fall-through address at the new
stack pointer, and the CALL. When the stack pointer is a constant offset from its entry value,
`RuleStoreVarnode` makes the STORE a COPY to a stack slot nothing reads and dead code removes it.
After `sub rsp, rax` the stack pointer is the entry value minus a run-time size, the STORE keeps
its pointer, and it prints. Ghidra prints the same statements; IDA prints none.

## The change

`RuleCallPush` (`p4_calls/kuna_callpush.rs`, option `callpush`) deletes that STORE when the
stored value is a constant one word wide within 15 bytes after the instruction, a CALL/CALLIND
follows at the same instruction address whose destination is not the stored address, the pointer
is the stack pointer register as that instruction wrote it, and `check_spacebase` cannot place it
on the stack. It runs in `oppool2` right after `RuleStoreVarnode`, so a tracked push is that
rule's first. Stack-argument stores and probe touches are not matched.

## Why no value changes

The deleted store writes the word below the stack pointer that only the callee's `ret` reads;
the C call performs that transfer. No printed statement reads the slot: the one idiom whose
caller does (`call 1f; 1: pop`) is excluded by the destination test and kept in the fixture
(`pc_here`), and a callee that pops its return address as data is lifted as a BRANCH by
`callpopret`, so there is no CALL to match.

An alloca frame is not runnable C in either arm (the alloca prints as stack-pointer arithmetic,
and the stack-passed arguments never reach their call), so the round trip cannot be "compile and
run the printed function". The proof is the statement set instead:

* the CLI test (`a_calls_own_return_address_push_is_part_of_the_call`) decompiles four builds
  (gcc/clang, -O0/-O2) of `callpush_x86_64.c`: off prints a store of every listed return address
  (each taken from `objdump -d`), on prints none, both arms make the same calls, `stacked` keeps
  the same number of other stores, and the `call 1f` push is printed in both;
* the whole-corpus diff below.

## Whole corpus (castbench full: 15 binaries x O0/O2/O2-noinline, every function)

`hunks2.py` and `sidefx.py` (beside this file) over `decompile-all` with `callpush off` and on
(the off arm is byte-identical to main's castbench arm on all 45 binary/level pairs); per-function
results in `corpus-hunks.json`. 75 functions change, all of them frames whose stack pointer is
moved by a run-time amount. 1,607 return-address stores are removed, every one verified: its
constant is the address after a `call` in that binary. The rest of each diff, after mapping
variable names by the lines that match with names erased, falls into these classes:

| class | hunks | what it is |
|---|---:|---|
| fold | 605 | a load or call result held in a temporary across the push now prints at its use. `check_implied_cover` kept it explicit because its live range crossed a may-alias STORE, and that STORE is gone. The order of calls is unchanged (read on the errno, `dcgettext`, `error` sequences) |
| taildup | 82 | a tail block the push kept large (`abort()`, `sub_aeea(); return 1;`) is duplicated into its predecessors instead of reached by `goto` |
| condmerge | 72 | a block that held only the push beside a condition merges into it (`if (a && (v = f(), v))`, `else if`) |
| decl | 52 | declarations renumbered, or a variable only the pushes used is gone |
| header | 10 | a header note (`// ternary`, iteboolean counts) changes because a block got simpler |
| hand-read | 56 | expression reuse (`memcpy(v38,...)` for the variable holding that address), loop and brace layout (`do {} while (true)` vs `while (true) {}`), two independent assignments printed in the other order, a stack-pointer alias assigned a few lines earlier (tar `sub_59300`: same address), the other of two equal copies chosen (`v37 = v8`), and stack slots written back from the registers they were just loaded from (tar `argp_parse`: `v17 = v19[0]; ... v19[0] = v17;`), copies heritage needed only because the push might alias them |

The per-function multiset of calls, memory stores and returns (`sidefx.py`) is identical in 45
of the 75 functions; in the other 30 every difference is a count that grows in the on arm (tail
duplication) except two, both read by hand: cp -O2 `copy_internal` prints one fewer duplicated
`return v & 0xffffffff;` and three more duplicated no-return `sub_6770()` tails, and tar -O2
`argp_parse` drops the `v19[0] = v17;` write-back above. The removed guards also free tracked
return-address slots that only they kept alive (clang -O0: `v3 = 0x1249;` before a `strlen`
that precedes the alloca).

Across all 20,230 functions of the 45 binaries no function's arity changes, and one parameter
type does: tar -O2-noinline `argp_parse`'s sixth parameter goes from `unsigned long` to `int8`.
Its spill slot was also the word the pushes were stored through (`v13 = &v6;
((unsigned long *)v13)[-1] = 0x3d780;`), so the return address's word type was a use of that
slot; without the push the slot is typed by the parameter's own uses. Neither is the program's
`void *input`, and the 444-slice typesweep does not move. The 75 changed functions declare
2,407 locals before and 2,302 after; one declares one more (tar -O2-noinline `sub_59300`, the
stack-pointer alias above, split into two names).

A stack array whose lowest slot was the push (clang's alloca form, `(long)v4 + v2 + 0x10`) is
declared from its next slot: `v4` moves from offset -80 (32 bytes) to -72 (24 bytes) and every
reference shifts by 8, so every address is unchanged.

## What is left

After `callpush`, 396 `(unsigned long *)&v` casts remain on the shared set. Of main's 1,252, the
262 outside the alloca row were never pushes (except the eight noted above), and each class has an
owner:

| class | `(unsigned long *)&v` on main | left | owner |
|---|---:|---:|---|
| pointer local indexed at another width | 130 | 122 | a record for the base (`structsynth`, the `structdeep` lane); IDA pays the same cast on an integer base |
| pointer parameter indexed at another width | 37 | 37 | the same |
| libc shell member (`stat`, `tm`, `obstack`, `re_pattern_buffer`) | 87 | 87 | `libctypes glibc` (opt-in), which gives the shells their fields |
| global indexed at another width | 4 | 4 | `globalref` / the global's declared type |
| `struct_N` member at another width | 3 | 3 | `structsynth` field width (`structmerge`) |
| stack buffer read as words | 1 | 1 | frame-slot typing (`framelayout` / `slotptr`) |
| alloca row, not a push | | 142 | an alloca recognizer (no lane yet) |

* The 142 alloca-row casts left are the stack-argument stores (the calls are missing those
  arguments) and the probe touches. Both want an alloca recognizer (`v = alloca(n)`, stack
  arguments matched against the moved stack pointer), a larger change.
* About 120 return-address stores into tracked stack slots that other aliasing keeps alive
  (`v61 = 0x5b035;` in tar -O2 `sub_5aa30`, `v22[1] = 0x56a91;` in tar -O2-noinline). They carry
  no cast and print in both arms.
* The 90 locals and buffers at all widths are the real "declared at another width" lever, most
  of them a sign-only mismatch against a libc `int *` parameter (`waitpid(v2,(int *)&v5,0)` with
  `unsigned int v5`).
