#!/usr/bin/env python3
"""Which scored variables does the element-pair rule move, by ground-truth type?

    python3 docs/features/structsynth/arrays_credit.py <before>/rows.json <after>/rows.json

Both inputs are `scripts.decbench.typesweep --dump-decisions` outputs for
`--option structsynth param`, one from a build without the rule (`before`) and
one with it (`after`). Every `test`-arm decision whose type goes from a
synthesized `struct_N *` to anything else is counted, grouped by the DWARF type
decbench scored it against and by whether `type_match` accepts it now.

`type_match` compares pointee names, so a `struct_N *` is never accepted today.
If decbench credited any `struct_N *` against a DWARF pointer to a structure,
every row below whose ground truth is such a pointer would be a match lost.
"""
import json
import re
import sys
from collections import Counter

SYN = re.compile(r"^struct_\d+ \*$")
before, after = (json.load(open(p)) for p in sys.argv[1:3])
rows = Counter()
for sl in before:
    for x, y in zip(before[sl]["decisions"]["test"], after[sl]["decisions"]["test"]):
        if x["kuna_type"] and SYN.match(x["kuna_type"]) and not (
                y["kuna_type"] and SYN.match(y["kuna_type"])):
            rows[(x["gt_type"], y["kuna_type"], bool(y["ok"]))] += 1
print(f"{sum(rows.values())} scored variables leave a struct_N *")
for (gt, kuna, ok), n in rows.most_common():
    print(f"  {n:4d}  gt {gt:20s} -> {kuna:18s} {'MATCH' if ok else ''}")
