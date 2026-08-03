---
case_id: O2-noinline-sysvinit-utmpdump-print_utline
pool: novel
group_id: sysvinit::print_utline
status: already-fixed
tier: N
margin: 0
fresh_verdict: today's build emits zero raw register identifiers in either NOVEL raw_reg row — PR #226 (694ab8ef) closed the whole class; corpus re-verification of all 267 stored raw-register functions comes back 267/267 clean under every mode
option_closing: null
feature_slug: null
scope: small
confidence: high
---

Track: **raw register names appearing as variables in emitted C** (NOVEL pool) —
`mydoom::scan_textfile` (`raw_regx8`) and `sysvinit::print_utline` (`raw_regx6`).
This record covers both rows plus the corpus-wide sweep; the mydoom row already has its
own record (`O2-noinline-mydoom-mydoom-scan_textfile.md`, which proposed the fix that
shipped).

## Verdict in one line

**Disproven as a live defect.** Both rows, and every other raw-register leak in the
2026-07-27 corpus, were the `PTRSUB(spacebase, off)` render that PR #226 fixed on
2026-08-01. It is *not* the copy-shadow/printer diagnosis on file. A separate, much
smaller residual (28 functions / 83,168) survives through a different code path and is
described at the bottom — it is not campaign-grade at that breadth.

## The stored panes are stale, and this one is stale twice over

Two traps a follow-up agent will hit:

1. The `kuna_*.c` artifacts under `~/github/decbench/results/full_run` were **partially
   regenerated after the pool was mined**. `docs/decbench/novel.json` was written
   2026-08-01 17:28 UTC; `O2-noinline/sysvinit/decompiled/kuna_utmpdump.c` was rewritten
   2026-08-01 23:54 UTC and `O2-noinline/mydoom/decompiled/kuna_mydoom.c` at 23:49 UTC,
   both by a post-#226 build. So `triage --case` prints a "stored" pane with **zero**
   raw registers while the pool row says `raw_reg: 6`. That is not a mining bug — it is
   an artifact that no longer matches the snapshot it was mined from.
2. The benchmark-era artifacts (2026-07-15) that were *not* regenerated still carry the
   original form, and they are the honest "before" evidence. Both are quoted below.

## Side-by-side

### `print_utline` — before (benchmark-era artifact, `O0/sysvinit/decompiled/kuna_utmpdump.c`, 2026-07-15)

```c
// Function: print_utline @ 0x18ff
void print_utline(void)
{
  ...
  v13 = PTRSUB(RSP,0x164);
  if (inet_ntop(v12,v13,v1,v14) == 0) {
    v1[0] = 0;
  }
  sub_1727((int8)v7);
  sub_182f(PTRSUB(RSP,0x30),4);
  sub_182f(PTRSUB(RSP,0x34),0x20);
  sub_182f(PTRSUB(RSP,0x10),0x20);
  sub_182f(PTRSUB(RSP,0x54),0x100);
  printf("[%d] [%05d] ...",(uint8)(uint4)(int4)v3,(uint8)v5,PTRSUB(RSP,0x30),8,0x20);
  return;
}
```

Six `PTRSUB(RSP, …)` — exactly the `raw_reg: 6` the miner scored. `RSP` and `PTRSUB` are
both undeclared; the body is not C.

### `print_utline` — after (today's build, `9623dc27`, no flags = `--mode auto`)

