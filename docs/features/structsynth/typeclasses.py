#!/usr/bin/env python3
"""Classify every type_match decision the structsynth flip moves.

    python3 docs/features/structsynth/typeclasses.py <typesweep-out>/rows.json

reading the `--dump-decisions` rows of a `scripts.decbench.typesweep --option
structsynth param` run. Decisions are paired positionally per function (the
decision list is in ground-truth order in both arms). Each changed decision is
classed by what kuna now says (a synthesized `struct_N` or something else), by
what the ground truth is, and by whether the metric matched it before and after.

The ground-truth classes follow the rule that a primitive pointer (`char *`,
`bool *`, `unsigned long *`, and a typedef of one such as `__uintmax_t *`) must
match exactly, while a pointer to an aggregate only needs to be identified as a
structure: a `struct_N *` that replaces a correct primitive pointer is a genuine
false positive; a `struct_N *` against a named aggregate is the metric's
name-comparison cost.
"""
import collections
import json
import re
import sys

PRIM = (r"(const )?(unsigned |signed )?(char|short|int|long|long long|long int|"
        r"long unsigned int|short int|short unsigned int|bool|_Bool|float|double|void|"
        r"size_t|ssize_t|u?int\d+_t|mp_limb_t|wchar_t|wint_t|off_t|uintptr_t|ptrdiff_t|"
        r"__u?intmax_t|u?intmax_t|idx_t|__off_t|__off64_t|__ino_t|__dev_t|__mode_t|__time_t|"
        r"__ssize_t|__size_t|mode_t|uid_t|gid_t|pid_t|time_t|dev_t|ino_t|__uid_t|__gid_t|"
        r"__pid_t|char32_t|__u?int\d+_t)( const)?")
PRIM_PTR = re.compile(rf"^{PRIM}\s*\*$")
PRIM_PTR_PTR = re.compile(rf"^{PRIM}\s*\*\s*\*+$")
# Pointer typedefs whose DWARF spelling carries no star.
VOID_PTR_TYPEDEFS = {"V", "sharefile_handle"}


def gt_class(gt: str) -> str:
    if PRIM_PTR.match(gt) or gt in VOID_PTR_TYPEDEFS or gt == "void":
        return "primitive pointer"
    if PRIM_PTR_PTR.match(gt):
        return "pointer to a primitive pointer"
    if gt.endswith("*"):
        return "pointer to a named aggregate"
    return "non-pointer"


def main(path: str) -> None:
    rows = json.load(open(path))
    cls = collections.Counter()
    examples = collections.defaultdict(list)
    functions = set()
    for slice_name, v in rows.items():
        base = collections.defaultdict(list)
        test = collections.defaultdict(list)
        for r in v["decisions"]["base"]:
            base[r["fn"]].append(r)
        for r in v["decisions"]["test"]:
            test[r["fn"]].append(r)
        for fn in test:
            if len(base[fn]) != len(test[fn]):
                raise SystemExit(f"{slice_name} {fn}: decision lists differ in length")
            for b, t in zip(base[fn], test[fn]):
                if b["kuna_type"] == t["kuna_type"]:
                    continue
                gt = re.sub(r"\s+", " ", (t["gt_type"] or "").strip())
                key = ("struct_N" if "struct_" in t["kuna_type"] else "other",
                       gt_class(gt),
                       f"{'match' if b['ok'] else 'miss'} -> {'match' if t['ok'] else 'miss'}")
                cls[key] += 1
                functions.add((slice_name, fn))
                examples[key].append(f"{slice_name} {fn} {t['gt_name']}: GT {gt} | "
                                     f"{b['kuna_type']} -> {t['kuna_type']}")
    print(f"{sum(cls.values())} changed decisions in {len(functions)} functions")
    for key, n in cls.most_common():
        print(f"{n:6d}  {key[0]:8s}  {key[1]:32s}  {key[2]}")
    for key in cls:
        if key[2] == "match -> miss":
            print(f"\n{key}:")
            for line in examples[key]:
                print("   ", line)


if __name__ == "__main__":
    main(sys.argv[1])
