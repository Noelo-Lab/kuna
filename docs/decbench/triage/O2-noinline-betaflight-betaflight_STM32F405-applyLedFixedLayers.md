---
case_id: O2-noinline-betaflight-betaflight_STM32F405-applyLedFixedLayers
pool: novel
status: feature-candidate
tier: N
margin: 0
fresh_verdict: structurally unchanged from the stored run and already tied-best (kuna GED 25 == ghidra 25, vs angr 42 / ida 41); the one real kuna defect in the pane is that the single 4-byte `hsvColor_t color` stack slot is declared TWICE under the SAME name (`unsigned int v12;` and `uint4 v12;`, both `// stack - 0x24`), which is invalid C
option_closing: null
feature_slug: symbol-keyed-local-decls
scope: small
confidence: high
---

## Side-by-side

Metrics from the triage dump (identical structure across stored and fresh):

| output | loc | gotos | labels | switches | cases | ifs | loops |
|--------|---|---|---|---|---|---|---|
| ghidra(stored) | 144 | 2 | 2 | 1 | 5 | 10 | 2 |
| ida(stored) | 130 | 6 | 2 | 1 | 5 | 10 | 2 |
| kuna(stored) | 160 | 2 | 2 | 1 | 5 | 10 | 2 |
| kuna(fresh) | 153 | 2 | 2 | 1 | 5 | 10 | 2 |

**ghidra (stored)** — the whole declaration block; three stack locals, each declared once:

```c
void applyLedFixedLayers(void)
{
  ushort uVar1;
  undefined1 uVar2;
  undefined2 uVar3;
  uint uVar4;
  int iVar5;
  uint *puVar6;
  int iVar7;
  undefined4 uVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  undefined4 local_24;      /* <- the hsvColor_t `color`, ONE declaration */
  uint local_20;
  undefined4 local_1c;
```

**ida (stored)** — same shape, three stack locals, each declared once:

```c
  int v23; // [sp+Ch] [bp-Ch] BYREF
  int v24; // [sp+10h] [bp-8h]
  int v25; // [sp+14h] [bp-4h]
```

