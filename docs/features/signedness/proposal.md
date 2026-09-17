# `signedness` — proposal

**Status: draft, awaiting go/no-go.** A working prototype is on the branch, with
the measurements below. Nothing here changes default output.

## What is being proposed

A value option `signedness upstream|auto|prefer-signed|prefer-unsigned` (default
`upstream`, byte-identical to not having it) that decides a declared integer
local's signedness at the P9 declaration seam from the operations the body
applies to it, instead of from whichever type-inference vote happened to be most
specific. This is TRex's *type rounding* (USENIX Security 2025, §3.3.5 / §5.1)
reduced to the one decision kuna can make without touching its type lattice.

## The three questions for the reviewer

**1. Is the correctness rule airtight?** The rule is: flip only when *every*
signedness-sensitive reader of the value agrees, where "signedness-sensitive" is
the whitelist enumerated in the module header and in `docs/spec/09-emission.md`,
the walk follows implied (inlined) results so a change of C type cannot escape
into a surrounding expression, and any unclassified reader vetoes the variable
outright. The whitelist coincides with the ops kuna's own cast strategy coerces
with `care_uint_int = true`, which is upstream's own statement of where
signedness carries meaning.

Measured, not argued: 42 functions change across `fmt`/`ls`/`sort`/`du` at `-O0`
and `-O2`, every hunk is a declaration flip or a dropped cast and nothing else;
15 of them build into a two-version program with stubbed callees and a shared
memory arena, and 2,000 pseudo-random input vectors per function produce **0**
divergences (6 run to completion, 5 fault identically in both arms, 4 are glibc
aborts inside the harness's own `realloc`); the 27 that will not compile produce
the identical gcc error set in both arms.

**2. Is a readability-only option worth the option budget?** It buys **zero**
metric. `extract_variables` exports prototype parameter types, `ScopeLocal`
stack-symbol types and `framelayout` slots — never a HighVariable declaration —
and decbench normalizes `unsigned` away anyway. `structscore --trex`, whose last
scoring step *is* signedness, is byte-identical with the option on (mean 1.7322,
`c_primitive` 58/95 on `fmt -O2`). What it buys is 63 casts that stop being
written and 52 declarations that stop contradicting their own bodies.

**3. Should `auto` become the default?** It already clears the repo's bar: built
with `auto` as the default, `make test` is 675/675 **PARITY OK** and
`make test-stages` is **PARITY OK** — zero assertions move on either corpus —
and the speed delta is +0.13% (two interleaved min-of-15 runs pooled;
per-run minima +4.29% and +0.13%, median +3.91%), well inside the +5% budget.
This PR still ships `upstream`; a flip would be its own PR.

## Witnesses (coreutils `fmt` `-O2`, CLI vocabulary)

```
$ kuna decompile-all fmt --addr 0x3060                      # today
  unsigned int v1;
  unsigned int v3;
    if (0 <= (int)v1)
    } while (v5 < (int)v3 >> 3);
  while (v4 < (int)v1) {

$ kuna decompile-all fmt --addr 0x3060 --option signedness auto
  int v1;
  int v3;
    if (0 <= v1)
    } while (v5 < v3 >> 3);
  while (v4 < v1) {
```

The 8-byte form is `sub_7130`:

```
  unsigned long v3;                          ->  long v3;
    if (0x80 <= (long)v3) goto label_71ee;   ->    if (0x80 <= v3) goto label_71ee;
```

Three functions that do **not** move are the rule working, and all three were
proposed as witnesses in the design note:

* `sub_74d0` also prints `if (0 <= (long)v28)`, but the same `v28` is read as
  `v28 >> 0x36` — an unsigned shift — so the demands disagree and `auto`
  declines. (The design note expected this one to flip; it should not.)
* `sub_7900`'s `unsigned long v3; if ((int)v3)` is a **width** change: `(int)v3`
  is a `SUBPIECE` rendered as a cast, which is classified `Opaque` here, so the
  option leaves that line alone. Narrowing a declaration is a different decision.
* `fmt::main`'s `unsigned int v12` has no signedness-sensitive reader at all —
  only `^`, `&`, assignments and the `return` — so `auto` leaves it and
  `prefer-signed` declares it `int` (measured:
  `--option signedness prefer-signed` also moves `v10`, `v11`, `v9` to `long`
  and `v3` to `int` in that function).

## Open design points the reviewer may want to overrule

* **`INT_ZEXT` as a definition is silent** (TRex's `ZeroExtendTgt => None`) while
  as a *reader* it is a hard unsigned demand. Without that asymmetry every value
  a 32-bit x86-64 instruction widens into its 64-bit register is vetoed and the
  option does nothing. The argument is that a zero-extension describes the
  *source* operand's type, and `int v = *p;` with an `unsigned char *p` is
  ordinary C.
* **`INT_SEXT`/`INT_2COMP` as definitions vote signed** but `INT_ZEXT` does not.
  This asymmetry is TRex's (`SignExtendTgt => Some(true)`), not ours. It can only
  ever make a flip more likely, never make output wrong, because a definition
  never constrains the printed C.
* **`INT_LEFT` is neutral, not unsigned.** TRex marks `LeftShift` unsigned with
  an "XXX: should this be `None`?" comment in its own source; `<<` produces the
  same bits either way in two's complement, so it is classified as carrying the
  operand's type rather than demanding a signedness.
* **1-byte values re-sign to `int1`, never `char`.** `get_base_no_char` on the
  signed side; widening a byte into a character type is a different decision with
  its own option. One such flip appears in the sweep, rendered `signed char`.
* **`prefer-signed`/`prefer-unsigned` are unmeasured** beyond "they compile and
  they only touch values with no demand at all". If the reviewer wants them
  dropped, `auto` alone is the shippable core.
