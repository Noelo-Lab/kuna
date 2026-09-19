#!/usr/bin/env python3
"""Layout precision and record identity for the ledger, on tuning and held-out builds.

    D=docs/features/structsynth/dedup_heldout.py
    ~/.virtualenvs/decbench/bin/python $D fetch main <main kuna>     # cache json + .h
    ~/.virtualenvs/decbench/bin/python $D fetch branch <branch kuna>
    ~/.virtualenvs/decbench/bin/python $D score branch               # fields-only P/R/F1
    ~/.virtualenvs/decbench/bin/python $D absorb main branch         # record identity

`score` is `layoutscore.py`'s fields-only measure over eight sets: the eight
builds the growth bounds were tuned on, 17 a reviewer picked, 22 measured only
after the pointer rule was fixed, 28 a second reviewer picked outside coreutils
(the table and integer rules were drawn from these), 26 chosen before any
result under those two rules was seen, 24 and 18 a third reviewer picked outside
all of those (the padding rule was drawn from the 18), and 34 chosen before any
result under the padding rule was seen.  `absorb` compares two builds parameter by
parameter: a parameter the branch answers with a strictly larger structure than
main gave it is an absorption, and it is on the SAME record when some parameter
of the same DWARF record measured exactly that structure on main.  Field
precision cannot see a wrong record whose fields line up (`stat` and
`cycle_check_state` agree at every measured offset); this can.  Outputs are
cached under $DEDUP_CACHE (default /tmp/structdedup-heldout).
"""
import json, os, re, subprocess, sys, tempfile, shutil
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor

WT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WT))
from scripts.decbench import structscore as SS

RES = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
TUNING = [("coreutils", b, o) for b in ("fmt", "ls", "sort", "du") for o in ("O0", "O2")]
HELDOUT = ([("coreutils", b, o) for b in ("cp", "pr", "od", "tail", "df", "csplit")
            for o in ("O0", "O2")]
           + [("kmod", "kmod", "O0"), ("kmod", "kmod", "O2"), ("dash", "dash", "O0"),
              ("dash", "dash", "O2"), ("gzip", "gzip", "O2")])
HELDOUT2 = ([("coreutils", b, o) for b in ("mv", "rm", "stat", "join", "wc", "split")
             for o in ("O0", "O2")]
            + [("cronie", "crond", "O0"), ("cronie", "crond", "O2"), ("libexpat", "xmlwf", "O0"),
               ("libexpat", "xmlwf", "O2"), ("sysvinit", "init", "O2"), ("dpkg", "dpkg-divert", "O2"),
               ("diffutils", "cmp", "O2"), ("diffutils", "sdiff", "O2"), ("libacl", "setfacl", "O2"),
               ("shadow", "chage", "O2")])
