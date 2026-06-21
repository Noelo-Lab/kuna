# Intentional default-behavior divergences from upstream

Option-gated sub-stage exposures (the GH-558 pattern) keep default output
byte-identical to upstream and are NOT listed here — they live in `UPSTREAM.md`
*Divergence* (file-level) and `tests/stages/` (behavior). This file records the cases
where kuna **intentionally changes default behavior**: unambiguous bugs where keeping
the upstream default would mean shipping wrong output. Each entry records the flip,
the justification, and the measured effect on the upstream test suite (the
gh558-experiment protocol: run the 204+675 upstream assertions, list every change).

---

## DIV-1: GH-2786 — nested sign tokens merge into the `--`/`++` operator

- **Flip**: `PrintLanguage::parentheses` (printlanguage.cc, unary_prefix case) now
  parenthesizes when two identical `-` (or `+`) unary-prefix tokens would abut.
  Upstream printed `-(-x)` as `--x`.
- **Justification**: `--x` re-parses as pre-decrement — a semantically different,
  assignable expression. Emitting source that means something else is an unambiguous
  bug, not a representation policy; there is no legitimate "other choice" to expose
  as a sub-stage. (Float double-negation is deliberately not folded by analysis for
  NaN/sign correctness — maintainer-confirmed — so the renderer must handle it.)
- **Changed upstream assertions**: **0 of 675** (and 204/204 unit tests unchanged) —
  `docs/baseline.json` still passes as PARITY OK without regeneration; no upstream
  datatest exercises adjacent sign tokens.
- **Verification**: `tests/stages/gh2786-unaryspacing.xml` asserts the fixed form
  `-(-in_f0)` is emitted and the mis-parsing `--in_f0` token is gone (MIPS:BE:32
  bytes from the issue attachment).
- **Date**: 2026-06-06.

---

## DIV-2: eight stage-model sub-stage fixes become the default

