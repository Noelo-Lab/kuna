# Bitfields on typed stack structs — has_bitfields flag wired (w10-bitfields-stack) — ACCEPT, +21

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-bitfields-stack` @ `d39c05e`, base
`36e01a6`/549). Verdict: **ACCEPT**. The session's largest single wave (+21). Direct payoff of the
Chain B stack-symbol struct-typing root — once stack locals carry their struct types, bitfield
members can finally render.

## The two fixes

1. **`get_type_partial_struct_impl` (dtype.rs:5276) now sets the `has_bitfields` flag** — transcribes
   the `TypePartialStruct` ctor (type.cc:2776): partial-of-partial unwrap, and inherit `has_bitfields`
   when the container `has_bit_fields_in_range` over the partial window. Previously the flag was never
   set, so `printc` never took the bitfield-extraction render path for stack-struct members — the code
   was registered-but-inert until the container's `has_bit_fields_in_range` could be consulted.
2. **`op_set_input` (funcdata_op.rs:423) null==null no-op short-circuit** — was `expect("stale vn")`
   panicking when both old and new slot were null; now a no-op, matching `opSetInput`'s
   already-equal early return. Surfaced by the partial-struct teardown path.

## Effect

549 → **573** (+21 over the integration base 552; the wave measured +21 on its own 549 base, and the
gained set is disjoint from the Stack-Return +3, so it composes cleanly). Gained: Bitfields
#11/12/13/17/19/20/21/28/29 + MIPS Bitfields #11/12/13/14/15/16/17/19/20/21/28/29 (x12).

## Gate (at integration onto 552)

- datatests: `[675, 573]`; passing-set diff vs base 552: **regressed-set EMPTY** (gained exactly 21).
- `cargo test --workspace --no-fail-fast`: **0 failures** (the fence — the byte-parity catcher the
  stringmatch oracle misses).
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ tree byte-untouched.
- switch cluster / for-loop cluster / Chain B / Stack-Return all held (empty regressed-set).

## Residual

Bitfields #14/15/16/18 and MIPS #18 still out — they need the HighVariable spacebase-symbol binding
seam (per-byte member render of a sub-field that doesn't bind to a single mapped symbol), distinct
from the has_bitfields type-flag path landed here.

## No special-casing

Faithful type.cc:2776 ctor transcription + the opSetInput no-op; no name/address/value/type-name
hardcoding.
