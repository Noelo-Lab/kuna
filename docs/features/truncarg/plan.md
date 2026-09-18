# truncarg: implementation plan

## 1. The change

A strict fix in emission, with no option: it only adds a conversion the binary
performs. `p9_emit/kuna_truncarg.rs (narrowed_arg_cast)` is consulted by the
default arm of `get_input_cast` when `cast_standard` asks for nothing. For a
CALL/CALLIND input narrower than `int`, integer- or unknown-typed, not an
enum, and in a slot no declared prototype type-locks, it casts the argument to
the unsigned integer of its width unless one of these holds:

- the argument is a ZEXT;
- `int_promotion_type` (for an expression) or the natural extension of the
  printed type (for a variable, load or cast) already zero-extends;
- `c_range` bounds the expression's C value inside `[0, 2^(8*size))`, over
  constants with a clear sign bit, truth values and `+ * & | ^` of those.

An argument that already prints as a truncating SUBPIECE cast, all of whose
reads are call arguments, is retyped to the unsigned type instead of getting
a second cast (`retype_truncation`).

## 2. The unknown byte

The promotion check runs through `FuncdataCastContext::with_unknown_byte_as`,
a view that reports a non-constant one-byte TYPE_UNKNOWN as `int1`, so the
check sees the signedness the printer will spell. The view is on only when
`ArchContext::unknown_byte_is_char` is set, which the per-function seam copies
from `realtypes && out_lang == C`. With `realtypes` off, or in Rust output, the
byte is spelled unsigned and the upstream reading stands.

## 3. Tests

- `tests/stages/kuna-truncarg.xml`: ten assertions. #1-#4 and #8 fail on main.
  #5-#7, #9 and #10 are guards (a plain input, a sign extension, a declared
  narrow parameter, an AND of unsigned bytes, a sum that fits its byte). #10
  fails with `c_range` disabled.
- `tests/stages/kuna-endptrbound.xml` #5: its `push_back(v2,*v3)` argument is
  a `movzx edx, byte [rsi]` byte and now reads `push_back(v2,(uint1)*v3)`.
- `roundtrip/roundtrip.sh <kuna>`: compile-and-run check of eight callers.

## 4. Verification

The four gates plus `make test-cli`, `kuna catalog --check` and
`counters --check`; the corpus diff classified by `classify_diff.py` and
`breakdown.py`; typesweep over the 444-slice set; interleaved min-of-15 timing
of `decompile-all` (`record.json`).