HELDOUT3 = [tuple(x.split()) for x in (
    "coreutils shuf O2|coreutils ptx O2|coreutils tsort O2|coreutils dd O2|coreutils pinky O2|"
    "coreutils nl O2|coreutils tr O2|coreutils uniq O2|coreutils cut O2|coreutils date O2|"
    "coreutils chmod O2|coreutils ptx O0|coreutils shuf O0|coreutils dd O0|dpkg dpkg-query O2|"
    "dpkg dpkg O2|iproute2 rtmon O2|iproute2 ip O2|openssh-portable ssh-keyscan O2|"
    "openssh-portable sftp O2|diffutils diff3 O2|zlib minigzip O2|cronie crontab O0|"
    "sysvinit last O2|gnutls psktool O2|bzip2 bzip2 O2|e2fsprogs e2fsck O2|rsyslog rsyslogd O2"
).split("|")]
HELDOUT4 = [tuple(x.split()) for x in (
    "bash bash O2|gnutls certtool O2|gnutls gnutls-cli O2|gnutls ocsptool O2|shadow passwd O2|"
    "shadow groupadd O2|shadow login O2|sysvinit bootlogd O2|sysvinit killall5 O2|"
    "sysvinit shutdown O2|openssh-portable ssh-agent O2|openssh-portable scp O2|"
    "openssh-portable ssh-add O2|dpkg dpkg-statoverride O2|dpkg dpkg-trigger O2|zlib example O2|"
    "cronie cronnext O2|base-passwd update-passwd O2|grep grep O0|diffutils diff O0|"
    "iproute2 ip O0|rsyslog rsyslogd O0|e2fsprogs e2fsck O0|coreutils touch O2|coreutils expr O2|"
    "coreutils seq O2"
).split("|")]
REVIEW1 = [tuple(x.split()) for x in (
    "openssh-portable sshd O2|openssh-portable ssh-keygen O2|gnutls gnutls-serv O2|"
    "libselinux libselinux.so.1 O2|libbsd libbsd.so.0.11.7 O2|libedit libedit.so.0.0.70 O2|"
    "zlib libz.so.1.2.13 O2|cronie crontab O2|libacl libacl.so.1.1.2301 O2|"
    "libselinux libselinux.so.1 O0|libedit libedit.so.0.0.70 O0|openssh-portable sftp-server O0|"
    "openssh-portable sftp-server O2|zlib libz.so.1.2.13 O0|libedit libedit.so.0.0.70 O2-noinline|"
    "openssh-portable ssh-keygen O2-noinline|gnutls srptool O2|openssh-portable ssh-keygen O0|"
    "openssh-portable sshd O0|openssh-portable ssh O2|gnutls gnutls-cli-debug O2|"
    "openssh-portable ssh-pkcs11-helper O2|libacl chacl O2|sysvinit utmpdump O2"
).split("|")]
REVIEW2 = [tuple(x.split()) for x in (
    "bash bash O0|openssh-portable ssh O0|openssh-portable sftp O0|openssh-portable scp O0|"
    "openssh-portable ssh-agent O0|gnutls certtool O0|gnutls gnutls-cli O0|dpkg dpkg O0|"
    "dpkg dpkg-query O0|sysvinit init O0|libbsd libbsd.so.0.11.7 O0|"
    "libselinux libselinux.so.1 O2-noinline|openssh-portable ssh O2-noinline|"
    "openssh-portable sshd O2-noinline|libexpat xmlwf O2-noinline|cronie crond O2-noinline|"
    "dpkg dpkg O2-noinline|iproute2 ip O2-noinline"
).split("|")]
FRESH6 = [tuple(x.split()) for x in (
    "bash bash O2-noinline|bash man2html O0|bash mkbuiltins O2|dpkg dpkg-divert O0|"
    "dpkg dpkg-query O2-noinline|dpkg dpkg-trigger O0|dpkg dpkg-statoverride O0|"
    "rsyslog rsyslogd O2-noinline|e2fsprogs e2fsck O2-noinline|kmod kmod O2-noinline|"
    "dash dash O2-noinline|openssh-portable ssh-keysign O2|openssh-portable ssh-add O0|"
    "openssh-portable sftp O2-noinline|gnutls gnutls-serv O0|gnutls ocsptool O0|iproute2 rtmon O0|"
    "grep grep O2|gzip gzip O0|diffutils diff O2|diffutils diff3 O0|zlib minigzip O0|"
    "libacl setfacl O0|sysvinit shutdown O0|sysvinit killall5 O0|cronie crontab O2-noinline|"
    "libbsd libbsd.so.0.11.7 O2-noinline|bzip2 bzip2 O0|coreutils ginstall O2|coreutils stty O0|"
    "coreutils cksum O2-noinline|shadow useradd O0|shadow usermod O2-noinline|shadow newusers O2"
).split("|")]
SETS = (("TUNING", TUNING), ("HELDOUT", HELDOUT), ("HELDOUT2", HELDOUT2),
        ("HELDOUT3", HELDOUT3), ("HELDOUT4", HELDOUT4), ("REVIEW1", REVIEW1),
        ("REVIEW2", REVIEW2), ("FRESH6", FRESH6))
CACHE = Path(os.environ.get("DEDUP_CACHE", "/tmp/structdedup-heldout"))
OPTS = ["--option", "structsynth", "param"]
ENV = dict(os.environ, KUNA_SPECS=os.environ.get("KUNA_SPECS", str(WT / "specs")),
           SLEIGHHOME=os.environ.get("SLEIGHHOME", str(WT / "specs")))


