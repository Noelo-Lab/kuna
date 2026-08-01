---
case_id: O2-noinline-libacl-libacl.so.1.1-set_acl_fd
pool: novel
status: feature-candidate
tier: N
margin: 0
fresh_verdict: today's default (auto=aggressive) reproduces the *structure* ghidra gets (0 gotos / 0 labels, all 14 branches, correct function boundary) and beats IDA; the miner's (code *) hint is disproven, but every access through the callback-table parameter is emitted against a `void *`-declared pointer, so the C does not compile and the access width is lost
option_closing: realtypes off (workaround only)
feature_slug: realtypes-pointee-size
scope: small
confidence: high
---

## Side-by-side

Recorded scores: kuna GED **62 (best of all)**, ghidra 62, angr 68, ida 70, phoenix 70,
binja 89. Source CFG 32 nodes / 70 edges (exact GED, not approximated).

| output | loc | gotos | labels | ifs | loops |
|---|---|---|---|---|---|
| ghidra(stored) | 63 | 0 | 0 | 13 | 0 |
| ida(stored) | 75 | **1** | **1** | 12 | 0 |
| kuna(stored) | 57 | 0 | 0 | 13 | 0 |
| kuna(fresh) | 48 | 0 | 0 | 14 | 0 |

### ghidra (stored) — the co-best pane

```c
int set_acl_fd(undefined8 param_1,int param_2,__mode_t param_3,undefined8 *param_4)
{
  code *pcVar1;
  ...
        if (param_4 == (undefined8 *)0x0) {
          return iVar3;
        }
        if ((code *)param_4[1] != (code *)0x0) {
          param_1 = (*(code *)param_4[1])(param_4,param_1);
        }
        pcVar1 = (code *)*param_4;
        if (pcVar1 != (code *)0x0) {
          uVar6 = dcgettext(0,"setting permissions for %s",5);
          (*pcVar1)(param_4,uVar6,param_1);
        }
```

Ghidra uses **exactly the same `(code *)` indirect-call rendering as kuna** — 6+ of them —
and still ties for best. Its parameter is `undefined8 *param_4`, so `param_4[1]` is a
stride-8 index and the expression is well formed.

### ida (stored)

```c
      v11 = *(long long (**)(long long, long long))(a4 + 8);
      if ( v11 )
        v5 = v11(a4, a1);
      v12 = *(void (**)(long long, char *))a4;
```

IDA does *not* recover `struct error_context` either — it casts the raw `a4 + 8` load
exactly like kuna/ghidra do and merely spills the loaded value into a typed local. It also
emits a `goto LABEL_13` that the source does not have, and scores **70 — worse than kuna's
62**. The miner hint ("IDA prints typed function-pointer locals") describes a cosmetic
difference that costs IDA 8 GED points, not a kuna gap.

### kuna (fresh, today's default `--mode auto` → aggressive)

```c
int4 set_acl_fd(unsigned long a0,unsigned int a1,unsigned int a2,void *a3) // return-dupe x3
{
  code *v1;
  ...
      if (!a3)
        return v2;
      if ((code *)a3[1])                                   // <-- a3 is void *
        a0 = (*(code *)a3[1])(a3,a0);
      v1 = (code *)*a3;                                    // <-- deref of void *
      if (v1)
        (*v1)(a3,dcgettext(0,"setting permissions for %s",5),a0);
      if (!(code *)a3[2])
        return v2;
      (*(code *)a3[2])(a3,a0);
```

`--mode reliable` (the mode the benchmark actually ran) is structurally identical; the only
difference is the `""` argument, which reliable still prints as the raw `0x71d3`. On this
one point today's default is the *most* faithful pane of the three: the source passes `""`,
kuna(auto) prints `""`, ghidra prints `&DAT_001071d3`.

Second witness for the same defect, `acl_calc_mask` @ 0x2820 (same binary, no flags):

```c
unsigned long sub_2820(void *a0) // return-dupe
{
  ...
    *(void *)__errno_location() = 0x16;     // 4-byte store rendered with no width
    return 0xffffffff;
  }
  v2 = sub_5690(*a0,0x712c);                // deref of void *
```

## Source

