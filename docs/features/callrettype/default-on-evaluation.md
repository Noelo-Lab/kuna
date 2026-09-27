# callrettype: default-on evaluation

Measured on the branch rebased onto `0096e984d` (origin/main after #720
structheadless, #726 callpush, #727 castindex, #728 castternary and six
commits that do not touch the ELF type or cast paths), and re-checked on
`e38cd49b0`, `a1091b591` and `29cfedee0` as the last section says. The castbench base arm
is the campaign's build of `0096e984d` itself, and `--option callrettype off`
on the branch build prints the same bytes as it on all 45 castbench binaries.
Every other criterion compares `--option callrettype off` with the default on
the same build. The option's cost is its own second decompile.

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675 PARITY OK. No assertion moved; `docs/baseline.json` untouched. A single-function decompile states nothing. |
| (b) | `make test-stages` | 1400/1400 PARITY OK. The only new keys are the three `callrettype #N` assertions of `kuna-callrettype.xml` (a negative control: pass 1 pins `off`); `docs/baseline-stages.json` re-recorded for them on `0096e984d`, nothing else moved. |
| (c) | `make test-cli` | 246/246. One probe moved, with its reason: `protoorder-types-keeps-a-float-in-a-gpr-an-integer` pinned `return (int)(float)h(a0,v1) + v1 + 3;`; `h` is declared `float h(int a0,float a1)` in the same listing, so the `(float)` converted from a type the call does not have, and the probe now pins `return (int)h(a0,v1) + v1 + 3;`. The same line moved in `a_float_in_a_general_register_keeps_its_integer_uses_round_trip`, whose compiled round trip (the printed callers against a bit-preserving `h`) still prints what the source prints. The two `callrettype-*` probes follow the rebuilt fixture's hash. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | option off 1,615 perfect, mean .3697; default 1,621, mean .3701. 6 moved onto perfect, 0 off; 17 improved, 2 worse; `decomp_vars` identical (96,889 both arms), so no variable row is fabricated or lost. The two worse rows: who -O2 `who` (a reused stack slot now typed `char *`, the value `time_string` stores there; the DWARF variable at that offset is another one) and pwd -O2-noinline `file_name_prepend` (a `calleevote` vote through the `dirent` shell). `typesweep-report.md`, `typesweep-moved.csv`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json`, option off vs on on the final build | On the third-review build (`0096e984d`): fmt -0.81%, ls -1.28%, sort -2.49%, bash -0.26% min (`speed-review3.json`); worst +0% of the +5% budget. Earlier builds: fmt +1.44% min / -0.25% median, ls -0.96% min / -0.04% median, sort +2.69% min / +0.03% median, kmod_O2ni +1.34% min / +3.25% median, dpkgdivert_O2 +0.84% min / -0.19% median, bash -0.20% min / +2.17% median. Worst min +2.69% (within +5%). The audit's second decompile is the whole cost the option adds: the first audit (every statement, and votes a later pass refused) cost fmt -O2 +76% and bash -O2 +55% (218 second decompiles, 51.6 s); auditing only statements the variable carries, only the tie contradictions, and only functions up to 1,000 p-code ops brings bash to 26 second decompiles. Re-measured after the rebase onto `f434b6a60` (min of 15): fmt -0.23%, ls -1.52%, sort +1.63%, bash +2.25%. `speed-final.json`, `speed-rebased.json`, `speed.py`. |
| (f) | whole-corpus `decompile-all` before/after, every hunk classified | Ten binaries outside the castbench corpus (bash, dash, kmod, crontab, dpkg-divert, minigzip, xmlwf, e2fsck, sftp, ip; 8,878 functions; numbers from the build after the third review, on `0096e984d`): 1,623 changed; 1,512 differ only in types, conversions and names; 44 only in how a type spells the same expression (a literal's sign or suffix, `&dat_X` for an address, `v[2]` for `*(v + 0x10)`); 67 read by hand (every one in dash, kmod, crontab, dpkg-divert, e2fsck and ip, the largest in bash): the retyped variable's accesses spelled as members or elements, character literals, a ternary printed as if/else, variable renumbering. No call changed its argument count, no function's return became or stopped being `void`, and no `variables[]` row appeared or disappeared. `corpus-hunks.json`, `corpus.py`, `hunkclass.py`. |
| (g) | `modes.rs` | Coherent: the catalog default is on, so every preset inherits it; no preset names the option. |
| (h) | castbench full (45 binaries, 4,815 functions shared with IDA) | 35,588 -> 34,808 casts (-780), 188.2 -> 184.0 per kloc, 29.9 -> 29.3 per 100 statements, 0.941x -> 0.920x IDA. By level: O0 0.952 -> 0.932, O2 0.976 -> 0.958, O2-noinline 0.891 -> 0.867. 393 functions fewer (810 casts), 25 more (+30). The 25, read one by one: 15 keep a callee's pointer in an integer lvalue (`*a0 = (int8)sub_60b5b(0x58);` beside `void * sub_60b5b(uint8 a0)`, `a0->field_0x48 = (long)sub_1563d(...)`; option off assigns a pointer to an integer with no conversion), 3 subtract two pointers as integers (`(long)sub_10369(a0) - (long)a0`), 3 take a callee's pointer as their own return type (find -O2 `ctime_format` then prints its static buffer as `(unsigned char *)0x38700`; sdiff `interact` returns `edit`'s recovered `FILE *`; tar `write_long_name` keeps one refused call's `(void *)`), 1 re-signs a parameter's pointee from a callee's `unsigned char *` (diff -O2-noinline `sub_b040`), 1 keeps a callee's `unsigned int` in a variable the caller reads as `int` (tar -O2 `sub_30000`, `v = (int4)v`), 1 renumbers variables (tar -O2 `sub_27340`), and 1 keeps a vote a later inference pass refused (tar -O0 `sub_2ba80`: `v1 = (int8 *)sub_2b9f2(a0);` and `(uint8)v1 >> 8` for a `CONCAT71`; the audit leaves that case alone, see (e)). No new line assigns an integer to a pointer variable (984 on main, 959 on the branch). |

All criteria pass; the option ships default on.

## After review (2026-09-25)

Review found a wrong-output class that none of the corpora contain: a caller
that zero-extends a callee's result in place before returning it
(`return (unsigned short)s16(x)` in a function returning `long`) printed the
callee's `short` as its own return type, because the return trimming had
removed the `movzwl` before the vote. The statement is now refused in that
case (spec 04), and a redo the run discards restores the callee's earlier
statement. Re-measured on the same base (`850e8c692`) with the fixed build:
castbench prints byte-identical C to the reviewed head on all 45 binaries
(35,588 -> 34,808 unchanged) and option off still prints main's bytes on all
45; the typesweep rows are identical (1,615 -> 1,621, 0 lost); the corpus
sweep's option-on arm is byte-identical on all ten binaries (0 arity, void or
`variables[]` moves). `make test` 675/675 and `make test-stages` 1397/1397
PARITY OK, `make test-cli` 242/242 (the two `callrettype` probes re-hashed for
the rebuilt fixture), `make rust-test` 7,554 passed and 0 failed. Speed
(interleaved min-of-15, option off vs on): fmt -0.05%, ls +0.32%, sort +0.02%, bash -0.47% (medians +0.33%, +0.15%, +1.76%, +2.94%); `speed-review.json`.

