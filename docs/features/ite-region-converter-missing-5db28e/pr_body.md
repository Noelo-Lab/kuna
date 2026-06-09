## stackguard — strip the glibc `-fstack-protector` canary epilogue (angr StackCanarySimplifier port)

Closes the angr-vs-kuna gap surfaced by angr's
`test_ite_region_converter_missing_break_statement::authenticate`
(`binaries/tests/x86_64/ite_region_converter_missing_breaks`, x86-64 GCC, `-fstack-protector`).

See [`docs/features/ite-region-converter-missing-5db28e/analysis.md`](../docs/features/ite-region-converter-missing-5db28e/analysis.md) for the full analysis.

### The gap

The function saves the TLS canary at entry (`canary = *(fs:0x28)`) and at every exit runs
`if (canary != *(fs:0x28)) __stack_chk_fail(); return v;`. That canary check is a **single
shared return point** — so a return deep inside the loop (the `Authenticated!` path) can't
return directly; Ghidra's structurer routes it through a `goto` to the shared tail. angr's
`StackCanarySimplifier` strips the canary epilogue, after which the bare-return tail is
duplicated by `ActionReturnSplit` into each predecessor and the goto disappears.

Metrics (angr reference | kuna default): loc `21 | 40`, **gotos `0 | 1`**, labels `0 | 1`.

> angr also folds the unrelated trailing `if(c) v=1 else v=0` into a ternary `c ? 1 : 0`.
> Ghidra's C printer has **no ternary operator**, so that exact rendering is out of scope —
> stripping the canary epilogue is the reproducible win this PR delivers.

### Before / after (`option stackguard`)

```
  off (default)                         on
  ------------------------------        ------------------------------
  if (v1 == 0) {                        if (v1 == 0) {
    ...                                   ...
    v2 = 1;                               return 1;          // direct, no goto
    goto label_2417;                    }
  }                                     ...
  ...                                   return v2;
label_2417:
  if (v6 != *(int8 *)(v4 + 0x28))       // (canary check + __stack_chk_fail gone)
    v2 = sub_10e0();                     // (sub_10e0 == __stack_chk_fail@plt)
  return v2;
```

### Mechanism

New `decompiler/cpp/kuna_stackguard.{cc,hh}` — `ActionStripStackGuard` (ElementId 4021),
run in `actfullloop` immediately before `ActionReturnSplit`. Detection is **purely
structural** (kuna's BFD console loader doesn't resolve the `__stack_chk_fail@plt` stub to
its name — it renders as `sub_<addr>`): a CBRANCH whose `INT_EQUAL`/`INT_NOTEQUAL` boolean
has BOTH operands deriving from a `LOAD` of `<base>+0x28`, with a handler `CALL` on the
corrupted-canary branch. The corrupted branch is removed with the stock
`Funcdata::removeBranch` (CBRANCH→fall-through, MULTIEQUALs patched); `removeUnreachableBlocks`
collects the orphaned `__stack_chk_fail` block; the dead canary load/store/compare are reaped
by the next deadcode pass. Self-gates once no handler call remains.

Anchor edits (all `(kuna)`-marked, in `UPSTREAM.md`): `architecture.{hh,cc}` flag
`strip_stack_guard`, `options.cc` registration, `coreaction.cc` action, `kuna_stages.cc`
settable+surface rows.

### Option

```
option stackguard on      # strip the canary epilogue (angr-like)
option stackguard off     # default — upstream byte-identical
```

LLM-discoverable via `kuna.catalog` (`source_decompiler: angr`, `change_kind: opt-in-tool`).

### Ablation / parity

- Default **OFF (opt-in)**: stripping deletes real canary-check instructions, and ablation
  with default-ON changes **3** of 675 upstream datatest assertions (Multi-size return #1–3).
  Default output stays byte-identical, so no `docs/divergences.md` entry.
- Gates: `catalog --check` → catalog OK; `run_tests --all --baseline docs/baseline.json` →
  **PARITY OK** (204/204 unit, 675/675 datatest); `make test-stages` → 149/149; new
  two-pass test `tests/stages/ghangr-ite-region-converter-missing-5db28e.xml` + baseline bump.

Demo: `tools/ite-region-converter-missing-5db28e_angr_demo.sh`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
