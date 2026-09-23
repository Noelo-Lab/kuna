"""Render the round-G G.6 speed table from speed.json (+ the quiet re-run, if any)."""
import json, sys
main = json.load(open('/home/mahaloz/kwt/_final-g/speed.json'))
try:
    quiet = json.load(open('/home/mahaloz/kwt/_final-g/speed-quiet.json'))
except Exception:
    quiet = {}
FN = {'fmt': 151, 'ls': 404, 'sort': 343, 'bash': 2538}
LAB = {'fmt': 'coreutils fmt', 'ls': 'coreutils ls', 'sort': 'coreutils sort', 'bash': 'bash'}
rows = []
for name in ('fmt', 'ls', 'sort', 'bash'):
    r = main.get(name)
    if not r: continue
    src = quiet.get(name) or r
    tag = ' (re-run)' if name in quiet else ''
    rows.append((name, r, src, tag))
print("| binary | functions | baseline min | round E min | round F min | **round G min** | Δ G vs F (min / median) | Δ G vs baseline (min) |")
print("|---|---:|---:|---:|---:|---:|---:|---:|")
for name, r, s, tag in rows:
    f = lambda v: f"{v:,.1f} ms"
    print(f"| {LAB[name]}{tag} | {FN[name]} | {f(s['base']['min_ms'])} | {f(s['roundC']['min_ms'])} | "
          f"{f(s['roundF']['min_ms'])} | **{f(s['roundG']['min_ms'])}** | "
          f"{s['delta_min_pct_roundG_vs_roundF']:+.2f}% / {s['delta_median_pct_roundG_vs_roundF']:+.2f}% | "
          f"{s['delta_min_pct_vs_base_roundG']:+.2f}% |")
print()
for name, r, s, tag in rows:
    print(f"# {name}: first run G/F {r['delta_min_pct_roundG_vs_roundF']:+.2f} min "
          f"{r['delta_median_pct_roundG_vs_roundF']:+.2f} med, G/base {r['delta_min_pct_vs_base_roundG']:+.2f}"
          + (f" | QUIET G/F {s['delta_min_pct_roundG_vs_roundF']:+.2f} min "
             f"{s['delta_median_pct_roundG_vs_roundF']:+.2f} med, G/base {s['delta_min_pct_vs_base_roundG']:+.2f}" if tag else ""))
