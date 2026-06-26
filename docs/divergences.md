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

## DIV-7: duplicate scalar local declarations are collapsed

- **Flip**: `dedupvardecls` → **on** (`option dedupvardecls off` restores the upstream
  one-declaration-per-HighVariable rendering). kuna's C printer walks **HighVariables**
  (the W4 `ScopeLocal` Symbol walk is the missing surface), so when the angr-style naming
  maps many distinct scalar HighVariables that share one stack slot to the **same name +
  type + storage**, kuna emits one declaration line *per high* — a wall of textually
  identical declarations (on x86_64/cvs `main` the single slot `stack - 0x3c` is declared
  166×). The function body refers to all of them by the one shared name, so the duplicate
  declaration *lines* are pure noise and, strictly, invalid C re-declarations. With the
  flag on, a declaration is suppressed when its **fully-rendered signature** (final
  declarator type + name + array adornment + storage comment) is byte-identical to one
  already emitted — the scalar analogue of the composite-symbol declaration collapse kuna
  already performs (arrays/structs/unions), approximating Ghidra's once-per-Symbol
  `emitScopeVarDecls` walk. Inspired by the angr decompiler (one variable per storage
  location, declared once), opportunity `test_decompiling_x8664_cvs::main`.
- **Mechanism**: a per-`Architecture` flag `dedup_var_decls` (default on, set in
  `resetDefaults`, carried into the `ArchSeam`) read by `PrintC::emit_local_var_decls`
  (`s9_emit/printc.rs`); the option parser + the `DeclDedup` signature tracker live in
  `s9_emit/kuna_dedupvardecls.rs`. Pure presentation: only redundant declaration *lines*
  are removed — the statement body markup, naming, and which HighVariables exist are all
  unchanged. The collapse is provably lossless (it removes only lines whose emitted bytes
  are character-identical to an already-emitted one).
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without
  regeneration) — the corpus has no function where multiple same-named scalar highs share a
  slot, so the collapse is invisible to it.
- **Speed**: target `main` of x86_64/cvs measured at +0.14% (off 2236 ms → on 2239 ms,
  budget 5%) — an O(decls) HashSet pass, well within budget.
- **Stage-testcase**: `tests/stages/ghangr-x8664-cvs-863633.xml` decompiles cvs `main`
  from its own .text bytes (two passes: `dedupvardecls off` then `on`) and asserts the slot
  `stack - 0x58` is declared 4× off + 1× on (= 5) and `stack - 0x3c` 2× off + 1× on (= 3),
  proving both the bug and the collapse. `docs/baseline-stages.json` (+2 assertions; the
  `kuna-catalog.xml` provenance count moved 3→4 angr-derived options).
- **Date**: 2026-06-25.

---

## DIV-8: ELF relocatable objects (`ET_REL` / `.o`) load by default

