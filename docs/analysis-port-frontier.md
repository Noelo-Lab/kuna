# Analysis-port frontier — Docker-enabled (post-Increment-17)

## Context

The ELF-feasible analyzer frontier is **complete** (Increments 1–17 in
`docs/analysis-port-log.md`; the completeness sweep is Increment 15). Every feasible-at-tier,
decompiler-relevant ELF analyzer has been ported onto kuna's current analysis tier
(`kuna-analysis`). This document tracks the **remaining** work under the chosen
**"Feasible + Docker-unblock"** scope:

- **Port** everything still portable onto kuna's current analysis tier.
- Use the **devcontainer** (this PR) to build cross-arch ELF fixtures that unblock the passes
  that were stuck only because the build host lacked cross linkers (the documented ARM decode
  e2e being the headline).
- Close the **cosmetic `_INIT_<i>` / `_FINI_<i>`** array-element naming.
- **Document (do not build)** the items that need a brand-new engine subsystem — a
  post-disassembly Listing / ReferenceManager tier that does not yet exist — or non-ELF
  loaders (PE / Mach-O / COFF) and the large recognizer subsystems (Go pclntab, FID).

One PR per item.

## Work-list

Ordered easy → hard.

| # | Item | Type | Depends-on | Status | Notes |
|---|------|------|-----------|--------|-------|
| 1 | devcontainer + cross-arch toolchains | infra | — | **done (THIS PR)** | The reproducible env + ARM/aarch64/riscv/mips/ppc/sparc linkers. See `docs/devcontainer.md`. Unblocks #2–#5. |
| 2 | ARM decode e2e | port/test | #1 | todo | Linked Thumb fixture via the container ARM linker; the deferred Increment-8/17 e2e (paint TMode + decode a Thumb function end-to-end). |
| 3 | AArch64 PLT/markers linked-exe e2e | test | #1 | todo | `elf_plt` already supports AArch64; add a LINKED fixture + e2e proving import names resolve. |
| 4 | RISC-V64 PLT linked-exe e2e | test | #1 | todo | `elf_plt` supports RISC-V; add a LINKED fixture + e2e. |
| 5 | MIPS16 ISA_MODE context paint | port | #1 | todo | The MIPS analog of ARM `$t` (`MIPS_ElfExtension.applyIsaMode`, a context bit); needs a MIPS16 fixture (container mips gcc). |
| 6 | `_INIT_<i>` / `_FINI_<i>` array-element naming | port | — | todo | Reshape `AnalysisOutput.entries` to carry optional names + commit seam; host-testable; documented follow-up from Increment 15. |
| 7 | Build-plan doc: infeasible-at-tier + non-ELF + huge subsystems | doc | — | todo | Concrete build plans (NOT implementations) for: AIF / discovered-no-return / operand-reference markup (all need a post-disassembly Listing / ReferenceManager tier that does not exist), non-ELF loaders (PE / Mach-O / COFF), Go pclntab name recovery, FID fingerprinting. |

## Problems / notes log

- **2026-06-23 — Host lacked cross linkers.** The original build host had **no ARM linker**
  (and no aarch64 / riscv linkers either): `which arm-linux-gnueabihf-gcc` was empty,
  `make`/`ld.lld` could assemble objects but not produce a *linked* cross-arch ELF. This is
  exactly what blocked the documented ARM decode e2e (Increment 8/17): the `s1_loader`
  `arm_markers` pass and its `.o` unit test were done, but the full decode e2e needs a
  **linked** ARM fixture. **Now provided by the container** (#1): `ubuntu:22.04` with
  `gcc-arm-linux-gnueabihf` + every other cross toolchain *and* its `binutils` (so the cross
  `ld`s are present). Proof captured in the PR: the container links a `PT_LOAD`, `Type: EXEC`,
  `Machine: ARM` Thumb executable (odd entry `0x100d1` = Thumb bit). This unblocks #2–#5.
