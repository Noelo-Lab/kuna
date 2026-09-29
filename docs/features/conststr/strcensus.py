#!/usr/bin/env python3
"""Every read-only-data address kuna prints (cast constant, &dat_X, bare hex) over ALL functions, by what the bytes are."""
import collections, json, re, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench"); sys.path.insert(0, str(Path(__file__).parent))
import castbench as CB
import census as C0
arm = Path(sys.argv[1])
pat = re.compile(r"(\((?:[A-Za-z_][\w ]*?)\s*\*+\s*\))\s*(0x[0-9a-f]+)|&dat_([0-9a-f]+)\b|\b(0x[0-9a-f]{3,})\b")
out = collections.Counter(); ex = collections.defaultdict(list)
for opt, proj, b in CB.corpus("full"):
    p = arm / opt / proj / f"{b}.c"
    if not p.exists(): continue
    data, secs, syms = C0.elfinfo(opt, proj, b)
    for line in p.read_text(errors="replace").splitlines():
        if line.startswith("//"): continue
        for m in pat.finditer(line):
            if m.group(2): form, v = "cast" + m.group(1).replace(" ", ""), int(m.group(2), 16)
            elif m.group(3): form, v = "&dat", int(m.group(3), 16)
            else: form, v = "bare", int(m.group(4), 16)
            cls, sec, note = C0.classify(opt, proj, b, v)
            if not cls.startswith("ro-"): continue
            if sec in (".dynstr", ".eh_frame", ".eh_frame_hdr", ".gnu.version", ".dynsym", ".rela.dyn", ".rela.plt", ".gnu.hash", ".note.gnu.property", ".note.gnu.build-id", ".note.ABI-tag", ".interp", ".gnu.version_r"): cls += "@" + sec
            key = (form if form in ("&dat", "bare") else "cast", cls)
            out[key] += 1
            if len(ex[key]) < 6: ex[key].append(f"{opt}/{b} {hex(v)} {note[:50]} | {line.strip()[:110]}")
for k, n in out.most_common(): print(n, k)
for k in ex:
    print("==", k)
    for e in ex[k]: print("   ", e)