- **Flip**: `relocobjects` → **on**. The faithful `LoadImageBfd` port
  (`kuna_analysis::loadimage_object`) builds its byte map only from `PT_LOAD` program headers,
  so a **relocatable object** (`ET_REL` — a `.o`, which has no program headers) mapped zero
  bytes and **every** function failed to lift (`Unable to load N bytes at ...`) — i.e. kuna
  produced *no output at all* for a `.o`. Default-on, kuna now takes a new load path for
  `ET_REL`: lay the `SHF_ALLOC` sections out above `0x40_0000` (angr's CLE default), apply the
  `.rela.*` relocations (`R_X86_64_PC32`/`PLT32`/`32`/`32S`/`64`), rebase defined symbols, and
  bind undefined externs to synthetic named call targets. The kuna analog of angr CLE's ELF
  relocatable backend. `option relocobjects off` (or env `KUNA_RELOC_OBJECTS=0`) restores the
  upstream `PT_LOAD`-only loader (a `.o` then errors again).
- **Unlike DIV-1..6 this is a loader *capability*, not a rendering change**: it produces output
  where upstream produced an *error*, and it does not alter the rendering of any file that
  already loaded. Linked `ET_EXEC`/`ET_DYN` images keep the `PT_LOAD` path **byte-for-byte**
  (the new path is guarded on `kind()==Relocatable` *and* empty `segments()`).
- **Changed upstream assertions: 0 of 675** (`make test` PARITY OK without regeneration) and
  **0 of the stage corpus** beyond the two `kuna-catalog.xml` provenance counts, which were
  bumped in place (`source_decompiler="angr"` 5→6, `change_kind="structure-recovery"` 2→3) to
  include the new settable. The XML datatest path never constructs an `ObjectLoadImage`, so the
  corpus is structurally immune to the new loader path.
- **Mechanism**: a load-time branch in `ObjectLoadImage::from_bytes` →
  `from_relocatable` → `s1_loader::elf_reloc::layout_relocatable`. Because the loader runs at
  `load file` — *upstream* of the per-function option machinery (the image is opened before any
  `option` command) — the `relocobjects` toggle is bridged to the loader by the
  `RELOC_OBJECTS_ENV` (`KUNA_RELOC_OBJECTS`) process env var the console / CLI sets, rather than
  by an `Architecture` flag that would be read too late.
- **Tests**: loader unit tests (a hand-assembled `ET_REL` exercising layout + each relocation
  kind + symbol rebasing, and the real `ptx.o` fixture asserting `fix_output_parameters`
  rebases to `0x400660` and its bytes load) in `kuna-analysis` (`make rust-test`). No XML stage
  test: the datatest path bypasses the ELF object loader entirely, so a loader capability cannot
  be expressed as an XML `decompilertest`; the cargo suite is the gate.
- **Speed**: layout + relocation patching is a one-time O(sections + relocations) pass at load,
  negligible vs decompilation; the target's measured on-decompile wall-time is within budget
  (and faster than the off/error path).
- **Date**: 2026-06-25.

---

## DIV-9: i386-PIE PLT imports are named (and `exit` flagged no-return) by default

- **Flip**: `i386_pie_plt` → **on** (loader-tier gate; `option i386_pie_plt off` restores the
  pre-fix rendering where i386-PIE GOT-relative PLT stubs stay `sub_<addr>`). Closes angr
  `test_decompiling_nl_i386_pie::usage`. Unlike DIV-4/5/6 this **is a correctness fix** (the
  i386-PIE analog of the already-unconditional x86-64/aarch64 PLT decoders), recorded here
  because it changes default output for an entire binary class (every i386 PIE ELF).
- **Problem**: `kuna-analysis::s1_loader::elf_plt::decode_i386` decoded only the non-PIC
  `FF 25 <abs32>` (`jmp *abs32`) stub form and skipped the PIE `FF A3 <disp32>`
  (`jmp *disp32(%ebx)`, GOT-relative) form. So **no** i386-PIE dynamic import was named, and
  in particular `exit@plt` was never resolved → never flagged no-return by `NoReturnKnownPass`
  → its `call exit` was assumed to fall through, manufacturing a bogus back-edge → a spurious
  `do{}while(true)` + `goto` + an unresolvable stack-pointer MULTIEQUAL cycle (`// esp`
  un-unified locals, explicit frame stores, dropped call args, a recovery-failure marker).
- **Mechanism**: `decode_i386` now also matches `FF A3 <disp32>` and computes
  `slot = GOT_base + sign(disp32)`, where `GOT_base` is the `_GLOBAL_OFFSET_TABLE_` symbol
  value (fallback `.got.plt`/`.got` section base) — the value the PIC prologue
  (`call __x86.get_pc_thunk.bx; add $_,%ebx`) loads into `%ebx`. The decoded slot is matched
  against the dynamic-reloc GOT-name map exactly as the `FF 25` arm does (an unmatched target
  is simply not recorded), so the resolver/IRELATIVE slots fall out automatically. The non-PIC
  `FF 25` arm is untouched (non-PIE i386 stays byte-identical). The whole downstream cascade
  (import naming → `exit` no-return → loop collapse → stack recovery) is pre-existing machinery
  that merely lacked the stub→name mapping.
- **Why a flag at all** (the sibling x86-64/aarch64 decoders ship unconditionally): the
  standing "output-changing ⇒ `--option`-flaggable" requirement. The gate is **default-on**
  (loader fidelity); `option i386_pie_plt off` is the escape hatch / before-image. Because the
  PLT→name map is baked at `load file` (upstream of every per-function `option`), the loader
  reads the gate through the `KUNA_I386_PIE_PLT` env var (`kuna_decomp::kuna_i386_pie_plt`),
  which the `kuna` CLI sets on the `decomp_dbg` subprocess; the `Architecture`
  `analysis_i386_pie_plt` bool exists only for catalog visibility.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without regeneration):
  the XML datatest corpus uses `<binaryimage>` bytechunks that never reach the ELF loader /
  `resolve_plt_imports`, and contains no i386-PIE binary. Speed: the collapsed spurious loop
  makes the target **faster** (`usage`: 130 ms on vs 422 ms off, −69%).
