---
case_id: O2-noinline-iproute2-ip-netns_add
pool: novel
status: covered-by-option
tier: N
margin: 0
fresh_verdict: reproduces on today's product default (binary is 722 KiB so `--mode auto` = `reliable`) -- kuna emits 16 phantom byte-stores INTO the read-only .rodata string on an error-return path that has no such code
option_closing: readonly
feature_slug: null
scope: small
confidence: high
---

## Side-by-side

The case is **novel-pool**: kuna already scores best (GED 8; ida 14, binja 25, ghidra 26).
Nothing here is a score gap. The mined defect signal (`subpiece: 14`, `goto: 4`) is real
and points at a genuine correctness bug in kuna's output.

`netns_add` inlines one `strcpy(proc_path, "/proc/self/ns/net")`, which GCC lowered to a
single `movdqa`/`movaps` pair plus a 2-byte tail store. It occurs **exactly once** in the
machine code:

```
   21140:  66 0f 6f 05 a8 11 06 00   movdqa xmm0,XMMWORD PTR [rip+0x611a8]  # 822f0
   21155:  66 89 84 24 40 10 00 00   mov    WORD PTR [rsp+0x1040],ax        # 't'
   21160:  0f 29 84 24 30 10 00 00   movaps XMMWORD PTR [rsp+0x1030],xmm0
```

**ghidra (stored)** -- splits it byte-wise (same as kuna), but only at the one real site,
and the destination is the stack buffer:

```c
local_1038 = 0x74;
local_1048[0] = s__proc_self_ns_neblackhole_001822f0[0];
...
acStack_1040[7] = s__proc_self_ns_neblackhole_001822f0[0xf];
```

Ghidra's `mount --make-shared` failure path is just:

```c
if (iVar3 != -1) { uVar9 = 0xffffffff; flock(iVar3,8); close(iVar3); goto LAB_00120f85; }
goto LAB_00120f7f;
```

**ida (stored)** -- one statement, the best rendering of the three:

```c
v33 = 116;
*(__m128i *)special_file = _mm_load_si128((const __m128i *)&xmmword_822F0);
```

**kuna (stored 2026-07-27 AND fresh on today's build, `--mode reliable` == today's `auto`
for this 722 KiB binary)** -- the copy is emitted **twice**. The second occurrence is
phantom, sits on the `v3 == -1` error-return path, and *writes into the read-only literal*:

```c
label_21004:
      if (v3 == -1) {
        s_822f0[0] = (char)s_822f0[0];
        s_822f0[1] = SUB81(s_822f0[0],1);
        s_822f0[2] = SUB81(s_822f0[0],2);
        ...
        s_822f0[0xf] = SUB81(s_822f0[8],7);
        return 0xffffffff;
      }
      flock(v3,8);
      close(v3);
      return 0xffffffff;
```

The real site additionally splits one 16-byte stack buffer across **three** symbols, one of
which is an internal placeholder name leaking into the C:

```c
        v11[7] = s_822f0[7];
        $$undef00000004[0] = s_822f0[8];   // <-- internal name in emitted C
        v14[1] = s_822f0[9];
```

**kuna, `--option readonly on`** (or `--mode aggressive`, or `--option formatstring on`)
-- phantom block gone, real site collapses to two correct word assignments:

```c
        v15 = 0x74;
        v11 = 0x65732f636f72702f;   /* "/proc/se" */
        v14 = 0x656e2f736e2f666c;   /* "lf/ns/ne" */
        if (0 <= (int4)mount(&v11,v7,"none",0x1000,0)) { ... }
```

## Source

`~/github/decbench/results/full_run/O2-noinline/iproute2/compiled/ipnetns.i`

The only copy in the function (ipnetns.c:914):

```c
  strcpy(proc_path, "/proc/self/ns/net");
 } else {
  snprintf(proc_path, sizeof(proc_path), "/proc/%d/ns/net", pid);
 }
```

The path kuna decorates with the phantom copy (ipnetns.c:884-891) contains **no copy at
all**:

```c
   if (lock != -1) {
    flock(lock, LOCK_UN);
    close(lock);
   }
   return -1;
```

## Analysis

**The one structural symptom.** kuna emits a 16-statement block of **stores into read-only
.rodata** (`s_822f0[0] = (char)s_822f0[0]; s_822f0[1] = SUB81(s_822f0[0],1); ...`) on the
`lock == -1` error-return path, where the machine code is only `cmp ebp,-1 / je 20f7f /
mov r13d,-1 / ret`. It is phantom code, it is a write to a string literal, and it is a
duplicate of a copy that occurs once in the binary. Ghidra, IDA and Binja all emit the copy
exactly once and never store into the literal.

**Root cause.** Chain, verified against the pipeline's own p-code (`decomp_dbg` `print raw`):

1. `strings` (analysis tier, default ON) plants a typelocked `char[N]` at `ram:0x822f0`.
2. That array type makes P3's `RuleSplitCopy` / `SplitDatatype::split_copy` fire on the
   16-byte rodata->stack COPY. `SplitDatatype::build_in_subpieces`
   (`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs:3684`) builds the input
   pieces as SUBPIECE ops whose **output varnodes are addressed in ram at the source
   global's own sub-addresses** -- i.e. it synthesises *definitions of read-only memory*:

   ```
   0x00021160:11ca:  r0x000822f0:1 = SUB81(r0x000822f0(0x00021123:b44),#0x0:4)
   ...
   0x00021160:11d9:  s0xffffffffffffefb8:1 = r0x000822f0:1(0x00021160:11ca)
   ```

   (`r...` is ram space; this is a faithful port of Ghidra's `buildInSubpieces`, and Ghidra
   creates the same ram-addressed pieces.)
