## What

`RuleFuncPtrEncoding` — the rule that erases the ISA-mode / alignment bit-clear
SLEIGH puts on an indirect-call target — was fully ported and registered, but its
**only input** was a hard-coded stub:

```rust
/// `data.getArch()->funcptr_align` (C++).  // STUB(W4)
fn funcptr_align(_data: &Funcdata) -> int4 { 0 }
```

and the rule's first two lines are `let align = funcptr_align(data); if align == 0 { return 0; }`.
**The rule could never fire on any architecture.** This PR reads the value live,
`data.get_arch().funcptr_align` — the same accessor `p2_lift/jumptable.rs:3661` and
`p3_dataflow/ruleaction_6.rs:609` already use. That is the whole behavioural change.

The stub's premise was stale: `infra/architecture.rs:2105 decode_funcptr_align()` has
parsed the cspec's `<funcptr align="N"/>` into the field since W4, `:3902` calls it during
`parse_compiler_config`, and `:1869` copies it into the ArchSeam `Funcdata` reads.

Root-cause analysis and the full evidence: `docs/features/funcptralign/analysis.md`
(campaign record: `docs/decbench/triage/novel-code-ptr-cluster.md`).

## Before / after

### ARM / Thumb — `crazyflie` `cf2.elf` @ `0x804001c` (`dwHandleInterrupt`), the witness

SLEIGH lowers every Thumb indirect call through `BXWritePC` (`ARM.sinc:221`), whose body
is `local tmp = addr & 0xfffffffe`, so an `INT_AND` feeds the `CALLIND`.

```c
/* before */
void sub_804001c(int4 a0)
{
  dwReadSystemEventStatusRegister();
  if ((dwIsClockProblem(a0)) && (*(uint4 *)(a0 + 0x44)))
    (*(code *)(*(uint4 *)(a0 + 0x44) & 0xfffffffe))(a0);
  if ((dwIsTransmitDone(a0)) && (*(int4 *)(a0 + 0x40))) {
    dwClearTransmitStatus(a0);
    (*(code *)(*(uint4 *)(a0 + 0x40) & 0xfffffffe))(a0);
  }
  ...
```

```c
/* after */
void sub_804001c(int4 a0)
{
  dwReadSystemEventStatusRegister();
  if ((dwIsClockProblem(a0)) && (*(code **)(a0 + 0x44)))
    (**(code **)(a0 + 0x44))(a0);
  if ((dwIsTransmitDone(a0)) && (*(int4 *)(a0 + 0x40))) {
    dwClearTransmitStatus(a0);
    (**(code **)(a0 + 0x40))(a0);
  }
  ...
```

```c
/* ghidra (stored decbench artifact, ghidra_cf2.c:49507) */
void dwHandleInterrupt(int param_1)
{
  FUN_0803e9f4();
  iVar2 = FUN_0803f1b4(param_1);
  if ((iVar2 != 0) && (*(code **)(param_1 + 0x44) != (code *)0x0)) {
    (**(code **)(param_1 + 0x44))(param_1);
  }
  ...
```

The source performs **no** bit-clear — `libdw1000.i`, `dwHandleInterrupt`:
`if(dwIsClockProblem(dev) && dev->handleError != 0) { (*dev->handleError)(dev); }`.
The fields are `dwHandler_t` function pointers. kuna was asserting an operation the
program does not perform, and the mask also blocked the pointer-to-code type from
back-propagating onto the LOAD, which is why every call site needed the extra `(code *)`.

### AARCH64 `align=4` — verified explicitly against stock Ghidra

The five AARCH64 cspecs declare `<funcptr align="4"/>`, i.e. `funcptr_align = 2`, so the
seam strips a **2-bit address-alignment** mask rather than a mode bit. Probe built with
`clang --target=aarch64-linux-gnu -O1 -ffreestanding -c`: `handle` does a plain
`blr x8`, `handle_masked` really executes `and x8, x8, #0xfffffffffffffffc; blr x8`.

| | `handle` (no mask) | `handle_masked` (`& ~3`) |
|---|---|---|
| kuna **before** | `(**(code **)&a0[2])()` | `(*(code *)(*(uint8 *)&a0[2] & 0xfffffffffffffffc))(*a0)` |
| kuna **after** | `(**(code **)&a0[2])()` | `(**(code **)&a0[2])(*a0)` |
| **Ghidra 12.1 headless** | `(**(code **)(param_1 + 2))()` | `(**(code **)(param_1 + 2))(*param_1)` |

