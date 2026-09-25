# callrettype: default-on evaluation

Measured on the branch rebased onto `850e8c692` (origin/main after #720
structheadless, #727 castindex, #728 castternary and #726 callpush). The
castbench base arm is a fresh build of `850e8c692` itself, and `--option callrettype off` on the
branch build prints the same bytes as it on all 45 castbench binaries. Every
other criterion compares `--option callrettype off` with the default on the
same build. Speed was measured on the build before the rebase (base
`c960fb18d`, all six binaries) and again on the `f434b6a60` base (fmt, ls,
sort, bash); the option's cost is its own second decompile.

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675 PARITY OK. No assertion moved; `docs/baseline.json` untouched. A single-function decompile states nothing. |
| (b) | `make test-stages` | 1397/1397 PARITY OK. The only new keys are the three `callrettype #N` assertions of `kuna-callrettype.xml` (a negative control: pass 1 pins `off`); `docs/baseline-stages.json` re-recorded for them, nothing else moved. |
| (c) | `make test-cli` | 242/242. One probe moved, with its reason: `protoorder-types-keeps-a-float-in-a-gpr-an-integer` pinned `return (int)(float)h(a0,v1) + v1 + 3;`; `h` is declared `float h(int a0,float a1)` in the same listing, so the `(float)` converted from a type the call does not have, and the probe now pins `return (int)h(a0,v1) + v1 + 3;`. The same line moved in `a_float_in_a_general_register_keeps_its_integer_uses_round_trip`, whose compiled round trip (the printed callers against a bit-preserving `h`) still prints what the source prints. The two `callrettype-*` probes follow the rebuilt fixture's hash. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | option off 1,615 perfect, mean .3697; default 1,621, mean .3701. 6 moved onto perfect, 0 off; 17 improved, 2 worse; `decomp_vars` identical (96,889 both arms), so no variable row is fabricated or lost. The two worse rows: who -O2 `who` (a reused stack slot now typed `char *`, the value `time_string` stores there; the DWARF variable at that offset is another one) and pwd -O2-noinline `file_name_prepend` (a `calleevote` vote through the `dirent` shell). `typesweep-report.md`, `typesweep-moved.csv`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json`, option off vs on on the final build | fmt +1.44% min / -0.25% median, ls -0.96% min / -0.04% median, sort +2.69% min / +0.03% median, kmod_O2ni +1.34% min / +3.25% median, dpkgdivert_O2 +0.84% min / -0.19% median, bash -0.20% min / +2.17% median. Worst min +2.69% (within +5%). The audit's second decompile is the whole cost the option adds: the first audit (every statement, and votes a later pass refused) cost fmt -O2 +76% and bash -O2 +55% (218 second decompiles, 51.6 s); auditing only statements the variable carries, only the tie contradictions, and only functions up to 1,000 p-code ops brings bash to 26 second decompiles. Re-measured after the rebase onto `f434b6a60` (min of 15): fmt -0.23%, ls -1.52%, sort +1.63%, bash +2.25%. `speed-final.json`, `speed-rebased.json`, `speed.py`. |
| (f) | whole-corpus `decompile-all` before/after, every hunk classified | Ten binaries outside the castbench corpus (bash, dash, kmod, crontab, dpkg-divert, minigzip, xmlwf, e2fsck, sftp, ip; 8,878 functions; numbers from the build after the second review): 1,636 changed; 1,522 differ only in types, conversions and names; 47 only in how a type spells the same expression (a literal's sign or suffix, `&dat_X` for an address, `v[2]` for `*(v + 0x10)`); 67 read by hand (every one in dash, kmod, crontab, dpkg-divert, e2fsck and ip, the largest in bash): the retyped variable's accesses spelled as members or elements, character literals, a ternary printed as if/else, variable renumbering. No call changed its argument count, no function's return became or stopped being `void`, and no `variables[]` row appeared or disappeared. `corpus-hunks.json`, `corpus.py`, `hunkclass.py`. |
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
1397/1397 PARITY OK, `make test-cli` 242/242, RUSTTEST27. Speed (interleaved
min-of-15, option off vs on): SPEED27; `speed-review2.json`.
