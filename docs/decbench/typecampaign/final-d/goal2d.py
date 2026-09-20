"""Goal-2 census: varcensus + phantom locals/args, baseline vs final kuna."""
import json, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/_final-main")
sys.path.insert(0, "/home/mahaloz/kwt/_final-d/tools")
from scripts.pipeline import varcensus as VC
import callsite_metrics as CM

R = Path("/home/mahaloz/github/decbench/results/full_run_address_2026-09-11")
OUT = Path("/home/mahaloz/kwt/_final-d/goal2")
KUNAS = {"d": ("/home/mahaloz/kwt/_final-d/kuna", "/home/mahaloz/kwt/_final-main/specs"),
         "d-argclobberoff": ("/home/mahaloz/kwt/_final-d/kuna", "/home/mahaloz/kwt/_final-main/specs")}
BINS = [(o, b) for o in ("O2", "O0") for b in ("fmt", "ls", "sort", "du")]


def run(label, opt, b):
    kb, specs = KUNAS[label]
    env = dict(os.environ, SLEIGHHOME=specs, KUNA_SPECS=specs)
    binp = R / opt / "coreutils" / "stripped" / b
    d = OUT / label
    d.mkdir(parents=True, exist_ok=True)
    extra = ["--option", "argclobber", "off"] if label.endswith("argclobberoff") else []
    t = subprocess.run([kb, "decompile-all", str(binp)] + extra, capture_output=True, text=True, env=env, timeout=3600)
    j = subprocess.run([kb, "decompile-all", str(binp), "--json"] + extra, capture_output=True, text=True, env=env, timeout=3600)
    (d / f"{opt}-{b}.txt").write_text(t.stdout)
    (d / f"{opt}-{b}.json").write_text(j.stdout)
    payload = json.loads(j.stdout)
    doc = VC.build(binp, t.stdout, payload, [], None)
    ph_locals = ph_args = args = 0
    fn_with = 0
    for f in payload.get("functions") or []:
        code = f.get("code") or ""
        u = CM.unassigned_locals(code)
        ph_locals += len(u)
        fn_with += bool(u)
        for callee, lst in CM.sites(code).items():
            for site in lst:
                args += len(site)
                ph_args += sum(1 for a in site if a in u)
    doc["totals"].update({"phantom_locals": ph_locals, "phantom_args": ph_args,
                          "functions_with_phantom": fn_with, "call_args": args})
    (d / f"{opt}-{b}.census.json").write_text(json.dumps(doc))
    return label, opt, b, doc["totals"]


if __name__ == "__main__":
    jobs = [(l, o, b) for l in KUNAS for (o, b) in BINS]
    res = {}
    with ThreadPoolExecutor(max_workers=16) as ex:
        for l, o, b, tot in ex.map(lambda x: run(*x), jobs):
            res.setdefault(l, {})[f"{o}/{b}"] = tot
            print(l, o, b, {k: tot.get(k) for k in ("functions", "declarations", "declarations_non_thunk", "single_def_single_use_non_thunk", "blob16", "phantom_locals", "phantom_args")}, flush=True)
    (OUT / "summary.json").write_text(json.dumps(res, indent=1))
    print("GOAL2_DONE")
