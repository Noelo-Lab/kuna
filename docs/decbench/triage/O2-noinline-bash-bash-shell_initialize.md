---
case_id: O2-noinline-bash-bash-shell_initialize
group_id: bash::shell_initialize
status: feature-candidate
tier: M
margin: 72
bucket: ahead
fresh_reproduces: true
fresh_ged: 72
option_closing: null
root_cause_family: joern-parse-kuna-emission
feature_slug: cstyle-null-cmp
scope: small
angr_ref: angr.analyses.decompiler.structured_codegen.c.CBinaryOp._c_repr_chunks (cstyle_null_cmp=True default)
kuna_stage: S9 emit (printc.rs OpEmitKind::Binary for CPUI_INT_EQUAL/CPUI_INT_NOTEQUAL)
---

## Side-by-side

Source (bash shell.c `shell_initialize`, from O0 `shell.i`, line markers stripped; body only):

```c
static void
shell_initialize ()
{
  char hostname[256];
  int should_be_restricted;

  if (shell_initialized == 0)
    {
      sh_setlinebuf (stderr);
      sh_setlinebuf (stdout);
    }
  initialize_shell_builtins ();
  initialize_traps ();
  initialize_signals (0);
  if (current_host_name == 0)
    {
      if (gethostname (hostname, 255) < 0)
        current_host_name = "??host??";
      else
        current_host_name = (char *)strcpy (sh_xmalloc((1 + strlen (hostname)), "shell.c", 1958), (hostname));
    }
  if (interactive_shell)
    get_current_user_info ();
  tilde_initialize ();
  should_be_restricted = shell_is_restricted (shell_name);
  initialize_shell_variables (shell_environment, privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_job_control (jobs_m_flag);
  initialize_bash_input ();
  initialize_flags ();
  initialize_shell_options (privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_bashopts (privileged_mode||restricted||should_be_restricted||running_setuid);
}
```

angr (stored; declarations trimmed):

```c
long long shell_initialize(void)
{
    /* ...11 local/extern decls trimmed... */
    if (!shell_initialized)
    {
        sh_setlinebuf(stderr);
        sh_setlinebuf(stdout);
    }
    initialize_shell_builtins();
    initialize_traps(v3, v4, v5, v6, v7, v8);
    initialize_signals(0);
    if (!current_host_name)
    {
        if (gethostname(&v0, 255) < 0)
            current_host_name = "??host??";
        else
            current_host_name = strcpy(sh_xmalloc(strlen(&v0) + 1, "shell.c", 1958), &v0);
    }
    if (interactive_shell)
        get_current_user_info();
    tilde_initialize();
    v9 = shell_is_restricted();
    initialize_shell_variables(shell_environment, privileged_mode || restricted || g_5409d4 || v9);
    initialize_job_control(jobs_m_flag);
    initialize_bash_input(v3, v4, v5, v6, v7, v8);
    initialize_flags(v3, v4, v5, v6, v7, v8);
    initialize_shell_options(privileged_mode || restricted || g_5409d4 || v9);
    initialize_bashopts(privileged_mode || restricted || g_5409d4 || v9);
    return v1 - *((long long *)(40 + v10));
}
```

kuna (fresh = stored byte-for-byte modulo indentation; declarations trimmed):

```c
void shell_initialize(void)
{
  /* ...5 local decls trimmed... */
  v5 = *(void *)(v4 + 0x28);
  if (dat_14099c == 0) {
    sh_setlinebuf(dat_1407e0);
    sh_setlinebuf(dat_140780);
  }
  initialize_shell_builtins();
  initialize_traps();
  initialize_signals(0);
  if (dat_140988 == (char *)0x0) {
                  /* WARNING: branchflip: flipped negated guard for linearity ... */
    if (0 <= (int4)gethostname(v3,0xff)) {
      v2 = strlen(v3);
      RAX = strcpy((char *)sh_xmalloc(v2 + 1,"shell.c",0x7a6),v3);
    }
    else {
      dat_140988 = "??host??";
    }
  }
  if (dat_140978 != 0) {
    get_current_user_info();
  }
  tilde_initialize();
  v1 = shell_is_restricted(dat_140948);
  initialize_shell_variables(dat_140958,((dat_142474 != 0 || dat_14247c != 0) || dat_1409d4 != 0) || v1 != 0);
  initialize_job_control(dat_14248c);
  initialize_bash_input();
  initialize_flags();
  initialize_shell_options(((dat_142474 != 0 || dat_14247c != 0) || dat_1409d4 != 0) || v1 != 0);
  initialize_bashopts(((dat_142474 != 0 || dat_14247c != 0) || dat_1409d4 != 0) || v1 != 0);
  return;
}
```

Triage structural metrics (regex level): angr 0 gotos / 4 ifs / 0 loops; kuna 0 gotos / 4 ifs / 0 loops — **identical control structure**.

## Analysis

**Symptom.** kuna recorded GED 72 (ghidra 84) while angr/ida/phoenix all scored 0, yet kuna's
decompilation is control-flow-identical to angr's and to the source (same 4 ifs, same nesting,
no gotos, no missing or extra code; the branchflip if/else arm swap is CFG-isomorphic). Fresh
kuna (current main, post-Jun-29 SAILR merges) is byte-identical to the stored run: the gap
fully reproduces.

**Root cause (measured, not speculated).** Joern-parsing the three blocks in isolation via
decbench's own `extract_cfgs_from_source` + `cfgutils.similarity.vj_ged` (the exact GED path;
both graphs are under the 60-node approximation threshold, so this is the *exact* metric):

