import json, collections, re, sys
sys.argv = sys.argv[:1]
LIBC = {"FILE*", "stat*", "timespec*", "tm*", "dirent*", "passwd*", "group*", "spwd*", "sgrp*", "termios*",
        "utmpx*", "sigaction*", "timeval*", "option*", "lconv*", "obstack*", "re_pattern_buffer*",
        "statfs*", "rlimit*", "winsize*", "_IO_FILE*", "DIR*", "mbstate_t*", "__mbstate_t*", "fd_set*",
        "pollfd*", "sockaddr*", "addrinfo*", "utsname*", "stat64*", "flock*", "iovec*", "utmp*"}
R = json.load(open('/home/mahaloz/kwt/o2census/.scratch/sweep-o2/rows.json'))


def bucket(d, karity):
    s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
    cls = d["cls"] or ""
    kt = re.sub(r"struct_\d+", "struct_N", d.get("kuna_type") or "")
    if s == "reg" or (cls.startswith("ptr") and d["gt_type"] == "void"):
        return "U"
    prog = cls == "ptr_struct" and d["gt_type"] not in LIBC
    if d["ok"] is None:
        if s == "arg":
            return "S:struct-arg-missing" if prog else "P:arity-short"
        return "P:stack-slot-missing"
    if prog:
        if kt == "struct_N *":
            return "S:struct_N*(credited by #93)"
        if kt == "void *":
            return "S:struct->void*"
        return "S:struct->int/prim-ptr"
    if cls == "ptr_char":
        return "P:char*->int/void*"
    if cls in ("int_s4", "int_u4") and kt in ("unsigned long", "long", "undefined8"):
        return "P:int32->64-bit"
    if cls == "ptr_ptr":
        return "P:T**->int/prim*"
    if cls == "bool":
        return "P:bool->int/uchar"
    if cls == "ptr_void":
        return "P:void*->" + ("struct_N*" if kt == "struct_N *" else "int/prim*")
    if cls == "ptr_struct":
        return "P:libc-struct*->other"
    return "P:other(%s/%s)" % (s, cls)


res = {}
for opt in ("O2", "O2-noinline"):
    solo = collections.Counter()
    one = collections.Counter()
    vars_ = collections.Counter()
    ex = collections.defaultdict(list)
    for k, v in R.items():
        if k.split("::")[1] != opt:
            continue
        vals = v["base"]["values"]
        by = collections.defaultdict(list)
        for d in v["decisions"]["base"]:
            by[d["fn"]].append(d)
        for fn, ds in by.items():
            if fn not in vals or vals[fn] == 1.0:
                continue
            bl = [bucket(d, 0) for d in ds if d["ok"] is not True]
            for b in bl:
                vars_[b] += 1
            if "U" in bl:
                continue
            bs = set(bl)
            if len(bs) == 1:
                b = bl[0]
                solo[b] += 1
                if len(bl) == 1:
                    one[b] += 1
                ex[b].append((len(bl), len(ds), k, fn))
    res[opt] = {"solo": dict(solo), "one": dict(one), "vars_nonperfect": dict(vars_),
                "ex": {b: sorted(e)[:4] for b, e in ex.items()}}
tot = collections.Counter()
for opt in res:
    for b, n in res[opt]["solo"].items():
        tot[b] += n
print("bucket | O2 solo(one-short) | O2-ni solo(one-short) | vars in non-perfect O2/O2-ni")
for b, _ in tot.most_common():
    a, c = res["O2"], res["O2-noinline"]
    print(f"{b} | {a['solo'].get(b,0)} ({a['one'].get(b,0)}) | {c['solo'].get(b,0)} ({c['one'].get(b,0)}) | {a['vars_nonperfect'].get(b,0)}/{c['vars_nonperfect'].get(b,0)}")
json.dump(res, open('/home/mahaloz/kwt/o2census/.scratch/levers.json', 'w'), indent=1)
