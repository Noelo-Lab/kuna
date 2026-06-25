# call-return variable folding — analysis

- **angr testcase**: `test_call_return_variable_folding` (angr/tests/analyses/decompiler/test_decompiler.py)
- **binary**: `x86_64/decompiler/ls_gcc_O0`
- **function**: `print_long_format` (`0x40ade4`), arch `x86_64`
- **angr version**: 9.2.213

## What the angr test asserts

```python
assert "if (timespec_cmp(" in dec.codegen.text or "if ((int)timespec_cmp(" in dec.codegen.text
assert "&& localtime_rz(" in dec.codegen.text
```

i.e. a **call's return value that is used exactly once is folded into its use
site** — the call expression is inlined into the condition rather than being
spilled to a named temporary:

```c
// angr (good)
if (timespec_cmp(...) <= -1) { ... }
... && localtime_rz(...) != NULL ...
```

## What kuna does (the gap)

kuna spills every call return into a named local and only then uses it:

```c
// kuna (default, before)
v5 = timespec_cmp();
if (v5 <= -1) {
  gettime((timespec *)current_time);
}
...
(v10 = localtime_rz(dat_227438,&v19,(tm *)v18), v10 != (tm *)0x0)   // comma-operator spill
```

## Root cause (the owning stage / sub-stage)

S6 / `merge-aggressiveness` → `ActionMarkExplicit` (`coreaction.cc:3105`
`baseExplicit`, ported to
`decompiler/crates/kuna-decomp/src/s6_variables/coreaction_cleanup.rs::base_explicit`).

`baseExplicit` decides whether a Varnode gets its own statement (**explicit**)
or is inlined at its single use (**implied**). Ghidra/kuna force **every** call
output explicit:

```rust
if dop.is_call() {
    if dop.code() == CPUI_NEW && dop.num_input() == 1 { return -2; }
    return -1;            // <-- always explicit: the call output is never inlined
}
```

This is conservative-by-design: inlining a call output moves the call's
evaluation to the use site, which would reorder it past other side-effecting
statements. angr performs the equivalent "call-expression folding" but only
where it is order-safe.

## Hypothesis / fix (confirmed empirically)

Relax the `is_call() ⇒ explicit` rule, **gated by a new option
`foldcallret` (default-OFF)**, for call outputs that are *order-safe* to fold:

1. the call output has **exactly one** descendant (single use), and
2. that use is in the **same basic block** as the call, and
3. there is **no intervening side-effecting / memory op** (CALL / CALLIND /
   CALLOTHER / STORE / LOAD) strictly between the call and its use.

When those hold, fall through to the normal descendant-count path so the printer
inlines the call expression at the use site.

A throwaway prototype (relaxing only on single-use) already reproduced the angr
output on `print_long_format`:

```c
if (((... ) && (v4)) && (localtime_rz(dat_227438,&v19,(tm *)v18) != (tm *)0x0)) {
  if (timespec_cmp() <= -1) { ... }
```

— matching both angr substrings (`if (timespec_cmp(` and `&& localtime_rz(`).
The `numInstances()>1` merge guard did **not** block it: these call outputs are
single-instance highs.

## Scope: small

One gated decision point in `base_explicit`, the foldability predicate isolated
in a new module `kuna_callretfold.rs`, plus the standard option flag plumbing
(seams/architecture/options/stages.toml) that every existing on/off option
already carries. No new pass type, no Action, no S7 surgery. Modelable exactly
like the existing on/off feature options (`flagcompare`, `booleanmask`).
