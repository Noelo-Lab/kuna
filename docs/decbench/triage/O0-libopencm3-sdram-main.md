---
case_id: O0-libopencm3-sdram-main
pool: novel
status: feature-candidate
tier: N
margin: 0
fresh_verdict: reproduces on today's build in BOTH --mode auto and --mode reliable — the emitted body of main starts at label_8000986 (the 'p'/'P' command handler) and the real function entry (clock_setup/console_setup/sdram_init/...) is emitted after an unconditional `goto label_8000792;` as unreachable dead code
option_closing: null
feature_slug: finalorder-entry-first
scope: small
confidence: high
---

## Side-by-side

The miner hint was "17 gotos at O0". The goto count is real but is NOT the defect —
angr(14)/ida(16)/ghidra(17) all sit in the same band because gcc lowered the source's two
nested `switch (c)` statements on `char` into binary compare cascades. The real defect is
an ordering bug that every other decompiler gets right and the GED metric cannot see.

Ghidra (stored) — entry first, correct:

```c
void main(void)
{
  FUN_080003d0();            // <- entry code is the FIRST thing emitted
  FUN_08000304();
  FUN_080003fc();
  FUN_080002c8(DAT_080009e8);
  ...
  local_10 = FUN_080006fc(0xd0000000,0);
LAB_08000792:
  FUN_080002c8(DAT_080009f4);
  switch(uVar1) { ... }
```

IDA (stored) and angr (stored) likewise open with `sub_80003D0(); sub_8000304(); ...` /
`sub_80003d1(); sub_8000305(); ...`.

kuna (stored 2026-07-27 AND fresh today — structurally identical, only DIV-34..39 print
normalization differs, e.g. `v3 = v3 + -0x10` -> `addr = &addr[-0x10]`):

```c
void main(void)
{
  char *addr; // stack - 0x10
  int4 i; // stack - 0xc
  char v1; // r0
  uint1 v2; // r0

label_8000986:                                  // <-- FUNCTION ENTRY LANDS HERE
  setISAMode(1);
  console_puts("Previous ");                    // the 'p'/'P' command handler
  setISAMode(1);
  v2 = console_getc(1);
  if (v2 != 0x70) {
    ...
    goto label_8000792;
  }
label_80009b0:
  setISAMode(1);
  console_puts("Page\n");
  addr = &addr[-0x100];
  setISAMode(1);
  dump_page(addr,0);
  goto label_8000792;                           // <-- unconditional, no fallthrough
  clock_setup();                                // <-- REAL ENTRY, now UNREACHABLE
  console_setup();
  sdram_init();
  console_puts("SDRAM Example.\n");
  console_puts("Original data:\n");
  dump_page(0xd0000000,0);
  for (i = 0; i <= 0xff; i = i + 1) {
    *(char *)(i + -0x30000000) = (char)i;
  }
  console_puts("Modified data (with Fill Increment)\n");
  addr = (char *)dump_page(0xd0000000,0);
label_8000792:
  console_puts("CMD> ");
  switch(console_getc(1)) { ... }
```

The emitted C is semantically wrong, not merely ugly: control enters `main`, falls into the
`label_8000986` "Previous " handler, and the initialization block can never execute.

## Source

`/home/mahaloz/github/decbench/results/full_run/O0/libopencm3/compiled/sdram.i:3829`

```c
int
main(void)
{
 int i;
 uint8_t *addr;
 char c;

 clock_setup();
 console_setup();
 sdram_init();

 console_puts("SDRAM Example.\n");
 console_puts("Original data:\n");
 addr = (uint8_t *)(0xd0000000);
 (void) dump_page(addr, (void *)(0));
 addr = ((uint8_t *)(0xd0000000));
 for (i = 0; i < 256; i++) {
  *(addr + i) = i;
 }
 console_puts("Modified data (with Fill Increment)\n");
 addr = ((uint8_t *)(0xd0000000));
 addr = dump_page(addr, (void *)(0));
 while (1) {
  console_puts("CMD> ");
  switch (c = console_getc(1)) {
  case 'f': case 'F': ...
  case 'n': case 'N': ...
  case 'p': case 'P':                       /* the block kuna hoists to the top */
   console_puts("Previous ");
   switch (c = console_getc(1)) { ... }
   break;
  case '?': default: ...
  }
 }
}
```

