---
case_id: O2-noinline-bash-bash-compspec_dispose
status: metric-artifact
gap_survives: false
recorded_kuna_ged: 72
divergence_lever: null
proposed_new_option: null
runtime_choice: false
angr_feature: null
---

## Current kuna vs ghidra

Current kuna (default flags, benchmark surface `decompile-all --addr 0x9dc30`) fully and
correctly recovers the function — the `refcount--`, the outer `if (refcount==0)` guard, the
eight `FREE()`-macro-expanded `if (ptr != 0) sh_xfree(ptr, ...)` guards, and the tail
`free(cs)`. This is a faithful reconstruction of the source.

```c
// CURRENT KUNA  — ~30 LOC, 9 ifs (1 outer + 8 inner), 0 gotos, 0 labels, 0 loops, 0 switch
void compspec_dispose(int4 *a0)
{
  *a0 = *a0 + -1;
  if (*a0 == 0) {
    if (*(int8 *)&a0[6]  != 0) { sh_xfree(*(int8 *)&a0[6], "pcomplib.c",0x4e); }
    if (*(int8 *)&a0[8]  != 0) { sh_xfree(*(int8 *)&a0[8], "pcomplib.c",0x4f); }
    if (*(int8 *)&a0[10] != 0) { sh_xfree(*(int8 *)&a0[10],"pcomplib.c",0x50); }
    if (*(int8 *)&a0[0xc]!= 0) { sh_xfree(*(int8 *)&a0[0xc],"pcomplib.c",0x51); }
    if (*(int8 *)&a0[0xe]!= 0) { sh_xfree(*(int8 *)&a0[0xe],"pcomplib.c",0x52); }
    if (*(int8 *)&a0[0x10]!=0) { sh_xfree(*(int8 *)&a0[0x10],"pcomplib.c",0x53); }
    if (*(int8 *)&a0[0x12]!=0) { sh_xfree(*(int8 *)&a0[0x12],"pcomplib.c",0x54); }
    if (*(int8 *)&a0[0x14]!=0) { sh_xfree(*(int8 *)&a0[0x14],"pcomplib.c",0x55); }
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x0009e530 */
    sh_xfree(a0,"pcomplib.c",0x57);
    return;
  }
  return;
}
```

Ghidra's stored output for this address is **degenerate garbage** — it recovered *nothing*
of the function and emitted an infinite self-recursive stub:

```c
// GHIDRA  — 2 real body lines, 0 ifs, 0 loops. A degenerate self-call stub. Recovered nothing.
void compspec_dispose(void)
{
  compspec_dispose();
  return;
}
```

Stored GED scores (`results/full_run/O2-noinline/bash/evaluated/bash.toml`):

| decompiler | ged.compspec_dispose | recovered the function? |
|---|---|---|
| ghidra | **0.0** | no — degenerate self-call stub |
| ida    | **72.0** | yes — full 8-guard recovery |
| kuna   | **72.0** | yes — full 8-guard recovery |

The two decompilers that actually recovered the function (kuna **and** IDA) both score 72;
only ghidra's garbage output scores 0. On the neighbouring `compspec_copy` (which ghidra
*did* recover), all three agree (66/66/66). That is the tell: ghidra's 0 here is not a win,
it is a spurious match between two degenerate graphs.

## Divergence experiment

No divergence lever is responsible for this gap, and none closes it — the symptom is on the
*metric/source-parse* side, not on kuna's output shape.

- kuna's output shows no divergence symptom to chase (no extra gotos/labels, no switch, no
  noreturn overrun). The only divergence marker present is the single `tailcalljump` WARNING
  comment on the tail `free(cs)`.
- I flipped the one relevant lever, **`tailcalljump off`**, as a control. It makes kuna
  *dramatically worse*: without tail-call recognition kuna decodes straight through the
  tail-jump at 0x9e530 into the internal `free`/allocator body (mh_magic checks, `memset
  0xcf` scrub loops, an unrolled duff's-device switch, several gotos). The clean 30-line
  function balloons to ~200 lines of allocator internals. So the **default-ON** kuna output
  is already the correct, minimal, source-matching one — there is no lever to flip toward
  ghidra, and every lever move is away from source.

I confirmed the metric mechanism directly with pyjoern + `cfgutils.similarity.vj_ged`
(the exact tooling `decbench/metrics/ged.py` uses), parsing the **healthy** source CFG from
the preprocessed `pcomplib.i`:

```
SOURCE compspec_dispose : nodes=18  edges=41   (a real, non-degenerate CFG)
GHIDRA output           : nodes=1   edges=0    (degenerate stub)   -> GED(source, ghidra) = 101
KUNA   output           : nodes=19  edges=26   (correct recovery)  -> GED(source, kuna)   = 35
```

With a *correct* source parse the result **flips**: kuna 35 decisively beats ghidra 101.

## Analysis / runtime-choice verdict

**Metric artifact — degenerate/unstable source-CFG parse.** kuna's decompilation is correct
and strictly better than ghidra's; the 72-vs-0 is a scoring artifact, not a quality gap.

Root cause: `decbench/metrics/ged.py` scores `GED(source_cfg, decompiled_cfg)`, and the
source CFG is built by pyjoern/Joern from the preprocessed `.i`. The source definition is
**K&R old-style** (`compspec_dispose(cs) COMPSPEC *cs; { ... }`). For ghidra to have scored
**0**, the benchmark-time source CFG had to have collapsed to ~1 node (GED(1-node,1-node)=0),
i.e. Joern failed/degenerated on that K&R definition at benchmark time. Ghidra's own output
is *also* a ~1-node degenerate stub, so garbage-vs-garbage matched perfectly (GED 0). kuna
and IDA produced the real ~19-node CFG and were charged the full size of their (correct!)
graphs against the collapsed source — hence both land on exactly 72.

My re-parse of the same source produced a healthy 18-node CFG (Joern version/parse
instability on the K&R form is exactly why benchmark and re-run disagree), and under that
healthy parse kuna (35) beats ghidra (101). So:

- **Not already-fixed by session-1**: this function was already fully recovered on old kuna;
  F1/#120/#122 did not touch it. The score is unchanged because the output was always good.
- **Not a divergence-lever / new-runtime-choice**: kuna's default output already matches the
  true (macro-expanded) source shape; no existing or proposed option moves it toward ghidra,
  and `tailcalljump off` moves it strongly away.
- **Not a genuine bug**: kuna is not worse than IDA (both 72) and is *better* than ghidra
  (whose output is non-functional garbage).

The `gap_survives=false`: current kuna produces a strictly superior, correct decompilation;
the ghidra-beats-kuna number is an artifact of a degenerate/unstable source-CFG parse
(source_nodes collapsed to ~1 at benchmark time) coinciding with ghidra's degenerate output.
A fair re-score inverts it. There is nothing to fix in kuna.
