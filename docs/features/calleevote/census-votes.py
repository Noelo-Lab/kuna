"""Reverse-vote potential over O2/O2-noinline: for each wrong/missing GT arg, do all callers (C-text) pass one committed type?"""
import json, re, subprocess, sys, os, collections
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(__file__))
from census1 import split_args, call_args, arg_type  # noqa
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
KUNA = sys.argv[1]; rows = json.load(open(sys.argv[2])); outdir = sys.argv[3]
os.makedirs(outdir, exist_ok=True)
def committed(t):
    t = re.sub(r"\((param|local|cast|lit)\)$", "", t).strip()
    if re.match(r"^struct_\d+ \*$", t): return t
    if t in ("char *", "FILE *") or re.match(r"^[A-Za-z_]\w* \*$", t) and t.split()[0] not in ("void", "long", "int", "unsigned", "undefined8", "undefined4", "undefined1", "undefined2", "short", "code", "bool", "double", "float", "int8", "uint8", "int4", "uint4", "int2", "uint2", "int1", "uint1", "undefined"):
        return t
    if t == "char **": return t
    return None
def work(key):
    proj, opt, binn = key.split("::")
    jf = f"{outdir}/{proj}.{opt}.{binn}.json"
    if not os.path.exists(jf):
        subprocess.run([KUNA, "decompile-all", f"{R}/{opt}/{proj}/stripped/{binn}", "--json", "--max-fn-seconds", "120"], stdout=open(jf, "w"), stderr=subprocess.DEVNULL)
    try: d = json.load(open(jf))
    except Exception: return []
    nm = subprocess.run(["nm", f"{R}/{opt}/{proj}/compiled/{binn}"], capture_output=True, text=True).stdout
    sym = {}
    for line in nm.splitlines():
        p = line.split()
        if len(p) == 3 and p[1] in "tT": sym.setdefault(p[2], int(p[0], 16))
    funcs = d["functions"]
    callers = collections.defaultdict(list)
    def body(f):
        c = f.get("code") or ""
        return c.split("{", 1)[1] if "{" in c else ""
    for f in funcs:
        for m in set(re.findall(r"\bsub_([0-9a-f]+)\(", body(f))):
            if int(m, 16) != f["address"]:
                callers[int(m, 16)].append(f)
    addrtaken = set()
    for f in funcs:
        for m in re.findall(r"(?<!\w)(?:&)?sub_([0-9a-f]+)(?!\w|\()", body(f)):
            addrtaken.add(int(m, 16))
    v = rows[key]
    by = collections.defaultdict(list)
    for dd in v["decisions"]["base"]: by[dd["fn"]].append(dd)
    out = []
    for fn, ds in by.items():
        a = sym.get(fn)
        if a is None: continue
        args = [dd for dd in ds if dd["gt_is_arg"]]
        for i, dd in enumerate(args):
            if dd["ok"] is not False: continue
            cs = callers.get(a, [])
            if a in addrtaken: st = "addrtaken"; types = []
            elif not cs: st = "nocallers"; types = []
            else:
                types = []
                for cf in cs:
                    for al in call_args(body(cf), "sub_%x" % a):
                        types.append(arg_type(cf["code"], al[i]) if i < len(al) else "short")
                ct = [committed(t) for t in types]
                if types and all(ct) and len(set(re.sub(r"struct_\d+", "struct_N", c) for c in ct)) == 1: st = "agree:" + re.sub(r"struct_\d+", "struct_N", ct[0])
                else: st = "no"
            out.append(dict(key=key, fn=fn, i=i, gt=dd["gt_type"], cls=dd["cls"], kt=dd["kuna_type"], st=st, types=types[:5], solo=sum(1 for x in ds if x["ok"] is not True) == 1))
    return out
keys = [k for k in rows if k.split("::")[1] != "O0"]
allr = []
with ThreadPoolExecutor(10) as ex:
    for r in ex.map(work, keys): allr += r
json.dump(allr, open(f"{outdir}/census2.json", "w"))
c = collections.Counter((r["st"] if not r["st"].startswith("agree") else r["st"]) for r in allr)
print(c.most_common(25))
