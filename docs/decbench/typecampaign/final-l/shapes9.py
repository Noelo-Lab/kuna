"""Cast residue by shape against IDA, signedness merged, on ONE shared set (every arm ∩ IDA).

  shapes9.py LABEL=DIR ... [--top N]
castbench.py shapes, with i/u of one width merged (IDA's _QWORD and __int64 and kuna's unsigned long
and long are one cast), `unsigned char`/`_BYTE`/`signed char` merged as `b8` (plain `char` is kept),
and every arm counted on the same functions. Never writes into the decbench tree."""
import collections, json, re, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB

ARMS = [a.split("=", 1) for a in sys.argv[1:] if "=" in a and not a.startswith("--")]
top = int(next((a.split("=", 1)[1] for a in sys.argv[1:] if a.startswith("--top=")), "30"))


def merge(t):
    m = re.match(r"^(?:[iu](8|16|32|64|128)|uchar)(\**)$", t)
    if not m:
        return t
    w = m.group(1) or "8"
    return ("b8" if w == "8" else f"int{w}") + m.group(2)


cnt = {l: collections.Counter() for l, _ in ARMS}
ida = collections.Counter()
n = 0
for opt, proj, b in CB.corpus("full"):
    i = CB.funcs(CB.RES / opt / proj / "decompiled" / f"ida_{b}.c")
    ks = {l: CB.funcs(Path(d) / opt / proj / f"{b}.c") for l, d in ARMS}
    if not i or any(not v for v in ks.values()):
        continue
    common = set(i)
    for v in ks.values():
        common &= set(v)
    for a in common:
        n += 1
        for (t, s), c in i[a][3].items():
            ida[(merge(t), s)] += c
        for l in ks:
            for (t, s), c in ks[l][a][3].items():
                cnt[l][(merge(t), s)] += c
rep = {"shared_functions": n, "ida_total": sum(ida.values())}
for l in cnt:
    keys = set(cnt[l]) | set(ida)
    more = sorted(keys, key=lambda k: -(cnt[l][k] - ida[k]))
    rep[l] = {"total": sum(cnt[l].values()),
              "more": [[cnt[l][k] - ida[k], cnt[l][k], ida[k], f"({k[0]}) {k[1]}"] for k in more[:top]],
              "fewer": [[cnt[l][k] - ida[k], cnt[l][k], ida[k], f"({k[0]}) {k[1]}"] for k in more[::-1][:12]]}
print(json.dumps(rep, indent=1))