def stem(p, b, o):
    return f"{o}-{p}-{b}"


def fetch_one(build, kuna, p, b, o):
    d = CACHE / build
    d.mkdir(parents=True, exist_ok=True)
    binary = RES / o / p / "stripped" / b
    jp, hp = d / f"{stem(p, b, o)}.json", d / f"{stem(p, b, o)}.h"
    if not jp.exists():
        r = subprocess.run([kuna, "decompile-all", str(binary), "--json", "--max-fn-seconds", "120"]
                           + OPTS, capture_output=True, text=True, timeout=3600, env=ENV)
        if not r.stdout.strip():
            return f"{build} {stem(p, b, o)} json EMPTY rc={r.returncode} {r.stderr[-200:]}"
        jp.write_text(r.stdout)
    if not hp.exists():
        out = Path(tempfile.mkdtemp(prefix="structdedup-ls-"))
        try:
            subprocess.run([kuna, "decompile-project", str(binary), "-o", str(out),
                            "--max-fn-seconds", "120"] + OPTS,
                           capture_output=True, text=True, timeout=3600, env=ENV)
            hs = sorted(out.glob("*.h"))
            hp.write_text(hs[0].read_text() if hs else "")
        finally:
            shutil.rmtree(out, ignore_errors=True)
    return f"{build} {stem(p, b, o)} ok"


def parse_header(h):
    """{name: {size, fields:[(off,size,type)]}} with filler (array members) dropped."""
    raw = SS.header_layouts(h)
    out = {}
    for name, body in SS.STRUCT_RE.findall(h):
        if name not in raw:
            continue
        types = {}
        for line in body.split("\n"):
            m = SS.MEMBER_RE.match(line)
            if m:
                types[m.group("name")] = (" ".join(m.group("type").split()) + " " + m.group("stars"),
                                          bool(m.group("count")))
        fields = []
        for f in raw[name]["fields"]:
            ty, arr = types.get(f["name"], ("?", False))
            if arr:
                continue
            fields.append((f["offset"], f["size"], ty.strip()))
        out[name] = {"size": raw[name]["size"], "fields": fields}
    return out


_dwarf = {}


def dwarf(p, b, o):
    k = (p, b, o)
    if k not in _dwarf:
        _dwarf[k] = SS.dwarf_functions(RES / o / p / "compiled" / b)
    return _dwarf[k]


def params(build, p, b, o):
    d = CACHE / build
    payload = json.loads((d / f"{stem(p, b, o)}.json").read_text())
    layouts = parse_header((d / f"{stem(p, b, o)}.h").read_text())
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    rows = []
    for low, info in dwarf(p, b, o).items():
        fn = by_addr.get(low)
        for param in info["params"]:
            lay = param["struct"]
            if not lay:
                continue
            gt = sorted({(f["offset"], f["size"]) for f in lay["fields"] if f["kind"] != "pad"})
            pname, pred = None, None
            if fn:
                for v in fn.get("variables") or []:
                    if v.get("arg_index") == param["index"]:
                        rec = SS.parse_type(v.get("type"))
                        if rec["kind"] == "struct" and rec["base"] in layouts:
                            pname, pred = rec["base"], layouts[rec["base"]]
                        break
            rows.append({"fn": low, "fname": info["name"], "idx": param["index"],
                         "gt_name": lay["name"], "gt": gt, "pname": pname,
                         "psize": pred["size"] if pred else None,
                         "pfields": pred["fields"] if pred else None})
    return rows


def prf(tp, pred, gt):
    pr = tp / pred if pred else 0.0
    rc = tp / gt if gt else 0.0
    return pr, rc, (2 * pr * rc / (pr + rc) if pr + rc else 0.0)


