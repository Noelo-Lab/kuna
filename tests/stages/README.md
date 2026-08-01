# Stage-model issue testcases

kuna-owned datatests derived from **real, open decompiler issues** (sourced from the
PHADE issue dataset), each demonstrating that the issue is fixable through the kuna
stage model (`docs/phases.md`): a named sub-stage decision point,
controlled by a durable P0 assertion, instead of a hardcoded heuristic.

These run with the same upstream harness as `tests/datatests/` but from a
separate directory so the upstream baseline (`docs/baseline.json`) is never touched:

```bash
make test-stages
# or, directly:
decompiler/target/release/kuna test --datatests --datatests-dir tests/stages \
    --baseline docs/baseline-stages.json
```

After adding a testcase, re-record the stages baseline (never `docs/baseline.json`):

```bash
decompiler/target/release/kuna test --datatests --datatests-dir tests/stages \
    --save-baseline docs/baseline-stages.json
```

## Testcase conventions

Each `.xml` is a standard `<decompilertest>` (bytechunks + console script +
`<stringmatch>` assertions) and must document in a leading comment:

- the **upstream issue** it derives from (`GH-<n>`),
- the **stage / sub-stage** (new model) that owns the decision,
- the **P0 assertion** exercised (e.g. an `option ...` command),
- which asserts prove the **bug reproduces** (default pipeline) and which prove the
  **stage-level intervention fixes it**.

A testcase that can only be made to pass by patching decompiler *behavior* (rather
than exposing a decision) is a negative result for the stage model and belongs in the
writeup, not here.

## Current testcases

