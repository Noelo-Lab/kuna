# [PROPOSAL] angr-newburry-main-57ccb1 — eliminate the SUID-guard goto via condition-based join restructuring

**Status:** draft proposal — needs human go/no-go before any implementation worker is spent.

**Opportunity:** `test_decompiling_newburry_main :: main`
(`/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/newbury`, lighttpd `server.c::main`, x86_64 @ `0x40f696`).

**Proposed option name:** `gotoreduce` (shared with the sibling proposal — see below; do **not** mint a per-binary option).

## The problem

kuna's `newburry/main` differs from angr's by exactly **one goto + one label**
(all other metrics match: 6 loops, ~20 ifs, 1 ternary). The goto is a forward
jump *into* a then-branch:

```c
  if (getuid() == 0) {
label_f6ec:  setlocale(...); ... do { server loop } while(...);   // BODY
  } else {
    ... if (egid==gid) goto label_f6ec;                            // <-- goto into BODY
    v2 = -1; fputs("Are you nuts ...", stderr);
  }
  return v2;
```

angr emits zero gotos by **inverting the SUID-check guards** so the exceptional
`fputs; return -1` path is nested and the shared **BODY** (`setlocale; ...; server
loop`) becomes the fall-through join:

```c
  if (getuid()) {
      if (geteuid() != getuid() || !(getegid() == getgid())) {
          fputs("Are you nuts ...", stderr); return -1;
      }
  }
  setlocale(...); while (true) { server loop }                     // BODY = fall-through
```

Full side-by-side: [`angr-vs-kuna.txt`](./angr-vs-kuna.txt). Root-cause analysis:
[`analysis.md`](./analysis.md).

## Why it is LARGE (Hard rule 7)

The goto is produced in **S8 structuring**,
`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs` — a verbatim port
of Ghidra's `CollapseStructure`/`TraceDAG`. Reproducing angr's output requires
**condition-based block-graph restructuring**: invert the guard predicates so the
exceptional path nests, then re-parent the multi-predecessor BODY out of the first
`if`-then to become the fall-through join. That is a **new pass type** mutating the
BlockGraph, not a value-level Action/Rule and not a gated early-return:

- A gated early-return in `blockaction.rs` can only *suppress* the goto fallback;
  it cannot *synthesize* a guard inversion or re-parent a join — so it would yield
  invalid/garbled C, not angr's structure.
- The canonical small template (`kuna_loweredswitch.rs`) is an **S2 pcode** Action,
  inapplicable to block-graph structuring.

This is the **same root** as two already-filed proposals:

- **`kuna-gotoreduce-gap`** — morton/my_message_callback, **PR #54**: 1 goto+label
  to a shared *epilogue*; angr *tail-duplicates*. Ruled large (new S8 pass type).
- **`kuna-irreducible-loop-gap`** — fmt0/main family, **PR #46**: needs promoting
  kuna's read-only `RegionIdentifier` (`s7_regions/kuna_regionid.rs`) into a real
  **emit-path SAILR/Phoenix condition-based structurer**.

The pipeline record states this whole goto-reduction family shares **one** root and
should be closed by **one separately-funded project**, not point fixes. This
`newburry/main` case is another member: it is a *join-fall-through via guard
inversion* variant (vs. morton's *tail-duplication* variant), but both are emitted
by the same `CollapseStructure` fallback and both need the same SAILR emit-path
structurer.

## Proposed implementation plan (when the family project is funded)

This is **not** a standalone implementation — it folds into the shared SAILR
emit-path structurer that closes `gotoreduce` (PR #54) and `irreducibleloops`
(PR #46). The pieces specific to this case:

1. **Promote `s7_regions/kuna_regionid.rs`** from a read-only console surface into
   an emit-driving structurer (the shared family prerequisite).
2. Add a **condition-based goto-elimination** transform over the region/block
   graph that recognizes a multi-predecessor join `J` reached by (a) a then-branch
   and (b) a forward goto from a sibling branch, where the non-goto remainder of
   the sibling is a *terminating* path (returns/exits). Transform: invert the
   sibling's guard chain (de Morgan) so the terminating path nests, hoist `J` to be
   the fall-through join after the merged guard, and drop the goto+label.
3. Gate the whole structurer behind the **`gotoreduce`** option (default OFF while
   developing), early-returning to byte-identical output when off.
4. Verify on the family corpus (morton, newburry, and the irreducible-loop set)
   under one option to avoid per-binary option sprawl.

## Speed / risk assessment

- **Risk:** high — block-graph restructuring is the most semantically delicate part
  of the pipeline; an incorrect guard inversion produces *wrong* C, not just ugly C.
  Must be guarded by strict shape preconditions (terminating sibling remainder,
  single join, no intervening side effects on the inverted predicate evaluation
  order) and validated against the full 675-assertion datatest corpus.
- **Speed:** an S8 graph pass over already-collapsed structure; expected within the
  +5% budget, but must be measured on the family corpus (newburry/main is a large
  function — good speed canary). Not measured in this proposal (no implementation).
- **Default:** even if the family ablation is clean, this stays **opt-in** until the
  shared structurer is proven across the whole family.

## Recommendation

Do **not** dispatch a standalone implementation worker for `newburry/main`. Fold it
into the funded **`gotoreduce` / SAILR emit-path structurer** project (PR #54 / PR
#46). Track this binary as a *regression target* for that project.
