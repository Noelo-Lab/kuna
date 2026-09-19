# aliasoverlap: analysis

## 1. The symptom

`inside(p, w) { unsigned v; memcpy(&v, p + 7, 4); p[8] = w; return v; }`,
built with gcc -O2 (clang -O2 emits the same pair), is
`mov 0x7(%rdi),%eax; mov %sil,0x8(%rdi); ret`. kuna on main prints

    *(char *)(a0 + 8) = a1;
    return *(unsigned int *)(a0 + 7);

The load reads bytes 7..10 and the store writes byte 8, so the printed C
returns the new byte where the binary returns the old one. Compiled and run on
a filled buffer, main's `inside` returns `f3ce5a84` where the source returns
`f3cea984`. The same happens to a 4-byte store below the load (`below`, store
at `p+5`) and to an element store into an 8-byte indexed load (`indexed`,
`memcpy(&v, a + i, 8); a[i + 1] = w`), and on the DWARF build, where the
accesses print as `p->c8` and `*(unsigned int *)&p->c7`. The structdedup lane
found it (cx6 `rd_c8`/`rd_c9`/`rd_c10`, gcc -O2, with `structsynth off`): main
gets all three wrong in a round trip, e.g. `rd_c8` returns `4201fc33` for
`42016633`.

## 2. The root cause

`ActionMarkImplied` marks a Varnode implied (printed at its use rather than as
its own statement) unless `checkImpliedCover` finds that moving it is unsafe.
For a LOAD, it walks every STORE inside the load's cover and forces the load
explicit when `isPossibleAlias(storePtr, loadPtr, 2)` says the two pointers may
be equal. That predicate compares pointer values only:

- `isPossibleAliasStep` returns "different" when one pointer is the other plus
  any constant (INT_ADD / PTRSUB / PTRADD / INT_XOR with a constant input);
- the INT_ADD arm returns "different" for one base plus two different
  constants (`return !functionalEquality(base1, base2)`);
- two constant pointers are "different" unless they are equal.

None of this looks at how many bytes each access touches, so `a0 + 8` (1 byte)
and `a0 + 7` (4 bytes) count as two objects.

Upstream Ghidra has the same hole (coreaction.cc `ActionMarkImplied::isPossibleAlias`,
unchanged on master at f9e13846, 2026-06-16); its constant arm even carries
`// FIXME: these could be NEAR each other and still have an alias`.

## 3. Siblings checked

- `is_possible_alias` has one caller, `check_implied_cover`'s load-crosses-store
  loop; `is_possible_alias_step` is only called from it.
- The load/call-crosses-call arm of `check_implied_cover` is unconditional.
- `foldcallret`/`foldcallretphi` (`kuna_callretfold.rs op_is_barrier`) treat
  every STORE as a barrier, with no offset reasoning.
- `SplitDatatype::split_load` (`p3_dataflow/subflow.rs`) does move a LOAD past
  a STORE or a call; section 6 covers it, and this PR fixes it too.
- CSE (`cse_find_in_block`) and `RuleMultiCollapse` use `functional_equality`,
  which equates two LOADs only when they come from the same machine
  instruction.
- `RuleDoubleLoad`, which merges two LOADs into one at the later of the two,
  already refuses when anything between them may write the loaded space
  (`no_write_conflict`). The bitfield rewrites (`p5_types/bitfield/`) re-create
  a LOAD next to the original (pull) or re-read the container for their own
  read-modify-write (insert), and `op_stack_load` adds a stack read at its use.
  None of them moves an existing read across a store.

## 4. What the fix must keep

A store beside the loaded bytes (an adjacent byte, the next array element) is
still no alias, so the load keeps folding past it; `tests/stages/kuna-aliasoverlap.xml`
pins `after`, `before` and `next` for that. For a `PTRADD` index the distance
between two indices counts elements, so it must be scaled by the element size
before it is compared with byte widths: unscaled, `a[i]` against `a[i + 1]`
(4-byte elements, 4-byte accesses) would read as distance 1 < 4 and turn every
such load explicit.

## 5. The fix only ever adds explicit loads

In every case where main's predicate answers "may alias", the new one does
too: two equal constants overlap; a constant step or two distinct constants
off one base used to be "different" outright and now are "different" only if
the ranges are disjoint; wherever main recursed, the new code recurses with
the same operands (or answers "may alias" outright for the commuted `PTRADD`
matches). So a load main printed as its own statement still is, and every
output change is a load that was folded past a store now printed as a local
ahead of it.

## 6. The second path: `SplitDatatype::split_load`

When a LOAD's type says it spans several fields (in practice a DWARF-typed
struct pointer; no function in the 52 stripped footprint binaries changed),
`RuleSplitLoad` splits it into one LOAD per field. If the loaded value's only use is a COPY, the split LOADs were built at
the COPY and wrote straight into the COPY's output, which can sit after a
STORE or a call. Upstream Ghidra does the same (subflow.cc
`insertPoint = (copyOp == 0) ? loadOp : copyOp`). With
`struct S { int i0; char c4, ..., c11; }`, gcc -O2 -g:

    unsigned d1(struct S *s, int w) { unsigned v; memcpy(&v, &s->c7, 4); s->c9 = w; return v; }

is `mov 0x7(%rdi),%eax; mov %sil,0x9(%rdi); ret`, and main printed

    s->c9 = (char)w;
    v1._0_1_ = s->c7;
    ...
    v1._2_1_ = s->c9;

The same happened with a call between the load and the COPY (`mov
0x7(%rdi),%r12d; call sink; mov %r12d,%eax` printed `sink(s)` first) and with a
store through a second pointer. clang -O2 -g is the same. This path does not go
through `is_possible_alias`, so the first half of the fix did not reach it.

The fix declines the split when a STORE or a call lies between the LOAD and
its COPY, or the two sit in different blocks. The read then prints whole, as
its own statement ahead of the store or call: `v1 = *(unsigned int *)&s->c7;`.
When nothing lies between the two the output is unchanged (the `plain` and
`splitplain` controls).

The first attempt split at the LOAD and kept the COPY, as the review suggested.
The DWARF footprint showed why that is wrong: on tar O0 `validate_uparams`
(`uparams = *upptr;`) each 8-byte read's COPY writes the global `uparams`
at the return, and the write of `uparams+0x20` lies between. Split at the
LOAD, the pieces went to a temporary and the COPY into `uparams` stopped
printing, so four of the five stores vanished from the output. Declining the
split keeps main's output there, since a write to a global Varnode is neither a
STORE nor a call.
