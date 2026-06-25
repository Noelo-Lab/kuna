# morton / my_message_callback — angr-vs-kuna gap analysis

- **angr testcase:** `test_decompiling_morton_my_message_callback`
- **binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/morton`
- **selector:** `my_message_callback` (`x86_64`, func @ `0x403de7`)
- **angr version:** 9.2.213

## The measured difference

| metric | reference (angr) | kuna |
|---|---|---|
| loc | 96 | 67 |
| **gotos** | **0** | **1** |
| **labels** | **0** | **1** |
| ifs | 9 | 9 |
| loops | 0 | 0 |
| switches / cases / ternaries / casts | 0 | 0 |

The pipeline's own "reference is better" signals reduce to exactly two, and they are the
same fact twice:

```
* ref has fewer gotos (0 vs 1)
* ref has fewer labels (0 vs 1)
```

So the **single concrete structural difference** is one residual `goto` + its `label:`.

## Where the goto comes from

The function is an if/else-if cascade over the parsed command token
(`temperature` / `humidity` / `UV` / else=*Invalid Command*) followed by a **common
epilogue** `free(v6); return;`. The "Invalid Command" arm has to skip the rest of the
body (`puts(v6)`, the publish, the error handling) and jump straight to that shared
epilogue. kuna renders this as:

```c
        else {
          v1 = strcmp(v3,"UV");
          if (v1 != 0) {
            v5 = strlen("Invalid Command");
            mosquitto_publish(a0,0,"app/error",v5 & 0xffffffff,"Invalid Command",1);
            goto label_41d6;          // <-- the residual goto
          }
          ...
        }
      ...
label_41d6:
  free(v6);
  return;
```

angr emits **zero** gotos for the identical CFG by **duplicating the `free(ptr); return;`
epilogue** into each arm that reaches it (it has several `free(ptr); return;` early-return
tails), turning the cross-edge into a structured early return instead of a goto:

```c
        else
        {
            v5 = mosquitto_publish(a0, 0, "app/error", strlen(v9) & 4294967295, v9, 1, 0);
            free(ptr);
            return;                    // duplicated tail, no goto
        }
        puts(ptr);
        ...
```

## Owning stage

This goto is produced by **S8 structuring** — `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`,
the **verbatim port of Ghidra's `CollapseStructure` / `BlockTrace` / `TraceDAG`**. The
module header says so explicitly:

> The schema-precedence order ... is *output-determining*: it decides which gotos get
> emitted when several structurings are possible ... Likewise `LoopBody` ordering ... and
> the `TraceDAG` bad-edge scoring ... are tie-breakers that decide *which* edge becomes a
> goto — transcribed verbatim.

Ghidra's algorithm collapses the CFG by schema-matching and, **when stuck, removes
("gotos") an edge** chosen by the DAG trace. There is no schema in the cascade that
duplicates a small return tail to *avoid* that goto — that is precisely the class of
transform angr's structurer (the Phoenix/SAILR "goto-reduction via tail duplication and
condition restructuring" passes) adds *on top of* the base structuring.

## Hypothesis for the kuna change (and why it is large)

Matching angr means adding a **new structuring transform**: detect a goto edge whose
target is a small, side-effect-light epilogue (here `free(v6); return;`) and **duplicate
that tail into the goto source**, eliminating the unstructured edge. That is:

- a **new pass *type*** (return/epilogue tail-duplication), not modelable as a single
  `Action`/`Rule` like `kuna_loweredswitch.rs`;
- it **touches S8/S7 structuring/region code** (the `CollapseStructure` collapse loop and
  the `BlockGraph` topology) well beyond a single gated early-return;
- it must reason about block duplication, statement cost, and re-running the collapse —
  multi-step, with real correctness risk.

Per **Hard rule 7**, this is a **large** gap → proposal, not a one-shot option. See
`proposal.md`. This is the same SAILR-structurer family as the irreducible-loop gap
(proposal PR #46) and is best done as a deliberate, human-approved structuring effort.

No existing option covers goto/structuring reduction (checked `kuna catalog --json`: the
catalog has no structuring/goto-elimination entry; closest are presentation/dataflow
folds, none of which touch the `CollapseStructure` goto set).