`libacl/perm_copy_fd.c` (preprocessed:
`~/github/decbench/results/full_run/O2-noinline/libacl/compiled/la-perm_copy_fd.i:5363`).
The fourth parameter is a table of three function pointers:

```c
struct error_context {
  void (*error) (struct error_context *, const char *, ...);   /* +0  */
  const char *(*quote) (struct error_context *, const char *); /* +8  */
  void (*quote_free) (struct error_context *, const char *);   /* +16 */
};

static int
set_acl_fd (char const *path, int fd, mode_t mode, struct error_context *ctx)
{
  int ret = 0;
  acl_t acl = acl_from_mode (mode);
  if (!acl) {
    do { if ((ctx) && (ctx)->error) (ctx)->error((ctx), ""); } while(0);
    return -1;
  }
  if (acl_set_fd (fd, acl) != 0) {
    ret = -1;
    if ((*__errno_location ()) == 95 || (*__errno_location ()) == 38) {
      (void) acl_free (acl);
      goto chmod_only;
    } else {
      const char *qpath = (((ctx) && (ctx)->quote) ? (ctx)->quote((ctx), (path)) : (path));
      do { if ((ctx) && (ctx)->error) (ctx)->error((ctx),
             dcgettext (((void *)0), "setting permissions for %s", 5), qpath); } while(0);
      do { if ((ctx) && (ctx)->quote_free) (ctx)->quote_free((ctx), (qpath)); } while(0);
    }
  }
  (void) acl_free (acl);
  return ret;

chmod_only:
  ret = fchmod (fd, mode);
  if (ret != 0) { /* the same quote / error / quote_free triple again */ }
  ...
}
```

Every `(code *)` call site in every pane corresponds to one of `ctx->error`,
`ctx->quote`, `ctx->quote_free`. The `error_context` type is a *header* type that is not in
the binary's debug info, and the source deliberately duplicates the quote/error/quote_free
triple across the `else` arm and the `chmod_only` tail — so the two identical-looking blocks
in kuna's output are real, not a duplication artifact.

## Analysis

**The nominated gap does not exist.** On today's build kuna reproduces the source control
flow as well as any tool measured: 0 gotos, 0 labels, both copies of the quote/error/
quote_free triple present, the `chmod_only` tail correctly attached, the `errno == 95 ||
errno == 38` short-circuit intact, and the whole function boundary recovered. kuna ties
ghidra for the best GED and beats IDA, which invents a `goto`. The `(code *)` cast the miner
flagged is the standard Ghidra-lineage rendering for an indirect call through an untyped
pointer, and the co-best pane (ghidra) emits it identically — so it costs nothing. Neither
IDA nor ghidra recovered a callable prototype or the `error_context` struct either; there is
no P4/P5 gap to close here, and no reference pane to copy.

**The one structural symptom that is real** — found while checking the hint, and reproduced
on today's default build in both `auto` and `reliable`:

> kuna declares the callback-table parameter `void *a3` and then emits `*a3`, `a3[1]` and
> `a3[2]` against it. Dereferencing or indexing a `void *` is a hard C error
> (`error: invalid use of void expression`, verified with `gcc -std=gnu11 -c`), and under
> GCC's void-arithmetic extension the indices would denote byte offsets **1 and 2 instead
> of 8 and 16** — the element stride the expression was built with is silently destroyed by
> the declaration. The same defect turns a 4-byte errno store into
> `*(void *)__errno_location() = 0x16;`, which carries no access width at all.

**Root cause — one function, P9 emit.** `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`:

```rust
fn realtype_unknown_base(size: int4, under_pointer: bool, long_is_8: bool) -> Option<&'static str> {
    if under_pointer {
        // pointer-to-unknown → `void *` (the modifier walk adds the `*` chain).
        return Some("void");
    }
    Some(match size { 1 => "char", 2 => "unsigned short", 4 => "unsigned int",
                      8 => if long_is_8 { "unsigned long" } else { "unsigned long long" },
                      _ => return None })
}
```

