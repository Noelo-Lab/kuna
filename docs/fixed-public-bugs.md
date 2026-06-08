# Fixed public bugs

Every public **Ghidra** decompiler issue that kuna has fixed, across all sessions. kuna is
a standalone extraction of Ghidra's C++ decompiler (+ its SLEIGH specs); each issue below
is an open/known bug in **Ghidra** (`https://github.com/NationalSecurityAgency/ghidra/issues/<N>`),
reproduced and fixed here. The companion `docs/issue-coverage.md` is the full triage ledger
(including not-viable / deferred); this file lists only the **fixes**.

Each fix is one of two kinds:

- **Bug-fix** — corrects plainly incorrect behavior. There is one right answer; kuna simply
  produces it. These are always-on (a SLEIGH spec correction, or a renderer fix). No option.
- **Decision point** — a scenario where *different choices are correct in different contexts*,
  so the fix is exposed as a durable, named option (`option <name> ...`) rather than hardcoded.
  The default is recorded; `option <name> <other>` always recovers the alternative. Defaults
  that were deliberately flipped away from upstream are tracked in `docs/divergences.md`
  (DIV-1/2/3); options left opt-in are destructive as a global default and correct only
  per-program.

Totals: **37 issues fixed** — 17 bug-fixes, 20 decision points.

---

## Bug-fixes (17)

Correct, always-on corrections of incorrect behavior. Sixteen are SLEIGH spec corrections
(decode/lift); one is a renderer fix. Each has a `tests/stages/gh<N>-*.xml` testcase and is
recorded in `UPSTREAM.md` *Divergence*.