- **Flip** (user decision 2026-06-07: "all of these on by default if they are not
  overly destructive"): `compareform` → **original**, `arraynotation` → **on**,
  `thumbfuncptr` → **on**, `inferfuncentry` → **on**, `booleanmask` → **on**,
  `addcarrychain` → **on**, `ovlesssimplify` → **on**, `memsetrecover` → **on**
  (with a new minimum-fill guard: ≥2 COPYs and ≥16 bytes, so a lone string NUL
  terminator is never claimed as a memset — found via the Stack string #6 ablation).
  Every option remains settable; `option <name> off` (or `compareform canonical`)
  restores the upstream behavior per option.
- **Deliberately NOT flipped** (destructive, stay opt-in):
  - `returnpair single` — ablation showed 3/675 upstream tests legitimately need the
    multi-register join (real wide returns would be truncated);
  - `v850indirectbranch` — the jmp-shape predicate (CALLIND of a named register)
    matches every register-indirect call on non-V850 arches; the global flip diverges
    into unbounded jump-table sub-queries. Correct only as a per-program assertion.
- **Justification**: these eight all resolve open upstream issues (GH-558, 8471,
  6930, 1282, 7190, 8913, 9230, 1537) where the kuna output is strictly more faithful
  to source or strictly more readable, and the ablation matrix shows their corpus
  effect is text-only (compareform 12, arraynotation 10) or nil (the other six, after
  the memset guard).
- **Changed upstream assertions: 22 of 675** (204/204 unit tests unchanged), exactly
  the two presentation sets, no interaction effects:
  - `compareform original` (12): Bitfields #2/#7/#11, Boolean thru Less-than #1,
    Ccmp test #1, Else-if #14, For-loop with skip #2, If/Switch #2, Inline target #4,
    No for-loop global call #1, Promotion on compare #1, Signed byte #4
  - `arraynotation on` (10): Heap string #3/#4/#7, Pointer Compare #1,
    Pointer to array #1/#3, Relative pointers #6, Stack string #1/#11/#12
  The 22 datatest regexes were updated in place to assert the new rendering
  (old forms remain in git history); `docs/baseline.json` regenerated.
- **Stage-testcase inversion**: the nine option-gated `tests/stages/gh*` testcases
  for flipped options now assert fix-under-default and old-behavior-under-
  `option … off`, so both directions of each decision stay pinned. Two recorded
  default-interactions: GH-8471's symbolic pointer renders `&fn[1]` (arraynotation)
  and GH-7190's opt-out garbage shows `0 <=` (compareform).
- **Date**: 2026-06-07.

---

## DIV-3: six more stage-model fixes become the default

- **Flip** (this session, per the standing "auto-flip ablation-clean fixes" decision):
  `flagcompare` → **on**, `stackprobeloop` → **on**, `dynamichashmax` → **on**,
  `arraystride` → **on**, `condexeplace` → **on**, `inputvarnodeadjust` → **on**.
  Every option remains settable; `option <name> off` restores the upstream behavior.
- **Deliberately NOT flipped** (destructive, stay opt-in):
  - `switchmodbound` — may over-bound an unrelated register-indirect jump on a program
    whose switch genuinely has no modulo/and-mask guard; correct only per-program.
  - `stackalias` — relaxes a dead-store race in alias analysis; sound on the repro but
    a global default could keep a genuinely-dead store elsewhere.
  - `sparcstructret` — reclassifies a post-call `unimp` as fall-through; correct for the
    SPARC struct-return ABI but would mis-handle a real trap on another target.
- **Justification**: all six resolve open upstream issues (GH-1276/8777, 8017, 8467,
  8724, 9203, 9218) where the kuna output is strictly more faithful, and the ablation
  (all six forced on, full 204+675 suite) changed **0 of 675** assertions — their fix
  patterns do not occur in the upstream corpus.
- **Changed upstream assertions: 0 of 675** (204/204 unit unchanged); `docs/baseline.json`
  passes as PARITY OK without regeneration.
- **Stage-testcase inversion**: the five two-pass testcases (gh1276, gh8777, gh8017,
  gh8724, gh9203) now set `option <name> off` for the bug pass so both directions stay
  pinned under the new default (the two single-pass testcases gh8467/gh9218 assert the
  fix directly). `docs/baseline-stages.json` regenerated (121 assertions).
- **Date**: 2026-06-08.

---

## DIV-4: lowered comparison-cascade switch recovery becomes the default

- **Flip**: `loweredswitch` → **on** (the angr LoweredSwitchSimplifier port). Reconstructs a
  GCC-lowered comparison cascade (a binary-search `if/else` tree over one variable) into a C
  `switch` with a synthesized BRANCHIND + JumpTable; `option loweredswitch off` restores the
  upstream `if/else-if` rendering. Unlike DIV-1..3 this is **not a correctness fix** — it is a
  deliberate structure-recovery / readability default (SAILR-style), recorded here because it
  changes default output.
- **Corpus-clean guard**: the pass fires only on the GCC **binary-search structure** — the
  cascade must contain at least one range/`jle` split. A purely linear equality chain is treated
  as a hand-written `if/else-if` and left alone. This guard is exactly what keeps the default off
  upstream's `elseif` / `copytrim` / `partialunion` comparison chains.
- **Changed upstream assertions: 0 of 675** (204/204 unit unchanged); `docs/baseline.json` passes
  as PARITY OK without regeneration. **Measured**: without the binary-search guard the flip
  regressed **10** assertions (6 `elseif`, 3 `copytrim`, 1 `partialunion`) by converting
  hand-written cascades to switches; adding the guard made the ablation clean while still
  recovering the `fmt`/`main` getopt switch.
- **Mechanism**: a stage-model feedback edge (mechanism c restart) — detect the cascade late on
  the simplified CFG, then synthesize the S2 BRANCHIND+JumpTable artifact pre-SSA on the restart
  so heritage rebuilds phi. See `PROGRESS.md` and `kuna_loweredswitch.{hh,cc}`.
- **Stage-testcase**: `tests/stages/ghangr-loweredswitch.xml` (`fmt`/`main`) sets `option
  loweredswitch off` for the bug pass so both directions stay pinned under the new default.
  `docs/baseline-stages.json` (128 assertions).
- **Date**: 2026-06-08.

## DIV-5: angr-style default naming becomes the default

- **Flip**: `namestyle` → **angr** (master toggle; `option namestyle ghidra` restores the
  upstream Ghidra scheme byte-for-byte). Default decompiler output is re-skinned to read like
  the angr decompiler:
  - locals / temps / `extraout_*` / `unaff_*` / `in_*` → `v1`, `v2`, `v3` … (sequential);
  - arguments with a generated default name → `a0`, `a1` … (by signature slot);
  - global data (`<type>Ram<addr>`, volatile annotations) → `dat_<addr>` (lowercase hex, no
    leading zeros, no `0x`);
  - unnamed functions `func_0x…` → `sub_<addr>`; code labels `code_r0x…` → `label_<addr>`;
  - **every local declaration** gains a trailing source-location comment: `// rax` (register),
    `// stack - 0x10` (frame-relative), `// rdx:rax` (a split/`join` value, decoded into its
    register pieces), or `// tmp` (a decompiler-internal SSA temporary with no machine home —
    `v1`/`v2`-style). Dynamic (merged) symbols are resolved to a representative Varnode via
    `Funcdata::findLinkedVarnode` so they are annotated too. Symbols with real/recommended
    names are untouched.
  Like DIV-4 this is **not a correctness fix** — it is a deliberate presentation default
  (P0 surface-rendering, S9 sub-stage `naming-policy`), recorded here because it changes
  default output.
- **Mechanism**: a per-`Architecture` flag `name_style_angr` (default on) read at every naming
  site. Local/argument/global naming is intercepted in `Scope::buildDefaultName` and the
  `persist` arm of `ScopeInternal::buildVariableName`; function names in
  `Architecture::nameFunction`, `PrintC::genericFunctionName`, `FspecSpace::printRaw`; labels in
  `PrintC::emitLabel`; the location comment in `PrintC::emitVarDeclStatement`; the volatile-data
  annotation in `PrintC::pushAnnotation`. Helpers + the `OptionNameStyle` option live in
  `kuna_naming.{hh,cc}`. The cross-function default-name guard in `ActionNameVars::makeRec`
  now recognises both `param_N` and the new `aN`/`vN` defaults (`kunaIsGeneratedName`).
- **Changed upstream assertions: 185 of 675 re-pinned in place** to the new default names
  (50 datatest files; 204/204 unit unchanged); plus one script-name update (`pointerrel.xml`
  `rename`/`retype` commands). `docs/baseline.json` keys are unchanged (same assertion names,
  same pass set) and stay PARITY OK. `option namestyle ghidra` reproduces the pre-DIV-5
  rendering for any consumer that needs upstream-identical names.
- **Caveat**: a global symbol is named once in the persistent global scope, so toggling
  `namestyle` mid-session does not re-name already-created globals (locals/args/functions do
  re-derive on the next `decompile`). Set the option at startup for a uniform scheme.
- **Stage-testcase**: `tests/stages/namestyle.xml` (a loop calling an unnamed helper and
  writing a global) decompiles once at the default and once under `option namestyle ghidra`,
  pinning both the angr names (`a0`/`v1; // eax`/`sub_401106`/`dat_40402c`) and the restored
  Ghidra names (`param_1`/`iVar1`/`func_0x00401106`). `docs/baseline-stages.json` (141 assertions).
- **Date**: 2026-06-09.

## DIV-6: residual unknown types render as real C types

- **Flip**: `realtypes` → **on** (master toggle; `option realtypes off` restores the
  upstream `xunknownN`/`undefined<N>` rendering). A value whose real type type-inference
  never resolves (metatype `TYPE_UNKNOWN`) is relabelled to a size-correct **real C type**
  instead of the `xunknownN` placeholder:
  - 1 byte → `char`;
  - 2/4/8 bytes → `unsigned short` / `unsigned int` / `unsigned long` (or `unsigned long
    long` on an LLP64 target where `long` is 4 bytes) — **unsigned**, since the real sign is
    genuinely unknown (conservative; matches Ghidra's documented "`TYPE_UNKNOWN` is treated
    as an unsigned integer");
  - a pointer-to-unknown (any depth) → `void *` (`void **`, …);
  - sizes with no single natural C type (3/5/6/7/10/16…) keep the `undefined<N>` form.
  Like DIV-4/DIV-5 this is **not a correctness fix** — it is a deliberate presentation
  default (P0 surface-rendering, S9 sub-stage `literal-format`), recorded here because it
  changes default output. It deliberately goes beyond upstream Ghidra, which keeps
  `undefinedN` as first-class types and renders pointer-to-unknown as `undefined *`.
- **Mechanism**: a per-`Architecture` flag `realtypes` (default on, set in `resetDefaults`)
  read by the printc declarator chokepoints (`declarator_parts` / `type_name_for_decl` /
  `push_cast_type`) via a `RealTypeCtx` resolved once per function in `doc_function_full`.
  Pure presentation: the relabel changes only the printed type NAME — the `Datatype` and its
  `TYPE_UNKNOWN` metatype are untouched, so type propagation, merge, cast insertion
  (`ActionSetCasts`), and constant-literal formatting (`'a'` vs `0x61`) are all unchanged.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without
  regeneration) — the corpus has no `<stringmatch>` asserting an `xunknown`/`undefined`
  token, so the relabel is invisible to it. Two kuna integration tests that pinned the old
  `xunknown4 promote_compare` signature were updated in place to `unsigned int
  promote_compare` (the new default); the catalog byte-compat fixture was regenerated from
  the Rust emitter to carry the new 23rd settable.
- **Stage-testcase**: `tests/stages/realtypes-fauxware.xml` decompiles the `authenticate`
  function of the `fauxware` test binary (no prototype declared, so its 8-byte params /
  1-byte buffer / 4-byte temp stay `TYPE_UNKNOWN`) and asserts the real-C-type signature
  `unsigned long authenticate(unsigned long a0,unsigned long a1)`, a `char [8]` buffer and an
  `unsigned int` temp, and that no `xunknown`/`undefined` placeholder survives.
  `docs/baseline-stages.json` (+5 assertions). Single-pass (no `option` command): the kuna
  option-toggle runtime write-path is an unimplemented seam on this tree, so the off-pass is
  exercised by the printc unit test `realtypes_relabels_unknown_bases` instead.
- **Date**: 2026-06-21.
