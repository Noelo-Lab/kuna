"""One census over the `libctypes` `opaque` vs `glibc` A/B dumps, one regex per
metric — the numbers in `glibc.md` and `corpus-hunk-classification-glibc.txt`.

    python3 docs/features/libctypes/census.py <dir>..

Each `<dir>` holds one `<tag>.opaque.c` / `<tag>.glibc.c` pair per binary, each
the output of `kuna decompile-all <bin> --option libctypes opaque|glibc`.
`fieldnames.txt` beside this file is the set of member names the published
layouts install; it is what makes the cross-field bucket a member claim rather
than any piece write.
"""
import glob, os, re, sys
PAT = {
  "PTRSUB(":        r'PTRSUB\(',
  "field_0x":       r'field_0x[0-9a-f]+',
  "piece":          r'\b[A-Za-z_]\w*\._\d+_\d+_',
  "undefined8":     r'\bundefined8\b',
  "stack decls":    r'// stack - ',
}
NAMES = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "fieldnames.txt")).read().split()
CROSS = re.compile(r'\b[A-Za-z_]\w*\._\d+_\d+_\s*=\s*[^;]*?(?:->|\.)(' +
                   '|'.join(sorted(NAMES, key=len, reverse=True)) + r')\b')
rows, tot = [], {k: [0, 0] for k in PAT} | {"cross-field piece write": [0, 0]}
for d in sys.argv[1:]:
    for f in sorted(glob.glob(os.path.join(d, "*.glibc.c"))):
        tag = os.path.basename(f)[:-len(".glibc.c")]
        o = os.path.join(d, tag + ".opaque.c")
        if not os.path.exists(o): continue
        so, sg = open(o).read(), open(f).read()
        r = {}
        for k, p in PAT.items():
            a, b = len(re.findall(p, so)), len(re.findall(p, sg))
            tot[k][0] += a; tot[k][1] += b; r[k] = (a, b)
        a, b = len(CROSS.findall(so)), len(CROSS.findall(sg))
        tot["cross-field piece write"][0] += a; tot["cross-field piece write"][1] += b
        r["cross-field piece write"] = (a, b)
        rows.append((tag, r))
w = max(len(t) for t, _ in rows)
for tag, r in rows:
    print(f"{tag:{w}s} " + "  ".join(f"{k} {v[0]}->{v[1]}" for k, v in r.items()))
print()
for k, v in tot.items():
    print(f"{k:26s} {v[0]:6d} -> {v[1]:6d}")
print("binaries:", len(rows))
