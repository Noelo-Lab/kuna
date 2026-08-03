---
case_id: loose-undefname-leak
pool: novel
group_id: iproute2::ip (filed) / tar::xstrcat (best witness)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: reproduces on today's build and is WORSE than filed — the `$$undefNNNNNNNN` identifier is not a cosmetic placeholder, it is a SECOND identifier for a stack Symbol that is already declared under its `vN` name, so the emitted C reads a variable that is never written (221 of 283 affected functions) while the real update lands on the other name
option_closing: null
feature_slug: undefname-leak
scope: small
confidence: high
---

## Summary

`$$undefNNNNNNNN` is Ghidra's `Scope::buildUndefinedName()` placeholder for a Symbol
created with an empty name (`database.cc:2854`). It is supposed to be invisible: upstream
`ActionNameVars::apply` renames every such Symbol before the printer runs
(`coreaction.cc:3075-3079`), and `PrintC` reads `sym->getDisplayName()` **live at emit
time**, so it can never print a stale name.

kuna binds the name **per HighVariable** (`HighVariable::kuna_name`, a cached `String`) and
the printer reads that cache. Two consequences, both live today:

* **(A) stale cache — 264 of 283 affected functions.** One ScopeLocal Symbol, two
  HighVariables. The partial-cover high is visited first, caches `$$undef000000NN`; the
  whole-cover high is visited later and renames the *same* Symbol to `vN`. Nothing
  back-fills the first cache, so one stack slot is declared **twice, under two different
  identifiers**, and the body uses both.
* **(B) never renamed — 19 functions in the sample, 101 Symbols across 72 functions.**
  No high reaches the whole-cover rename gate at all, so the Symbol keeps `$$undefNN` in
  the database. kuna's port of the upstream catch-all,
  `Database::assign_default_names` (`p0_knowledge/database.rs:4031`), **has zero call
  sites in the entire tree** — the `coreaction.cc:3079` line was never wired.

(A) is the correctness story. (B) additionally leaks the placeholder out of the C into the
structured surfaces (`decompile-all --json` `variables`, `decompile-project`'s `.asm`
stack-frame comments), where consumers read it as a variable *name*.

## Verify-first transcript (today's build, main @ bb2bdb9a)

### The filed case still reproduces

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ kuna decompile-all ~/github/decbench/results/full_run/O2-noinline/iproute2/stripped/ip --json
  -> 1912 functions, 95 occurrences of `$$undef[0-9a-f]{8}` across 33 functions