```
$ SLEIGHHOME=/home/mahaloz/github/kuna/specs \
  ./decompiler/target/release/kuna decompile \
      ~/github/decbench/results/full_run/O2-noinline/sysvinit/stripped/utmpdump --addr 0x1a00
```
```c
void sub_1a00(void)
{
  char *v1;
  ...
  v14 = *(unsigned long *)(v4 + 0x28);
  v1 = (char *)inet_ntop((-(uint4)((!v10 && !v12) && !v13) & 0xfffffff8) + 10,
                         &Stack0000000000000164,v3,
                         (-(uint4)((!v10 && !v12) && !v13) & 0xffffffe2) + 0x2e);
  if (!v1) {
    v3[0] = 0;
    v1 = v3;
  }
  v2 = sub_1840((int8)v9);
  sub_1940(&Stack0000000000000030,4);
  sub_1940(&Stack0000000000000034,0x20);
  sub_1940(&Stack0000000000000010,0x20);
  sub_1940(&Stack0000000000000054,0x100);
  __printf_chk(1,"[%d] [%05d] [%-4.4s] [%-*.*s] [%-*.*s] [%-*.*s] [%-15.15s] [%-28.28s]\n",
               (int4)v5,v7,&Stack0000000000000030,8,0x20,&Stack0000000000000034,0xc,0x20,
               &Stack0000000000000010,0x14,0x100,&Stack0000000000000054,v1,v2);
  return;
}
```

Six `PTRSUB(RSP,…)` → six `&Stack<hex>`. Zero register identifiers, zero p-code operators.
Same result under `--mode reliable`, `--mode aggressive` and `--mode fast` (transcript
below), so this is a **code fix**, not a mode-preset effect.

### `scan_textfile` — before (`O0/mydoom/decompiled/kuna_mydoom.c`, 2026-07-15) / after

```c
  *(void *)&PTRSUB(ESP,0x14)[v2] = 0;          //  before  (x8, i386 PE, MSVC _chkstk frame)
  *(void *)&PTRSUB(ESP,0x10)[v2] = 0x80;
```
```c
  *(unsigned int *)&(&Stack00000008)[v2] = 0;  //  today
  *(unsigned int *)&(&Stack00000004)[v2] = 0x80;
```

### Mode sweep (both rows)

```
########## mode=reliable  mydoom scan_textfile 0x405af0
15:  *(unsigned int *)&(&Stack00000008)[v2] = 0;
...
########## mode=aggressive mydoom scan_textfile 0x405af0     (identical)
########## mode=fast       mydoom scan_textfile 0x405af0     (identical)
########## mode=reliable/aggressive/fast  utmpdump print_utline 0x1a00
19:  v1 = (char *)inet_ntop(...,&Stack0000000000000164,v3,...);
25:  sub_1940(&Stack0000000000000030,4);
```

`kuna catalog --json` (83 options) has **no** option whose `use_when`/`symptoms` mention
registers, spacebases, stack locations or undeclared identifiers — nothing to sweep, and
nothing to flip. The change was unflagged, as a strict output-correctness fix should be.

## Corpus-wide re-verification (the number that settles it)

**Before.** Scanning all 803 stored `kuna_*.c` artifacts (89,143 function blocks) with a
register-name matcher over comment- and string-stripped text:

| form | occurrences | functions |
|---|---|---|
| `PTRSUB(RSP, …)` | 277 | 259 |
| `PTRSUB(ESP, …)` | 6 | (in the 259) |
| `PTRSUB(sp, …)` (ARM Cortex-M) | 12 | 8 |
| **total** | **295** | **267** |

Every single occurrence is the operand of a `PTRSUB`. There is no other form.

**After.** I re-decompiled **all 267** of those functions on today's build — 53 binaries,
`decompile-all --json --addr …` per binary, default flags:

```
stored raw-reg functions: 259                       (x86/x86-64 subset)
  re-decompiled clean (no raw register today): 259   (of which now print &Stack<hex>: 259)
  STILL raw register today: 0
  no output today: 0
```

plus the 8 ARM `PTRSUB(sp,…)` functions checked individually (betaflight ×3, crazyflie,
nuttx, riot-os ×2) — e.g. betaflight `0x802181c`:

```c
  *(unsigned int *)&(&Stackfffffff0)[v3 * 4 + v1] = 0xffffffff;   // was PTRSUB(sp,-0x10)
```

**267 / 267 closed.** Independently, a fresh sweep of 109 `decompile-all` runs
(**83,168 functions**, x86-64 / i386-PE / ARM Thumb) finds **0** functions leaking a
register through the PTRSUB path.

