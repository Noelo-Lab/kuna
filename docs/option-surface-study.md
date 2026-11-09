# The option surface study

How kuna's 395 runtime options were classified, what changes, and why. The rules this
study produced are in `docs/agents.md` (*Adding features*) and
`docs/improvement-pipeline.md` (*Standing requirements*); this file is the record of the
decisions behind them, one row per option.

## The three questions

1. **Which options do not need a flag?** A strict correctness fix, whose `off` value only
   reproduces output that is wrong about the program, needs no flag: it is always on. A
   feature that only applies to one architecture, OS, format or source language is enabled
   automatically on that target and its row says so.
2. **Which default-off options should be on?** Anything the `auto` mode already turns on for
   every binary under 500 KiB, or that measured net-positive with no speed cost, is a
   catalog default, not a preset override.
3. **What can be merged?** Refinements of one feature become part of that feature; families
   of flags over one mechanism become one option with values.

## What the catalog looked like

| | count |
|---|---:|
| options in `phases.toml` | 395 |
| default `on` | 264 |
| default `off` | 87 |
| tagged `correctness-fix` | 167 |
| default `off` but turned on by the `aggressive` preset (so on for every binary under 500 KiB) | 46 |

Four habits produced the surface:

- **Every PR had to add exactly one option**, including PRs that were strict bug fixes. 167
  rows are tagged `correctness-fix`; nearly all of them describe an `off` value that prints
  something false about the program.
- **A refinement of a feature became a new flag** instead of extending the feature's option
  (`calleearity` grew five siblings, `loweredswitch` four, `aif` five, `noreturn_*` seven).
- **Presentation defaults stayed `off` in the catalog because the vendored datatest corpus
  pins Ghidra's spelling**, while the `auto` mode (what every file front-end runs) turned them
  on anyway. The catalog default was therefore not the product default, which misleads an
  agent reading `kuna catalog`.
- **Target-gated features stayed `off` pending evidence** even where the code is inert off
  its target, so a PE needed `--option rtti on` and an ARM image `--option poolentry on`.

## The decisions

| bucket | options | meaning |
|---|---:|---|
| REMOVE | 164 | the flag goes; the default behaviour is unconditional |
| FOLD | 23 | the flag goes; the behaviour is part of its parent option |
| AUTO | 21 | target-gated; default on (or `auto`), the row names the target |
| FLIP | 8 | default off -> on, with the datatest corpus pinned per test |
| MERGE | 25 | fold into one multi-valued option (follow-up) |
| KEEP | 154 | a real judgement, destructive, speed trade, heuristic or policy |

After REMOVE and FOLD the catalog has **208** options; after the merges about 183.

### REMOVE: fixes that are always on

The test of this bucket: the `off` value reproduces a decode, a prototype, a type or a
statement that is false about the program, and nothing else. None is destructive, none
trades speed, and the stage test that proved each one keeps its fixed-output assertions.

