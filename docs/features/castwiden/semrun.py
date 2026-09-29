"""decompile-all both castwiden arms (off, the default) with struct definitions
printed, for semhunks.py to type structure fields:

  python3 semrun.py <kuna> <outdir> castbench|corpus8|extra11

castbench is castbench.py's full corpus (15 binaries x O0/O2/O2-noinline),
corpus8 eight whole binaries outside it, extra11 eleven binaries outside
both (32-bit PEs among them).  Output: <outdir>/{off,default}/<opt>_<project>_<bin>.c
"""
import json, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, "/home/mahaloz/kwt/castbench")
R = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
CORPUS8 = ["O0/bzip2/bzip2", "O2/bash/bash", "O2/kmod/kmod", "O2/dpkg/dpkg-divert", "O0/shadow/chage",
           "O2/cronie/crontab", "O2-noinline/dash/dash", "O0/cronie/crond"]
EXTRA11 = ["O0/e2fsprogs/e2fsck", "O0/zlib/example64", "O2/iproute2/ip", "O2/libexpat/xmlwf",
           "O2/openssh-portable/sftp-server", "O2/mirai/mirai", "O2/sysvinit/last", "O2/zlib/minigzip",
           "O2/minipig/minipig.exe", "O2/mydoom/mydoom.exe", "O2-noinline/sysvinit/init"]


def binaries(which):
    if which == "castbench":
        import castbench
        return [f"{o}/{p}/{b}" for o, p, b in castbench.corpus("full")]
    return {"corpus8": CORPUS8, "extra11": EXTRA11}[which]


def one(job):
    k, out, rel, arm = job
    opt, proj, b = rel.split("/")
    cmd = [k, "decompile-all", str(R / opt / proj / "stripped" / b), "--json", "--option", "structdefs", "on"]
    if arm == "off":
        cmd += ["--option", "castwiden", "off"]
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    if p.returncode != 0:
        return f"{rel} {arm} rc={p.returncode}"
    d = json.loads(p.stdout)
    parts = [f"// Function: {f['name']} @ {hex(f['address'])}\n{f['code']}\n" for f in d["functions"] if f.get("code")]
    dst = out / arm / f"{opt}_{proj}_{b}.c"
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text("\n".join(parts))
    return None


if __name__ == "__main__":
    k, out, which = sys.argv[1], Path(sys.argv[2]), sys.argv[3]
    jobs = [(k, out, rel, arm) for rel in binaries(which) for arm in ("off", "default")]
    with ThreadPoolExecutor(12) as ex:
        errs = [e for e in ex.map(one, jobs) if e]
    print(f"{which}: {len(jobs) - len(errs)}/{len(jobs)} runs", *errs, sep="\n")