| Ghidra # | Arch | Bug | Fix |
|---|---|---|---|
| [#2786](https://github.com/NationalSecurityAgency/ghidra/issues/2786) | x86 | Nested unary minus rendered as `--x`, which re-parses as pre-decrement (invalid/different C) | Renderer parenthesizes adjacent identical sign tokens → `-(-x)` (DIV-1) |
| [#9001](https://github.com/NationalSecurityAgency/ghidra/issues/9001) | HCS12 | `BRN` decoded as 1 byte; its `rel8` operand was mis-decoded as the next instruction | `:BRN rel8` consumes the operand (correct 2-byte no-op) |
| [#1243](https://github.com/NationalSecurityAgency/ghidra/issues/1243) | 8051 | `ADDC` truncated the carry-out, so 16-bit `ADD`+`ADDC` was not recognized | carry-in computed in a 2-byte temp so `CY` propagates |
| [#7418](https://github.com/NationalSecurityAgency/ghidra/issues/7418) | x86 | 32/16-bit `CALL/JMP rel16` left the high bytes of EIP set | mask the target to the low 16 bits (`tempEIP & 0xFFFF`) |
| [#7139](https://github.com/NationalSecurityAgency/ghidra/issues/7139) | x86 | 16-bit `modrm` `disp16` printed unsigned (sign bit ignored) | use the signed `simm16` token (display only) |
| [#8790](https://github.com/NationalSecurityAgency/ghidra/issues/8790) | x86 | Register-form prefetch/NOP opcodes `0f 0d`/`0f 18` (mod=3) failed to decode | add the mod=3 register-form constructors |
| [#9184](https://github.com/NationalSecurityAgency/ghidra/issues/9184) | x86 | YMM FMA used a 128-bit temp, truncating the upper half of every 256-bit result | widen the YMM FMA temp to `tmp:32` |
| [#5897](https://github.com/NationalSecurityAgency/ghidra/issues/5897) | ARM | Register-to-register move printed the deprecated pre-UAL `cpy` | emit the UAL mnemonic `mov` |
| [#8456](https://github.com/NationalSecurityAgency/ghidra/issues/8456) | ARM | `MRS` (Banked register) was unimplemented (undecodable) | add the banked-register `MRS` constructor |
| [#8391](https://github.com/NationalSecurityAgency/ghidra/issues/8391) | ARM | Cortex-M33 (v8-m) Thumb `mrrc`/`mcrr` coprocessor transfers failed to decode | un-guard them from `@ifndef CDE` (no CDE overlap) |
| [#7890](https://github.com/NationalSecurityAgency/ghidra/issues/7890) | ARM | Scalar VFP `vcvt` int↔float emitted an FPSCR-reading pseudo-op (unreadable output) | lift to native `int2float`/`float2int` |
| [#1951](https://github.com/NationalSecurityAgency/ghidra/issues/1951) | AVR8 | `rjmp`/`rcall` relative target not masked to the code space (wrong address) | mask `& 0x0FFF` (4 KiW wrap) on 16-bit AVR |
| [#8844](https://github.com/NationalSecurityAgency/ghidra/issues/8844) | AVR8 | `rjmp`/`rcall` did not wrap at the 4 KiW boundary on small AVRs | same `& 0x0FFF` rel-target mask (shares #1951's edit) |
| [#4788](https://github.com/NationalSecurityAgency/ghidra/issues/4788) | V850/RH850 | RH850 `Bcond disp17` conditional branch was undecodable (mis-decoded as `ld.hu`) | add the disp17 `Bcond` constructor |
| [#6389](https://github.com/NationalSecurityAgency/ghidra/issues/6389) | 8085 | Undocumented 8085 instructions were missing (failed to decode) | add DSUB/ARHL/RDEL/LDSI/SHLX/LHLX/RSTV/JNK/JK + K/V flags |
| [#7451](https://github.com/NationalSecurityAgency/ghidra/issues/7451) | RISC-V | Zfa-extension `fli.s` was unrecognized | add the `fli.s` constructor + `fli` pcodeop |
| [#6904](https://github.com/NationalSecurityAgency/ghidra/issues/6904) | PowerPC | e500 32-bit Book-E ops left junk in the upper word of the 64-bit GPR (spurious `CONCAT44`) | `mr`/`or`/`or.` zero-extend their 32-bit result (partial) |

---

## Decision points (20)

A named sub-stage decision exposed as `option <name> ...`. The fix is gated on a durable
Architecture flag (default off ⇒ byte-identical to upstream until deliberately flipped).
Each has a `tests/stages/gh<N>-*.xml` testcase asserting **both** directions of the choice.
Default column: `DIV-2`/`DIV-3` = flipped on by default (corpus-clean, see `docs/divergences.md`);
`opt-in` = left default-off (destructive as a global default; correct only per-program).

| Ghidra # | Arch | Bug / scenario | Option (default) | Decision |
|---|---|---|---|---|
| [#558](https://github.com/NationalSecurityAgency/ghidra/issues/558) | x86-64 | Comparisons rewritten to equivalent expressions (`x <= 8` → `x < 9`); pointer arithmetic vs array notation | `compareform` (DIV-2: original) · `arraynotation` (DIV-2: on) | keep the source comparison form / render `&base[i]` |
| [#1282](https://github.com/NationalSecurityAgency/ghidra/issues/1282) | 8051 | Flag-as-high-bit lowering leaked `(b<<7) s>> 7` mask noise | `booleanmask` (DIV-2: on) | fold the sign-extension-mask idiom |
| [#7190](https://github.com/NationalSecurityAgency/ghidra/issues/7190) | V850 | Signed compare leaked OV-flag arithmetic into the condition | `ovlesssimplify` (DIV-2: on) | collapse the OV-flag idiom to a clean signed `<` |
| [#8913](https://github.com/NationalSecurityAgency/ghidra/issues/8913) | x86 (6502) | 8-bit carry-chain wide adds rendered as raw `CARRY`/`CONCAT` intrinsics | `addcarrychain` (DIV-2: on) | recombine into a single wide add |
| [#8471](https://github.com/NationalSecurityAgency/ghidra/issues/8471) | ARM | Thumb function pointer (`fn|1`) rendered as a raw hex literal | `thumbfuncptr` (DIV-2: on) | keep the symbolic `&fn[1]` |
| [#6930](https://github.com/NationalSecurityAgency/ghidra/issues/6930) | x86-64 | A constant equal to a function entry at a single-bit image base rendered as `0x100000` | `inferfuncentry` (DIV-2: on) | infer the function pointer |
| [#9230](https://github.com/NationalSecurityAgency/ghidra/issues/9230) | x86-64 | Unrolled/SIMD constant-fill stores not recovered as `memset`/`bzero` | `memsetrecover` (DIV-2: on) | collapse the constant-fill run into `builtin_memset` |
| [#1537](https://github.com/NationalSecurityAgency/ghidra/issues/1537) | x86 | Compiler XMM-register `memset` on a global badly recognized | `memsetrecover` (DIV-2: on) | same sub-stage as #9230 — pure routing, zero new code |
| [#1276](https://github.com/NationalSecurityAgency/ghidra/issues/1276) | 8051 | Flag/boolean shifted into the sign bit then sign-tested: `(b<<7) s< 0` noise | `flagcompare` (DIV-3: on) | fold to a clean compare |
| [#8777](https://github.com/NationalSecurityAgency/ghidra/issues/8777) | PIC24 | `bra ge` lowered to the explicit `N==V` signed-overflow (`SBORROW`) idiom | `flagcompare` (DIV-3: on) | fold the `N==V` idiom to a signed compare (shares #1276) |
| [#8017](https://github.com/NationalSecurityAgency/ghidra/issues/8017) | x86 | gcc `-fstack-check` stack-probing loop broke stack-pointer tracking | `stackprobeloop` (DIV-3: on) | resolve the probe-loop stack-pointer MULTIEQUAL |
| [#8467](https://github.com/NationalSecurityAgency/ghidra/issues/8467) | AArch64 | Dense unrolled NEON code aborted with "Unable to find unique hash for varnode" | `dynamichashmax` (DIV-3: on) | raise the DynamicHash collision budget 8→16 |
| [#8724](https://github.com/NationalSecurityAgency/ghidra/issues/8724) | x86-64 | Compiler-lowered `sizeof(*p)*i` offset not recovered as an index | `arraystride` (DIV-3: on) | recover the strided-induction array index |
| [#9203](https://github.com/NationalSecurityAgency/ghidra/issues/9203) | RISC-V | A late conditional-const rule placed a COPY into the wrong (loop) block → malformed `do/while` | `condexeplace` (DIV-3: on) | keep the const-COPY out of loop blocks |
| [#9218](https://github.com/NationalSecurityAgency/ghidra/issues/9218) | 68000 | `link`/`unlk` frame aborted with "Cannot properly adjust input varnodes" | `inputvarnodeadjust` (DIV-3: on) | absorb overlapping input varnodes instead of aborting |
| [#6990](https://github.com/NationalSecurityAgency/ghidra/issues/6990) | SPARC | 32-bit void function over-claimed `o0:o1` as a 64-bit `undefined8` return | `returnpair` (opt-in: pair) | drop the passively-active second return register |
| [#8817](https://github.com/NationalSecurityAgency/ghidra/issues/8817) | V850 | `jmp [reg]` switch dispatch classified as a computed call (switch not recovered) | `v850indirectbranch` (opt-in: off) | reclassify CALLIND→BRANCHIND so switch recovery runs |
| [#9191](https://github.com/NationalSecurityAgency/ghidra/issues/9191) | x86-64 | Jump-table recovery gave up ("too many branches") — index bound ignored | `switchmodbound` (opt-in: off) | bound the LOAD-table by its modulo/and-mask index |
| [#8500](https://github.com/NationalSecurityAgency/ghidra/issues/8500) | x86-64 | A store through a stack-pointer alias was dropped (double-deref miscompiled) | `stackalias` (opt-in: off) | preserve the store across the deadcode race |
| [#6882](https://github.com/NationalSecurityAgency/ghidra/issues/6882) | SPARC | Struct-return post-call `unimp` treated as a terminator → bogus `(*pcVar1)()` | `sparcstructret` (opt-in: off) | treat the post-call `unimp` as fall-through |

---

## Why some decision points are opt-in (destructive as a global default)

Five decision points are deliberately left default-off because their trigger shape also
matches legitimate non-bug code on other targets — they are correct **per program**, not
globally (the ablation in `docs/divergences.md` / `docs/stage-critique.md` confirms each):

- `returnpair single` — would truncate genuine multi-register wide returns.
- `v850indirectbranch on` — its `jmp [reg]` predicate matches every register-indirect call
  on non-V850 architectures.
- `switchmodbound on` — may over-bound an unrelated register-indirect jump.
- `stackalias on` — relaxes a dead-store race that is genuinely dead elsewhere.
- `sparcstructret on` — reclassifies a real trap on non-SPARC targets.

See `docs/issue-coverage.md` for the per-commit testcase/commit references and the
not-viable / deferred issues this file omits.
