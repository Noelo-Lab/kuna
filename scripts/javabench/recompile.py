"""Recompile decompiled top-level classes one file at a time and byte-compare.

Per decompiled file: ``javac --release <class-file version> [-g] -proc:none
-implicit:none`` against ``classes.jar`` + ``deps/*.jar`` (annotation processors
are disabled: no project code runs). The javac matching the original build JDK
is used (``Project.javac_jdk``).

``class_compiles`` (the headline number) is the result of compiling the RAW
decompiled file. To still byte-score the healthy methods of a class that fails
because of a few bad methods (decbench compiles per function), up to three
salvage rounds replace the bodies of methods that contain javac errors with
``throw new RuntimeException()`` (line-count preserving) and recompile; stubbed
methods score 0 / ``compiled=False``. Errors outside method bodies (imports,
headers, field initializers) are not salvaged and fail every method of the file.
"""

from __future__ import annotations

import hashlib
import os
import re
import subprocess
import sys
import time
import zipfile
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
from dataclasses import dataclass, field
from pathlib import Path

from . import projects as P
from .bytecode import listings, match_classes, similarity
from .classfile import parse_class

SERVER_SRC = Path(__file__).parent / "java" / "Recompile.java"
STUB = 'throw new RuntimeException("javabench-stub");'
SALVAGE_ROUNDS = 3


def server_dir() -> Path:
    digest = hashlib.sha256(SERVER_SRC.read_bytes()).hexdigest()[:12]
    d = P.TOOLS_DIR / f"recompile-server-{digest}"
    if not (d / "Recompile.class").exists():
        d.mkdir(parents=True, exist_ok=True)
        javac = os.path.join(P.JDKS[17], "bin/javac")
        r = subprocess.run([javac, "-J-XX:-UsePerfData", "--release", "17", "-d", str(d), str(SERVER_SRC)],
                           capture_output=True, text=True, timeout=300)
        if r.returncode != 0:
            raise SystemExit(f"cannot build compile server:\n{r.stderr}")
    return d


@dataclass
class Job:
    id: str
    src: Path
    out: Path
    opts: list[str]


@dataclass
class JobResult:
    ok: bool
    ms: int = 0
    diags: list[tuple[str, int, int, str]] = field(default_factory=list)


def javac_opts(release: int, debug: str, mode: str = "release") -> list[str]:
    target = ["--release", str(release)] if mode == "release" else ["-source", str(release), "-target", str(release)]
    return target + ["-proc:none", "-implicit:none", "-nowarn", "-Xlint:none",
            "-XDsuppressNotes", "-encoding", "UTF-8", "-Xmaxerrs", "100000", debug]


def run_jobs(jobs: list[Job], classpath: list[Path], jdk: int, workers: int, workdir: Path,
             xmx: str = "2g", timeout: int = 7200) -> dict[str, JobResult]:
    if not jobs:
        return {}
    workdir.mkdir(parents=True, exist_ok=True)
    sdir = server_dir()
    java = os.path.join(P.JDKS[jdk], "bin/java")
    nshard = max(1, min(workers, len(jobs)))
    shards: list[list[Job]] = [[] for _ in range(nshard)]
    for i, j in enumerate(sorted(jobs, key=lambda j: -j.src.stat().st_size)):
        shards[i % nshard].append(j)
    cp = os.pathsep.join(str(p) for p in classpath)
    procs = []
    for k, sh in enumerate(shards):
        jf = workdir / f"shard{k}.tsv"
        rf = workdir / f"shard{k}.result.tsv"
        with open(jf, "w") as f:
            f.write(f"#CP\t{cp}\n")
            for j in sh:
                f.write(f"{j.id}\t{j.src}\t{j.out}\t{chr(0x1f).join(j.opts)}\n")
        log = open(workdir / f"shard{k}.log", "wb")
        cmd = [java, f"-Xmx{xmx}", "-XX:+UseSerialGC", "-XX:-UsePerfData",
               f"-Djavabench.sp={workdir / 'empty-sourcepath'}",
               f"-Djava.io.tmpdir={workdir}", "-cp", str(sdir), "Recompile", str(jf), str(rf)]
        procs.append((subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT), rf, sh, log))
    t0 = time.monotonic()
    results: dict[str, JobResult] = {}
    for p, rf, sh, log in procs:
        try:
            p.wait(timeout=max(1, timeout - (time.monotonic() - t0)))
        except subprocess.TimeoutExpired:
            p.kill()
            p.wait()
        log.close()
        if rf.exists():
            for line in rf.read_text(encoding="utf-8", errors="replace").splitlines():
                f = line.split("\t")
                if f[0] == "J" and len(f) >= 4:
                    results[f[1]] = JobResult(f[2] == "1", int(f[3]))
                elif f[0] == "D" and len(f) >= 6 and f[1] in results:
                    results[f[1]].diags.append((f[2], int(f[3]), int(f[4]), f[5]))
        for j in sh:
            if j.id not in results:
                results[j.id] = JobResult(False, 0, [("javabench.server.crash", -1, -1, f"exit {p.returncode}")])
    return results


