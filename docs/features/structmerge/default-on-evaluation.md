# structmerge -- the default-on evaluation

The option ships `off`. This is what was measured for the flip, and which
criterion held it.

## The criterion that held it

The campaign's standing rule for a default change is "no large speed hit and
accuracy the same or better". On `type_match` accuracy is exactly the same: the
444-slice sweep is 1,522 perfect in both arms, 0 improved and 0 worsened, because
the metric cannot see a record's members at all -- a `struct_2 *` and a
`struct_3 *` score alike.

So the accuracy question is the goal-3 instrument's, and it answers two ways:

* on the eight layout builds the instrument is published on, per-parameter
  claimed-field precision RISES, 0.8710 -> 0.8737, with recall 0.0865 -> 0.0894
  and F1 0.1574 -> 0.1622;
* over the 177 builds of `dedup_heldout.py`'s eight sets, F1 rises on every set
  and true fields rise by 813, but pooled claimed-field precision FALLS, 0.9330
  -> 0.9312, and falls on five of the eight sets (the largest, HELDOUT2, by 0.62
  points). 916 of the 1,105 fields the merge adds are real DWARF fields; the
  other 189 are not.

A claimed field that is not there is the error this lane exists to avoid, so
"the same or better" is not met on the broadest precision measurement available,
and the default stays `off`. Everything the option does is still one flag away,
and the evidence for flipping it is in `analysis.md` for whoever wants to.

## The criteria that pass

| criterion | result |
|---|---|
| (d) 444-slice typesweep, new default vs old | perfect 1,522 -> 1,522; 0 improved, 0 worsened; aggregate identical |
| (e) interleaved min-of-15 on `fmt`, `ls`, `sort` and `bash` at -O2 | +0.91%, -4.08%, -0.42%, +0.32%; worst +0.91% against a +5% budget |
| (f) whole-corpus `decompile-all` before/after, ten binaries | 92 of 4,904 functions change; 91 are the record rename the option is for, 1 is a field respelling, 0 skeleton deltas, 0 declaration-count deltas, nothing unclassified (`hunks.md`) |
| (g) `p0_knowledge/modes.rs` | nothing to do: the preset-membership test covers `on|off` options with a shipped default of `off`, and `structmerge` is `off|siblings`, exactly as `structsynth` is |
| record identity | 567 absorptions over 177 builds; 31 confirmed on the same DWARF record, **0 on a different one** |

(a) `make test`, (b) `make test-stages` and (c) `make test-cli` were not re-run
with the default flipped, because the deciding criterion above already fails. The
stage test is written for the flip either way: its first pass names `option
structmerge off` explicitly, so it keeps testing the off arm whichever way the
default goes.

## What would change the answer

The precision trade is the growth bound, not the agreement rule. A reader is
declared to hold at most twice the fields it measured, and most readers claim two
or three, so the union is usually a third or fourth field added to a two-field
record -- which is where a wrong one costs the most proportionally. A rule that
raised the bar for adding a field to a small record (more shared claims, or a
second reader agreeing) would cost recall and buy precision, and is the next
thing to measure on this option.
