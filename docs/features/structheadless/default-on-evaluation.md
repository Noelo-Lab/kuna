# structheadless -- the default-on evaluation

The option ships `off`. Everything below was measured on build ce4e278b2 (main
b3878d32e, with castimplied, castarith and castsign on, plus this branch), both
arms of the same build. The first evaluation (build e2071591f on main dbe854ba3)
reached the same verdict; a round-I rebase onto #719 then exposed a new false
positive -- a caller-voted `char **argv` retyped as a record, which lost coreutils
`tail`'s `parse_obsolete_option` at -O0 (1.0 -> .923) -- and this build fixes it.

## The criteria that hold it

| criterion | result |
|---|---|
| layout precision (the gate: fields-only >= .8713 on the published instrument) | **fails**: .8713 -> .8298 (880/1010 -> 1068/1287, eight builds). Of the 89 top-level misses the option adds, 69 are members of a named embedded record, an array of records or a union (`layoutdepth.py`: .9594 -> .9526 at depth), 6 padding, 5 a member read at another width, 4 inside a member, 1 an array element and 4 past gnulib `FTSENT`'s end. Removing all 20 that are not members at depth would give .843: the gate cannot pass without declining records that are right |
| (d) 444-slice typesweep, improved >= worse | **fails by one**: 1,615 -> 1,615 perfect, 0 perfect functions lost, 1 improved (grep `bmexec_trans`), 2 worse (bzip2 `BZ2_bzReadClose` at -O0 and -O2-noinline: `BZFILE *` is `typedef void`; the function reads its own `bzFile` members, no caller states a type, and nothing in a stripped binary tells an exported opaque handle from a record parameter) |

## The other criteria

| criterion | result |
|---|---|
| (a) `make test` with the rule on | inert by construction: the console harness never marks a function closed, so no datatest can reach the rule (675/675 either way) |
| (b) `make test-stages` | inert for the same reason; the stage test pins both passes (1364/1364 with the option off) |
| (c) `make test-cli` | 240/240 with the option off; three probes pin both values, one of them the `argv` shape the rebase exposed |
| (e) speed, interleaved min-of-15, `decompile-all --json`, -O2 | see `record.json` `speed` (fmt, ls, sort, bash) |
| (f) whole-corpus hunks, 45 binaries | 3,447 of 20,230 functions change; 0 skeleton deltas; every other hunk read and classified (`hunks.md`) |
| (g) `p0_knowledge/modes.rs` | nothing to do: the option is `off|closed`, not in any preset, as `structmerge` is |
| (h) castbench full, both directions | 38,602 -> 37,049 casts (1.021 -> 0.980 times IDA); 219 functions fewer (-1,584), 22 more (+31), every one read (`analysis.md`) |

## The false-positive classes the review named

| class | now |
|---|---|
| a record replacing a correct primitive pointer | none left among parameters DWARF describes: 100 of the 102 parameters moved from a primitive pointer to a `struct_N *` are struct pointers; tar's `usage_argful_short_opt` `void *cookie` (an `argp_fmtstream` behind a generic callback parameter) and bzip2's `BZFILE *` are the `void *` handles. The `char **` class is gone: a headless record gives way to a `char *`, `char **` or declared record every caller passes, and never retypes a value a declared call gives a pointee |
| one object named by several `struct_N` | priced, not fixed: a headless record is its reader's partial view (tar's `tar_stat_info` goes from one name over nine functions to five). `structmerge siblings`, off by default, is the rule that unions agreeing readers |
| pointer -> integer demotions | 8 declarations by name, of which one lowers a function's pointer count (find -O2-noinline `perform_arm_swap`, a moved `pred_right`, cast-neutral); tar's `wordsplit_varexp`/`wordsplit_cmdexp` list pointers are fixed by refusing a headless record that types a dereferenced member as a word |

## What flipping it would buy and cost

On top of castarith it removes 4.0% of all casts on the shared set, taking
every opt level but -O2 under IDA's count (O0 0.973, O2 1.031, O2-noinline
0.929), and under decbench#93's crediting it is +125 perfect functions. A flip
needs the precision gate restated at depth, or the member-at-depth claims to be
modelled as embedded records, and an answer for an exported API's opaque handle.
