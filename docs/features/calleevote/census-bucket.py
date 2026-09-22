import json, collections, re, sys
LIBC = {"FILE*", "stat*", "timespec*", "tm*", "dirent*", "passwd*", "group*", "spwd*", "sgrp*", "termios*",
        "utmpx*", "sigaction*", "timeval*", "option*", "lconv*", "obstack*", "re_pattern_buffer*",
        "statfs*", "rlimit*", "winsize*", "_IO_FILE*", "DIR*", "mbstate_t*", "__mbstate_t*", "fd_set*",
        "pollfd*", "sockaddr*", "addrinfo*", "utsname*", "stat64*", "flock*", "iovec*", "utmp*"}
def bucket(d):
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
        if kt == "struct_N *": return "S:struct_N*"
        if kt == "void *": return "S:struct->void*"
        return "S:struct->int/prim-ptr"
    if cls == "ptr_char": return "P:char*->int/void*"
    if cls == "ptr_ptr": return "P:T**->int/prim*"
    if cls == "ptr_void": return "P:void*->" + ("struct_N*" if kt == "struct_N *" else "int/prim*")
    if cls == "ptr_struct": return "P:libc-struct*->other"
    return "P:other(%s/%s)" % (s, cls)
R = json.load(open(sys.argv[1]))
out = []
for k, v in R.items():
    proj, opt, binn = k.split("::")
    if opt == "O0" and "--o0" not in sys.argv: continue
    vals = v["base"]["values"]
    by = collections.defaultdict(list)
    for d in v["decisions"]["base"]:
        by[d["fn"]].append(d)
    for fn, ds in by.items():
        if fn not in vals or vals[fn] == 1.0: continue
        bl = [(bucket(d), d) for d in ds if d["ok"] is not True]
        for b, d in bl:
            if b == "S:struct->void*":
                out.append(dict(key=k, fn=fn, solo=all(x == "S:struct->void*" for x, _ in bl), nblock=len(bl), gt=d["gt_type"], name=d["gt_name"], val=vals[fn]))
json.dump(out, open(sys.argv[2], "w"), indent=0)
c = collections.Counter((o["key"].split("::")[1], o["solo"]) for o in out)
print(c)
print(collections.Counter(o["gt"] for o in out if o["solo"]).most_common(20))
