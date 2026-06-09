# Plan — `stackguard` (strip glibc stack-protector epilogue)

Mirror of angr's `StackCanarySimplifier`. New file `kuna_stackguard.{cc,hh}`; option `stackguard`
(on/off), `Architecture::strip_stack_guard` flag, ElementId 4021. Default-OFF during dev.

## New code (`kuna_stackguard.cc` / `.hh`)

- `class ActionStripStackGuard : public Action` with `clone()` and `apply(Funcdata&)`.
- `apply`:
  1. `if (!data.getArch()->strip_stack_guard) return 0;`
  2. For each `CPUI_CALL` op: `FuncCallSpecs *fc = data.getCallSpecs(op)`; require
     `fc && fc->getName() == "__stack_chk_fail"`.
  3. `failblock = op->getParent()`; require `failblock->sizeIn() == 1`.
  4. `H = (BlockBasic*)failblock->getIn(0)`; require `H->lastOp()->code() == CPUI_CBRANCH` and
     `H->sizeOut() == 2`.
  5. Robustness gate `isCanaryCompare(cbranch)`: the CBRANCH boolean is written by
     `INT_EQUAL`/`INT_NOTEQUAL`, and BOTH operands peel (through COPY/CAST/SUBPIECE0/MULTIEQUAL)
     to a `CPUI_LOAD` whose pointer is `INT_ADD(reg, 0x28)` (the canary slot vs. fresh fs:0x28).
  6. Find the out-edge index of `H` that targets `failblock`; call `data.removeBranch(H, idx)`.
  7. `data.removeUnreachableBlocks(false, true);`
  8. `return 1;` (one canary per apply; the repeating fullloop re-invokes and self-gates when no
     `__stack_chk_fail` remains).
- `class OptionStackGuard : public ArchOption` toggling `glb->strip_stack_guard`.

## Anchor edits (minimal, `// (kuna)` marked, recorded in UPSTREAM.md)

- `architecture.hh`: `bool strip_stack_guard;` flag.
- `architecture.cc` `resetDefaultsInternal`: `strip_stack_guard = false;` (or true if ablation
  is clean and we flip default-ON).
- `coreaction.cc` `universalAction`: `actfullloop->addAction(new ActionStripStackGuard("returnsplit"));`
  immediately before `ActionReturnSplit`.
- `options.cc`: `registerOption(new OptionStackGuard()); // (kuna) ...`
- `kuna_stages.cc`: `settableTable` + `surfaceTable` rows (all provenance fields populated).

## Test (`tests/stages/ghangr-ite-region-converter-missing-5db28e.xml`)

Two-pass `<decompilertest>` over the `authenticate` bytes (self-contained bytechunk + neighbour
symbols memcmp/MD5/xor_/puts/sleep/__stack_chk_fail):
- pass 1 `option stackguard off`: assert the bug present — `__stack_chk_fail` call rendered and a
  `goto` label in the tail.
- pass 2 default (after we decide default): assert the fix — no `__stack_chk_fail`, no `goto`.

## Ablation / default

Run `run_tests --all --baseline docs/baseline.json` with default-ON. If 0/675 change → flip
default-ON + DIV-N entry; else keep default-OFF (opt-in-tool). Either way end at PARITY OK.
