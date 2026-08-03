---
case_id: O2-noinline-crazyflie-cf2-dwHandleInterrupt
pool: novel
group_id: crazyflie::dwHandleInterrupt (cluster record — 8 cases, 5 projects, 3 arches)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: the `(code *)` wart is THREE unrelated causes, not one. 80% of kuna's excess over ghidra is ARM/Thumb only and comes from ONE dead accessor — `RuleFuncPtrEncoding` reads a hard-coded `funcptr_align() = 0` stub, so the SLEIGH `BXWritePC` mask `& 0xfffffffe` survives to the C and blocks the `code *` back-propagation; kuna emits 4,673 masks corpus-wide vs ghidra's 579 and ZERO `(code **)` on any ARM binary. A second, smaller cause is the default-ON `inferfuncentry` mis-firing on integer constants that collide with a function entry (wrong code, closed by an existing flag). The third and largest by raw COUNT is ghidra-identical untyped-callback rendering and is not a kuna defect at all.
option_closing: inferfuncentry off (for the second cause only; nothing closes the ARM cause — all 84 non-default option settings swept)
feature_slug: funcptralign
scope: small
confidence: high
---

## What was actually verified

Eight NOVEL rows whose `defects` column is driven by `code_ptr`, re-decompiled today
(`decompiler/target/release/kuna`, `SLEIGHHOME=/home/mahaloz/github/kuna/specs`), each
against the stripped binary at the case address, plus the ghidra/ida panes and the original
`.i` source.

| case | arch | kuna `(code *)` | ghidra `(code *)` | cause | verdict |
|---|---|---|---|---|---|
| `O2-noinline-crazyflie-cf2-dwHandleInterrupt` | ARM Cortex-M4 | 6 | 3 | **thumb-mask** | feature-candidate |
| `O2-noinline-libopencm3-msc-msc_data_rx_cb` | ARM Cortex-M | 6 | 3 | **thumb-mask** | feature-candidate |
| `O2-noinline-nuttx-nuttx-med3` | ARM Cortex-M4 | 5 | **0** | **thumb-mask** | feature-candidate |
| `O2-noinline-coreutils-head-copy_fd` | x86-64 PIE | 1 | 0 | **inferfuncentry** | covered-by-option |
| `O2-noinline-x0r-usb-x0r-usb-IRC_Login` | i386 PE | 11 | 11 | untyped callback | not-a-defect |
| `O2-noinline-e2fsprogs-e2fsck-e2fsck_pass3` | x86-64 | 3 | 6 | untyped callback | not-a-defect |
| `O2-noinline-libacl-libacl.so.1.1-set_acl_fd` | x86-64 | 12 | 19 | untyped callback | not-a-defect (already recorded) |
| `O2-noinline-tar-tar-verify_volume` | x86-64 | **0** | 5 | — | already-fixed (DIV-34/36) |

Four of the eight top-ranked `code_ptr` rows are rows where kuna emits **fewer or equal**
`(code *)` than the co-best rival. The wart counter is measuring an inherited Ghidra
rendering convention, not a kuna defect (see *Metric note*).

---

## Cause 1 (dominant) — the ARM/Thumb ISA-mode mask: `funcptr_align` is a dead stub

### The pane

`crazyflie::dwHandleInterrupt` @ 0x804001c, fresh, today's default (`--mode auto`):

```c
  if ((sub_803f1b4(a0)) && (*(uint4 *)(a0 + 0x44)))
    (*(code *)(*(uint4 *)(a0 + 0x44) & 0xfffffffe))(a0);
  if ((sub_803f138(a0)) && (*(int4 *)(a0 + 0x40))) {
    sub_803f238(a0);
    (*(code *)(*(uint4 *)(a0 + 0x40) & 0xfffffffe))(a0);
  }
```

ghidra, same function:

```c
  if ((iVar2 != 0) && (*(code **)(param_1 + 0x44) != (code *)0x0)) {
    (**(code **)(param_1 + 0x44))(param_1);
  }
```

Source — `crazyflie/O2-noinline/compiled/libdw1000.i`, `dwHandleInterrupt`:

```c
void dwHandleInterrupt(dwDevice_t *dev) {
 dwReadSystemEventStatusRegister(dev);
 if(dwIsClockProblem(dev) && dev->handleError != 0) {
  (*dev->handleError)(dev);
 }
 if(dwIsTransmitDone(dev) && dev->handleSent != 0) {
  dwClearTransmitStatus(dev);
  (*dev->handleSent)(dev);
 }
 ...
```

The fields are `dwHandler_t` function pointers. The source performs **no** bit-clear. So
kuna's C asserts the program computes `handler & ~1` before every callback — bookkeeping the
machine does for ISA mode, not program semantics — and, because the mask is in the way, the
field never acquires a function-pointer type, so the load stays `*(uint4 *)` and every call
site needs an extra `(code *)` cast that ghidra does not need.

`libopencm3::msc_data_rx_cb` and `nuttx::med3` are the same shape.  `med3` is the sharpest
version: the source parameter is genuinely `int (*cmp)(...)`, ida prints
`int (*a4)(void)`, ghidra prints `code *param_4`, kuna prints `uint4 a3` and then
`(*(code *)(a3 & 0xfffffffe))(a0,a2)` five times.

### Root cause, by instrumentation

**Step 1 — what the lifter emits.** `specs/Ghidra/Processors/ARM/data/languages/ARM.sinc:221`

```
macro BXWritePC(addr) {
   SetThumbMode((addr & 0x1) != 0);
   local tmp = addr & 0xfffffffe;
   pc = tmp;
}
```
`ARMTHUMBinstructions.sinc:1576` — `:blx Hrm0305 { BXWritePC(Hrm0305); lr = inst_next|1; call [pc]; }`.
So **every** Thumb indirect call lowers to `AND target, 0xfffffffe` → `CALLIND`.
PR #231 erased the first line of that macro (the `setISAMode` CALLOTHER); this is the
second line, and #231 did not touch it.

`decomp_dbg` on `msc.elf` @ 0x8001000, `print raw`:

```
0x080010b0:14e:  r3(0x080010b0:14e) = *(ram,u0x00150200(0x080010b0:54f))
0x080010b6:15d:  u0x1000012a(0x080010b6:15d) = r3(0x080010b0:14e) & #0xfffffffe
0x080010b6:550:  pc(0x080010b6:550) = (cast)(u0x1000012a(0x080010b6:15d))
0x080010b6:15f:  callind pc(0x080010b6:550)
```

The AND (op `15d`, an original lifted op) is the sole producer of the CALLIND target; the
`(cast)` at op `550` is late-allocated (P9 `ActionSetCasts`, same op-number band as the
`(cast)` inserted before every LOAD in the same dump).