# --- salvage -------------------------------------------------------------------


def _method_bodies(text: bytes):
    from .javasrc import index_source

    idx = index_source(text)
    out = []
    for m in idx.methods:
        if m.body is None:
            continue
        b = m.body
        eci = None
        if m.kind == "ctor" and b.named_child_count and b.named_children[0].type == "explicit_constructor_invocation":
            eci = b.named_children[0]
        out.append((b.start_byte, b.end_byte, eci.start_byte if eci else None, eci.end_byte if eci else None))
    return out


def diag_base_bytes(base: bytes, cur: bytes, diags) -> list[int | None]:
    """Map javac char positions in ``cur`` to byte offsets in ``base``.

    Stubs preserve line counts, so a position maps by (line, column)."""
    cc = cur.decode("utf-8", errors="replace")
    bc = base.decode("utf-8", errors="replace")
    b_starts = [0]
    for i, ch in enumerate(bc):
        if ch == "\n":
            b_starts.append(i + 1)
    out: list[int | None] = []
    for _code, _line, pos, _msg in diags:
        if pos is None or pos < 0 or pos > len(cc):
            out.append(None)
            continue
        line = cc.count("\n", 0, pos)
        col = pos - (cc.rfind("\n", 0, pos) + 1)
        if line >= len(b_starts):
            out.append(None)
            continue
        lend = b_starts[line + 1] - 1 if line + 1 < len(b_starts) else len(bc)
        bchar = min(b_starts[line] + col, lend)
        out.append(len(bc[:bchar].encode("utf-8", errors="replace")))
    return out


def stub_spans(base: bytes, cur: bytes, diags, prev: list[tuple[int, int]]) -> list[tuple[int, int]] | None:
    """Body byte spans (in ``base``) to stub so the errors in ``diags`` go away;
    None = unsalvageable (an error outside every method body, or no progress)."""
    bodies = _method_bodies(base)
    spans = set(prev)
    for b in diag_base_bytes(base, cur, diags):
        if b is None:
            return None
        inner = None
        for s, e, _es, _ee in bodies:
            if s < b < e:
                if inner is None or (e - s) < (inner[1] - inner[0]):
                    inner = (s, e)
        if inner is None:
            return None
        spans.add(inner)
    if spans == set(prev):
        return None
    return sorted(spans)


def apply_stubs(text: bytes, spans: list[tuple[int, int]], diag_bytes: set[int] | None = None) -> bytes:
    outer = []
    for s, e in sorted(spans):
        if outer and s >= outer[-1][0] and e <= outer[-1][1]:
            continue
        outer.append((s, e))
    from .javasrc import index_source

    idx = index_source(text)
    eci_of = {}
    for m in idx.methods:
        b = m.body
        if b is not None and m.kind == "ctor" and b.named_child_count and \
                b.named_children[0].type == "explicit_constructor_invocation":
            eci_of[(b.start_byte, b.end_byte)] = b.named_children[0]
    out = []
    pos = 0
    for s, e in outer:
        out.append(text[pos:s])
        body = text[s:e]
        keep = b""
        eci = eci_of.get((s, e))
        if eci is not None and not (diag_bytes and any(eci.start_byte <= d < eci.end_byte for d in diag_bytes)):
            keep = text[eci.start_byte:eci.end_byte] + b" "
        nl = body.count(b"\n") - keep.count(b"\n")
        out.append(b"{ " + keep + STUB.encode() + b"\n" * max(0, nl) + b" }")
        pos = e
    out.append(text[pos:])
    return b"".join(out)


