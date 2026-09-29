# castobject — default-on evaluation

Measured on this branch's build on `origin/main` 29cfedee0, both arms from the
same binary: the default (on) against `--option castobject off`. With the option
off the branch is byte-identical to the castbench arm of `origin/main` 0096e984d
on all 45 castbench binaries (the commits since touch no ELF type or cast path).

This is the second evaluation. The first one counted votes over the slot's
readers and moved the declaration when the net removed casts, and a slot stored
from a register kept its variable at `unsigned int` under the new `int`
declaration, so the readers that wanted it unsigned printed with no cast (`v2 >>
0x14` for a logical shift, `0x100 <= v2` for an unsigned compare) and a
zero-extension printed as `ZEXT48(v1)`. The option now declines whenever any
reader wants the other sign, and re-checks the merged variable before the cast
pass (analysis.md section 2). Every number below is from the new build.

| criterion | result |
|---|---|
| (a) `make test` with the new default | 675/675 assertions, PARITY OK; none moved |
| (b) `make test-stages` | 1421/1421, PARITY OK; the only new keys are the 11 of `tests/stages/kuna-castobject.xml`, whose pass 1 sets the option off explicitly; `docs/baseline-stages.json` re-recorded for those 11 only (0 removed) |
| (c) `make test-cli` | 247/247; no probe pins an out-parameter declaration |
| (d) 444-slice typesweep (coreutils grep gzip diffutils bzip2 findutils tar shadow, O0/O2/O2-noinline, metric pinned at 625e892) | 1,615 -> 1,615 perfect, aggregate 3973.97 -> 3973.97, 0 improved / 0 worse / 0 moved. 10,738 of 10,748 functions have byte-identical variables; in the other 10 the retyped local is `int` in DWARF every time (status / wstatus / wait_status in ginstall strip, split closeout, timeout main, sort reap, diff finish_output, useradd tallylog_reset, tar sys_exec_info_script sys_wait_for_child wait_for_grandchild sys_wait_command), which type_match already scored as a match at `unsigned int`; split closeout's other stack locals only change their `vN` numbers |
| (e) speed, interleaved min-of-15 whole-binary `decompile-all` (speed.py, speed.json) | fmt O2 -1.55%, ls O2 -0.48%, sort O2 +4.91%, bash O2 -1.05% (min); sort ran under heavy machine load and re-ran at -3.18% on a quieter machine; within +5% either way |
| (f) whole-corpus hunks | corpus-hunks.txt: the 45 castbench binaries (3 change) and 36 more (the 24 `waitpid`/`wait` importers and 12 more: ginstall, dash and dpkg-divert at O0 and O2, useradd, passwd, scp, sftp-server and id at O0, mkdir at O2; 10 change). Every changed line is DECL/CALL/SHIFT/USUF/STORE, except split -O0 `sub_3e43`, where the register copy of the status takes `int` too and merges with the `int` errno temporary (MERGE, one declaration fewer, later register variables renumbered) |
| (g) `p0_knowledge/modes.rs` | default-on, so it is not in `AGGRESSIVE_OVERRIDES`; `aggressive_carries_every_default_off_option` does not apply |
| (h) castbench, full set, both directions | 35,588 -> 35,572 casts on the 4,815 functions shared with IDA (0.941x IDA either way at three decimals; per 100 statements 29.9 either way, O0 32.3 -> 32.2); 6 functions fewer, 0 more |

Every function the option changes on the castbench set declares `int stat_loc`
in IDA at the same address (sort -O0 0x48cf, diff -O0 0x11e1b, tar -O0 0x32ca8
0x33179 0x343ea 0x345e7).

Value preservation: the round trip compiles every printed fixture function with
gcc and clang at -O0 and -O2 in both arms, over objects initialized from a
parameter and run with top-bit values. Fifteen more such functions (a logical
shift, an unsigned compare, a zero-extension, a divide, a `gid_t` read signed,
each initialized from a parameter), built by gcc and clang at -O0 and -O2 and
decompiled, print what the binaries print in 32 of 32 rebuilt ON programs; the first
version mismatched in every build from the gcc -O0, gcc -O2 and clang -O0
fixtures.

Flipped: every criterion passes.

## Landing re-measurement on origin/main b273c2259

origin/main gained #718 (`callbacktype`) and #729 (`callrettype`) after the
numbers above. Re-measured on the rebased build, the default against
`--option castobject off`:

| criterion | result |
|---|---|
| castbench, full set | 34,829 -> 34,813 casts on the 4,815 functions shared with IDA (0.921x -> 0.920x IDA; per 100 statements 29.3 -> 29.3); 6 functions fewer (16 casts), 0 more; the same six functions as before. With the option off all 45 castbench files are byte-identical to the castbench arm of origin/main b273c2259 |
| 444-slice typesweep | 1,631 -> 1,631 perfect, aggregate 3995.12 -> 3995.12; 0 functions move onto perfect and 0 off it, 0 improve and 0 get worse; 10,738 functions have byte-identical variables and the other 10 score the same |
| hunks, `decompile-all` off vs on | sort -O0: 1 changed, 1 DECL/CALL/SHIFT/USUF; split -O0: 1 changed, 1 MERGE (0x3e43); diff -O0: 1 changed, 1 DECL/CALL/SHIFT/USUF; tar -O0: 4 changed, 4 DECL/CALL/SHIFT/USUF; nothing outside the classes of corpus-hunks.txt |