| option | phase | default | reason |
|---|---|---|---|
| `arm_markers` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armcondskip` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armderef` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armfuncmode` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armlibcmain` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armpltslot` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `armwalkmode` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `authptrcall` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `callfixup` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `coldentry` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `cppproto` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `datasyms` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `declaredlibcproto` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `dynrelocs` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `elfmain` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `entrymainproto` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `entrythumbflow` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `fdeinterior` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `gopclntab` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `goreg` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `gotypes` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `gowalkprune` | P1 | on | speed-only change with byte-identical output; the off path is dead weight |
| `i386_pie_plt` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `libcsigs` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `libproto` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `litpoolconst` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `machomain` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `mips_gp` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `mips_isa` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `msvcfpconst` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `msvcsig` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `mzsweep` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `pdatachained` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `pdatainterior` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `pdbinterior` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `peimportcall` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `pemain` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `peordinal` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `picbase` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `ppclocalentry` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `rawdiscover` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `relocobjects` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `relocrebase` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `rexthunk` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `swiftdemangle` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `switchreach` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `symbolnamebound` | P1 | 256 | security/robustness cap with no legitimate alternative value; hard-code the limit |
| `symbolnamerepair` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `thumbswitch` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `thunkentry` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `typedepth` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `unmappedentry` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `win32sigs` | P1 | on | ground-truth loader fact (symbols, relocations, headers, DWARF, PDB, pclntab); off only mis-names or mis-maps |
| `callpopret` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `calltrampoline` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `decodehalt` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `entryretdispatch` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `fastfailnoreturn` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `mappedflowboundary` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `mipsunaligned` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `msvcftol` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `pushimmediateret` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `stackprobeloop` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `stubtailcall` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `switchrunon` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `switchrunonguard` | P2 | on | flow-recovery fix: off reproduces a decode that is provably wrong |
| `booleanmask` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `calleepreserves` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `calleeretpreserves` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `calleescratchbody` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `cancelbytearithmetic` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `condexeplace` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `constspaceload` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `copywaves` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `deadwaves` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `earlyrestart` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `flagcompare` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `globalcallranges` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `globalcallstore` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `globalfieldstore` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `haltloopguard` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `indirectanchor` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `intfloatround` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `loadguardrange` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `loadguardreads` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `loopcounterstore` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `mulblob` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `multiregargs` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `ovlesssimplify` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `pairrefine` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `prunedssa` | P3 | on | speed-only change with byte-identical output; the off path is dead weight |
| `segmentize` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `simdlane` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `splitstorekeep` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `stackstoreguard` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `tiedstorekeep` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `truncptrflow` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `wideslice` | P3 | on | dataflow/heritage fix: off reproduces wrong SSA or a deleted statement |
| `calleearity` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleearitybody` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleearitycut` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleearityfwd` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleearitylive` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleearityscratch` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleedeadarg` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleeprotostack` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `calleereadarg` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `callerreads` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `callpush` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `callretpair` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `callsitestackargs` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `condexeret` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `condexeretuse` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `evalcurrentproto` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `exclusivearguse` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `exclusivecallret` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `gotimportproto` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `hiddenretarg` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `inputparamgap` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `msvcbyval` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `noreturnretuse` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `pltlocal` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `pushpadread` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `reloadarg` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `retinputhalf` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `retpushedhalf` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `retsysreg` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `stackarggap` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `varargforward` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `varargstackargs` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `zerocallregs` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `zerofillreturn` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `zeroidiomuse` | P4 | on | argument/return recovery fix: off reproduces a dropped or invented argument |
| `addcarrychain` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `aggregatefields` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `codescalar` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `floatbits` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `inferfuncentry` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `thumbfuncptr` | P5 | on | type/variable fix: off reproduces an impossible type or a split value |
| `bytehonest` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `calleepop` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `calleepopframe` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `calleepopslot` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `cookiescramble` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `dynamichashmax` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `fputruncviews` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `framelayout` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `guardedreadtrim` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `hideshadow` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `inputvarnodeadjust` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `localstackviews` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `protoranges` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `reglocals` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `slotptr` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `slotreport` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `slotsplit` | P6 | on | JSON `variables` report accuracy; one report shape, not six toggles |
| `tiedphitrim` | P6 | on | type/variable fix: off reproduces an impossible type or a split value |
| `clonephiorder` | P8 | on | printer fix: off prints a statement that is false about the program |
| `arraycoverwidth` | P9 | on | printer fix: off prints a statement that is false about the program |
| `declhightype` | P9 | on | printer fix: off prints a statement that is false about the program |
| `emptystrconst` | P9 | on | printer fix: off prints a statement that is false about the program |
| `indirectargs` | P9 | on | printer fix: off prints a statement that is false about the program |
| `paramrefdecl` | P9 | on | printer fix: off prints a statement that is false about the program |
| `partialconcat` | P9 | on | printer fix: off prints a statement that is false about the program |

### FOLD: refinements that belong to their parent

