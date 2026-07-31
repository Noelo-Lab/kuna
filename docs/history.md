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
| DIV-33 | `braceformat function next` (upstream option, new default) | no blank line between a function prototype and its `{` (upstream skip_line renders `)\n\n{`); `option braceformat function skip` restores | 0/675; print-only |
| DIV-34 | `nullprinting on` (upstream option, new default) | zero pointer constants render as `NULL` instead of `(type *)0x0` (upstream option_NULL default off); `option nullprinting off` restores | 675/675 via 1 datatest opt-out (forloop_loaditer); 2 stage asserts re-worded; print-only |
| DIV-35 | `inplaceops on` (upstream option, new default + ported consumer) | `out = out OP y` statements render as `out OP= y` (the upstream `emitInplaceOp` consumer of option_inplace_ops was never wired; kuna ports it), `x += -c` folds to `x -= c`; comma contexts (for-headers) exempt; `option inplaceops off` restores | 675/675 via 9 datatest opt-outs; 1 stage assert re-worded; print-only |
| DIV-36 | `truthycond` (new kuna option, default-on) | boolean-context zero comparisons render truthy (`if (x != 0)` → `if (x)`, `if (p == NULL)` → `if (!p)`) in if/while/for/ternary conditions and `&&`/`\|\|`/`!` operands; float/enum/equate zeros and value uses keep the explicit form; `option truthycond off` restores | 675/675 via 7 datatest opt-outs; 9 stage asserts re-worded (1 opt-out: branchflip polarity test); print-only |

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