- **Testcase**: the `tests/stages/*.xml` bytechunk harness cannot carry `.rel.plt`/`.dynsym`/GOT
  structure, so this is gated by a cargo integration test instead —
  `kuna-console/tests/verify_i386_pie_plt.rs` (drives `bootstrap_from_object` over a vendored
  i386-PIE `nl` ELF → `load function usage` → `print C`, asserting the named libc calls and the
  absence of the spurious `do{}while(true)`/`goto`/`sub_<addr>`), plus the decoder unit test
  `elf_plt.rs::tests::i386_pie_plt_decode`. The catalog byte-compat fixture was regenerated to
  carry the new 41st settable.
- **Date**: 2026-06-25.

---

## DIV-10: loop-exit gotos become structured `break;`

- **Flip**: `loopbreak_recovery` → **on** (a port of Ghidra `BlockGraph::scopeBreak`). An edge
  that leaves a loop to the loop's single successor — and a switch case that exits straight to
  the switch successor — is the semantic `break;`. kuna's port had left `scopeBreak` an explicit
  SEAM stub (`docs/rust-port/losses.md`), so every such loop-exit edge rendered as a raw `goto
  <successor-label>;` plus a synthesized `label_NNNN:` on the successor. The pass retags those
  edges `f_goto_goto → f_break_goto`; the printer then emits `break;` and `markUnstructured`
  (which runs after) skips the now-dead label. `option loopbreak_recovery off` restores the
  raw-`goto` rendering. Unlike DIV-1..3 this is **not a correctness fix** — it is a deliberate
  structure-recovery / readability default (angr Phoenix/SAILR loop-successor refinement),
  recorded here because it changes default output. Note it *converges* kuna toward upstream
  Ghidra, which runs `scopeBreak` unconditionally; kuna had simply not ported it.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without regeneration;
  `make rust-test` — the cargo unit + golden differential + `.sla` suite — also green with the
  flip on). The corpus has no `<stringmatch>` asserting a loop-exit `goto`/`break`/`label`
  rendering, so the retag is invisible to it. The two `tests/stages/kuna-catalog.xml` provenance
  meta-assertions (`source_decompiler: angr`, `change_kind: structure-recovery`) were bumped in
  place for the new 24th settable.
- **Mechanism**: a per-`Architecture` flag `recover_loop_break` (default on, set in
  `resetDefaults`, copied into the per-function seam in `build_arch_handle`) read by
  `ActionFinalStructure::apply`, which calls `kuna_scope_break` (the faithful `scopeBreak`
  recursion over the structured tree) between `finalizePrinting` and `markUnstructured`. Pure
  print-prep: it only flips `gototype` flags on already-structured `BlockGoto`/`BlockIf`/
  `BlockSwitch` nodes — no CFG/SSA/type change. See `PROGRESS.md` and
  `kuna_loopbreak_recovery.rs`.
- **Speed**: median decompile of `1after909::doit` is 440.2 ms off vs 440.7 ms on (+0.12%, well
  within the 5% budget) — one extra O(blocks) tree walk at finalize time.
- **Stage-testcase**: `tests/stages/ghangr-1after909-doit-73591e.xml` (`1after909`/`doit`) runs
  pass 1 `option loopbreak_recovery off` (asserts the nine `goto label_239f;` + the `label_239f:`
  line, the bug) and pass 2 `option loopbreak_recovery on` (asserts they become `break;` and the
  label is suppressed). `docs/baseline-stages.json` (+2 assertions).
- **Date**: 2026-06-25.

---

## DIV-11: `setlocale` gets its `char *` libc prototype by default

- **Flip**: `kuna-analysis::s1_protos` (the `ApplyDataArchiveAnalyzer` analog: the built-in libc
  prototype table) now curates `char *setlocale(int category, const char *locale)`. Any object
  that imports `setlocale` gets the call typed: its result is a `char *` and a NULL `locale`
  argument renders `(char *)0x0` instead of a bare `0`. This is a **correctness fix** (the
  signature is the standard `<locale.h>` declaration), recorded here because it changes the
  default rendering of every binary that calls `setlocale` (e.g. coreutils' `tee_O2`:
  `main` goes from `setlocale(6,0x6dc1)` to `setlocale(6,"")`). There is **no option flag** —
  like the other LIBC table entries (`puts`, `strchr`, `malloc`, …) the prototype is
  unconditional; `setlocale` was simply missing from the table.
