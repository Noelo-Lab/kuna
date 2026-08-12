# [PROPOSAL] IFUNC-stub tail calls & FP-return recovery

Status: **proposal** (large / multi-part — no implementation in this PR).
Origin: the interp-bee/jit-bee IDA-vs-kuna study (`docs/features/funcboundflow/analysis.md`, finding #2).

## Symptom

`f64_log_base @ 0xf090` is `log(arg2) / log(arg1)`:

```
IDA:   double sub_F090(double a1, double a2)        KUNA:  float8 sub_f090(void)
       { double v3 = sub_15620(a2);                        { float8 v1; float8 v2;
         return v3 / sub_15620(a1); }                        sub_15620(); sub_15620();
                                                             return v1 / v2; }   // v1,v2 UNINITIALIZED
```

kuna drops the `double` parameters and reads `xmm0` uninitialized, because it types
the callee `sub_15620` (a glibc `log` errno wrapper) as returning **`void`**. Every
`f64_*` math wrapper in the binary has the same shape, so this is the whole
math-function family.

## Root cause (a five-link chain)

1. `sub_15620` sets errno then **tail-jumps** to `0x7350` (`jmp 0x7350`), a
   `.plt.sec` **IFUNC** stub (`endbr64; jmp *0xdef20(%rip)`).
2. The stub's GOT slot `0xdef20` carries an `R_X86_64_IRELATIVE` relocation
   (→ resolver `0x15c80`) — **no symbol**. `elf_plt.rs::build_got_name_map` only
   maps symbol-bearing relocations, so the stub gets no `PltSym`/`ImportSym`, and
   **no function symbol is created at `0x7350`**.
3. Because `0x7350` is not a known function entry, `query_call(0x7350)` is `None`,
   so `tailcalljump` (default-on) does **not** fire on `sub_15620`'s `jmp 0x7350`.
   Flow instead walks INTO the stub, whose `jmp *GOT` becomes an unresolved
   `CALLIND`, and `sub_15620` renders `(*dat_def20)(); return;`.
4. With the tail call unrecovered, `sub_15620` has no value at its return → kuna
   concludes the prototype returns **`void`**.
5. That `void` prototype is applied to every caller. IDA instead infers `double`
   from the **caller's** use of `xmm0` after the call (call-site output recovery),
   but a `void` callee prototype pre-empts that inference in kuna.

## Proposed fix (staged, each stage independently useful, all gated)

New option `ifuncfpret` (P1, default-off opt-in; flip to default-on later per DIV
process if 0/675).

- **Stage A — discover `.plt.sec` IFUNC stubs as functions.** In `elf_plt.rs`, for
  a `.plt.sec` stub whose GOT slot has an `R_X86_64_IRELATIVE` reloc, emit a
  function entry at the stub addr (named `ifunc_<resolver>` or left `sub_<addr>`).
  → `query_call(0x7350)` resolves; `tailcalljump` fires; `sub_15620` renders
  `return sub_7350(...)` instead of `(*dat_def20)()`. (Rendering win; return still
  unknown.)
- **Stage B — indirect tail-return recovery.** Recover `jmp *GOT` (the stub body)
  as `return (*ptr)(...)` (tail-return) rather than `(*ptr)(); return;`, so the
  stub's return is the GOT target's return (unknown, not void).
- **Stage C — don't conclude `void` on an inconclusive tail.** When a function's
  only return path is an unresolved indirect tail-call, leave the return **unknown**
  rather than `void`, so call-site output recovery is not pre-empted.
- **Stage D — call-site xmm0 output recovery.** With the callee return unknown,
  a caller that reads `xmm0` after the call recovers a `double` return (kuna's
  ParamMeasure/output path already does this for known cases; verify it fires here).

## Test strategy

- Minimal reproducer: a `-fcf-protection` static binary with an IFUNC (`__attribute__((ifunc(...)))`)
  double-returning function tail-called through a `.plt.sec` stub. Two-pass stage
  test: option off = `void` + `(*dat)()`; option on = `double` + named tail call.
- Real oracle: `interp-bee` `f64_log_base @ 0xf090` and the `f64_*` family.

## Effort / risk

Multi-part, analysis-tier (loader + flow + prototype recovery). Medium-high. Gated
default-off = byte-identical (all gates green) until the chain is validated end-to-end;
the value is realized only with all four stages, so it ships as one option flip.