3. `RuleSplitCopy` lives in `oppool2`, i.e. **inside `mainloop`**
   (`infra/universalaction.rs:558`, `:615`), so `ActionHeritage` runs again afterwards. The
   next heritage pass now sees byte-sized *defs* at `ram:0x822f0..0x822ff` overlapping the
   8-byte *reads* at the same address, and runs `Heritage::refinement` ->
   `refine_write` -> `split_pieces` (`p3_dataflow/heritage.rs:3254 / :3170 / :3010`),
   which materialises a byte-wise **write-back** of the merged value into ram in the shared
   return block:

   ```
   Basic Block 24 0x00020f7f-0x00020fb0
   0x00020fb0:11aa:  u0x1000044a:1 = SUB81(u0x1000044a(0x00021004:112e),#0x0:4)
   ...
   0x00020fb0:11b2:  r0x000822f0:1 = u0x1000044a:1
   ...  (and the same 8 for 0x822f8..0x822ff)
   ```
4. P9 prints that block as the 16 phantom assignments.

**Owning phase: P3 (dataflow).** The decision that goes wrong is "may a datatype-driven
copy split create varnode *definitions* at a read-only ram address", and its consequence in
heritage refinement. It is not a P9 printing bug (the ops genuinely exist in the IR) and
not a P5/P6 typing bug in the sense the miner hint suggested -- the miner's guess ("the
SUBPIECEs come from char-buffer manipulation the type system should have recovered") is
**half right**: the char-array type is what *triggers* the split, but the recovery the type
system owes here is not a better buffer type, it is recognising that the source is
constant. Note P6 does also mis-handle the destination (one 16-byte stack buffer becomes
`v11` + `$$undef00000004` + `v14`, three symbols for two 8-byte slots, with an internal
placeholder name in the emitted C) -- that is a **separate, secondary** symptom, 92 of the
94 `$$undef` occurrences in this binary survive the fix below, so it needs its own case.

**Reproduction / mode facts.**

- `ip` is 739,976 bytes (> 500 KiB), so `--mode auto` selects **`reliable`**, not
  `aggressive`. The fresh no-flag triage run and `--mode reliable` agree: the bug is live
  on today's product default. It also reproduces on the *unstripped* binary, so it is not a
  stripped-input artifact.
- `--mode aggressive` hides it here, but only incidentally: minus-one bisection over the 13
  aggressive overrides shows the sole responsible flag is **`formatstring`**, whose
  decompile -> prototype-override -> re-decompile loop retypes the `__snprintf_chk`
  destination and thereby stops the split. That only helps functions that contain a
  printf/scanf-family call; it is not a fix.
- Whole-binary scan of `ip` (`decompile-all --mode reliable`, 1912 functions): **48**
  phantom stores into rodata across **2** functions (`netns_add` @0x20e10, `__get_hz`
  @0x6cbf0), 42 `SUB81`s.

**The option that closes it.** `--option readonly on` (Ghidra `OptionReadOnly` /
`Architecture::readonlypropagate`, default off in both Ghidra and kuna) folds the read-only
LOAD to a constant *before* the split, so `SplitDatatype::generate_constants` short-circuits
`build_in_subpieces` and no ram defs are ever created. Measured whole-binary on `ip`,
`--mode reliable`:

| run | LOC | rodata stores | SUB81 | `$$undef` | gotos | funcs | wall |
|---|---|---|---|---|---|---|---|
| default | 62652 | 48 | 42 | 94 | 746 | 1912 | 31.9s |
| `--option readonly on` | 62591 | **0** | **0** | 92 | 746 | 1912 | 28.1s |

No goto/function-count change, LOC -61, no speed cost.

**Why the references win.** IDA keeps the copy as a single `_mm_load_si128` intrinsic
assignment. Ghidra splits it byte-wise exactly like kuna but never synthesises the
write-back, because in Ghidra those ram-addressed SUBPIECE outputs do not come back through
a heritage refinement pass.

## Proposed fix

Ranked, both small:

1. **Narrow (preferred).** In `SplitDatatype::build_in_subpieces`
   (`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs:3684`), extend the existing
   `generate_constants` short-circuit: when the root varnode is address-tied into a
   **read-only** range (the fact is already available -- `Varnode::is_read_only`, used by
   `p2_lift/jumptable.rs:1700,2155`, and `SymbolTable::is_read_only`, used by
   `p9_emit/printc.rs:7419`), read the bytes from the loadimage and emit per-piece
   *constants* instead of ram-addressed SUBPIECE outputs. This removes the illegal defs at
   their source, so heritage refinement never has anything to reconcile, and it is
   independent of the global `readonlypropagate` policy. Risk: low and local -- it only
   changes copies whose source is a constant anyway; the belt-and-braces companion is to
   make `Heritage::refine_write` (`p3_dataflow/heritage.rs:3170`) decline to emit write-back
   pieces for a read-only address, which would also catch any other producer of the same
   shape.
2. **Default flip.** Turn `readonly` on by default. It closes the case exactly and cheaply,
   but it is a *global* semantic change (every read-only load becomes a constant), it is
   default-off in upstream Ghidra for good reasons (RELRO / `.data.rel.ro` before
   relocation, sections wrongly marked read-only), and `readonly` is **not in the 81-row
   catalog** -- it is a Ghidra-inherited option registered in
   `p0_knowledge/options.rs:968` with no `settableTable` row -- so a flip would need a
   catalog row, catalog-count bumps, a DIV row in `docs/history.md`, and 0/675 datatest
   evidence. Worth measuring, but option 1 is the honest fix for *this* defect.

Not in scope for this record: the `$$undef00000004` placeholder name leaking into emitted C
and the 3-symbols-for-one-16-byte-buffer split (P6 variable merging) -- 92 `$$undef`
occurrences in this binary are untouched by either fix above and deserve their own case.
