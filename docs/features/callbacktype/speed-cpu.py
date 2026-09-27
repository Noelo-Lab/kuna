"""CPU time (user+sys of the child tree), `callbacktype off` vs `on`, interleaved min-of-N.

Usage: speed-cpu.py N <bin,bin,...> relative to the decbench results tree.
"""
import os, resource, subprocess, sys, statistics
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
K = os.environ.get("KUNA_BIN", "/home/mahaloz/kwt/callbacktype/decompiler/target/release/kuna")
SP = "/home/mahaloz/kwt/callbacktype/specs"
N = int(sys.argv[1])
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
for b in sys.argv[2].split(","):
    t = {"off": [], "on": []}
    for i in range(N):
        for lab in (["off", "on"] if i % 2 == 0 else ["on", "off"]):
            r0 = resource.getrusage(resource.RUSAGE_CHILDREN)
            subprocess.run([K, "decompile-all", f"{R}/{b}", "--json", "--option", "callbacktype", lab],
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            r1 = resource.getrusage(resource.RUSAGE_CHILDREN)
            t[lab].append(r1.ru_utime + r1.ru_stime - r0.ru_utime - r0.ru_stime)
    mo, mn = min(t["off"]), min(t["on"])
    mdo, mdn = statistics.median(t["off"]), statistics.median(t["on"])
    print(b, "cpu min off %.2f on %.2f delta %+.2f%%  med %+.2f%%" % (mo, mn, (mn - mo) / mo * 100, (mdn - mdo) / mdo * 100),
          os.getloadavg(), flush=True)
print("CPU_DONE")
