"""Whole-binary decompile-all, castternary off vs on (default); per-function hunks."""
import concurrent.futures as cf, json, os, re, subprocess, sys
from pathlib import Path
K = sys.argv[1]; out = Path(sys.argv[2]); lst = sys.argv[3]; W = int(sys.argv[4])
bins = [l.strip() for l in open(lst) if l.strip()]
FH = re.compile(r"^// Function: (\S+) @ (0x[0-9a-f]+)", re.M)

def split(txt):
    hs = list(FH.finditer(txt)); d = {}
    for i, m in enumerate(hs):
        e = hs[i + 1].start() if i + 1 < len(hs) else len(txt)
        d[m.group(2)] = (m.group(1), txt[m.start():e])
    return d

def one(b):
    tag = b.replace("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/", "").replace("/stripped/", "/").replace("/", "_")
    res = {}
    for arm, extra in (("off", ["--option", "castternary", "off"]), ("on", [])):
        p = subprocess.run([K, "decompile-all", b, "--max-fn-seconds", "120"] + extra, capture_output=True, text=True, timeout=7200)
        if p.returncode != 0:
            return tag, {"error": f"{arm} rc={p.returncode}"}
        res[arm] = p.stdout
        if arm == "off" and " ? " not in p.stdout:
            return tag, {"functions": len(split(p.stdout)), "changed": 0, "hunks": [], "no_conditional": True}
    a, c = split(res["off"]), split(res["on"])
    hunks = []
    for addr in sorted(set(a) | set(c)):
        x, y = a.get(addr, ("", ""))[1], c.get(addr, ("", ""))[1]
        if x != y:
            xl, yl = x.splitlines(), y.splitlines()
            rm = [l for l in xl if l not in yl]; ad = [l for l in yl if l not in xl]
            hunks.append({"addr": addr, "name": (a.get(addr) or c.get(addr))[0], "off": rm, "on": ad,
                          "nlines_off": len(xl), "nlines_on": len(yl)})
    return tag, {"functions": len(c), "changed": len(hunks), "hunks": hunks}

done = json.load(open(out / "hunks.json")) if (out / "hunks.json").exists() else {}
todo = [b for b in bins if b.replace("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/", "").replace("/stripped/", "/").replace("/", "_") not in done]
with cf.ThreadPoolExecutor(W) as ex:
    for tag, r in ex.map(one, todo):
        done[tag] = r
        print(tag, r.get("changed", r.get("error")), flush=True)
        json.dump(done, open(out / "hunks.json", "w"), indent=0)
print("CORPUS_DONE", flush=True)
