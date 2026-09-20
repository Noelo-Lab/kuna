# `decompile-project` took its own order, so a `struct_N` meant two records

## What was reported

Round C of the type campaign measured per-parameter claimed-field precision
against DWARF falling from 0.8945 to 0.5520 (filler counted, 0.7816 to 0.4710)
on the eight goal-3 builds, and named `protoorder` (#669) as the cause: "the
callee's recovered pointer type wins the vote at the call site even where the
caller measured the record itself"
(`docs/decbench/typecampaign/results.md` §C.4).

## That is not what happens

With `KUNA_SS_DEBUG` instrumentation on `kuna_structsynth::accepts` and the
install site, du -O2 gives the same answer in both arms:

* 36 structures are installed in each arm, over the same bases;
* 32 of the 36 field lists are byte-identical between the arms, and the other 4
  differ only in the *spelling of a field's type* (`xunknown8` vs a pointer),
  never in an offset or a width;
* `hash_rehash`'s parameter, the witness §C.4 names, measures
  `{0,8,16,24,(32),40,48,56,64,72}` in both arms and installs exactly that.

The caller's own measurement already wins: `structsynth` type-locks the
parameter after the vote, so a vote cannot replace a layout the caller measured.

## What does happen

The instrument reads parameter types from `kuna decompile-all --json` and the
layouts those names refer to from the `.h` of `kuna decompile-project`. Both
surfaces keep a `structsynth` ledger, and the ledger numbers `struct_N` in the
order the program is visited in. `decompile-all` decompiles callees first
(`protoorder`, on by default); `decompile-project` kept its own address-order
schedule and declined the option with a warning. So the two numbered the same
records differently:

```
du -O2, names used by decompile-all, resolved against the exported header
  --option protoorder off : 29 of 30 agree
  default                 :  1 of 30 agree
```

`struct_9` in the default export is an 11-member record with a `long` at 0x18
and `unsigned int`s at 0x2c and 0x48; `struct_9` in `decompile-all` is the
ten-member `hash_table` layout. Reading one against the other is what the
0.5520 measures.

It is a defect in its own right, not only in the instrument: anyone who exports
a project and then works from `decompile-all --json` (or the reverse) resolves a
`struct_N *` against the wrong declaration, and nothing on either surface says
so.

## The fix

`decompile-project` asks the same question `decompile-all` asks
(`callee_first_decision`) and runs the same loop with the export's own
per-function options (`export_options`), so the two visit the program in one
order. `--option protoorder off` gives both the old schedule. A streamed export
writes each body as it finishes and cannot buffer a plan, so it keeps the
address-order schedule and still says the option does not reach it;
`decompile-graph` decompiles one function per row and is unchanged.

## What it measures

`docs/features/structsynth/layoutscore.py`, eight builds, both arms
(`layoutscore.log`):

| arm | fields only | filler counted | F1 (fields / filler) |
|---|---|---|---|
| round B (before protoorder) | 0.8945 | 0.7816 (798/1021) | 0.1376 / 0.1525 |
| round C (the join) | 0.5520 | 0.4710 (609/1293) | 0.1021 / 0.1135 |
| **after this fix** | **0.8709** (877/1007) | **0.7836** (1010/1289) | **0.1678 / 0.1882** |
| `--option protoorder off`, same binary | 0.9001 (748/831) | 0.7993 (864/1081) | 0.1456 / 0.1642 |

`final-c/layoutdiff.py` on du -O2: parameters that claim fewer true fields than
the `protoorder off` arm, **23 -> 0**.

The fields-only precision sits 0.024 under round B's, and `layoutwhere.py`
(ls -O0) says where: the shared parameters are identical in the two arms
(117/128 = 0.9141 each, none worse), and the default types **14 more**
parameters as struct pointers — the ones the caller never dereferences, where
the callee's record supplies the kind. Those 14 claim 131 fields of which 92 are
true (0.7023). They claim nothing at all with the option off, so every true
field among them is new: recall 0.0562 -> 0.0929 and F1 0.1021 -> 0.1678, both
above round B.

`structscore --all` on du -O2: TRex mean 1.9159 (round C 1.916, unchanged --
that score reads `decompile-all` only), layout F1 0.2094 (round C 0.097, round B
0.197).
