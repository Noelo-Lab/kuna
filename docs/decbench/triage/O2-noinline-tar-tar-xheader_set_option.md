---
case_id: O2-noinline-tar-tar-xheader_set_option
group_id: tar::xheader_set_option
status: covered-by-option
tier: L
margin: 241
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: 241
option_closing: listing
root_cause_family: noreturn-propagation-stripped
feature_slug: null
scope: small
angr_ref: angr.analyses.cfg.cfg_fast (function-returning call-graph fixpoint; NOTE angr's stored output here is a truncated stub, not a real win)
kuna_stage: S1/external-refinement (kuna-analysis Listing consumer noreturn_propagate, gated on option listing)
closed_by: PR #121 (benchlisting, DIV-15)
ged_after_fix: 20
---

## Side-by-side

Ground truth (compiled binary symtab): `xheader_set_option` @ 0x1c010 is **99 bytes**
(0x1c010–0x1c073). Its last instruction is `call 0x1a0f0 <xheader_set_single_keyword>`
at 0x1c06e with only alignment padding after it — GCC knows the callee never returns
(`USAGE_ERROR` wrapper: `xheader_set_single_keyword` → `call usage` → `usage`'s error
path ends in `call exit@plt`). The next symbol is `xheader_format_name` @ 0x1c080
(985 bytes).

### angr (stored) — complete, 9 LOC

```c
// Function: xheader_set_option @ 0x1c010
void xheader_set_option(void)
{
    unsigned long long v2;  // rbx
    unsigned long long v0;  // [bp-0x10]

    v0 = v2;
    sub_41c020();
    return;
}
```

Note: `sub_41c020` = load base 0x400000 + 0x1c020 — an address **inside
xheader_set_option's own prologue**. angr's output is a failed/truncated
decompilation (prologue spill + a call into its own body), not the source's
strtok/strchr loop.

### kuna (stored = fresh, identical; 227 LOC, 14 gotos — trimmed to the head; the remaining ~160 lines are the swallowed body of xheader_format_name, labels 1c0f9…1c3e8)

```c
// Function: xheader_set_option @ 0x1c010
  char * xheader_set_option(unsigned long a0)
  {
    ... 20 locals incl. v19 fs_offset, v20..v24 stack slots (xheader_format_name's frame) ...
    v4 = (char *)strtok(a0,0x6079c);
    if (v4 != (char *)0x0) {
      do {
        v5 = strchr(v4,0x3d);
        if (v5 == (char *)0x0) {
          v7 = v4;
          sub_1a0f0();                 // xheader_set_single_keyword — noreturn, NOT detected
          v23 = 0x6079c;               // <- from here on this is xheader_format_name's
          v22 = *(void *)(v19 + 0x28); //    body (0x1c080+), decompiled as fall-through
          v3 = *v5;
          v24 = v4;
          if (v3 != '\0') { ... goto label_1c0f9; }
          ... goto label_1c18d;
        }
        sub_1a320();                   // xheader_set_keyword_equal — correct
        v4 = (char *)strtok(0,0x6079c);
      } while (v4 != (char *)0x0);
    }
    return (char *)0x0;
    do {
label_1c0f9:                           // %d/%f/%n/%p template-expansion loop of
      ...                              // xheader_format_name: ~160 more lines,
label_1c18d: ... label_1c1c0: ...      // 14 gotos / 11 labels / 3 loops total
label_1c208: ... label_1c210: ...
label_1c22f: ...
    *v4 = '\0';
    return v7;
  }
```

### kuna (fresh, `--option listing on`) — 17 LOC, 0 gotos, faithful to source

```c
// Function: xheader_set_option @ 0x1c010
void xheader_set_option(unsigned long a0)
{
  char *v1; // rax

  v1 = (char *)strtok(a0,0x6079c);
  if (v1 != (char *)0x0) {
    do {
      if (strchr(v1,0x3d) == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
        sub_1a0f0(v1,0);
      }
      sub_1a320();
      v1 = (char *)strtok(0,0x6079c);
    } while (v1 != (char *)0x0);
  }
  return;
}
```

Metrics: stored kuna 227 LOC / 14 gotos / 11 labels / 28 ifs / 3 loops; fresh (default)
identical → **fresh_reproduces: true**; fresh with `listing on`: 17 LOC / 0 gotos /
2 ifs / 1 loop.

Rescore (`--option listing on`): before **241.0** (validates the recorded score on
current main d12ef72), after **20.0** (Δ −221), `source_nodes: 1`,
`source_file: .../full_run/O0/tar/compiled/buffer.i`.

## Analysis

**Symptom.** kuna decompiles past a call to an unresolved internal noreturn wrapper
and swallows the entire next function. `xheader_set_option`'s final instruction is
`call xheader_set_single_keyword` (stripped: `sub_1a0f0`), which GCC placed with no
code after it because the callee is noreturn-by-body
(`xheader_set_single_keyword` → `usage` → `exit@plt`, depth-2 chain through
**internal, name-stripped** functions). kuna treats the call as returning, flows
through the padding into `xheader_format_name` (985 bytes), and emits its whole
%d/%f/%n/%p template loop inside `xheader_set_option`: 227 LOC / 14 gotos, GED 241.
This is a genuine kuna-specific bug — ghidra (161) partially overran too; IDA (23)
discovered the noreturn and stayed bounded.

**Why existing default-on noreturn options don't fire.** `noreturn_known` (S1) and
`noreturn_extern`/`noreturn_externmatch` (S2) are *name-list* matchers — the callee
here is an internal stripped `sub_1a0f0`, so no name matches. `noreturn_disc`
(default-off) needs ≥3 call sites with no valid fall-through as evidence;
`xheader_set_single_keyword` doesn't have them. `noreturn_propagate` (default-ON
since DIV-14, the kuna analog of angr CFGFast's returning fixpoint, PR #52) is
exactly the right mechanism — seeds from the Known list (`exit`), concludes `usage`
then `xheader_set_single_keyword` noreturn at fixpoint — **but it is a Listing
consumer and `listing` is default-off**, so it was a no-op in the benchmark run.
Flipping `--option listing on` alone closes the gap (241 → 20, output faithful to
the 12-line source function).

**Why angr scored 0 — a metric-artifact overlay.** angr did not win by structuring:
its stored output is a 9-LOC truncated stub whose only call targets base+0x1c020,
an address inside the function's own prologue (a failed angr decompilation). It
scored 0 because the **source CFG is degenerate**: rescore reports
`source_nodes == 1`, and the extracted `source_file` is `buffer.i` — the *wrong
translation unit* (`xheader_set_option` lives in `xheader.c`). Against a 1-node
source graph, GED ≈ output-CFG size, so the tiniest (most broken) output wins and
kuna's inflated overrun output pays maximum penalty. The residual after=20 vs
angr=0 is therefore meaningless; the actionable, real component of the 241 margin
is the boundary overrun, which the option removes. (artifact_suspect was false only
because IDA's 23 didn't cluster with kuna; the degenerate source makes every score
in this group unreliable regardless.)

**Sibling.** `O2-tar-tar-xheader_set_option` (recorded kuna 216, angr 0, bucket
"ahead", already ARTIFACT-SUSPECT): at plain O2 the function is genuinely 976 bytes
(callees inlined), kuna's 146 LOC is legitimate content, and angr's stored output is
the same truncation pattern (21-LOC prologue stub calling base+0x1ca38 inside its
own body) — a pure scoring artifact there; `listing on` trims kuna 146 → 139 LOC /
4 → 2 gotos (inlined USAGE_ERROR tails), but the margin itself is artifact.

## Proposed fix

No new kuna feature needed — the mechanism (`noreturn_propagate`, S1
external-refinement, kuna-analysis Listing consumer) already ships and is default-on.
Two small follow-ups:

1. **Benchmark harness**: run the decbench kuna backend
   (`decbench/decompilers/raw/kuna_raw.py` → `kuna decompile-all`) with
   `--option listing on` so the default-on Listing consumers
   (`noreturn_propagate`, `noreturn_disc` eligibility) actually execute on real-ELF
   targets — this case alone is Δ −221. Longer-term, evaluate flipping `listing`
   default-on for real-ELF inputs (per the 0/675-datatest + ≤5% speed policy: the
   Listing is real-ELF-only, so datatests stay byte-identical by construction; the
   cost is Listing build time per binary, which `decompile-all`'s load-once model
   amortizes — needs one speed measurement on a large binary before a DIV entry).
2. **decbench-side bug**: source-function extraction mapped
   `tar::xheader_set_option` to `O0/tar/compiled/buffer.i` with `source_nodes==1` —
   file the degenerate-source/wrong-TU extraction bug; every score in this group
   (both siblings, all decompilers) is unreliable until then, and angr's 0 rewards
   a truncated stub.

Risks: none in kuna source for this record (option-only). The harness flag change
should be gated behind the campaign's rescore diff to catch any function where the
Listing consumers regress output.
