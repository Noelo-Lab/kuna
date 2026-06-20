# Upstream provenance and sync

> **The vendored C++ source (`decompiler/cpp/`, `decompiler/unittests/`) has been removed** —
> the decompiler and SLEIGH compiler are now fully ported to Rust (`rust/`); see
> `docs/RUST_PORT.md`. Still vendored from upstream: the SLEIGH `specs/` and the XML
> regression corpus `decompiler/datatests/`. The C++ anchors cited throughout the Rust code
> and docs refer to the upstream Ghidra commit below (recoverable from git history or an
> upstream checkout). `tools/sync_upstream.py` now applies only to `specs/` + `datatests/`.

kuna vendors Ghidra's SLEIGH processor specifications and decompiler regression corpus (the
C++ decompiler source it was ported from is recorded at the commit below).

## Source

- Repo:   https://github.com/NationalSecurityAgency/ghidra.git
- Commit: GHIDRA_REV=cef869af04c4740a71ad31a55704045b1b0d1644
  (2026-06-01, "Merge ... GP-6874_ryanmkurtz_security--SQUASHED")

## Vendored path map (upstream → kuna)

| Upstream (in the ghidra repo) | kuna |
|---|---|
| `Ghidra/Features/Decompiler/src/decompile/cpp/` | `decompiler/cpp/` |
| `Ghidra/Features/Decompiler/src/decompile/unittests/` | `decompiler/unittests/` |
| `Ghidra/Features/Decompiler/src/decompile/datatests/` | `decompiler/datatests/` |
| `Ghidra/Processors/<P>/data/languages/` | `specs/Ghidra/Processors/<P>/data/languages/` |

Intentionally **excluded**: `src/decompile/zlib/` (the default Makefile build uses the
system `<zlib.h>` + `-lz`; the bundled copy is only used when `LOCAL_ZLIB` is defined,
which kuna never does), and `src/decompile/build.gradle` (Gradle/Eclipse glue).

## Local-modifications policy