- **Problem**: with no prototype, a `setlocale(...)` call's result was an untyped `undefined8`,
  so the standard `setlocale(LC_ALL, "")` idiom lost its `char *` and the string literal, and a
  wrapper that stored/typed the result was typed against an undefined. This surfaced on angr
  `test_decompiling_tee_O2_tail_jumps` (gnulib `setlocale_null_androidfix`), whose body is
  `return setlocale(category, NULL);`.
- **Mechanism**: one entry added to the `LIBC` table in
  `kuna-analysis/src/s1_protos/mod.rs` (`ret: char *`, params `(int, const char *)`,
  non-variadic). The pass already matches table names against the object's FUNC/import symbols
  and parks each `PrototypePieces` on its callee via
  `Architecture::set_function_prototype_pieces`, which `ActionDefaultParams` reads when typing
  callers. Nothing else changed.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without regeneration):
  the XML datatest `<binaryimage>` bytechunks never reach the ELF loader / the libproto pass,
  and no datatest imports `setlocale`. `make rust-test` green (new unit test
  `s1_protos::tests::setlocale_signature_is_char_ptr_int_char_ptr` pins the entry's shape).
  Speed: typing one extra call is in the noise (no measurable change on `tee_O2`).
- **Scope note — what this does *not* fix**: when a wrapper's *whole* body is
  `return setlocale(...);` and the compiler emits a tail-position `call setlocale; ret` (or an
  `-O2` tail `jmp`) with **no intervening use of the return register**, kuna does not thread the
  callee's result into the function's own return (the wrapper stays `void` with a bare `return;`).
  That is a **general** return-value-recovery characteristic — it affects any `return libcfn(...)`
  in that shape (e.g. `return strchr(s,c);`), with or without the correct prototype — rooted in
  `AncestorRealistic` rejecting a return register defined only by the trailing call
  (`killedbycall`). It is a separate follow-up, documented in
  `docs/features/setlocale-rettype/analysis.md`; this DIV pins only the prototype/type half.
- **Testcase**: `tests/stages/ghangr-setlocale-rettype.xml` declares the same prototype the
  libproto table seeds (inline via `parse line`, since the stage harness does not run the
  analysis tier) and decompiles a `call setlocale; ret` wrapper, asserting the NULL `locale`
  argument types `(char *)0x0` and the call resolves to the named `setlocale` (the pre-fix shape
  rendered a bare `0`). `docs/baseline-stages.json` (+3 assertions, 176 → 179).
- **Date**: 2026-06-25.

---

## DIV-12: the region structurer becomes the default structuring path

- **Flip**: `regionstructure` → **on** (a port of angr's Phoenix/SAILR structurer, built
  increment by increment over Inc 0–6). kuna's S8 now structures the CFG by walking the
  `KunaRegionIdentifier` region tree and matching the Phoenix/SAILR acyclic-sequence, ITE,
  cyclic-loop (Inc 3), and acyclic switch-case (Inc 4) schemas — with Inc 5's short-circuit
  condition folding (`&&`/`||`) — instead of running Ghidra's `CollapseStructure` unconditionally.
  When a function cannot be collapsed to a single structured root (irreducible / multi-entry
  loops) the region structurer falls back to `CollapseStructure`, so it is never *worse* than
  upstream. `option regionstructure off` restores the unconditional Ghidra `CollapseStructure`
  path. Like DIV-10, this is **not a correctness fix** — it is a deliberate
  structure-recovery default (the region-driven structurer is the foundation for the
  irreducible/SAILR goto-reduction work that later increments build on); it is recorded here
  because it changes the default *engine* that produces S8 output.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without regeneration).
  The full 675-assertion datatest corpus is **byte-identical** with the region structurer on vs
  off (verified during Inc 5: the last divergence, `elseif.xml` Else-if #14, was closed by Inc
  5's `&&`/`||` condition folding, so on now reproduces Ghidra's `CollapseStructure` output
  exactly across the whole corpus). `docs/baseline.json` untouched. `make rust-test` green.
- **Mechanism**: the per-`Architecture` flag `region_structure` now defaults **on** (set `true`
  in both the `Architecture` struct-literal ctor and `resetDefaults`, copied into the
  per-function seam in `build_arch_handle`) and is read by `ActionBlockStructure`, which routes
  S8 through the region structurer (`s8_structure::region_structurer`) when set, falling back to
  `CollapseStructure` on an un-collapsible region. No CFG/SSA/type change — it swaps the S8
  block-structuring engine and is byte-output-equivalent to Ghidra on the reducible corpus.
