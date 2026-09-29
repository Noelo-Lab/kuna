import json, collections, re, sys
from pathlib import Path
arm = Path(sys.argv[1]); rows = json.load(open(sys.argv[2]))
TR = re.compile(r"^\[globalref\] (\S+) 0x([0-9a-f]+) (.*) (\S+)$")
cache = {}
def verdicts(opt, proj, b):
    k = (opt, proj, b)
    if k in cache: return cache[k]
    names = {}
    for line in (arm/opt/proj/f"{b}.c").read_text(errors="replace").splitlines():
        m = re.match(r"^// Function: (\S+) @ (0x[0-9a-f]+)", line)
        if m: names[m.group(1)] = int(m.group(2), 16)
    v = collections.defaultdict(list)
    for line in (arm/opt/proj/f"{b}.trace").read_text().splitlines():
        m = TR.match(line)
        if m and m.group(1) in names:
            v[(names[m.group(1)], int(m.group(2), 16))].append(m.group(4))
    cache[k] = v; return v
for r in rows:
    vv = verdicts(r['opt'], r['proj'], r['bin']).get((int(r['fn'],16), int(r['v'],16)), [])
    # last decision of the function's final print is the one that counts; record the set
    r['why'] = "/".join(sorted(set(x for x in vv if x != 'named'))) or ("named?" if vv else "no-trace")
json.dump(rows, open(sys.argv[3], "w"), indent=0)
c = collections.Counter((r['cls'], r['why']) for r in rows)
for k, n in c.most_common(): print(f"{n:5} {k}")