**Step 2 — the rule that is supposed to erase it exists and is fully ported.**
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs`, `RuleFuncPtrEncoding`
(registered as `funcptrencoding` at `infra/universalaction.rs:515`) transcribes upstream
`ruleaction.cc:9929` line for line: match `CALLIND`, require `in(0)` written by
`CPUI_INT_AND` with a constant mask, accept when `testmask & (!0 << align) == val`,
then `op_remove_input` + rewrite the AND to `CPUI_COPY`. For ARM (`align == 1`, size 4)
that test is `0xFFFFFFFF & 0xFFFFFFFE == 0xfffffffe` — an exact match.

**Step 3 — its only input is hard-coded to zero.** `ruleaction_8.rs:251-257`:

```rust
/// `data.getArch()->funcptr_align` (C++).  // STUB(W4)
///
/// The W4 `Architecture` skeleton does not carry `funcptr_align`; it defaults to
/// 0 ("encoding disabled"), so `RuleFuncPtrEncoding` no-ops.  Recorded as a loss.
fn funcptr_align(_data: &Funcdata) -> int4 {
    0
}
```

and the rule's first two lines are `let align = funcptr_align(data); if align == 0 { return 0; }`.
**The rule can never fire on any architecture.**

**Step 4 — the stub's premise is stale; `funcptr_align` is live.** The comment says the
`Architecture` skeleton does not carry the field. It does:
`infra/architecture.rs:244` declares it, `:2105 decode_funcptr_align()` parses
`<funcptr align="2"/>` (present in `ARM.cspec:33`, `ARM_v45`, `ARM_win`, `ARM_apcs`,
AARCH64, MIPS, Loongarch) into the bit position, `:3902` calls it during
`parse_compiler_config`, and `:1869` copies it into the ArchSeam that `Funcdata`
reads (`substrate/context.rs:797`). Two other consumers already read it live —
`p2_lift/jumptable.rs:3661` and `p9_emit/coreaction_render.rs:1190`.

Proved live at runtime rather than by reading: `tests/stages/gh8471-thumbfuncptr.xml`
runs on `ARM:LE:32:v8:default` and its assertion #2 (`call_function(&fn[1])`, the
default/`thumbfuncptr on` pass) is reachable only through
`RulePtrsubUndo::preserve_thumb_funcptr`, which short-circuits on
`funcptr_align == 0` (`p3_dataflow/ruleaction_6.rs:612`). It passes today:

```
$ decomp_test_dbg -usesleighenv -path tests/stages datatests gh8471-thumbfuncptr.xml
Success -- GH8471 #1: thumbfuncptr off restores the raw-hex form (old behavior, opt-in)
Success -- GH8471 #2: default pipeline preserves the symbolic form, &fn[1] via DIV-2 arraynotation
Total tests applied = 2 / Total passing tests = 2
```

So the architecture-side plumbing is complete and only the rule's own accessor is dead.

**Step 5 — the consequence on the type, measured.** With the AND out of the way, upstream's
`TypeOpCallind` types `in(0)` as pointer-to-code and `TypeOpLoad` back-propagates it onto
the LOAD pointer, giving ghidra's `*(code **)(p)`. Corpus evidence that kuna's machinery
does exactly this whenever no mask intervenes:

| | kuna `(code **)` | note |
|---|---|---|
| x86 / x86-64 projects | 2,914 (openssh 902, x0r-usb 663, zlib 369, bash 230, coreutils 181, …) | no mask on the target → type propagates |
| **every ARM project** (crazyflie, betaflight, cleanflight, libopencm3, nuttx, chibios, riot-os, freertos, u-boot) | **0** | mask on every target |

Fresh `decompile-all` on today's build confirms: `msc.elf` 108 fns → 50 `(code *)`,
**0** `(code **)`, 39 masked indirect calls; `cf2.elf` 2,914 fns → 384 `(code *)`,
**0** `(code **)`, 363 masked indirect calls in 225 functions.

### Option sweep — nothing covers it

All 84 non-default settings of the 83-row catalog were run on
`cf2.elf --addr 0x804001c`, counting `0xfffffffe`. Every one returned **6** — the baseline.
`--mode aggressive` = 6, `--mode fast` = 0 and `--mode reliable` = 0 only because both
degrade this Cortex-M function to an empty `void sub_804001c(void) { return; }` (a separate
recall issue, not a fix). `condfold on|wide` swept explicitly: 6.

### Breadth

Stored kuna artifacts (regenerated 2026-08-01 23:35, verified identical to today's build on
all four cases re-run here):

- `& 0xfffffffe` in emitted C: **kuna 4,673 vs ghidra 579** corpus-wide. The excess is
  entirely ARM: crazyflie 1808/38, betaflight 927/0, libopencm3 637/131, cleanflight 564/24,
  nuttx 288/0, chibios 64/…; the x86 projects are at parity (openssh 121/131, bash 74/72,
  iproute2 79/73, coreutils 23/25 — those are real source-level `& ~1`).
- Thumb-masked indirect calls: **4,027 occurrences in 2,496 function-instances across 97
  binary-instances** in 9 projects.
- Of the 3,804 `(code *)` occurrences where kuna exceeds ghidra on the same function,
  **3,033 (80%) carry the Thumb mask**.

### Owning phase & mechanism

**P3 — definition web** (`p3_dataflow/`, rule pool). One module, one function body:

- `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs` — replace the
  `funcptr_align(_data) -> 0` stub with the live read (`data.get_arch().funcptr_align`,
  the same accessor `p2_lift/jumptable.rs:3661` already uses), delete the two `STUB(W4)`
  comment blocks at `:71-72`, `:251-257` and the `// -- STUB(W4): funcptr_align defaults to 0`
  line inside `apply_op`, and update the unit test at `:2163-2169` that currently asserts
  the rule no-ops *because* of the stub (plus the note at `:2511`).
- `docs/spec/03-ssa-and-simplification.md` (which already lists `RuleFuncPtrEncoding` as
  ported — the prose is currently wrong) and `docs/spec/02-lift-and-flow.md` §ISA-mode,
  the natural companion to #231's paragraph.

