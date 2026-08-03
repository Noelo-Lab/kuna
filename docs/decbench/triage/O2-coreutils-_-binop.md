---
case_id: O2-coreutils-_-binop
pool: ida
group_id: coreutils::binop
status: covered-by-option
tier: M
margin: 50
fresh_verdict: reproduces unchanged on today's build (fresh == stored, 11 nested `if`s); `--option condfold wide` collapses them to 4 (`if` count 11 -> 4, LOC 29 -> 15) and is the only thing that moves it — `condfold on` does nothing here
option_closing: condfold=wide
feature_slug: null
scope: small
confidence: high
---

## Side-by-side

**IDA (stored)** — one `if` with an 11-term `&&` chain:

```c
long long binop(char *s1)
{
  unsigned int v1 = 1;
  if ( strcmp(s1, "=") && strcmp(s1, "!=") && strcmp(s1, "==") && strcmp(s1, "-nt")
    && strcmp(s1, "-ot") && strcmp(s1, "-ef") && strcmp(s1, "-eq") && strcmp(s1, "-ne")
    && strcmp(s1, "-lt") && strcmp(s1, "-le") && strcmp(s1, "-gt") )
    LOBYTE(v1) = strcmp(s1, "-ge") == 0;
  return v1;
}
```

**kuna (fresh, no flags — identical to the stored pane)** — 11 nested `if`s:

```c
bool binop(char *a0)
{
  bool v1 = 1;
  if (strcmp(a0,"=")) { if (strcmp(a0,"!=")) { if (strcmp(a0,"==")) { ...
    ... if (strcmp(a0,"-gt")) v1 = strcmp(a0,"-ge") == 0; ... } } }
  return v1;
}
```

Ghidra's pane is the same 11-deep nest (bucket `inherited`; binja = 0, angr = 26).

**kuna with `--option condfold wide`** — 4 `if`s:

```c
bool sub_2920(char *a0)
{
  bool v1 = 1;
  if (((strcmp(a0,"=")) && (strcmp(a0,"!="))) && (strcmp(a0,"=="))) {
    if ((strcmp(a0,"-nt")) && (strcmp(a0,"-ot"))) {
      if (((strcmp(a0,"-ef")) && ((strcmp(a0,"-eq") && (strcmp(a0,"-ne"))))) && (strcmp(a0,"-lt"))) {
        if ((strcmp(a0,"-le")) && (strcmp(a0,"-gt")))
          v1 = strcmp(a0,"-ge") == 0;
      }
    }
  }
  return v1;
}
```

Metrics from `triage --case O2-coreutils-_-binop --option condfold wide`:

| output | loc | ifs |
|---|---|---|
| ida(stored) | 21 | 1 |
| kuna(stored) | 29 | 11 |
| kuna(fresh, condfold wide) | 15 | 4 |

`--option condfold on` produces **no change at all** on this function — only `wide`
moves it.

## Source

`~/github/decbench/results/full_run/O0/coreutils/compiled/lbracket.i` (and `test.i`,
identical — that is why the group has 6 cases across two binaries):

```c
binop (char const *s)
{
  return (((strcmp (s, "=") == 0)) || ((strcmp (s, "!=") == 0)) || ((strcmp (s, "==") == 0)) ||
          ((strcmp (s, "-nt") == 0)) || ((strcmp (s, "-ot") == 0)) || ((strcmp (s, "-ef") == 0)) ||
          ((strcmp (s, "-eq") == 0)) || ((strcmp (s, "-ne") == 0)) || ((strcmp (s, "-lt") == 0)) ||
          ((strcmp (s, "-le") == 0)) || ((strcmp (s, "-gt") == 0)) || ((strcmp (s, "-ge") == 0)));
}
```

One 12-clause short-circuit chain. IDA's single `if` with 11 `&&`s plus the trailing
`strcmp(...) == 0` is that chain, De Morgan'd. kuna's 11-deep nest is the *same*
short-circuit CFG spelled as statements — semantically right, structurally noisy.

## Analysis

### Symptom (one, named)

**A 12-clause short-circuit condition is emitted as an 11-deep `if` nest instead of a
folded `&&` chain.** No gotos, no missing blocks, no wrong values — the condition
tree is correct; only its rendering is unfolded.

### Root cause

This is the known `condfold` family (`docs/options.md`, and the memory note
`kuna-condfold-isComplex-root`): upstream Ghidra's `ruleBlockOr` declines to fold a
condition operand whose block `isComplex()` (more than a couple of printed
statements), and each `iVar1 = strcmp(a0, "-xx");` clause is exactly that. kuna
already ships the relaxation as `--option condfold off|on|wide`
(`p8_structure/kuna_condfold.rs`), default **off**, and it is **not** in the
`aggressive` preset — which is why a no-flag run on today's build still shows the
full nest.

`wide` is a *bounded* relaxation: both admission rules share a printed-width budget
(5 statements at `on`, 9 at `wide` — `MAX_SHAPE_STMTS` / `ShapeVerdict` in
`kuna_condfold.rs`). Twelve `strcmp` clauses do not fit in 9, so the fold breaks into
groups of 3/2/4/2 and four `if`s survive. That is the entire residual: **11 → 4 is a
budget artifact, not a matcher failure.**

### Owning phase

**P8** — Structured AST & Goto Quality (`p8_structure/kuna_condfold.rs`).

### Metric-artifact check

Not an artifact. Source CFG is 23 nodes / 33 edges and non-degenerate; the case is
not `approximated`; `source_ambiguous` is set only because two translation units
(`test.i`, `lbracket.i`) carry the identical function. binja scores 0 here, so a 0 is
reachable.

### Siblings

All 6 cases in the group are the same function in two binaries (`[` and `test`) at
three optimisation levels.

- `O2-coreutils-test-binop`, `O2-noinline-coreutils-{_,test}-binop`: identical
  symptom (same margins 50/50/50).
- `O0-coreutils-{_,test}-binop` (margin 18, flagged `artifact? yes`): a **different
  and much better** shape — today's build emits a clean early-return cascade
  (`if (!strcmp(a0,"=")) return 1; …`, `// return-dupe x11`), which is arguably closer
  to the source than IDA's. No `condfold` work is needed there.

## Proposed fix

**No new feature. This is a default-flip / policy question, and a narrow one.**

1. **The flip candidate**: `condfold` is default-off and outside the `aggressive`
   preset. Every un-folded short-circuit condition in the ida pool is a `condfold`
   case; a measured `condfold=wide` default (or adding it to `aggressive`) is the
   real decision, and it needs the full DIV-row treatment: datatest ablation, speed
   budget, and a benchmark rescore over the whole group.

2. **The residual, if anyone wants the last 3 `if`s**: the only thing standing
   between `wide` and IDA's single `if` is the shared printed-width budget in
   `p8_structure/kuna_condfold.rs`. A third policy level (an `unbounded`/`max` value
   on the existing option — options take values, not just on/off) would close it in
   one module. I would rank that *below* the default-flip: an unbounded fold produces
   a 12-term condition on one source line, and whether that reads better than a
   4-deep nest is a taste call, which is precisely what a value-carrying option is
   for. It is not a correctness item — kuna's output here compiles and computes the
   right value.

GED value: real but capped. The Joern CFG for `A && B` and for `if(A){if(B){}}` are
close, so the 11→4 collapse is worth less than the `if`-count suggests; measure with
`scripts.decbench.rescore --case O2-coreutils-_-binop --siblings` before promising a
number.