| File | Issue | Stage / sub-stage | P0 assertion |
|---|---|---|---|
| `gh558-compareform.xml` | [GH-558](https://github.com/NationalSecurityAgency/ghidra/issues/558) | S3 simplification quiescence (`canonicalcompare`) → S9 presentation (`presentcompare`) | `option compareform canonical\|original` |
| `gh558-arraynotation.xml` | [GH-558](https://github.com/NationalSecurityAgency/ghidra/issues/558) (secondary) | S9 literal/format policy (pointer notation) | `option arraynotation on\|off` |
| `gh2786-unaryspacing.xml` | [GH-2786](https://github.com/NationalSecurityAgency/ghidra/issues/2786) | S9 surface (adjacent sign tokens) | default-flip (DIV-1, `docs/history.md`) |
| `gh8471-thumbfuncptr.xml` | [GH-8471](https://github.com/NationalSecurityAgency/ghidra/issues/8471) | S5 const-pointer (Thumb mode bit) | `option thumbfuncptr on\|off` |
| `gh6930-inferfuncentry.xml` | [GH-6930](https://github.com/NationalSecurityAgency/ghidra/issues/6930) | S5 const-pointer (single-bit image base) | `option inferfuncentry on\|off` |
| `gh6990-returnpair.xml` | [GH-6990](https://github.com/NationalSecurityAgency/ghidra/issues/6990) | S4 trial finalization (return-register join) | `option returnpair pair\|single` |
| `gh1282-boolsignshift.xml` | [GH-1282](https://github.com/NationalSecurityAgency/ghidra/issues/1282) | S3 simplification quiescence (boolean sign mask) | `option booleanmask on\|off` |
| `gh7190-ovlesssimplify.xml` | [GH-7190](https://github.com/NationalSecurityAgency/ghidra/issues/7190) | S3 simplification quiescence (V850 OV-flag compare) | `option ovlesssimplify on\|off` |
| `gh8817-v850indbranch.xml` | [GH-8817](https://github.com/NationalSecurityAgency/ghidra/issues/8817) | S2 flow classification → switch model | `option v850indirectbranch on\|off` |
| `gh8913-addcarrychain.xml` | [GH-8913](https://github.com/NationalSecurityAgency/ghidra/issues/8913) | S5 value recovery (carry-chain wide add) | `option addcarrychain on\|off` |
| `gh9230-memsetseq.xml` | [GH-9230](https://github.com/NationalSecurityAgency/ghidra/issues/9230) | S5 const-sequence (constant-fill run) | `option memsetrecover on\|off` |
| `gh1537-simd-memset.xml` | [GH-1537](https://github.com/NationalSecurityAgency/ghidra/issues/1537) | S5 const-sequence (same sub-stage, global/SIMD form — pure routing, zero new code) | `option memsetrecover on\|off` |
| `gh9001-hcs12brn.xml` | [GH-9001](https://github.com/NationalSecurityAgency/ghidra/issues/9001) | S1 decode-table (HCS12 spec-fix, no option) | SLEIGH `:BRN rel8` consumes its operand |

| `gh1243-8051-addc.xml` | [GH-1243](https://github.com/NationalSecurityAgency/ghidra/issues/1243) | S2 pcode-lift (8051 spec-fix, no option) | SLEIGH ADDC carry-in computed in a 2-byte temp (`do_addc`) so the carry-out survives |

| `gh7418-x86-call-rel16-mask.xml` | [GH-7418](https://github.com/NationalSecurityAgency/ghidra/issues/7418) | S1 decode / S2 p-code lift (x86 spec-fix, no option) | SLEIGH `rel16` masks tempEIP to low 16 bits (32/16-bit) |

| `gh7139-x86-16-disp16-signed.xml` | [GH-7139](https://github.com/NationalSecurityAgency/ghidra/issues/7139) | S1 decode / operand rendering (x86 spec-fix, no option) | SLEIGH 16-bit disp16 (modrm mod=2) uses signed `simm16` |

| `gh8790-x86-prefetch-regform.xml` | [GH-8790](https://github.com/NationalSecurityAgency/ghidra/issues/8790) | S1 decode-table (x86 spec-fix, no option) | SLEIGH prefetch `0f 0d`/`0f 18` gain a mod=3 register-form variant |

| `gh9184-x86-fma-ymm256.xml` | [GH-9184](https://github.com/NationalSecurityAgency/ghidra/issues/9184) | S1/S2 decode + pcode-semantics (x86 spec-fix, no option) | SLEIGH YMM FMA uses 256-bit `local tmp:32` (was `tmp:16`) |

| `gh5897-arm-cpy-mov.xml` | [GH-5897](https://github.com/NationalSecurityAgency/ghidra/issues/5897) | S1 decode-table (ARM spec-fix, no option) | SLEIGH reg-to-reg move constructor renders UAL `mov`, not pre-UAL `cpy` |

| `gh8456-arm-mrs-banked.xml` | [GH-8456](https://github.com/NationalSecurityAgency/ghidra/issues/8456) | S1 decode-table (ARM spec-fix, no option) | SLEIGH MRS (Banked register) constructor added |

| `gh8391-arm-thumb-mrrc.xml` | [GH-8391](https://github.com/NationalSecurityAgency/ghidra/issues/8391) | S1 decode-table (ARM spec-fix, no option) | Thumb `mrrc`/`mcrr` un-guarded from `@ifndef CDE` so v8-m (CDE) decodes them |

| `gh7890-arm-vcvt.xml` | [GH-7890](https://github.com/NationalSecurityAgency/ghidra/issues/7890) | S2 p-code lift (ARM scalar VFP vcvt, spec-fix, no option) | SLEIGH `vcvt.*32` lifts to native `int2float` (no `VectorSignedToFloat` pseudo-op / fpscr read) |

| `gh1951-avr-rcallwrap.xml` | [GH-1951](https://github.com/NationalSecurityAgency/ghidra/issues/1951) | S1 decode-table / 02 P-code lift (avr8 SLEIGH spec-fix, no option) | SLEIGH rel12addr masks RJMP/RCALL target `& 0x0FFF` (4 KiW wrap) |

| `gh4788-v850-bcond-disp17.xml` | [GH-4788](https://github.com/NationalSecurityAgency/ghidra/issues/4788) | S1 decode-table (V850 spec-fix, no option) | SLEIGH adds the RH850 Bcond disp17 branch (`f9 07 49 fe` -> `bnc`, not `ld.hu`) |

| `gh6389-8085-undoc.xml` | [GH-6389](https://github.com/NationalSecurityAgency/ghidra/issues/6389) | S1 decode-table (8085 spec-fix, no option) | Nine undocumented 8085 instructions (DSUB/ARHL/RDEL/LDSI/SHLX/LHLX/RSTV/JNK/JK) now decode |

| `gh7451-riscvzfaflis.xml` | [GH-7451](https://github.com/NationalSecurityAgency/ghidra/issues/7451) | S1 decode-table (RISCV spec-fix, no option) | SLEIGH adds Zfa `fli.s` (constant table + `fli` pcodeop) |

| `gh6904-ppc-e500-concat44.xml` | [GH-6904](https://github.com/NationalSecurityAgency/ghidra/issues/6904) | P-code lift (PowerPC e500 spec-fix, no option) | `or`/`or.` (`mr`/`mr.`) zero-extend their 32-bit result on the 64-bit-register e500, killing spurious CONCAT44 at function boundaries (partial) |

| `gh1276-flagcompare-boolsign.xml` | [GH-1276](https://github.com/NationalSecurityAgency/ghidra/issues/1276) | S3 simplification quiescence (boolean-into-sign-bit compare) | `option flagcompare on\|off` |

| `gh8777-flagcompare-sborrow.xml` | [GH-8777](https://github.com/NationalSecurityAgency/ghidra/issues/8777) | S3 simplification quiescence (N==V SBORROW signed compare) | `option flagcompare on\|off` |

| `gh9191-switchmodbound.xml` | [GH-9191](https://github.com/NationalSecurityAgency/ghidra/issues/9191) | S2 switch model (modulo/and-mask LOAD-table index bound) | `option switchmodbound on\|off` |

| `gh8017-stackprobeloop.xml` | [GH-8017](https://github.com/NationalSecurityAgency/ghidra/issues/8017) | S2 stack-pointer normalization (stack-probe loop) | `option stackprobeloop on\|off` |

| `gh8467-dynamichashmax.xml` | [GH-8467](https://github.com/NationalSecurityAgency/ghidra/issues/8467) | S6 alias-facets (dynamic-symbol DynamicHash collision budget) | `option dynamichashmax on\|off` |

| `gh8500-stackalias.xml` | [GH-8500](https://github.com/NationalSecurityAgency/ghidra/issues/8500) | S6 alias-facets / heritage ordering (store-through stack-pointer-alias) | `option stackalias on\|off` |

| `gh8724-arraystride.xml` | [GH-8724](https://github.com/NationalSecurityAgency/ghidra/issues/8724) | S3 simplification quiescence (strided-induction array index → counter*stride) | `option arraystride on\|off` |

| `gh6882-sparcstructret.xml` | [GH-6882](https://github.com/NationalSecurityAgency/ghidra/issues/6882) | S2 flow classification (post-call `unimp` terminator) | `option sparcstructret on\|off` |

| `gh9203-condexe-loopcopy.xml` | [GH-9203](https://github.com/NationalSecurityAgency/ghidra/issues/9203) | S3 simplification quiescence (ActionConditionalConst const-COPY block placement) | `option condexeplace on\|off` |

| `gh9218-inputvarnodeadjust.xml` | [GH-9218](https://github.com/NationalSecurityAgency/ghidra/issues/9218) | S6 storage reconciliation (`stack-frame-layout`, overlapping input varnodes) | `option inputvarnodeadjust on\|off` |
| `ghangr-ite-region-converter-missing-5db28e.xml` | angr `test_ite_region_converter_missing_break_statement` (StackCanarySimplifier) | S7 region recovery (`edge-virtualization`, strip the -fstack-protector canary epilogue so the shared-return goto is eliminated) | `option stackguard on\|off` |
| `ghdec-whiledo-complex.xml` | decbench `O0-iproute2-ip-lookup_flag_data_by_name` (invalid C: statement inside `while(...)` parens) | S8 structure recovery (whileDo overflow-syntax decision; `FlowBlock::isComplex` virtual-dispatch parity, porting-divergence correctness fix, no option) | default (upstream `isComplex`: BlockList/BlockIf unconditionally complex) |
| `ghdec-isamode-inject.xml` | decbench `O2-noinline-betaflight-...-accDetect` + `O0-libopencm3-sdram-main` (ARM `setISAMode(1);` bookkeeping user-op emitted as a C statement inside jump-table-reachable blocks) | S2 lift/flow (`<callotherfixup>` injection drain in `generate_ops_with_jumptables`; porting-divergence correctness fix, no option) | default (upstream `flow.cc:819` drains `injectlist` on every jump-table round) |

C-surface normalization testcases (`kuna-cnorm-*.xml`, no GH issue; user-driven
readability defaults measured on the angr `fmt` corpus binary, DIV-34+ in
`docs/history.md`):

| File | Normalization | P0 assertion |
|---|---|---|
| `kuna-cnorm-protogap.xml` | no blank line between a function prototype and its `{` (DIV-34) | `option braceformat function skip\|next` (upstream option, kuna default flip) |
| `kuna-cnorm-nullprint.xml` | zero pointer constants render as `NULL`, not `(type *)0x0` (DIV-35) | `option nullprinting on\|off` (upstream option, kuna default flip) |
| `kuna-cnorm-compoundassign.xml` | `out = out OP y` statements render as `out OP= y`, `x += -c` folds to `x -= c` (DIV-36) | `option inplaceops on\|off` (upstream option, kuna default flip + ported `emitInplaceOp` consumer) |
| `kuna-cnorm-truthycond.xml` | boolean-context zero compares render truthy: `if (x)` / `if (!p)` (DIV-37) | `option truthycond on\|off` (new kuna settable, P9 `condition-form`) |
| `kuna-cnorm-braceelide.xml` | single-statement if bodies drop their braces (statement indented) (DIV-38) | `option braceelide on\|off` (new kuna settable, P9 `brace-form`) |
| `kuna-cnorm-warnstyle.xml` | warnings render as terse `// slug` end-of-line comments (DIV-39) | `option warnstyle inline\|banner` (new kuna settable, P9 `warning-style`) |

Infrastructure testcases (no GH issue; they regression-test the kuna stage machinery
itself): `kuna-console.xml` (registry + `stage list/map/status`), `kuna-assert.xml`
(`kassert` routing + reported rewind scopes), `kuna-restarts.xml` (restart-reason
observability on the switchmulti multistage restart), `kuna-pipeline-quality.xml`
(reduced-pipeline sub-queries + the goto-quality metric), `kuna-regions.xml`
(S7 region-identification observability: `region tree/blocks/walk` from the angr
RegionIdentifier port over the REAL `bblocks` CFG — `build_from_block_graph`, the
W7 block-graph adapter closed in regionstructure Inc 0, including the per-block
`endsWithBranchindOrCbranch` `lastOp` probe — on a loop+diamond CFG;
`docs/spec/07-regions.md`), `kuna-listing-flag.xml`
(the Listing/xref disassembly tier flag `option listing on|off`, default-off: parses,
round-trips, and is inert on the XML path — PR1 of the Listing/xref tier).
`ghdec-fast-funcdisc.xml` clears that wrapper architecture and loads a real PE
twice: option off reproduces the missing direct callee, and option on discovers
and decompiles its body.
