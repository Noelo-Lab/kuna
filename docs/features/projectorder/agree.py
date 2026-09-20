"""Does the export render a function the way `decompile-all` renders it?"""
import re, sys
from pathlib import Path
ROOT = Path("/home/mahaloz/kwt/layoutvote/.scratch/export")
BUILDS = [(o, b) for o in ("O0", "O2") for b in ("fmt", "ls", "sort", "du")]

def functions(text):
    out, cur, buf = {}, None, []
    for line in text.split("\n"):
        m = re.match(r"^(?:[\w ]+[ \*] *)?(\w+)\(.*\)(?: //.*)?$", line)
        if m and not line.startswith(" ") and not line.endswith(";"):
            if cur: out[cur] = "\n".join(buf).rstrip()
            cur, buf = m.group(1), [line]
            continue
        if cur: buf.append(line)
    if cur: out[cur] = "\n".join(buf).rstrip()
    return out

tot = {"base": 0, "fix": 0, "n": 0}
print("| build | functions in both | export == decompile-all (before) | (after) |")
print("|---|---:|---:|---:|")
for opt, b in BUILDS:
    allf = functions((ROOT / f"all-{opt}-{b}.txt").read_text())
    row = []
    common = None
    for arm in ("base", "fix"):
        ex = functions((ROOT / f"{arm}-{opt}-{b}" / f"{b}.c").read_text())
        keys = set(ex) & set(allf)
        common = keys if common is None else common & keys
        row.append((keys, ex))
    counts = []
    for keys, ex in row:
        counts.append(sum(1 for k in common if ex[k] == allf[k]))
    print(f"| {opt} {b} | {len(common)} | {counts[0]} | {counts[1]} |")
    tot["n"] += len(common); tot["base"] += counts[0]; tot["fix"] += counts[1]
print(f"\npooled: {tot['n']} functions; export == decompile-all before {tot['base']} "
      f"({tot['base']/tot['n']:.1%}), after {tot['fix']} ({tot['fix']/tot['n']:.1%})")
