# Analysis — `setlocale` return type / return-value (follow-up on PR #59)

## The report

Reviewer, on the `tee_O2` `setlocale_null_androidfix` function (PR #59):

> Why are we not returning the setlocale call? Why is our function type wrong?
> It should be `char *`.

- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tee_O2`
  (coreutils `tee`, x86-64, GCC `-O2`), not stripped.
- **Function:** `setlocale_null_androidfix` @ `0x60e0` — a gnulib helper whose entire body is:

  ```c
  char *setlocale_null_androidfix(int category) {
      return setlocale(category, NULL);
  }
  ```

  At `-O2` it is a **tail jump**: `xor %esi,%esi ; jmp setlocale@plt`.

- angr (9.2.213) renders:

  ```c
  char *setlocale_null_androidfix(void) {
      int v1;  // edi
      return setlocale(v1, NULL);
  }
  ```

There are **two independent defects** behind the one report. PR #59 (the
`tailcalljump` option, default-off) fixes the *flow* defect — recovering the
`jmp setlocale@plt` as a `CALL setlocale` + `RETURN` instead of inlining the PLT
thunk into a `(*dat_209f68)(...)` indirect call. After PR #59 the function reads:

```c
void setlocale_null_androidfix(unsigned long a0) {
    setlocale(a0,0);     // tailcalljump on
    return;
}
```

i.e. the call is now *named* `setlocale`, but the two issues the reviewer flags
remain: (1) the type is `void`/`unsigned long`, not `char *`/`int`; (2) the
`setlocale(...)` result is dropped. This document pins both.

## Defect 1 — missing libc prototype (the *type* half) — FIXED here

### Root cause

kuna's `ApplyDataArchiveAnalyzer` analog — `kuna-analysis/src/s1_protos/mod.rs`,
the built-in `LIBC` prototype table that stands in for Ghidra's `.gdt` archives —
**did not contain `setlocale`**. So:

- the `setlocale(...)` call's result was an untyped `undefined8`;
- its parameters were untyped, so `category` came out `unsigned long` instead of
  `int` and the NULL `locale` argument rendered as a bare `0`;
- `main`'s `setlocale(LC_ALL, "")` idiom (the most common real use) rendered
  `setlocale(6,0x6dc1)` — losing the empty-string literal.

This is the same class of gap that `puts`/`printf`/`strchr`/`malloc`/… already
have entries for in the table; `setlocale` was simply absent.

### Fix

One row added to `LIBC`:

```rust
// locale.h — char *setlocale(int category, const char *locale)
("setlocale", Sig { ret: Ty::CharPtr, params: &[Ty::Int, Ty::CharPtr], vararg: -1 }),
```

The pass already matches table names against the object's FUNC/import symbols
and parks each `PrototypePieces` on its callee via
`Architecture::set_function_prototype_pieces`; `ActionDefaultParams` reads it
back when typing callers. No other change.

### Effect (real ELF, full analysis tier)

| | before | after |
|---|---|---|
| `main`'s call | `setlocale(6,0x6dc1)` | `setlocale(6,"")` |
| a normal `char *r = setlocale(c,0); return r;` wrapper | `void`/`undefined8` typing (rax stored to stack kept the value alive, but untyped) | `char *f(int4) { ... return setlocale(c,(char *)0x0); }` |
| `setlocale_null_androidfix` (PR #59 `tailcalljump on`) | `void f(unsigned long) { setlocale(a0,0); return; }` | `void f(int4) { setlocale(a0,(char *)0x0); return; }` — params/arg now typed, **return still dropped (Defect 2)** |

So the prototype fix corrects the **type** of the call, its parameters, and the
NULL argument everywhere, and fully fixes any wrapper whose return register is
kept alive by an intervening use. It is parity-safe (0/675) and default-on
(DIV-11). It does **not**, on its own, make `setlocale_null_androidfix` return
`char *` — that is Defect 2.

## Defect 2 — tail-position `return F()` value not threaded (the *return* half) — NOT fixed here

### Root cause

When a function's whole body is `return F();` and the compiler emits a
tail-position call with **no intervening use of the return register** —

```
…
call F            ; rax := F(...)
leave / (nothing)
ret               ; (or, at -O2, `jmp F` recovered by PR #59 to CALL F + RETURN)
```

— kuna does not recover that the function returns F's result. The function comes
out `void` with a bare `return;`, and the call's output is dropped. The raw p-code
is:

```
0x..:a:  call fsetlocale(...)        ; <-- NO output varnode (no `RAX = call …`)
0x..:b:  return(#0x1:4)              ; <-- NO return value (only the iop constant)
```

This is **general**, not `setlocale`-specific and **not** caused by the missing
prototype: with the curated `char *` prototype in place,

```c
char *wrap_strchr(char *s, int c) { return strchr(s, c); }
```

(`strchr` has been in the `LIBC` table since the beginning) decompiles identically
to `void wrap_strchr(char *,int4) { strchr(...); return; }`. The deciding factor
is purely the instruction shape `call F; ret` with rax untouched in between — the
exact same shape as a `-O2` tail call. By contrast, GCC `-O0` for `char *r = F();
return r;` spills rax to the stack and reloads it (`mov %rax,-0x8(%rbp); mov
-0x8(%rbp),%rax; ret`); that intervening use keeps rax live and kuna *does* return
it. So the bug only shows when the return register flows straight from the trailing
call into the `ret`.

### Where it lives

The return-value recovery chain is:

1. heritage `Heritage::guardReturns` (`s3_dataflow/heritage.rs`) adds the output
   register (rax) as a candidate input to every non-halt `RETURN`;
2. `ActionActiveReturn` recovers the *call's* output (rax) iff it is used
   downstream (`check_output_trial_use` → `collect_output_trial_varnodes` over the
   call's `INDIRECT`-creation ops);
3. `ActionReturnRecovery` validates the candidate via `AncestorRealistic` +
   `ancestorOpUse` and writes it onto the `RETURN`.

The circularity that defeats the tail-position shape: the call defines rax via an
`INDIRECT`-creation, so the rax that reaches the `RETURN` is *defined only by the
call*. `AncestorRealistic::enterNode` (`substrate/funcdata_varnode.rs`) treats a
register trial whose ancestry flows **through a call** as "likely `killedbycall`"
and rejects it as an unrealistic return value, so the candidate is dropped and the
call's output is dead-code-eliminated. This is a **faithful port of Ghidra's
behavior** — Ghidra's `AncestorRealistic` has the same `killedbycall` rejection —
so this is not a port regression; it is a place where kuna (like Ghidra by
default) declines to assume a function forwards its callee's return value.

### Why it is a separate, larger follow-up (not folded into this PR)

- It is **not** the reported "missing prototype" — it reproduces with `strchr`
  and any other already-typed libc function.
- A fix is a **new heuristic** ("a tail-position call whose result is otherwise
  unused, in a function with no proven `void` return, forwards the callee's return
  register"), i.e. a change to `AncestorRealistic` / return recovery, which is an
  output-changing change to a core S4/S5 action and would need its own ablation,
  speed gate, and `--option`. That is out of scope for a one-line prototype
  curation and is exactly the "if the real fix is large/architectural, stop at a
  design" case in the task brief.
- It is also entangled with PR #59: on `setlocale_null_androidfix` the artificial
  `RETURN` that the `tailcalljump` recovery plants is a bare `artificial_halt`
  (`flow.rs`, `self.artificial_halt(&addr, 0)`), so the cleanest place to *also*
  forward the value is inside that recovery — which lives on PR #59's branch, not
  on `main`.

**Recommendation:** open a follow-up (option-gated, e.g.
`tailcall_return_forward`) that, for a `call F; ret`/recovered-tail-call whose
callee returns non-`void` and whose result is otherwise unused, forces the call's
output onto the function `RETURN` so `ActionReturnRecovery` keeps it. Validate on
both the `-O0` (`call F; ret`) and the PR-#59 `-O2` (recovered tail) shapes;
`return strchr(...)` / `return setlocale(...)` are the canonical fixtures.

## What this PR ships

- **Defect 1 only:** the `setlocale` `char *` prototype (DIV-11, default-on,
  0/675 parity-safe).
- Unit test `s1_protos::tests::setlocale_signature_is_char_ptr_int_char_ptr`
  (pins the curated signature).
- Stage test `tests/stages/ghangr-setlocale-rettype.xml` (proves the call types
  `(char *)0x0` / resolves to named `setlocale`, vs the pre-fix bare `0`).

## Reproduce

```bash
make binaries
B=/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tee_O2
# the type fix on a real call site:
kuna decompile "$B" main | grep setlocale         # -> setlocale(6,"")
# the surviving Defect 2 (needs PR #59 for the named call; prototype alone here):
kuna decompile "$B" setlocale_null_androidfix --option tailcalljump on
```