**Gating.** This is a strict bug fix in the same sense as #231: the *processor cspec*, not
kuna, declares `<funcptr align="2"/>`, upstream Ghidra honours it, and the emitted C
currently states an operation the program does not perform. No new option, no DIV row (it
removes a divergence). Blast radius is bounded by the cspec: x86/x86-64 declare no
`<funcptr>` (`funcptr_align == 0`), so the rule stays inert there and the whole datatest
corpus is untouched except for its ARM/MIPS/AARCH64 members.

**Risks.**
1. Broad ARM text churn — ~4,000 masks disappear and ~3,000 `(code *)` casts turn into
   `(code **)` loads. `docs/baseline.json` / `baseline-stages.json` assertions that
   string-match a masked ARM call will move; inspect each, do not re-pin wholesale.
2. The rule rewrites the AND to a `COPY`, so a masked value that is *also* used as an
   integer elsewhere keeps its value (COPY is transparent) — but check that
   `RulePtrsubUndo::preserve_thumb_funcptr` (GH-8471/DIV-2) still sees the shape it
   expects; the two features touch the same low bit from opposite ends, and
   `tests/stages/gh8471-thumbfuncptr.xml` is the guard.
3. MIPS/AARCH64/Loongarch/8051 also declare `<funcptr>`; the rule becomes live there too.
   Corpus has no such target, so measure separately before claiming coverage.

**Stage testcase.** `tests/stages/ghdec-funcptralign.xml` is straightforward on the ARM
Thumb fixture #231 already vendored (`kuna-analysis/tests/fixtures/arm_thumb_switch_le32`)
or a 3-instruction `ldr r3,[r0,#4]; blx r3` chunk: two-pass, mask present vs absent.
Note the corpus-file-count bump in `kuna-base/src/xml.rs` + stages baseline re-record.

---

## Cause 2 (small, but WRONG CODE) — `inferfuncentry` collisions

`coreutils::copy_fd` @ 0x3290, fresh, today's default:

```c
unsigned long copy_fd(unsigned int a0,code *a1) // return-dupe
{
  code *v2;
  ...
  while( true ) {
    v2 = _DT_INIT;                 // <-- the constant 0x2000
    if (a1 <= (code *)0x2000)
      v2 = a1;
    v1 = sub_6e00(a0,v3,v2);
    ...
    a1 = &a1[-v1];                 // <-- nbytes -= n_read
```

Source is `copy_fd (int src_fd, uintmax_t n_bytes)` with `BUFSIZ`-style chunking; ida prints
`size_t nbytes`, ghidra `ulong param_2` and the plain literal `0x2000`. kuna types a
`size_t` as `code *` and prints the loop chunk size as a **function name**.

Closed by an existing flag:

```
$ kuna decompile head --addr 0x3290 --option inferfuncentry off
unsigned long sub_3290(unsigned int a0,uint8 a1)
    v2 = 0x2000;
    if (a1 <= 0x2000)
      v2 = a1;
    a1 -= v1;
```

The worst instance found is `gnutls ocsptool::cipher_to_flags` @ 0xdae3 (O0):

```c
code * sub_dae3(char *a0)          // default
{
  if (!a0) return (code *)0x20;
  ...
  return (code *)0x4000;
  return gnutls_ocsp_status_request_is_checked;   // <-- this is the integer 0x8000
  return (code *)0x1;
```

`gnutls_ocsp_status_request_is_checked` lives at **0x8000**, and one of the cipher flag
constants *is* 0x8000. One accidental collision retypes the whole return HighVariable
`code *`, so all 13 returns take a cast and the C returns a code address where the source
returns a bit flag. `--option inferfuncentry off` restores `unsigned long` and every
literal.

**Mechanism.** `inferfuncentry` (P5 / `const-pointer`, default **on**, kuna's own, GH-6930,
`p5_types/kuna_inferfuncentry.rs`) exists to skip `ActionConstantPtr::isPointer`'s
`bit_transitions < 3` rejection when the constant resolves exactly to a function entry —
written for a real function at a power-of-two *image base* (`0x100000`). In a stripped PIE
ELF the image base is 0, kuna's analysis tier synthesises `_DT_INIT`/`_DT_FINI` function
symbols from the dynamic tags, and ordinary `.text` entries sit at low addresses — so
0x2000, 0x8000, 0x1000 are all "exact function entries" and every buffer size and bit flag
in the binary is a candidate.

