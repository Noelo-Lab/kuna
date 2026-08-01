# kuna history

The condensed project history. This file replaces four retired doc trees —
`docs/history/` (frozen design docs + campaign logs), `docs/rust-port/` (port plan, ADRs,
losses ledger, verification protocol), `docs/divergences.md` (DIV registry), and
`docs/UPSTREAM.md` (vendored-tree provenance) — all deleted 2026-07-28; their full text
lives in git history before that date. Code comments citing `decompiler/cpp/<file>.{cc,hh}`
refer to the retired vendored C++ tree (recoverable from the pinned upstream commit below);
citations of `LOSS-NNN`, `UB-N`, or paths under `docs/rust-port/` and `docs/history/`
(in code comments or `phases.toml` prose) refer to the retired
ledgers and design docs summarized here.

## Timeline

| Date | Commit | Event |
|---|---|---|
| 2026-06-05 | `17b3c5eb` | **Started with Ghidra in C++**: vendored the Ghidra C++ decompiler (~196k LOC: `cpp/`, `unittests/`, the 83-file/675-assertion XML datatest corpus) and all 39 SLEIGH processor modules (`48169e0c`). kuna begins as the byte-identical C++ tree plus Python glue. |
| 2026-06-06 | `ed1ae4d8` | **C++ rewrite era begins** — the stage-model campaign: the P0 + S1–S9 phase model (today's P0–P9) derived from a structured Ghidra/angr/Reko study; GH-558 prototype (`64bb80aa`) is the first intentional divergence from upstream (`compareform`/`arraynotation`, the first settable options, and the `tests/stages/` family). |
| 2026-06-08 | `ca1d2a14` | C++ feature/fix wave complete: **37 public Ghidra issues fixed** (17 always-on bug-fixes, 16 of them SLEIGH spec corrections; 20 option-gated decision points) out of 58 reproduced; stage registry, `kassert`, restart log, per-stage console commands. |
| 2026-06-08 | `1b84f89e` | **First feature ported from angr**: LoweredSwitchSimplifier (`loweredswitch`, default-on next day as DIV-4). |
| 2026-06-09 | `1cf56a68` | The **autonomous angr-inspired feature pipeline** (`kuna.pipeline`) starts; its first autonomously-authored PR merges the same day (`638bb4b4`, PR #1: `stackguard`, angr's StackCanarySimplifier). angr-style naming (`namestyle`, DIV-5) and RegionIdentifier (`87016be4`) follow. |
| 2026-06-10 | `6a095f8a` | **Rust port begins**: 6-crate cargo workspace, 7 ADRs, `--engine {cpp,rust}` differential harness (`36b7862f`), 200-item checklist. |
| 2026-06-11→13 | `2942b5ec`…`f3df8f61` | Wave gates W1–W9 land in four days (foundations, SLEIGH runtime, core IR, framework, simplification rules, types/fspec, storage+structuring, print, console). M1 = 207/207 unit-test parity (`2c4a6bc4`); M2 = datatests run end-to-end under `--engine rust` (3/675). |
| 2026-06-19 | `bca4ae8d` | **Initial Rust port completed** — M3: full datatest parity **675/675** against the untouched C++ oracle, after the W10 parity grind (3 → 21 → 47 → 315 → 476 → 675). |
| 2026-06-20 | `b3838e1b` | SLEIGH compiler ported (`slacomp`): 148/148 specs compile to content-identical `.sla`; full suite green on Rust-built specs, both engines. |
| 2026-06-20 | `9346a1a5` | **C++ tree removed** — the Rust port is the engine. Python CLI becomes the single Rust `kuna` binary (`3b29635a`). |
| 2026-06-22 | `aa765712` | **Analysis-tier port begins**: Ghidra's Java analyzer suite recreated as `kuna-analysis` — 62 increments over ~5 weeks (no-return, demangling, strings, DWARF, entry discovery, ARM/Thumb painting, the Listing/xref tier, PE/Mach-O/COFF loaders, Go pclntab, FID, RTTI/Obj-C/PDB metadata), every increment holding 675/675. |
| 2026-07-04 | `6b270a3e` | decbench campaign: whole-binary benchmark loop (mine/triage/rescore) driving feature work from real-binary regressions vs Ghidra/angr. |
| 2026-07-05 | `b255b3d4` | Ghidra integration Phase 1: kuna as stock Ghidra's decompiler core (wire protocol + extension). |
| 2026-07-16→28 | `cdaa35bc`, `9d308212` | `kuna decompile-project` whole-binary export; WASM browser build; the project site + in-browser decompiler at kuna.noelo.org. |

## The Rust port: how it was verified

The port's central rule: the vendored C++ tree stayed **byte-for-byte untouched as the
oracle** the entire time — every claim of correctness was differential against an unmoved
reference, never self-graded. The Rust side reproduced the exact `decomp_dbg` /
`decomp_test_dbg` console command surface, so the existing XML datatests and harness
verified the port *unchanged* via an `--engine {cpp,rust}` switch.

**Structure.** 91 port items + 91 paired verify items + 18 infra items (182,926 LOC of
scope), each pinned to the C++ source with `git hash-object` blob shas. Items were grouped
into waves W0–W11, each wave fanned out to parallel agents in isolated git worktrees and
serially integrated under a wave gate; a gate passed only when every item's verify was
ACCEPTed. Bodies were line-faithful transliterations (same algorithms, iteration order,
comparators, registration order); only the shell (ownership, `Result`, crate layout) was
idiomatic. Determinism was enforced structurally: `BTreeMap`/`BTreeSet` only (`HashMap`
banned workspace-wide via clippy `disallowed_types`), stable sorts, mandatory wrapping
helpers, explicit widths.

**The porter/verifier split.** Every port item got an adversarial verifier that was
structurally independent: it received only the C++ source at the pinned blob sha, the Rust
diff, fresh gate output, and the checklist entry — never the porter's reasoning. Each
verdict had to check off a mandatory hunt list (signedness, integer widths/promotions,
wrapping, comparator totality, iteration-order provenance for every loop, do-while/
`lower_bound` boundaries, erase-while-iterating equivalence, exception→`Result`
partial-state parity) and write **≥3 new adversarial tests** targeting the fragile spots —
tests landed regardless of verdict. Verdicts were ACCEPT / ACCEPT-WITH-LOSSES (every
divergence cited in the losses ledger) / REJECT (required a failing test or a concrete
divergence trace against the C++ oracle; three REJECTs blocked the item for human
decision).

**How bugs were found.** Layered differential evidence, finest to coarsest:
golden vectors for opbehavior/float/comparators/XML; per-instruction SLEIGH lift-diff over
the corpus; stage-boundary snapshots B0–B5 (pipeline registration, lift, post-flow CFG,
post-heritage SSA, final IR, printed C) pinned byte-equal to C++; the 207 upstream unit
tests transcribed 1:1; and the 675 datatest assertions with **monotonicity** enforced —
a per-assertion passing-set diff at every wave meant no previously-passing assertion ever
regressed. The W10 "parity grind" root-caused each remaining divergence with dual-engine
traces; the ledger records wrong theses corrected by tracing (the dual-AddrSpaceManager
keystone, LOSS-132, made every analysis pass see zero varnodes) and one **vacuous parity
claim REJECTED** by a verifier (19/675 where all 19 were empty-negative assertions). The
SLEIGH compiler had its own oracle: `sleigh_opt` is deterministic, so 148/148 `.sla`
outputs were compared as decompressed element streams, then all specs were rebuilt with
the Rust compiler and the full suite re-run. Only after all gates held was the C++ tree
deleted.

**Cost.** The port-and-verification campaign ran roughly **two weeks of near-constant LLM
time** (2026-06-05 vendoring → 06-20 C++ removal, port proper 06-10 → 06-19) and cost
approximately **$8k** in API spend.

**Ledgers.** `losses.md` was the append-only accountability spine: ~250 numbered entries
(LOSS-001–252) recording every deliberate departure at decision time — deferrals (81),
faithful-to-C++-UB divergences (58), seams (24), dependency substitutions (9: zlib→flate2,
libbfd→`object`, bison/flex→hand parsers, std::regex→`regex`), and diagnosis journals.
Every parity-blocking entry was closed en route to 675/675; what stayed open is permanent
by design (deterministic iteration order replacing C++ heap-pointer order; debug-panics
where C++ relies on x86 shift-masking UB; content-equality rather than byte-equality for
deflate). The port also surfaced **six latent bugs in upstream Ghidra** (UB-1..UB-5):
`opcode_name[]` OOB read at `CPUI_MAX`, SIGFPE on `INT64_MIN / -1` in IntSdiv/IntSrem,
signed overflow in XML `convertCharRef`, a dangling-record read in `rangemap::erase`, a
`MemoryBank` page-copy overrun, and a sort-order violation in the pcode-snippet keyword
table making `||`/`abs` unlexable — all reproduced faithfully or fail-closed in Rust,
pinned by tests.

**Architecture decisions (ADR one-liners).** 0001: IR in slotmap generational arenas
owned by `Funcdata` (no `Rc<RefCell>`). 0002: ordered containers only, C++ comparators
transcribed field-by-field. 0003: `uintb→u64`/`intb→i64` with mandatory wrapping helpers.
0004: `Result<KunaError>` mirroring `LowlevelError`, caught at the same frames. 0005:
`universalAction` as a declarative SchedNode table, observably identical to C++ `clone()`.
0006: stage tables in `phases.toml` with build.rs codegen, byte-compatible catalog JSON.
0007: typed P0 store + per-stage artifact versioning for rerunability
(`GhidraExact` restart semantics by default).

## Divergences from upstream Ghidra (DIV registry)

A DIV entry records every **intentional change to default behavior** vs upstream (bug
fixes or evidence-backed default flips), with its measured effect on the 675 upstream
assertions. Byte-identical option-gated exposures need no DIV. Entries are append-only;
new default flips add a row here (full original entries with evidence: git history of
`docs/divergences.md`). Pre-rename entries say "stage"/S-codes for phase/P-codes.

| DIV | Option/area | What changed | Evidence/effect |
|---|---|---|---|
| DIV-1 | parenthesization (no flag) | `-(-x)` not `--x` (re-parses as pre-decrement) | 0/675 |
| DIV-2 | `compareform original`, `arraynotation`, `thumbfuncptr`, `inferfuncentry`, `booleanmask`, `addcarrychain`, `ovlesssimplify`, `memsetrecover` | 8 issue-fix options default-on (GH-558/8471/6930/1282/7190/8913/9230/1537) | 22/675 re-pinned (15 files) |
| DIV-3 | `flagcompare`, `stackprobeloop`, `dynamichashmax`, `arraystride`, `condexeplace`, `inputvarnodeadjust` | 6 more issue-fix options default-on | 0/675 |
| DIV-4 | `loweredswitch` | angr LoweredSwitchSimplifier: comparison cascades → `switch` | 0/675 (binary-search guard) |
| DIV-5 | `namestyle angr` | angr-style naming (`v1`/`a0`/`dat_`/`sub_`/`label_`) + storage decl comments | 185/675 re-pinned; `namestyle ghidra` restores |
| DIV-6 | `realtypes` | `xunknownN`/`undefinedN` render as real C types | print-only; 0/675 |
| DIV-7 | `dedupvardecls` | duplicate scalar local decls collapsed | 0/675; +0.14% speed |
| DIV-8 | `relocobjects` | ELF `ET_REL` `.o` loading (relocations + extern binding) | new capability; 0/675 |
| DIV-9 | `i386_pie_plt` | i386-PIE PLT stub decode → named imports, no-return works | 0/675; `usage` −69% time |
| DIV-10 | `loopbreak_recovery` | loop-exit gotos become `break;` (converges toward upstream) | 0/675 |
| DIV-11 | libc proto table (no flag) | `setlocale` prototype added | 0/675 |
| DIV-12 | `regionstructure` | angr Phoenix/SAILR region structurer is the default structuring path | 0/675 (CollapseStructure fallback) |
| DIV-13 | `noreturn_externmatch` | name-matched no-return externs in `ET_REL` | 0/675; ~23% faster on chcon.o |
| DIV-14 | `gotoreduce`, `crossjumprevert`, `taildup`, `dedupitetail`, `regionlooprefine`, `ifelseflatten`, `switchmultipred`, `tailcalljump`, `foldcallret`, `branchflip`, `stackguard`, `noreturn_extern`, `noreturn_propagate`, `switchsharedcase` | 14-flag angr SAILR structuring/readability sweep default-on (3 flags remove code) | 0/675 via per-test opt-outs in 12 files |
| DIV-15 | `decompile-all`: `listing on` | whole-binary driver builds the Listing by default | fixes function swallowing; 0/675 |
| DIV-16 | `noreturn_error` | `error(nonzero,…)` wrappers concluded no-return per call site | Listing-gated; 0/675 |
| DIV-17 | `iteregion` | assignment diamonds render as `?:` ternaries (angr ITERegionConverter) | print-only; 0/675; −2.6% speed |
| DIV-18 | `returndup` — **REVERTED** 2026-07-05 | early-return duplication was default-on; decbench measured −976 aggregate GED regression | opt-in runtime choice now |
| DIV-19 | `noreturn_reach` | CFG-reachability no-return discovery (+ 2026-07-08 hot/cold-split fix) | Listing-gated |
| DIV-20 | `decompile-all`: `funcstart_patterns on` (non-x86-64) | prologue-pattern function discovery on stripped ARM/AArch64/MIPS/PPC/RISC-V | betaflight 469→1470 fns; x86-64 identical |
| DIV-21 | no-return name list (data) | BSD `err`/`errx` family, `quick_exit`, glibc fatal internals added | 0/675 |
| DIV-22 | `noreturn_disc` | Ghidra's ≥3-site-evidence discovered-no-return analyzer default-on (as in Ghidra) | Listing-gated; removes code |
| DIV-23 | `earlyreturn` | per-edge const-guard early-return peeling | decbench +47 perfect / −576 GED; 1 opt-out |
| DIV-24 | DWARF global naming (bug fix) | globals mapped at real byte size so wide accesses get their name | 0/675 |
| DIV-25 | `switchreturn` | per-case const returns for wide switches (≤256 cases) | +2 perfect / −107 GED; 1 opt-out |
| DIV-26 | ELF data-symbol naming (no flag) | `.symtab`/`.dynsym` `STT_OBJECT` entries named as globals (`dat_20a098` → `optind`) | 0/675; commits last, DWARF+strings keep precedence |
| DIV-27 | own-prototype seeding (bug fix, no flag) | a function's parked (DWARF/libproto) signature is applied to its OWN decompile, not just to its callers | 0/675; also kills the bogus `undefined16` RAX:RDX return |
| DIV-28 | mid-string literals (bug fix, no flag) | a constant pointing INTO a readonly char array is typed `char *` (`bindtextdomain(0x68d8,…)` → `"coreutils"`) | 0/675; interior hits only, exact-hit path untouched |
| DIV-29 | DWARF enums (no flag) | `DW_TAG_enumeration_type` builds a real enum type (`quotearg_style(4,…)` → `…(shell_escape_always_quoting_style,…)`) | 0/675; falls back to the underlying int when anonymous/memberless |
| DIV-30 | uncomputed return half (no flag) | a recovered return PAIR whose half is a callee-saved restore / callee clobber is narrowed to the real half (kills `undefined16 main` + `v[8] = <uninit>`) | 0/675; subsumes `returnpair` on GH-6990 (3 stage assertions re-worded, 261/261) |
| DIV-31 | x86 `DF` unaffected (no flag) | the ABI's direction-flag guarantee is stated where the cspec is silent, folding `(uint8)df * -2 + 1` strides to `+1` | 0/675; x86-only, spec-silent models only |
| DIV-32 | whole-binary entry dedup (bug fix, no flag) | `decompile-all`/`functions`/`decompile-project`/wasm report each entry ADDRESS once — extra names move to `aliases[]`, and ARM/Thumb `entry\|1` addresses fold onto the real entry (symbol seeds, the enumeration key, and `--addr`) | 0/675 (analysis tier is parity-isolated); `arm_thumb_linked_le32` 6→2 entries, `fmt_arm` 32→14, x86-64 unchanged; 713/713 surviving functions byte-identical |
| DIV-33 | executable batch targets (bug fix, no flag) | unfiltered `decompile-all`/`decompile-project`/wasm runs skip callable symbols in data sections while preserving them for naming, inventory, and explicit address selection | 0/675 (analysis tier is parity-isolated); private PE `bc4c15d8…3ae1b` 693→351 targets and 63.58→7.78 s; all 351 executable artifacts byte-identical |
| DIV-34 | `braceformat function next` (upstream option, new default) | no blank line between a function prototype and its `{` (upstream skip_line renders `)\n\n{`); `option braceformat function skip` restores | 0/675; print-only |
| DIV-35 | `nullprinting on` (upstream option, new default) | zero pointer constants render as `NULL` instead of `(type *)0x0` (upstream option_NULL default off); `option nullprinting off` restores | 675/675 via 1 datatest opt-out (forloop_loaditer); 2 stage asserts re-worded; print-only |
| DIV-36 | `inplaceops on` (upstream option, new default + ported consumer) | `out = out OP y` statements render as `out OP= y` (the upstream `emitInplaceOp` consumer of option_inplace_ops was never wired; kuna ports it), `x += -c` folds to `x -= c`; comma contexts (for-headers) exempt; `option inplaceops off` restores | 675/675 via 9 datatest opt-outs; 1 stage assert re-worded; print-only |
| DIV-37 | `truthycond` (new kuna option, default-on) | boolean-context zero comparisons render truthy (`if (x != 0)` → `if (x)`, `if (p == NULL)` → `if (!p)`) in if/while/for/ternary conditions and `&&`/`\|\|`/`!` operands; float/enum/equate zeros and value uses keep the explicit form; `option truthycond off` restores | 675/675 via 7 datatest opt-outs; 9 stage asserts re-worded (1 opt-out: branchflip polarity test); print-only |
| DIV-38 | `braceelide` (new kuna option, default-on) | a single-statement if body drops its braces (statement indented on the next line); Copy-leaf bodies only (labels/comments/multi-statement/else arms keep braces, dangling-else impossible); `option braceelide off` restores | 675/675 via 4 datatest opt-outs (copytrim, ifswitch + boolless/condconst oracle pins); 1 stage assert re-worded; print-only |
| DIV-39 | `warnstyle` (new kuna option, default `inline`) | analysis warnings render as terse `// slug` end-of-line comments on the line they describe (`usage(1); // no-return`, `if (v8) { // branch-flip`, prototype-line `// early-return x3`) instead of full `/* WARNING: ... */` banner lines; unknown texts keep full info behind `warn:`; `option warnstyle banner` restores | 675/675 via 3 datatest opt-outs (inline, inlinetarget, switchreturn); 14 stage asserts re-worded to slug forms; print-only |
| DIV-40 | file front-ends default to `mode auto` | raw inputs `<500 KiB` select `aggressive`, `500 KiB–<2 MiB` select `reliable`, and `>=2 MiB` select `fast`; explicit `--mode reliable\|aggressive\|fast` restores a fixed policy | 0/675 (datatest console has no file-policy default); private PE `bc4c15d8…3ae1b` resolves to `fast`: 3.23 s median / 351 of 351 functions versus default-analysis control incomplete after 618.60 s; web and native share exact boundary tests |
| DIV-41 | `fast` and `aggressive` presets: `fast_funcdisc on` | correct the metadata-only fast inventory with a rooted recursive Listing walk plus conservatively validated absolute pointer-table roots (4096-candidate ceiling on non-ARM); full prologue patterns, AIF gap scanning, and ordinary Listing consumers remain off in `fast`; `aggressive` includes the option through its all-safe-off-options contract; address-selected commands retain their exact scope and skip the preset-provided walk | 0/675 (analysis tier parity-isolated); private PE `bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b` previously exported 350 import veneers plus only its entry and omitted real `0x402d80` code; corrected `--max-fn-seconds 10` export: 462.28 s / 1,475 MiB peak RSS / 3,140 bodies + 13 isolated failures (3,153 definitions); public `pdb_prog.exe` recovers direct callee `0x140001000`, and `aif_gap_x86_64` recovers indirect-only target `0x13ae`, both with nonempty C bodies |
| DIV-42 | unfiltered `fast` whole-binary watchdog defaults to 10 seconds per function | native `decompile-all`/`decompile-project` and WASM whole-binary decompile/project isolate probed pathological functions after 10 s instead of inheriting the native 120 s default; native selectors retain 120 s and explicit `--max-fn-seconds` wins, including `0` | 0/675 (driver policy only); private PE `bc4c15d8…3ae1b` no-flag confirmation: 445.06 s / 1,511,288 KiB peak RSS / 3,140 bodies + 13 isolated failures (3,153 definitions); public native/WASM policy matrices plus Node-WASI and browser-shim fast-project coverage |
| DIV-43 | `aggressive` preset drops `dwarf_lines` (bug fix, no new flag) | the `.debug_line` pass annotates rather than recovers, so blanket-enabling it under `aggressive` — which `auto` selects below 500 KiB (DIV-40) — made `/* src.c:NNN */` per-instruction comments the DEFAULT rendering of every small `-g` binary; it joins `v850indirectbranch` as a manual `--option dwarf_lines on` opt-in, which still wins over the preset | 0/675 (mode presets are not a datatest surface); coreutils `fmt` (219 KiB, resolves to `aggressive`) `main` 321 → 179 lines with 140 source-line comments gone, `decompile-all` 7,106 → 5,299 lines with 1,740 gone; non-comment output identical apart from the `braceelide` single-statement bodies the comments had been suppressing; runtime unchanged within noise (4.36 s vs 4.80 s median of 3) |
| DIV-44 | Cortex-M vector-table oracle scans program-header-executable sections (bug fix, no flag) | the vector table is DATA the CPU reads, but the oracle only looked at `SHF_EXECINSTR` sections, so a firmware whose `.isr_vector` is flagged `WA` inside an `RWE` `PT_LOAD` — every FreeRTOS demo image — never matched, never region-painted `TMode=1`, and lost its whole Thumb call tree; the candidate set now also covers allocated sections contained in a `PF_X` `PT_LOAD` (`phdr_executable_sections`), with the acceptance test (SRAM initial SP, `word[1] == e_entry`, odd Thumb handlers landing in executable memory) unchanged | 0/675; decbench freertos `RTOSDemo.out` 8 → 146 (O0), 8 → 118 (O2), 9 → 147 (O2-noinline) functions, byte-identical to the control experiment that hand-flipped `.isr_vector` to `AX`; unchanged on coreutils `ls` (617), betaflight STM32F405 (5,798 `aggressive`), crazyflie (2,790), cleanflight (2,840), and on relocatable objects (no program headers) |
| DIV-45 | `kuna decompile` exits non-zero on a per-function pipeline abort (bug fix, no new flag) | three compounding defects made a failed function look like a success: the caught panic payload was passed as `&Box<dyn Any>` (so both downcasts failed and every `LOSS-131` record read "panic with non-string payload"), the CLI surfaced the console's stderr only when the emitted C was *empty* (an abort still renders the un-decompiled shell, so it never was), and the shell's comment blamed structuring. Now the payload round-trips, the emitted comment names the real reason (`/* WARNING: decompilation failed: … */`; the generic "structuring declined" text stays for a genuine no-`sblocks` render), and `kuna decompile` prints `error: decompilation failed for <fn> in <binary>: <reason>` plus the forwarded `decomp_dbg` stderr and exits 1 — still printing the recovered shell on stdout. `decompile-all`/`decompile-project`/wasm are unchanged (their per-function `error` field already isolated it, and now carries the real text) | 0/675 and stages PARITY OK (no datatest function aborts, so the console arm is inert); coreutils `comm` `sub_3994` @ 0x3994 before: exit 0, empty stderr, `/* structuring declined at a stub */`; after: exit 1, the real reason ("called Option::unwrap() on a None value") on stderr with the `jumptable.rs:1811` panic site forwarded |

| DIV-46 | unnamed spacebase references render as stack locations (bug fix, no new flag) | a `PTRSUB(spacebase, off)` whose offset P6 bound no Symbol to fell out of the P9 SPACEBASE arm into the *functional* render, emitting the internal p-code operator and a raw machine register into the C body (`*(void *)&PTRSUB(ESP,8)[v2] = 0;` — neither `PTRSUB` nor `ESP` is declared anywhere, so the function is not C); it now takes the upstream `PrintC::pushUnnamedLocation` leaf the arm had been declining, naming the storage in kuna's existing unnamed-location convention (`&Stack00000008`, the analogue of Ghidra's `&stack0x00000008`, undeclared extern-like by design). The symbol-bound path is untouched | 0/675 (no datatest reaches a symbol-less spacebase PTRSUB) and 305/305 stages; 217 of 13,759 functions across 12 decbench binaries emitted `PTRSUB(` before, 0 after (mydoom 4/161, betaflight 109/6,388, cleanflight 72/3,118, mirai 12/181, the rest 1–4 each); interleaved medians `decompile-all --mode reliable`: mydoom 1,553 → 1,563 ms (+0.66%), gzip 2,939 → 2,952 ms (+0.44%), cleanflight 24,453 → 24,339 ms (-0.47%); `tests/stages/ghdec-spacebase-unnamed.xml` (0/6 pre-fix, 6/6 post-fix) |
| DIV-47 | P6 Cover extension restored (bug fix, no new flag) | two dropped cover-extension steps in the merge port each emitted C that computes a **different value** than the binary. (a) `Funcdata::build_copy_pair_range` omitted `Cover::addRefPoint` (`merge.cc:1121`), so `Merge::checkCopyPair`'s dominance range was a point, no intervening write was ever found inside it, and `markRedundantCopies` silenced load-bearing restores — a `-O0` guard-clause function returned a failed call's NULL instead of its parameter, and `for` loops lost their initializer. (b) `ActionMarkImplied` dropped `Merge::markImplied`'s operand cover-dirtying (`merge.cc:1595-1605`) and a Varnode `coverdirty` never reached its HighVariable (`Varnode::setFlags`, `varnode.cc:377-378`), so Covers were never regrown through implied consumers and the speculative merges folded two values live at the same printed expression into one variable | 675/675 PARITY OK and 305/305 stages with no re-pin or opt-out; both directions are upstream-faithful restorations, not new policy; `verify_cover_miscompile` (fixture `covercopy_x86_64`) pins the value-carrying statement for each; `verify_declared_vars`' sanity floor drops 5→4 because (b) lets the size phi coalesce with `strlen` as upstream does |
| DIV-49 | structured components print in final order (bug fix, no new flag) | `ActionFinalStructure` never called `BlockGraph::orderBlocks` (`block.hh:437`, the first line of `blockaction.cc:2192`), so whenever structuring left more than one top-level component the printer walked the root list in **collapse order** — whichever component the collapse happened to append first. The function then opened in the middle of its body and reached its real entry only after an unconditional `goto`, i.e. emitted the entry as unreachable dead code (`libopencm3` `sdram` `main` opened on the `'p'`/`'P'` command handler and printed `clock_setup(); console_setup(); sdram_init();` after `goto label_8000792;`). The producer of the ordering key was already ported (`BlockGraph::add_block` keeps a composite `index` at the minimum over its members); only the sort was missing. It now runs, keyed on `FlowBlock::compareFinalOrder` (`block.cc:709`): entry component (`index == 0`) first, RETURN-terminated components last, everything else by ascending index, root graph only (upstream does not recurse), stable so the RETURN-terminated components upstream's comparator reports as equal keep a deterministic order. Applies to both structuring engines — `CollapseStructure` and the default-on region structurer hand `ActionFinalStructure` the same root list | 675/675 PARITY OK and 324/324 stages with no re-pin and no per-test opt-out (the stages delta is purely the 6 new keys); across 12 decbench binaries / 3,434 functions the "unconditional `goto` followed by a statement" signature goes 25 -> 0 and "body opens with a label" 29 -> 3 (the 3 residual labels sit at the entry block's own address, a legitimate back-edge target); blast radius 30 of 3,434 functions (0.87%) change text at all, and the whole changed set keeps its goto count and line count exactly (1,172 gotos / 16,123 lines before and after) — the emitted statements are a permutation; interleaved medians, `decompile-all --mode reliable`: O0 diffutils `diff` (526 functions) 7.38 s -> 7.07 s, O2 `bzip2` (113 functions) 15.06 s -> 15.18 s (+0.8%); `tests/stages/ghdec-finalorder-entryfirst.xml` (4 of 6 assertions fail pre-fix, 6/6 post-fix) |