def score(build):
    for label, bins in SETS:
        T = P = G = 0
        for p, b, o in bins:
            tp = pred = gtn = 0
            for r in params(build, p, b, o):
                gtn += len(r["gt"])
                if r["pfields"] is None:
                    continue
                gk = set(r["gt"])
                pred += len(r["pfields"])
                tp += sum(1 for f in r["pfields"] if (f[0], f[1]) in gk)
            pr, rc, f1 = prf(tp, pred, gtn)
            print(f"{label} {b} {o}: P={pr:.4f} ({tp}/{pred}) R={rc:.4f} gt={gtn}")
            T, P, G = T + tp, P + pred, G + gtn
        pr, rc, f1 = prf(T, P, G)
        print(f"{label} POOLED fields only: precision {T}/{P} = {pr:.4f}  recall {rc:.4f}  F1 {f1:.4f}")


def absorb(base, build):
    events = []
    for label, bins in SETS:
        for p, b, o in bins:
            br = {(r["fn"], r["idx"]): r for r in params(base, p, b, o)}
            for r in params(build, p, b, o):
                a = br.get((r["fn"], r["idx"]))
                if a is None:
                    continue
                if (a["pfields"] is None) != (r["pfields"] is None):
                    events.append({"set": label, "bin": f"{b} {o}", "kind": "retyped-diff",
                                   "fname": r["fname"], "idx": r["idx"]})
                    continue
                if r["pfields"] is None:
                    continue
                L, S = set(map(tuple, a["pfields"])), set(map(tuple, r["pfields"]))
                if L == S and a["psize"] == r["psize"]:
                    continue
                gk = set(r["gt"])
                extra = S - L
                lost = L - S
                events.append({
                    "set": label, "bin": f"{b} {o}", "fname": r["fname"], "idx": r["idx"],
                    "gt_name": r["gt_name"], "kind": "absorb" if not lost else "other",
                    "Lsize": a["psize"], "Lclaims": len(L), "Ssize": r["psize"], "Sclaims": len(S),
                    "Lptr": any("*" in f[2] for f in L),
                    "Lmaxend": max((f[0] + (f[1] or 0) for f in L), default=0),
                    "extra": len(extra), "extra_tp": sum(1 for f in extra if (f[0], f[1]) in gk),
                    "lost": len(lost), "lost_tp": sum(1 for f in lost if (f[0], f[1]) in gk),
                    "L": sorted(L), "S": sorted(S)})
    return events


def identity(base, events):
    """Tag each absorption with whether the structure came from the same DWARF record."""
    import collections
    own = {}
    for _, bins in SETS:
        for p, b, o in bins:
            m = collections.defaultdict(set)
            for r in params(base, p, b, o):
                if r["pfields"] is not None:
                    m[tuple(sorted(map(tuple, r["pfields"])))].add(r["gt_name"])
            own[f"{b} {o}"] = m
    for e in events:
        if "S" not in e:
            continue
        owners = own[e["bin"]].get(tuple(sorted(map(tuple, e["S"]))), set())
        e["record"] = "same" if e["gt_name"] in owners else ("different" if owners else "uncheckable")
    return events


if __name__ == "__main__":
    import collections
    cmd = sys.argv[1]
    if cmd == "fetch":
        build, kuna = sys.argv[2], sys.argv[3]
        jobs = int(os.environ.get("LS_JOBS", "6"))
        todo = sorted({t for _, bins in SETS for t in bins},
                      key=lambda t: -(RES / t[2] / t[0] / "stripped" / t[1]).stat().st_size)
        with ThreadPoolExecutor(jobs) as ex:
            for line in ex.map(lambda t: fetch_one(build, kuna, *t), todo):
                print(line, flush=True)
    elif cmd == "score":
        score(sys.argv[2])
    elif cmd == "absorb":
        ev = identity(sys.argv[2], absorb(sys.argv[2], sys.argv[3]))
        for label, _ in SETS:
            c = collections.Counter(e.get("record") for e in ev if e["set"] == label)
            added = [e for e in ev if e["set"] == label and "extra" in e]
            print(f"{label}: {sum(c.values())} absorbed, same record {c['same']}, "
                  f"different {c['different']}, uncheckable {c['uncheckable']}, added fields "
                  f"true {sum(e['extra_tp'] for e in added)}/{sum(e['extra'] for e in added)}")
        for e in ev:
            if e.get("record") == "different":
                print("  different:", e["bin"], e["fname"], e["idx"], e["gt_name"], "L", e["L"])