**Breadth.** `_DT_INIT`/`_DT_FINI` leaks into 45 functions / 100 occurrences (coreutils 92,
openssh 5, libacl 2, gnutls 1). A/B over whole binaries: `head` (O2-noinline, 204 fns) goes
18 code-ptr texts in 5 functions → **0**; `od` (O0, 252 fns) 8 → 2. `tar` (1,585 fns) and
`libacl` (142 fns) are **unchanged** — there the `(code *)` is the genuine-callback family.
Corpus-wide 431 functions declare a `code *X` that is never called in the body.

**Verdict:** `covered-by-option`, i.e. a default-narrowing question, not a new feature. The
honest narrowing is not "flip it off" (it fixes the real GH-6930 case) but "require the
resolved entry to be at a non-zero image base, or refuse when the constant also feeds an
arithmetic/comparison chain" — that is a separate one-module change in
`kuna_inferfuncentry.rs` and should be its own case.

---

## Cause 3 (the largest by raw count) — untyped callback fields: NOT a defect

`libacl::set_acl_fd`, `e2fsprogs::e2fsck_pass3`, `x0r-usb::IRC_Login`: `(code *)a3[1]`,
`(*(code *)a0[0x2c])(...)`, `(code *)&v5`. Every one is the standard Ghidra-lineage
rendering for an indirect call through a pointer with no recovered prototype, and ghidra
emits it identically or more often (19 vs 12, 6 vs 3, 11 vs 11). Neither ida nor ghidra
recovers `struct error_context` / `e2fsck_ctx->progress` either. Nothing to fix; the
`realtypes-pointee-size` follow-up on `set_acl_fd` was the real defect there and shipped as
#232.

`tar::verify_volume` is `already-fixed`: the 5 stored `(code *)0x0` are gone because
DIV-34 renders zero pointers as `NULL` and DIV-36 elides the comparison entirely
(`if (dat_84d50)`); `--option truthycond off` brings back `if (dat_84d50 != NULL)`, never
`(code *)0x0`. This is exactly the print-normalization staleness `docs/decbench-loop.md`
warns about.

---

## Metric note — the `code_ptr` wart column mis-ranks the NOVEL pool

Two independent problems with `scripts/decbench/novel.py:50`
(`"code_ptr": (re.compile(r"\(code\s*\*\)"), 8)`):

1. **It is absolute, not relative.** `(code *)` is inherited Ghidra rendering; four of the
   eight top `code_ptr` rows are rows where kuna emits **fewer** than the co-best rival.
   The pool's #1 entry — `libacl::set_acl_fd`, defect score 152, entirely `code_ptrx19` —
   is a pane where ghidra emits 19 and kuna 12. Weight 8 (the second-highest after
   `halt_baddata`) then dominates the ranking. Scoring `max(0, kuna − best_rival)`, or
   restricting the pattern to the ARM-masked form
   `\(code \*\)\([^;\n]*&\s*0xfffffffe`, would surface the 2,496 genuinely-defective ARM
   functions instead.
2. **The published counts are stale.** `docs/decbench/novel.json` was mined 2026-08-01
   17:28; the tree's kuna artifacts were regenerated 23:35–23:55 the same day. Recomputing
   `defect_profile` over the current artifacts: 212 `code_ptr` cases, **408 → 303**
   occurrences, 47 of them now zero (`tar::verify_volume` 5→0,
   `coreutils::copy_fd` 4→1, `bash::edit_and_execute_command` 2→0). Re-run `novel` before
   trusting the column.

---

## Recommendation

One PR, `feat/decbench-funcptralign`, P3, strict bug fix, no flag: make
`RuleFuncPtrEncoding` read the live `funcptr_align`. It is the single largest
kuna-specific correctness/readability defect in the NOVEL pool by breadth (2,496 function
instances, 9 projects, 3 of the 4 cases triaged here), it has zero GED value, and it is a
one-function change whose plumbing already exists and is already proven live by an existing
stage test. File the `inferfuncentry` collision narrowing as a separate, smaller case.