| option | phase | default | reason |
|---|---|---|---|
| `aifbracket` | P1 | on | refines the AIF gap walk; meaningless without it (-> `aif`) |
| `aifnoppad` | P1 | on | refines the AIF gap walk; meaningless without it (-> `aif`) |
| `aifpoolslide` | P1 | on | refines the AIF gap walk; meaningless without it (-> `aif`) |
| `dwarfstructs` | P1 | on | part of DWARF import (-> `dwarf`) |
| `dwarfvariants` | P1 | on | part of DWARF import (-> `dwarf`) |
| `gonoreturn` | P1 | on | known-function no-return rules (extern match, glibc error(), Go list) (-> `noreturn_known`) |
| `gopairreturns` | P1 | off | two-result case of Go DWARF signatures (-> `godwarfsig`) |
| `noreturn_discstrict` | P1 | on | narrows the discovered-no-return tally (-> `noreturn_disc`) |
| `noreturn_error` | P1 | on | known-function no-return rules (extern match, glibc error(), Go list) (-> `noreturn_known`) |
| `objcrefconst` | P1 | on | part of Objective-C recovery (-> `objc`) |
| `variantnames` | P1 | on | part of DWARF import (-> `dwarf`) |
| `gojumpchain` | P2 | on | normalizes the barrier diamond gowritebarrier folds (-> `gowritebarrier`) |
| `gottailcall` | P2 | on | the GOT/one-row-table case of a tail jump (-> `tailcalljump`) |
| `loweredswitchexact` | P2 | on | correctness guards of the lowered-switch re-roll (-> `loweredswitch`) |
| `loweredswitchheads` | P2 | on | correctness guards of the lowered-switch re-roll (-> `loweredswitch`) |
| `loweredswitchlabels` | P2 | on | correctness guards of the lowered-switch re-roll (-> `loweredswitch`) |
| `loweredswitchvalue` | P2 | on | correctness guards of the lowered-switch re-roll (-> `loweredswitch`) |
| `noreturn_extern` | P2 | on | known-function no-return rules (extern match, glibc error(), Go list) (-> `noreturn_known`) |
| `noreturn_externmatch` | P2 | on | known-function no-return rules (extern match, glibc error(), Go list) (-> `noreturn_known`) |
| `tailcallsaved` | P2 | on | narrows tailcallframe (-> `tailcallframe`) |
| `foldcallretphi` | P6 | on | second gate of the same fold (-> `foldcallret`) |
| `itecondlist` | P8 | on | lets the diamond matchers see a BlockList condition (-> `iteregion`) |
| `orchain` | P8 | on | declines a returndup split (-> `returndup`) |

### AUTO: on where the target is, inert elsewhere

Each row's text now states the target. Two need a gate added before the flip
(`v850indirectbranch` matches any register-indirect call; `linuxsyscall` must check the ELF
OS ABI), and `msvcstackguard` needs a `/GS` fixture to measure against.

| option | phase | default | reason |
|---|---|---|---|
| `cortexmvectors` | P1 | off | ARM only (Cortex-M / literal pools / Thumb); needs listing |
| `godwarfsig` | P1 | off | Go DWARF only |
| `ifuncfpret` | P1 | off | x86-64 ELF IRELATIVE only |
| `itaniumrtti` | P1 | off | ELF with __cxxabiv1 typeinfo only |
| `macho-arm64e` | P1 | off | Mach-O only |
| `objc` | P1 | off | Mach-O only |
| `poolentry` | P1 | off | ARM only (Cortex-M / literal pools / Thumb); needs listing |
| `ptrentry` | P1 | off | ARM only (Cortex-M / literal pools / Thumb); needs listing |
| `rtti` | P1 | off | PE only |
| `swiftmetadata` | P1 | off | Mach-O/ELF with swift5_* sections only |
| `tailcallentry` | P1 | off | ARM only (Cortex-M / literal pools / Thumb); needs listing |
| `unwindstarts` | P1 | off | Mach-O only |
| `cortexmpriv` | P2 | off | ARM Cortex-M only (VERSION_7M MRS/MSR guard) |
| `linuxsyscall` | P2 | off | i386 Linux ELF only; needs an OSABI gate added first |
| `sparcstructret` | P2 | off | SPARC unimp-trap idiom only |
| `v850indirectbranch` | P2 | off | V850 only; needs an arch gate added to its predicate first |
| `x64syscall` | P2 | off | x86-64 only (SYSCALL user-op) |
| `armfloatargs` | P4 | off | ARM VFP hard-float ELF only |
| `armfloatreturn` | P4 | off | ARM VFP hard-float ELF only |
| `rustabi` | P4 | off | Rust only (ScalarPair returns); already has an auto value |
| `msvcstackguard` | P7 | off | PE /GS only; the ELF twin (stackguard) is already on |

