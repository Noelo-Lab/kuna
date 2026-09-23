import json, re, subprocess, sys, os, collections
from concurrent.futures import ThreadPoolExecutor
def split_args(s):
    depth = 0; cur = ""; out = []
    for ch in s:
        if ch in "([": depth += 1
        if ch in ")]": depth -= 1
        if ch == "," and depth == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip(): out.append(cur.strip())
    return out

def call_args(code, callee):
    res = []
    for m in re.finditer(r"\b%s\(" % re.escape(callee), code):
        i = m.end(); depth = 1; j = i
        while j < len(code) and depth:
            if code[j] == "(": depth += 1
            elif code[j] == ")": depth -= 1
            j += 1
        res.append(split_args(code[i:j-1]))
    return res

def arg_type(code, a):
    a = a.strip()
    if a.startswith('"'): return "char *(lit)"
    m = re.match(r"^\(([^()]*)\)", a)
    if m: return m.group(1) + "(cast)"
    if re.match(r"^a\d+$", a):
        head = code.split("{", 1)[0]
        m = re.search(r"([\w\s\*]+?)\s*\b%s\b" % a, head)
        return (m.group(1).strip() if m else "?") + "(param)"
    if re.match(r"^v\d+$", a):
        m = re.search(r"^\s*([\w\s\*]+?)\s*\b%s\b(\[\w+\])?;" % a, code, re.M)
        return (m.group(1).strip() if m else "?") + "(local)"
    if a.startswith("&"): return "&(addr)"
    if re.match(r"^(0x)?[0-9a-f]+$", a): return "const"
    return "expr:" + a[:30]


if __name__ == "__main__":
    R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
    KUNA = sys.argv[1]
    bucket = json.load(open(sys.argv[2]))
    outdir = sys.argv[3]
    os.makedirs(outdir, exist_ok=True)
    sel = [o for o in bucket if o["solo"] or "--all" in sys.argv]
    bykey = collections.defaultdict(list)
    for o in sel:
        bykey[o["key"]].append(o)
    
    def work(key):
        proj, opt, binn = key.split("::")
        stripped = f"{R}/{opt}/{proj}/stripped/{binn}"
        unstr = f"{R}/{opt}/{proj}/compiled/{binn}"
        jf = f"{outdir}/{proj}.{opt}.{binn}.json"
        if not os.path.exists(jf):
            subprocess.run([KUNA, "decompile-all", stripped, "--json", "--max-fn-seconds", "120"], stdout=open(jf, "w"), stderr=subprocess.DEVNULL)
        d = json.load(open(jf))
        byaddr = {f["address"]: f for f in d["functions"]}
        nm = subprocess.run(["nm", unstr], capture_output=True, text=True).stdout
        sym = {}
        for line in nm.splitlines():
            p = line.split()
            if len(p) == 3 and p[1] in "tT": sym.setdefault(p[2], int(p[0], 16))
        rows = []
        for o in bykey[key]:
            a = sym.get(o["fn"])
            if a is None:
                rows.append(dict(o, err="nosym")); continue
            x = json.loads(subprocess.run([KUNA, "xrefs", stripped, "--to", hex(a), "--json"], capture_output=True, text=True).stdout or "{}")
            kinds = collections.Counter(r["kind"] for r in x.get("xrefs", []))
            callers = sorted({r["from_function"]["address"] for r in x.get("xrefs", []) if r["kind"] in ("call", "jump") and r.get("from_function")})
            f = byaddr.get(a, {})
            types = []
            for c in callers:
                cf = byaddr.get(c)
                if not cf or not cf.get("code"): types.append("nocode"); continue
                for args in call_args(cf["code"], "sub_%x" % a):
                    types.append(tuple(arg_type(cf["code"], t) for t in args))
            rows.append(dict(o, addr=hex(a), kinds=dict(kinds), ncallers=len(callers), argtypes=[list(t) if isinstance(t, tuple) else t for t in types], proto=(f.get("code") or "").split("\n", 1)[0]))
        return rows
    
    allrows = []
    with ThreadPoolExecutor(8) as ex:
        for rows in ex.map(work, list(bykey)):
            allrows += rows
    json.dump(allrows, open(f"{outdir}/census.json", "w"), indent=1)
    c = collections.Counter()
    for r in allrows:
        if r.get("err"): c["nosym"] += 1; continue
        if r["kinds"].get("data") or r["kinds"].get("read"): c["address-taken"] += 1
        elif r["ncallers"] == 0: c["no-callers"] += 1
        else: c["callers"] += 1
    print(c)