```

Filed observation was 92 in `ip`; the stored 2026-07-27 pane has 47 (the run stores a
sampled subset). Unchanged by the `rodata-phantom-store` work, as filed.

`ip` is 722 KiB, so a no-flag run is `--mode reliable` — i.e. **the benchmark's own option
surface**. Mode makes no difference:

```
$ kuna decompile <ip> --addr 0x2b420 --mode reliable   # and aggressive, and fast
unsigned long sub_2b420(unsigned long a0,unsigned int a1) // return-dupe x2
{
  char $$undef00000001 [16]; // stack - 0x38
  int4 v1; // eax
  char v2 [16]; // stack - 0x48
  int8 v3; // fs_offset
  char v4 [12]; // stack - 0x34
  ...
  v4[0] = SUB1612(0,4);
  $$undef00000001 = CONCAT124(v4[0],a1);
```

### Option sweep: 84 non-default option values, none closes it

```
$ # every non-default value of every one of the 83 catalog options, on tar::xstrcat
$ bash sweep.sh | grep -v ': 3$'
(no output — all 84 flips leave exactly 3 occurrences)
```

Explicitly checked, per the loop's standing instruction: `--option condfold on`,
`--option condfold wide`, `--mode aggressive` (the 21-option preset), `namestyle ghidra`,
`dedupvardecls off`, `realtypes off`. All 3/3. **Not covered by an option.**

## The correctness witness — `tar::xstrcat` (O2, `sub_611a0`, DWARF name `xstrcat`)

Fresh, no flags (518 KiB ⇒ `reliable`):

```c
char * sub_611a0(int8 a0,uint8 *a1)
{
  uint4 $$undef00000000; // stack - 0x58     <-- declared, READ TWICE, NEVER WRITTEN
  ...
  uint8 v12; // stack - 0x58                 <-- SAME SLOT, written twice, read once
  unsigned long *v13; // stack - 0x50

  v12 = *a1;
  v13 = (unsigned long *)a1[1];
  v6 = a1[2];
  if (a0) { // branch-flip
    v8 = 0;
    v9 = a0;
    do {
      v4 = v12 & 0xffffffff;
      if (0x30 <= (uint4)$$undef00000000) { // branch-flip
        v1 = &v13[1];
        v7 = v13;
        v13 = v1;
      }
      else {
        v12 = (uint8)((uint4)$$undef00000000 + 8);
        v7 = (unsigned long *)(v4 + v6);
      }
      ...
```

`a1` is a `va_list`; the loop is the x86-64 `va_arg` expansion
(`if (gp_offset >= 48) use overflow_arg_area else reg_save_area + gp_offset;
gp_offset += 8`). As emitted, the loop **tests and increments an uninitialised
variable** and the induction update it does perform (`v12 = ...`) is never read again.
Recompiled, this loop does not terminate correctly. This is wrong code, not a spelling
problem.

### Instrumented proof that the two identifiers are ONE Symbol

`print map` / `print high` are engine stubs in `decomp_dbg`, so the discriminator used was
the console `rename`, which renames a **Symbol** in the local scope:

```
$ decomp_dbg
[decomp]> load file .../O2/tar/stripped/tar
[decomp]> load addr 0x611a0
[decomp]> decompile
[decomp]> rename v12 QQQ
[decomp]> decompile
[decomp]> print C

char * sub_611a0(int8 a0,uint8 *a1)
{
  uint8 QQQ; // stack - 0x58          <-- ONE declaration now
  unsigned long *v12; // stack - 0x50
  ...
      v4 = QQQ & 0xffffffff;
      if (0x30 <= (uint4)QQQ) { ... }
      else { QQQ = (uint8)((uint4)QQQ + 8); ... }
```

Renaming the **one** Symbol collapsed *both* identifiers and the loop became correct;
everything else in the body is byte-identical. Repeated on `iproute2::ip sub_10290`
(`rename v7 ZZZ` ⇒ `char ZZZ [16]` once, `ZZZ[0] = 0; ... v2 = ZZZ[0];` instead of
`v7[0] = 0; ... v2 = $$undef00000001[0];`). Two binaries, same result.

That rules out the "two distinct Symbols at one address" hypothesis (a
`handleSymbolConflict` spawn) and pins mechanism (A).

### Instrumented proof of mechanism (B)

`kuna decompile-all --json` exposes `extract_variables`
(`infra/decompile_drive.rs:1193-1211`), which reads the ScopeLocal **Symbol table**
directly. For `ip sub_2b420` that table itself carries the placeholder:

```json
{"name": "$$undef00000001", "type": "unsigned int", "kind": "stack",
 "stack_offset": -56, "size": 4}
```

— whereas for `sub_10290` (mechanism A) the table is clean (`v7` only) even though the C
prints `$$undef00000001`. That split is the discriminator used for the corpus counts below.

Same leak in the project export (`kuna decompile-project`):

```
lcd-serial.elf.c:2327:  unsigned int $$undef00000002; // stack - 0x30
lcd-serial.elf.c:2443:      return sub_80030c8((unsigned int)$$undef00000002,v80._4_4_,v37,v73);
lcd-serial.elf.asm:2897:; stack: $$undef00000005 @ [stack-0x158] (int4[34])
```

## Source

`iproute2 ip sub_2b420` is `set_qlen` (`iplink.c`):

```c
static int set_qlen(const char *dev, int qlen)
{
  struct ifreq ifr = { .ifr_ifru.ifru_ivalue = qlen };
  int s;
  s = get_ctl_fd();
  if (s < 0) return -1;
  strlcpy(ifr.ifr_ifrn.ifrn_name, dev, 16);
  if (ioctl(s, 0x8943, &ifr) < 0) { perror("SIOCSIFXQLEN"); close(s); return -1; }
  close(s);
  return 0;
}
```

One `struct ifreq ifr`. kuna splits it into `char v2[16]` (the name field, `stack - 0x48`)
plus a 4-byte slot at `stack - 0x38` (the `ifru_ivalue` union member) that never gets a
name — the frame layout is a separate, known issue (the "one 16-byte buffer split across
three symbols" loose thread); what this record is about is that the un-named piece is
*printed*.

`tar sub_611a0` = `xstrcat` (`readelf -sW .../O2/tar/compiled/tar` ⇒
`743: 00000000000611a0 341 FUNC LOCAL DEFAULT 16 xstrcat`) — a varargs string
concatenator; its source file is not in the vendored `.i` set, but the `va_arg` shape is
unambiguous in the binary.

## Breadth (corpus-wide, measured)

Fresh `kuna decompile-all --json` over a stratified sample: **101 stripped binaries**
(one per project × {O0, O2, O2-noinline}), **78,787 functions**, today's build, no flags:

| measure | value |
|---|---|
| binaries with ≥1 leaked identifier | **58 / 101 (57%)** |
| functions with ≥1 leaked identifier in the emitted C | **283 (0.36%)** |
| total `$$undefNNNNNNNN` occurrences in emitted C | **1,573** (1,030 reads / 141 writes) |
| functions where the placeholder is declared next to a same-storage `vN` (mechanism A) | **259** |
| functions where a placeholder is **read and never written** (uninitialised read) | **221** |
| functions whose ScopeLocal Symbol table itself carries a placeholder (mechanism B) | **72 (101 Symbols)** |
| mechanism split (functions) | A only 264 · B only 11 · both 8 |

Top projects by leaking functions: e2fsprogs 45, betaflight 42, iproute2 34, tar 28,
libedit 22, cleanflight 21, findutils 12, rsyslog 10, grep 8, gzip 8, openssh 8. Both
x86-64 and ARM Cortex-M, all three optimisation levels.

**Nobody else does this.** Across the full stored 2026-07-27 run (803 binaries × 11
decompilers):

```
kuna:    152 of 803 stored panes contain $$undef
angr:      0 / ida: 0 / ghidra: 0 / binja: 0 / phoenix: 0 / dewolf: 0 / r2dec: 0
```

GED value: **zero** — an identifier rename never changes the CFG. This is a Tier-0/Tier-1
correctness item in the `features.md` sense, mined out of the pool that does not rank by
margin.

Regression risk today: **none measured.** Full `KUNA_DUMP=1` renders of both test corpora
grep clean for `$$undef` — datatests 675/675 assertions, 0 occurrences; stages 356/356,
0 occurrences. (The `verify_w10_spacebase_render.rs` comment claiming "the whole corpus
carries exactly ONE pre-existing `&$$undef` in `forloop_thruspecial`" is stale — that stem
now renders `int4 *v2; // stack - 0x28`.)

## Analysis — root cause

Owning phase: **P6** (variable & storage model — `ActionNameVars` lives in
`p6_variables/`). The symptom appears at P9, but P9 only reads a cache P6 wrote; per
`docs/phases.md`, "a symptom's phase is not always its decision's phase".

The divergence from upstream is already documented in kuna's own source, one paragraph
short of the bug (`p6_variables/coreaction_cleanup.rs:2777-2790`):

> ORDER (the load-bearing detail): in C++ a single shared `Symbol` object is attached to
> BOTH the offset-constant high and the stack-slot high, and the undefined ones are
> renamed ONCE at the end of `apply` … The render reads `getSymbol()->getDisplayName()` at
> print time, so it always sees the FINAL name. **The kuna model binds the name PER-HIGH
> off the database Symbol**, so the spacebase pass must run AFTER the main naming loop …

The port fixed that ordering hazard for the *spacebase* pass. The same hazard **inside**
the main loop, between two highs of the same Symbol, was not fixed:

1. `p6_variables/varmap.rs:1520` `resolve_default_name_override` renames the covering
   Symbol only when it is `is_name_undefined() && sym_off == 0 && size == entry_size` —
   the faithful port of the C++ `namerec` gate (`sym->isNameUndefined() &&
   high->getSymbolOffset() < 0`, `coreaction.cc:3046`). On a miss it returns
   `symbol.get_display_name()` **as-is**, i.e. `$$undefNN`.
2. `p6_variables/coreaction_cleanup.rs:2637` `h.set_kuna_name(sym_name)` caches whatever
   that returned onto the HighVariable, permanently.
3. `p9_emit/printc.rs:2394` (`emit_local_var_decls`) and `printc.rs:6728` (the body
   render) read `h.kuna_name()`. Upstream `PrintC::emitVarDecl`/`pushSymbol` read
   `sym->getDisplayName()`. The deferral that is harmless in C++ is not harmless here.
4. `p0_knowledge/database.rs:4031` `Database::assign_default_names` — a complete,
   tested port of `ScopeInternal::assignDefaultNames` — is **never called**. Upstream calls
   it as the last statement of `ActionNameVars::apply`
   (`coreaction.cc:3079: data.getScopeLocal()->assignDefaultNames(base);`), which is what
   guarantees no `$$undef` Symbol survives the pass. `grep -rn assign_default_names
   decompiler/` returns the definition and nothing else.

So (A) is step 2+3 (cache written before the rename that step 1 defers to a later high),
and (B) is step 4 (the catch-all was ported but not wired).

## Proposed fix — `undefname-leak`, one module, no option

All of it lands at the **tail of `name_local_highs_angr`**
(`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs:2378-2799`), after
the existing spacebase pass — the same place the port already chose to resolve this class
of ordering hazard:

1. **Wire the catch-all (mechanism B).** Call the already-ported
   `Database::assign_default_names(scope_local, &mut base, arch)` — a one-line
   `Funcdata`/`ScopeLocal` accessor away — reproducing `coreaction.cc:3079`. Every
   remaining `$$undef` Symbol gets its `buildDefaultName`. This also cleans
   `extract_variables` and the project `.asm` annotations for free, because those read the
   Symbol table.
2. **Repair the stale caches (mechanism A).** Re-walk the highs; for any whose
   `kuna_name()` matches the placeholder shape (`len == 15 && starts_with("$$undef")`,
   i.e. `Symbol::is_name_undefined`), re-resolve the covering Symbol through the **existing**
   `ScopeLocal::query_container_for_link(addr, usepoint)` (`p6_variables/varmap.rs:1638`,
   which already returns `display_name`) on the high's name representative, and
   `set_kuna_name` to the now-final name. This is exactly upstream's "read the Symbol name
   at print time", applied once at the end of the pass.

**Why this ordering is the zero-churn form.** Neither step can renumber an existing local:
step 1 runs after every `vN` has been handed out and only names Symbols that had none;
step 2 consumes no `base` at all. The tempting one-liner — drop the
`sym_off == 0 && size == entry_size` gate in `resolve_default_name_override` so the first
touch always renames — would work too, but it moves *when* a Symbol consumes `base`, which
renumbers every later local in any function where a partial high precedes its whole-cover
sibling. Prefer the repair form; keep the C++-faithful gate.

Owning files:

* `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs` — the two-step
  tail in `name_local_highs_angr` (the whole feature).
* `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs` (or `funcdata_varnode.rs`) —
  one thin accessor to reach `Database::assign_default_names` on the local scope.
* No change to `p9_emit/printc.rs`.

**No option.** This is a strict bug fix that only corrects wrong output (AGENTS.md
*Adding features*: "a strict bug fix that only corrects wrong output needs no flag"), of
the same family as `spacebase-unnamed-location` and `symbol-keyed-local-decls`, both of
which shipped unflagged. It ships with a `tests/stages/ghdec-undefname-leak.xml` two-pass
testcase and a DIV row.

Risks:

* `assign_default_names` walks `nametree` by the `$$undef` name prefix; kuna's `NameKey`
  ordering must place every placeholder in one contiguous run (it does —
  `database.rs:4044-4048` already relies on this and is unit-tested at `database.rs:4286`).
* Step 2 must skip highs bound through the **global** scope (`kuna_global`) and
  proto-partial pieces, which take their name from a different query; the existing
  `query_container_for_link` local-first / global-fallback split already handles that.
* A Symbol renamed in step 1 gets a `vN` number higher than every other local in the
  function. Cosmetic; the alternative (renumbering) is worse.

Measurement for the PR: re-run the 101-binary sweep and require **0** occurrences of
`$$undef` in emitted C and in `--json` `variables`; datatests 675/675 and stages 356/356
byte-identical (both already grep clean, so any diff is a regression); `scripts.pipeline.timeit`
within the 5% budget (the added work is one scope walk plus one high walk per function).
GED delta is expected to be exactly 0 — this item is worth doing for the 221 functions
whose C currently reads an uninitialised variable, not for the metric.
