import json, collections
R = json.load(open('/home/mahaloz/kwt/o2census/.scratch/sweep-o2/rows.json'))
LIBC = {"FILE*", "stat*", "timespec*", "tm*", "dirent*", "passwd*", "group*", "spwd*", "sgrp*", "termios*",
        "utmpx*", "sigaction*", "timeval*", "option*", "lconv*", "obstack*", "re_pattern_buffer*",
        "statfs*", "rlimit*", "winsize*", "_IO_FILE*", "__dirstream*", "DIR*", "mbstate_t*", "__mbstate_t*",
        "fd_set*", "pollfd*", "sockaddr*", "addrinfo*", "utsname*", "stat64*", "flock*", "iovec*", "utmp*"}
out = {}
for opt in ("O2", "O2-noinline"):
    c = collections.Counter()
    arity = collections.Counter()
    sts = collections.Counter()
    for k, v in R.items():
        if k.split("::")[1] != opt:
            continue
        vals = v["base"]["values"]
        funcs = {f["fn"]: f for f in v["funcs"]}
        for fn, f in funcs.items():
            if f.get("gt_lowpc") is not None and f["gt_lowpc"] != f["kaddr"]:
                c["inst_mismatch_fn"] += 1
            if len(f.get("gt_inst") or []) > 1:
                c["multi_instance_fn"] += 1
        for d in v["decisions"]["base"]:
            s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
            if s == "reg":
                continue
            cls = d["cls"]
            if cls == "ptr_struct":
                lib = d["gt_type"] in LIBC
                sts[("libc" if lib else "program", s, d["ok"] is True)] += 1
            if s == "arg" and d["ok"] is None:
                arity[(d.get("lockind"), d["pass"])] += 1
            if s == "arg":
                c["arg_lockind_" + str(d.get("lockind"))] += 1
    out[opt] = {"c": dict(c), "arity_miss_lockind": {"|".join(map(str, k)): n for k, n in arity.most_common()},
                "ptr_struct": {"|".join(map(str, k)): n for k, n in sts.items()}}
print(json.dumps(out, indent=1))
