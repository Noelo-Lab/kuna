## x8664-cvs-863633: collapse duplicate scalar local declarations (`option dedupvardecls`, DIV-7)

Closes the angr-vs-kuna gap on `test_decompiling_x8664_cvs::main` (x86_64/cvs). kuna emitted a
**wall of duplicate local-variable declarations** — on cvs `main` the single stack slot
`stack - 0x3c` (`option_index`) was declared **166 times**, `stack - 0x38` 53×, etc. — where angr
declares each local exactly once. This was the dominant cause of kuna's `main` being ~30% longer
than angr (677 vs 472 loc); with the fix on, `main` drops to **461 loc**.

See [`docs/features/x8664-cvs-863633/analysis.md`](docs/features/x8664-cvs-863633/analysis.md) for
the full side-by-side analysis.

### Why angr is better

angr's variable recovery yields one variable per storage location, so its declaration block lists
each local once. kuna's C printer instead walks **HighVariables** (the W4 `ScopeLocal` Symbol walk
is the documented missing surface), so when the angr-style naming maps many distinct *scalar*
HighVariables that share one stack slot to the **same name + type + storage**, kuna emits one
declaration line *per high* — textually identical, and (strictly) invalid C re-declarations. The
function body refers to all of them by the one shared name, so the duplicate declaration *lines*
are pure noise.

### Mechanism (S9 emit)

`emit_local_var_decls` (`s9_emit/printc.rs`) now skips a declaration whose **fully-rendered
signature** (final declarator type + name + array adornment + storage comment) is byte-identical to
one already emitted — the scalar analogue of the composite-symbol (array/struct/union) collapse kuna
already performs, approximating Ghidra's once-per-Symbol `emitScopeVarDecls` walk. The option parser
and the `DeclDedup` signature tracker live in the new module
[`s9_emit/kuna_dedupvardecls.rs`](decompiler/crates/kuna-decomp/src/s9_emit/kuna_dedupvardecls.rs)
(ElementId 4091). Pure presentation: the statement body markup, naming, and which HighVariables
exist are all unchanged — only redundant declaration *lines* are removed (provably lossless: it
removes only lines whose emitted bytes are character-identical to an already-emitted one).

### Option

- **`option dedupvardecls on|off`** (arch flag `dedup_var_decls`), **DIV-7 default-on**.
- Flip off to restore the one-declaration-per-high rendering: `option dedupvardecls off`, or
  `kuna decompile <bin> <fn> --option dedupvardecls off`.
- Discoverable via `kuna catalog --json` (provenance: `source_decompiler="angr"`).

### Ablation / parity / speed

- **Ablation: 0 of 675** upstream datatest assertions change with the feature default-ON
  (`make test` stays **PARITY OK** without baseline regeneration).
- **Speed: +0.14%** on the target (off 2236 ms → on 2239 ms, 5% budget) — an O(decls) HashSet pass.
- A clean ablation + within-budget speed is why this ships **default-ON** (DIV-7).
- Stage test: `tests/stages/ghangr-x8664-cvs-863633.xml` (+2 assertions); `make test-stages` PARITY OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
