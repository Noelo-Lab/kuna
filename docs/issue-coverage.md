# PHADE issue coverage ledger

Durable, resumable state for the "all remaining reproduced issues" work session. One row
per **reproduced** PHADE Ghidra issue (`repro.status == "reproduced"` in
`~/github/PHADE/issue_dataset/ghidra/issues/*.json`). This file is the single source of
truth a resumed context reconstructs from — it is updated **after every issue**.

- **58** reproduced total · **11** done in prior sessions · **1** deferred prior · **46** in scope this session.
- Buckets: `stage-exposure` (option-gated C++ decompiler decision, the kuna pattern) ·
  `spec-fix` (SLEIGH `.slaspec`/`.sinc`/`.cspec`/`.pspec` correction) ·
  `not-viable` (needs whole new instruction families, the Java/GUI front end, or can't be
  expressed in the C++-only harness — reason required).
- Status vocabulary: `done(prior)` · `deferred(prior)` · `triaged` (reproduced + bucketed,
  not yet started) · `assigned` (ElementId/option locked, in a wave) · `committed` (fix
  landed, gates green) · `deferred` (attempted, didn't converge — reason) ·
  `not-viable` (reason).
- **ElementId registry**: stage-exposure fixes draw a unique ElementId from 4010 upward,
  monotonic, never recycled (gaps are fine). 4000–4009 are taken by prior fixes.
- Repro method → harness assertion surface: `C` = `print C`, `disasm` = `disassemble`,
  `pcode` = `print raw`.

## SLEIGH scope rule (user decision)

Small missing-instruction adds (a few constructors, exact encoding in the issue) are
viable `spec-fix`. Whole extension families (FMA4, XOP, Xtensa FLIX, the GH-2033 catalog)
are `not-viable`.

## Done in prior sessions (11)

| GH | Arch | Bucket | Option / ElementId | Testcase | Commit |
|---|---|---|---|---|---|
| 558 | x86-64 | stage-exposure | `compareform` 4000 / `arraynotation` 4001 | gh558-compareform.xml, gh558-arraynotation.xml | (prior) |
| 1282 | 8051 | stage-exposure | `booleanmask` 4005 | gh1282-boolsignshift.xml | (prior) |
| 1537 | x86 | stage-exposure | `memsetrecover` 4009 (routing) | gh1537-simd-memset.xml | (prior) |
| 2786 | x86 | default-flip (DIV-1) | — | gh2786-unaryspacing.xml | (prior) |
| 6930 | x86-64 | stage-exposure | `inferfuncentry` 4003 | gh6930-inferfuncentry.xml | (prior) |
| 6990 | Sparc | stage-exposure | `returnpair` 4004 (opt-in) | gh6990-returnpair.xml | (prior) |
| 7190 | V850 | stage-exposure | `ovlesssimplify` 4006 | gh7190-ovlesssimplify.xml | (prior) |
| 8471 | ARM | stage-exposure | `thumbfuncptr` 4002 | gh8471-thumbfuncptr.xml | (prior) |
| 8817 | V850 | stage-exposure | `v850indirectbranch` 4007 (opt-in) | gh8817-v850indbranch.xml | (prior) |
| 8913 | x86(6502) | stage-exposure | `addcarrychain` 4008 | gh8913-addcarrychain.xml | (prior) |
| 9230 | x86-64 | stage-exposure | `memsetrecover` 4009 | gh9230-memsetseq.xml | (prior) |

## Deferred in prior sessions (1)

| GH | Arch | Reason |
|---|---|---|
| 28 | x86 | Feature request (per-function killed-register list) — GUI enhancement, not a core decompiler bug. |

## Remaining this session (46) — triaged

Live triage (Phase B, 9-agent workflow) results. Buckets: **20 spec-fix · 12 stage-exposure · 13 not-viable · 1 already-correct**. ElementIds locked for stage-exposure options (4010–4018; three options are shared by issue pairs).

| GH | Arch | Bucket | Stage | Family | Diff | EID | Option | Status |
|---|---|---|---|---|---|---|---|---|
| 766 | 68000 | not-viable | S2 | m68k-abs-short-addr-sext | medium | — | — | **not-viable** |
| 809 | MIPS | not-viable | P0 | java-constant-reference-analyzer | hard | — | — | not-viable |
| 1243 | 8051 | spec-fix | S2 | 8051-addc-carry-semantics | easy | — | — | **committed** |
| 1276 | 8051 | stage-exposure | S5 | flag-bit-extraction-simplification | hard | 4010 | flagcompare | **committed** |
| 1708 | x86 | not-viable | S1 | x86-fma4-decode-catalog | hard | — | — | not-viable |
| 1951 | AVR | spec-fix | S2 | avr8-rel-branch-wrap-mask | medium | — | — | **committed** |
| 2033 | x86 | not-viable | S1 | x86-mishegos-decode-catalog | hard | — | — | not-viable |
| 3723 | AARCH64 | already-correct | S2 | arm-missing-instruction-decode | medium | — | — | **already-correct** |
| 3847 | x86 | already-correct | S2 | x87-fpu-semantics | easy | — | — | already-correct |
| 4788 | V850 | spec-fix | S2 | v850-rh850-missing-decode | medium | — | — | **committed** |
| 5666 | x86 | not-viable | S2 | x86-addrsize-stack-push | medium | — | — | **not-viable** |
| 5897 | ARM | spec-fix | S2 | arm-mnemonic-display | easy | — | — | **committed** |
| 6333 | V850 | not-viable | S2 | v850-rh850-missing-decode | hard | — | — | not-viable |
| 6342 | Xtensa | not-viable | S2 | xtensa-flix-bundle-width | hard | — | — | not-viable |
| 6389 | 8085 | spec-fix | S1 | 8085-undocumented-opcodes-missing | medium | — | — | **committed** |
| 6664 | x86 | not-viable | S3 | x86-realmode-segmented-addr | hard | — | — | not-viable |
| 6674 | V850 | stage-exposure | S2 | jumptable-modulo-bound | hard | 4011 | switchmodbound | triaged |
| 6682 | x86 | not-viable | S2 | x86-realmode-mz-overlay-load | hard | — | — | not-viable |
| 6723 | x86_64 | not-viable | S5 | x86-xor-zero-idiom-param-recovery | hard | — | — | not-viable |
| 6836 | x86 | not-viable | S1 | coff-loader-section-layout | hard | — | — | not-viable |
| 6858 | x86 | stage-exposure | S6 | x86-stack-clash-probe-loop | hard | 4012 | stackprobeloop | triaged |
| 6882 | Sparc | stage-exposure | S4 | sparc-struct-return-unimp-flow | hard | 4016 | sparcstructret | **committed** |
| 6904 | PowerPC | spec-fix | S2 | ppc-e500-32bit-over-64bit-reg | hard | — | — | **committed** |
| 7139 | x86 | spec-fix | S1 | x86-16bit-disp-signedness | medium | — | — | **committed** |
| 7332 | x86 | not-viable | S6 | x86-alloca-chkstk-callfixup | medium | — | — | **not-viable** |
| 7377 | ARM | not-viable | S2 | missing-core-pcode-op-rotate | hard | — | — | not-viable |
| 7418 | x86 | spec-fix | S2 | x86-addrsize-stack-push | easy | — | — | **committed** |
| 7451 | RISC-V | spec-fix | S2 | riscv-zfa-missing-decode | medium | — | — | **committed** |
| 7890 | ARM | spec-fix | S2 | arm-vfp-pseudoop-fpscr | hard | — | — | **committed** |
| 8017 | x86 | stage-exposure | S6 | x86-stack-clash-probe-loop | hard | 4012 | stackprobeloop | **committed** |
| 8028 | x86 | not-viable | S1 | java-autoanalysis-code-following | hard | — | — | not-viable |
| 8391 | ARM | spec-fix | S2 | arm-thumb-cde-guard | medium | — | — | **committed** |
| 8456 | ARM | spec-fix | S2 | arm-missing-instruction-decode | medium | — | — | **committed** |
| 8467 | AARCH64 | stage-exposure | S6 | dynamic-hash-maxduplicates | medium | 4013 | dynamichashmax | **committed** |
| 8500 | x86_64 | stage-exposure | S3 | stack-alias-deadstore-elim | hard | 4014 | stackalias | **committed** |
| 8694 | x86_64 | not-viable | P0 | dwarf-local-import-trigger | medium | — | — | not-viable |
| 8724 | x86_64 | stage-exposure | S5 | ptr-arith-array-struct-recovery | hard | 4015 | arraystride | **committed** |
| 8777 | PIC | stage-exposure | S5 | flag-bit-extraction-simplification | hard | 4010 | flagcompare | **committed** |
| 8790 | x86 | spec-fix | S1 | x86-prefetch-regform-modrm | easy | — | — | **committed** |
| 8794 | x86 | not-viable | S1 | x86-xop-decode-catalog | hard | — | — | not-viable |
| 8844 | AVR | spec-fix | S2 | avr8-rel-branch-wrap-mask | medium | — | — | **committed** |
| 9001 | HCS12 | spec-fix | S1 | hcs12-brn-operand-consume | easy | — | — | **committed** |
| 9184 | x86 | spec-fix | S2 | x86-avx-vector-width | easy | — | — | **committed** |
| 9191 | x86_64 | stage-exposure | S2 | jumptable-modulo-bound | hard | 4011 | switchmodbound | **committed** |
| 9203 | riscv64 | stage-exposure | S3 | condexe-multiequal-block-placement | hard | 4017 | condexeplace | **committed** |
| 9218 | 68000 | stage-exposure | S2 | input-varnode-storage-reconcile | hard | 4018 | inputvarnodeadjust | **committed** |

### Per-issue triage detail

**GH-766** (68000 · S2 · medium) — spec-fix
  - observed: `disasm: 0x00000000: move.b (A0)+,(-0x180).w ; raw pcode: r0xfffffe80:1(...) = u0x0000b600 ; print C: 'xRamfffffe80 = *in_A0;'. The 16-bit absolute-short operand 0xfe80 is sign-extended to the full 32-bit address 0xfffffe80 (shown as xRamfffffe80) instead of masking to the 24-bit bus 0xfffe80.`
  - bug_token: `xRamfffffe80`
  - bytes: `11d8fe80`
  - spec: specs/Ghidra/Processors/68000/data/languages/68000.sinc

**GH-809** (MIPS · P0 · hard) — not-viable
  - not-viable: The reportable defect (the wrong DAT_ffff8d40 data reference instead of 0x8d40) is created by the Java-side MipsConstantReferenceAnalyzer, which does not exist in kuna (no loader/analyzer). The lw decode and sign-extension that DO exist in kuna are confirmed-correct MIPS semantics (the issue's own bug_summary records the maintainer's note that 'the lw decode itself sign-extends like IDA' and 'the dispute centers on the analyzer's reference creation').
  - observed: `0x00000000: lui a1, 0x0\n0x00000004: lw a1, -0x72c0(a1)  (decode/disasm correct). lw SLEIGH: 'RT = sext(*[ram]:4 OFF_BASE)' (mips32Instructions.sinc:398-399); with a1=0 the effective addr is sext(-0x72c0)=0xffff8d40. The DAT_ffff8d40 *reference* is never synthesized in kuna (no Java analyzer); decom`
  - bug_token: `0xffff8d40`
  - bytes: `3c0500008ca58d40`

**GH-1243** (8051 · S2 · easy) — spec-fix
  - observed: `ADDC at 0x3 disassembles correctly ('ADDC A,#0x0'). The buggy semantics are verbatim in the vendored spec 8051_main.sinc lines 712-715: ':ADDC Areg,Data is ... { tmp:1 =$(CY)+ Data; addflags(ACC,tmp); ACC = ACC + tmp; resultflags(ACC); }'. $(CY) = PSW[7,1] => INT_RIGHT (register,0x48,1),(const,0x7,4`
  - bug_token: `tmp:1 =$(CY)+`
  - bytes: `D374FF340074005002740 1F580D3740034FF74005002740 2F590 80FE`
  - spec: specs/Ghidra/Processors/8051/data/languages/8051_main.sinc

**GH-1276** (8051 · S5 · hard) — stage-exposure
  - observed: `uVar1 = (uVar3 < 0xcd) + 0x37;\n    cVar4 = BANK0_R4 - uVar1;\n    uVar3 = uVar3 + 1;\n  } while ((BANK0_R4 < uVar1) << 7 < '\0');  -- and the TI check IS preserved: 'cVar2 = TI; if (cVar2 == \x01) break;' (read every iteration, no 'Removing unreachable block').`
  - bug_token: `<< 7 <`
  - bytes: `e4fdfcc2998f99209910 0ded ae04 7001 0c14 c394 cd ee94 37 40 ed 309 90 37 f01 22 7 f00 22`

**GH-1708** (x86 · S1 · hard) — not-viable
  - not-viable: Needs the whole AMD FMA4 extension family. fma.sinc contains only FMA3 (vfmadd132pd/vfmadd213pd/... 3-operand VEX forms); there are NO FMA4 4-operand encodings (VEX 0x68/0x69/0x6A/0x6B map: vfmaddps/pd/ss/sd, vfmsub*, vfnmadd*, vfnmsub*, vfmaddsub*, vfmsubadd* -- ~20 mnemonics x xmm/ymm x VEX.W operand-order swap = 60+ constructors). This is a whole instruction-set extension family, not a few-constructor add, so per the SLEIGH SCOPE RULE it is not-viable.
  - observed: `> Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `Unable to resolve constructor`
  - bytes: `c4e3f96840f070c4c37168c800c4c36968d110c4c36168da20`
  - spec: specs/Ghidra/Processors/x86/data/languages/fma.sinc

**GH-1951** (AVR · S2 · medium) — spec-fix
  - observed: `disasm: 0x0000: rcall -0xff2 ; raw pcode: '0x0000:5: call ffunc_0xf807:8(free)'. The rcall at word 0 lifts to a CALL whose target is word 0xf807 (byte addr 0x1f00e) — the full-width unmasked wrap — instead of the 8KB/12-bit-masked word 0x807 (byte 0x100e).`
  - bug_token: `0xf807`
  - bytes: `06d8`
  - spec: specs/Ghidra/Processors/Atmel/data/languages/avr8.sinc

**GH-2033** (x86 · S1 · hard) — not-viable
  - not-viable: This is the canonical large GH-2033 mishegos differential-fuzzing catalog explicitly named as not-viable in the triage guidance ('the large GH-2033 catalog'). The issue body enumerates dozens of distinct mnemonics (BNDCL, BNDMOV, CLRSSBSY, CLWB, PREFETCHNTA, RDPID->RDSEED, INCSSPQ, RDSSPQ, WRSSQ, WRUSSQ, MOVSXD, JRCXZ, FXSAVE64, INT1, RETF, VADDSD, VPEXTRB, XBEGIN, ...) PLUS broad prefix-handling rework (REX-before-prefix, 66 in 64-bit mode, F3/F2 mandatory-prefix priority, LOCK/BND/segment-override line rendering). That is a whole-module decode/prefix overhaul, far beyond a few constructors.
  - observed: `0x00000000: NOP       dword ptr [RAX]`
  - bug_token: `NOP       dword ptr \[RAX\]`
  - bytes: `f30f1a08`
  - spec: specs/Ghidra/Processors/x86/data/languages/mpx.sinc;specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-3723** (AARCH64 · S2 · medium) — spec-fix
  - observed: `/* WARNING: Control flow encountered bad instruction data */\nvoid FUN_0(void)\n{\n  /* WARNING: Bad instruction - Truncating control flow here */\n  halt_baddata();\n}\n... > Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `halt_baddata\(\)`
  - bytes: `FEDEFFE7`
  - spec: specs/Ghidra/Processors/AARCH64/data/languages/AARCH64instructions.sinc

**GH-3847** (x86 · S2 · easy) — already-correct
  - observed: `Raw pcode: 'u... = log2(ST0(i)); ... ST1 = ST1(i) * u...(cast log2)' . Decompiled C: 'fVar1 = (float10)log2(in_ST0); return in_ST1 * fVar1;' -- the FYL2X correctly computes ST1 * log2(ST0).`
  - bytes: `d9f1`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-4788** (V850 · S2 · medium) — spec-fix
  - observed: `0x00000000: ld.hu     -0x1b8[r25], r0`
  - bug_token: `ld\.hu`
  - bytes: `f9074 9fe`
  - spec: specs/Ghidra/Processors/V850/data/languages/Instructions/Special.sinc

**GH-5666** (x86 · S2 · medium) — spec-fix
  - observed: `Raw pcode: SP = SUB42(...,#0x0); SP = SP + #0xfffc:2; u... = segment(SS(i),SP); *(ram,...) = #0x1006. Decompiled C: 'pxVar1 = (xunknown4 *)segment(in_SS,(int2)&stack0x00000000 + -4); *pxVar1 = 0x1006;' -- 2-byte SP arithmetic + segment(SS) store + bogus (int2) cast, function args lost.`
  - bug_token: `segment\(in_SS`
  - bytes: `67e8c3ffffff`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-5897** (ARM · S2 · easy) — spec-fix
  - observed: `0x00000000: cpy       r12,r7`
  - bug_token: `cpy`
  - bytes: `07c0a0e1`
  - spec: specs/Ghidra/Processors/ARM/data/languages/ARMinstructions.sinc

**GH-6333** (V850 · S2 · hard) — not-viable
  - not-viable: Whole RH850 instruction-set superset, not these two encodings alone: the issue is titled 'RH850 Missing Instructions' (plural) and is fundamentally that RH850 is a superset of V850 with a large catalog of missing instructions; the canonical fix is a whole third-party extension module (esaulenka/ghidra_v850). The issue body provides NO mnemonics or semantics for the two sample encodings (only the bytes and 'they should decode'), so a correct small add cannot be constructed from the issue.
  - observed: `Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `Unable to resolve constructor`
  - bytes: `e627200 8ea1f2008`
  - spec: specs/Ghidra/Processors/V850/data/languages/Instructions/Logic.sinc;specs/Ghidra/Processors/V850/data/languages/Instructions/Arithmetic.sinc

**GH-6342** (Xtensa · S2 · hard) — not-viable
  - not-viable: Needs a whole variable-width FLIX (VLIW bundle) decoding framework / processor-configuration-specific instruction extension, not a few constructors. Xtensa FLIX is explicitly listed in the triage not-viable examples. The issue itself states flixinsn is hard-coded to 64 bits and the canonical fix is a separate module (flto/ghidra-xtensa-flix); decoding 6- and 11-byte HiFi4 bundles into multiple parallel slot ops (addmi/l32i.n/blti.w15) requires a per-config FLIX bundle format + parallel-slot p-code, a whole-extension effort.
  - observed: `0x00000000: FLIX      0x47c7f 0x00000003: l32i.n    a1, a3, 0x14 0x00000005: bnez.n    a0, 0x9`
  - bug_token: `FLIX`
  - bytes: `fec747185 3cc`
  - spec: specs/Ghidra/Processors/Xtensa/data/languages/flix.sinc;specs/Ghidra/Processors/Xtensa/data/languages/xtensaArch.sinc

**GH-6389** (8085 · S1 · medium) — spec-fix
  - observed: `Each undocumented opcode byte yields 'Low-level ERROR: r0x0000: Unable to resolve constructor / Aborting process' (no instruction decoded), whereas a valid byte (00=NOP) decodes fine. The undocumented opcodes (DSUB 0x08, ARHL 0x10, RDEL 0x18, LDSI 0x38, LHLX 0xED, SHLX 0xD9, RSTV 0xCB, JK 0xFD, JNK `
  - bug_token: `Unable to resolve constructor`
  - bytes: `08`
  - spec: specs/Ghidra/Processors/8085/data/languages/8085.slaspec

**GH-6664** (x86 · S3 · hard) — not-viable
  - not-viable: The reported AddressOutOfBoundsException ('Offset must be between 0x0 and 0x10ffef, got 0x24105e2c') is thrown in Ghidra's Java AddressSpace.getAddress when it constructs a Java Address from the flat far-pointer fullEncoding (segment<<16 \| offset). kuna's C++ core computes fullEncoding=(base<<8*innersz)+(val) = 0x2410<<16+0x5e2c in SegmentedResolver::resolve (architecture.cc:1455) but the actually-resolved address uses the correct segmentop pcode (base<<4)+inner from x86-16-real.pspec, and the C++ core never builds a Java Address, so it does NOT throw. Reproducing the crash requires (a) Ghidra's Java MzLoader to set the tracked DS=0x2410 context for the constResolve, and (b) the Java AddressSpace bounds check. Both are Java front-end, absent from kuna's C++-only harness. kuna also cannot load the MZ at all via its BFD loader ('unsupported/!recognized binary').
  - observed: `Function FUN_6260f decompiles cleanly to full C with no exception. segmentop user-ops fire e.g. 'u0x00009a00 = segmentop(ram,#0x95d:2,u0x00006400:2)' and resolve via the spec pcode res=(zext(base)<<4)+zext(inner). No 0x24105e2c, no AddressOutOfBounds, no 'Offset must be between', zero ERROR/Aborting`
  - target: 0x6260f (FUN_6260f, seg:off 6257:009f)
  - spec: specs/Ghidra/Processors/x86/data/languages/x86-16-real.pspec

**GH-6674** (V850 · S2 · hard) — stage-exposure
  - observed: `void FUN_30404(uint4 param_1)\n{ ... case 2: ... if (*(uint1 *)(unaff_gp + -0x729f) == param_1) {\n  *(xunknown1 *)(unaff_gp + -0x7088) = 3;\n  /* WARNING: Could not recover jumptable at 0x000304d2. Too many branches */\n  /* WARNING: Treating indirect jump as call */\n  (*(code *)(iRam000304d6 * 2 `
  - bug_token: `\(\*\(code \*\)\(iRam000304d6 \* 2 \+ 0x304d4\)\)\(\)`
  - target: 0x30404 (raw binary, base 0x0; loaded via kprobe bytechunk at offset 0x30404)

**GH-6682** (x86 · S2 · hard) — not-viable
  - not-viable: Two-fold. (1) The function at seg:off 7e7c:0b9c is a DOS overlay segment: its file offset 0x8ecec exceeds the file size 0x8a8c0 (0 bytes available), so it is loaded into a separate overlay region by Ghidra's Java MzLoader, which kuna lacks; the bytes cannot be located/mapped in the C++-only harness ('Bytes at 0x0007f35c are not mapped'). (2) The headline error 'Response buffer size exceeded' / 'Decompiler results exceeded payload limit' is a Java-side response/payload cap (Max-Payload MBytes, LinkedByteBuffer, setMaxPayloadMBytes), not a C++ LowlevelError. Only 'Unlinked join address' (translate.cc:749) is a C++ error, but it cannot be reached because the overlay function bytes are unmappable.
  - observed: `decomp_test_dbg: 'Low-level ERROR: Bytes at 0x0007f35c are not mapped / Unable to proceed with function: FUN_7f35c'. The function's linear address (0x7f35c = 0x7e7c*16+0xb9c) lies beyond the MZ load image (0x7af30) and its file offset 0x8ecec is past EOF (file 0x8a8c0): it is a DOS overlay segment, `
  - target: 7e7c:0b9c (linear 0x7f35c) - not mappable (overlay)

**GH-6723** (x86_64 · S5 · hard) — not-viable
  - not-viable: bug not reproducible C-only: the dropped-arg call (evbuffer_drain) sits behind PLT relocations the Java loader resolves; kuna's unrelocated ELF never surfaces the call. Revisit as stage-exposure if a relocated repro is built.
  - observed: `evhttp_read_body decompiles (debug syms resolve many calls: evhttp_request_free.part.0, evhttp_is_connection_close, evhttp_request_dispatch, evhttp_make_header). But ZERO 'evbuffer' calls appear: the 'call evbuffer_drain' / 'call evbuffer_readln' sites are PLT relocations (R_X86_64_PLT32) in an unre`
  - target: evhttp_read_body @ 0x5bd0 (unrelocated REL); buggy drain call at 0x5cda

**GH-6836** (x86 · S1 · hard) — not-viable
  - not-viable: Pure Java front-end COFF-loader bug. kuna has no Ghidra Java CoffLoader; it loads via BFD, which recognizes the file as pe-i386 and lays the split address space out CORRECTLY (.text@0x0, .data@0xc0000000, .bss@0xc001dcfc), so meminit@0x14a4 already decompiles to a real function. The bug only exists in Ghidra's Java COFF loader section/symbol-address placement, which is entirely outside the C++-only kuna harness.
  - observed: `void meminit(int4 param_1,int4 param_2)\n{\n  ...\n  iRamc001e1e0 = param_2 - param_1;\n  xRamc0021bd0 = 0xc0021bc0;\n  pxVar1 = (xunknown1 *)(iRamc0021be0 + (param_1 - iRamc0021d98) * 0x20);\n  while (1 <= iVar2) { ... } return; }  -- a real, sensible function body (NOT 'meminit(void){return;}', no`
  - target: meminit / 0x14a4

**GH-6858** (x86 · S6 · hard) — stage-exposure
  - observed: `do { pxVar5 = (xunknown1 *)ppxVar1; *(xunknown4 *)&pxVar5[-0x1000] = *(xunknown4 *)&pxVar5[-0x1000]; ppxVar1 = (xunknown1 **)&pxVar5[-0x1000]; } while (&pxVar5[-0x1000] != axStack_14018); ... *(int4 *)&pxVar5[-0x1148] = unaff_EBX + 0x1bea; *(xunknown4 *)&pxVar5[-0x114c] = 0x147a; iVar4 = func_0x0000`
  - bug_token: `= func_0x000010d0\(\);`
  - target: 0x13ed (func_0x000013ed = main; Ghidra-rebased 0x113ed)

**GH-6882** (Sparc · S4 · hard) — stage-exposure
  - observed: `void p(void){ code *pcVar1; xunknown1 axStack_8 [8]; f(&stack0xfffffff8); /* WARNING: Does not return */ pcVar1 = (code *)IllegalInstructionTrap(8); (*pcVar1)(); }`
  - bug_token: `(*pcVar1)()`
  - bytes: `9de3bfa0f007a04081c7e00c81e800009de3bf98b007bff840000000f023a04000000008f007bff840000000f027bff4f007bff481c7e00891ee0008`
  - target: p @ 0x10 (in reconstructed image; calls patched to resolve f/r)
  - spec: specs/Ghidra/Processors/Sparc/data/languages/SparcV9.sinc;specs/Ghidra/Processors/Sparc/data/languages/SparcV9_32.cspec

**GH-6904** (PowerPC · S2 · hard) — spec-fix
  - observed: `void FUN_688(int4 *param_1){ xunknown4 *pxVar1; int8 in_r3; uint8 uVar2; uVar2 = in_r3 - 1; ... uVar2 = (uVar2 & 0xffffffff) >> 1; ... if ((int4)in_r3 == 2) ... }  -- the 32-bit param register r3 is modeled as int8 and the ctr loop counter as uint8 with spurious upper-word masking.`
  - bug_token: `int8 in_r3`
  - bytes: `3463ffff9421fff04182003c706900013943ffff408200385469f87e7d2903a68144000881240000394a0001914400088089000081490008394a0001914900084200ffe0382100104e800020812400082c0a00003929000191240008808400004082ffb84bffffe0`
  - target: some_ctr_func @ 0x688 (root bug); some_funcptr_func @ 0x5fc (issue target)
  - spec: specs/Ghidra/Processors/PowerPC/data/languages/ppc_common.sinc;specs/Ghidra/Processors/PowerPC/data/languages/ppc_32_e500_be.slaspec

**GH-7139** (x86 · S1 · medium) — spec-fix
  - observed: `0x00000000: PUSH      dword ptr ES:[BX + 0xf7fc]`
  - bug_token: `BX \+ 0xf7fc`
  - bytes: `6626ffb7fcf7`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-7332** (x86 · S6 · medium) — spec-fix
  - observed: `func_0x00401090(); if (&stack0x00000000 == (xunknown1 *)0x8) { func_0x00401050(0x406034); return 0xffffffff; } (*pcRam00405000)(&stack0xfffffff8,0x104); func_0x00401050(0x406030,&stack0xfffffff8);  -- and func_0x00401090 itself is the chkstk/_alloca_probe stub: 'for (; 0x1000 <= in_EAX; in_EAX = in_`
  - bug_token: `&stack0x00000000`
  - target: 0x401000 (func_0x00401000 = main); alloca stub at 0x401090
  - spec: specs/Ghidra/Processors/x86/data/languages/x86win.cspec;specs/Ghidra/Processors/x86/data/languages/x86.pspec

**GH-7377** (ARM · S2 · hard) — not-viable
  - not-viable: Requires a brand-new core p-code opcode (CPUI_INT_ROTATE) plus opbehavior/emulate/printC support and new simplification rules, then re-lifting rotate instructions across every SLEIGH spec (PowerPC rlwinm/rlwnm, ARM ror, x86 ROR, MIPS ROTR). Per the triage rule, a brand-new core p-code opcode is not-viable in the C++-only harness scope (it is an architecture-wide IR change, not an option-gated sub-stage decision nor a bounded spec edit).
  - observed: `int4 FUN_0(uint4 *param_1)\n{\n  return (int4)(*param_1 << 7 \| *param_1 >> 0x19) >> 0x1d;\n}`
  - bug_token: `<< 7 \\| \*param_1 >> 0x19`
  - bytes: `8063000054633830 7c63ee704e800020`
  - target: bitfield_get_b (ELF sym off 0xc; .text file off 0x40)

**GH-7418** (x86 · S2 · easy) — spec-fix
  - observed: `Disasm: '0x00010000: CALL 0x17004'. Raw pcode: '0x00010000:2: call ffunc_0x00017004:8(free)'. Target is the UNMASKED 0x17004 (inst_next 0x10004 + rel16 0x7000) instead of the SDM-mandated masked 0x7004.`
  - bug_token: `0x00017004`
  - bytes: `66e80070`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-7451** (RISC-V · S2 · medium) — spec-fix
  - observed: `0x0000000c: flw       fa5,-0xc(s0) Low-level ERROR: r0x00000010: Unable to resolve constructor`
  - bug_token: `Unable to resolve constructor`
  - bytes: `53071 4f0`
  - target: 0x10 (function 'test' .text offset 0x10; ET_REL object, non-PIE)
  - spec: specs/Ghidra/Processors/RISCV/data/languages/riscv.rv32f.sinc;specs/Ghidra/Processors/RISCV/data/languages/riscv.instr.sinc

**GH-7890** (ARM · S2 · hard) — spec-fix
  - observed: `xVar1 = VectorSignedToFloat(param_1,(uint1)(in_fpscr >> 0x16) & 3);  // raw pcode: u0x...= fpscr(i) >> #0x16 ; ... & #0x3:1 ; s0 = VectorSignedToFloat(s0(i),...)`
  - bug_token: `VectorSignedToFloat`
  - bytes: `c00ab8ee`
  - spec: specs/Ghidra/Processors/ARM/data/languages/ARMneon.sinc

**GH-8017** (x86 · S6 · hard) — stage-exposure
  - observed: `pxVar2 = &stack0xfffffffffffffff8; do { pxVar3 = pxVar2; *(xunknown8 *)&pxVar3[-0x1000] = *(xunknown8 *)&pxVar3[-0x1000]; pxVar2 = &pxVar3[-0x1000]; } while (&pxVar3[-0x1000] != axStack_6008); ... *(xunknown8 *)&pxVar3[-0x28] = 0x11b9; func_0x00001030(0x2004);`
  - bug_token: `&pxVar3\[-0x1000\]`
  - target: main @ 0x1149 (de-rebased; symbol resolved)

**GH-8028** (x86 · S1 · hard) — not-viable
  - not-viable: Two parts both outside the C++ harness: (1) the concrete defect 'undefined bytes (?? NNh) after the eb4a JMP because the inlined nested-function bodies are not picked up as code' is a Ghidra Java auto-analysis / CodeBrowser recursive-descent code-following + listing-rendering behavior; kuna's standalone disassemble does LINEAR decode and never produces undefined-byte listing tokens, and the decompiler follows flow through the whole function. (2) The headline ask ('support for nested functions' with shared parent stack frame) is a Java-side function-model enhancement. Neither is expressible in the C++-only decompiler core.
  - observed: `Linear disassemble 0x0..0x100 decodes ALL bytes as valid instructions: 0x18 JMP 0x64; 0x1a PUSH DI; 0x1b MOV CX,0xffff; ... 0x26 RET; 0x27 MOV byte ptr SS:[DI],AL; ... 0x63 RET; 0x64 PUSH ES ... -- NO '?? NNh' undefined bytes anywhere. print C recovers FUN_0(...) as one function following the JMP. g`

**GH-8391** (ARM · S2 · medium) — spec-fix
  - observed: `> Low-level ERROR: r0x00001000: Unable to resolve constructor / Aborting process   (under ARM:LE:32:v8-m). Under ARM:LE:32:v8 and v7 the SAME bytes decode: '0x00001000: mrrc2 p4,0x0,r0,r1,cr8'`
  - bug_token: `Unable to resolve constructor`
  - bytes: `51fc0804`
  - target: 0x1000177c (vaddr; file off 0x277c) / generic_save_state
  - spec: specs/Ghidra/Processors/ARM/data/languages/ARMTHUMBinstructions.sinc;specs/Ghidra/Processors/ARM/data/languages/ARM8m_le.slaspec

**GH-8456** (ARM · S2 · medium) — spec-fix
  - observed: `(empty disassembly) ; > Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `Unable to resolve constructor`
  - bytes: `000209e1`
  - spec: specs/Ghidra/Processors/ARM/data/languages/ARMinstructions.sinc

**GH-8467** (AARCH64 · S6 · medium) — stage-exposure
  - observed: `> Decompiling FUN_128d0 / Function ERROR: Unable to find unique hash for varnode / Aborting process. The function is the NEON byte-search 'indexbytebody' (dup v0.16B,w1; ld1 {v1.16B,v2.16B}; cmeq; addp; ...); partial C printed then decompile aborts.`
  - bug_token: `Unable to find unique hash for varnode`
  - bytes: `c20600b4eb0300aa258080d20502a8f2200c014e03e87b92a50c044e091040f24a1040920002005461a0df4c248100d1420004ab238c206e448c206e631c254e841c254e66bc244ec6bc264ec63c084e24f97fd3c624c49ac620c49a29020054c60200b561a0df4c428000f1238c206e448c206ea9000054661ca44ec6bce64ec63c084e06ffffb4631c254e841c254e66bc244ec6bc264ec63c084ee20000544401098b84104092848000d1e40704cbc620c49ac624c49a060100b4c600c0da638000d1c610c0da6004468b00000bcb000100f9c0035fd600008092000100f9c0035fd6000000000000000000000000e00740f9e20b40f9e1634039e8830091`
  - target: 0x128d0 (de-rebased: ET_EXEC non-PIE, use addr directly; = indexbytebody)

**GH-8500** (x86_64 · S3 · hard) — stage-exposure
  - observed: `/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffffe4 : 0x00001155 */\n/* WARNING: Restarted to delay deadcode elimination for space: stack */\n\nxunknown4 identity(void)\n{\n  xunknown4 xStack_1c;\n  return xStack_1c;\n}`
  - bug_token: `return xStack_1c;`
  - target: identity / 0x1139

**GH-8694** (x86_64 · P0 · medium) — not-viable
  - not-viable: The bug requires the Java front-end DWARF analyzer's 'Import Local Variable Info' option (which attaches DWARF-derived local-variable annotations to varnodes and triggers the DynamicHash maxduplicates collision). kuna has NO DWARF importer / no Java analyzer, so those annotations are never created and the uniqueHash error never fires -- the function decompiles cleanly.
  - observed: `uint8 func_0x01004240(int8 param_1){ ... full readable C body, ~hundreds of lines, ends with 'goto code_r0x01004268; }' } -- 0 occurrences of 'unique hash for varnode'; decompilation COMPLETES. (The 'error' grep hits are Zig type-name strings like error{Unexpected,...} embedded in the binary, not de`
  - target: 0x1004240 (string_menu.StringMenu.run; ET_EXEC, use addr directly)

**GH-8724** (x86_64 · S5 · hard) — stage-exposure
  - observed: `void test(int8 *param_1,int8 param_2)\n{ ... do { iVar2 = iVar2 + 1; iVar1 = *param_1 + iVar3; iVar3 = iVar3 + 0x414; func_0x000002d0(*(xunknown4 *)(iVar1 + 0x400 + (int8)*(int4 *)(iVar1 + 0x410) * 4)); } while (param_2 != iVar2); ... }`
  - bug_token: `iVar3 \+ 0x414`
  - target: test / 0x3c0

**GH-8777** (PIC · S5 · hard) — stage-exposure
  - observed: `for (iVar1 = 0; bVar2 = 0 <= iVar1 + -7,\n      (bVar2 \|\| !SBORROW2(iVar1,7)) && (!bVar2 \|\| SBORROW2(iVar1,7)); iVar1 = iVar1 + 1) { ... }  -- bra ge decodes correctly ('bra ge,0x10'); the GE condition lifts to (SRL_N \|\| !SRL_OV) && (!SRL_N \|\| SRL_OV) i.e. N==V.`
  - bug_token: `SBORROW2\(`
  - bytes: `020020007300200003065100040 03d000000200000048800610141 00faff37000000 0600`

**GH-8790** (x86 · S1 · easy) — spec-fix
  - observed: `> Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `Unable to resolve constructor`
  - bytes: `0f0dc0`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-8794** (x86 · S1 · hard) — not-viable
  - not-viable: Needs the whole AMD XOP instruction-set extension. grep -rn 'vphadddq\|XOP\|xop' over all x86 *.sinc/*.slaspec returns nothing -- there is no XOP support at all. XOP uses its own 0x8F escape-byte map (distinct from the 0xC4/0xC5 VEX maps) and is a large family (vphadddq and the rest of vphadd*, vpcmov, vpperm, vprot*, vpsh*, vpmac*, vfrczp*, etc. -- dozens of constructors across three XOP map selectors mmmmm=8/9/A). Adding it is a whole-extension-family build, so per the SLEIGH SCOPE RULE it is not-viable.
  - observed: `> Low-level ERROR: r0x00000000: Unable to resolve constructor / Aborting process`
  - bug_token: `Unable to resolve constructor`
  - bytes: `8fe878c2`
  - spec: specs/Ghidra/Processors/x86/data/languages/ia.sinc

**GH-8844** (AVR · S2 · medium) — spec-fix
  - observed: `disasm: 0x0000: rjmp -0xfd2 ; on decompile: 'Low-level ERROR: Bytes at 0xf817 are not mapped'. The rjmp at word 0 branches to word 0xf817 (byte 0x1f02e), the unwrapped out-of-range target, instead of wrapping at 4KiW to word 0x817 (byte 0x102e, FUN_code_0817).`
  - bug_token: `0xf817`
  - bytes: `16c8`
  - spec: specs/Ghidra/Processors/Atmel/data/languages/avr8.sinc

**GH-9001** (HCS12 · S1 · easy) — spec-fix
  - observed: `0x000000: BRN 0x2 ; 0x000001: JMP 0x2, +IX ; 0x000003: BGND ; 0x000004: BRN 0x6 ; 0x000005: ORAB 0x3da7 ; 0x000008: NOP ; 0x000009: RTS. BRN decodes as a 1-BYTE instruction (next CU at 0x1), so the rel8 operand byte (0x05) is not consumed and is mis-decoded as the start of the following instruction `
  - bug_token: `BRN       0x2`
  - bytes: `21052100021fa3da7a73d`
  - spec: specs/Ghidra/Processors/HCS12/data/languages/HCS_HC12.sinc

**GH-9184** (x86 · S2 · easy) — spec-fix
  - observed: `Decompiled C: 'xunknown1 axVar4 [16];' (accumulator 16 bytes, should be 32) and per-iteration 'axVar4 = vfmadd231ps_fma(ZEXT1632(axVar4), ...)' plus '*param_3 = ZEXT1632(axVar4)'. Raw pcode: vfmadd231ps_fma writes 'u0x00356c00:10' (size 0x10=16 bytes), then 'YMM0 = ZEXT1632(u...:10)' zero-extends 16`
  - bug_token: `ZEXT1632`
  - target: dot8_fma
  - spec: specs/Ghidra/Processors/x86/data/languages/fma.sinc

**GH-9191** (x86_64 · S2 · hard) — stage-exposure
  - observed: `void func_0x00401014(void)\n{\n  syscall();\n  /* WARNING: Could not recover jumptable at 0x00401030. Too many branches */\n  /* WARNING: Treating indirect jump as call */\n  (**(code **)(SUB168((ZEXT816(0) << 0x40 \| ZEXT816(0x27)) % ZEXT816(5),0) * 8 + 0x401089))();\n  return;\n}`
  - bug_token: `Could not recover jumptable at 0x00401030\. Too many branches`
  - target: 0x401014 (ET_EXEC, non-PIE, use target_addr directly)

**GH-9203** (riscv64 · S3 · hard) — stage-exposure
  - observed: `if (iVar1 == 0) {\n    iVar1 = 7;\n    do {\n      iVar1 = iVar1 + -1;\n      xVar2 = 0;\n    } while (iVar1 != 0);\n  }\n  else {\n    xVar2 = 0xffffffffffffffa1;\n  }\n  return xVar2;  -- the 'xVar2 = 0;' assignment is misplaced INSIDE the do..while loop body.`
  - bug_token: `do \{[^}]*= 0;[^}]*\} while`
  - bytes: `397106fc4af022f826f42e8411492400ef0030602a8909ed414585471d471d8f7dff4274a274e2704a85027921618280130910fafdb7`

**GH-9218** (68000 · S2 · hard) — stage-exposure
  - observed: `0x00000000: link.w A6,0x0 ... 0x0000001e: unlk A6 / 0x00000020: rts  ;  > Decompiling FUN_0 / Low-level ERROR: Cannot properly adjust input varnodes / Unable to proceed with function: FUN_0 / Aborting process  (no C output produced).`
  - bug_token: `Cannot properly adjust input varnodes`
  - bytes: `4e560000226e000a0c6e000100086f0e2069000410280001b015670265ea4e5e4e75`
  - target: FUN_0 @ 0x0

## Wave plan (Phase C)

Spec-fixes (20) edit disjoint vendored specs (no shared-table contention) — run as a
parallel lane, easy→hard. Stage-exposures (12) share the 5 kuna files — worktree
implement + serial replay. Three options are shared by issue pairs (one fix closes two
issues): `flagcompare` (1276+8777), `switchmodbound` (6674+9191), `stackprobeloop`
(6858+8017).

**Wave S1 — easy spec-fixes (disasm/pcode, single-constructor):**
9001 (HCS12 BRN), 8790 (x86 prefetch reg-form), 5897 (ARM cpy→mov), 7418 (x86 66-CALL mask),
9184 (x86 AVX YMM width), 1243 (8051 ADDC carry).

**Wave S2 — medium spec-fixes (grouped by file/arch):**
3723 (AArch64 UDF), 8456 (ARM MRS-banked), 8391 (ARM Thumb CDE), 1951+8844 (AVR8 branch wrap),
4788 (V850 Bcond), 6389 (8085 undoc), 7451 (RISC-V Zfa), 7139 (x86 16-bit disp), 5666 (x86 67-CALL),
766 (68000 abs-short).

**Wave S3 — hard spec-fixes:**
7890 (ARM VFP vcvt pseudo-op), 6904 (PPC e500 reg width), 7332 (x86 alloca/chkstk callfixup).

**Wave X1 — easy/medium stage-exposures:**
8467 (dynamichashmax, dynamic.cc 8→16), 1276+8777 (flagcompare, S5/S3 simplification family).

**Wave X2 — hard stage-exposures (attempt; honest defer if non-convergent):**
6674+9191 (switchmodbound), 9203 (condexeplace), 9218 (inputvarnodeadjust),
8500 (stackalias), 8724 (arraystride), 6882 (sparcstructret), 6858+8017 (stackprobeloop).

**ElementId registry (locked):** 4010 flagcompare · 4011 switchmodbound · 4012 stackprobeloop ·
4013 dynamichashmax · 4014 stackalias · 4015 arraystride · 4016 sparcstructret · 4017 condexeplace ·
4018 inputvarnodeadjust. Burned-on-defer (never recycled).

**Not-viable (13) — precise reasons recorded above:** 809 (Java const-ref analyzer), 1708 (x86 FMA4
catalog), 2033 (x86 decode catalog), 6333 (RH850 superset), 6342 (Xtensa FLIX VLIW), 6664 (x86
real-mode segmented addr), 6682 (x86 real-mode MZ overlay), 6723 (PLT-reloc not C-reproducible),
6836 (COFF loader), 7377 (new core INT_ROTATE opcode), 8028 (Java auto-analysis), 8694 (Java DWARF
import), 8794 (x86 XOP catalog). **Already-correct (1):** 3847 (FYL2X handled in vendored ia.sinc).
