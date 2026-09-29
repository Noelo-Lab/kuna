"""Markdown tables for results L.6 from the speed10.py JSONs (min / median / CPU-min deltas)."""
import json, sys
def load(p):
    try:
        return json.load(open(p))
    except FileNotFoundError:
        return {}
FN = json.load(open("/home/mahaloz/kwt/_final-l/fncount10.json"))
def ms(x):
    return f"{x:,.1f} ms"
def dl(r, a, b):
    v = r["delta_min_median_pct"].get(f"{a}/{b}")
    return "n/a" if v is None else f"{v[0]:+.2f}% / {v[1]:+.2f}%" + (f" (cpu {v[2]:+.2f}%)" if len(v) > 2 else "")
def canon(files):
    d = {}
    for f in files:
        d.update(load(f))
    print("| binary | functions | baseline min | round G min | round I min | **final min** | final vs G | final vs I | final vs baseline | round loads |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---|")
    for b in ("fmt", "ls", "sort", "bash"):
        if b not in d:
            continue
        r = d[b]
        lo = r.get("round_loads")
        lo = f"{min(lo)}–{max(lo)}" if lo else "?"
        print(f"| {b} | {FN[b]['L']['functions']} | {ms(r['base']['min_ms'])} | {ms(r['G']['min_ms'])} | {ms(r['I']['min_ms'])} | **{ms(r['L']['min_ms'])}** | {dl(r,'L','G')} | {dl(r,'L','I')} | {dl(r,'L','base')} | {lo} |")
def trio(files):
    d = {}
    for f in files:
        d.update(load(f))
    print("| binary | baseline min | round F min | round G min | round I min | **final min** | final vs G | final vs F | final vs baseline | round loads |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---|")
    for b in ("kmod-O2ni", "dpkg-divert-O2ni", "crontab-O2ni"):
        if b not in d:
            continue
        r = d[b]
        lo = r.get("round_loads")
        lo = f"{min(lo)}–{max(lo)}" if lo else "?"
        print(f"| {b} | {ms(r['base']['min_ms'])} | {ms(r['F']['min_ms'])} | {ms(r['G']['min_ms'])} | {ms(r['I']['min_ms'])} | **{ms(r['L']['min_ms'])}** | {dl(r,'L','G')} | {dl(r,'L','F')} | {dl(r,'L','base')} | {lo} |")
def ablate(f):
    d = load(f)
    for b, r in d.items():
        print(f"\n{b}: L {r['L']['min_ms']} base {r['base']['min_ms']} L/base {r['delta_min_median_pct'].get('L/base')} loads {r.get('round_loads')}")
        rows = [(k.split('/', 1)[1], v) for k, v in r["delta_min_median_pct"].items() if k.startswith("L/L-")]
        for k, v in sorted(rows, key=lambda x: -x[1][0]):
            print(f"  {k[2:][:70]:70s} min {v[0]:+.2f} med {v[1]:+.2f} cpu {v[2]:+.2f}")
if __name__ == "__main__":
    what = sys.argv[1]
    if what == "canon":
        canon(sys.argv[2:])
    elif what == "trio":
        trio(sys.argv[2:])
    else:
        ablate(sys.argv[2])
