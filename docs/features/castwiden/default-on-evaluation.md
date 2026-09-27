# castwiden: default-on evaluation

Default `literal`, measured on main 29cfedee0 + this branch (07d37d9ba), both arms of
one build (`--option castwiden off` against the default); (a) (b) (c) (d) (h) re-run
after rebasing onto 243177605 (#718).

| criterion | result |
|---|---|
| (a) `make test` | 5 of 675 assertions move, all the intended form: `dupptr` #1 #2 #5 (`argv[(int8)a + 1]` -> `argv[a + 1L]`), `longdouble` #15 (`ptr[(int8)(val * 3) + 1]` -> `ptr[val * 3 + 1L]`), `retspecial` #3 (`rethidden->c = (int8)num;` -> `= num;`). Each pins upstream's form, so each file opts out with `option castwiden off` (as `union_datatype.xml` does for `castimplied`); `docs/baseline.json` is untouched. 675/675 PARITY OK. |
| (b) `make test-stages` | 16 assertions of other options move, each read and each the intended rule: `kuna-castindex` #1 #6, `kuna-castternary` #7 #8 #9 and `regionstructure-loop` #3 (`(int8)a1 * 4` -> `a1 * 4L` inside an index), `ghdec-subright` #5 #8 (`(uint8)a1 * (uint8)a0` -> `(uint8)a1 * a0`), `structsynth-locals` #2 #5 and `structsynth-nest` #1 #3 (an `int4` field beside `int8` fields), `kuna-dwarfvariants` #7 and `kuna-libctypes` #34 (stores through a field or element of the cast's type), `kuna-callplaceholder` #1 (`strncpy(v7,a0,(int8)(a1 + -1))` -> `strncpy(v7,a0,a1 + -1)`, `size_t` of the cast's width). Each regex was moved to the new text; `docs/baseline-stages.json` re-recorded only to add the 14 `castwiden` keys (1413 -> 1427 on 243177605). 1427/1427 PARITY OK. |
| (c) `make test-cli` | 2 probes move: `bytecode-append-call-loses` (`(char *)((unsigned long)a1 + 0x402000)` -> `(char *)(a1 + 0x402000UL)`) and `protoorder-types-keeps-a-stack-array-whole` (`&v3[(long)argc + 0x14]` -> `&v3[argc + 0x14L]`, probe id recomputed). Neither probe is about the cast; each pins the call or the array it is about with the new spelling. 262/262 on 243177605. |
| (d) 444-slice typesweep | identical: 1,615 perfect (15.03%), mean .3697 in both arms, 0 improved, 0 worse, on all 10,748 functions; on 243177605 1,625 perfect (15.12%), mean .3713 in both arms, 0 improved, 0 worse. The option changes C text only: `decompile-all --json` over fmt O2, ls O0, gzip O2 and tar O0 (2,514 functions, 20,342 variables) differs in `code` alone, so no variable or argument is added or removed. |
| (e) speed | interleaved min-of-15 whole-binary `decompile-all`, off vs default: fmt +0.15%, ls +0.65%, sort -0.03%, bash -1.18%; the worst delta is +0.65%, inside the +5% budget (`speed.json`). |
| (f) whole-corpus sweep | 64 binaries (the 45 castbench builds, 8 outside it, 11 more incl. two 32-bit PEs), 2,341 changed functions, 6,422 changed lines, classified by what the C means (`semhunks.py`, `semhunks.json`): every changed line parses to the same C tree once casts are set aside, every removed cast is an 8-byte integer cast, none sits under a shift, a comparison or a unary op, none of an op's two operands both lose theirs, and every one is typed from the declarations, struct definitions and signatures and gives its operation (or its 64-bit ring chain) the same type, or its destination the cast's type or an 8-byte integer. 0 flagged, 0 unverified. On the review corpus as the first version printed it, the same classifier flags its wrong-value hunks: 2 lost parentheses (mirai), 1 shared widening and 3 regrouped chains (e2fsck). |
| (g) `modes.rs` | coherent: the option is multi-valued and no preset overrides it, so every mode prints the default. |
| (h) castbench | 35,588 -> 34,061 casts on the 4,815 shared functions (0.941x -> 0.901x IDA), 481 functions fewer, 0 more; per 100 statements 29.9 -> 28.7 (IDA 27.0), at O2 31.5 -> 30.4 (IDA 26.1). The off arm is byte-identical to the main 0096e984d arm in all 45 files. Rebased onto 243177605 (#718, +21 casts in the base): 35,609 -> 34,082 (0.942x -> 0.901x), the same 481 functions fewer and 0 more. |

On the landing base 59d38b581 (#729 `callrettype` and #743 `castobject`, both on), re-measured
on the rebased build: (a) 675/675 PARITY OK; (b) 1441/1441 PARITY OK, the stages baseline
re-recorded as main's 1427 keys plus the 14 `castwiden` keys, none missing; (c) `make test-cli` 264/264;
(d) the 444-slice typesweep identical in both arms, 1,631 perfect (15.17%), mean .3717,
all 65,715 type decisions identical; (f) `semhunks.py` over the 45 castbench builds and the 8
outside it: 1,718 changed functions, 4,802 changed lines, 0 flagged; every line that differs
from the earlier runs (castbench on 243177605, corpus8 on 0096e984d) is a renumbered variable, a pointer type the two new options changed
(`unsigned long *` to `FILE **` beside the same `v3 * 8L`), or a call whose result main no
longer widens, plus one new `long - (long)(int)x` to `long - (int)x` argument, verified; (h)
castbench 34,813 -> 33,289 casts (0.920x -> 0.880x IDA), 482 functions fewer, 0 more, per 100
statements 29.3 -> 28.0 (IDA 27.0), at O2 30.9 -> 29.9 (IDA 26.1); the off arm is
byte-identical to main in all 45 files.

Value preservation: the compiled round trip over `castwiden_x86_64.c` (four builds,
three option values, gcc and clang at -O0 and -O2 with `-fwrapv`) prints the binary's
values, including a widened value read twice, a 32-bit sum or product beside a long
of the same operator, and a widened operand beside a chain the printer writes
without parentheses; the unit test checks that the rule accepts exactly the
combinations whose type and value C leaves unchanged.
