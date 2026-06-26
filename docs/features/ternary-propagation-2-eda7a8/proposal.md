# [PROPOSAL] angr ternary propagation — `ternary_propagate`

Close the angr-vs-kuna gap on `test_ternary_propagation_2::print_only_size`: angr renders a
value-producing `if/else` diamond as a single C ternary (`v = cond ? f() : g()`), while kuna
today **produces no output at all** for the target object. This proposal documents the gap and
a multi-step plan, and asks for a human go/no-go before an implementation worker is spent.

See `analysis.md` for the full evidence and `angr-vs-kuna.txt` for the side-by-side.

## The problem

Target function `print_only_size` in `du.o` (ET_REL `.o`, x86-64):

```c
// angr
v2 = (a0 == -1 ? dcgettext(NULL, "Infinity", 5) : human_readable());
fputs_unlocked(v2, stdout);
```

The shape is an `if/else` whose **both arms are side-effecting CALLs** assigning a merge
variable that is then used once. angr's region simplification collapses it into `?:`.

## Why this is large (not a single option-gated pass)

The scope decider (Plan agent, Opus) returned **`scope: large`**, verbatim in `record.json`.
Two independent gaps across two tiers must both be closed:

### Step 1 — Loader prerequisite (S1, `relocobjects`)

kuna cannot load `du.o` at all: loading any function fails with
`Unable to load 512 bytes at r0x00405290`. The `relocobjects` ET_REL loader
(`kuna-analysis/src/s1_loader/elf_reloc.rs`, `loadimage_object.rs`) binds undefined **data**
externs (e.g. `stdout`, referenced via `R_X86_64_PC32`) to a synthetic extern region with
**no backing bytes**, and a read through that region aborts the whole load. The object also
uses `R_X86_64_REX_GOTPCREL` relocs that the loader logs as `unhandled kind GotRelative
(skipped)`.

Plan: back the synthetic data-extern region with zero bytes so reads through a data extern
succeed, and materialise a GOT slot for `R_X86_64_REX_GOTPCREL` that points at the extern
(the angr CLE "extern object" backing-page behaviour). Gate behind the existing
`relocobjects` capability or a new loader sub-option; verify `du.o` (and the other
GOTPCREL-using `.o`s in the corpus) decompile.

This is the angr reference: CLE's `ExternObject` / relocation backend, mirrored by
`elf_reloc.rs`.

### Step 2 — Ternary structuring (S8) — the named angr advantage

Even once loaded, kuna (like Ghidra) renders the diamond as a literal `if/else`. Ghidra emits
`?:` only from `RuleConditionalMove` — a single **side-effect-free** `MULTIEQUAL`; here both
arms contain side-effecting CALLs, so nothing fires.

Plan: a new S8 structuring/print-tree pass (a `kuna_ternary_propagate.rs` Action modelled on
`kuna_branchflip.rs` / `kuna_gotoreduce.rs`) that recognises an `if (c) { v = f(); } else
{ v = g(); }` block merging into a single use of `v`, and rewrites it to `v = c ? f() : g();`.
Requires extending kuna's emit to render a side-effecting `?:` (today `?:` only comes from
`RuleConditionalMove`). angr reference: its `RegionSimplifier` / "ternary propagation" pass.

### Scope budget exceeded

- **>1 new module** (loader change + a new S8 module).
- **Two tiers**: S1 loader (kuna-analysis) + S8 structuring (kuna-decomp).
- **S8 beyond a single gated early-return**: folds two assignment arms + the merge into one
  statement, plus new side-effecting `?:` emit.
- **No end-to-end witness today**: the named target can't be loaded until Step 1 lands, so the
  before/after demo on `du.o` is empty and a firing stage test could only be a contrived
  externless bytechunk.

## Speed / risk assessment

- **Loader (Step 1):** low runtime risk — the loader runs once at `load file`; backing the
  extern region with zeros is O(region size). Risk is *correctness*: data externs would read as
  zero (not the real libc value), which is acceptable (angr does the same — externs are opaque).
- **Structuring (Step 2):** a gated S8 rewrite; default-OFF while developed, ablation-gated to
  default-ON only if 0/675 datatests change and the target speed budget (+5%) holds. Ternary
  folding of side-effecting calls must preserve evaluation order (only fold when the merge use
  dominates and no intervening side effects reorder) — the main correctness risk.

## Proposed option

`ternary_propagate` — S8 structure-recovery, `source_decompiler = "angr"`,
`inspiration = "test_ternary_propagation_2; RegionSimplifier ternary propagation; print_only_size"`,
default-OFF opt-in until ablation+speed clear it. The loader prerequisite (Step 1) may ship as a
separate `relocobjects` extension or its own option, sequenced first.

## Recommendation

Two-PR sequence: **(1)** loader extension so ET_REL `.o`s with data externs / GOTPCREL load
(unblocks this and other `du.o`-style targets); **(2)** the `ternary_propagate` S8 rewrite. A
human should approve before an implementation worker is dispatched, since neither piece fits a
single option-gated session and they are independently useful.
