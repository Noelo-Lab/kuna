---
case_id: O2-noinline-cleanflight-cleanflight_DALRCF405-ftoa
pool: novel
group_id: cleanflight::ftoa
status: feature-candidate
tier: N
margin: 0
fresh_verdict: reproduces on HEAD e38ffc31 in every mode — one 4-byte stack slot carries SEVEN overlapping declarations (`v2`/`v5`/`v6`/`v7` all `// stack - 0x28`, `v8` `-0x27`, `v9` `-0x26`, `v10` `-0x25`) and the body READS `v8` twice and `v5` once on paths where neither is ever assigned; the P6 symbol map is CORRECT (instrumented: `ScopeLocal::restructure` builds exactly one `xunknown4 @ -0x28` entry, ghidra's `local_28`), and the defect is one predicate in `ActionNameVars::linkSymbols` (`narrower_addrtied_local`, coreaction_cleanup.rs:2567) refusing to reuse it — an env-gated ablation of that predicate reproduces Ghidra's render exactly at 675/675 + 394/394 and 13 changed functions in 24,379
option_closing: null
feature_slug: tiedpartialname
scope: small
confidence: high
---

## Verdict in one line

The `concat`/`subpiece`/`undefined` counters on this row are **inherited and do not
move** — what is kuna-specific here is that a partial write to a mapped scalar stack
local is named as its **own local** instead of a **field of the parent symbol**, which
makes the emitted C **read variables that are never assigned**. One predicate, one
module, measured green.

## Reproduction (today's build, pasted)

`make binaries` at HEAD `e38ffc31`. The stripped binary is 346,852 bytes (< 500 KiB), so a
no-flag run resolves `auto -> aggressive`.

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ ./decompiler/target/release/kuna decompile \
    ~/github/decbench/results/full_run/O2-noinline/cleanflight/stripped/cleanflight_DALRCF405.elf \
    --addr 0x800f04c

int4 sub_800f04c(float4 a0,int4 a1) // ternary x2
{
  unsigned int v1;
  char v10; // stack - 0x25
  unsigned int v11; // stack - 0x24
  unsigned int v12; // stack - 0x20
  unsigned int v2; // stack - 0x28
  char v3 [16];
  uint4 v4; // r4
  char v5; // stack - 0x28
  unsigned short v6; // stack - 0x28
  undefined3 v7; // stack - 0x28
  undefined3 v8; // stack - 0x27
  unsigned short v9; // stack - 0x26

  v2 = 0;
  v11 = 0;
  v12 = 0;
  a0 = (a0 <= 0.0) ? a0 - 0.0005 : a0 + 0.0005;
  v4 = (uint4)(a0 * 1000.0);
  sub_800f018((v4 ^ (int4)v4 >> 0x1f) - ((int4)v4 >> 0x1f),v3,10);
  v2 = (0 <= (int4)v4) ? CONCAT31(v8,0x20) : CONCAT31(v8,0x2d); // branch-flip   <-- v8 NEVER ASSIGNED
  if (sub_8008a00(v3) != 1) { // branch-flip
    v1 = v2;
    if (sub_8008a00(v3) != 2) { // branch-flip
      if (sub_8008a00(v3) != 3) // branch-flip
        sub_80088dc(&v2,v3);
      else {
        v9 = SUB42(v2,2);
        v5 = (char)v2;
        v6 = CONCAT11(0x30,v5);
        v2 = CONCAT22(v9,v6);
        sub_80088dc(&v2,v3);
      }
    }
    else {
      v5 = (char)v2;
      v6 = CONCAT11(0x30,v5);
      v10 = SUB41(v1,3);
      v7 = CONCAT12(0x30,v6);
      v2 = CONCAT13(v10,v7);
      sub_80088dc(&v2,v3);
    }
  }
  else {
    v6 = CONCAT11(0x30,v5);          <-- v5 NEVER ASSIGNED ON THIS PATH
    v7 = CONCAT12(0x30,v6);
    v2 = CONCAT13(0x30,v7);
    sub_80088dc(&v2,v3);
  }
  v4 = (uint4)(uint1)sub_8008a00(&v2) - 3 & 0xff;
  sub_8009014(a1,&v2,v4);
  *(char *)(a1 + v4) = 0;
  sub_80088dc(a1,".");
  sub_80088dc(a1,(int4)&v2 + v4);
  return a1;
}
```

`v8` (`undefined3 @ -0x27`) is read in both ternary arms and assigned nowhere; `v5`
(`char @ -0x28`) is read on the `strlen(intString1) == 1` path and assigned only on the
other two. Both are supposed to be **bytes of the same 4-byte slot as `v2`**, which IS
assigned — the aliasing is real in the IR and lost in the C.

Mode / option sweep — nothing closes it:

```
$ for m in auto aggressive; do kuna decompile ... --mode $m | grep -c '// stack - 0x2[5678]$'; done
7
7
$ kuna decompile-all <bin> --json --mode reliable      # the option surface the benchmark scored
  ... identical body, same 7 declarations ...
$ for o in <all 26 default-off catalog rows>; do kuna decompile ... --option $o on | grep -c ...; done
   -> 7 for every one of the 26
$ kuna decompile ... --option condfold on   -> 7
$ kuna decompile ... --option condfold wide -> 7
```

So `option_closing: null`, and the disagreement between `auto` and `reliable` is nil —
this is a code-level defect, not a mode difference.

> Side observation, not this track: `kuna decompile --addr 0x800f04c --mode reliable` and
> `--mode fast` **fail to load the function at all** on this Cortex-M binary
> (`Execution error: Unable to load 512 bytes at r0x0644a8dc`) while `--mode auto`/
> `aggressive` and `decompile-all --mode reliable` succeed. The single-function
> `load addr` path depends on an analysis option carried only by the aggressive preset.

## Side-by-side

**ghidra** (stored pane, same binary, same address) — one symbol, partial fields:

```c
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined1 auStack_1c [16];
  ...
  if ((int)uVar4 < 0) { local_28 = CONCAT31(local_28._1_3_,0x2d); }
  else                { local_28 = CONCAT31(local_28._1_3_,0x20); }
  ...
    local_28._0_2_ = CONCAT11(0x30,(undefined1)local_28);
    local_28._0_3_ = CONCAT12(0x30,(undefined2)local_28);
    local_28       = CONCAT13(0x30,(undefined3)local_28);
  ...
      local_28._3_1_ = SUB41(uVar1,3);
```

8 `CONCAT` + 1 `SUB41` + 2 `undefinedN` casts — i.e. **ghidra emits the same operator
soup**; what it does not emit is a second name for the same bytes.

**ida** — `int v7[3]` plus `LOBYTE/BYTE1/HIBYTE` macros, 0 CONCAT.
**angr** — declares `char v1/v2/v3` at `-0x27/-0x26/-0x25` and **assigns each of them**
(`v1 = 48;`), so its separate-names model is at least self-consistent; 0 CONCAT.
**binja** — `*(&var_28 + 1) = 0x30`, 0 CONCAT.

**kuna with the one-predicate ablation applied** (private build, see *Analysis*):

```c
  unsigned int v1;
  unsigned int v2; // stack - 0x28
  char v3 [16];
  uint4 v4; // r4
  unsigned int v5; // stack - 0x24
  unsigned int v6; // stack - 0x20

  v2 = (0 <= (int4)v4) ? CONCAT31(v2._1_3_,0x20) : CONCAT31(v2._1_3_,0x2d);
  ...
        v2._2_2_ = SUB42(v2,2);
        v2._0_1_ = (char)v2;
        v2._0_2_ = CONCAT11(0x30,(char)v2);
        v2 = CONCAT22(v2._2_2_,(unsigned short)v2);
  ...
    v2._0_2_ = CONCAT11(0x30,(char)v2);
    v2._0_3_ = CONCAT12(0x30,(unsigned short)v2);
    v2 = CONCAT13(0x30,(undefined3)v2);
```

Ghidra's shape exactly: 3 stack declarations instead of 9, every partial read
re-derived from the parent, **no read of an unassigned variable**.

## Source

`~/github/decbench/results/full_run/O0/cleanflight/compiled/typeconversion.i:1239`:

```c
char *ftoa(float x, char *floatString)
{
    int32_t value;
    char intString1[12];
    char intString2[12] = { 0, };
    ...
    if (value >= 0) intString2[0] = ' ';
    else            intString2[0] = '-';

    if (strlen(intString1) == 1) {
        intString2[1] = '0'; intString2[2] = '0'; intString2[3] = '0';
        strcat(intString2, intString1);
    } else if (strlen(intString1) == 2) { ... }
```

The `char intString2[12] = {0,}` zero-init compiles to three word stores, so **every**
Ghidra-family decompiler models the array as three 4-byte scalars and turns the
subsequent byte writes into read-modify-write `CONCAT` chains. That reassembly failure is
**inherited** (ghidra 8 CONCAT vs kuna 10 on this function) and is not what this record
files. What the source makes unambiguous is that `intString2[0]` has a defined value
(`' '` or `'-'`) at the point kuna reads `v5`, and that `intString2[1..3]` are zero at the
point kuna reads `v8` — kuna's C says otherwise.

## Analysis (instrumented, not read)

Owning phase: **P6 — Variable & Storage Model** (`docs/phases.md`); the symptom is visible
at P9 but the decision is P6's `ActionNameVars`.

**Step 1 — the symbol map is CORRECT.** `ScopeLocal::restructure` already has an
env-gated dump (`KUNA_DBG_VARMAP`, varmap.rs:1382/1399). On this function it builds
exactly the three entries ghidra has:

```
$ KUNA_DBG_VARMAP=1 decomp_dbg < probe.dbg 2>&1 >/dev/null | grep ENTRY
[varmap] ENTRY addr=0xffffffd8 size=4 type=xunknown4     <-- ghidra's local_28
[varmap] ENTRY addr=0xffffffdc size=4 type=
[varmap] ENTRY addr=0xffffffe0 size=4 type=
[varmap] ENTRY addr=0xffffffe4 size=16 type=
```

All seven sub-ranges (`-0x28:1/2/3/4`, `-0x27:3`, `-0x26:2`, `-0x25:1` — visible as
distinct `RangeHint`s in the same dump) are absorbed into the single 4-byte entry. So this
is **not** a `MapState`/`RangeHint` defect, and it is not the `subright` thread either:
the final IR carries the partials as ordinary addr-tied stack varnodes
(`print raw`: `s0xffffffd9:3 = SUB43(s0xffffffd8,#0x1)`, `s0xffffffd8:1 = SUB41(...)`).

Independent corroboration from the product surface: `kuna decompile-all --json`'s
`variables` array for this function lists **only** `v2`, `v11`, `v12`, `v3` — the four
real symbols. `v5`…`v10` are declared in the body with no symbol behind them.

**Step 2 — the naming pass refuses to reuse the entry.** Instrumenting the
`handleSymbolConflict` port in `ActionNameVars::linkSymbols`
(`p6_variables/coreaction_cleanup.rs:2540-2614`):

```
[name] addr=0xffffffd8 size=4 tied=true ... entry_size=4 narrower=false reuse=true  conflict=false entry_name=v2
[name] addr=0xffffffd8 size=1 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
[name] addr=0xffffffd8 size=2 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
[name] addr=0xffffffd8 size=3 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
[name] addr=0xffffffd9 size=3 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
[name] addr=0xffffffda size=2 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
[name] addr=0xffffffdb size=1 tied=true ... entry_size=4 narrower=true  reuse=false conflict=true  entry_name=v2
```

Every partial finds the right entry (`entry_name=v2`) and is then rejected by
`narrower_addrtied_local` (coreaction_cleanup.rs:2567) — an **addr-tied, non-input,
non-persist, non-constant Varnode strictly narrower than a non-composite containing
entry**. That flips `reuse_directly` off and runs the conflict scan, which finds a 4-byte
varnode at `-0x28` in a different HighVariable:

```
[name]   scan other_vn_addr=0xffffffd8 size=4 same_group=false oh_has_piece=false high_has_piece=true
[name]   scan other_vn_addr=0xffffffd8 size=4 same_group=true  oh_has_piece=true  high_has_piece=true
```

The scan's escape hatch (`is_same_group`, the `Merge::mergeAddrTied` -> `groupWith`
`VariableGroup` test) saves the second candidate but not the first: **one of the 4-byte
highs at that address was never given a `VariablePiece`**, so `conflict = true` and the
partial falls through to the fresh-`vN` tail and is given its own Symbol.

That the escape hatch *can* work is visible in the same binary: in
`applyLedFixedLayers` (`0x8030a88`) the two partials of the `-0xe4` slot come back
`narrower=true, conflict=false` and render correctly as `v19._2_1_` / `v19._3_1_`, while
the `-0xdc` slot in the same function comes back `conflict=true` and splits into five
names. Same predicate, two outcomes, decided by whether the merge happened to group the
highs.

**Step 3 — why a name instead of a field turns into a read-before-def.** The
`SUB41(v2,0)` in the `strlen == 1` arm is an *implied* varnode: no statement is emitted for
it in either build (ghidra does not emit one either). C++
`PrintLanguage::pushVnImplied` prints an implied varnode through its **Symbol** when it has
one — the whole-symbol arm prints the bare name, the partial arm calls `pushPartialSymbol`.
With the entry reused, that is `(char)v2`, which re-derives the value and is always
correct. With a private Symbol it is the bare name `v5`, and because the definition was
elided there is nothing that ever assigns it. The identical chain explains `v8`
(`SUB43(v2,1)`, one implied use in each ternary arm).

**Step 4 — the divergence is kuna's, and its own witness no longer needs it.** Upstream
`Funcdata::handleSymbolConflict` (`funcdata_varnode.cc:1018`) returns the entry
**unconditionally** for `vn->isAddrTied()`; `narrower_addrtied_local` is a kuna carve-out
added for LOSS-234 `zeroprop` (the 1-byte char return overlapping the 4-byte `int4 *ptrint`
parameter). Instrumenting its own datatest, `tests/datatests/condconst2.xml`:

```
$ KUNA_DBG_NAME=1 decomp_test_dbg -path <dir-with-only-condconst2.xml> datatests 2>&1 >/dev/null | grep '\[name\]'
[name] addr=0x20 size=4 tied=false in=true ... narrower=false reuse=true conflict=false entry_name=ptrint
[name] addr=0x24 size=4 tied=false in=true ... narrower=false reuse=true conflict=false entry_name=val
   ... (six rows, every one in=true / narrower=false)
$ KUNA_DBG_VARMAP=1 ... | grep ENTRY     -> (no output: zeroprop has no stack entries at all)
```

The guard **never fires on the case it was written for**. Its char-return high has no
covering entry today and takes the no-symbol tail instead.

**Step 5 — ablation.** An env-gated `narrower_addrtied_local = false`, compiled into a
private binary (`KUNA_ABL_NARROWER`; the repository tree was restored and rebuilt
afterwards, `git status` clean):

| gate | control (same binary, env unset) | ablated |
|---|---|---|
| `kuna test --datatests --baseline docs/baseline.json` | 675/675, PARITY OK | **675/675, PARITY OK** |
| `kuna test --datatests --datatests-dir tests/stages --baseline docs/baseline-stages.json` | 394/394, PARITY OK | **394/394, PARITY OK** |

and on `ftoa` it produces the Ghidra-shaped pane quoted above.

## Breadth (corpus-wide)

**A/B `decompile-all` over 24,379 functions / 15 binaries / 5 arch-format families**
(ARM Cortex-M ELF, x86-64 ELF, x86-64 PIE, PE32, MIPS-family `mirai`):

| | value |
|---|---|
| functions compared | 24,379 |
| **functions changed by the ablation** | **13 (0.053%)** |
| stack-local declarations | 19,657 -> 19,620 (**-37**) |
| `sym._n_m_` partial-field renders | 691 -> **797** (+106) |
| `undefinedN` tokens | 257 -> 266 (+9) |

Per-binary: betaflight O0 4, cleanflight O2ni 2, e2fsck O2ni 2, ssh O2ni 2, bash O0 1,
ip O2ni 1, x0r-usb O2ni 1; crazyflie / nuttx / tar / ocsptool / mydoom / factor / usbmidi 0.

**Every one of the 13 diffs was read** (the standing "diff every changed function"
requirement). All 13 are the same shape — N overlapping declarations collapse to 1, and
their uses rebind to `parent._off_size_` / `(type)parent`. No statement is dropped, no
call or argument changes, no control flow changes, no function boundary moves. The
largest are `betaflight 0x8018378` and `cleanflight 0x800f04c` (both `ftoa`) and
`betaflight 0x8051f32`/`0x8051f38` at 9 -> 3 and 8 -> 3 declarations.

A static scan of the same 24,372 panes for the *symptom* (declared stack locals whose
byte ranges overlap) finds **35 functions (0.14%)**, of which **4** additionally read a
never-assigned overlapping local. Extrapolated to the benchmark's 94,575 functions that is
~135 functions carrying aliased declarations, ~50 the fix would change and ~16 with a
genuine read-before-def. **This is a narrow, correctness-shaped fix, not a volume fix** —
rank it the way round 1 ranked `spacebase-unnamed-location`, not by GED.

**GED delta: 0.** No basic block appears or disappears; the case is tied at 8 with ghidra
and ida already. The novel-pool wart counters do not move either (see below).

## What this does NOT fix (and what is disproven)

1. **The `concat` counter does not move.** kuna emits 10 `CONCAT` before and after; ghidra
   emits 8 on the identical statements. Confirms the census: CONCAT here is failed
   reassembly of source-level byte writes into a zero-initialised `char[12]`, which the
   whole Ghidra family fails the same way. Not a campaign item.
2. **The `subpiece` counter does not move** (2 before, 2 after) and this is *not* the
   `subright` thread (#251): these SUBPIECEs are at nonzero offsets of a **stack symbol**,
   which upstream's own addr-tied/`overlap == c` guard deliberately leaves alone.
3. **The `undefined` counter does not move** (2 before, 2 after): the two `undefined3 v7`
   / `undefined3 v8` declarations are replaced by two `(undefined3)v2` casts, which ghidra
   also emits. As the census already corrected, novel.md's `undefined` tag is the
   `undefinedN` **type token**, not the `$$undef` placeholder #257 removed.
4. **Not a metric artifact**: `approximated: false`, `degenerate_source: false`,
   `source_nodes: 16`, `source_edges: 21`, kuna 8 = ghidra 8 = ida 8, angr/binja 19. The
   row is in the novel pool because the pane is bad, not because the score is.
5. **Not closed by any option or mode**: all 26 default-off catalog rows, `condfold on`
   and `condfold wide`, and all four modes leave the seven declarations in place.

## Relation to the sibling records

`novel-concat-subpiece-soup.md` (the cluster record) already flagged the residue in
`applyLedFixedLayers` as "the P6 partial-symbol/merge thread, not `subright`". This record
names that thread and gives it a measured lever.

`novel-tiedslot-partial-symbol.md` triages the **same cluster** and lands on
`needs-proposal` with a different proximate cause: a skipped forced
`Merge::mergeAddrTied` at the slot. **The two findings are consistent, not competing** —
the skipped merge is precisely why a second, *ungrouped* 4-byte HighVariable exists at
`-0x28` for the conflict scan to trip over (`oh_has_piece=false` in the Step-2 transcript).
The difference is what each identifies as the actionable lever:

* the merge-side repair is the deeper one (it would also collapse the self-piece CONCAT
  rebuild traffic), and that record is right that upstream's `eliminateIntersect` cannot
  snip a full-block-cover intersection — hence `needs-proposal`;
* the **naming-side** repair is sufficient for the rendering defect on its own, is one
  predicate in one module, and is measured at 0/675 + 0/394 with 13 changed functions.

They should be sequenced, not merged: ship the naming predicate first (it is a strict
correctness fix and cannot regress the merge work), then treat the merge as a separate
proposal whose payoff is the CONCAT/SUBPIECE traffic this record explicitly does not claim.

## Proposed fix

* **Owning phase**: P6 (Variable & Storage Model) — `ActionNameVars` / the
  `Funcdata::linkSymbol` + `handleSymbolConflict` port.
* **One module, one predicate**:
  `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs:2567`. Restrict
  `narrower_addrtied_local` to the shape it was written for. The zeroprop witness is a
  narrower addr-tied local overlapping a wider **input/parameter** entry; the ftoa shape is
  a narrower addr-tied local overlapping a wider **ordinary mapped stack local**. Two
  candidate gates, in order of preference:
  1. require the containing entry's category to be a **parameter** (`info.category`, already
     fetched and currently discarded at the `let _ = info.category;` line) — the tightest
     restatement of LOSS-234;
  2. require the entry's space to be a **register** space rather than the stack spacebase —
     zeroprop's conflict is at `r0`, every case in this record is at a stack offset.
  The wholesale removal measured above is the upper bound of the behaviour change: if the
  tightened gate lands anywhere between "removal" and "today", it is bounded by 13
  functions in 24,379.
* **Option**: `tiedpartialname` (`on` = a narrower addr-tied Varnode reuses its containing
  scalar Symbol and renders `sym._off_size_`; `off` = today's fresh-`vN`). Per AGENTS.md
  this only ever changes emitted C, so gate it; the default-ON flip is carried by the
  0/675 ablation already measured.
* **Stage test**: `tests/stages/ghdec-tiedpartialname.xml` (no angr analog), two-pass — a
  4-byte stack slot zero-initialised then byte-poked; option off = `v5`/`v8` declared and
  read unassigned, default = `v2._1_3_` / `(char)v2` and three declarations.
* **Risks to measure on the PR**:
  (a) the `zeroprop` regression the gate was added to prevent — the datatest
  (`condconst2.xml`, assertion `\(char\)ptrint` `min=0 max=0`) is green under full removal
  today, but the tightened gate must be verified against it explicitly, not by the corpus;
  (b) the `undefinedN` token count rises slightly (257 -> 266 on the sample) because a
  partial that used to carry a concrete type now renders as an `(undefinedN)` cast of the
  parent — expected, matches ghidra, but it will show in a wart re-mine;
  (c) `x0r-usb 0x4036d0` and `bash 0x9c2e3` are the only two changed functions where the
  parent symbol is 8 bytes rather than 4 — re-read those two panes in the PR sweep;
  (d) speed — the change removes work (a scan is skipped), but run
  `scripts.pipeline.timeit` anyway for the ≤5% budget.

## Siblings (all verified to move together under the ablation)

| case | binary | address |
|---|---|---|
| `O0-cleanflight-cleanflight_DALRCF405-ftoa` | O0 cleanflight | `0x800effc` |
| `ftoa` (betaflight build of the same source) | O0 betaflight | `0x8018378` |
| `applyLedFixedLayers` | O2ni cleanflight | `0x8030a88` |
| unnamed | O0 betaflight | `0x8051f32`, `0x8051f38`, `0x802a338` |
| unnamed | O2ni e2fsck | `0x6d630`, `0x71860` |
| unnamed | O2ni ssh | `0x57110`, `0x61aa0` |
| unnamed | O2ni ip | `0x78dc0` |
| unnamed | O0 bash | `0x9c2e3` |
| unnamed | O2ni x0r-usb | `0x4036d0` |