| DIV-48 | the `realtypes` relabel keeps an unknown pointee's size (bug fix, no new flag) | `realtype_unknown_base` short-circuited to `void` for EVERY size under a pointer, so `undefined1 *`, `undefined4 *` and `undefined8 *` all rendered `void *` — but only the *declaration* was relabelled: `opLoad`/`opPtradd` still build index and cast expressions from the ORIGINAL pointee size, so the declaration contradicted its own body (`void *a3` alongside `a3[1]`, which means byte offset 1 under GCC's void-arithmetic extension where the expression was built for offset 8) and a store cast down to `*(void *)` lost its access width entirely. The same size table a scalar uses now applies to the pointee (`undefined8 *` → `unsigned long *`, `undefined4 *` → `unsigned int *`), and `void` survives only as the **fallback** for residual sizes with no natural single C type (0, 3, 5, 6, 7, 10, …). P5 never mis-typed anything — `option realtypes off` already produced the stride-consistent `xunknown8 *a3` — and a genuine `TYPE_VOID` pointee is not a residual unknown, so `free`/`malloc`/`memcpy`/`realloc` keep their opaque `void *` | 0/675 and 321/321 stages; libacl `set_acl_fd` (the case that surfaced it) before: `void *a3` with `(*(code *)a3[1])(a3,a0)` — `gcc -std=gnu11 -fsyntax-only` gives 12 "invalid use of void expression" errors + 12 "dereferencing 'void *' pointer" warnings; after: `unsigned long *a3`, same `a3[1]`, 0 errors 0 warnings, matching the source (`struct error_context` is three function pointers at +0/+8/+16). Across 5 whole-binary `decompile-all --mode reliable` runs (libacl, gzip, mydoom, mirai, coreutils `fmt`): `*(void *)` cast sites 737 → 1, `void *` occurrences 1,585 → 198, emitted line counts byte-for-byte unchanged; the single residual is a 10-byte x87-`long double` pointee, where `void` is the designed fallback. Least-contended-run timings (`decompile-all --mode reliable`, 7 interleaved pairs on a loaded box): gzip 2,924 → 2,939 ms (+0.51%), libacl 792 → 793 ms (+0.13%), mirai 4,651 → 4,582 ms (-1.48%); `tests/stages/ghdec-realtypes-pointee.xml` (3/7 pre-fix, 7/7 post-fix) |

