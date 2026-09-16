# kuna history

Where kuna came from, for a reader who wants the far past. **Day-to-day work does not
need this file** — it records nothing you must do and nothing you must add to. Open it to
place an old commit, to resolve a `DIV-N` / `LOSS-NNN` / `UB-N` citation you hit
somewhere, or to sync the vendored upstream tree.

It replaces four retired doc trees — `docs/history/` (frozen design docs + campaign logs),
`docs/rust-port/` (port plan, ADRs, losses ledger, verification protocol),
`docs/divergences.md` (the DIV registry), and `docs/UPSTREAM.md` (vendored-tree
provenance) — all deleted 2026-07-28; their full text lives in git history before that
date. Code comments citing `decompiler/cpp/<file>.{cc,hh}` refer to the retired vendored
C++ tree (recoverable from the pinned upstream commit below); citations of `LOSS-NNN`,
`UB-N`, or paths under `docs/rust-port/` and `docs/history/` refer to the retired ledgers
and design docs summarized here.

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

## Divergences from upstream Ghidra (historical index)

kuna is its own decompiler, not a tracking mirror, so "does this differ from upstream
Ghidra?" stopped being a question worth answering per change. **DIV numbers are retired:
nothing new gets one, and no PR, option or default flip needs a row here.** What a change
does belongs in its option's `phases.toml` row, its `docs/spec/` chapter and its stage
test — all of which a reader reaches without this file.

What follows is a frozen index, kept only so that a `DIV-N` citation in a code comment,
a `phases.toml` row or an old PR body still resolves to something. Each row is one line;
the original entries, with their evidence and measurements, are in git history (this file
before 2026-09-16, and `docs/divergences.md` before 2026-07-28).

