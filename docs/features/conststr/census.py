#!/usr/bin/env python3
"""conststr census: every '(T *)<const>' kuna prints on the castbench shared set, by what it addresses."""
import collections, json, re, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castcount as CC
import castbench as CB
from elftools.elf.elffile import ELFFile

RES = CB.RES
arm = Path(sys.argv[1]) if len(sys.argv) > 1 else None; which = sys.argv[2] if len(sys.argv) > 2 else "full"
ELFS = {}

def elfinfo(opt, proj, b):
    key = (opt, proj, b)
    if key in ELFS: return ELFS[key]
    p = RES / opt / proj / "stripped" / b
    data = p.read_bytes()
    e = ELFFile(open(p, "rb"))
    secs = []
    for s in e.iter_sections():
        if s["sh_flags"] & 2 and s["sh_size"]:
            secs.append((s["sh_addr"], s["sh_addr"] + s["sh_size"], s.name, bool(s["sh_flags"] & 1),
                         bool(s["sh_flags"] & 4), s["sh_type"], s["sh_offset"]))
    syms = []
    cp = RES / opt / proj / "compiled" / b
    if cp.exists():
        ce = ELFFile(open(cp, "rb"))
        st = ce.get_section_by_name(".symtab")
        if st:
            for sy in st.iter_symbols():
                if sy["st_info"]["type"] == "STT_OBJECT" and sy["st_value"]:
                    syms.append((sy["st_value"], sy["st_size"], sy.name))
    ELFS[key] = (data, secs, sorted(syms))
    return ELFS[key]

def classify(opt, proj, b, v):
    data, secs, syms = elfinfo(opt, proj, b)
    sec = next((s for s in secs if s[0] <= v < s[1]), None)
    endsec = next((s for s in secs if s[1] == v), None)
    if sec is None:
        if endsec: return "one-past-section", endsec[2], ""
        if v < 0x1000: return "nonaddr-small", "", ""
        if v >= 1 << 63 or v >= 0xffff0000: return "nonaddr-negative", "", ""
        return "nonaddr-gap", "", ""
    lo, hi, name, w, x, typ, fo = sec
    symnote = ""
    for sv, ss, sn in syms:
        if sv == v: symnote = f"start:{sn}/{ss}"; break
        if sv < v < sv + ss: symnote = f"inside:{sn}+{v-sv}/{ss}"; break
        if ss and v == sv + ss: symnote = f"onepast:{sn}/{ss}"
    if x: return "code", name, symnote
    if typ == "SHT_NOBITS": return "bss", name, symnote
    off = fo + (v - lo)
    if w: return "writable", name, symnote
    # read-only data
    bs = data[off: fo + (hi - lo)]
    nul = bs.find(b"\0")
    s = bs[:nul] if nul >= 0 else bs
    prev = data[off - 1] if v > lo else 0
    printable = all(c in (9, 10, 13) or 0x20 <= c < 0x7f for c in s)
    if nul == 0: kind = "ro-emptystr"
    elif nul < 0: kind = "ro-unterminated"
    elif printable: kind = "ro-str"
    else: kind = "ro-str-nonprint"
    if prev not in (0,) and nul != 0: kind += "+suffix"
    return kind, name, symnote + f" bytes={s[:24]!r}"

HEXNAME = re.compile(r"\b([A-Za-z]+)_([0-9A-F]+)\b")

def main():
  rows = []
  for opt, proj, b in CB.corpus(which):
      kp = arm / opt / proj / f"{b}.c"; ip = RES / opt / proj / "decompiled" / f"ida_{b}.c"
      if not kp.exists() or not ip.exists(): continue
      ksrc = kp.read_text(errors="replace"); isrc = ip.read_text(errors="replace")
      kf = {a: t for _, a, t in CC.split_functions(ksrc)}
      idf = {a: t for _, a, t in CC.split_functions(isrc)}
      vocab = CC.harvest_types(CC.tokenize(ksrc), ksrc)
      for a in sorted(set(kf) & set(idf)):
          toks = CC.tokenize(kf[a])
          for ty, shape, line, txt in CC.find_casts(toks, vocab):
              if shape != "<const>" or not ty.endswith("*"): continue
              # locate the constant token after this cast text on the line
              L = kf[a].splitlines()[line - 1] if line - 1 < len(kf[a].splitlines()) else ""
              m = re.search(re.escape(txt) + r"(0x[0-9a-fA-F]+|\d+)", re.sub(r"\s+", "", L))
              if not m: continue
              v = int(m.group(1), 0)
              cls, sec, note = classify(opt, proj, b, v)
              hexs = f"{v:X}"
              idan = sorted({f"{p}_{h}" for p, h in HEXNAME.findall(idf[a]) if h == hexs})
              rows.append(dict(opt=opt, proj=proj, bin=b, fn=hex(a), ty=ty, v=hex(v), cls=cls, sec=sec,
                               note=note, ida=idan, line=L.strip()[:160]))
  json.dump(rows, open(Path(sys.argv[3] if len(sys.argv) > 3 else "rows.json"), "w"), indent=0)
  c = collections.Counter(r["cls"] for r in rows)
  print(f"{len(rows)} pointer-typed constant casts on the shared set")
  for k, n in c.most_common(): print(f"  {n:5} {k}")


if __name__ == "__main__":
    main()
