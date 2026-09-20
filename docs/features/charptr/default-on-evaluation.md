# `charptr` default-on evaluation — HELD OPT-IN

Evaluated on `feat/charptr` over base `2e28ece4a`. "Default on" for an option of
this kind means membership in `AGGRESSIVE_OVERRIDES`
(`p0_knowledge/modes.rs`), because `--mode auto` picks `aggressive` under
500 KiB and that is the mode `decompile-all`, the web front-end and the
benchmark all run.

| criterion | result |
|---|---|
| (a) `make test` with the new default | **pass** — 675/675, PARITY OK. The datatest harness applies no mode, so no assertion moves. |
| (b) `make test-stages` | **pass** — PARITY OK; the stage test's pass 1 carries its own `option charptr off`. |
| (c) `make test-cli` | **FAIL** — 203/209. Six probes move. |
| (d) 444-slice typesweep, new default vs old | **pass** — perfect 1,349 -> 1,353, aggregate 3657.76 -> 3662.69, 4 onto perfect, 0 off perfect, 14 improved, 3 worse. improved (18) >= worse (3). |
| (e) `timeit` interleaved min-of-15, fmt/ls/sort -O2 + bash -O2 | **pass** — option-on against the same build with it off: fmt -1.59%, ls -0.37%, sort +2.67%, bash +4.36%; worst +4.36%, inside the +5% budget. |
| (f) whole-corpus `decompile-all` over 8 binaries, every hunk classified | **pass** — every hunk falls in a documented class; 0 arity changes over 2,290 functions. |
| (g) `modes.rs` coherent | held as an `EXCLUDED_ON_PURPOSE` entry with this evaluation cited. |

## The failing criterion

With `("charptr", "on")` in `AGGRESSIVE_OVERRIDES`, six `tests/cli` probes fail,
all of them pinning the pointer spelling a *different* feature produces:

```
FAIL protoorder-types-the-callers-argument
     expected int callee\(unsigned char \*a0,int a1\)                     actual <no match>
     expected unsigned long caller\(unsigned char \*a0,int a1\)            actual <no match>
     expected caller\(\(unsigned char \*\)0x402000,3\);                   actual <no match>
FAIL protoorder-lock-declines-a-register-saturating-callee
FAIL protoorder-off-loses-the-callee-type
FAIL protoorder-types-keeps-a-wide-store-whole
     expected \*\(unsigned long \*\)\(\(long\)a0 \+ 0x10\) = 0x2020726174737575;
FAIL protoorder-types-keeps-a-word-fill-whole
FAIL ptrfromuse-default-declares-a-dereferenced-parameter-void
     expected int fill\(void \*a0,int a1\)                                actual <no match>
```

Each of those probes would have to be re-pinned to `char *`, and one of them
(`protoorder-types-keeps-a-wide-store-whole`) pins a *store rendering* that
moves with the pointee.

## The decision

Held opt-in. The gain is real and one-directional but small — +4.93 of an
aggregate 3657.76, which is +0.13% of the corpus — and it does not buy
re-pinning two other features' regression probes. The class census in
`analysis.md` says why the gain is small and where the rest of the `char *` gap
actually is (register-only ground-truth variables, and frame slots kuna reports
as `undefined8`), so the case for flipping this option should be re-made only
together with that work, not on its own.
