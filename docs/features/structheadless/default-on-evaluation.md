# structheadless -- the default-on evaluation

The option ships `off`. Everything below was measured on build a54805cab (main
c960fb18d, with castimplied, castarith, castsign and globalref on, plus this
branch), both arms of the same build. The code is the code measured on main
b3878d32e (build ce4e278b2), which reached the same verdict, as did the first
evaluation (build e2071591f on main dbe854ba3); a round-I rebase onto #719 had
exposed a false positive -- a caller-voted `char **argv` retyped as a record,
which lost coreutils `tail`'s `parse_obsolete_option` at -O0 (1.0 -> .923) --
and this code fixes it.

## The criteria that hold it

| criterion | result |
|---|---|
| layout precision (the gate: fields-only >= .8713 on the published instrument) | **fails**: .8713 -> .8298 (880/1010 -> 1068/1287, eight builds). Of the 89 top-level misses the option adds, 69 are members of a named embedded record, an array of records or a union (`layoutdepth.py`: .9594 -> .9526 at depth), 6 padding, 5 a member read at another width, 4 inside a member, 1 an array element and 4 past gnulib `FTSENT`'s end. Removing all 20 that are not members at depth would give .843: the gate cannot pass without declining records that are right |
| (d) 444-slice typesweep, improved >= worse | **fails by one**: 1,615 -> 1,615 perfect, 0 perfect functions lost, 1 improved (grep `bmexec_trans`), 2 worse (bzip2 `BZ2_bzReadClose` at -O0 and -O2-noinline: `BZFILE *` is `typedef void`; the function reads its own `bzFile` members, no caller states a type, and nothing in a stripped binary tells an exported opaque handle from a record parameter) |

## The other criteria

| criterion | result |
|---|---|
| (a) `make test` with the rule on | inert by construction: the console harness never marks a function closed, so no datatest can reach the rule (675/675 either way) |
| (b) `make test-stages` | inert for the same reason; the stage test pins both passes (1372/1372 with the option off) |
| (c) `make test-cli` | 240/240 with the option off; three probes pin both values, one of them the `argv` shape the rebase exposed |
| (e) speed, interleaved min-of-15, `decompile-all --json`, -O2 | see `record.json` `speed` (fmt, ls, sort, bash) |
| (f) whole-corpus hunks, 45 binaries | 3,378 of 20,230 functions change; 0 skeleton deltas; every other hunk read and classified (`hunks.md`); no function changes its parameter count or its phantom locals and arguments |
| (g) `p0_knowledge/modes.rs` | nothing to do: the option is `off|closed`, not in any preset, as `structmerge` is |
| (h) castbench full, both directions | 37,477 -> 36,040 casts (0.991 -> 0.953 times IDA); 219 functions fewer (-1,584), 57 more (+147), every one read (`analysis.md`): 36 (+117) are globalref declining to name a constant address the function now uses as a record and as another pointer, or whose record a direct read overlaps |

## The false-positive classes the review named

| class | now |
|---|---|
| a record replacing a correct primitive pointer | none left among parameters DWARF describes: 100 of the 102 parameters moved from a primitive pointer to a `struct_N *` are struct pointers; tar's `usage_argful_short_opt` `void *cookie` (an `argp_fmtstream` behind a generic callback parameter) and bzip2's `BZFILE *` are the `void *` handles. The `char **` class is gone: a headless record gives way to a `char *`, `char **` or declared record every caller passes, and never retypes a value a declared call gives a pointee |
| one object named by several `struct_N` | priced, not fixed: a headless record is its reader's partial view (tar's `tar_stat_info` goes from one name over nine functions to five). `structmerge siblings`, off by default, is the rule that unions agreeing readers |
| pointer -> integer demotions | 8 declarations by name, of which one lowers a function's pointer count (find -O2-noinline `perform_arm_swap`, a moved `pred_right`, cast-neutral); tar's `wordsplit_varexp`/`wordsplit_cmdexp` list pointers are fixed by refusing a headless record that types a dereferenced member as a word |

## What flipping it would buy and cost

On top of castarith and globalref it removes 3.8% of all casts on the shared
set (O0 0.945, O2 1.008, O2-noinline 0.900 times IDA's count), and under
decbench#93's crediting it is +125 perfect functions. A flip needs the precision
gate restated at depth, or the member-at-depth claims to be modelled as
embedded records, and an answer for an exported API's opaque handle. The 117
casts globalref gives back would go if globalref declared the address at the
record and spelled its other uses as casts of it, or printed a direct read
inside a named record as its member; that is globalref's rule, not this one's.

## The opaque handle, decided

bzip2's `BZ2_bzReadClose (int *bzerror, BZFILE *b)` is the only worse row, at
-O0 and -O2-noinline. `BZFILE` is `typedef void`, so by the rule that a
`struct_N *` over a ground-truth `void *` is a false positive, both rows are
real misses. The record is not wrong about the object: the function casts `b`
to its own `bzFile *` first and reads `writing`, `lastErr` and `initialisedOk`
and passes `&strm`, all past the start. Nothing in a stripped, statically linked
binary separates an exported API's opaque handle from a record parameter (every
caller is a direct call, no caller states a type), so the only way to lose the
two rows is to decline the shape, and with it the 883 headless parameters of
closed functions DWARF describes as struct pointers.
They stay, as a priced false positive, stated in the catalog row's `use_when`,
and they are one of the two reasons the option ships off.
