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

## Remaining this session (46) — ledger

Bucket/stage/difficulty below are the **pre-triage prediction**; Phase B replaces them
with live evidence (status `triaged`). `EID`/`option`/`testcase`/`commit` are filled at
assign/commit time.

| GH | Arch | Method | Bytes | Pred. bucket | Pred. family | Diff | EID | Option | Testcase | Status | Commit |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 766 | 68000 | disasm | raw_hex | spec-fix | m68k-address-mask | med | — | — | — | REMAINING | — |
| 809 | MIPS | pcode | raw_hex | not-viable | mips-ref-analyzer | med | — | — | — | REMAINING | — |
| 1243 | 8051 | pcode | raw_hex | spec-fix | 8051-carry-semantics | med | — | — | — | REMAINING | — |
| 1276 | 8051 | C | raw_hex | spec-fix(+stage) | volatile-sfr | med | — | — | — | REMAINING | — |
| 1708 | x86 | disasm | raw_hex | not-viable | x86-avx-catalog (FMA4) | hard | — | — | — | REMAINING | — |
| 1951 | AVR | pcode | raw_hex | spec-fix | avr-branch-mask | easy | — | — | — | REMAINING | — |
| 2033 | x86 | disasm | raw_hex | not-viable | x86-disasm-catalog | hard | — | — | — | REMAINING | — |
| 3723 | AARCH64 | C | raw_hex | spec-fix | missing-trap-encoding | easy | — | — | — | REMAINING | — |
| 3847 | x86 | pcode | raw_hex | spec-fix | x87-transcendental | med | — | — | — | REMAINING | — |
| 4788 | V850 | disasm | raw_hex | spec-fix | rh850-missing-instr | easy | — | — | — | REMAINING | — |
| 5666 | x86 | pcode | raw_hex | spec-fix | x86-prefix-semantics | med | — | — | — | REMAINING | — |
| 5897 | ARM | disasm | raw_hex | spec-fix | arm-mnemonic-ual | easy | — | — | — | REMAINING | — |
| 6333 | V850 | disasm | raw_hex | spec-fix | rh850-missing-instr | med | — | — | — | REMAINING | — |
| 6342 | Xtensa | disasm | raw_hex | not-viable | vliw-bundle (FLIX) | hard | — | — | — | REMAINING | — |
| 6389 | 8085 | disasm | raw_hex | spec-fix | 8085-missing-instr | med | — | — | — | REMAINING | — |
| 6664 | x86 | C | binary | not-viable | x86-segmented-realmode | hard | — | — | — | REMAINING | — |
| 6674 | V850 | C | binary | stage-exposure | switch-recovery | hard | — | — | — | REMAINING | — |
| 6682 | x86 | C | binary | not-viable | payload-limit/realmode | hard | — | — | — | REMAINING | — |
| 6723 | x86_64 | C | binary | stage-exposure | param-recovery-dataflow | hard | — | — | — | REMAINING | — |
| 6836 | x86 | C | binary | not-viable | coff-loader | hard | — | — | — | REMAINING | — |
| 6858 | x86 | C | binary | stage-exposure | stack-spill-cleanup | hard | — | — | — | REMAINING | — |
| 6882 | Sparc | C | binary | stage-exposure(+spec) | sparc-struct-return | hard | — | — | — | REMAINING | — |
| 6904 | PowerPC | C | binary | spec-fix | ppc-bookE-regsize | hard | — | — | — | REMAINING | — |
| 7139 | x86 | disasm | raw_hex | spec-fix | x86-disp-sign | easy | — | — | — | REMAINING | — |
| 7332 | x86 | C | binary | stage-exposure | alloca-stub/callfixup | hard | — | — | — | REMAINING | — |
| 7377 | ARM | C | binary | not-viable | new-pcode-op (INT_ROTATE) | hard | — | — | — | REMAINING | — |
| 7418 | x86 | pcode | raw_hex | spec-fix | x86-prefix-semantics | med | — | — | — | REMAINING | — |
| 7451 | RISC-V | disasm | binary | spec-fix | riscv-zfa-missing | med | — | — | — | REMAINING | — |
| 7890 | ARM | pcode | raw_hex | spec-fix | arm-vfp-semantics | med | — | — | — | REMAINING | — |
| 8017 | x86 | C | binary | stage-exposure | stack-clash-probe | hard | — | — | — | REMAINING | — |
| 8028 | x86 | disasm | raw_hex | not-viable | nested-functions | hard | — | — | — | REMAINING | — |
| 8391 | ARM | disasm | binary | spec-fix | arm-thumb-coproc | med | — | — | — | REMAINING | — |
| 8456 | ARM | C | raw_hex | spec-fix | arm-missing-instr | easy | — | — | — | REMAINING | — |
| 8467 | AARCH64 | C | binary | stage-exposure | dynamic-hash-maxdup | med | — | — | — | REMAINING | — |
| 8500 | x86_64 | C | binary | stage-exposure | alias-analysis | hard | — | — | — | REMAINING | — |
| 8694 | x86_64 | C | binary | not-viable | dynamic-hash + DWARF(Java) | med | — | — | — | REMAINING | — |
| 8724 | x86_64 | C | binary | stage-exposure | array-index-recovery | hard | — | — | — | REMAINING | — |
| 8777 | PIC | C | raw_hex | stage-exposure | signed-flag-compare | med | — | — | — | REMAINING | — |
| 8790 | x86 | disasm | raw_hex | spec-fix | x86-modrm-regform | easy | — | — | — | REMAINING | — |
| 8794 | x86 | disasm | raw_hex | not-viable | x86-xop-catalog | hard | — | — | — | REMAINING | — |
| 8844 | AVR | pcode | raw_hex | spec-fix | avr-branch-mask | easy | — | — | — | REMAINING | — |
| 9001 | HCS12 | disasm | raw_hex | spec-fix | hcs12-operand-consume | easy | — | — | — | REMAINING | — |
| 9184 | x86 | C | binary | spec-fix | x86-vector-semantics (FMA YMM) | med | — | — | — | REMAINING | — |
| 9191 | x86 | C | binary | stage-exposure | switch-recovery | hard | — | — | — | REMAINING | — |
| 9203 | riscv64 | C | raw_hex | stage-exposure | block-placement-rule | hard | — | — | — | REMAINING | — |
| 9218 | 68000 | C | binary | stage-exposure | input-varnode-storage | hard | — | — | — | REMAINING | — |

## Wave plan (filled in Phase C after triage)

_TBD — see plan Phase C._
