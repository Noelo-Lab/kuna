# Lane `engine` — where kuna's casts come from, rule by rule

Read-only census. Worktree `/home/mahaloz/kwt/castcensus`, branch `campaign/castcensus`,
base main `796939d54`. **No engine change ships from this lane**; the only code in the branch
is env-gated scratch instrumentation (`KUNA_CASTCENSUS=1`), marked *NOT for merge*.

**Headline.** Every printed cast in kuna's C comes from one of eight sites, and 4,586 of the
5,858 casts in `fmt/ls/sort/du` O0 (78%) are `CPUI_CAST` ops created by **one function**,
`ActionSetCasts`, on one decision, `CastStrategyC::cast_standard`. Attributing each of those
ops to the rule that made it gives a distribution that is *not* what a reading of the C
suggests: **48.6% of all casts are the pointer↔integer round trip around a single
`CPUI_INT_ADD`**, and **74% of those have a `void *` base with a constant offset**. That base
type is kuna's own default, set by `ptrfromuse void` (#690). Flipping it to `byte` removes
**14.5% of all casts** (5,622 → 4,808 on those four binaries) at **zero cost on type_match**
(typesweep: 241 → 241 perfect, aggregate +1.10, 0 functions worsened). That is the single
biggest measured, free, *type-side* cast removal available today.

---

## 1. Every place a cast can be printed

Eight sites. Six of them route through the same emitter, `PrintC::op_type_cast_ir`.

| # | site | file:line | what it prints |
|---|---|---|---|
| 1 | `op_type_cast_ir` for `CPUI_CAST` | `p9_emit/printc.rs:5907` | `(T)x` for every CAST op ActionSetCasts inserted |
| 2 | `op_int_sext_ir` / `op_int_zext_ir` → (1) | `printc.rs:6144` / `:6113` | `(long)x` when `is_sext_cast`/`is_zext_cast` says the extension *is* a cast and `is_extension_cast_implied` says it is not implied |
| 3 | `op_subpiece_ir` → (1) | `printc.rs:6177`, dispatch `:6271` | `(int)x` / `(char)x` for a truncation `is_subpiece_cast` calls a cast |
| 4 | `op_float_float2float_ir` | `printc.rs:5871` | `(float8)x` for INT2FLOAT / FLOAT2FLOAT |
| 5 | `op_bool_truncation_ir` (kuna `boolbyte`) | `printc.rs:6289-6290` | the **double** cast `(bool)(unsigned char)x` |
| 6 | `push_partial_symbol_ir` final cast | `printc.rs:7279` | a cast wrapping a `sym.field` access |
| 7 | `push_constant_ir`, TYPE_PTR arm | `printc.rs:7628` | `(struct_2 *)0x2b080` — a pointer-typed constant |
| 8 | `lowered_switch_label_form` (kuna `loweredswitch`) | `printc.rs:5367` | a cast on a synthesized switch label |

`option_nocasts` (`printc.rs:433`, `set_no_cast_printing`) suppresses 1/4/5/7/8 wholesale.
**That is the "just hide them" lever and it is not the answer** — it is listed only so nobody
proposes it.

The only *producer* of site 1 is `Funcdata::action_set_casts` (`p9_emit/coreaction_casts.rs:1113`),
which inserts a `CPUI_CAST` in exactly two places:

- `cast_input` (`coreaction_casts.rs:1312`), tail at **`:1404-1416`**, when
  `get_input_cast(op, slot)` returns `Some(T)`.
- `cast_output` (`coreaction_casts.rs:1448`), tail at **`:1519-1528`**, when the op's
  `getOutputToken` type differs from the output HighVariable's def-facing type and
  `cast_standard` still demands a cast (`:1500`).

`get_input_cast` (`:305`) dispatches to 12 per-op-code bodies ported from `typeop.cc`; nine of
those, and the default (`:750`), end in **`CastStrategyC::cast_standard`** (`p9_emit/cast.rs:667`).
So one function decides ~78% of kuna's casts.

## 2. Method