### FLIP: default off -> on

Datatest assertions that pin the upstream spelling get `option <name> off` in their script,
the precedent set by `returndup`, `truthycond`, `branchflip`, `foldcallret`, `braceelide` and
`castwiden`.

| option | phase | default | reason |
|---|---|---|---|
| `lazyfuncs` | P1 | off | byte-identical output, 30-40% faster single-function decompile |
| `calltargettype` | P5 | off | in the aggressive preset with measured net-positive evidence |
| `ptrdepthcap` | P5 | off | in the aggressive preset with measured net-positive evidence |
| `libcnames` | P6 | off | in the aggressive preset with measured net-positive evidence |
| `ctypes` | P9 | off | presentation default `auto` already applies at every size; pin the datatests |
| `importdata` | P9 | off | in the aggressive preset with measured net-positive evidence |
| `partialaccess` | P9 | off | presentation default `auto` already applies at every size; pin the datatests |
| `voidtailreturn` | P9 | off | presentation default `auto` already applies at every size; pin the datatests |

### MERGE: one option per mechanism (follow-up)

| option | phase | default | reason |
|---|---|---|---|
| `aifcorroborate` | P1 | off | aif=off|on|strict|corroborate (-> `aif`) |
| `aifstrict` | P1 | off | aif=off|on|strict|corroborate (-> `aif`) |
| `bejoinpass` | P4 | whole | bejoin + bejoinpass (-> `bejoin`) |
| `fieldtype` | P5 | off | structsynth + structmerge + structheadless + fieldtype + stackstruct (-> `structsynth`) |
| `structheadless` | P5 | off | structsynth + structmerge + structheadless + fieldtype + stackstruct (-> `structsynth`) |
| `structmerge` | P5 | off | structsynth + structmerge + structheadless + fieldtype + stackstruct (-> `structsynth`) |
| `stackstruct` | P6 | off | structsynth + structmerge + structheadless + fieldtype + stackstruct (-> `structsynth`) |
| `retsplitglobal` | P8 | on | retsplitglobal + retsplitlimit (-> `retsplit`) |
| `taildup` | P8 | on | gotoreduce and taildup describe the same rewrite (-> `gotoreduce`) |
| `castarith` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castimplied` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castindex` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castnarrow` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castsign` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castternary` | P9 | on | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `castwiden` | P9 | literal | seven implied-cast elisions; one `casts` policy (-> `casts`) |
| `rustfmtargs` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtflags` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtio` | P9 | off | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtliteral` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtpanic` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtpieces` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtresult` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtstr` | P9 | on | nine flags for one Rust formatting recovery (-> `rustfmt`) |
| `rustfmtwrite` | P9 | off | nine flags for one Rust formatting recovery (-> `rustfmt`) |

### KEEP

**destructive (REMOVES CODE or splices bodies)** (24): `argclobber`, `callarrayextent`, `cleanupcode`, `constselectjump`, `funcboundflow`, `gobool`, `gobounds`, `gomorestack`, `gowritebarrier`, `jtsharepartial`, `msvcstrappend`, `outline`, `overlapbranch`, `ptrauthtrap`, `returnpair`, `securitycheck`, `stackalias`, `stackguard`, `switchguardbound`, `switchmodbound`, `switchmultipred`, `switchsharedcase`, `tailcallframe`, `unrolledguard`