def categorize(code: str) -> str:
    return code.replace("compiler.err.", "") if code else "unknown"


# --- byte comparison --------------------------------------------------------------

_JAR_NAMES: dict[str, dict[str, list[str]]] = {}


def _tops_index(jar: str) -> dict[str, list[str]]:
    d = _JAR_NAMES.get(jar)
    if d is None:
        d = {}
        for n in zipfile.ZipFile(jar).namelist():
            if n.endswith(".class"):
                d.setdefault(n[:-6].split("$")[0], []).append(n[:-6])
        _JAR_NAMES[jar] = d
    return d


def original_listings(jar: str, keys: list[str]) -> dict[str, tuple[list[str], list[str]]]:
    z = zipfile.ZipFile(jar)
    by_cls: dict[str, list[str]] = {}
    for k in keys:
        by_cls.setdefault(k.split("#")[0].replace(".", "/"), []).append(k)
    out = {}
    for cls, ks in by_cls.items():
        cf = parse_class(z.read(cls + ".class"))
        ms = {f"{cf.dotted}#{m.name}{m.descriptor}": m for m in cf.methods}
        for k in ks:
            if k in ms:
                out[k] = listings(cf, ms[k])
    return out


def compare_file(args) -> dict[str, dict]:
    jar, outdir, tops, keys = args
    z = zipfile.ZipFile(jar)
    tix = _tops_index(jar)
    original = {}
    for t in tops:
        for n in tix.get(t.replace(".", "/"), []):
            cf = parse_class(z.read(n + ".class"))
            original[cf.name] = cf
    recompiled = {}
    for p in Path(outdir).rglob("*.class"):
        try:
            cf = parse_class(p.read_bytes())
        except Exception:
            continue
        recompiled[cf.name] = cf
    rmap = match_classes(original, recompiled)
    inv = {o: r for r, o in rmap.items()}
    res = {}
    for k in keys:
        cls, rest = k.split("#", 1)
        internal = cls.replace(".", "/")
        ocf = original.get(internal)
        if ocf is None:
            res[k] = {"similarity": 0.0, "exact": False, "error": "original-class-missing"}
            continue
        om = next((m for m in ocf.methods if m.name + m.descriptor == rest), None)
        rname = inv.get(internal)
        if rname is None:
            res[k] = {"similarity": 0.0, "exact": False, "error": "class-not-recompiled"}
            continue
        rcf = recompiled[rname]
        rm = next((m for m in rcf.methods if m.name + m.descriptor == rest), None)
        if rm is None or om is None:
            res[k] = {"similarity": 0.0, "exact": False, "error": "method-not-in-recompiled-class"}
            continue
        try:
            osim, oex = listings(ocf, om)
            rsim, rex = listings(rcf, rm)
        except Exception as e:  # pragma: no cover
            res[k] = {"similarity": 0.0, "exact": False, "error": f"listing-failed: {e}"}
            continue
        sim, changed = similarity(osim, rsim)
        exact = oex == rex
        res[k] = {"similarity": 1.0 if exact else round(sim, 6), "exact": exact, "changed_lines": changed,
                  "original_insns": len(osim), "recompiled_insns": len(rsim)}
    return res


# --- driver ----------------------------------------------------------------------