- **Speed**: ON-vs-OFF decompile speed is unchanged on the datatest corpus (per Inc 5's
  verification); the region walk is O(regions) and runs in place of `CollapseStructure`.
- **Value beyond parity**: on the reducible corpus the output is identical to Ghidra, but the
  region structurer adds value on irreducible / SAILR cases as the later goto-reduction and
  region-refinement increments land — it is the primary structuring path those passes extend.
- **Stage-testcase**: `tests/stages/regionstructure-shortcircuit.xml` (added with Inc 5) already
  asserts the `&&`-folded form across both passes (off = `CollapseStructure`, on = region
  structurer); the existing region-structurer stage tests now exercise the default-on path.
- **Date**: 2026-06-26.

---

## DIV-13: name-matched known-no-return externs are flagged in ET_REL `.o` files by default

- **Flip**: `option noreturn_externmatch` (S2 flow-follow, ElementId 4104) now ships **default-on**.
  When set, the `FlowEnvironment::query_call_no_return` seam (`infra/decompile_drive.rs`) also
  reports a direct call no-return if the callee **name** matches the vendored ELF known-no-return
  list (`exit`/`abort`/`__stack_chk_fail`/…), in addition to the proto flag set by the
  address-keyed `noreturn_known` scan. `flow.rs` ORs this query at the artificial-halt site, so a
  name match plants the halt and flow stops at the call.
- **Problem**: in an **ET_REL `.o`**, a known-no-return libc function like `__stack_chk_fail` is an
  *undefined extern* — its `.symtab` entry has `address()==0` and there is no PLT stub — so the
  address-keyed `noreturn_known` pass (DIV-on by default) emits no `NoReturnFact` for it. The
  `relocobjects` loader (DIV-8) installs a `FunctionSymbol` named `__stack_chk_fail` at a synthetic
  target (the call **prints** the name) but no no-return flag is ever set, so flow runs off the end
  of the function into the inter-function alignment padding (`00 00 …` decoded as
  `add byte ptr [rax], al`) and the decompiler emits dozens of garbage `*v = *v + c;` lines. Surfaced
  on angr `test_decompiling_incorrect_duplication_chcon_main` (coreutils `chcon.o`, `main`): kuna
  emitted ~90 such lines after `__stack_chk_fail()`; angr emits none.
- **Mechanism**: a single gated check at the flow seam. The new module
  `kuna-decomp/src/s2_lift/kuna_noreturn_externmatch.rs` carries the name matcher; it
  `include_str!`s the **same** vendored list `noreturn_known` uses
  (`kuna-analysis/data/ElfFunctionsThatDoNotReturn`, a build-time include, not a crate dependency)
  and applies the **same** leading-`_` strip + global/`std` namespace guard. So the feature adds no
  risk class beyond the already-default-on `noreturn_known`; it merely reaches the ET_REL extern the
  address scan structurally misses. On a normal dynamically-linked ELF the proto flag is already set,
  so the OR is a no-op (byte-identical). This is the always-on companion of `noreturn_extern`
  (DIV-none, default-off opt-in; same seam, same vendored list) added in PR #90.
- **Changed upstream assertions: 0 of 675** (`make test` stays PARITY OK without regeneration): the
  XML datatest `<binaryimage>` bytechunks do not run the analysis tier nor target name-matched
  returning externs, and every real-ELF call to a listed name is already proto-marked by
  `noreturn_known`. **Speed**: a ~23% *win* on the target (`chcon.o::main` median 449.8 ms off ->
  344.3 ms on, n=5) — there is less dead padding to decompile. `make rust-test` green (3 new unit
  tests in the module pin the matcher + namespace guard).
- **Testcase**: `tests/stages/ghangr-incorrect-duplication-chcon-a0e113.xml` — a self-contained
  bytechunk (`call __stack_chk_fail; mov eax, 0xdeadbeef; ret`, with a `__stack_chk_fail` symbol; the
  ET_REL relocation cannot be applied in the bytechunk model, so the chcon.o bytes are not usable
  verbatim). Pass 1 (`option noreturn_externmatch off`) asserts the dead `0xdeadbeef` after the call
  survives (the bug); pass 2 (`on`) asserts the call is flagged no-return (artificial halt) and the
  dead code is gone. `docs/baseline-stages.json` (+3 assertions).
- **Date**: 2026-06-26.