The prologue is unconditionally first; the `p`/`P` arm is reachable only from the switch.

## Analysis

**Structural symptom (one):** the top-level component list of the structured block graph is
emitted in *collapse order*, not in *final order*, so a non-entry component can print first
and the function's entry block is emitted after an unconditional `goto` — i.e. the entry
code is emitted as unreachable dead code.

**Owning phase: P8 (structure).**

**Root cause: `BlockGraph::orderBlocks()` is an unported STUB.**
`decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs:3786` — `ActionFinalStructure::apply`:

```rust
        // STUB(W7/W8): `orderBlocks`/`scopeBreak` (the rest of the goto/break
        // print-prep) remain unported in `block.rs`.  Recorded as losses.
        data.finalize_switch_printing();
        data.finalize_forloop_printing();
```

Upstream `ActionFinalStructure::apply` (blockaction.cc:2192) opens with the call kuna skips:

```cpp
  graph.orderBlocks();          // <-- not ported
  graph.finalizePrinting(data);
  graph.scopeBreak(-1,-1);
  graph.markUnstructured();
  graph.markLabelBumpUp(false);
```

`BlockGraph::orderBlocks` (block.hh:437) is three lines —
`if (list.size()!=1) sort(list.begin(),list.end(),compareFinalOrder);` — and
`FlowBlock::compareFinalOrder` (block.cc:709) is exactly the missing rule:

```cpp
  if (bl1->getIndex() == 0) return true; // Make sure the entry point comes first
  if (bl2->getIndex() == 0) return false;
  ... // RETURN-terminated blocks last
  return (bl1->getIndex() < bl2->getIndex());   // otherwise by index
```

kuna already maintains the invariant this sort depends on: `BlockGraph::add_block`
(`decompiler/crates/kuna-decomp/src/substrate/block.rs:1712`) faithfully keeps a composite's
`index` at the *minimum* over its components, and `identify_internal` (block.rs:1782) removes
the absorbed nodes and *appends* the new composite at the end of the parent's list. So kuna
ported the producer of the ordering key and never ported its only consumer: whenever
structuring cannot collapse the function to a single top-level component, the residual list
is in whatever order the collapse happened to append, and printing (`emit_block_graph`,
`p9_emit/printc.rs:2810`, a faithful `for blk in list` walk) faithfully emits that wrong
order. P9 is innocent — it prints the list it is handed.

**Not the region structurer.** `--option regionstructure off` (the pure Ghidra
`CollapseStructure` path), `regionlooprefine off`, `regionedgeorder on` and `condfold wide`
all produce the identical hoisted-entry output, because they all build components through the
same `new_block_*` / `identify_internal` primitives and all end at the same
`ActionFinalStructure`. No existing option closes this.

**Why the reference wins:** Ghidra/IDA/angr all order the function body from the entry. Ghidra
in particular reaches the *same* goto-heavy shape as kuna on this function (17 gotos, no
`while(true)`) and still reads correctly, purely because `orderBlocks()` runs.

**Why the metric missed it.** kuna is the *best* scorer here (GED 58 vs angr 77, ghidra 85,
ida 79) — decbench builds its CFG from the emitted statements and edges, and permuting three
top-level components barely perturbs node/edge sets. The score cannot see that the entry is
unreachable. This is a case where the winning pane contains a correctness defect.

