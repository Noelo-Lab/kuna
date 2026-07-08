# [PROPOSAL] Return-storage inference (`undefined16 main` + `char[16]` return → `int` / `return v16 ^ 1`)

Part of the **kuna → IDA Pro parity** program. Gap #2 — the general/stripped case (the
DWARF-having case is handled by `ida-dwarf-prototypes`). **Large — needs human go/no-go.**

## The problem

`fmt/main` returns a bogus 16-byte struct:

```c
// kuna
undefined16 main(uint4 a0,void *a1) { ...
  char v4 [16];
  ...
  v4[0] = v16 ^ 1;    // the real int result
  v4[8] = v22;        // a GARBAGE uninitialized stack value
  return v4;          // returns a 16-byte value
}
// IDA Pro
int __fastcall main(...) { ...
  return (unsigned __int8)v16 ^ 1;
}
```

kuna models `main`'s return as a **16-byte value** (`undefined16`) and materializes it by
writing the genuine result to byte 0 and an uninitialized stack slot (`v22`) to byte 8 — an
`RAX:RDX` (or XMM) pair treated as a struct return. IDA recovers the true return: the low int in
`EAX`. This is pervasive (it recurs at every `return` in the function) and produces *invalid*
output — the phantom `v4[8] = v22` reads uninitialized memory.

## Root cause (hypothesis — step 1 of the work is to confirm)

kuna's return-value / output-storage recovery defaults `main`'s return to a wide register pair
(SysV `RAX:RDX` = 16 bytes, the struct-return storage) rather than inferring the **narrow**
storage the function actually defines and callers actually read (`EAX`, 4 bytes). Because the
high half (`RDX`) is never meaningfully written, it picks up an uninitialized stack spill
(`v22`). The likely seam is the prototype's `ProtoModel` output/return-storage selection +
the "which bytes of the return register are live at every RETURN" analysis (Ghidra's
`ActionReturnRecovery` / `RuleReturnSplit` territory). A focused trace (which pass installs the
16-byte return varnode; where the `RAX:RDX` join is formed) is **step 1**.

## The IDA / Ghidra reference

Ghidra's return-value recovery (`ActionReturnRecovery`, the `ParamMeasure`/`ScoreProtoModel`
return-storage scoring) determines the actual return register+width from the def/use at each
RETURN and the callers' reads, narrowing `RAX:RDX` to `EAX` when only the low bytes carry a
value. IDA's Hex-Rays does the equivalent (microcode return-value analysis).

## Why this is large (needs go/no-go)

1. **It is core return-value dataflow, not a naming/markup pass.** It touches how the return
   storage varnode is chosen and how the RETURN's input is typed/sized across the whole
   function — an S4 (call/proto) + S5 (type) interaction, not an isolated Action.
2. **High blast radius.** Changing return-storage inference perturbs *every* function's return
   rendering and can shift many datatest baselines; it must be validated against the full 675
   corpus with careful re-pinning, and cross-checked against the ported Ghidra
   `ActionReturnRecovery` semantics to avoid regressing the many small functions that already
   return correctly.
3. **Interacts with the ABI/`ProtoModel`.** The 16-byte-vs-4-byte choice is a proto-model/output
   decision; getting it right without breaking genuine struct-by-value returns (which *are*
   `RAX:RDX`) requires the liveness/scoring discrimination Ghidra uses, not a blanket narrowing.

## Relationship to other proposals

For DWARF binaries, `ida-dwarf-prototypes` fixes the *fmt* case directly (applies the `int`
return type → narrows the storage). This proposal is the **general** fix (stripped / no-DWARF),
and the correct end state where both apply.

## Proposed plan (for an approved worker)

1. Trace the 16-byte-return origin on `fmt/main` (instrument the proto output-storage +
   RETURN-input formation; identify the pass that installs `undefined16`).
2. Compare to the ported `ActionReturnRecovery`/return-scoring; find where kuna diverges
   (missing liveness-based narrowing, or a default-wide output storage).
3. Implement the narrowing (return storage = the covered bytes actually defined-and-read),
   guarding genuine struct-by-value returns.
4. Full-corpus baseline validation + re-pin with per-assertion justification.

## Speed / risk

- **Speed:** an added liveness/scoring step at return recovery; likely negligible, confirm.
- **Risk:** HIGH (blast radius above). Mitigation: stage behind careful corpus diffing; this is
  the one item where an `--option` ablation gate may be warranted despite the program's default
  "no flag" policy, at least during bring-up.

## Testing

Real-ELF `verify_*.rs` (`regglobal_fmt_x86_64`: `main` returns `int`, no `[8] = v22` phantom) +
a struct-by-value-return fixture that must STAY 16-byte. Full `make test` re-pin.

## Effort

**Large (multi-week).** Recommend approving as a scoped investigation first (step 1), then a
go/no-go on the implementation once the exact divergence is known.
