# funcboundflow — bound fall-through at a known function entry

## Where it came from

An IDA-vs-kuna study of two static-pie, stripped x86-64 "Bee" language runtimes
(`interp-bee`, `jit-bee`), decompiled with `kuna decompile-project`. 30 functions
were sampled from the Bee-owned application range `0x7c20..0x155af` (10 small / 10
medium / 10 big) and diffed against IDA Pro at the same addresses. The single
highest-impact divergence, present across the whole sample, was **cross-function
merge**: kuna decoding the *next* function's body into the current one.

## The bug

A kuna `FunctionSymbol` is an entry address with **no extent**, and the S2 flow
follower (`p2_lift/flow.rs`) treats CALL/CALLIND as fall-through — it stops only at
a RETURN or a callee it can *prove* no-return (`query_call_no_return`). So a
function whose last act is a `call` to a routine kuna cannot prove no-return:

- In a stripped, statically-linked binary that is exactly the unnamed `exit` /
  `abort` / `__stack_chk_fail` bodies and every app-level `die()` / `throw`
  wrapper built on them — no symbol to key `noreturn_known` off, no PLT name to
  key `noreturn_externmatch` off.
- The compiler emits the `call` with **no trailing `ret`**, only inter-function
  alignment padding, then the next function.

kuna's follower runs the padding's fall-through straight into the next function's
entry and decodes *that* function's body into the current one. The following
function is emitted twice: once correctly under its own entry, once as a garbage
tail of its predecessor (dead code after the `die()` call, uninitialized reads, a
second unrelated function inlined).

### Confirmed example

`numeric_item_to_i64 @ 0xa070` — `.eh_frame` FDE `0xa070..0xa0a8`, last instruction
`call bee_error_throw` (→ `exit`). kuna followed the fall-through and absorbed the
whole bodies of `require_numeric_pair @ 0xa0b0` and `print_value_lines @ 0xa100` —
both of which kuna *also* lists as their own functions. IDA 9.2 and Ghidra both
bound decompilation to the function body and stop at `0xa0a8`.

```
IDA (correct):                              kuna, funcboundflow off (the bug):
if ( v6 != 1 )                              if (*(int4*)(a0+4) != 1) {
  sub_A230("domain error: ...");             sub_a230();
return (int)*(double*)(...);                  v1 = *(int4*)&v7[4];        // require_numeric_pair
                                              ... fprintf/free loops ...   // print_value_lines
                                              return sub_30270(v3); }      // (next functions)
                                            return (int8)*(float8*)(...);
```

### Prevalence

**70 of 195 (36%)** Bee application functions end in a `call rel32` to a no-return
target (last instruction, no `ret` after) — every one is a merge site. The same
pattern pervades the statically-linked glibc.

## The fix

`p2_lift/kuna_funcboundflow.rs (kuna_should_bound_at_entry)` is the decision; the
truncation lives at the fall-through push of `flow.rs (FlowInfo::process_instruction)`.
When the fall-through target is the entry of another known function
(`query_call(next).is_some()`) and is not the current function's own entry, kuna
plants a no-return artificial RETURN (mirroring `check_for_flow_modification`'s
no-return-call halt) instead of decoding the target, and emits a `funcboundflow`
truncation warning. `option funcboundflow` (default on, DIV-67); the gate is
fast-pathed so default-off does zero per-instruction symbol lookups.

It **overlaps** `noreturn_extern` / `noreturn_externmatch` (which stop the same
leak by callee *name*) but is name-independent — it bounds at the function
*boundary* whatever the callee, which is why it fixes the stripped-static case the
name-based options cannot reach.

## Other divergences the study found (not fixed here)

Recorded for follow-up PRs:

1. **FP return/param recovery for unidentified callees** — `f64_log_base @ 0xf090`
   is `log(a2)/log(a1)`; kuna types the callee (glibc `log`) as `void`, so callers
   read `xmm0` uninitialized and lose their `double` params. IDA recovers it.
2. **Unrecovered jump tables → indirect calls** — `bird_codepoint_to_opcode @ 0xae20`
   renders `(*(code*)(v1 + 0xb0e3c))()` for a PIC relative jump table IDA recovers
   as a `switch` (205 `// jump-as-call` sites file-wide).
3. **128-bit reg-pair / `__int128` return → invalid C** — `eval_result_noun @ 0xa3e0`
   renders `char v1[16]; ... return v1 << 0x40;` (array shift is invalid C); the
   16-byte value is merged into a `char[16]` local (20 sites in interp-bee).
4. **`.rodata` float constants shown as `dat_`** — `f64_div_bee @ 0xf2a0` returns
   `dat_b0c90` where IDA shows `INFINITY` / `-INFINITY`.

On the bulk of the sample kuna is on par with IDA and sometimes cleaner (it keeps
the full 128-bit return where IDA drops a register).
