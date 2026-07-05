---
case_id: O2-noinline-openssh-portable-ssh-kex_choose_conf
status: divergence-lever
gap_survives: true
recorded_kuna_ged: 54
divergence_lever: taildup
proposed_new_option: null
runtime_choice: true
angr_feature: null
---

## Current kuna vs ghidra

The source `kex_choose_conf` (openssh `kex.c`) is the textbook C error-cleanup idiom:
seven `goto out;` early-exits, all converging on ONE shared epilogue

```c
 out:
	kex_prop_free(my);
	kex_prop_free(peer);
	return r;
```

Ghidra reproduces this *exactly* (GED 0): every early exit is `goto LAB_0019a6c7;` and
`LAB_0019a6c7:` is the single shared cleanup (`FUN_0019a160(local_98); FUN_0019a160(puVar11);
return`). Counts: **217 LOC, 5 gotos, 7 labels, 2 returns** (the 2nd return is the
`__stack_chk_fail` tail).

Current kuna (DEFAULT, `taildup` ON) — the `taildup` divergence DUPLICATES that shared
return-tail at each early-exit site instead of jumping to it. Result: **0 real gotos, 0
labels, 6 returns** (204 LOC). Each exit inlines its own copy of the cleanup:

```c
            if (v6 != 0) {
              *(void *)(v2 + 0x98) = *(void *)(v13 + (int8)v9);
              *(void *)(v13 + (int8)v9) = 0;
                    /* WARNING: taildup: duplicated return-call tail to remove goto */
              sub_9a160(v20);        // <- copy 1 of the shared epilogue
              sub_9a160(v12);
              v6 = v14;
              return v6;
            }
            ...
              if (v6 != 0) {
                *(void *)(v2 + 0x98) = v9[v13];
                v9[v13] = 0;
                sub_9a160(v20);      // <- copy 2 of the same epilogue
                sub_9a160(v12);
                v6 = v14;
                return v6;
              }
            ...
            if (v14 != 0) {
              ...
              sub_9a160(v20);        // <- copy 3 ...
              sub_9a160(v12);
              v6 = v14;
              return v6;
            }
```

The duplicated cleanup blocks are extra CFG nodes that the source (and ghidra) do not have —
that inflation is the GED 54. kuna is here *more* linearized than ghidra, not less; the
metric is against SOURCE, and the source is goto-shaped, so kuna's goto-removal moves it away.

kuna with `--option taildup off` — restores the single shared epilogue, matching ghidra:
**192 LOC, 4 gotos, 1 label (`label_9a6c7:`), 1 return**:

```c
            if (v6 != 0) {
              *(void *)(v2 + 0x98) = *(void *)(v13 + (int8)v9);
              *(void *)(v13 + (int8)v9) = 0;
              goto label_9a6c7;
            }
            ...
              if (v6 != 0) { ...; goto label_9a6c7; }
            ...
            if (v14 != 0) { ...; goto label_9a6c7; }
            ...
label_9a6c7:
  sub_9a160(v20);
  sub_9a160(v12);
  v6 = v14;
  return v6;
}
```

This is structurally the same shape as ghidra's `LAB_0019a6c7` epilogue and the source's
`out:` label (`branchflip`'s arm-swaps remain but are CFG-neutral for GED — same nodes/edges).

## Divergence experiment

| run | LOC | real gotos | labels | returns | shape vs ghidra/source |
|---|---|---|---|---|---|
| ghidra (target, GED 0) | 217 | 5 | 7 | 2 | shared `out:` epilogue + gotos |
| kuna default (taildup ON) | 204 | 0 | 0 | 6 | epilogue DUPLICATED 6x (the GED 54) |
| kuna `--option taildup off` | 192 | 4 | 1 | 1 | shared `label_9a6c7:` epilogue + gotos — MATCHES |

Winning lever: **`taildup` OFF**. It alone collapses the six duplicated return-tails back
into the single shared cleanup label + gotos, reproducing the source/ghidra structure.
`branchflip` warnings persist but only swap if/else arms (no node/edge change → not the GED
driver); no other lever was needed.

## Analysis / runtime-choice verdict

Root cause: kuna's default-on `taildup` (angr/SAILR-derived tail duplication — "duplicated
return-call tail to remove goto"). It rewrites a shared bare-return epilogue reached from many
predecessors into a private copy per predecessor, eliminating the `goto`. This is genuinely
the better majority choice for angr-style output: goto-free, linear code reads cleaner on the
bulk of functions, which is exactly why we turned it on by default (DIV/[[kuna-default-on-sweep]]).

But this function is the canonical counter-shape: a C error-handling body where every failure
check does `goto out;` into ONE cleanup+return (the `kex_prop_free(my); kex_prop_free(peer);
return r;` free-chain). Ghidra keeps that epilogue merged and therefore matches the source CFG
exactly (GED 0); kuna's tail duplication forks it into six copies and pays GED 54 for extra,
source-absent cleanup nodes.

This is a RUNTIME CHOICE, not a bug: ghidra's form is recoverable today via `--option taildup
off`, and `taildup` stays default-on because it wins on the majority (goto-free linearization).
Source-shape signal telling an agent to flip it off: the function has multiple early-exit
error checks that all free/dispose the same locals and `return` (a shared cleanup / free-chain
epilogue with many predecessors — the `goto out;` idiom). On that shape, keeping the epilogue
merged (`taildup off`) matches source and ghidra; on ordinary functions leave it on.

(Thematically this is the same target the pending `returndup` angr feature would auto-detect —
"shared bare-epilogue return kept merged; dispose/free-chain shape" — but no pending feature is
required here: the existing `taildup` lever already closes the gap, so this is filed as a
divergence-lever, not angr-feature-pending.)
