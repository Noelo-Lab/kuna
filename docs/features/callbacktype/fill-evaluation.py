import json, pathlib, re

contended = json.load(open('.scratch/speed.json'))
quiet = json.load(open('.scratch/speed-quiet.json'))
quiet.update(json.load(open('.scratch/speed-bash-quiet.json')))
order = ["fmt-O2", "ls-O2", "sort-O2", "bash-O2"]

rows = ["| binary | off (min ms) | on (min ms) | delta min | delta median | contended min-of-15 |",
        "|---|---|---|---|---|---|"]
worst = max(quiet[k]["delta_min_pct"] for k in order)
worst_c = max(contended[k]["delta_min_pct"] for k in order)
for k in order:
    q, c = quiet[k], contended[k]
    rows.append("| `%s` | %.1f | %.1f | %+.2f%% | %+.2f%% | %+.2f%% |" % (
        k, q["off"]["min_ms"], q["on"]["min_ms"],
        q["delta_min_pct"], q["delta_median_pct"], c["delta_min_pct"]))
rows += ["",
 "Same build, both arms, `kuna decompile-all --json --max-fn-seconds 120`, the arms alternated",
 "per iteration and the minimum taken. The first four columns are min-of-11 at a quiet moment;",
 "the last is the first pass, min-of-15, taken while other campaign lanes were running.",
 "**Worst delta %+.2f%% quiet, %+.2f%% contended — inside the +5%% budget either way.**" % (worst, worst_c),
 "`ls` parks nothing at all (its two arms are byte-identical), so its column is the harness's",
 "own noise floor: +2.60%% contended against -0.12%% quiet on the same pair of binaries.",
 "Driver: `docs/features/callbacktype/speed.py`; raw samples in `record.json`."]

doc = pathlib.Path('docs/features/callbacktype/default-on-evaluation.md')
t = doc.read_text()
start = t.index("## (e) speed")
end = t.index("## (f) whole-corpus")
t = t[:start] + "## (e) speed — interleaved, min-of-N\n\n" + "\n".join(rows) + "\n\n" + t[end:]
doc.write_text(t)

rec = pathlib.Path('docs/features/callbacktype/record.json')
d = json.loads(rec.read_text())
d["speed"] = {
    "method": ("interleaved whole-binary `kuna decompile-all --json --max-fn-seconds 120`, "
               "`--option callbacktype off` against the new default, same build "
               "(docs/features/callbacktype/speed.py); min-of-11 at a quiet moment, with the "
               "first contended min-of-15 pass kept beside it"),
    "binaries": "O2 coreutils fmt/ls/sort + bash",
    "results": {k: {"off_min_ms": quiet[k]["off"]["min_ms"],
                    "on_min_ms": quiet[k]["on"]["min_ms"],
                    "delta_min_pct": quiet[k]["delta_min_pct"],
                    "delta_median_pct": quiet[k]["delta_median_pct"],
                    "contended_delta_min_pct": contended[k]["delta_min_pct"],
                    "samples_off": quiet[k]["off"]["samples"],
                    "samples_on": quiet[k]["on"]["samples"]} for k in order},
    "worst_delta_min_pct": worst,
    "worst_delta_min_pct_contended": worst_c,
    "speed_budget_pct": 5.0,
    "speed_within_budget": worst <= 5.0 and worst_c <= 5.0,
    "note": ("`ls` parks nothing and its two arms are byte-identical, so its delta is the "
             "harness's noise floor: +2.60% contended, -0.12% quiet."),
}
rec.write_text(json.dumps(d, indent=1) + "\n")
print("quiet worst", worst, "contended worst", worst_c)
