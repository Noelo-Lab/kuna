"""Classify every changed hunk between two castbench arms: the rewrite may only drop casts, rewrite
`*(T *)((long)p + i * S)` into a subscript and `(long)p - (long)q` into `p - q`. Invariants per hunk:
the identifier multiset (variables, globals, callees) is unchanged; constants may only lose a scale
equal to an element size (1,2,4,8); the cast count drops."""
import difflib, sys, collections, re, json
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB, castcount as CC
from pathlib import Path
base, arm = Path(sys.argv[1]), Path(sys.argv[2])
TYPEWORDS = set(CC.BASE) | set(CC.QUAL) | set(CC.TAGKW) | set(CC.SEED)
cls = collections.Counter(); odd = []; per_bin = collections.Counter()
for opt, proj, b in CB.corpus("full"):
    try:
        bs = (base/opt/proj/f"{b}.c").read_text(errors="replace"); ks = (arm/opt/proj/f"{b}.c").read_text(errors="replace")
    except OSError: continue
    voc = CC.harvest_types(CC.tokenize(bs), bs)
    bf = {a: t for _, a, t in CC.split_functions(bs)}; kf = {a: t for _, a, t in CC.split_functions(ks)}
    for a in bf:
        if a not in kf or bf[a] == kf[a]: continue
        bl, kl = bf[a].splitlines(), kf[a].splitlines()
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, bl, kl, autojunk=False).get_opcodes():
            if tag == "equal": continue
            old = "\n".join(bl[i1:i2]); new = "\n".join(kl[j1:j2])
            to, tn = CC.tokenize(old), CC.tokenize(new)
            ido = collections.Counter(t.text for t in to if t.kind == "id" and t.text not in TYPEWORDS and t.text not in voc)
            idn = collections.Counter(t.text for t in tn if t.kind == "id" and t.text not in TYPEWORDS and t.text not in voc)
            no = collections.Counter(t.text for t in to if t.kind == "num"); nn = collections.Counter(t.text for t in tn if t.kind == "num")
            lost = no - nn; gained = nn - no
            co = len(CC.find_casts(to, voc)); cn = len(CC.find_casts(tn, voc))
            diff_shape = bool(re.search(r"\((long|int8)\)[&\w\[\]\-\>\.]+(\([^()]*\))? - \((long|int8)\)", old))
            sub_shape = new.count("[") > old.count("[")
            ok = ido == idn and not gained and all(int(x, 0) in (1, 2, 4, 8) for x in lost.elements()) and cn < co
            kind = ("diff+sub" if diff_shape and sub_shape else "diff" if diff_shape else "subscript" if sub_shape else "other")
            per_bin[(opt, b)] += 1
            if ok and kind != "other":
                cls[kind] += 1
            else:
                cls["UNCLASSIFIED"] += 1
                odd.append((f"{opt}/{b}@{hex(a)}", kind, dict(ido - idn), dict(idn - ido), dict(lost), dict(gained), co, cn, old[:220], new[:220]))
print(dict(cls), " binaries with hunks:", len(per_bin))
for o in odd[:30]:
    print(json.dumps(o))
