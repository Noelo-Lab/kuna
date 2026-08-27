# arraysubfield — sized sub-field rendering for partial accesses into array symbols

Status: **shipped** (was `[PROPOSAL] wideintrender`, PR #288, rescoped to Option A).
Origin: the interp-bee/jit-bee IDA-vs-kuna study (`docs/features/funcboundflow/analysis.md`,
finding #4).

This file is the record of what shipped, what did not, and why the two things that did
not ship are blocked. The original three-option proposal is preserved below in
*What was proposed*.

## The reported symptom

`interp-bee` `eval_result_noun @ 0xa3e0` is `mov %rdi,%rdx; xor %eax,%eax; ret` — a
16-byte struct/`__int128` return where `RAX=0`, `RDX=a0`, i.e. the value `a0 << 64`:

```c
undefined16 sub_a3e0(uint8 a0)
{
  char v1 [16];

  v1[8] = 0;
  v1[0] = a0;
  return v1 << 0x40;
}
```

Two separate defects sit in that four-line function, and only one of them shipped.

1. `v1[0] = a0` names one `char`, so the statement **claims a one-byte store** where the
   program stores eight. `v1[8] = 0` claims the ninth byte. Ghidra emits
   `v1._0_8_` / `v1._8_8_` here; kuna was a regression from upstream. **Fixed.**
2. `return v1 << 0x40;` shifts an array. Ghidra emits this too — it is upstream-faithful,
   not a kuna bug — and it is still not compilable C. **Not fixed** (see *Option B*).

## What shipped: the routing gate (Option A)

`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`, ~10 lines.

Upstream `PrintC::pushSymbolDetail` (`printlanguage.cc:256-258`) routes **every** partial
cover of a mapped Symbol through `pushPartialSymbol` and lets that walk's per-type arms
choose the token. kuna's port routed `TYPE_STRUCT` and `TYPE_UNION` into the walk but split
`TYPE_ARRAY` off into a dedicated branch (`printc.rs:7241`) that computes

```rust
let index = sym_off / elsize;   // and emits name[index]
```

**without ever reading `v.get_size()`**. So any access wider than one element got a
subscript naming a single element.

The correct machinery was already ported and already worked — it was simply unreachable
for a plain array:

- `push_partial_symbol_ir`'s `TYPE_ARRAY` arm (`printc.rs:6620`) carries upstream's
  `TypeArray::getSubEntry` guard, `noff + sz <= elsize`;
- on failure it falls to `PartialEntry::Unnamed(off, sz)` (`printc.rs:6720`) →
  `printlanguage::unnamed_field` (`printc.rs:6812`) → `._0_8_`.

The fix admits plain `TYPE_ARRAY` to the routing gate. **Re-routing, not a size guard on
the standalone branch**: re-routing preserves `arr[3]` for genuine in-element accesses via
the walk's own arm and keeps descending afterwards (an array of unions still resolves to
`arr[3].ffield`), whereas bolting a size guard onto the standalone branch would have
dropped in-element accesses to a bare name. The comment block at `printc.rs:7186` already
argued for exactly this change and asserted it was byte-inert for plain arrays; the
measurement below falsifies that assertion, and the comment now says what is true.

### Gating: unflagged

CLAUDE.md: *a strict bug fix that only corrects wrong output needs no flag; when in doubt,
gate it.* The blast radius was measured **before** deciding. Across 14,080 functions in 12
binaries, 308 functions / 4,953 lines change, and every changed line is one of three
things:

| kind | lines |
|---|---|
| a subscript that spanned elements becoming the sized member (`s_800fd37[1]` → `s_800fd37._1_4_`) | 4,937 |
| a within-element truncation gaining the explicit cast the walk's `finalcast` arm emits (`v7[0]` → `(char)v7[0]`) | 13 |
| a bare array name used as a scalar operand becoming the piece it actually is (`CONCAT22(v7,…)` → `CONCAT22(v7[0]._2_2_,…)`) | 3 |

**Zero** genuine in-element renders move. There is no judgment call in the set, so it ships
unflagged, as a *convergence* onto upstream's own rendering rather than a divergence — a
Convergences row in `docs/history.md`, not a DIV row (the registry's own rule: "a port
defect whose fix moves kuna back onto upstream's default earns no DIV row").

## What did NOT ship, and what blocks it

### Option B — a scalar 16-byte integer type (P5)

Would make `return v1 << 0x40;` legal by typing the value a scalar instead of
`undefined1[16]`. **Blocked on there being no C spelling for it.**
`substrate/dtype.rs:6088` (`if s > self.max_basetype_size.get()` → array of unknown bytes)
is the right site and the change there is small, but `p9_emit/kuna_ctypes.rs:109
(integer_spelling)` only knows `int_size` / `long_size` / `long_long_size` — there is no
16-byte entry, so a scalar 16-byte `TYPE_INT` prints as `uint16`, a kuna type name and
still not compilable C. Option B needs an `__int128` spelling added to the ctypes table
first, and is only worth doing once that exists.

Its payoff is also narrower than it looks. Of the 201 invalid-C functions the scout found
across seven binaries, 147 are 16/32-byte SIMD blobs (AVX2 `pmovmskb` chains,
`SUB161(v >> n, m)`) that are `TYPE_UNKNOWN` and **must** stay arrays — Option B's own
carve-out excludes them.

### Option C — fold `CONCAT88(0,x) << 64` in the mid-end (P3)

Its target subset is not new mid-end work: it is a **coverage gap in the already-ported
`RuleDivOpt`** (`p3_dataflow/ruleaction_6.rs:2094`), which demonstrably fires. In `sort`
`sub_d040 @ 0xd040` the same function recovers `a0 / 3`, `a0 / v3` and `a0 % v3` while
leaving `SUB168(v1 * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe` — the
`(a0/3)*2` half of an `a0 % 3` — raw. Filed as its own issue: **#343**. It has nothing
to do with rendering and belongs in P3.

### The whole-container operand

`return v1 << 0x40;`, `return v1;`, `return v1 * v2;` all survive this PR untouched. The
scout's invalid-C detector — an array used as a bare operand of `<< >> * / % ^ & |`, or
bare-returned — counts **exactly the same 201 functions before and after** over its seven
binaries (236 over the twelve measured here). This PR does not reduce that number by one.
It fixes the *store size*, not the container.

An existing shipped option already gives the valid-but-lossy IDA rendering for the return
subcase, which the original proposal did not mention:

```
$ kuna decompile ./repro.o shift64  --option returnpair single
unsigned long shift64(void) { return 0; }
$ kuna decompile ./repro.o mul_wide --option returnpair single
long mul_wide(long a0,long a1) { return a0 * a1; }
```

`returnpair single` drops the second return register — valid C, lossy, exactly IDA's
behavior. It is orthogonal to this fix: it removes the 16-byte container entirely, so
there is no array symbol left for the partial walk to render, and the two never interact.
`--option ctypes on` does not help either (no 16-byte case in `integer_spelling`).

## What was proposed (superseded)

The original PR proposed three gated, default-off options: **A** sized sub-field writes
(printer-only), **B** scalar wide integers (P5, Ghidra-divergent), **C** a mid-end
`CONCAT88(0,x) << 64` fold. A shipped, unflagged, as above. B is blocked on the ctypes
spelling. C was refiled as #343 because its target is a rule-coverage bug, not new work.
The proposal's framing — "entangled across three subsystems" — overstated the cost of the
tractable part: the variable-merge (P6) and type-factory (P5) legs needed no change at all.

## Test

`tests/stages/kuna-arraysubfield.xml`. Single image, four functions, gcc -O2 -no-pie:
`lo8`/`hi8` read 8 bytes at offsets 0 and 8 of a `char g[16]` global (the repair),
`elem` reads one byte at offset 3 (the preservation), `shift64` is the reported
reproducer. Assertion #7 pins `return v1 << 0x40;` — the part that is *not* fixed.

Because the change is unflagged there is no option to switch off for a two-pass arm; the
before/after is pinned instead by #3 (`min=0`: no size-blind subscript survives on either
wide read) against #1/#2, and reproduced by hand with a pre-fix binary:

```
             before                     after
lo8      return g[0];               return g._0_8_;
hi8      return g[8];               return g._8_8_;
elem     return g[3];               return g[3];        (unchanged)
```

Four existing stage assertions and two `kuna-console` integration-test pins moved — every one
of them a pin on the defective spelling: `ghdec-returncopysplit` #1/#5/#6,
`kuna-retinputhalf` #2, `kuna-rustabi` #5, `verify_return_uncomputed`,
`verify_rustabi_pair`. `verify_return_uncomputed` also carries a `min=0` arm ("nothing should
write the phantom high half", spelled `!code.contains("[8] =")`) which the repair would have
made **vacuous** rather than failing; it was widened to reject both spellings.
`kuna-rustabi` is the clearest witness of the discrimination, since one function contains
both kinds at once:

```c
  v2._1_7_ = 0;            /* was v2[1]   -- a 7-byte write */
  v2[0] = a0 < 0xb;        /* unchanged   -- a genuine 1-byte element write */
  v2._8_4_ = v1;           /* was v2[8]   -- a 4-byte write */
  v2._12_4_ = 0;           /* was v2[0xc] -- a 4-byte write */
```

## Known residual

`v2[0] = (char[8])s_92e50._0_8_;` (`ghdec-returncopysplit` pass 2). The **left** side is a
whole-symbol cover of a `char[8]`: the walk breaks out immediately and returns `false`, so
the standalone branch still fires and renders `v2[0]`. Upstream would render the bare name
`v2` there. That is a second, pre-existing divergence at the same site, untouched by this
PR and deliberately out of its scope — fixing it means deleting the standalone branch
outright, which is a wider change than the routing gate. It is rare: **11 lines** across the
14,080 functions measured, every one a `char[8]` half of a 16-byte `movdqa` copy.
