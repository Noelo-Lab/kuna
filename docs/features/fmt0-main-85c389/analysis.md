# fmt0-main-85c389 — analysis

- **Opportunity:** `test_decompiling_fmt0_main :: main`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/fmt_0` (GNU coreutils `fmt`)
- **Function:** `main` @ `0x4019b0` (arch `x86_64`)
- **Reference:** angr 9.2.213

## What angr does better

angr recovers a clean, **valid** C structuring of `main`'s option-parsing loop:

```c
v1 = 0;
while (true) {
    v6 = getopt_long(v3, cur, "0123456789cstuw:p:g:", &long_options.name, NULL);
    if (v6 == -1)
        break;
    if (v6 > 119)            goto LABEL_401d6a;
    if (v6 > 98) { switch (v6) { case 99: ...; case 103: ...; default: LABEL_401d6a: ... } }
    else if (v6 == -131) { version_etc(...); exit(0); }
    else if (v6 == -130) { usage(0); }
}
```

3 gotos, 2 loops, 187 loc, fully valid C.

## What kuna produces (the bug)

kuna emits **syntactically invalid C** — verified to be the genuine raw `decomp_dbg`
`print C` output (not a pipeline/CLI text artifact; confirmed by driving `decomp_dbg`
directly and by inspecting `scripts/decompile.py`, which only strips newlines):

```c
  if ((2 <= a0) && (v7 = (char *)a1[1], v13 = a1, v2 = "", v8 = dat_216128, *v7 == '-')) goto label_1ba7;
                    /* WARNING: Subroutine does not return */
  while (dat_216128 = v8, v14[0] = (char *)0x0           // <- incomplete: no ") {" no body
  while (v5 = getopt_long(a0,a1,"...",long_options,0)    // <- incomplete
  if v5 == -1 {                                          // <- missing parens
    ...
    elsev8 = (char *)dcgettext(...), ... if v14[0] != ...// <- mangled else-branch
  if 0x77 < v5 goto label_1d6a;99 <= v5) {               // <- totally broken
```

kuna: 8 gotos, 4 loops, **invalid C** (the `compare` tool also flags a recovery-failure
marker). See `angr-vs-kuna.txt` for the full side-by-side.

## Root cause — irreducible loop

`main` contains an **irreducible (multi-entry) loop**. GNU `fmt`'s `-WIDTH` first-argument
special case (`fmt` accepts a leading `-NN` as a width) compiles to a `goto` that jumps
**into** the `getopt_long` option-parsing loop body:

```
if ((2 <= a0) && ... *v7 == '-') goto label_1ba7;   // label_1ba7 is INSIDE the loop
```

kuna's region tree confirms the multi-entry cyclic structure (block `0x1ba7` is a
**sibling entry** into the loop region):

```
region head=0x1a6e nodes=2 cyclic
  region head=0x1a6e nodes=3 cyclic
    block 0x1a6e
    region head=0x1a90 nodes=23 cyclic      <- the getopt loop body
      ...
  block 0x1ba7                              <- second entry into the loop
```

kuna's structurer is Ghidra's **collapse-based** `BlockGraph` engine
(`s8`/`blockaction.rs`; `BlockWhileDo` etc.), whose only response to irreducibility is
`markUnstructured` → gotos. On this loop the collapse engine additionally produces a
while-block whose **condition block carries multiple side-effecting statements** that the
S9 printer (`printc.rs::emitBlockWhileDo`) cannot render as a single expression — so it
emits incomplete `while (...` lines: invalid C.

## Owning stage

- **S7** region structuring / **S8** block structuring (`blockaction.rs`) — the structurer
  fails on the irreducible loop.
- **S9** emit (`printc.rs`) — the printer emits invalid C for the side-effecting
  while-condition.

angr handles this with its `RegionIdentifier` + SAILR/Phoenix **condition-based**
structuring and goto/loop refinement. kuna *has* an angr-`RegionIdentifier` port
(`kuna_regionid`/`kuna_regiongraph`, S7), but it is a **parallel, read-only** surface
(queried via the console `region tree`); it does **not** drive `print C`.

## Hypothesis / why this is NOT a small option-gated Action

`loweredswitch` is the canonical small template because it **manufactures a missing S2
artifact** (a `JumpTable`) that the existing collapse structurer already knows how to render.
Here there is **no artifact to manufacture**: the collapse structurer itself is wrong on
irreducible loops. Making kuna emit angr-like output requires either augmenting the collapse
structurer with condition-based irreducible-loop handling, or promoting the read-only
`kuna_regiongraph` into an emit-capable structured-tree producer plus a new S9 printer path.
Both are new pass infrastructure spanning S7/S8/S9 and well over the >3-anchor-file /
>1-new-module limit. **A scope-decider (recorded in `record.json`) confirmed `scope:
large`.** Per Hard rule 7 this goes through a `[PROPOSAL]` draft PR for human go/no-go —
see `proposal.md`.
