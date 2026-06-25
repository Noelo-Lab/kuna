# angr-vs-kuna gap analysis — `test_decompiling_x8664_cvs::main`

- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/cvs` (x86-64, gcc, not stripped)
- **Function:** `main` @ `0x404df0`
- **angr version:** 9.2.213
- **Reference decompiler:** angr

## Metrics (reference angr | kuna, default)

```
  loc           472 | 677
  gotos          10 | 50
  labels          9 | 26
  switches        1 | 1
  cases          25 | 25
  ifs            41 | 66
  loops           5 | 6
```

angr is ~30% shorter. Two distinct gaps are visible:

1. **(deep, NOT this feature)** kuna emits 50 gotos / 26 labels vs angr's 10 / 9 — a
   region-structuring quality gap (S7). Closing it needs real structuring work and
   would be a *large* proposal, not one option-gated pass.

2. **(this feature)** kuna emits a **wall of duplicate local-variable declarations**.
   The single stack slot at `stack - 0x3c` is declared **~200 times** as
   `int4 option_index; // stack - 0x3c`; `stack - 0x38` is declared **50 times** as
   `char *end; // stack - 0x38`; `stack - 0x58` 3 times as `void **v19; // stack - 0x58`.
   angr declares each local exactly once. These duplicate declarations are the
   dominant contributor to the loc gap (≈250 of kuna's 677 lines are this noise).

## What angr does better (the concrete construct)

angr's variable-recovery produces **one variable per storage location**, so its
declaration block lists each local once. kuna's printer walks **HighVariables**, not
the Ghidra `ScopeLocal` *Symbol* table (the W4 scope walk is the documented missing
surface — see `printc.rs::emit_local_var_decls`, "The W4 ScopeLocal symbol walk is the
missing surface"). When the SSA form leaves many distinct HighVariables that the
naming pass (angr-style, names by stack slot) maps to the **same name + same storage +
same type**, kuna emits one declaration line *per high*. They are textually identical
(`<type> <name>; // <storage>`) and the body refers to all of them by the one shared
name — so the duplicates are pure noise (and, strictly, invalid C: a redeclaration).

The existing code already collapses this for **composite** (array/struct/union) mapped
symbols (`printc.rs:2020-2034`) but explicitly **keeps the per-high behavior for
scalars** ("scalars keep the per-high behavior"). The cvs `main` locals are all
scalars, so the dedup never fires.

## Owning stage / sub-stage

- **Stage:** S9 (Surface Rendering & Refinement) — `emit_local_var_decls`
  (`s9_emit/printc.rs`, C++ `PrintC::emitLocalVarDecls`/`emitScopeVarDecls`).
- **Sub-stage:** local-declaration emission (the symbol-vs-high declaration walk).

## Hypothesis / planned change

Add an option-gated S9 step that, in `emit_local_var_decls`, **collapses declarations
that would render an identical line** (same final declarator type, name, array
adornment, and storage comment) to a single declaration — the scalar analogue of the
existing composite dedup, approximating Ghidra's one-decl-per-Symbol `ScopeLocal` walk.

This is purely presentational: the body output (statements, gotos, casts) is unchanged;
only redundant declaration *lines* are removed. It is safe because two highs that share
a name are already indistinguishable in the body, so one declaration is correct C.

Option: **`dedupvardecls`** (`on|off`), default-off while developing; flip to default-on
only if the full 675-assertion ablation is clean and the speed gate passes.