## Analysis

### The mechanism was (a) — the `PTRSUB` form, closed by #226

`PrintC::op_ptrsub_ir`'s SPACEBASE arm bailed to `op_func_ir` when no Symbol covered the
offset, which prints the opcode as a call token and pushes input 0 — the spacebase
register varnode — through `push_vn_explicit_ir` → `get_register_name` → `RSP`/`ESP`/`sp`.
PR #226 (`694ab8ef`, "fix(p9): (NOVEL) render unnamed spacebase refs as stack locations")
added `push_spacebase_unnamed_ir`, the upstream `PrintC::pushUnnamedLocation` leaf, so the
arm now names the storage instead of printing the operator. `git log -S
push_spacebase_unnamed_ir` returns exactly that one commit.

### It is **not** (b), the copy-shadow/printer diagnosis — refuted for both rows

The filed diagnosis (raw `EAX`/`Unique` on `fmt/main` = a register↔global copy-shadow
merge rendered by member address instead of by high symbol, in
`printc.rs::push_vn_explicit_ir`) does not apply here:

- The copy-shadow mirror already exists in-tree and is a *different* branch — the
  `renders_as_global` loop in the unnamed-location tail of `push_vn_explicit_ir`
  (`printc.rs:6913-6959`), which walks the high's instances looking for an addr-tied
  **global** member and re-renders as `dat_<addr>`. Its output token is `dat_<hex>`,
  never `&Stack<hex>`.
- All 295 corpus occurrences were syntactically inside `PTRSUB(...)`, i.e. an *operand of
  an op render*, not a leaf reached through the high-name path at all.
- Instrumented confirmation: `decomp_dbg` `print raw` on `find` `sub_28e28`
  (`O2-noinline/findutils`) shows the two shapes explicitly —

  ```
  0x0002943b:5cca:  u0x0000a600(...) = ->(RSP(i),#0xc7)      <- PTRSUB, renders &Stack00000000000000c7
  0x00028f69:5e2e:  u0x10001682(...) = (cast)(RSP(i))        <- CAST,   renders (int8)RSP
  0x00028f69:5cb7:  u0x1000168a(...) = u0x10001682 + #0xae
  ```

  The `->` (PTRSUB) op now prints a stack location; the `(cast)` op still prints `RSP`.
  Two different printer paths on the *same* input varnode.

### Owning phase

P9 emit for the closed class (`p9_emit/printc.rs`, `op_ptrsub_ir` SPACEBASE arm). The
residual below is not P9's.

## The residual: 28 functions, a third mechanism (not #226, not the copy-shadow)

The fresh 83,168-function sweep finds **28 functions (0.034%)** that still print an
undeclared register identifier — via a different route:

| project | functions | form |
|---|---|---|
| betaflight | 9 | `(char *)((int4)sp + 6)` |
| crazyflie | 6 | `(char *)((int4)sp + 6)` |
| u-boot | 4 | `*(undefined0 **)(a2 - 0xc) = sp;` |
| cleanflight | 3 | `(char *)((int4)sp + 6)` |
| nuttx | 3 | `*(undefined0 **)a0 = sp;` |
| findutils (`find`, x86-64) | 2 | `v5 = (char *)((int8)RSP + 0xae);` |
| libopencm3 | 1 | `(char *)((int4)sp + 6)` |

85 occurrences total. Root cause, from the `print raw` transcript above: the frame
reference **stayed an `INT_ADD` on a `CAST` of the spacebase input** instead of being
normalized to a spacebase `PTRSUB`. The bare input varnode then reaches
`push_vn_explicit_ir`'s no-bound-name tail. It has no name because
`HighVariable::has_name` (`p6_variables/variable.rs:883-886`) returns `false` for the
unaffected stack-pointer input — a *faithful* port of upstream — and
`kuna_unnamed_location_name` (`printc.rs:8451-8460`) then returns the register name.

