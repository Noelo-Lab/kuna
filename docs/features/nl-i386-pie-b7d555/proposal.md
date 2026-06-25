# [PROPOSAL] i386 PIE PLT import resolution (closes `test_decompiling_nl_i386_pie::usage`)

**Worker:** `w1782360925-19` · **Branch:** `feat/angr-nl-i386-pie-b7d555` · **Opportunity:**
`test_decompiling_nl_i386_pie::usage` · angr 9.2.213.

This is a **draft proposal** parked for a human go/no-go. The gap is fully root-caused and the
fix is small, but it does not fit the standard feature-worker mold and changes **default**
decompiler output for an entire binary class. Please approve the approach (and the two
sub-decisions at the end) before an implementation worker is dispatched on this branch.

## The problem

kuna decompiles `usage` in `i386/nl` (an i386 **PIE** binary) into ~209 loc of broken C — a
spurious `do{}while(true)` loop, a `goto`, three un-unified `// esp` stack-pointer values,
explicit frame-slot stores, dropped call arguments, a recovery-failure marker, and unresolved
`sub_<addr>()` call names. angr produces ~80 clean loc with named libc calls. Full root-cause in
[`analysis.md`](analysis.md); side-by-side in [`angr-vs-kuna.txt`](angr-vs-kuna.txt).

**Root cause (one construct):** `usage` ends in a shared `exit(a0)` tail. kuna never marks
`exit@plt` no-return, so the `call exit` is assumed to fall through, manufacturing a bogus
back-edge → a spurious loop → an unresolvable stack-pointer MULTIEQUAL cycle → the entire
structural failure. `exit` isn't flagged no-return because the i386-PIE PLT stub for it is never
named: `kuna-analysis/src/s1_loader/elf_plt.rs::decode_i386` decodes only the non-PIC
`FF 25 <abs32>` stub form and **skips** the PIE `FF A3 <disp32>` (`jmp *disp32(%ebx)`) form (a
"documented seam"). That form is statically decodable: `got = GOT_base + disp32`
(`_GLOBAL_OFFSET_TABLE_` = `0x9edc`; `_exit@plt` `jmp *0x14(%ebx)` → `0x9ef0` = the `_exit`
`R_386_JUMP_SLOT` r_offset — verified).

## The angr reference

angr resolves PLT/GOT imports as part of CLE loading and consults a libc no-return database
(`exit`, `abort`, `__assert_fail`, …); the no-return marking removes the dead fall-through that
would otherwise loop. kuna already has the *equivalent* machinery (`resolve_plt_imports` +
`NoReturnKnownPass` + the engine's flow-halt) — it only lacks the i386-PIE PLT-stub decode that
feeds it.

## Proposed fix (single function)

In `kuna-analysis/src/s1_loader/elf_plt.rs`:

1. Derive the i386 GOT base once (prefer the `_GLOBAL_OFFSET_TABLE_` symbol value; fall back to
   the `.got.plt` then `.got` section address). Thread it into `decode_i386` (precedent:
   `decode_ppc_text` already takes the `object::File` to compute its TOC base).
2. Extend `decode_i386` to also match `0xFF 0xA3 <disp32>` and `record(stub, GOT_base + disp32,
   …)` — exactly as the existing `FF 25` arm records, keeping the `endbr32`/`bnd` stub-start
   back-up. Leave the `FF 25` absolute arm untouched (non-PIE i386 stays byte-identical).
3. Update the `decode_i386` doc comment (drop the "not statically decodable — skipped" seam
   note).

It is the i386-PIE analog of the already-shipped `decode_x86_64` (RIP-relative) and
`decode_aarch64` (adrp/ldr veneer) decoders.

## Why this needs human go/no-go (does not fit the worker mold)

1. **Wrong tier for the prescribed shape.** The fix is an **analysis-tier loader** change
   (`kuna-analysis`), not a `kuna-decomp` Action/Rule. It cannot carry the prescribed
   `kuna_<slug>.rs` module, Architecture-struct flag, ElementId (4000+), or
   `coreaction*/universalaction` registration. The improvement is delivered by a **multi-pass
   cascade** (PLT decode → import-name fact → no-return fact → engine flow-halt → loop collapse →
   stack recovery), not a single gated op-rewrite.

2. **Default-behavior change for a binary class.** Like the sibling x86_64/aarch64 PLT decoders,
   PLT import naming ships **unconditionally** (loader fidelity — "only ever produces more
   names"). This changes default output for **every i386-PIE binary**, which a human should
   approve. *Open sub-decision (A):* ship it **unconditionally** (matching the sibling decoders,
   the decider's recommendation) **or** gate it behind a default-on `--option`/`settableTable`
   row modeled on **`mips_gp`** (itself an arch-specific PIC GOT-recovery loader pass that *is*
   option-gated and catalog-visible) to honor the standing "output-changing ⇒ `--option`-
   flaggable" requirement?

3. **The stage-test harness can't exercise it.** `tests/stages/*.xml` accepts only raw
   `<binaryimage>` bytechunks, which carry no `.rel.plt` / `.dynsym` / GOT structure — a stage
   XML can never reach the PLT/no-return path. So the prescribed `tests/stages/ghangr-<slug>.xml`
   + baseline bump is **not buildable**. *Open sub-decision (B):* approve testing via a vendored
   i386-PIE ELF fixture + a `kuna-console/tests/verify_i386_pie_plt.rs` integration test (driving
   `bootstrap_from_object` → `load function usage` → `print C`, asserting named libc calls and
   the absence of the spurious `do{}while(true)`/`goto`/`sub_<addr>`), plus a focused unit test
   in `elf_plt.rs`/`noreturn.rs` over the fixture — modeled on `verify_aarch64_plt.rs` and
   `noreturn.rs::run_over_fauxware_flags_exit_only`. Fixture: vendor `nl` (48 KB) or build a
   minimal `gcc -m32 -fpie -pie` i386-PIE ELF under `kuna-analysis/tests/fixtures/`.

## Speed / risk

- **Speed:** negligible — adds one extra opcode-pattern check per PLT-section byte at load time
  (no per-decompile cost). Will be measured (`scripts.pipeline.timeit`, off vs on) at
  implementation time per the standing speed requirement.
- **Risk:** low. The `FF 25` arm is unchanged (non-PIE i386 unaffected); the `FF A3` arm only
  *adds* names, cross-checked against the dynamic-reloc GOT-name map (an unmatched GOT target is
  simply not recorded). Blast radius on the 675 datatests is **zero** (bytechunk corpus never
  calls `resolve_plt_imports`; no i386-PIE binary present). `make test` stays PARITY OK.

## Proposed option name (if sub-decision A → gated)

`i386_pie_plt` — "Decode i386 PIE GOT-relative (`jmp *disp(%ebx)`) PLT stubs so dynamic imports
(exit/dcgettext/…) are named and known-no-return functions are flagged." Default on. Modeled on
the `mips_gp` settableTable row. (Next free ElementId is 4100 if a marshaled gate element is
wanted; an analysis-pass option may not need one.)

## Decider verdict (verbatim)

Recorded in `record.json` under `decisions` (scope: **small** as a fix; the decider recommended
shipping unconditionally as loader fidelity with the integration-test harness — this proposal
elevates it to a human go/no-go solely because of the mold/default-behavior/test-harness
deviations above).