`declarator_parts` (printc.rs:7609-7613) sets `under_pointer = true` whenever the residual
`TYPE_UNKNOWN` base sits under *any* pointer modifier, so the `realtypes` relabel collapses
`undefined1 *`, `undefined4 *` and `undefined8 *` all to `void *`. Only the **declaration**
text is relabeled; `opLoad` / `opPtradd` keep building index and cast expressions from the
*original* pointee size. Declaration and expression therefore disagree about the stride.

Proof it is this line and nothing upstream of it — the same function with the gate off:

```
$ kuna decompile <stripped> --addr 0x56c0 --option realtypes off
int4 sub_56c0(xunknown8 a0,xunknown4 a1,xunknown4 a2,xunknown8 *a3)
   ...  if ((code *)a3[1])            // stride 8: correct, and self-consistent
```

The pointee really is an 8-byte residual unknown; P5 never mis-typed anything, and P3/P4
are not involved. This is purely the P9 `realtypes` presentation policy (option `realtypes`,
default **on**, phase P9 / subphase `literal-format`).

**Prevalence.** Not a one-off. Across a full `kuna decompile-all` of this 291-function
`libacl.so`: **68** `*(void *)(...)` load/store sites with no access width, and **24
functions** in which a `void *`-declared name is dereferenced or indexed. Every one of those
is uncompilable C.

**Why the reference "wins" this sub-point.** Ghidra keeps the placeholder name
(`undefined8 *`) rather than trying to render a real C type, so its declaration and its
`param_4[1]` agree. kuna's `realtypes` feature is the better idea — it just took the wrong
branch for the pointer case.

## Proposed fix

**Mechanism.** In `realtype_unknown_base`, drop the unconditional `under_pointer → "void"`
early return and relabel the *pointee* by its size with the existing table, so the declared
element type keeps the stride the expressions were built with:

| internal type | today | proposed |
|---|---|---|
| `undefined1 *` | `void *` | `char *` |
| `undefined2 *` | `void *` | `unsigned short *` |
| `undefined4 *` | `void *` | `unsigned int *` |
| `undefined8 *` | `void *` | `unsigned long *` (LP64) |
| unknown of size 0/3/5/6/7/… under a pointer | `void *` | `void *` (keep — no natural C type, and no index expression can be built either) |

`under_pointer` then only selects the *fallback*, not the whole answer. `declarator_parts`
already lays out the `*` chain from the modifier walk, so no other change is needed there.

**Owning files** (all P9): `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`
— `realtype_unknown_base` (~line 8028), `realtype_relabel` (~8015), and the doc comment on
`declarator_parts` (~7607) that currently states the `void *` rule as intended behavior.
The `realtypes` row in `decompiler/crates/kuna-decomp/phases.toml` and the generated
`docs/options.md` both spell out "pointer-to-unknown->void *" in the summary and need the
same wording change; spec chapter for P9 emit likewise.

**Scope.** Small — one match arm plus prose. This is a strict correctness fix (the current
output does not compile and loses the access width), so per the repo rules it needs no new
flag; it rides the existing `realtypes` option, whose `off` value already preserves the
upstream `xunknownN *` rendering for byte-for-byte comparison.

**Risks.**
1. Broad textual churn. Every `void *` declaration and every `*(void *)(...)` access
   changes text, so `docs/baseline.json` / `docs/baseline-stages.json` assertions that
   string-match a `void *` will move. These must be inspected individually — the change is
   only sanctioned where the old text was the uncompilable form. Do **not** re-pin to
   absorb anything else.
2. `(void *)0x0` NULL casts become `(unsigned long *)0x0`. With `truthycond` on (default,
   DIV-36) most such casts are already elided to `if (!a3)`, so the blast radius is smaller
   than it looks, but the interaction should be measured.
3. Genuine opaque pointers (`malloc`/`free` arguments, `void *` in a recovered prototype)
   must be untouched — those are real `TYPE_VOID`, not `TYPE_UNKNOWN`, and
   `realtype_relabel` already returns `None` for them. Worth an explicit regression case:
   `free @ 0x23c0` in this binary prints `void free(void *a0)` and must keep printing that.

**A stage testcase** is easy to build from this binary: two-pass, `option realtypes off` =
`xunknown8 *a3` + `a3[1]`, default = `unsigned long *a3` + `a3[1]`, with the bug state
(`void *a3` + `a3[1]`) never reachable.