So the leaf render is upstream-faithful; the divergence is upstream that reference would
already be a PTRSUB. That makes this a **P5/P3 pointer-arithmetic normalization** gap
(the `INT_ADD(spacebase, const)` → `PTRSUB` rewrite blocked by an interposed CAST), with
only a *containment* available in P9 — and neither containment is clean: naming the leaf
`in_RSP` would diverge from `has_name`, and rendering it `&Stack00000000` would still be
an undeclared identifier. **No one-module mechanism, so this residual is
`needs-proposal`, not a feature candidate** — and at 0.034% breadth it does not earn a
proposal yet.

For scale, the same scanner over the *stored* corpus per decompiler (undeclared register
identifiers; `DH`/`pc` hits are string-literal false positives in every column):

| decompiler | functions | leaking |
|---|---|---|
| angr | 101,140 | ~0 |
| ghidra | 77,262 | 0 (66 = `DH` in gnutls strings) |
| ida | 88,725 | 51 (`R8`/`R9`/`R10`/`R11`, real) |
| binja | 61,096 | 7,450 (`r4`–`r15`, real) |
| kuna (stored, pre-#226) | 89,143 | 267 |
| **kuna (fresh, today)** | **83,168** | **28** |

#226 moved kuna from worse-than-IDA to better-than-IDA on this symptom.

## What actually replaced it, and why that is the bigger case

`&Stack00000008` is Ghidra-parity (`&stack0x00000008`) but it is **still an undeclared
identifier**. Fresh corpus: **7,381 occurrences across 1,248 functions (1.5%)**. And it is
not alone — a gcc-verified taxonomy of undeclared identifiers over the 83,168-function
fresh sweep:

| bucket | occurrences | functions |
|---|---|---|
| `dat_<hex>` globals | 221,159 | 24,350 |
| `label_<hex>` (implicit, fine) | 60,065 | 6,273 |
| libc externs (`stderr`, `optind`, …) | 13,486 | 7,478 |
| **`Stack<hex>` unnamed locations** | **7,381** | **1,248** |
| `sub_<hex>` used as a value | 3,072 | 1,279 |
| `name._4_4_` member tokens | 1,886 | 400 |

Confirmed with a real compiler on a whole-project export (which is the artifact that is
*supposed* to compile):

```
$ kuna decompile-project .../O2-noinline/sysvinit/stripped/utmpdump -o /tmp/proj
$ gcc -fsyntax-only -w /tmp/proj/utmpdump.c
error: 'dat_3fe8' undeclared (first use in this function)
error: 'Stack0000000000000054' undeclared (first use in this function)
error: 'Stack0000000000000010' undeclared (first use in this function)
error: 'stderr' undeclared (first use in this function)
...
```

`utmpdump.h` (the "recompile prelude") declares **neither** the `dat_<hex>` globals nor
the `Stack<hex>` locations. That is the live invalid-C lever now, at 20× the breadth of
anything in this track, and it is a **P9 declaration-emission / project-export** case, not
a naming case. It deserves its own record; flagging it here rather than folding it into a
track whose own symptom is closed.

## Proposed fix

None for this track. Both rows are `already-fixed` by #226 (`694ab8ef`); the record's
`status` for `O2-noinline-mydoom-mydoom-scan_textfile` should move from
`feature-candidate` to `already-fixed` and `docs/decbench/features.md`'s
`spacebase-unnamed-location` row from "PR in flight" to shipped.

Two follow-ups worth their own cases, in priority order:

1. **`dat_<hex>` / `Stack<hex>` are never declared** — 24,350 and 1,248 functions
   respectively; the project export does not compile. P9 emit + project export.
2. **Spacebase `INT_ADD`-through-`CAST` never becomes a `PTRSUB`** — 28 functions,
   85 occurrences, concentrated in ARM Cortex-M firmware. P5/P3. `needs-proposal`,
   low priority at this breadth.
