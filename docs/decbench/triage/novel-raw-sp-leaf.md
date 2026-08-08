---
case_id: novel-raw-sp-leaf
pool: novel
group_id: crazyflie::USBD_GetDescriptor (switch-arm fragments) + 17 siblings across 7 ARM binaries
status: feature-candidate
tier: N
margin: 0
fresh_verdict: the track's `code_ptr` and x86 `raw_reg` halves are both dead on today's build — the mined row `O2-noinline-crazyflie-cf2-USBD_GetDescriptor` goes code_ptrx8 -> code_ptrx1 and Thumb-mask 8 -> 0 (#249 funcptralign), and 34,102 fresh functions across 25 binaries contain ZERO bare x86/ARM GPR tokens. One leaf survives: the entry **stack-pointer register** renders as a bare, undeclared `sp` — 46 occurrences in 18 functions across 7 ARM binaries (0 on x86/x86-64/PE). It is the *leaf* form of the exact bug DIV-46 fixed for the PTRSUB form, and it has ZERO benchmark value: none of the 18 functions is in decbench's scored set.
option_closing: null
feature_slug: spleaf
scope: small
confidence: medium
---

## Side-by-side

### The mined row — `USBD_GetDescriptor` @ `0x801ddac` (NOVEL #3, `code_ptrx8,gotox3,subpiecex1`, score 76)

kuna GED 10 · angr 11 · ida 11 · ghidra 18 · source 20 nodes/28 edges · `approximated: false` ·
`degenerate_source: false` · `artifact_suspect: false`.

**stored kuna** (pinned 9623dc27) — the `(code *)` + Thumb-mask form the pool scored:

```c
    v3 = (*(code *)(*(uint4 *)(*(int4 *)(a0 + 0x5ec) + 4) & 0xfffffffe))(*(char *)(a0 + 2),&v4);
```

**fresh kuna** (today, `--mode auto` = aggressive on this 318 KiB binary), same line:

```c
      v3 = (**(code **)(*(int4 *)(a0 + 0x5ec) + 4))(*(char *)(a0 + 2),&v4);
```

`(code *)` 8 → **1**, `0xfffffffe` 8 → **0**. Structure is byte-identical in shape
(`triage --case ...`: kuna stored 58 loc / 3 gotos / 2 labels / 1 switch / 6 cases;
kuna fresh 58 / 3 / 2 / 1 / 6). The `gotox3` residue belongs to the crossing-edge/`condfold`
track, not this one, and is untouched by every option swept below.

### The survivor — six "functions" that are one source function's switch arms

`kuna decompile cf2.elf --addr 0x801de30` (transcript, today's build):

```
[decomp]> load file .../O2-noinline/crazyflie/stripped/cf2.elf
.../cf2.elf successfully loaded: ARM:LE:32:v8:default
[decomp]> option listing on
[decomp]> option aif on
[decomp]> option funcstart_patterns on
[decomp]> read symbols
[decomp]> load addr 0x801de30
[decomp]> decompile
Decompiling sub_801de30
Decompilation complete
[decomp]> print C

void sub_801de30(unsigned int a0,unsigned int a1,unsigned int a2,int4 a3,unsigned int a4,unsigned int a5) // return-dupe x2
{
  uint2 v1;
  int4 v2; // r4

  (**(code **)(a3 + 0x18))(a0,(char *)((int4)sp + 6));
  if (!a5._2_2_)
    return;
  v1 = *(uint2 *)(v2 + 6);
  if (!v1)
    return;
  if (v1 <= a5._2_2_)
    a5._2_2_ = v1;
  sub_801dbe8();
  return;
}
```

`sp` is declared nowhere — not in the function, not in `<name>.h` of
`kuna decompile-project`. `v2 // r4` is read before any write. `sub_801de40/50/60/70/80` are
the same body with offsets `0x14/0x10/0xc/8/4`.

The same binary, same run, emits the *correct* function at `0x801ddac` — the six arms
included, with the stack slot named:

```c
  uint2 v4; // stack - 0x12
  ...
      v3 = (**(code **)(*(int4 *)(a0 + 0x5ec) + 4))(*(char *)(a0 + 2),&v4);
```

And `betaflight sub_805e4b4` emits **both** conventions in one body:

```c
    v5 = &Stack000002ac;          // the DIV-46 unnamed-stack token
    v5[v7] = (&v5[(int4)sp])[4];  // the raw register leaf
```

## Source

`O2-noinline/crazyflie/compiled/usbd_req.i`, the single function the six fragments come from:

```c
static void USBD_GetDescriptor(USB_OTG_CORE_HANDLE *pdev, USB_SETUP_REQ *req)
{
  uint16_t len;
  uint8_t *pbuf;
  len = req->wLength ;
  switch (req->wValue >> 8) {
  ...
  case 3:
    switch ((uint8_t)(req->wValue)) {
    case 0x00: pbuf = pdev->dev.usr_device->GetLangIDStrDescriptor(pdev->cfg.speed, &len); break;
    case 0x01: pbuf = pdev->dev.usr_device->GetManufacturerStrDescriptor(pdev->cfg.speed, &len); break;
    ...
```

There is no source function corresponding to `sub_801de30`. `(char *)((int4)sp + 6)` is
`&len` — the stack slot kuna names `v4 // stack - 0x12` when it decompiles the whole
function — seen from a frame whose prologue is outside the pseudo-function's body.

## Analysis

### 1. The two halves the track was mined for are closed (verified, not assumed)

**`code_ptr`.** `(code **)` / `(code *)` / `0xfffffffe` counts, fresh whole-binary
`decompile-all` today vs the stored panes (the fresh column covers kuna's whole discovered
list, the stored column only decbench's scored subset — **not address-matched**, so read the
ratio, not the absolute):

| binary | stored kuna `code**`/`code*`/mask | fresh kuna | stored ghidra |
|---|---|---|---|
| crazyflie cf2 | 0 / 239 / 246 | **251 / 142 / 13** | 178 / 76 / 7 |
| betaflight | 0 / 347 / 346 | **466 / 168 / 29** | 148 / 86 / 9 |
| nuttx | 0 / 88 / 118 | **425 / 36 / 40** | 59 / 42 / 14 |
| cleanflight | 0 / 186 / 191 | **232 / 78 / 12** | 92 / 61 / 10 |

Every ARM binary goes 0 → hundreds of `(code **)`, and the Thumb ISA-mode mask collapses by
~10x. That is exactly what `novel-code-ptr-cluster.md` predicted and #249 shipped.

**`raw_reg`.** A declaration-aware token scan (comments and string literals stripped) over
**34,102 functions in 25 binaries** — x86-64 ELF, i386 PE, ARM Cortex-M, ARM u-boot — for
every x86 and ARM register identifier: **zero** occurrences of any GPR/flag token. The only
survivor is `sp`. (The 7 apparent rsyslog hits the pool reports are the string literal
`"sp-if-no-1st-sp"`.)

### 2. The survivor: 46 occurrences, 18 functions, 7 ARM binaries, 0 elsewhere

| binary | leaking fns | occurrences |
|---|---:|---:|
| O2-noinline crazyflie cf2 | 6 | 6 |
| O2-noinline betaflight | 3 | 12 |
| O0 betaflight | 3 | 12 |
| O2-noinline cleanflight | 1 | 6 |
| O0 cleanflight | 1 | 6 |
| O2-noinline / O0 nuttx | 2 | 2 |
| O2-noinline u-boot | 2 | 2 |
| **13 non-ARM binaries** (tar, rsyslogd, coreutils, zlib, libacl, utmpdump, mydoom.exe, x0r-usb.exe, minipig.exe, dexter.dll, …) | **0** | **0** |

### 3. Why the leaf leaks — P9, the arm DIV-46 did not cover

`print raw` on `sub_801de30`:

```
0x0801de32:a7:	u0x1000001e(0x0801de32:a7) = (cast)(sp(i))
0x0801de32:a3:	u0x10000022(0x0801de32:a3) = u0x1000001e(0x0801de32:a7) + #0x6
```

`sp(i)` is a function **input** Varnode at the stack-pointer register with no bound Symbol.
The same instruction inside the correctly-bounded `0x801ddac` lifts to the spacebase form:

```
0x0801de32:2c7:	r1(0x0801de32:2c7) = ->(sp(i),#0xffffffee)      # PTRSUB(sp,-0x12) -> &v4
```

A symbol-less leaf reaches `PrintC::push_vn_explicit_ir`'s `pushUnnamedLocation` tail
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs:6960`), which calls
`kuna_unnamed_location_name` (`printc.rs:8469`). That helper returns, **in order**: the
register name, then `dat_<addr>` for a processor space, then the `Space<hex>` leaf — so a
stack-pointer-register leaf takes the *first* branch and prints `sp`, never reaching the
`Stack<hex>` tail that produces `&Stack00000000`. This is the same helper and the same
"no Symbol covers this storage" situation DIV-46 repaired for the **PTRSUB** arm
(`op_ptrsub_ir`, `tests/stages/ghdec-spacebase-unnamed.xml`, which already asserts
`&Stack00000008` for a positive frame offset). The leaf arm was not covered.

**Owning phase: P9 — emit** (`p9_emit/printc.rs`). The upstream cause of *why a raw `sp`
read exists at all* is P1 (below), but the token itself is a render decision, and it is the
only part with a one-module mechanism.

### 4. Why the raw `sp` read exists — P1, and the obvious diagnosis is WRONG

16 of the 18 leaking entries are **mid-basic-block**: the instruction immediately before the
entry falls straight through into it (Thumb linear sweep, per entry):

```
cf2.elf        0801de30  prev@0801de2e: ldrb r0,[r0,#2]   entry: ldr r3,[r3,#0x18]   falls-through=True
betaflight     08068ee8  prev@08068ee6: uxtb r2,r2        entry: cmp r2,#5           falls-through=True
cleanflight    0803b344  prev@0803b342: uxtb r2,r2        entry: cmp r2,#5           falls-through=True
betaflight     0805e4b4  prev@0805e4b0: tbb [pc,r3]       entry: ldr r4,[r2,r5]      falls-through=True
```

The crazyflie six sit exactly 6 bytes past each true `tbb` target
(table base `0x801de14`, bytes `33 2b 23 1b 13 0b`; the accepted entries are
`table_end + 2*byte`). The listing tier cannot know that: `listing/walk.rs:11-12` records
BRANCHIND/CALLIND "with the computed/indirect predicates but **no static successor**
(deferred jump-table resolution)", so a Thumb `tbb`/`tbh` leaves its whole arm region an
**undefined gap**, and the gap-walkers probe it. Resolving TBB/TBH in the Listing tier is
**PR #239 step 4** (approved sequence, not started) — it would delete both the gap and these
entries, and it is a recall win as well.

**The obvious diagnosis — "AIF invents them, so turn AIF off" — is refuted by measurement.**
`--option aif off` on cf2 does remove the six (2,914 → 2,379 functions, and all six vanish),
but the same run then leaks `sp` in **16 different functions / 17 occurrences**, all of which
are present in the default run and clean there; and the count of functions that declare an
ARM condition flag (`ng`/`zr`/`cy`/`ov`/`q`) as an unwritten input — a state no ABI can pass,
so a proof of mid-body entry — **rises from 57 to 119** (1.96% → 5.00% of the list). Without
AIF the same bytes get decoded as A32 instead of Thumb, which is worse. AIF is net positive
here. That flag-input rate is the honest breadth number for the mid-body-entry class:
crazyflie 1.96%, betaflight 1.03%, u-boot 2.78%, nuttx 1.12%, cleanflight 0.42%, chibios
0.13%, **tar 0.00%, rsyslogd 0.00%**.

The remaining 2 of 18 (nuttx `0x8006494` @ O2-noinline, `0x800b1b6` @ O0 — one preceded by
padding, one by `bx lr`) are **real** functions: a hand-written Cortex-M context-save routine
(`*(undefined0 **)a0 = sp;`) with no `.i` source, which angr also discovers (`sub_8006495`).
There kuna's C is semantically right and only the token is wrong — which is why the fix has
to be at the leaf, not upstream.

### 5. Metric-artifact check

- `approximated: false`, `degenerate_source: false`, `source_nodes: 20` — the mined row is a
  clean measurement, and the source parses (the `.i` function is a real 6-case switch).
- **None of the 18 leaking functions appears in any decbench pane** (checked by
  `// Function: <name> @ 0x<addr>` marker in `kuna_*.c` for all 18): decbench decompiles the
  DWARF ground-truth function list, and these pseudo-functions are not in it. The class is
  **unscoreable** — it can never move GED, and `rescore` has nothing to run.
- Consequence for the census's metric recommendation: **widening `novel.py`'s `raw_reg`
  pattern to `sp|lr|pc` would surface ZERO new rows**, for the same reason. The stored kuna
  panes for all four ARM projects contain 0 bare register tokens. The useful metric change is
  the other one the prior record asked for: score `code_ptr` as `max(0, kuna − min_rival)` or
  drop it — today it scores kuna 269 against rivals emitting 600+, which is what put this
  closed defect at NOVEL #3 in the first place.

### 6. Option sweep

Every non-default value of all **88 catalog options** (≈90 settings) plus all three modes,
on `cf2.elf --addr 0x801de30`, counting bare `sp`:

```
BASELINE(default) sp=1
mode aggressive   sp=1
mode reliable     sp=0      <-- emits `void sub_801de30(void) { return; }`
mode fast         sp=0      <-- same stub
SWEEPDONE                   <-- no option changed the count
```

The six options `aggressive` does not carry were swept explicitly on the parent row too
(`condfold on`, `condfold wide`, `ptrentry on`, `cortexmvectors on`, `paramcopyhoist on`,
`dwarf_lines`): gotos 3, `(code *)` 1, mask 0 in every case. `reliable`/`fast` "close" it
only by not discovering the function (`reliable` decodes nuttx `0x8006494` as A32 garbage:
`(*(code *)(a0 - (a0 >> 0x11)))();`). **`option_closing: null`.**

## Proposed fix

**Slug `spleaf`. P9. One module. Strict bug fix — no flag** (the same argument DIV-46 shipped
under: emitting a raw machine register into the C body is not a judgment call, and the fix
removes a divergence rather than adding a behavior).

- `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`, `kuna_unnamed_location_name`
  (`:8469`) — before the `get_register_name` branch, test whether `(loc,size)` **is** the
  architecture's stack-pointer/spacebase register, and if so name the storage it holds at
  entry (`Address(stack_space, 0)`), which falls to the existing `Space<hex>` tail and
  yields the established `Stack00000000` token. Semantically exact under kuna's own model:
  stack varnode offsets are entry-SP-relative (the same pane already carries
  `s0x00000006` for `[sp,#6]`), so entry `sp` ≡ `&Stack00000000` — which is literally what
  `betaflight sub_805e4b4` and `sub_803c3c6` already print for the same register in the same
  binaries. The call site to check is `push_vn_explicit_ir` (`:6960`); the PTRSUB call site
  (`:7035`) already behaves.
- `docs/spec/09-emission.md` (the chapter whose `Anchors:` own `p9_emit/`) — extend the
  DIV-46 paragraph in §9.3 to say the spacebase *register leaf* takes the same
  unnamed-location convention.
- `tests/stages/ghdec-spleaf.xml`, modelled on `tests/stages/ghdec-spacebase-unnamed.xml`
  (two-pass, register token vs `Stack00000000`). Note the corpus-file-count bump in
  `kuna-base/src/xml.rs` and the stages-baseline re-record.

**Risks / the "would this produce WRONG output?" axis.**

1. **It buys consistency, not compilability.** `&Stack…` and `dat_…` are already undeclared
   extern-like tokens by design (DIV-46); `kuna decompile-project` on nuttx today produces
   2,537 `gcc -fsyntax-only` errors of which the `sp` leak is **one**. Do not sell this as
   "makes the output compile" — sell it as "no raw machine register survives into the C",
   which is the DIV-46 invariant.
2. **Blast radius is 46 tokens in 18 functions corpus-wide**, all ARM, so the whole-corpus
   before/after sweep the loop requires is cheap and should be run at whole-binary scope on
   all 25 binaries — but note that 16 of those 18 functions are garbage regardless, so the
   diff will look like nothing changed anywhere else. Confirm that, do not assume it.
3. **Do not "fix" this upstream instead.** Folding `INT_ADD(sp,c)` into `PTRSUB(sp,c)` would
   render the prettier `&Stack00000006`, but it is a P3/P5 rule change with a corpus-wide
   blast radius, and it cannot cover the nuttx case where the stack pointer is *stored*, not
   offset. The leaf covers 18/18.
4. **Priority.** ZERO GED value (§5) and 0.05% of functions. This is a correctness-hygiene
   item, not a campaign lever. The lever hiding behind this track is **PR #239 step 4
   (Listing-tier TBB/TBH)**: it deletes 13 of the 18 witnesses *and* is already-approved
   recall work — and, unlike "turn AIF off", it is not refuted by measurement.
