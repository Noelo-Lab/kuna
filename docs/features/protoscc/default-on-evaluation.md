# `protoorder cycles` as the default — evaluation

Measured on `2888a2a94` (origin/main), both arms from ONE build of this branch:
`--option protoorder types` (the old default) against `--option protoorder cycles`.

| criterion | result |
|---|---|
| (a) `make test` with the new default | **pass** — `datatests: 675/675 assertions passed`, `PARITY OK`. The console path the datatests run never parks (`park_recovered_proto` is written true only by the whole-program drivers), so no assertion can move. |
| (b) `make test-stages` | **pass** — `datatests: 1296/1296 assertions passed`, `PARITY OK`; nothing re-recorded. No stage test names `protoorder` in its script (same reason as (a)). |
| (c) `make test-cli` | **pass** — `tests/cli: 219/219 passed` before the two new probes, 221/221 with them; no existing probe moved. Two probe notes that called the default `types` were reworded (notes are not part of a probe id). |
| (d) 444-slice typesweep, new default vs old | **pass** — perfect 1,353 -> 1,399 (+46), aggregate 3669.96 -> 3714.21 (+44.26), 46 onto perfect, 0 off, 116 more improved, 1 worse. `char *` true positives 4,232 -> 4,423 (192 gained, 1 lost); no other class moves. |
| (e) speed, interleaved min-of-15, fmt/ls/sort/bash -O2 | **pass** — worst min delta +1.77% (ls). min / median of 15, `types` -> `cycles`: fmt 5306 -> 5231 ms (-1.42% / +3.44%), ls 14690 -> 14950 (+1.77% / -3.99%), sort 16484 -> 15366 (-6.78% / -4.48%), bash 90781 -> 95538 (+5.24% / -2.57%) on a first run whose samples spread 90-124 s in both arms, re-run at a quieter moment: 93763 -> 92940 (-0.88% / +0.27%). `speed-run1.json`, `speed-run2-bash.json`. |
| (f) whole-corpus before/after, 15 binaries | **pass** — 708 of 10,288 functions change; every one classified (below), no hunk outside the option's documented effect. A review over 11 more binaries found one that was: tar -O2 `make_hol`, redone by the `structsynth` sweep, cast its recursive argument to the superseded `struct_54 *` from its own first-pass statement. Fixed (a function never reads its own statement; the sweep forgets statements naming a superseded structure); over all 26 binaries `types` is byte-identical to main, the default differs from the reviewed build only in that line, and no result names a superseded structure after the sweep (`r3c-stale.txt`). |
| (g) `p0_knowledge/modes.rs` | **pass** — no preset names `protoorder`; it stays in `EXCLUDED_ON_PURPOSE` (a run order, not a pass), whose comment now names `cycles` as its default. |

## (d) The worse row

coreutils -O2 `ginstall` `install_file_in_file` (0.16 -> 0.12): `to_relname` goes
from `char *` to `stat *`. The chain is `install_file_in_file` -> `copy` ->
`copy_internal`, and `copy_internal` (in a cycle with `copy_dir`) recovers its own
`dst_relname` as `stat *a3` under both values. Under `types` it stated nothing;
under `cycles` it states that wrong type, `copy` takes it, and so does its caller.
A recovered vote is only as good as the callee's own recovery, which is the
documented cost of the option (`use_when`: "flip to `types` if a caller's argument
takes a type from a recursive callee whose own recovery is wrong about it").

## (f) The corpus

Binaries: -O0 cp, tar, find, diff; -O2 ls, sort, fmt, grep, gzip, bash; -O2-noinline
find, cp; ARM Cortex-M nuttx, ChibiOS `ch.elf`, FreeRTOS `RTOSDemo.out`.
`corpus-report.json` (classifier `docs/features/protoorder/corpus-diff.py`):

| class | functions |
|---|---|
| casts | 295 |
| declarations | 280 |
| struct-number | 80 |
| literal | 19 |
| member-reach | 19 |
| pointer-arith | 3 |
| merge | 1 |
| REVIEW, read by hand (`review-labels.json`) | 11: member-reach 4, regroup 3, store-width 2, pointer-arith 1, wrong-callee-vote 1 |

No stack object split or merged, no stack local read without a write, no call
made or lost, no `goto` or `return` moved. Both values render the same 90,420
call arguments, 18,556 argument rows and 342 phantom arguments, with and without
`argclobber` (`callsite.json`); `argclobber` changes the same one function under
both (fmt -O2 `main`).

The two store-width rows are improvements: bash `named_function_string` and
`sub_466f0` printed one `movq $0x0,0x10(%rax)` as two `int4` stores through an
`int4 *`, and print it as one 8-byte field store through the struct pointer the
callee states. The wrong-callee-vote row is nuttx `sub_801295c`, whose literal
`4` (a qsort element size) renders `&UndefinedInstruction` because nuttx's
self-recursive qsort recovers its own `size` parameter as `char *`; the value is
unchanged, and it is the same residual as the (d) row.