def recompile_and_compare(proj: P.Project, run_dir: Path, files: list[dict], jobs: int,
                          debug_flag: str, tag: str = "recompile") -> tuple[dict, dict]:
    """``files``: [{rel, tops, keys, release}] -> (per-file results, per-key byte results).

    ``keys`` maps key -> decompiled body byte span (or None) for stub tracking.
    """
    dec_root = run_dir / "decompiled"
    work = run_dir / tag
    jar = str(proj.dir / "classes.jar")
    classpath = [proj.dir / "classes.jar"] + sorted((proj.dir / "deps").glob("*.jar"))
    state = {}
    for i, f in enumerate(files):
        fid = f"f{i:06d}"
        state[fid] = {"rel": f["rel"], "release": f["release"], "spans": [], "text": None,
                      "raw_ok": None, "ok": False, "out": None, "rounds": 0, "errors": [],
                      "unsalvageable": False}
    pending = list(state)
    for rnd in range(SALVAGE_ROUNDS + 1):
        if not pending:
            break
        jl = []
        for fid in pending:
            st = state[fid]
            if rnd == 0:
                src = dec_root / st["rel"]
            else:
                src = work / f"src-r{rnd}" / fid / Path(st["rel"]).name
                src.parent.mkdir(parents=True, exist_ok=True)
                src.write_bytes(st["text"])
            jl.append(Job(fid, src, work / f"out-r{rnd}" / fid, javac_opts(st["release"], debug_flag, proj.release_mode)))
        print(f"[recompile] {proj.name} round {rnd}: {len(jl)} files", file=sys.stderr, flush=True)
        res = run_jobs(jl, classpath, proj.javac_jdk, min(jobs, 16), work / f"jobs-r{rnd}")
        nxt = []
        for j in jl:
            st = state[j.id]
            r = res[j.id]
            st["rounds"] = rnd + 1
            if rnd == 0:
                st["raw_ok"] = r.ok
                st["errors"] = [(categorize(c), ln, msg) for c, ln, _p, msg in r.diags]
            if r.ok:
                st["ok"] = True
                st["out"] = str(j.out)
                continue
            base = (dec_root / st["rel"]).read_bytes()
            cur = st["text"] if st["text"] is not None else base
            spans = stub_spans(base, cur, r.diags, st["spans"])
            if spans is None or rnd == SALVAGE_ROUNDS:
                st["unsalvageable"] = True
                continue
            st["spans"] = spans
            st["diag_bytes"] = st.get("diag_bytes", set()) | {
                b for b in diag_base_bytes(base, cur, r.diags) if b is not None}
            st["text"] = apply_stubs(base, spans, st["diag_bytes"])
            nxt.append(j.id)
        pending = nxt
    # byte comparison
    tasks = []
    fid_of = {}
    for fid, st in state.items():
        f = files[int(fid[1:])]
        fid_of[f["rel"]] = fid
        if st["ok"] and f["keys"]:
            tasks.append((jar, st["out"], f["tops"], sorted(f["keys"])))
    per_key: dict[str, dict] = {}
    with ProcessPoolExecutor(max_workers=jobs) as ex:
        for r in ex.map(compare_file, tasks, chunksize=4):
            per_key.update(r)
    for fid, st in state.items():
        f = files[int(fid[1:])]
        for k, span in f["keys"].items():
            stubbed = bool(span) and any(s <= span[0] and span[1] <= e for s, e in st["spans"])
            rec = per_key.get(k)
            if not st["ok"]:
                per_key[k] = {"similarity": 0.0, "exact": False, "compiled": False, "error": "class-did-not-compile"}
            elif stubbed:
                per_key[k] = {"similarity": 0.0, "exact": False, "compiled": False, "error": "method-stubbed"}
            elif rec is None:
                per_key[k] = {"similarity": 0.0, "exact": False, "compiled": True, "error": "not-compared"}
            else:
                rec["compiled"] = True
    file_results = {}
    for fid, st in state.items():
        file_results[st["rel"]] = {
            "tops": list(files[int(fid[1:])]["tops"]),
            "raw_compiles": bool(st["raw_ok"]),
            "compiles_after_salvage": st["ok"],
            "stubbed_methods": len(st["spans"]),
            "rounds": st["rounds"],
            "error_categories": dict(Counter(c for c, _l, _m in st["errors"])),
            "first_errors": st["errors"][:5],
        }
    return file_results, per_key
