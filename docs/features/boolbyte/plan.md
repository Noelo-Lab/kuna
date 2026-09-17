# boolbyte — plan

**Option.** `boolbyte on|off`, default `off`. P5, subphase `type-propagation`,
tier `core`, `change_kind = type-inference`, `live_field = bool_byte`.
Module `decompiler/crates/kuna-decomp/src/p5_types/kuna_boolbyte.rs`, consulted
from `build_localtypes` in `coreaction_infertypes.rs` exactly where
`ptrfromuse`'s candidate is folded.

**What it does.** Offers `bool` as one more candidate in `getLocalType`'s fold
for a one-byte Varnode whose every read is a truth test. The fold, not a
replacement seed: `SUB_BOOL` 10 beats `SUB_INT_CHAR` 19 and `SUB_UINT_PLAIN` 16,
and loses to anything more specific.

**The predicate.**

* One byte; not a constant, not type-locked, not covered by a type-locked symbol
  seed, not in the unique space, not persistent (global); if it is a function
  INPUT, storage the prototype model says can carry a parameter; and not one
  byte of a mapped symbol bigger than one byte, because the declaration that
  would change there is the container's.
* *Use side* (a bounded breadth-first walk, visited set, ten hops — exhausting
  them refuses — crossing `COPY`/`MULTIEQUAL`/`INT_ZEXT`): every terminal read is
  a `CBRANCH` condition, a `BOOL_*` operand, or an `INT_EQUAL`/`INT_NOTEQUAL`
  against 0, and at least one is. Everything else refuses — a call argument, a
  store and a `CPUI_INDIRECT` most deliberately. An INDIRECT output is the value
  *after* the op it annotates, so it is the op that says a callee may have
  replaced this byte, not an identity.
* *Def side*, for a Varnode this function writes: non-zero mask at most 1 **and**
  every def is a literal 0/1, a `booloutput` result, or a copy/phi/zero-extension
  of those — an INDIRECT def refuses for the same reason the read does. The mask alone is not enough (`x & 1` has mask 1 and is a parity
  test). Once proven, `== 1`, `& 1`, `^ 1` and a `RETURN` of the value are
  admitted too.
* A function input has no def, so for a parameter the use shape is the whole of
  the evidence. That is an inference about the calling convention, not a proof
  about the value, which is why the option ships off.

**What it deliberately does not do.**

* It does not touch the `TYPE_BOOL` propagation gate at
  `coreaction_infertypes.rs:481-483`. The census (analysis.md §3) shows that gate
  is not what blocks these values, and it is what keeps a seeded bool from
  reaching a value that can exceed 1.
* It does not type globals. A persistent Varnode's declaration belongs to the
  global scope, whose Symbol this rule does not touch, so the disagreement prints
  as `if (!(bool)dat_10978)` on every read. Deciding a global's type needs the
  whole program.
* It does not touch `HighVariable::get_type_representative` (mechanism B-ii).
  That is the separate `boolrep` item, and it is quality-only.

**Verification.**

1. `tests/stages/kuna-boolbyte.xml` — two passes over one bytechunk: the witness
   (`flagparam`, `char a0` -> `bool a0`) and three refusals (a byte also widened
   and added, a byte tested for its low bit, a byte stored through a pointer),
   each identical in both passes.
2. Four unit tests on the fold order and on the fact that produces the bug
   (`get_base(1, TYPE_INT)` is the ASCII `char`).
3. `typesweep` both arms over the 444 campaign slices, reporting improved and
   worsened.
4. Whole-corpus `decompile-all` before/after over eight binaries, every changed
   line classified.
5. `timeit` on a function the option changes, a function it does not, and two
   whole binaries.
