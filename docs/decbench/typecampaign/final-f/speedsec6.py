"""Render the round-F F.6 speed table from speed.json (+ the quiet re-run, if any)."""
import json, sys
main = json.load(open('/home/mahaloz/kwt/_final-f/speed.json'))
try:
    quiet = json.load(open('/home/mahaloz/kwt/_final-f/speed-quiet.json'))
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
print("| binary | functions | baseline min | round C min | round E min | **round F min** | Δ F vs E (min / median) | Δ F vs baseline (min) |")
print("|---|---:|---:|---:|---:|---:|---:|---:|")
for name, r, s, tag in rows:
    f = lambda v: f"{v:,.1f} ms"
    print(f"| {LAB[name]}{tag} | {FN[name]} | {f(s['base']['min_ms'])} | {f(s['roundC']['min_ms'])} | "
          f"{f(s['roundE']['min_ms'])} | **{f(s['roundF']['min_ms'])}** | "
          f"{s['delta_min_pct_roundF_vs_roundE']:+.2f}% / {s['delta_median_pct_roundF_vs_roundE']:+.2f}% | "
          f"{s['delta_min_pct_vs_base_roundF']:+.2f}% |")
print()
for name, r, s, tag in rows:
    print(f"# {name}: first run F/E {r['delta_min_pct_roundF_vs_roundE']:+.2f} min "
          f"{r['delta_median_pct_roundF_vs_roundE']:+.2f} med, F/base {r['delta_min_pct_vs_base_roundF']:+.2f}"
          + (f" | QUIET F/E {s['delta_min_pct_roundF_vs_roundE']:+.2f} min "
             f"{s['delta_median_pct_roundF_vs_roundE']:+.2f} med, F/base {s['delta_min_pct_vs_base_roundF']:+.2f}" if tag else ""))