`KUNA_CASTCENSUS=1` bumps a counter at each emission site keyed by
`(op-code, get_input_cast arm, which return in cast_standard fired, the (reqbase,curbase)
metatype pair)`; the extension site additionally records the reader op and whether
`is_extension_cast_implied` fired; `cast_fixup_ptrsub`/`cast_fixup_ptradd` record demotions.
Instrumentation is in the branch: `cast.rs` `pub mod census`, bumps at `coreaction_casts.rs`
:1432/:1542 and `printc.rs` (6 sites). Aggregation: `.scratch/buckets.py`. Text-side counter
for cross-checking: `.scratch/castshapes.py`.

Corpus: `fmt ls sort du` at O0 and at O2, whole-binary `kuna decompile-all` on the decbench
stripped inputs (`/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/`).
The instrumented total (5,858 print events, O0) agrees with the text counter (5,622) to 4%,
the difference being `(bool)` and macro-shaped casts the regex declines.

## 3. The distribution

O0, `fmt ls sort du`, 5,858 printed casts; O2, same four, 6,801.

| family | O0 | % | O2 | % | bucket |
|---|---|---|---|---|---|
| **PTRARITH** — CAST on an `INT_ADD`/`INT_SUB` input or output | **2,893** | **48.6** | **2,349** | **35.8** | **(b)** |
| **EXT** — SEXT/ZEXT printed as a cast | 808 | 13.6 | 1,041 | 15.8 | (c) 32% / (a) 19% / mixed |
| **MEMACCESS** — LOAD/STORE pointer or value cast | 800 | 13.4 | 964 | 14.7 | (b) |
| **CALL** — cast on an argument, or on a call's returned value | 514 | 8.6 | 550 | 8.4 | (b) return / (c) args |
| COMPARE — signed/unsigned compare operands | 238 | 4.0 | 369 | 5.6 | (b) signedness |
| TRUNC — SUBPIECE as a cast | 185 | 3.1 | 396 | 6.0 | (a)/(c) |
| **PTRCONST** — `(T *)0x1234` | 155 | 2.6 | 256 | 3.9 | **(b)** no symbol |
| EXT-in — a cast *before* a ZEXT/SEXT | 122 | 2.1 | 120 | 1.8 | (c) |
| FLOAT — INT2FLOAT/FLOAT2FLOAT/FLOAT_TRUNC | 116 | 1.9 | 105 | 1.6 | **(a)** |
| COPY / OTHER / PARTIAL | 118 | 2.0 | 418 | 6.4 | mixed |

---

## 4. Shape by shape: the rule, the bucket, the fix

### 4.1 `*(T *)((long)p + K)` — the pointer↔integer round trip. **2,893 O0 (48.6%), bucket (b)**

**The rule.** Two casts per access, from two different sites:

- the inward one, `(long)p`: `get_input_cast` default arm (`coreaction_casts.rs:750`) for
  `CPUI_INT_ADD`. `inputTypeLocal(slot)` is an *integer* of the op's size; the varnode's
  read-facing type is `void *`. `cast_standard(int8, void*, care_uint_int=false, …)`
  reaches the `TYPE_INT` arm at **`cast.rs:774`**, where `curbase.get_metatype()` is
  `TYPE_PTR` — not in the accepted set — and falls out to `Some(reqtype)` at **`cast.rs:815`**.
  Census key: `IN CPUI_INT_ADD intadd cs:meta ("int","ptr")`.
- the outward one, `(T *)`: `cast_output` (`coreaction_casts.rs:1448`). The INT_ADD's
  `getOutputToken` is an integer; the result's HighVariable is a pointer; `cast_standard`
  at `:1500` takes the same `cast.rs:815` exit. Census key:
  `OUT CPUI_INT_ADD cs cs:meta ("ptr","int") cast`.

O0 split of the 1,066 inward casts, by what the pointer points to:

| pointee | const offset? | n | |
|---|---|---|---|
| **`void` (no pointee)** | **yes** | **787** | 74% of the family |
| `void` | no | 100 | |
| `int8` / `ptr` / `unknown` / `uint8` | yes | 137 | |
| everything else | | 42 | |

The matching outward casts are 1,294 (`ptr<-int`) + 383 (`ptr<-uint`).

