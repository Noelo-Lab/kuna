# piecehi: implementation plan

A strict correctness fix with no option: it only stops the return-pair repair
from dropping bits the function computes into its return register.

## 0. Why

`strip_uncomputed_return_piece` (run by `ActionOutputPrototype`) drops the half
of a two-register return the function never wrote. It took any `PIECE` at a
RETURN for that pair and called a half leftover when its source sat at the
half's own address. `((u64)hi << 32) | lo` at -O0 becomes `RAX = PIECE(ESI,
EDI)`, whose halves are the argument registers themselves, so both read as
leftover: the RETURN was cut to `EDI` and `ESI` lost its only reader
(`unsigned int join_lo_hi(unsigned int a0) { return a0; }`). A computed high
half beside an argument low half kept the high half alone (`return a0 + 1;` at
-O2), returning the high bits as the whole value.

## 1. The change (`p4_calls/kuna_returnuncomputed.rs`)

- `storage_pieces(vn)`: the storage locations a Varnode occupies, most
  significant first: the pieces of its join record, or its own storage.
- `slot_storage(whole, lsb, width)`: where the `width` bytes `lsb` bytes above
  the least significant byte of `whole` are stored (endian-aware; `None` when
  they straddle two join pieces, which falls back to the half's own address).
- `strip_uncomputed_return_piece`: the placement test for each half compares
  against `slot_storage(whole, 0, lo_size)` / `slot_storage(whole, lo_size,
  hi_size)` instead of the half's own address.
- `(true, false)` (computed high half, leftover low half) is left alone unless
  `spans_two_locations(whole)`: the high half of one register is never returned
  by itself.

## 2. Tests

- `kuna_returnuncomputed/tests.rs`: the slot mapping on little- and big-endian
  registers; a never-written high half of one register is still dropped; a
  computed high half of one register is never returned alone.
- `tests/stages/kuna-returnpiece.xml`: seven gcc -O0 functions and gcc -O2
  `join_hi_sum` (10 assertions; 9 fail on main, the control passes on both).
- `kuna-cli/tests/decompile_all_cli.rs`
  `a_value_built_in_one_return_register_round_trips_through_the_printed_c`:
  arity and return width of eight functions in four fixture builds, and the
  printed C compiled by gcc and clang at -O0 and -O2 must print what the
  fixture prints (main: every build differs; fix: 16/16 match).

## 3. Measurements

Corpus footprint, 444-slice typesweep with arity counters, castbench, speed:
see `record.json`.