Three reasons this is right and not just convenient:

1. Upstream `RuleFuncPtrEncoding::applyOp` (`ruleaction.cc:9928`) is **one code path** —
   `slide = ~0 << align; if ((testmask & slide) == val) { opRemoveInput; opSetOpcode(COPY); }`.
   There is no `align == 1` branch, and the comparison is exact: on AARCH64 a `& ~3`
   matches and a `& ~1` does **not**. kuna's port is the same arithmetic.
2. AArch64's own lifting emits no mask at all — `AARCH64base.sinc:887`,
   `:blr Rn_GPR64 { pc = Rn_GPR64; x30 = inst_start + 4; call [pc]; }` — so the rule is
   inert on ordinary AArch64 code and only ever fires on a mask the *program* wrote.
3. Stock Ghidra 12.1 headless strips that program-written mask and prints the identical
   form (table above).

Pinned at the rule level by a new unit test, `funcptrencoding_strips_aarch64_word_alignment`
(align=2 strips `& ~3` **and** declines `& ~1`), alongside `..._strips_thumb_mode_bit` and
`..._noops_without_cspec_funcptr`.

MIPS is the ARM shape: `mips.sinc:925 JXWritePC` under `ISA_VARIANT` (every
`mips32*`/`mips64*` slaspec) does `tmp = -2 & addr`, so `jalr` carries the mask too.

## Breadth

`kuna decompile-all`, whole binary, occurrences of `& 0xfffffffe` / `(code *)` / `(code **)`:

| binary | masks | `(code *)` | `(code **)` |
|---|---|---|---|
| crazyflie `cf2.elf` (ARM Cortex-M4) | 264 → **15** | 255 → **46** | 0 → **205** |
| nuttx `nuttx` (ARM Cortex-M) | 459 → **40** | 435 → **42** | 0 → **397** |
| libopencm3 `msc.elf` (ARM Cortex-M) | 56 → **6** | 50 → **10** | 0 → **41** |
| chibios `ch.elf` (ARM Cortex-M) | 38 → **13** | 40 → **30** | 0 → **16** |

Every ARM project in the corpus had **zero** `(code **)` before this change, because a
mask sat on every indirect-call target. The triage measured the class corpus-wide at
**4,027 Thumb-masked indirect calls in 2,496 function-instances across 97 binary-instances
in 9 ARM projects** (crazyflie 986 fns, betaflight 694, cleanflight 384, libopencm3 221,
nuttx 144, chibios 48, riot-os 13, freertos 4, u-boot 2).

**x86-64 / i386 controls are byte-identical** — their cspecs declare no `<funcptr>`, so
`funcptr_align == 0` and the rule stays inert. Whole-binary `decompile-all`
(`--max-fn-seconds 0`) diffs, before vs after:

| binary | lines | changed lines |
|---|---|---|
| zlib `minigzip` (x86-64) | 8,923 | **0** |
| gzip `gzip` (x86-64) | 9,465 | **0** |
| bash `bash` (x86-64) | 120,695 | **0** |
| x0r-usb `x0r-usb.exe` (i386 PE) | 2,205 | **0** |
| coreutils `head` (x86-64) | 4,403 | **0** |

## Ablation

`kuna test --all --baseline docs/baseline.json` → **PARITY OK, 675/675**, with the fix
live and **no re-pin, no per-test opt-out**. The corpus's two `ARM:LE:32:v8` and six
`MIPS:*:32:default` datatests contain no masked indirect call.

`make test-stages` → **PARITY OK**; the stages baseline was re-recorded only to add the
two new keys.

That clean ablation, plus the fact that the alignment width is declared by the *processor
cspec* rather than by kuna and that upstream honours it unconditionally, is why this ships
**without an option**: `CLAUDE.md` — *"a strict bug fix that only corrects wrong output
needs no flag"*. It is recorded in `docs/history.md` under **Convergences (upstream
behavior restored — not divergences)**, which is exactly where the registry puts a port
defect whose fix moves kuna back onto upstream's default.

## Speed