## Upstream provenance & sync

- Upstream: https://github.com/NationalSecurityAgency/ghidra.git — pinned port anchor:
  GHIDRA_REV=cef869af04c4740a71ad31a55704045b1b0d1644 (2026-06-01). Updated automatically
  by `tools/sync_upstream.py` on a successful sync; keep the `GHIDRA_REV=<hex>` token on
  one line — the script greps for it.
- Still vendored (the only paths the sync script maps):
  `Ghidra/Features/Decompiler/src/decompile/datatests/` → `tests/datatests/` and
  `Ghidra/Processors/<P>/data/languages/` → `specs/Ghidra/Processors/<P>/data/languages/`.
  The C++ source (`decompiler/cpp/`, `unittests/`) was removed after the port was proven.
- Sync: `python3 tools/sync_upstream.py --ghidra <checkout> --to origin/master [--dry-run]`,
  then `make test` (675/675 PARITY OK), review, commit. Plain `git apply` first, `--3way`
  fallback; hard-fails on renames/binary diffs (handle manually). The DIV-2 re-pinned
  datatest files (15 files / 22 assertions) no longer clean-apply — expect `--3way` there.
  If a sync intentionally changes the corpus, re-pin with
  `kuna test --datatests --save-baseline docs/baseline.json` and say so in the commit;
  never re-pin to absorb an unexplained regression.
