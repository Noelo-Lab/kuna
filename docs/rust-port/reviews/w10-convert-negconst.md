# Convert negative-constant render (w10-convert-negconst) — ACCEPT, +10

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-convert-negconst` @ `407cfc2`,
base 843b0e2/413). Verdict: **ACCEPT**.

## Root divergence

`printc.rs` `push_constant_ir_fmt` hardcoded `sign=false` into `resolve_integer_format`,
dropping the C++ `pushConstant` metatype dispatch. C++ (`printc.cc:1813` `pushConstant`, line
1832) passes `sign=true` for `TYPE_INT`, and `push_integer` (`printc.cc:1381-1391`) strips a
set top bit into a leading `-` plus the two's-complement magnitude. So a negative
`recv_signed(int4)` convert rendered its full unsigned bit pattern (`4294966784`/`0xfffff448`/
`037777777445`/`0b…`) instead of `-512`/`-0xbb8`/`-0333`/`-0b1100110011001100`.

## Fix

Added `push_constant_ir_fmt_sign`; at the constant render site `push_vn_explicit_ir` set
`sign = ct.get_metatype() == TYPE_INT`, faithfully mirroring `pushConstant`. The `force_char`
short-circuit (printc.cc:1381) preserves Convert #17's `L'a'`. Predicate-driven, no hardcoded
constants/addresses.

## Effect

413 → 423 (+10). Target: Convert #2/#6/#10/#14. Collateral from the same signed-render fix:
Bitfields #23, MIPS Bitfields #23, If/Switch #1, Intermediate pointers #10, Partial splitting
#7, Signed byte #3.

## Gate (re-run at integration onto 413)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 423]`; full per-assertion passing-set diff vs base 413: **regressed-set
  EMPTY** (gained exactly the 10 above). All 17 Converts pass.
- switch cluster: switchind 16/16, switchmulti 8/9 — unchanged.
- oracle **PARITY OK**; C++ tree byte-untouched.

## Note

The disclosure-pin `verify_w10_callarg_piece_switchhide_guard.rs` moved with the render: the
signed PTRSUB byte-offset now prints `PTRSUB(v1,-0x18)` (was `0xffffffffffffffe8`) — strictly
closer to the oracle; the pin was updated to record the more-faithful render.