| DIV | Option/area | What it did |
|---|---|---|
| DIV-1 | — | parenthesization (no flag) — `-(-x)` not `--x` (re-parses as pre-decrement) |
| DIV-2 | `compareform original`, `arraynotation`, `thumbfuncptr`… | 8 issue-fix options default-on (GH-558/8471/6930/1282/7190/8913/9230/1537) |
| DIV-3 | `flagcompare`, `stackprobeloop`, `dynamichashmax`… | 6 more issue-fix options default-on |
| DIV-4 | `loweredswitch` | angr LoweredSwitchSimplifier: comparison cascades → `switch` |
| DIV-5 | `namestyle angr` | angr-style naming (`v1`/`a0`/`dat_`/`sub_`/`label_`) + storage decl comments |
| DIV-6 | `realtypes` | `xunknownN`/`undefinedN` render as real C types |
| DIV-7 | `dedupvardecls` | duplicate scalar local decls collapsed |
| DIV-8 | `relocobjects` | ELF `ET_REL` `.o` loading (relocations + extern binding); extended by DIV-70 to COFF `.obj` |
| DIV-9 | `i386_pie_plt` | i386-PIE PLT stub decode → named imports, no-return works |
| DIV-10 | `loopbreak_recovery` | loop-exit gotos become `break;` (converges toward upstream) |
| DIV-11 | — | libc proto table (no flag) — `setlocale` prototype added |
| DIV-12 | `regionstructure` | angr Phoenix/SAILR region structurer is the default structuring path |
| DIV-13 | `noreturn_externmatch` | name-matched no-return externs in `ET_REL` |
| DIV-14 | `gotoreduce`, `crossjumprevert`, `taildup`, `dedupitetail`… | 14-flag angr SAILR structuring/readability sweep default-on (3 flags remove code) |
| DIV-15 | `decompile-all` | whole-binary driver builds the Listing by default |
| DIV-16 | `noreturn_error` | `error(nonzero,…)` wrappers concluded no-return per call site |
| DIV-17 | `iteregion` | assignment diamonds render as `?:` ternaries (angr ITERegionConverter) |
| DIV-18 | `returndup` | REVERTED 2026-07-05, SUPERSEDED by DIV-54 |
| DIV-19 | `noreturn_reach` | CFG-reachability no-return discovery (+ 2026-07-08 hot/cold-split fix) |
| DIV-20 | `decompile-all` | `funcstart_patterns on` (non-x86-64) |
| DIV-21 | — | no-return name list (data) — BSD `err`/`errx` family, `quick_exit`, glibc fatal internals added |
| DIV-22 | `noreturn_disc` | Ghidra's ≥3-site-evidence discovered-no-return analyzer default-on (as in Ghidra) |
| DIV-23 | `earlyreturn` | per-edge const-guard early-return peeling |
| DIV-24 | — | DWARF global naming (bug fix) — globals mapped at real byte size so wide accesses get their name |
| DIV-25 | `switchreturn` | per-case const returns for wide switches (≤256 cases) |
| DIV-26 | — | ELF data-symbol naming (shipped flagless; DIV-76 adds the `datasyms` gate) — `.symtab`/`.dynsym` `STT_OBJECT` entries named as globals (`dat_20a098` → `optind`) |
| DIV-27 | — | own-prototype seeding (bug fix, no flag) — a function's parked (DWARF/libproto) signature is applied to its OWN decompile, not just to its callers |
| DIV-28 | — | mid-string literals (bug fix, no flag) — a constant pointing INTO a readonly char array is typed `char *` (`bindtextdomain(0x68d8,…)` → `"coreutils"`) |
| DIV-29 | — | DWARF enums (no flag) — `DW_TAG_enumeration_type` builds a real enum type (`quotearg_style(4,…)` → `…(shell_escape_always_quoting_style,…)`) |
| DIV-30 | — | uncomputed return half (no flag) — a recovered return PAIR whose half is a callee-saved restore / callee clobber is narrowed to the real half (kills `undefined16… |
| DIV-31 | — | x86 `DF` unaffected (no flag) — the ABI's direction-flag guarantee is stated where the cspec is silent, folding `(uint8)df * -2 + 1` strides to `+1` |
| DIV-32 | — | whole-binary entry dedup (bug fix, no flag) — `decompile-all`/`functions`/`decompile-project`/wasm report each entry ADDRESS once — extra names move to `aliases[]`… |
| DIV-33 | — | executable batch targets (bug fix, no flag) — unfiltered `decompile-all`/`decompile-project`/wasm runs skip callable symbols in data sections while preserving them… |
| DIV-34 | `braceformat function next` | no blank line between a function prototype and its `{` (upstream skip_line renders `)\n\n{`); `option braceformat function skip` restores |
| DIV-35 | `nullprinting on` | zero pointer constants render as `NULL` instead of `(type *)0x0` (upstream option_NULL default off); `option nullprinting off` restores |
| DIV-36 | `inplaceops on` | `out = out OP y` statements render as `out OP= y` (the upstream `emitInplaceOp` consumer of option_inplace_ops was never wired; kuna ports it), `x += -c` folds to… |
| DIV-37 | `truthycond` | boolean-context zero comparisons render truthy (`if (x != 0)` → `if (x)`, `if (p == NULL)` → `if (!p)`) in if/while/for/ternary conditions and `&&`/`\\|\\|`/`!`… |
| DIV-38 | `braceelide` | a single-statement if body drops its braces (statement indented on the next line); Copy-leaf bodies only (labels/comments/multi-statement/else arms keep braces… |
| DIV-39 | `warnstyle` | analysis warnings render as terse `// slug` end-of-line comments on the line they describe (`usage(1); // no-return`, `if (v8) { // branch-flip`, prototype-line `//… |
| DIV-40 | — | file front-ends default to `mode auto` — raw inputs `<500 KiB` select `aggressive`, `500 KiB–<2 MiB` select `reliable`, and `>=2 MiB` select `fast`; explicit… |
| DIV-41 | — | `fast` and `aggressive` presets: `fast_funcdisc on` — correct the metadata-only fast inventory with a rooted recursive Listing walk plus conservatively validated… |
| DIV-42 | — | unfiltered `fast` whole-binary watchdog defaults to 10 seconds per function — native `decompile-all`/`decompile-project` and WASM whole-binary decompile/project… |
| DIV-43 | — | `aggressive` preset drops `dwarf_lines` (bug fix, no new flag) — the `.debug_line` pass annotates rather than recovers, so blanket-enabling it under `aggressive`… |
| DIV-44 | — | Cortex-M vector-table oracle scans program-header-executable sections (bug fix, no flag) — the vector table is DATA the CPU reads, but the oracle only looked at… |
| DIV-45 | — | `kuna decompile` exits non-zero on a per-function pipeline abort (bug fix, no new flag) — three compounding defects made a failed function look like a success: the… |
| DIV-46 | — | unnamed spacebase references render as stack locations (bug fix, no new flag) — a `PTRSUB(spacebase, off)` whose offset P6 bound no Symbol to fell out of the P9… |
| DIV-47 | — | P6 Cover extension restored (bug fix, no new flag) — two dropped cover-extension steps in the merge port each emitted C that computes a different value than the… |
| DIV-48 | — | the `realtypes` relabel keeps an unknown pointee's size (bug fix, no new flag) — `realtype_unknown_base` short-circuited to `void` for EVERY size under a pointer… |
| DIV-49 | — | structured components print in final order (bug fix, no new flag) — `ActionFinalStructure` never called `BlockGraph::orderBlocks` (`block.hh:437`, the first line of… |
| DIV-50 | — | renaming reconciles a full-size input with `guardInput`'s leftover pieces (bug fix, no new flag) — gnulib's `rpl_fcntl` at `-O2` produced no body at all in 48 of… |
| DIV-51 | — | short-circuit `0`/`1` select diamonds re-roll into a boolean assignment (`iteboolean`, default-ON) — gcc at `-O0` materializes a source `x = a && b;` as a… |
| DIV-52 | — | one local declaration per ScopeLocal Symbol (bug fix, no new flag) — `PrintC::emitLocalVarDecls` walks HighVariables where upstream `emitScopeVarDecls` walks the… |
| DIV-53 | — | the browser inventory shares the decompile/project discovery policy (bug fix, no new flag) — `kuna_wasm list` — the command that fills the web UI's function sidebar… |
| DIV-54 | — | `returndup` default-ON (supersedes the DIV-18 revert) — The whole-block gotoless return duplicator (angr SAILR `ReturnDuplicatorHigh`… |
| DIV-55 | — | a datatype copy split declines a read-only output (bug fix, no new flag) — `SplitDatatype::testCopyConstraints` (`p3_dataflow/subflow.rs`, upstream… |
| DIV-56 | — | a diamond whose condition was concatenated with its predecessor still re-rolls (`itecondlist`, default-ON) — `iteregion` (DIV-17) and `iteboolean` (DIV-51) both… |
| DIV-57 | `peimportcall` | A Windows PE reaches its imports through an Import Address Table slot, which is *data*: `call dword ptr [slot]` lifts to a `CALLIND` whose target is the contents of… |
| DIV-58 | — | no `$$undef` placeholder survives the naming pass (bug fix, no new flag) — `Scope::buildUndefinedName` (`database.cc:2854`) names a Symbol created with an empty… |
| DIV-59 | — | a C++ (namespaced) callee renders its qualified name at the call site, and `--addr` names the function it addresses (bug fix, no new flag) — `kuna… |
| DIV-60 | `cppproto` | kuna's DWARF pass keyed every recovery off a subprogram DIE's own `DW_AT_name`. |
| DIV-61 | `fdeinterior` | A kuna `FunctionSymbol` is an entry address with no extent -- `Database::add_function` takes an address and `find_function` answers *is a function registered AT… |
| DIV-62 | `cppsig` | kuna applied no demangled C++ signature. |
| DIV-63 | — | full-depth DWARF type resolution (`typedepth`, new kuna option, default-ON) — kuna's DWARF type mapper truncated ordinary C declarations to `void`. |
| DIV-64 | — | `aggressive` preset carries `itaniumrtti` (new kuna option, default-OFF; the preset makes it the default rendering) — The option itself diverges from nothing -- it… |
| DIV-65 | `libcsigs` | kuna's built-in libc prototype table carried 27 entries. |
| DIV-66 | — | `decompile-all` / `decompile-project` / wasm: one shared decompile step, and `formatstring` leaves the `aggressive` preset (no new flag) — Two defects, one root.… |
| DIV-67 | `funcboundflow` | A kuna `FunctionSymbol` is an entry address with no extent, and CALL/CALLIND are fall-through, so the S2 flow follower stops only at a RETURN or a callee it can… |
| DIV-68 | — | `kuna functions` takes the whole-binary DISCOVERY policy (driver-tier bug fix, no new flag) — `kuna functions` listed 1 of the 5,797 functions `kuna decompile-all`… |
| DIV-69 | `orchain` | DIV-54 flipped `returndup` on evidence from two of the three optimisation levels. |
| DIV-70 | — | `relocobjects` extended to COFF objects (scope extension of an existing default-ON option, no new flag) — A COFF `.obj` exposed exactly one function. |
| DIV-71 | `evalcurrentproto` | An x86 Windows function that takes arguments rendered as `(void)`. |
| DIV-72 | — | selecting an EXTERNAL reports an external (driver + loader bug fix, no new flag) — Clicking `CellClass::Cell_Coord` in the browser inventory of a COFF object… |
| DIV-73 | — | demangled operator names keep their spelling (loader bug fix, no new flag) — Every bracket-spelled C++ operator demangled to a bare `operator`. |
| DIV-74 | `msvcftol` | Every MSVC x86-32 float-to-integer conversion decompiled to an argument-less `__ftol()` and took its input dataflow with it. |
| DIV-75 | `ctypes` | The emitted C is not C, and it mixes two vocabularies inside one function. kuna interns its core types under the literal names… |
| DIV-76 | `datasyms` | GH-184: kuna printed `__fprintf_chk(dat_61a0,...)` where every rival prints `stderr`. |
| DIV-77 | `loadguardrange` | GH-182: for a stack array written through a computed index, kuna printed a bounded array object and then subscripted it past its own declared bound. betaflight… |
| DIV-78 | — | ELF known-no-return name list (data; no new option -- it rides the already-default-ON `noreturn_known` + `noreturn_externmatch`) — GH-273: every clang/g++ C++… |
| DIV-79 | `relocrebase` | GH-289: `kuna functions` and `kuna decompile-all` disagreed on how many functions a `.o` has, and a `-g` object's DWARF, strings and named globals never attached at… |
| DIV-81 | `cleanupcode` | A decompiled Rust function is mostly resource-management glue that is nowhere in the source. |
| DIV-84 | `dynrelocs` | A PIE called its own functions through a GOT slot that read back as zero, so kuna printed `(*dat_e0dc8)(...)` for a callee it had already named. kuna's ELF loader… |
| DIV-85 | `retinputhalf` | Two functions differing only in whether the second returned half is `x` or `x*3+7` recovered different arities, and one lost a parameter outright. |
| DIV-86 | — | DWARF aggregate-LAYOUT import (`dwarfstructs`, new kuna option, default-ON) — kuna read a `-g` binary's structs as names only, and a name with no width is not a… |
| DIV-87 | — | DWARF variant-part (discriminated-union) import (`dwarfvariants`, new kuna option, default-ON) — A Rust tagged enum carries NO `DW_TAG_member` of its own, so… |
| DIV-76 | — | ghidra-mode `setOptions`: unknown option elements are skipped per-element with a "Warning:" line instead of failing the command — One stale option bricks the whole… |
| DIV-77 | — | ghidra-mode defaults: the CLI `aggressive` engine-tier preset is applied at registerProgram, and address-derived fallback names print the Ghidra GUI convention… |
| DIV-80 | — | The output language follows the binary: `--language auto` is the default on `kuna decompile` / `decompile-all` and in the browser, and a Rust binary renders as Rust… |
| DIV-82 | `securitycheck` | Every checked slice index, string slice and non-constant `/`/`%` in a Rust binary rendered as a branch to a `panic_bounds_check()` call, roughly doubling the CFG of… |
| DIV-83 | — | Rust-mangled symbols are demangled by `rustc_demangle`, not by the Itanium C++ demangler that also accepts them (bug fix; no option -- the routing is decided by a… |
| DIV-88 | — | C++ anonymous namespaces demangle to the `anonymous_namespace` identifier instead of being deleted (loader-tier name reduction; no flag -- a strict fix), plus… |
| DIV-89 | — | the CLI's stdout boundary is fallible: a closed downstream pipe is a normal terminal condition, and the command's exit code survives it (driver-tier bug fix; no… |
| DIV-90 | — | `kuna decompile` reports the console's own failure reason instead of a fixed string, for both the binary load and the analysis commit (driver-tier bug fix; no flag)… |
| DIV-91 | — | a symbol's declared extent is clamped into the type factory's `int4` domain BEFORE the narrowing cast, not after (loader/commit-tier bug fix; no flag) — A single… |
| DIV-92 | `noreturn_discstrict` | the discovered-no-return evidence tally counts only evidence about the PROGRAM, so a kuna decode failure can no longer forge a no-return verdict |
| DIV-93 | — | the four ARM entry-discovery options -- `cortexmvectors` (#248), `ptrentry` (#255), `tailcallentry` (#259), `poolentry` (#278) -- join `AGGRESSIVE_OVERRIDES` and… |
| DIV-94 | `symbolnamechars` | A symbol name reached emitted C verbatim, and nothing between `.strtab` and the printer validated one byte of it. |
| DIV-95 | `symbolnamebound` | a ceiling on how many `Scope`s one qualified symbol name may nest, plus per-component (1024 B) and total-scope-path (4096 B) length ceilings |
| DIV-96 | — | `msvcfpconst` on by default: an MSVC `__real@` floating-point constant COMDAT is decoded from its own mangled symbol name, and both the recovered slot and the… |
| DIV-97 | — | `framelayout` on by default: the `decompile-all --json` `variables` array reports the union of every stack-frame slot any `restructure_varnode` pass recovered, not… |
| DIV-98 | — | `voidtailreturn` in the `aggressive` preset (shipped catalog default stays OFF): a void function's trailing bare `return;` is elided, so the body ends where the C… |
| DIV-99 | `cortexmpriv` | a synthesized `<callotherfixup>` makes the vendored ARM user op `isCurrentModePrivileged` return the constant `1`, so the privilege guard the Cortex-M SLEIGH wraps… |
| DIV-100 | — | the console's filename grammar learns a quote rule -- `load file`, `openfile write`/`append` and `parse file` read a path with `CommandStream::read_filename`, which… |
| DIV-101 | `varargstackargs` | a VARIADIC call's stack tail is scored as its own `ParamListStandard::fillinMap` resource section, so the register slots the ABI structurally leaves empty between… |
| DIV-102 | `calleearity` | a call whose recovered argument list came out EMPTY is reconciled with a sibling call to the SAME callee whose list is already final |
| DIV-103 | `calleearityfwd` | a call that finalizes with an EMPTY argument list is retried once, at the end of the same `ActionActiveParam` pass, against the sibling calls to the same callee… |
| DIV-104 | `picbase` | the i386 PIC base register is interpreted and folded into the on-demand cross-reference index, so a GOT-relative address formation is a reference to what it forms |
| DIV-105 | `tiedstorekeep` | `RulePropagateCopy` refuses the one marker propagation that deletes a `local = f();` frame store |
| DIV-106 | `overlapbranch` | a conditional branch whose own target lies strictly inside its fall-through instruction truncates that fall-through instead of letting it swallow the target |
| DIV-107 | `ppclocalentry` | the Listing walk refuses a function entry at a PPC64 ELFv2 local entry point -- an address a defined `STT_FUNC` symbol declares, through the `st_other` local-entry… |
| DIV-108 | `ptrdepthcap` | the type propagation refuses to adopt a candidate that deepens an already unsatisfiable pointer equation |
| DIV-109 | `tailcallframe` | a direct `jmp` whose target is NOT a known function is recovered as a tail call when the instructions ending at it tear down EXACTLY the frame the entry block built |
| DIV-110 | `widestrings` | the string-literal analysis pass also scans at 2-byte (UTF-16LE) width and plants a typelocked `wchar2[N]` at each wide literal — Ghidra `StringsAnalyzer`'s… |
| DIV-111 | `machomain` | the Mach-O `LC_MAIN` entry routine is named `main` through the existing `entry_names` overlay and declared `int main(int argc, char argv)` |
| DIV-112 | `calleedeadarg` | at an unlocked call site, a bounded decode of the CALLEE's own body may veto a register argument the callee provably never reads |
| DIV-113 | `rodatastring` | a run of wide COPYs out of read-only memory into one stack buffer is collapsed into a single `builtin_strncpy(dest, "...", n)` |
| DIV-114 | `inputparamgap` | in the FUNCTION'S OWN input recovery, `ParamListStandard::forceInactiveChain` may no longer demote an ACTIVE register trial because a run of unused argument… |
| DIV-115 | `simdlane` | a ONE-BYTE lane read of a constant-mask `pshufb` byte shuffle resolves to the source lane the mask selects |
| DIV-116 | `retsplitglobal` | a shared RETURN block carrying more than eight observable global stores is no longer a *bare epilogue*, so neither return splitter clones it into every predecessor |
| DIV-117 | `pdatachained` | a PE `.pdata` `RUNTIME_FUNCTION` whose `UNWIND_INFO` carries `UNW_FLAG_CHAININFO` no longer seeds a function entry; and, ungated, the exception-directory record… |
| DIV-118 | `noreturnretuse` | `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, funcdata_varnode.cc:1851) answers whether a Varnode reaching `opmatch` is used ONLY there. |
| DIV-119 | `fastfailnoreturn` | a Windows `int 0x29` (`__fastfail`) ends the flow, so the call's unbalanced stack pointer never reaches a join |
| DIV-120 | — | the CLI's decompiling surfaces (`kuna decompile`, `decompile-all`, `decompile-project`, `decompile-graph`) inject `errortoomanyinstructions off` (driver-tier… |
| DIV-121 | — | under a caller-declared function extent (`--define-function START-END` / console `function bounds`), a branch target outside the extent resolves to the artificial… |
| DIV-122 | `arraycoverwidth` | an access that spans more than one element of a mapped array Symbol renders the width-carrying `name._<off>_<size>_` field instead of an element-zero subscript |
| DIV-123 | `calleearitylive` | a call site whose recovered argument list is a strict PREFIX of a sibling call's is extended to that list when the callee's own body proves the missing registers… |
| DIV-124 | `calleepreserves` | at a call whose callee a bounded body walk decodes completely, a `killedbycall` register the callee provably never writes is downgraded to `unaffected`, and the… |
| DIV-125 | `emptystrconst` | a constant pointer whose read-only target renders as a ZERO-character string literal, and whose bytes past the terminator contradict string data, prints its address… |
| DIV-126 | `cookiescramble` | an `INT_XOR` of a stack-pointer-derived Varnode does not record a local-alias escape site, for the checker the call-site input recovery builds |
| DIV-127 | — | `kuna` locates `decomp_dbg`/`slacomp` and the SLEIGH tree for an extracted release archive as well as a checkout (driver-tier bug fix; no flag) — The official… |
| DIV-128 | `int3pad` | a decoded `int3` is named with a `// int3-pad xN` warning slug, and under `halt` also ends the flow |
| DIV-129 | `pdb` | an explicit `kuna_pdb_path`, then the CodeView record's own filename resolved beside the image, then `<image stem>.pdb` beside the image |
| DIV-130 | — | `loweredswitch` scrutinee (no flag) — The synthesized `BRANCHIND` that installs a recovered lowered switch is inserted with `Funcdata::op_insert` instead of the… |
| DIV-131 | `constselectjump` | Both jump-table models kuna walks are value-range models over one switch variable, and `JumpBasic::isprune` stops the backward walk at any marker op, so a… |
| DIV-132 | `armlibcmain` | Entry-discovery oracle 4 reads the `_start`->`__libc_start_main(main, ...)` idiom on x86-64, AArch64, RISC-V and PIE ARM, but its ARM path identifies the GOT slot… |
| DIV-133 | `calleeretpreserves` | An MSVC `/GS` `main` returned the frame-cookie check instead of the zero it had just set. |
| DIV-134 | `calleearitycut` | A bytecode reader called once rendered `sub_875e0();` although the disassembly loads all four of its argument registers. |
| DIV-135 | `indirectanchor` | A Windows PE called `BCryptGetProperty` for its side effect and then branched on a local nothing in the function assigns. |
| DIV-136 | `litpoolconst` | An ARM function returned a number that was nowhere in the emitted C. A wide immediate is not in the instruction: the compiler parks it in a literal pool in `.text`… |
| DIV-137 | `calleearityscratch` | A function that decodes a string with an unrolled XOR loop rendered `sub_417f80();` although `rdi` holds the decoded buffer at the CALL. `calleearitybody`… |
| DIV-138 | `codescalar` | An anti-debug stub declared a local `void` and then did arithmetic on it. |
| DIV-139 | `declaredlibcproto` | Naming a callee told kuna what it is and kuna did nothing with the name. |
| DIV-140 | `stackarggap` | A three-argument OpenSSL call rendered with five arguments, and the two invented ones were the reader's own incoming parameter and a constant. |
| DIV-141 | `win32sigs` | A Windows resource loader printed `LoadLibraryExW()` with no arguments and 39 spare stack locals holding the values it had just dropped. |
| DIV-142 | `calleeprotostack` | A Win32 API whose arguments are staged across an intervening call rendered with only the ones pushed after it. |
| DIV-143 | `paramrefdecl` | A function whose parameter is only ever used by ADDRESS declared that parameter a second time inside its own body. |
| DIV-144 | `calltrampoline` | A protected function assigned a global that does not exist, and the byte that global's address came from is one control never executes. |
| DIV-145 | — | `--jobs N\|auto` on `decompile-all` / `decompile-project` / `decompile-graph` (new driver-tier flag, default 1 = today's in-process loop) — A whole-binary run used… |
| DIV-146 | `loopcounterstore` | `RulePropagateCopy` refuses the one marker propagation that deletes a loop counter's write-back to its frame slot |
| DIV-147 | `indexaliasguard` | restore the index-alias arm of `Heritage::guard` (`heritage.cc:1194`) that kuna shipped behind a hard-coded `highPtrPossible == false` |
| DIV-148 | `zeroidiomuse` | an `INT_XOR`/`INT_SUB` of a value with itself is not a competing use of that value in `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`) |
| DIV-149 | `calleescratchbody` | a decoded callee that clobbers only SCRATCH registers still counts as a body for `calleepreserves`'s positive-evidence gate |
| DIV-150 | `kuna disassemble`/`kuna read` | a code listing always reports, on stderr and in the JSON `notes`, the addresses its own instructions branch or call to that no row of it starts at; the new… |
| DIV-151 | `decodehalt` | a `CPUI_RETURN` planted because the DECODE failed renders as upstream `PrintC::opReturn`'s pseudo-call and carries the truncation warnings |
| DIV-152 | `x64syscall` | the x86-64 `SYSCALL` user-op carries the Linux kernel ABI's register effects instead of reading and writing nothing |
| DIV-153 | `splitstorekeep` | the pieces `Heritage::refineWrite` cuts a frame store into carry the store's own `stack_store` mark |
| DIV-154 | `entrythumbflow` | carry the Thumb decode mode a container entry proves along the flow reachable from that entry |
| DIV-155 | `pdatainterior` | reject a discovered function entry that falls strictly inside another function's `.pdata` `RUNTIME_FUNCTION` body |
| DIV-156 | `retpushedhalf` | a register the function only ever PUSHED is not a placement source for a returned register half |
| DIV-157 | `tailcallsaved` | a stack-pointer run that loads nothing back through the stack pointer is argument cleanup, not a frame teardown, so it is not evidence of a tail call |
| DIV-158 | `constspaceload` | a LOAD from the CONSTANT space is the value of its pointer, not a memory read, even when that pointer is a temporary |
| DIV-159 | `branch_remove_internal` | sever an in-edge by dropping that slot from every MULTIEQUAL in the target block, not from the leading run of them |
| DIV-160 | `declhightype` | a merged local is declared at its own HighVariable's data-type -- the one every cast in its body was checked against -- not at whichever member Varnode happened to… |
| DIV-161 | `exclusivearguse` | a LOAD/STORE through a trial Varnode on a path that cannot co-execute with the call is not a competing use of that Varnode |
| DIV-162 | `callretpair` | the multi-trial arm of `FuncCallSpecs::buildOutputFromTrials` is completed on any image, not only a detected rustc one |
| DIV-163 | `callpopret` | a direct call whose callee provably pops its pushed return address and returns through the word above it is a transfer, not a fall-through call |
| DIV-164 | — | x86-64 `RDTSC` clears the upper halves of RAX and RDX after writing EAX and EDX (strict SLEIGH correction, no option) — Intel's [Instruction Set… |
| DIV-165 | — | `linuxsyscall` recognizes Linux i386-ABI `int 0x80` compatibility calls in x86-64 images (existing option, default off) — Long mode does not turn `int 0x80` into… |
| DIV-166 | — | `--stream` on `decompile-project` (new driver-tier flag, default off = today's write-the-folder-at-the-end export) — A whole-binary export wrote nothing at all… |
| DIV-167 | — | whole-binary aggregate exit (driver-tier correction, no flag) — A non-empty selected batch that produces zero function bodies now emits complete records/artifacts… |
| DIV-168 | `entryretdispatch` | an entry-point `push <continuation>; push <target>; ret` chain is classified as calls only when bounded raw-p-code provenance proves each target and adjacent exact… |
| DIV-169 | — | `--jobs N\|auto` also sizes the discovery decode, and is accepted by `kuna functions` (driver-tier flag, default 1 = today's serial walk) — A whole-binary load spent… |
| DIV-170 | `pushimmediateret` | a RETURN proven to pop the sole current-run immediate stack store is a terminal branch, not an ordinary function return |
| DIV-171 | — | `peimportcall` extends its existing default-on import-pointer binding from PE/COFF to Mach-O typed lazy/non-lazy symbol-pointer entries — A named `objc_msgSend`… |
| DIV-172 | — | `kuna` warns when the `decomp_dbg`/`decomp_test_dbg` it runs is a different build (driver-tier, no flag) — A render could come from a different engine than the… |
| DIV-173 | `loweredswitchlabels` | a recovered lowered switch carries consistent range-opcode and extension-path label semantics across restart/install instead of guessing from high case bits |
| DIV-174 | `cancelbytearithmetic` | fold only the complete one-byte multiply/left-shift cancellation tree |
| DIV-175 | `pebnames` | An anti-debug probe of the PEB rendered as magic offsets into nothing. |
| DIV-176 | `mappedflowboundary` | retain decoded paths at matching linked ELF x86 mapping boundaries |
| DIV-177 | `endptrbound` | a proven pointer walk over a stack buffer makes the walked range one array, and a walk bound written as the frame address one past it is rebuilt on that array… |
| DIV-178 | — | a `--jobs` / `--stream` worker that dies costs only the function it was running (driver-tier, no flag) — One panicking function lost its whole chunk. |
| DIV-179 | `rexthunk` | `kuna functions ab_o2.exe` listed `sub_140001ed8`, one byte into the `JMP qword ptr [__imp_InitializeSListHead]` that ends `__scrt_initialize_type_info`, and `kuna… |
| DIV-180 | `pdbinterior` | reject a discovered function entry inside a single-function PDB procedure when a function is committed at the procedure's start and that function's own code reaches… |
| DIV-181 | `loweredswitchvalue` | a re-rolled lowered switch dispatches on the value its cascade compared, or is withdrawn |
| DIV-182 | `tiedphitrim` | trim a loop head's direct read of an aliased location out of `Merge::mergeOp`'s forced merge |
| DIV-183 | `loweredswitchexact` | a re-rolled lowered switch labels every case value, is installed only when it routes every value and keeps every value its compare blocks compute, and prints labels… |
| DIV-184 | `loweredswitchheads` | lowered-switch detection tries every head on the switch variable, not only the first in block order |

### Convergences (upstream behavior restored)

Port defects whose fix moved kuna *back onto* upstream's default. Same status as the
table above: a frozen index, not a register anything is added to.

| Area | What changed |
|---|---|
| `callsitestackargs` (P4 active-input trial scoring) | kuna dropped every stack-passed call argument at call sites whose callee prototype is unlocked — x86-64 everything past the 6th register argument, i386 every… |
| `funcptrencoding` (P3 rule pool, the ISA-mode / alignment mask on an… | On processors that steal the low bits of a function pointer the *instruction* clears them before branching, so SLEIGH lifts the clear as a real `INT_AND`… |
| `subright` (P3 cleanup pool, non-least-significant truncation) | `RuleSubRight::apply_op` (`ruleaction.cc:7271`) was a **port stub**: only the leading `doesSpecialPrinting`/`isPieceStructured` half was transcribed and the… |
| `cspecprotos` (P4 prototype-model registry, the cspec's named models) | `Architecture::build_default_proto` read exactly one cspec element, `<default_proto><prototype>`, where the C++ `parseCompilerConfig`… |
| `maxlennop` (instruction decode, the parser's 16-byte instruction window) | `ParserContext::getInstructionBytes`/`getInstructionBits` guard only the **start** byte of a read against `MAX_INSTRUCTION_LEN` (16) and then pull a… |
| `arraysubfield` (P9 emission, the partial-symbol walk's routing gate) | A partial access into a plain **array**-typed symbol never entered `PrintC::pushPartialSymbol`. |
| `jtsharepartial` (P2 jump-table recovery, the partial sub-decompilation's scope) | Recovering a BRANCHIND means cloning the function's raw p-code into a partial `Funcdata`, building its blocks and running the reduced `"jumptable"` action set… |
| `ObjectLoadImage::load_fill` (the loader's 512-byte read buffer) | Upstream stages every read through a 512-byte window and copies the answer back out of it with `memcpy(ptr,buffer,size)` (`loadimage_bfd.cc`), which for a… |
| `phiopflags` (P3 SSA construction, the op-property triple heritage caches on… | `Heritage` built the `glb->inst[opc]` triple for its own ops from a hand-written local table, and that table gave `CPUI_MULTIEQUAL` only `marker` where… |

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
