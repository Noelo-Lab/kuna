"""A/A/B: origin/main vs origin/main vs this branch's default, interleaved, min-of-N.

The first two arms are the SAME binary run twice, so their delta is the noise
floor of this box at the moment of measurement.
"""
import subprocess, sys, time, os
WT="/home/mahaloz/kwt/indirectonly"; MAIN="/home/mahaloz/kwt/_rev-lcg-main"
ARMS={"mainA":(MAIN+"/decompiler/target/release/kuna", MAIN+"/specs"),
      "mainB":(MAIN+"/decompiler/target/release/kuna", MAIN+"/specs"),
      "branch":(WT+"/decompiler/target/release/kuna", WT+"/specs")}
def one(which,b):
    k,sp=ARMS[which]
    env=dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
    t0=time.perf_counter()
    r=subprocess.run([k,"decompile-all",b],capture_output=True,env=env)
    dt=(time.perf_counter()-t0)*1000.0
    if r.returncode!=0: raise SystemExit("FAILED "+r.stderr.decode()[:200])
    return dt
b=sys.argv[1]; n=int(sys.argv[2]) if len(sys.argv)>2 else 7
for a in ARMS: one(a,b)
res={a:[] for a in ARMS}
for _ in range(n):
    for a in ("mainA","mainB","branch"): res[a].append(one(a,b))
m={a:min(v) for a,v in res.items()}
print("%s n=%d  load=%s" % (os.path.basename(b), n, os.getloadavg()[0]))
for a in ("mainA","mainB","branch"): print("  %-7s min=%9.1f ms" % (a, m[a]))
print("  NOISE FLOOR mainB vs mainA = %+.2f%%" % ((m['mainB']-m['mainA'])/m['mainA']*100))
print("  branch vs best-of-main     = %+.2f%%" % ((m['branch']-min(m['mainA'],m['mainB']))/min(m['mainA'],m['mainB'])*100))