**It is not a demotion.** I instrumented `cast_fixup_ptrsub` (`coreaction_casts.rs:1219`) and
`cast_fixup_ptradd` (`:1187`): PTRSUB→INT_ADD demotion fires **0 times** on this corpus, and
PTRADD→INT_ADD 54 times. These ops were already `INT_ADD` when P9 got them — P5 never built a
PTRSUB, because a `void *` has no field at offset K to build one against.

**Bucket (b): the fix is the type.** Two witnesses.

*Witness 1 — the same function types one base and not the other.* `ls` O0 `sub_5297 @0x5297`
(`.scratch/fresh/kuna_ls.c:1133`):

```c
void sub_5297(void *a0, struct_4 *a1)
{
  a1->field_0x8  = *(unsigned long *)((long)a0 + 0x20);   // 2 casts
  a1->field_0x18 = (unsigned int)*(unsigned short *)((long)a0 + 0x1c);
  a1->field_0x1c = *(unsigned int *)((long)a0 + 0x14);
  …                                                        // ~20 more, all 2 casts
}
```
`a1` got a synthesized record (`structsynth`) and every write through it is **cast-free**.
`a0` did not and every read through it costs **two**. Same access shape, same function,
0 vs 2 — decided entirely by whether a record exists.

*Witness 2 — kuna pays a cast for being right.* `ls` O0 `@0x53e7`:

```c
// IDA (ida_ls.c:1)                              // kuna (main 796939d54)
long long file_or_link_mode(long long a1)        unsigned int sub_53e7(void *a0)
{                                                {
  if (byte_2B358 && *(char *)(a1 + 185))           if ((dat_2b358) && (*(char *)((long)a0 + 0xb9)))
    return *(unsigned int *)(a1 + 172);              v1 = *(unsigned int *)((long)a0 + 0xac);
  else                                             else
    return *(unsigned int *)(a1 + 48);               v1 = *(unsigned int *)((long)a0 + 0x30);
}                                                }
// 3 casts                                       // 6 casts
```
IDA declares the base `long long` — an integer — so `a1 + 185` needs no cast. kuna declares
it `void *`, which is *more* information, and pays `(long)a0` three times for it.

**The measured fix, and it is free.** `ptrfromuse` already has the two better answers:

| `--option ptrfromuse` | `sub_53e7` | casts, 4 bins O0 | per kloc |
|---|---|---|---|
| `off` | `long a0` → `*(unsigned int *)(a0 + 0xac)` | 5,031 | 145.2 |
| **`byte`** | **`char *a0` → `*(unsigned int *)&a0[0xac]`, and `a0[0xb9]` outright** | **4,808** | **138.8** |
| `void` (today's default, #690) | `void *a0` → `*(unsigned int *)((long)a0 + 0xac)` | 5,622 | 162.3 |

At O2 the same flip gives **6,522 → 5,879, −9.9%** (163.0 → 146.9 per kloc).

`byte` is **−14.5%** casts against today's default at O0, **−9.9%** at O2, and line count is
unchanged (34,638 vs 34,639 at O0) so nothing is being hidden — `a0[0xb9]` is a *better* rendering than
`*(char *)((long)a0 + 0xb9)`, not a shorter one. `char *` is also the truer claim: the base
*is* byte-addressed, which is exactly what the code does with it.

**And it costs nothing on the campaign metric.** typesweep `ptrfromuse=byte`, 18 slices /
1,312 functions (grep, gzip, diffutils, bzip2, findutils × O0,O2),
`.scratch/typesweep-ptrfromuse-byte/report.md`:

```
type_match PERFECT: base 241 -> test 241 (+0)
aggregate type_match: base 575.62 -> test 576.71 (+1.10)
moved ONTO perfect: 0      moved OFF perfect: 0
improved (not perfect): 3  worsened (not perfect): 0
control: byte-identical variables in both arms: 1097 functions, 0 scored differently
```

The remaining 26% of the family (a *typed* pointer whose access width ≠ pointee width) is the
`structsynth` / `layoutrecall` lever the campaign already runs: a record at the base turns two
casts into zero, as witness 1 shows.

**Parity risk.** `ptrfromuse` is pinned by `tests/stages/kuna-ptrfromuse.xml`,
`tests/stages/kuna-charptr.xml`, `tests/cli/ptrfromuse-default-declares-a-dereferenced-parameter-void.json`
(that filename *is* the assertion) and `tests/cli/protoorder-types-keeps-a-wide-store-whole.json`.
A default flip re-records those plus `docs/baseline-stages.json`; `docs/baseline.json` is
untouched (the option is kuna-only and the datatest corpus does not set it). It is a default
change, not a divergence — the option already exists, so no new `[[settable]]` row and no
catalog-count movement.

### 4.2 SEXT/ZEXT printed as a cast. **808 O0 (13.6%), 1,041 O2 (15.8%)**

**The rule.** `op_int_sext_ir` / `op_int_zext_ir` (`printc.rs:6144` / `:6113`). The extension
prints as `(long)x` unless `options.hide_exts` **and** `is_extension_cast_implied`
(`cast.rs:610`, C++ `CastStrategyC::isExtensionCastImplied`). `hide_exts` is already `true`
by default (`printc.rs:497`), so the whole question is that predicate — and it is narrow:

- `read_op == None` → **`return false` immediately** (`cast.rs:620-623`). An extension at the
  top of a statement is never implied.
- the reader must be `PTRADD` or one of 13 `INT_*` ops; **every other reader returns false**
  (`cast.rs:659`). There is no `CPUI_CALL` arm, no `CPUI_RETURN` arm, no `CPUI_COPY` arm.
- even for an accepted reader, the other operand must be explicit (or a small constant) **and
  have the same metatype** as the extension's output (`cast.rs:655-657`). `int` vs `uint`
  fails that test, though C's usual arithmetic conversions do not care about signedness when
  deciding the *width*.

Reader breakdown of the 954 extension print events at O0 (808 printed a cast):

| reader | n | implied fires? | bucket |
|---|---|---|---|
| `CPUI_INT_MULT` | 180 | no | **(a)** — `(long)a * (long)b` on two `int`s must stay 64-bit (one of the two is redundant; IDA keeps one) |
| `CPUI_CALL` | **154** | **no — no arm exists** | **(c)** — C converts an argument to the declared parameter type |
| `None` (statement top) | **134** | **no — early return** | **(c)** — assignment conversion already widens |
| `CPUI_INT_SUB` | 129 | no | (c) mostly — fails the metatype-equality clause |
| `CPUI_INT_ADD` | 98 / 27 | 27 yes | (c) mostly, same clause |
| `CPUI_CAST` | **47** | no | **(c)** — `(T)(long)x`, the outer cast subsumes the inner |
| `CPUI_COPY` | 17 | no | (c) — assignment |
| `CPUI_PTRADD` | 38 | **yes** | already suppressed |
| rest (OR/LEFT/2COMP/compares/STORE) | 130 | mostly no | mixed |

**Bucket (c): the cast is redundant under C's own conversion rules and the strategy inserts it
anyway.** The strictly-safe subset — call arguments with a known prototype, statement-top
assignments to a declared-wider lvalue, `return`, and stacked `(T)(long)` — is **≈350 of 954
at O0** and rises at O2. The `ida` lane's independent syntactic count of the same idea
(assignment RHS 238, `return` 57, identity 42, stacked inner 410 = 747) agrees on the order of
magnitude.

*Witness 1 (call argument, prototype known).* `sort` O0, same call site as IDA's:

```c
ida : v25 = (char *)memchr(ptr, v12, v19 - (char *)ptr);
kuna: v7  = memchr(v10,(int)v4,(long)v14 - (long)v10);
```
`memchr`'s second parameter **is** `int` — `libcwiden` (#714) put that signature in kuna's own
table — so `(int)v4` cannot change the value. IDA prints it as `v12`.

*Witness 2 (stacked).* `sort` O0 `.scratch/fresh/kuna_sort.c:1777`:

```c
*(char *)((long)(int)(unsigned int)(unsigned char)sub_4042((int)v1) + 0x23560)
```
Four stacked conversions of one byte-wide call result to 64 bits. Each inner one is subsumed
by its outer. IDA writes the load and lets promotion do the rest.

**Parity risk — this is the dangerous one.** `is_extension_cast_implied` is a verbatim port of
`CastStrategyC::isExtensionCastImplied` (cast.cc:249-298) and is pinned by
`decompiler/crates/kuna-decomp/tests/verify_w10_cast_strategy_adversarial.rs` and the
`cast_*` decision-matrix tests in `p9_emit/cast/tests.rs` (ported from upstream
`testtypes.cc`). Widening it is a **divergence from Ghidra** and must ship behind a named
option per AGENTS.md. Text pins to check before touching it: `tests/stages/*.xml` hold 32
`(char *)`, 27 `(void)`, 9 `(int *)`, 8 `(long *)`, 8 `(int)`, 5 `(int4)` literal spellings —
`kuna-signedness.xml`, `kuna-ctypes.xml`, `ghdec-subright.xml`, `kuna-msvcftol.xml`,
`kuna-charptr.xml` and `funcboundflow.xml` are the ones that pin a scalar cast. The datatest
corpus is almost clean here (4 `(void)`, 2 `(char *)`, 1 `(int4)` across all 83 files), so
`docs/baseline.json` is at low risk — but the option must default **off** until a 0/675 run
proves otherwise.

**Precedent.** kuna already has exactly this shape: `kuna_typeround`'s `SignPlan::drop_cast`
(`p9_emit/kuna_typeround.rs:369`, option `signedness`) drops a printed CAST because the
declaration it is casting to was itself changed to match. `printc.rs:5933` consults it. A
"redundant under C conversion" predicate belongs next to it, not inside the ported
`CastStrategyC`.

### 4.3 LOAD/STORE pointer and value casts. **800 O0 (13.4%), bucket (b)**

**The rule.** `get_input_cast_load` (`coreaction_casts.rs:385`, `TypeOpLoad::getInputCast`)
and `get_input_cast_store` (`:436`). The load's *required* pointer type is
`pointer-to(output's def-facing type)`; if the actual pointer's pointee differs in **size** it
takes the `cs:size-change` exit at `cast.rs:727` and a `(T *)` cast is built. 240 of the 800
are exactly that. The remainder are `cast_output` on the LOAD (`ptr<-int` 237, `ptr<-unknown`
111, `ptr<-uint` 14) — the loaded value's token type disagreeing with the variable's.

Bucket (b): a wrong-width or `void` pointee. Same lever as §4.1 — a record base makes the
access a field and the question disappears. 18 more take the `cs:from-void` exit
(`cast.rs:679`), where the pointee is literally `void`.

### 4.4 `(T *)0x2b080` — a global address with no symbol. **155 O0, 256 O2, bucket (b)**

**The rule.** `push_constant_ir`, TYPE_PTR arm, `printc.rs:7622-7624`: a pointer-typed
constant prints as a forced-hex integer behind a typecast. Nothing decided to cast here — the
constant simply never became a symbol reference.

*Witness.* `ls` O0, adjacent statements in one function:

```c
sub_e4ca((struct_2 *)0x2b080);        // no symbol -> cast + raw address
… dat_2b358 …                         // same binary, same region, symbol exists
```
IDA prints `&stru_2B080`. kuna *has* the `dat_<addr>` scheme and uses it two lines away; the
address here just never got a symbol minted. Bucket (b), and the owning tier is
`kuna-analysis` global-symbol creation / `linkConstant`, not P9. Note the memory item
*"global scope is all ram"*: every stock cspec claims the whole ram space, so `global add` is a
measured no-op — the symbol has to come from the analyzer tier.

### 4.5 Signed/unsigned compare operands. **238 O0, 369 O2, bucket (b)**

`get_input_cast_less` (`coreaction_casts.rs:538`) takes `inputTypeLocal` — a *signed* `int` for
SLESS, an *unsigned* `uint` for LESS — and coerces with `care_uint_int = true`, so any
operand whose declared signedness disagrees with the opcode's gets a cast
(`cast.rs:743`/`:774` "care" arms). Census: `("uint","int")` 135, `("int","uint")` 74. The fix
is the declared signedness of the variable, which `signedness`/`kuna_typeround` already
addresses and which is **worth zero on type_match** — a cast-quality-only lever.

### 4.6 What must stay — bucket (a)

- **FLOAT** 116 O0 / 105 O2: `(float8)i`, `(int)f`, `(float4)d` are real conversions.
- **`(long)a * (long)b` on two `int`s**: 180 O0 at `reader=CPUI_INT_MULT`. Dropping both
  changes the value. Dropping *one* does not (the other promotes it), which is what IDA does —
  so ~90 of the 180 are recoverable, but the predicate has to prove the surviving operand is
  the wide one.
- **TRUNC** 185 O0 / 396 O2: `is_subpiece_cast` (`cast.rs:847`). A narrowing cast is
  load-bearing wherever the destination is not already that width. kuna already emits **fewer**
  of these than IDA (955 vs 1,194 on the `ida` lane's paired corpus), so this is not a lever.
- **`kuna_truncarg`'s argument casts** (47 O0, census `IN CPUI_CALL call ?`): `narrowed_arg_cast`
  (`coreaction_casts.rs:379` → `kuna_truncarg.rs`) deliberately *adds* a cast so C's integer
  promotion cannot widen a trimmed argument. #678. Required.
- **`boolbyte`'s `(bool)(unsigned char)x`** (`printc.rs:6289`): a deliberate double cast so a
  `bool` test does not see high bits. #680. Required.

### 4.7 Bucket (d) — the printing form itself

Small but real: `push_partial_symbol_ir`'s final cast (`printc.rs:7279`, 14 O0 / 35 O2) wraps
a `sym.field` access in a cast when the member's type does not cover the access. And the
`*(T *)&x` shape the `ida` lane counts at 1,158 vs IDA's 123 is a *form* choice — a union
member or a re-declared local would read straight. Both are P9-local and neither needs a type
change; both are small.

---

## 5. Ranked, with the risk

| # | change | casts removed (O0, 4 bins) | bucket | risk |
|---|---|---|---|---|
| 1 | **`ptrfromuse` default `void` → `byte`** | **814 O0 (−14.5%), 643 O2 (−9.9%)** | (b) | **lowest**: option exists, type_match-neutral (measured), re-records 2 stage + 2 cli pins |
| 2 | more synthesized record bases for `void *`/weak pointees (`structsynth`, `layoutrecall`) | up to ~1,800 more of the PTRARITH family, 0 → 2 per access | (b) | medium; already the campaign's ranked lever, pays in quality not in today's metric |
| 3 | a "redundant under C conversion" predicate for extensions at calls / assignments / `return` / stacked casts | ≈350 | (c) | **highest**: Ghidra divergence, needs a named option, default off, pinned by `verify_w10_cast_strategy_adversarial.rs` + `cast/tests.rs` |
| 4 | mint a symbol for every referenced static address | 155 O0 / 256 O2 | (b) | low, but it is a `kuna-analysis` change, not P9 |
| 5 | declared signedness matching the compare | up to 238 | (b) | low; `signedness` exists, metric-neutral |

Items 1, 2 and 4 all satisfy the user's constraint literally: the cast disappears **because the
type became right**. Item 3 is the only one where the cast disappears because it was provably
redundant, and it is the only one that needs an option.

## 6. Reproduce

```bash
cd /home/mahaloz/kwt/castcensus
export SLEIGHHOME=$PWD/specs
export KUNA_SPECS=$PWD/specs
R=/home/mahaloz/github/decbench/results/full_run_address_2026-09-11
KUNA_CASTCENSUS=1 ./decompiler/target/release/kuna decompile-all $R/O0/coreutils/stripped/ls \
  > /dev/null 2> .scratch/census_ls.txt
python3 .scratch/buckets.py .scratch/census_{fmt,ls,sort,du}.txt
python3 .scratch/castshapes.py .scratch/abl/{void,byte,off}_ls.c      # the ablation
```

Files: `.scratch/castshapes.py` (text counter), `.scratch/buckets.py` (rule → bucket),
`.scratch/census_*.txt` (raw rule counts), `.scratch/abl/` (ptrfromuse arms),
`.scratch/typesweep-ptrfromuse-byte/` (metric control).