- Local-modification policy: byte-identical to upstream until 2026-06-06 (GH-558); since
  then kuna intentionally diverges — it is its own decompiler, not a tracking mirror. New
  logic goes in `kuna_*` modules; edits to ported-core files stay minimal and carry a
  `// (kuna)` marker. The per-file divergence table for the retired C++ tree (71 rows) is
  in git history of `docs/UPSTREAM.md`.

## Retired campaign notes (what the deleted docs recorded)

- **Stage-model study** (`docs/history/stage-model.md` + `evidence/`): the P0+S1–S9 model
  was synthesized from 10 structured readers over Ghidra/angr/Reko (~168 pipeline steps,
  ~80 feedback loops, ~98 decision points), 3 competing taxonomies, and 2 adversarial
  critiques; it killed 4 "phantom stages" of the older 19-stage guess. Empirical critique
  over the first 10 issue fixes: symptom→sub-stage navigation 7/10 clean; ablations sized
  each option's blast radius and justified the DIV-2/DIV-3 default flips.
- **Issue campaign** (`fixed-public-bugs.md`, `issue-coverage.md`): 58 public Ghidra
  issues reproduced, 37 fixed (17 bug-fixes / 20 decision points); 5 options deliberately
  left opt-in as destructive-by-default (`returnpair`, `v850indirectbranch`,
  `switchmodbound`, `stackalias`, `sparcstructret`).
- **Analysis-tier port** (`analysis-port-*.md`): plan generated 2026-06-22 by a 13-agent
  workflow, executed as 62 logged increments; Ghidra ships 142 analyzers, ~57
  decompiler-relevant — every feasible one ported, plus the Listing tier, multi-format
  loaders (6 ELF chokepoints → `ObjectFormat` trait), kuna-native FID (`.fid`, not
  Ghidra's non-redistributable `.fidb`), and RTTI/Obj-C/PDB metadata (Swift deferred:
  no Rust demangler existed).
- **Region structurer** (`region-structurer-roadmap.md`, `regions.md`): the angr
  Phoenix/SAILR structurer roadmap that became DIV-12/DIV-14; deep goto-win schemas
  remain deferred.