**faithfulness vs readability judgement** (61): `armconstshift`, `arraynotation`, `arraystride`, `boolbyte`, `braceelide`, `branchflip`, `bytearrayc`, `callbacktype`, `callerptr`, `callrettype`, `castobject`, `charbyte`, `charptr`, `condfold`, `crossjumprevert`, `dedupitetail`, `dedupvardecls`, `devectorize`, `earlyreturn`, `elemptr`, `endptrbound`, `floatglobals`, `foldcallret`, `globalarrayanchor`, `globalref`, `gotoreduce`, `guardarm`, `ifelseflatten`, `indirectonly`, `iteboolean`, `iteexpr`, `iteregion`, `loopbreak_recovery`, `loopcondhoist`, `loopcontinue`, `loweredswitch`, `memsetrecover`, `mixedtailret`, `narrowret`, `nulterminator`, `paramcopyhoist`, `passthrough`, `ppcapu`, `realtypes`, `regionedgeorder`, `regionlooprefine`, `regionstructure`, `regnarrow`, `regsigned`, `rettriangle`, `returndup`, `rodatastring`, `stackaddrargtrial`, `stackviews`, `strlenscan`, `structdefs`, `switchreturn`, `switchselector`, `tailcalljump`, `truthycond`, `varargsharedfloat`

**heuristic scanner / speculative discovery (precision vs recall)** (24): `addrtable`, `addrtakenentry`, `aif`, `armframes`, `cfstrings`, `dwarf`, `dwarf_lines`, `eh_frame_full`, `entry_disc`, `fast_funcdisc`, `fid`, `flowmode`, `funcstart_patterns`, `listing`, `noreturn_disc`, `noreturn_known`, `noreturn_propagate`, `noreturn_reach`, `operand_refs`, `pdb`, `strings`, `widestrings`, `widestrings32`, `x86listingswitch`

**multi-valued policy** (42): `arrayextent`, `arraysize`, `bejoin`, `calleevote`, `callmechanisms`, `calloverlap`, `callwide`, `compareform`, `condstmts`, `cppsig`, `deadcascade`, `declorder`, `fatstr`, `formatstring`, `fpuemu`, `goabi`, `impliedrefs`, `indexaliasguard`, `infloopstyle`, `int3pad`, `jumptablemax`, `libctypes`, `namestyle`, `narrowargs`, `narrowext`, `outlang`, `pebnames`, `persistcalls`, `ppc32picimports`, `protoorder`, `ptrfromuse`, `readonlycalls`, `retsplitlimit`, `rustprobestack`, `signedness`, `sourcelang`, `spillargtrial`, `structsynth`, `symbolnamechars`, `syscallregs`, `termdup`, `warnstyle`

**speed trade** (3): `highsliceguard`, `libgccsigs`, `rustarrayzero`

## Not flipped, and why

- `stackstruct`, `regionedgeorder`, `rettriangle`, `iteexpr`, `devectorize`,
  `globalarrayanchor`, `constselectjump`, `switchmodbound`, `switchguardbound`,
  `unrolledguard`: in the `aggressive` preset but each has a recorded regression, a
  destructive shape or a speed cost. They stay preset-only.
- `charptr`, `paramcopyhoist`, `guardarm`, `loopcondhoist`, `switchselector`, `condfold`,
  `regsigned`: measured neutral or negative on decbench, or unevaluated.
- `listing`, `aif`, `funcstart_patterns`, `fid`, `addrtable`, `operand_refs`,
  `eh_frame_full`, `widestrings32`: heuristic discovery with a measured precision cost; the
  `decompile-all` drivers and the presets already pick them per surface.
- `highsliceguard`, `rustarrayzero`, `rustfmtio`, `rustfmtwrite`, `libgccsigs`,
  `rustprobestack`: over the speed budget or costing a recovery elsewhere.

## Execution

One PR per bucket slice, each gated by the four gates plus `make test-cli`:

1. P4 argument/return recovery fixes (REMOVE)
2. P3 dataflow and heritage fixes (REMOVE)
3. P1 loader and discovery fixes, P2 flow fixes (REMOVE + FOLD)
4. P5/P6/P8/P9 type, variable and printer fixes, the JSON report toggles (REMOVE + FOLD)
5. Target-gated defaults (AUTO)
6. Presentation defaults (FLIP)
7. Merges (MERGE), one family per PR