Interleaved `decompile-all` medians, two builds of this tree (pre-fix stub vs live read),
on a box running three sibling agents:

| binary | before | after | delta | pairs |
|---|---|---|---|---|
| libopencm3 `msc.elf` (ARM) | 592 ms | 597 ms | **+0.84%** | 5 |
| coreutils `head` (x86-64) | 5,368 ms | 5,443 ms | **+1.40%** | 5 |
| crazyflie `cf2.elf` (ARM) | 36,838 ms | 32,052 ms | **-12.99%** | 3 |

Worst case +1.40%, well inside the +5% budget — and that binary's emitted C is
*byte-identical* before and after, so the number is measurement noise, not cost. The ARM
witness gets materially **faster**: one op leaves the graph at every indirect call.

(`scripts.pipeline.timeit` is not usable here — it measures a feature by flipping its
option, and this change has none. The equivalent measurement is two builds of this tree,
interleaved OFF/ON, medians.)

## Benchmark delta — measured, and it is zero

`scripts.decbench.rescore` on `O2-noinline-libopencm3-msc-msc_data_rx_cb` (the sibling
thumb-mask case whose `case_id` exists in the pool on `main`):

```
ged_recorded 13.0   ged_before 18.0   ged_after 18.0   ged_delta 0.0   (source_nodes 44)
```

**GED is flat**, exactly as the cluster record predicted ("it has zero GED value"). GED is a
CFG edit distance; this fix removes an `INT_AND` from an expression and retypes a `LOAD` —
it changes no basic block and no edge, so the metric cannot see it. Reported rather than
hidden.

The headline witness `O2-noinline-crazyflie-*-dwHandleInterrupt` is **not scoreable**:
`rescore` resolves its source to `crtp_localization_service.i` (the function is in
`libdw1000.i`) and gets a 1-node degenerate source CFG, so before and after both come out
`inf`. That is the degenerate-source-CFG artifact `docs/decbench-loop.md` warns about, not
a result — `record.json` stores it as `null` with the explanation.

The win here is correctness and readability, and it is measured directly instead: kuna's
post-fix C is byte-for-byte Ghidra's on both the ARM witness and the AArch64 probe, the
invented bit-clear is gone, and the callee type is finally recovered.

## Test

`tests/stages/ghdec-funcptralign.xml` — two-pass, hand-laid ARM Thumb-2 dispatcher
(`push {r4,lr}; mov r4,r0; ldr r3,[r0,#4]; cmp r3,#0; beq +1; mov r0,r4; blx r3; pop {r4,pc}`),
the reduced `dwHandleInterrupt` shape. Pass 1 is the default pipeline; pass 2 re-runs with
`option thumbfuncptr off`, the *other* live `funcptr_align` reader (GH-8471 / DIV-2), so
the documented interaction risk is pinned: the mask must stay stripped either way.

**0 of 2 assertions pass pre-fix, 2/2 post-fix.**

## Gates

```
$ make test
datatests: 675/675 assertions passed
exit: 0
=== baseline parity ===
PARITY OK

$ make test-stages
datatests: 358/358 assertions passed
exit: 0
=== baseline parity ===
PARITY OK

$ make rust-test
293 x "test result: ok", 0 failed; options_md_matches_the_generator_byte_for_byte ... ok
(exit 0)

$ make check-spec
check-spec OK (lenient mode)
$ python3 tools/check_spec.py --strict
check-spec OK (strict mode)

$ kuna catalog --check
catalog OK: documents exactly the registered kuna options
```

`docs/baseline.json` is **untouched**. `docs/baseline-stages.json` gains exactly the two
new keys and the 356 → 358 count — no existing assertion moved:

```
-    356,        +    358,
-    356         +    358
+    "data:FUNCPTRALIGN #1: the SLEIGH ISA-mode bit-clear never reaches the emitted C",
+    "data:FUNCPTRALIGN #2: the callee field recovers as code ** and the call needs no cast",
```

Note: the first `make rust-test` run reported `1 target failed: -p kuna-cli --test
decompile_all_cli` while five concurrent cargo workspaces and two Joern runs shared the
box. That target passes in isolation (18/18) and the clean full re-run above is green —
it was a load flake in the wall-clock-sensitive watchdog tests, not this change (which
cannot reach an x86-64 CLI path: `funcptr_align == 0` there).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