**Prevalence (measured today, `kuna decompile-all` + a scan for "unconditional `goto`
immediately followed by a non-label statement at the same or deeper brace depth"):**

| binary | arch | functions | affected |
|---|---|---|---|
| libopencm3/sdram.elf | ARM Cortex-M | 56 | 1 (`main`) |
| libopencm3/usart-stdio.elf | ARM Cortex-M | 182 | 3 (`memchr`, `_vfprintf_r`, `_vfiprintf_r`) |
| libopencm3/button.elf | ARM Cortex-M | 31 | 0 |
| chibios/ch.elf | ARM Cortex-M | 774 | 0 |
| diffutils/diff | x86-64 | 533 | 6 (incl. `lines_differ`, `format_group`) |
| bzip2/bzip2 | x86-64 | 161 | 2 (`BZ2_decompress`, `fallbackSort`) |
| gzip/gzip | x86-64 | 264 | 0 |

Roughly 1% of functions, both architectures, always the goto-heavy ones. Two more witnesses:

```c
uint1 * memchr(uint4 *a0,uint4 a1,uint4 a2)   // usart-stdio.elf 0x8002e20
{ ...
label_8002e38:
  a0 = v6;
  if (!a2) return NULL;
  goto label_8002e2c;
  a1 &= 0xff;                    // <-- real entry: mask the search byte, unreachable
```

```c
bool lines_differ(char *s1,char *s2)          // diffutils/diff 0x11f90
{ ...
label_1234b:
  ...
  goto label_123b9;
  column = 0;                    // <-- real entry, unreachable
  v8 = (uint1 *)s1;
  v10 = (uint1 *)s2;
```

**Secondary observation (not the reported symptom, no action proposed here):** kuna emits the
ARM `setISAMode(1)` userop as a call statement before every `blx` (12 occurrences in this
function); Ghidra's pane has none. That inflates LOC but adds no CFG nodes, so it does not
move GED.

## Proposed fix

Mechanism — port the two upstream pieces, ~40 lines, one mechanism across two files:

1. `decompiler/crates/kuna-decomp/src/substrate/block.rs` — add
   `FlowBlock::compare_final_order` (entry `index == 0` first; a component whose `last_op` is
   `CPUI_RETURN` last; otherwise ascending `index`) and
   `BlockGraph::order_blocks(graph_id)` (`if list.len() != 1 { sort_by(compare_final_order) }`).
   Upstream `orderBlocks` is applied to the root graph only, not recursively — keep that.
2. `decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs:3786` — call it at the very
   top of `ActionFinalStructure::apply`, before `finalize_switch_printing()`, matching
   blockaction.cc:2192 ordering (it must precede `mark_unstructured`, whose label decisions
   are order-insensitive but whose output the printer consumes positionally).
3. Retire the `STUB(W7/W8): orderBlocks` note at blockaction.rs:3792 and describe the final
   ordering rule in `docs/spec/08-structuring.md` (the P8 owning chapter).
4. `tests/stages/` end-to-end case: `libopencm3/sdram.elf` `main` (or the cheaper
   `diffutils/diff` `lines_differ`) asserting the first emitted statement is `clock_setup();`
   and that no statement follows a depth-1 unconditional `goto` outside a label.

Classification — this is a strict bug fix restoring upstream behavior, not a judgment call:
the current output is unambiguously wrong C. Per `docs/agents.md` a strict bug fix needs no
flag. The one reason to gate it (`finalorder`, default-on) is blast radius: it reorders the
top-level components of *every* function whose structuring leaves more than one, which will
move `docs/baseline.json` assertions. Recommended sequence: implement unflagged, run
`make test`; if 0/675 move, ship unflagged; if any move, verify each moved assertion is an
improvement and gate behind `finalorder` with a DIV row rather than re-pinning the baseline.

Risks:
- `compare_final_order` reads `last_op()` on composite blocks; kuna's `last_op` equivalent
  must be checked to return the same thing upstream does for a `BlockGraph` (upstream
  `FlowBlock::lastOp` is virtual and returns null for a non-basic block unless overridden) —
  getting this wrong only misplaces return-terminated components, never the entry.
- Sorting the root list changes which edges the printer renders as fallthrough vs `goto`;
  goto counts can move in either direction on functions that already have a multi-component
  root. That is exactly what needs measuring in step 4.
- No interaction expected with `regionstructure`: both paths hand `ActionFinalStructure` the
  same kind of root list.