**Until 2026-06-06** files under `decompiler/` and `specs/` were byte-identical to
upstream. As of the GH-558 stage-model prototype, kuna **intentionally diverges**
(authorized decision: kuna is now its own decompiler, not a tracking mirror). New
kuna logic still prefers new files (`kuna_*.cc/.hh`, auto-linked by the upstream
Makefile's `$(wildcard *.cc)`), with minimal anchor edits in vendored files.

### Divergence (vendored files modified)

| File | What |
|---|---|
| `decompiler/cpp/op.hh` | new `addlflags` bit `canonical_lessequal` (0x1000) + accessors |
| `decompiler/cpp/funcdata_op.cc` | `replaceLessequal` records provenance on the rewritten op |
| `decompiler/cpp/coreaction.cc` | `RuleIntLessEqual` regrouped `analysis`→`canonicalcompare`; `ActionPresentCompareForm` registered after the last branch-flip pass; group lists updated |
| `decompiler/cpp/architecture.hh/.cc` | `present_lessequal` flag (default false) |
| `decompiler/cpp/options.cc` | registers kuna options (`compareform`, `arraynotation`) |
| `decompiler/cpp/printc.hh/.cc` | `option_arraynotation` (+ getter) + `&base[index]` mode in `opPtradd` |
| `decompiler/cpp/heritage.cc` | include + 2 recorder calls in `bumpDeadcodeDelay` (restart observability) |
| `decompiler/cpp/jumptable.cc` | include + 1 recorder call at the multistage-jump restart site |
| `decompiler/cpp/fspec.cc` | include + 2 recorder calls (`deindirect`/`forceSet` late-prototype restarts) |
| `decompiler/cpp/printlanguage.cc` | GH-2786 default-flip: parenthesize adjacent identical sign tokens (`docs/divergences.md` DIV-1) |
| `decompiler/cpp/ruleaction.cc` | GH-8471: include + 1-line guard in `RulePtrsubUndo::applyOp` (preserve Thumb funcptr) |
| `decompiler/cpp/coreaction.cc` | GH-6930: include + 1-line guard in `ActionConstantPtr::isPointer` (function-entry inference) |
| `specs/.../V850/data/languages/V850.cspec` | register-name case fix `ctbp`→`CTBP` (matches SLEIGH `CTBP`; the case-sensitive C++ register lookup otherwise aborts V850 at arch init) |
| `decompiler/cpp/coreaction.cc` | GH-6990: 1-line guard in `ActionReturnRecovery::buildReturnOutput` (single-register return) |
| `decompiler/cpp/coreaction.cc` | GH-1282: include + `RuleBoolSignShift` registered in oppool1 (gated by `option booleanmask`) |
| `decompiler/cpp/coreaction.cc` | GH-7190: include + `RuleOvLessSimplify` registered in oppool1 (gated by `option ovlesssimplify`) |
| `decompiler/cpp/flow.cc` | GH-8817: include + 2-line gated reclassify V850 `jmp [reg]` CALLIND→BRANCHIND in `xrefControlFlow` |
| `decompiler/cpp/coreaction.cc` | GH-8913: include + `RuleAddCarryChain` registered in oppool1 (gated by `option addcarrychain`) |
| `decompiler/cpp/coreaction.cc` | GH-9230: include + `RuleMemsetCopy` registered in cleanup pool (gated by `option memsetrecover`) |
| `decompiler/cpp/userop.hh/.cc` | GH-9230: `BUILTIN_MEMSET` (0x10000006) id + `registerBuiltin` case (`void *memset(void*,int,int)`) |
| `decompiler/cpp/constseq.hh` | GH-9230: `StringSequence` members widened private→protected (reused by `MemsetSequence`) |
| `decompiler/cpp/architecture.cc`, `decompiler/cpp/printc.cc` | DIV-2: eight kuna option defaults flipped ON (`docs/divergences.md`) |
| `decompiler/datatests/` (15 files, 22 assertions) | DIV-2: regexes re-pinned to kuna default output (compareform/arraynotation renderings); old forms in git history |
| `decompiler/cpp/testfunction.cc` | (kuna) `<cstdlib>` include + env-gated `KUNA_DUMP` block in `runTests` that echoes the captured console output (triage/repro aid; default-off ⇒ no behavior change) |
| `decompiler/unittests/testkunaregion.cc` | (kuna) **added** kuna-owned file inside the vendored unittests dir: unit tests for the angr RegionIdentifier port (`kuna_regionid.cc`; ports of angr `test_region_identifier_0/1` + a loop case). The port itself needed **zero** upstream anchor edits (console registration rides the existing `IfaceKunaCapability`). |
| `specs/.../HCS12/data/languages/HCS_HC12.sinc` | GH-9001: `:BRN rel8 ... ; rel8` now consumes its operand (was a 1-byte epsilon `SkipNextInstr` that left the rel8 byte dangling), so BRN is a correct 2-byte no-op (upstream PR #5907) |

| `specs/.../8051/data/languages/8051_main.sinc` | GH-1243: ADDC carry-in is now computed in a 2-byte `do_addc(op)` macro (`tmp:2 = zext(ACC)+zext(op)+zext(CY)`; `CY = tmp>0xff`; `ACC = tmp:1`) instead of the old 1-byte `tmp:1 = $(CY)+op` that truncated the carry-out, so CY propagates and ADD+ADDC multi-byte adds are recognised |

| `specs/.../x86/data/languages/ia.sinc` | GH-7418: 32/16-bit `rel16` branch operand now masks the target to the low 16 bits (`(inst_next + simm16) & 0xFFFF`), zeroing the high bytes of EIP per Intel SDM `tempEIP := (EIP+DEST) AND 0000FFFFH`; the high-bit-preserving 64-bit form is kept under `@ifdef IA64` |

| `specs/.../x86/data/languages/ia.sinc` | GH-7139: x86 16-bit modrm mod=2 displacement operands (`addr16`) now use the signed `simm16` token (matching the mod=1 `simm8_16` forms), so a disp16 with its sign bit set renders signed (`ES:[BX + -0x804]`) instead of unsigned (`0xf7fc`) — display-only, address arithmetic unchanged |

| `specs/.../x86/data/languages/ia.sinc` | GH-8790: added register-direct (mod=3) variants of the two prefetch/NOP opcodes — `:PREFETCH Rmr8` for `0f 0d /0` and `:NOP^"/reserved" Rmr8` for `0f 18 /0..3` — so e.g. `0f 0d c0` / `0f 18 c0` disassemble (as a NOP-like hint) instead of failing with "Unable to resolve constructor". Upstream only had the `m8` memory-operand forms. |

| `specs/.../x86/data/languages/fma.sinc` | GH-9184: every 256-bit YMM/`$(VEX_L256)` FMA constructor now uses a 256-bit temp (`local tmp:32`) before `ZmmReg1 = zext(tmp)`; upstream used `local tmp:16`, truncating the upper 128 bits of every YMM FMA result (accumulator rendered `undefined1[16]` with a per-iteration `ZEXT1632` round-trip). XMM/`$(VEX_L128)` forms correctly keep `tmp:16`. |

| `specs/.../ARM/data/languages/ARMinstructions.sinc` | GH-5897: register-to-register move constructors `:cpy^COND Rd,rm` / `:cpy^COND lr,rm` renamed to `:mov^COND ...` so the UAL mnemonic `mov` is emitted instead of the deprecated pre-UAL `cpy` (identical p-code; control-flow `cpy pc,...` forms left unchanged) |

| `specs/.../ARM/data/languages/ARMinstructions.sinc` | GH-8456: add ARMv7-A/v8 MRS (Banked register) — new `bankRegM0`/`bankRegM1`/`mrsBankIdx` token fields, banked-GPR `attach names`, `define pcodeop readBankedReg`, and two `:mrs^COND Rd,bankReg*` constructors (R=0, both M halves). `00 02 09 E1` -> `mrs r0,r9_fiq` (was undecodable) |

| `specs/.../ARM/data/languages/ARMTHUMBinstructions.sinc` | GH-8391: move the 64-bit Thumb coprocessor transfers `mcrr`/`mcrr2`/`mrrc`/`mrrc2` (op4=0xec4/0xfc4/0xec5/0xfc5, thc0811=0xc) OUT of the `@ifndef CDE` guards — they don't overlap the CDE encoding space, so they must decode under ARM:LE:32:v8-m (Cortex-M33). 32-bit `mcr`/`mcr2`/`mrc`/`mrc2` (op8=0xee/0xfe) stay guarded. `fc51 0408` -> `mrrc2 p4,#0x0,r0,r1,c8` |

| `specs/.../ARM/data/languages/ARMneon.sinc` | GH-7890: scalar VFP vcvt int<->float (`.f16/.f32/.f64` <- `.s32/.u32`) re-lifted from the `VectorSignedToFloat`/`VectorUnsignedToFloat` pseudo-ops (which read the FPSCR rounding-mode field `fpscr[22,2]`, injecting an `in_fpscr` artifact + noisy `(fpscr>>0x16)&3` and never optimizing away) to native SLEIGH `int2float` (sext/zext-to-8 idiom from ARMv8.sinc) |

| `specs/.../Atmel/data/languages/avr8.sinc` | GH-1951, GH-8844: rel12addr/rel12dst now mask the RJMP/RCALL relative target with `& 0x0FFF` on the 16-bit (PCBYTESIZE==2) AVR8 variant, so a 12-bit relative branch wraps at the 4 KiW code-space boundary (small AVRs) instead of producing an out-of-range word like 0xf807/0xf817; PCBYTESIZE==3 (xmega/atmega256) is unchanged via `@else` |

| `specs/.../V850/data/languages/{Helpers/Tokens.sinc, Helpers/Extras.sinc, Instructions/Special.sinc}` | GH-4788: add the RH850 (G3KH) Bcond disp17 conditional branch (`:b^c0003 adr17` with reg2 fixed to 0). New tokens `s0404`/`op1731` + `adr17` subtable; constructor is more specific than the overlapping `ld.hu disp16[reg1],reg2` (shared op0510=0x3F & word2.bit0=1), so `f9 07 49 fe` now decodes as `bnc +0x1963d8` instead of `ld.hu -0x1b8[r25], r0` |

| `specs/.../8085/data/languages/8085.slaspec` | GH-6389: added the nine undocumented 8085 instructions (DSUB 0x08, ARHL 0x10, RDEL 0x18, LDSI 0x38, SHLX 0xD9, LHLX 0xED, RSTV 0xCB, JNK 0xDD, JK 0xFD) plus K/V undocumented flag registers, so their valid opcode bytes decode instead of failing constructor resolution (semantics per upstream commit 1577081; JK/JNK/RSTV flag effects added against kuna K_flag/V_flag) |

| `specs/.../RISCV/data/languages/riscv.rv32f.sinc`, `riscv.table.sinc` | GH-7451: add the Zfa single-precision load-immediate `fli.s` (funct7=0x78 & op2024=0x1, disambiguated from `fmv.w.x`) — an `fliConstS` operand sub-table renders the Zfa constant table (index 8 -> 0.25) and the new `fli` pcodeop lifts the constant load; previously these bytes failed to decode (`??` undefined bytes / "Unable to resolve constructor") |

| `specs/.../PowerPC/data/languages/ppc_instructions.sinc` | GH-6904: for the E500 variant, `or` / `or.` (the `mr`/`mr.` register-move idiom) now zero-extend a 32-bit result so the upper word of the 64-bit rD is cleared (`A = zext(S:4 | B:4)` under `@ifdef E500`), removing spurious CONCAT44/ulonglong artifacts on 32-bit Book E parameters. PARTIAL fix — a complete fix needs the upper word cleared in all ~239 e500 integer constructors. |

| (kuna stage-exposure) | `architecture.hh` / `architecture.cc` (flag `fold_flag_compare` decl+init), `options.cc` (include + registerOption(OptionFlagCompare)), `kuna_console.cc` (per-option getter), `coreaction.cc` (include + register RuleBoolSignLess/RuleSborrowGe in oppool1 "analysis"), `kuna_stages.cc` (surfaceTable + settableTable rows) — all `(kuna) GH-1276/8777`, gated by default-off `fold_flag_compare`; new files `kuna_flagcompare.{hh,cc}` (ELEM_FLAGCOMPARE=4010). |

| `decompiler/cpp/jumptable.hh`, `decompiler/cpp/jumptable.cc` | GH-9191: `JumpBasic::kunaTryModuloBoundTable` (decl + def) bounds a LOAD-table jumptable by a modulo/and-mask on its index when no guard does; called from `recoverModel` (gated `option switchmodbound`, default-off) |
| `decompiler/cpp/dynamic.cc` | GH-8467: `maxduplicates` in both `DynamicHash::uniqueHash` overloads reads `8` or `16` from `dynamic_hash_maxdup_high` (option `dynamichashmax`, DIV-3 default-on) |
| `decompiler/cpp/flow.cc` | GH-6882: in `xrefControlFlow` a `kunaIsSparcStructRetTrap()`-matched BRANCHIND (post-call `unimp`) is dropped and made fall-through (option `sparcstructret`, default-off) |
| `decompiler/cpp/coreaction.{cc,hh}` | GH-8500 (`stackalias`), GH-8724 (`arraystride`, DIV-3), GH-9203 (`condexeplace`, DIV-3), GH-9218 (`inputvarnodeadjust`, DIV-3), GH-8017 (`stackprobeloop`, DIV-3), GH-1276/8777 (`flagcompare` rules in oppool1, DIV-3): include lines + minimal `(kuna)`-gated blocks at the relevant Action/decision points |
| `decompiler/cpp/architecture.{hh,cc}`, `decompiler/cpp/options.cc`, `decompiler/cpp/kuna_console.cc`, `decompiler/cpp/kuna_stages.cc` | the nine stage-exposure flags + option registrations + console getters + surface/settable rows (ElementIds 4010–4018); six flipped DIV-3 default-on (`docs/divergences.md`) |
| `decompiler/cpp/funcdata.hh`, `decompiler/cpp/jumptable.hh` | angr LoweredSwitchSimplifier port: `Funcdata::kunaInstallLoweredSwitch` and `JumpTable::kunaSetTrivialModel` declarations (defs live in `kuna_loweredswitch.cc`; member functions for private `bblocks`/`jumpvec`/`jmodel` access) |
| `decompiler/cpp/coreaction.cc` | angr port: include + 2 actions in `universalAction` — `ActionLowerSwitchInstall` before `ActionHeritage` (pre-SSA install), `ActionLowerSwitchDetect` after `ActionSwitchNorm` (detect+restart); `architecture.{hh,cc}` flag `recover_lowered_switch`, `options.cc` registerOption, `kuna_stages.cc` rows, `kuna_restartlog.{hh,cc}` `krestart_lowered_switch` — all `(kuna)`, `option loweredswitch` (ElementId 4019), DIV-4 default-on (binary-search-structure guard keeps it corpus-clean; `docs/divergences.md`) |
| `decompiler/cpp/database.cc` | DIV-5 angr naming: include + angr-mode blocks in `Scope::buildDefaultName` (aN/`dat_`/vN) and the `persist` arm of `ScopeInternal::buildVariableName` (`dat_<addr>`) |
| `decompiler/cpp/database.hh` | DIV-5 angr naming: 1-line `(kuna)` accessor `Scope::getFuncdata()` (lets the location-comment helper resolve a dynamic symbol's representative Varnode via `Funcdata::findLinkedVarnode`) |
| `decompiler/cpp/printc.cc` | DIV-5 angr naming: include + angr-mode branches in `PrintC::emitVarDeclStatement` (trailing `// loc` comment), `PrintC::emitLabel` (`label_<addr>`), `PrintC::genericFunctionName` (`sub_<addr>`), `PrintC::pushAnnotation` (volatile data `dat_<addr>`) |
| `decompiler/cpp/architecture.{hh,cc}` | DIV-5 angr naming: flag `name_style_angr` (decl + default-on init) + `Architecture::nameFunction` angr branch (`sub_<addr>`); include |
| `decompiler/cpp/fspec.cc` | DIV-5 angr naming: include + `FspecSpace::printRaw` angr branch (`sub_<addr>`) |
| `decompiler/cpp/coreaction.cc` | DIV-5 angr naming: include + `ActionNameVars::makeRec` default-name guard switched to `kunaIsGeneratedName` (recognises `param_N` and the new `aN`/`vN`) |
| `decompiler/cpp/options.cc`, `decompiler/cpp/kuna_stages.cc` | DIV-5 angr naming: `registerOption(OptionNameStyle)`; settable + surface rows for `option namestyle`; new files `kuna_naming.{hh,cc}` (ElementId 4020), `option namestyle angr` default (`docs/divergences.md`) |
| `decompiler/cpp/coreaction.cc`, `decompiler/cpp/architecture.{hh,cc}`, `decompiler/cpp/options.cc`, `decompiler/cpp/kuna_stages.cc` | angr StackCanarySimplifier port: include + `ActionStripStackGuard` in `universalAction` (before `ActionReturnSplit`); flag `strip_stack_guard` (decl + default-off init); `registerOption(OptionStackGuard)`; settable + surface rows — all `(kuna)`, `option stackguard` (ElementId 4021), default-off opt-in (ablation: 3 datatest assertions change if default-on) |

kuna-owned additions in the vendored directory: `kuna_compareform.{hh,cc}`,
`kuna_arraynotation.{hh,cc}`, `kuna_stages.{hh,cc}` (stage registry),
`kuna_console.{hh,cc}` (self-registering `IfaceKunaCapability` — console commands
added with zero upstream edits), `kuna_assert.{hh,cc}` (typed assertion API),
`kuna_restartlog.{hh,cc}` (restart observability side table),
`kuna_thumbfuncptr.{hh,cc}` (GH-8471), `kuna_inferfuncentry.{hh,cc}` (GH-6930),
`kuna_returnpair.{hh,cc}` (GH-6990), `kuna_booleanmask.{hh,cc}` (GH-1282),
`kuna_ovlesssimplify.{hh,cc}` (GH-7190), `kuna_v850indbranch.{hh,cc}` (GH-8817),
`kuna_addcarrychain.{hh,cc}` (GH-8913), `kuna_memsetsequence.{hh,cc}` (GH-9230),
`kuna_flagcompare.{hh,cc}` (GH-1276/8777), `kuna_switchmodbound.{hh,cc}` (GH-9191),
`kuna_stackprobeloop.{hh,cc}` (GH-8017), `kuna_dynamichashmax.{hh,cc}` (GH-8467),
`kuna_stackalias.{hh,cc}` (GH-8500), `kuna_arraystride.{hh,cc}` (GH-8724),
`kuna_sparcstructret.{hh,cc}` (GH-6882), `kuna_condexeplace.{hh,cc}` (GH-9203),
`kuna_inputvarnodeadjust.{hh,cc}` (GH-9218),
`kuna_loweredswitch.{hh,cc}` (angr LoweredSwitchSimplifier port, ELEM 4019),
`kuna_naming.{hh,cc}` (angr-style default naming, `option namestyle`, ELEM 4020, DIV-5),
`kuna_stackguard.{hh,cc}` (angr StackCanarySimplifier port, `option stackguard`, ELEM 4021, default-off)
(new files, not upstream edits). The `stage catalog` JSON command and the LLM
assertion catalog (`settableTable`) live in the existing `kuna_stages`/`kuna_console`
files; no new vendored edits.

`sync_upstream.py`'s clean-apply guarantee **no longer holds** for the files in the
table: a future sync touching them will need `--3way` or manual conflict resolution
(added kuna `kuna_*` files and new-file additions are unaffected). The sync remains
clean for `specs/`, `decompiler/unittests/`, `decompiler/datatests/`, and the ~110
unmodified `.cc` files.

Notes inherited from upstream:
- The bison/flex outputs (`grammar.cc`, `xml.cc`, `pcodeparse.cc`, `slghparse.cc/.hh`,
  `slghscan.cc`) are committed upstream and vendored here. The top-level Makefile's
  `touch-generated` target keeps their mtimes newer than the `.y`/`.l` sources so
  `make` never tries to regenerate them (bison/flex then aren't needed).
- `ruleparse.cc` is intentionally absent upstream (CPUI_RULECOMPILE is off by default).

## Build prerequisites

- g++ (C++11) and GNU make
- `binutils-dev` (libbfd) — required to link `decomp_dbg` / `decomp_opt` / `decomp_test_dbg`
- `zlib1g-dev` (libz)
- `bison` / `flex` — ONLY if a `.y`/`.l` grammar file is edited

## Porting upstream changes

```bash
# 1. Fetch the new upstream state in your ghidra checkout
git -C ~/github/ghidra fetch origin

# 2. Dry-run to see what would change
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master --dry-run

# 3. Apply, rebuild, re-test
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master
make test
python -m kuna.run_tests --baseline docs/baseline.json

# 4. Review `git diff`, then commit. The script updates GHIDRA_REV above on success.
```

The script diffs the ghidra repo between `GHIDRA_REV` and `--to` restricted to the
vendored upstream paths. The diff is taken with `--no-renames` (renames arrive as plain
delete+add pairs, so they can never straddle the vendored boundary) and
`core.quotepath=false`. Path prefixes are rewritten per the table above, then the patch
is applied at the kuna root: `git apply -p1 --check` first, then a plain
`git apply -p1`; if the plain apply fails (only possible if someone violated the
no-edits policy), it retries with `--3way`, which works because unmodified vendored
files have the same blob hashes as upstream. No `.rej` files are ever produced. The
rewritten patch is written to `.kuna_sync.patch` (gitignored) and removed after a
successful apply; `--dry-run` stops after the `--check` and leaves it for inspection.
`GHIDRA_REV` above is updated only on a successful apply (or an empty vendored diff)
and never during a dry run.

The script hard-fails rather than guessing on anything unusual in the patch:
quoted/unparseable diff headers, rename/copy lines, or binary file changes — handle
those manually. It also warns:

- if a `.y`/`.l` changed without its regenerated `.cc` in the same diff (you would then
  need bison/flex locally),
- if an added datatest references a processor whose module is not vendored,
- about added/deleted vendored files (a deleted `.cc` changes the upstream Makefile's
  `$(wildcard *.cc)` behavior — rebuild and re-run tests after every sync).

### Manual fallback

```bash
git -C ~/github/ghidra -c core.quotepath=false diff --no-renames <GHIDRA_REV>..<newrev> -- \
    Ghidra/Features/Decompiler/src/decompile/cpp \
    Ghidra/Features/Decompiler/src/decompile/unittests \
    Ghidra/Features/Decompiler/src/decompile/datatests > /tmp/up.patch
sed -i 's|\([ab]\)/Ghidra/Features/Decompiler/src/decompile/|\1/decompiler/|g' /tmp/up.patch
git apply -p1 --3way /tmp/up.patch          # from the kuna repo root
# (specs: same idea with Ghidra/Processors/<P>/data/languages → specs/Ghidra/Processors/<P>/data/languages)
# then update GHIDRA_REV in this file.
```
