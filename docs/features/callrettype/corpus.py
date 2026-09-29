"""Whole-binary decompile-all, callrettype off vs on, over binaries outside the cast corpus.

Writes <out>/<tag>.{off,on}.json and a per-function summary: code changed, and the
arity / phantom counters that type_match cannot see -- every function's printed
parameter count, void-ness of its return, its variables[] count split into args and
locals, and the argument count of every call it prints.
"""
import json, os, re, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = ["O2/bash/stripped/bash", "O2-noinline/dash/stripped/dash", "O0/kmod/stripped/kmod",
        "O2-noinline/cronie/stripped/crontab", "O2/dpkg/stripped/dpkg-divert", "O0/zlib/stripped/minigzip",
        "O2/libexpat/stripped/xmlwf", "O0/e2fsprogs/stripped/e2fsck", "O2/openssh-portable/stripped/sftp",
        "O0/iproute2/stripped/ip"]
K, OUT = sys.argv[1], sys.argv[2]
os.makedirs(OUT, exist_ok=True)
NOT_CALLS = {"if", "while", "for", "switch", "return", "sizeof"}


def calls(code):
    out = []
    for m in re.finditer(r"\b([A-Za-z_]\w*)\(", code):
        if m.group(1) in NOT_CALLS:
            continue
        i, depth, args, empty = m.end() - 1, 0, 1, True
        while i < len(code):
            c = code[i]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    break
            elif c == "," and depth == 1:
                args += 1
            elif depth >= 1 and not c.isspace():
                empty = False
            i += 1
        out.append((m.group(1), 0 if empty else args))
    return sorted(out)


def run(tag, b, arm):
    p = subprocess.run([K, "decompile-all", f"{D}/{b}", "--json", "--max-fn-seconds", "120",
                        "--option", "callrettype", arm], capture_output=True, text=True)
    open(f"{OUT}/{tag}.{arm}.json", "w").write(p.stdout)
    return p.returncode


jobs = [(b.split("/")[0] + "_" + b.split("/")[-1], b, arm) for b in BINS for arm in ("off", "on")]
with ThreadPoolExecutor(8) as ex:
    rcs = list(ex.map(lambda j: run(*j), jobs))
summary = {}
for b in BINS:
    tag = b.split("/")[0] + "_" + b.split("/")[-1]
    f = {arm: {x["address"]: x for x in json.load(open(f"{OUT}/{tag}.{arm}.json"))["functions"]} for arm in ("off", "on")}
    changed, arity, phantom, varcount = [], [], [], []
    for a in sorted(set(f["off"]) & set(f["on"])):
        o, n = f["off"][a], f["on"][a]
        co, cn = o.get("code") or "", n.get("code") or ""
        if co != cn:
            changed.append(a)
        if calls(co) != calls(cn):
            arity.append(a)
        void = lambda c: re.match(r"void [A-Za-z_]", c.split("\n", 1)[0]) is not None
        if void(co) != void(cn):
            phantom.append(a)
        vo = sorted((v.get("kind"), str(v.get("arg_index")), str(v.get("stack_offset")), v.get("size")) for v in o.get("variables") or [])
        vn = sorted((v.get("kind"), str(v.get("arg_index")), str(v.get("stack_offset")), v.get("size")) for v in n.get("variables") or [])
        if vo != vn:
            varcount.append(a)
    summary[tag] = {"functions": len(set(f["off"]) & set(f["on"])), "code_changed": len(changed),
                    "call_arity_moved": len(arity), "return_voidness_moved": len(phantom),
                    "variables_rows_moved": len(varcount), "changed": [hex(x) for x in changed],
                    "arity": [hex(x) for x in arity], "phantom": [hex(x) for x in phantom],
                    "varrows": [hex(x) for x in varcount]}
    print(tag, {k: v for k, v in summary[tag].items() if not isinstance(v, list)}, flush=True)
json.dump(summary, open(f"{OUT}/summary.json", "w"), indent=1)
print("CORPUS_DONE rcs", rcs)