**kuna (fresh, today's build, no flags == `--mode reliable` here)** — eight stack
declaration lines for the same three slots, two of them with the *same name*:

```c
void sub_8052b04(void)
{
  uint2 v1;
  int4 v10;
  int4 v11; // r8
  unsigned int v12; // stack - 0x24     <-- SYMPTOM: duplicate declaration
  uint4 v12; // stack - 0x24            <-- SYMPTOM: same name, same slot
  float4 v13; // s15
  unsigned short v14; // stack - 0x24
  undefined3 v15; // stack - 0x24
  char v16; // stack - 0x22
  unsigned short v17; // stack - 0x22
  char v18; // stack - 0x21
  unsigned int v19; // stack - 0x20
  char v2;
  uint4 v20; // stack - 0x1c
  ...
```

The stored (2026-07-27) kuna block has the identical two `v12` lines, so nothing
about this changed under the DIV-34..39 print-normalization wave.

Flipping `--option dedupvardecls off` exposes how many HighVariables actually
carry the name `v12` at that one slot:

```c
  unsigned int v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
```

Six highs, one Symbol. `dedupvardecls` (default **on**) collapses them to the two
distinct *rendered lines*, and stops there by design.

## Source

`~/github/decbench/results/full_run/O2-noinline/betaflight/compiled/ledstrip.i`

```c
typedef struct hsvColor_s {
    uint16_t h;
    uint8_t s;
    uint8_t v;
} hsvColor_t;                                    /* 4 bytes, packed */

static void applyLedFixedLayers(void)
{
    for (int ledIndex = 0; ledIndex < ledCounts.count; ledIndex++) {
        const ledConfig_t *ledConfig = &ledStripStatusModeConfig()->ledConfigs[ledIndex];
        hsvColor_t color = *getSC(LED_SCOLOR_BACKGROUND);   /* the stack - 0x24 slot */
        ...
                if (auxInput < centerPWM) {
                    color.h = scaleRange(auxInput, 1000, centerPWM, previousColor.h, color.h);
                    color.s = scaleRange(auxInput, 1000, centerPWM, previousColor.s, color.s);
                    color.v = scaleRange(auxInput, 1000, centerPWM, previousColor.v, color.v);
                } else {
                    color.h = scaleRange(auxInput, centerPWM, 2000, color.h, nextColor.h);
                    ...
        color.h = (color.h + hOffset) % (359 + 1);
        setLedHsv(ledIndex, &color);
    }
}
```

`color` is one 4-byte local written at 2-byte and 1-byte granularity
(`color.h` / `color.s` / `color.v`). Because the binary is stripped there is no
struct type, so every decompiler reconstructs the field writes as
CONCAT/SUBPIECE over one 4-byte slot. Ghidra: `local_24._0_3_ = CONCAT12(...)`.
kuna: `v12 = CONCAT13(v18,v15)`. **This is the same rendering, not a kuna gap.**

## Reported symptom — FIXED (DIV-52)

The duplicated declaration is gone: `applyLedFixedLayers` now opens with one
`unsigned int v12; // stack - 0x24`, the same choice Ghidra makes with its single
`undefined4 local_24`. The fix is the Symbol-keyed scalar collapse proposed below,
implemented inside `option dedupvardecls` with one change of arbitration: the
survivor takes the Symbol's own type only when that type is at least as wide as
every access the group covers, otherwise the widest member wins — kuna's ScopeLocal
ranges are sometimes narrower than the accesses that reach them, and a 2-byte
declaration for a slot the body writes 4 bytes into would be a new defect.

Prevalence, measured before and after over 14 decbench binaries / 11,874 functions
(the estimate in the analysis below was made on one binary): 18 functions emitted a
duplicate local declaration, now 0. The blast radius is exactly those 18 functions —
nothing else changes a byte, no function loses a declared name, none gains one.

Every secondary observation in this record is untouched and still open.

## Analysis

**Structural symptom (ONE).** Today's default build declares two different
locals with the same identifier in the same scope:

```c
  unsigned int v12; // stack - 0x24
  uint4 v12; // stack - 0x24
```

The emitted C is not compilable, and a reader cannot tell which `v12` any body
occurrence (`v12 = *(uint4 *)sub_80520ac(3);`, `v3 = v12 & 0xffff;`, `v12 =
CONCAT22(v17,...)`) refers to. Ghidra emits one `undefined4 local_24;`; IDA emits
one `int v23;`.

**Root cause.** kuna's declaration emitter walks **HighVariables**, not the
`ScopeLocal` **Symbol** table, and takes each declaration's type from that high's
own storage-representative Varnode:

* `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs` — `emit_local_var_decls`
  builds `decls` by iterating `fd.vbank().iter_loc()` and de-duplicating **by
  HighVariable id**, then `local_decl_type_and_comment` renders the type from
  `v.get_type()` of the high's representative varnode (printc.rs:2754).
* The existing "one declaration per Symbol" retain in the same function is
  deliberately restricted to composites — the key is
  `(name, Rc::as_ptr(kuna_symbol_type))` and the comment states *"Restricting to
  composites is load-bearing: primitive types are shared by every scalar local of
  that type."* A **scalar** mapped Symbol therefore keeps the per-high behavior.
* The scalar guard that does exist, `is_scalar_partial_piece` /
  `high_name_has_scalar_whole_sibling` (printc.rs:8074), only suppresses **strict
  partials** (`kuna_symbol_offset() > 0 || rep_size < sym_size`). All six `v12`
  highs are whole 4-byte covers at offset 0, so none is suppressed.
* `kuna_dedupvardecls` (default **on**) is the last line of defence, but its
  `DeclSignature` is `(decl_type, name, array, comment)` — the *fully rendered
  line*. Its own module doc says two highs "that render the same name but a
  *different* type ... both survive (a genuine collision the reader must see)".
  Here the six highs split into two type spellings (`undefined4` → `unsigned int`
  under `realtypes`, and `uint4`), so two lines survive.

Upstream Ghidra cannot hit this: `emitScopeVarDecls` walks the ScopeLocal Symbol
table once per Symbol (printc.cc:2667/2696) and declares `sym->getType()`
(printc.cc:1719), so one Symbol ⇒ exactly one declaration with one type.

**Owning phase: P9 emit** (declaration emission / naming policy). P6 variables is
the upstream contributor — six highs end up bound to one 4-byte stack Symbol with
disagreeing recovered types — but a Symbol-keyed declaration walk fixes the
invalid C regardless of how many highs share the Symbol, which is exactly the
invariant upstream relies on.

**Why the references win.** They declare per Symbol, not per SSA-level variable.

**Measured prevalence (today's build).** 5 / 6384 functions in
`betaflight_STM32F405` (0.1%) and 1 / 154 in `O2-noinline/bzip2` emit a duplicated
local name — `sub_802baf4`, `sub_804f5b4`, `sub_8052676`, `sub_8052684`,
`sub_8052b04`. Rare, but every occurrence is invalid C.

**The miner hint is disproven.** "8 CONCATs + 7 SUBPIECEs: sub-register
reconstruction soup" is not a kuna-vs-reference gap: Ghidra's pane has the same
count of the same forms (`CONCAT22`, `CONCAT12`, `CONCAT13`, `local_24._2_1_`,
`local_20 >> 0x18`) because the packed `hsvColor_t` has no type in the stripped
binary. kuna is *tied for best* on this function and its CFG shape (2 gotos,
2 labels, 1 switch/5 cases, 10 ifs, 2 loops) matches Ghidra's exactly.

**Verification performed.**
* Fresh no-flag run == stored run structurally; the binary is 537,492 B ≥ 500 KiB
  so `--mode auto` selects `reliable` — the benchmark's mode. `--mode reliable`
  and `--mode aggressive` produce byte-identical output here (4356 B both), so no
  mode default is hiding or causing this.
* Option sweep over the symptom-matching catalog entries: `dedupvardecls off`
  (worse: 6 × `v12`), `dedupvardecls on` (default, still 2), `realtypes off`
  (still 2: `xunknown4 v12` + `uint4 v12`), `namestyle ghidra` (still 2).
  `condfold on|wide` was **not** swept: it is a P8 goto/short-circuit lever and
  kuna already matches Ghidra's goto/label counts on this function.
* Source CFG 26 nodes / 40 edges — exact GED, not approximated, not degenerate.
  Not a metric artifact.

### Secondary observations (NOT the reported symptom; each needs its own case)

1. **P4 — a stack-passed argument is dropped at this call site.** Ghidra:
   `FUN_080170e0(iVar11,0x5dc,2000,local_24 & 0xffff,local_20 & 0xffff)` (5 args,
   matching `int scaleRange(int x,int srcFrom,int srcTo,int destFrom,int destTo)`).
   kuna: `sub_80170e0(v10,0x5dc,2000,v3)` (4 args) at **all eight** call sites in
   this function. kuna's own recovered prototype for the callee is correct —
   `int4 sub_80170e0(int4 a0,int4 a1,int4 a2,int4 a3,int4 a4)` — and kuna emits
   5-argument call sites elsewhere in the same firmware (88 of them), so this is
   NOT a blanket stack-trial failure; it is a per-call-site recovery miss whose
   visible fallout is that `v20 // stack - 0x1c` is **read but never assigned**
   and `v19 // stack - 0x20` is declared but never used.
2. **P2 — 14 spurious `setISAMode(1);` statements** that neither Ghidra nor IDA
   emits, one before most calls (ARM/Thumb ISA-mode CALLOTHER surfacing as a
   statement).
3. **P9/P6 — `option namestyle ghidra` is not honoured for stack-mapped locals.**
   Under `namestyle ghidra` most locals correctly become `iVar10` / `uVar3` /
   `xVar14`, but the stack-mapped ones keep angr names (`v12`, `v19`, `v20`) and
   lose their storage comments. `varmap.rs:1548`, `:1598`, `:1718` hard-code
   `format!("v{}", *base)` with no `name_style_angr` gate, while the unmapped-local
   path (`coreaction_cleanup.rs::kuna_default_local_name`) does gate correctly.

## Proposed fix

**Mechanism.** Make the scalar declaration walk Symbol-keyed, the way the
composite one already is.

In `emit_local_var_decls` (`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`),
the retain that currently collapses on `(name, composite-type-Rc)` should also
collapse **scalars** keyed on `(name, storage location)` — the `(text, offset)`
storage comment `local_decl_type_and_comment` already computes. A name plus a
mapped stack offset identifies exactly one `ScopeLocal` Symbol, which is the
identity the composite path was reaching for via the interned type Rc; the module
comment's objection ("primitive types are shared by every scalar local of that
type") applies to the *type* key, not to the storage key. Keep the surviving
declaration's type as the mapped Symbol's type (`h.kuna_symbol_type()`) when there
is one — the direct analogue of C++ `emitVarDecl`'s `sym->getType()`
(printc.cc:1719) — falling back to the widest whole-cover high's representative
type otherwise.

Equivalent narrower form, if the change is preferred inside the option that
already owns this surface: drop `decl_type` from
`kuna_dedupvardecls::DeclSignature` **when `comment` is `Some`** (i.e. the local
has a resolved storage location, so name+storage is a Symbol identity), and keep
the full four-field key for un-mapped locals where storage cannot disambiguate.

**Owning files.**
* `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs` — `emit_local_var_decls`
  (the `seen_sym` retain, ~printc.rs:2585) and `local_decl_type_and_comment`.
* `decompiler/crates/kuna-decomp/src/p9_emit/kuna_dedupvardecls.rs` — the
  `DeclSignature` key and its module doc, which currently *documents* the residual
  collision as intentional and would need to be updated.
* Spec chapter owning `p9_emit/` (find via its `Anchors:` header) — the collapse
  rule is prose-described there.

**Flagging.** This is an extension of the existing `dedupvardecls` option (P9,
default on), not a new option: no new `settableTable` row, no catalog-count bump.
The behaviour change is strictly "invalid C ⇒ valid C", which AGENTS.md classes as
a bug fix; keeping it inside `dedupvardecls` preserves the existing
`dedupvardecls off` escape hatch for byte-comparison work.

**Risks.**
* Choosing which type survives is a judgement call. Declaring the Symbol's type
  can make a body occurrence that was typed `uint4` render against an
  `unsigned int` declaration; use sites already carry explicit casts
  (`*(uint4 *)...`), so this is cosmetic, but it will move bytes on any datatest
  whose function has two same-named highs — run the `make test` /
  `make test-stages` gates and diff the decompiled C across fixtures before and
  after (the `[[kuna-verify-by-code-diff]]` practice from #197: the suites did not
  catch a naming regression on their own).
* The collapse must not fire for two *genuinely distinct* locals that happen to
  share a name — impossible for mapped locals (the storage offset differs) but the
  guard should be explicit: only collapse when both the name **and** the rendered
  storage comment match, never on name alone.
* Prevalence is low (0.1% of functions), so expect a near-zero datatest delta;
  if any assertion moves, that is a signal to re-examine, not to re-pin.

**End-to-end testcase.** `tests/stages/` two-pass on a small fixture that spills a
packed struct field-by-field to one stack slot (the `hsvColor_t` shape:
`uint16;uint8;uint8` written individually): `dedupvardecls off` reproduces the
repeated declarations, default shows exactly one declaration per slot.