## After the second review (2026-09-25)

The first fix took only an extension an instruction does in place. Review
showed the same wrong value through the last 32-bit write of the returned
value, which on x86-64 zero-extends too: `unsigned long keep_widened(int x) {
unsigned int r = neg32(x); return r; }` (a -O0 reload) and `keep_across`
(the result kept in `%ebx` or a stack slot across another call) printed `int`,
and a caller reading the whole register printed a sign extension the binary
never does. The trimming now reports the sign and width of whatever extension
it narrows the returned value back through, and the in-place scan is gone.
Re-measured on the same base: castbench totals unchanged (35,588 -> 34,808,
393 fewer, the same 25 more); exactly the 18 functions the previous head
turned from `unsigned int` to `int` go back, and against main no return type
turns from unsigned to signed. The typesweep rows are identical (1,615 ->
1,621, 0 lost). In the corpus sweep 13 functions go back to `unsigned int`
(1,647 -> 1,636 changed), with no new statement change and still 0 arity,
void or `variables[]` moves. `make test` 675/675 and `make test-stages`
1397/1397 PARITY OK, `make test-cli` 242/242, `make rust-test` 7,554 passed and 0
failed. Speed (interleaved min-of-15, option off vs on): fmt -1.33%, ls -1.18%,
sort -1.03% (the bash run did not finish before the pause; re-measured below).

## After the third review (2026-09-27)

The return was not the only reader outside the function. The dead-bit
trimming narrows a zero-extended call argument to its 32-bit value even when
the callee reads the whole register, so `unsigned int r = neg32(x); return
halve(r) + 1;` (`mov %eax,%edi` before `halve(unsigned long)`) printed `int
v1; ... halve(v1)`, which C sign-extends: 9223372036854775807 where the binary
returns 2147483647, at every compiler tried. A signed statement is now refused
for a result, or a value C computes from it at its width, that reaches such an
argument or a RETURN narrowed back through an extension at the other sign
(spec 04). A prototyped per-function round trip of 25 shapes at five compilers
(stores, returns and arguments, both signs, 16-bit, derived values) prints the
same values with the option off and on; before the fix 20 builds differed.
The fixture gains `pass_widened`, and the round trip fails without the
refusal.

Re-measured on `0096e984d`: castbench 35,588 -> 34,808 unchanged (393 fewer,
the same 25 more, all re-read; 10 functions in tar and cp change against the
previous head, where an `int` statement is now refused); option off prints
main's bytes on all 45 binaries; against main no return type turns from
unsigned to signed. Typesweep rows identical (1,615 -> 1,621, 0 lost). Corpus
sweep 1,636 -> 1,623 changed functions, the same 67 statement changes, 0
arity, void or `variables[]` moves. `make test` 675/675 and `make test-stages`
1400/1400 PARITY OK, `make test-cli` 246/246, `make rust-test` 7,609 passed
and 0 failed. Speed (interleaved min-of-15, option off vs on): fmt -0.81%, ls -1.28%, sort -2.49%, bash -0.26% (medians +0.20%,
-1.59%, -3.35%, +0.19%; fmt and ls re-measured at a quiet moment after a first
run at load average 55 gave +4.18% and +2.78%); `speed-review3.json`.

## Re-checked on `e38cd49b0`, `a1091b591` and `29cfedee0`

origin/main gained #732 (an ELF definition preferred over its import stub),
#737 (PE import ordinals), #741 (a PowerPC64 decode hint), #730 (Cortus APS3
constructors) and #738 (`hugefn`, `jumptablemax`) after the numbers above. On
the branch rebased onto `e38cd49b0` the typesweep rows are identical (1,615 ->
1,621, 0 lost). On the branch rebased onto `29cfedee0` the option-on castbench
output is byte-identical to the build measured above on all 45 binaries, and
option off is byte-identical to the `0096e984d` main arm on all 45, so none of
them reaches the corpus. There: `make test` 675/675 and `make test-stages`
1413/1413 PARITY OK, `make test-cli` 249/249, check-spec lenient and strict
OK, `kuna catalog --check` and `counters --check` clean, `make rust-test`
7,631 passed and 0 failed.