| block | Joern CFG nodes | edges | vj_ged vs source |
|---|---|---|---|
| source `shell_initialize` | 9 | 12 | — |
| angr (stored) | 9 | 12 | **0** |
| kuna (stored=fresh) | 27 | 39 | **72** (matches recorded exactly) |

The 18 extra nodes / 27 extra edges are entirely the three
`((dat_142474 != 0 || dat_14247c != 0) || dat_1409d4 != 0) || v1 != 0` argument expressions.
Micro-experiment (Joern CFG of `g(<expr>)` in a tiny function):

| emission style | nodes | edges |
|---|---|---|
| `a\|\|b\|\|c\|\|d` (bare identifiers, angr/ida/source style) | 1 | 0 |
| `((a \|\| b) \|\| c) \|\| d` (parenthesized, bare) | 1 | 0 |
| `a != 0 \|\| b != 0 \|\| c != 0 \|\| d != 0` | 7 | 9 |
| `((a != 0 \|\| b != 0) \|\| c != 0) \|\| d != 0` (kuna style) | 7 | 9 |

Parentheses are irrelevant. Joern materializes short-circuit `||`/`&&` control flow **only when
the operands are non-trivial expressions** (comparisons); bare-identifier chains stay in one
linear block. kuna (like Ghidra — hence ghidra's 84 in the same family) renders every int-typed
truth test as `x != 0` (upstream PrintC behavior: `CPUI_INT_NOTEQUAL => OpEmitKind::Binary` in
`decompiler/crates/kuna-decomp/src/s9_emit/printc.rs:863`), so each of the 3 call sites explodes
into ~6 extra CFG blocks: 3 x (6 nodes + 9 edges) = +18/+27 → vj_ged 72.

**Why angr wins.** angr's structured codegen has a default-on rendering option
`cstyle_null_cmp` (`structured_codegen/c.py`, `CBinaryOp._c_repr_chunks`): a `CmpNE(x, 0)`
prints as bare `x` and `CmpEQ(x, 0)` as `!x` (see also `if (!shell_initialized)` /
`if (!current_host_name)` in its output). Its `BitwiseOrToLogicalOr` peephole actually *builds*
`(a != 0) || (b != 0)` internally — the null comparisons are then dropped purely at print time.
IDA does the same (bare `||` operands, GED 0). So the winning mechanism is emission style, not
structuring: kuna's CFG recovery here is perfect.

**Artifact check.** Not the angr-truncation artifact: angr's block is complete (all 13 calls +
return), the source CFG is non-degenerate (9 nodes), ida=0 is an independent engine confirming.
The score *is* metric-sensitive (Joern's short-circuit expansion is inconsistent between operand
shapes), but the penalty is deterministic, applies to every decompiler through the same parser,
and kuna can close it with an output change that also reads more like real C — so this is
triaged as a kuna feature gap, not a decbench bug. (Note: the full `scripts.decbench.rescore`
was killed at 400 s while Joern-parsing the multi-MB whole-binary artifact on the shared
machine; the fresh GED above was validated by the extracted-block equivalent of the same
pipeline, which reproduces the recorded 72 exactly.)

**Why no existing option covers it.** `compareform` only toggles `V <= c` vs `V < c+1`
canonicalization, `namestyle=angr` is naming only, `branchflip`/`booleanmask`/`flagcompare`
operate on other idioms; nothing in the catalog changes null-comparison rendering. Option sweep
therefore not applicable (no plausible candidate flips this).

**Siblings.** None (group `bash::shell_initialize` has no sibling cases).

## Proposed fix

**Feature `cstyle-null-cmp` (S9 emit, option-gated, small).** An angr-parity rendering rule in
`kuna-decomp/src/s9_emit/printc.rs`: when emitting `CPUI_INT_NOTEQUAL(x, #0)` /
`CPUI_INT_EQUAL(x, #0)` (integer/pointer `x`, not float) **in a boolean context** — an
if/while condition or an operand of BOOL_AND/BOOL_OR/BOOL_NEGATE — print bare `x` / `!x`
instead of `x != 0` / `x == 0(/(T*)0x0)`. Restricting to boolean contexts keeps it
semantics-safe (angr applies it everywhere, which is lossy for value uses like
`v = (x != 0)`; kuna should not copy that). This also suppresses the `(char *)0x0` null-cast
rendering in guards as a side effect.

- Owning files: `s9_emit/printc.rs` (the `OpEmitKind::Binary(&NOT_EQUAL/&EQUAL)` paths,
  printc.rs:863 area) + a new `settableTable` entry in `stages.toml`; follow the
  count-test/golden checklist in memory `kuna-adding-option-count-tests`.
- Risk: none to analysis (pure print-time); main cost is datatest churn — `x != 0` guards are
  ubiquitous, so default-ON needs either a DIV-14-style per-test `option cstylenullcmp off`
  sweep or a baseline re-pin. Recommended landing: default-OFF first PR (0/675 untouched,
  ~0% speed cost — trivially within the <=5% policy), then flip default-on in a sweep PR once
  the opt-out count is known. Benchmark upside is broad: every function whose guards/args
  contain `||`/`&&` chains of truth tests is currently paying this same Joern penalty
  (ghidra's 84 here shows the whole Ghidra lineage is), so this single rendering rule should
  move many "ahead" cases, not just this one.
