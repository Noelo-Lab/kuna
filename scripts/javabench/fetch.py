"""Reproducibly fetch the dataset: ``<dataset>/<project>/{classes.jar,sources/,deps/,meta.json}``.

Every downloaded file is SHA-256 pinned in ``pins/<project>.json`` (the lock the
original fetch wrote). A project without a pin is resolved with coursier and its
lock WRITTEN to ``<workspace>/locks/``; every fetch with a pin or lock
re-downloads exactly the locked coordinates (``--intransitive``) and fails on
any checksum mismatch. Archives are treated as untrusted data: only
``.class`` / ``.java`` entries with safe relative names are extracted, into a
fresh directory, and nothing from them is ever executed.

Nothing is ever deleted: a rebuilt project directory is assembled next to the old
one and the old one is renamed to ``<project>.old-<timestamp>``.
"""

from __future__ import annotations

import gzip
import hashlib
import io
import json
import os
import shutil
import subprocess
import sys
import tarfile
import time
import urllib.request
import zipfile
from pathlib import Path, PurePosixPath

from . import projects as P
from .classfile import parse_class

VINEFLOWER_URL = f"{P.MAVEN}/org/vineflower/vineflower/1.12.0/vineflower-1.12.0.jar"
VINEFLOWER_SHA256 = "1dfcfe974395734fa467ce620661c7623d05ba83670de0529b1fbd63ff548b9d"


def _cache() -> Path:
    return P.DATASET_DIR / "_cache"


def log(msg: str) -> None:
    print(f"[fetch] {msg}", file=sys.stderr, flush=True)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def download(url: str, dest: Path, sha256: str | None = None, timeout: int = 900) -> str:
    """Download ``url`` to ``dest`` (skipped if present and matching); return its sha256."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists():
        digest = sha256_file(dest)
        if sha256 is None or digest == sha256:
            return digest
        raise SystemExit(f"checksum mismatch for cached {dest}: {digest} != {sha256}")
    part = dest.with_name(dest.name + f".part-{os.getpid()}")
    log(f"GET {url}")
    req = urllib.request.Request(url, headers={"User-Agent": "javabench-fetch/1"})
    with urllib.request.urlopen(req, timeout=timeout) as r, open(part, "wb") as out:
        shutil.copyfileobj(r, out, 1 << 20)
    digest = sha256_file(part)
    if sha256 is not None and digest != sha256:
        raise SystemExit(f"checksum mismatch for {url}: {digest} != {sha256}")
    os.replace(part, dest)
    return digest


def maven_url(coord: str, classifier: str | None = None, ext: str = "jar") -> str:
    g, a, v = coord.split(":")[:3]
    suffix = f"-{classifier}" if classifier else ""
    return f"{P.MAVEN}/{g.replace('.', '/')}/{a}/{v}/{a}-{v}{suffix}.{ext}"


def maven_cache_path(coord: str, classifier: str | None = None) -> Path:
    g, a, v = coord.split(":")[:3]
    suffix = f"-{classifier}" if classifier else ""
    return _cache() / "maven" / g.replace(".", "/") / a / v / f"{a}-{v}{suffix}.jar"


# --- tools -----------------------------------------------------------------


def ensure_tools() -> None:
    cs = P.TOOLS_DIR / "cs"
    if not cs.exists():
        gz = P.TOOLS_DIR / "cs-x86_64-pc-linux.gz"
        download(P.COURSIER_URL, gz, P.COURSIER_SHA256)
        tmp = P.TOOLS_DIR / f"cs.new-{os.getpid()}"
        tmp.write_bytes(gzip.decompress(gz.read_bytes()))
        tmp.chmod(0o755)
        os.replace(tmp, cs)
    jdk25 = Path(P.JDKS[25])
    if not (jdk25 / "bin/javac").exists():
        tgz = P.TOOLS_DIR / "dl-jdk25/jdk25.tar.gz"
        download(P.JDK25_URL, tgz, P.JDK25_SHA256)
        target = jdk25.parent
        target.mkdir(parents=True, exist_ok=True)
        with tarfile.open(tgz) as tf:
            for m in tf.getmembers():
                if m.name.startswith("/") or ".." in PurePosixPath(m.name).parts:
                    raise SystemExit(f"unsafe tar member {m.name}")
            tf.extractall(target)
    if not os.environ.get("VINEFLOWER_JAR"):
        download(VINEFLOWER_URL, P.TOOLS_DIR / "vineflower-1.12.0.jar", VINEFLOWER_SHA256)
    for v, home in P.JDKS.items():
        if not (Path(home) / "bin/javac").exists():
            log(f"warning: JDK {v} missing at {home}")


def coursier(args: list[str], json_out: Path) -> list[dict]:
    env = dict(os.environ, COURSIER_CACHE=str(_cache() / "coursier"))
    cmd = [str(P.TOOLS_DIR / "cs"), "fetch", *args, "--json-output-file", str(json_out)]
    log("coursier " + " ".join(args[:6]) + (" ..." if len(args) > 6 else ""))
    r = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=1800)
    if r.returncode != 0:
        raise SystemExit(f"coursier failed ({r.returncode}):\n{r.stderr[-4000:]}")
    return json.loads(json_out.read_text())["dependencies"]


# --- archive handling ------------------------------------------------------


def _safe_name(name: str) -> bool:
    p = PurePosixPath(name)
    return not p.is_absolute() and ".." not in p.parts and "\\" not in name and name.strip() == name


def class_entries(jar_bytes: bytes) -> dict[str, bytes]:
    """Base-version ``.class`` entries (no MR overlays, module-info) of a jar."""
    out: dict[str, bytes] = {}
    with zipfile.ZipFile(io.BytesIO(jar_bytes)) as z:
        for info in z.infolist():
            n = info.filename
            if not n.endswith(".class") or not _safe_name(n):
                continue
            if n.startswith("META-INF/") or n.endswith("module-info.class"):
                continue
            out[n] = z.read(info)
    return out


def extract_sources(zip_bytes: bytes, dest: Path, prefix: str = "") -> int:
    count = 0
    with zipfile.ZipFile(io.BytesIO(zip_bytes)) as z:
        for info in z.infolist():
            n = info.filename
            if not n.endswith(".java") or not _safe_name(n) or info.is_dir():
                continue
            if prefix:
                if not n.startswith(prefix):
                    continue
                n = n[len(prefix):]
            if n.startswith("META-INF/"):
                continue
            target = dest / n
            target.parent.mkdir(parents=True, exist_ok=True)
            if target.exists():
                continue
            target.write_bytes(z.read(info))
            count += 1
    return count


def write_jar(entries: dict[str, bytes], dest: Path) -> None:
    with zipfile.ZipFile(dest, "w", zipfile.ZIP_DEFLATED) as z:
        for name in sorted(entries):
            zi = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
            zi.compress_type = zipfile.ZIP_DEFLATED
            z.writestr(zi, entries[name])


# --- per-project fetch -----------------------------------------------------


def _lock_path(name: str) -> Path:
    return P.LOCK_DIR / f"{name}.json"


def _load_lock(name: str) -> dict | None:
    for p in (_lock_path(name), P.PINS_DIR / f"{name}.json"):
        if p.exists():
            return json.loads(p.read_text())
    return None


def _save_lock(name: str, lock: dict) -> None:
    P.LOCK_DIR.mkdir(parents=True, exist_ok=True)
    tmp = _lock_path(name).with_suffix(".json.new")
    tmp.write_text(json.dumps(lock, indent=1, sort_keys=True) + "\n")
    os.replace(tmp, _lock_path(name))


def _ga(coord: str) -> str:
    return ":".join(coord.split(":")[:2])


def fetch_maven(proj: P.Project, lock: dict | None, work: Path) -> tuple[dict, dict, list[Path]]:
    new_lock: dict = {"artifacts": {}, "deps": []}
    classes: dict[str, bytes] = {}
    sources = work / "sources"
    sources.mkdir()
    for coord in proj.modules:
        for classifier in (None, "sources"):
            key = coord + (":" + classifier if classifier else "")
            want = (lock or {}).get("artifacts", {}).get(key, {}).get("sha256")
            dest = maven_cache_path(coord, classifier)
            digest = download(maven_url(coord, classifier), dest, want)
            new_lock["artifacts"][key] = {"url": maven_url(coord, classifier), "sha256": digest}
            data = dest.read_bytes()
            if classifier is None:
                classes.update(class_entries(data))
            else:
                extract_sources(data, sources)
    if lock and lock.get("deps"):
        coords = [d["coord"] for d in lock["deps"]]
        resolved = coursier(["--intransitive", *coords], work / "coursier.json")
    else:
        args = []
        if proj.bom:
            args += ["--bom", proj.bom]
        resolved = coursier(args + proj.dep_coords, work / "coursier.json")
    self_ga = set(proj.self_artifacts)
    locked = {d["coord"]: d["sha256"] for d in (lock or {}).get("deps", [])}
    dep_files: list[Path] = []
    for d in sorted(resolved, key=lambda x: x["coord"]):
        coord = d["coord"]
        if _ga(coord) in self_ga or not d.get("file"):
            continue
        f = Path(d["file"])
        digest = sha256_file(f)
        if coord in locked and locked[coord] != digest:
            raise SystemExit(f"dependency checksum mismatch: {coord}")
        new_lock["deps"].append({"coord": coord, "sha256": digest})
        dep_files.append(f)
    return new_lock, classes, dep_files


def fetch_ghidra(proj: P.Project, lock: dict | None, work: Path) -> tuple[dict, dict, list[Path]]:
    want = (lock or {}).get("artifacts", {}).get("dist", {}).get("sha256") or proj.dist_sha256
    dest = _cache() / "ghidra" / proj.dist_url.rsplit("/", 1)[1]
    digest = download(proj.dist_url, dest, want)
    new_lock = {"artifacts": {"dist": {"url": proj.dist_url, "sha256": digest}}, "deps": []}
    classes: dict[str, bytes] = {}
    sources = work / "sources"
    sources.mkdir()
    deps_dir = work / "deps"
    deps_dir.mkdir()
    dep_files: list[Path] = []  # written straight into work/deps
    with zipfile.ZipFile(dest) as z:
        root = z.namelist()[0].split("/")[0]
        module_jars = set()
        for mod in proj.modules:
            base = mod.rsplit("/", 1)[1]
            jar = f"{root}/Ghidra/{mod}/lib/{base}.jar"
            src = f"{root}/Ghidra/{mod}/lib/{base}-src.zip"
            module_jars.add(jar)
            classes.update(class_entries(z.read(jar)))
            extract_sources(z.read(src), sources)
            new_lock["artifacts"][mod] = {
                "jar_sha256": hashlib.sha256(z.read(jar)).hexdigest(),
                "src_sha256": hashlib.sha256(z.read(src)).hexdigest(),
            }
        for n in sorted(z.namelist()):
            parts = n.split("/")
            if (
                len(parts) == 6 and parts[1] == "Ghidra" and parts[2] == "Framework"
                and parts[4] == "lib" and n.endswith(".jar") and n not in module_jars
                and _safe_name(n)
            ):
                data = z.read(n)
                out = deps_dir / parts[5]
                out.write_bytes(data)
                new_lock["deps"].append({"coord": n.split("/", 1)[1], "sha256": hashlib.sha256(data).hexdigest()})
    return new_lock, classes, dep_files


def class_stats(classes: dict[str, bytes]) -> dict:
    majors: dict[int, int] = {}
    lines = lvt = code = 0
    for data in classes.values():
        cf = parse_class(data)
        majors[cf.major] = majors.get(cf.major, 0) + 1
        for m in cf.methods:
            if m.code:
                code += 1
                lines += bool(m.code.lines)
                lvt += bool(m.code.locals)
    return {
        "classes": len(classes),
        "class_majors": {str(k): v for k, v in sorted(majors.items())},
        "methods_with_code": code,
        "methods_with_line_numbers": lines,
        "methods_with_local_variable_table": lvt,
    }


def fetch_project(proj: P.Project, force: bool = False) -> Path:
    final = proj.dir
    lock = _load_lock(proj.name)
    if final.exists() and (final / "meta.json").exists() and not force:
        meta = json.loads((final / "meta.json").read_text())
        if lock and meta.get("lock") == lock and sha256_file(final / "classes.jar") == meta.get("classes_jar_sha256"):
            log(f"{proj.name}: up to date ({final})")
            return final
    stamp = time.strftime("%Y%m%d-%H%M%S")
    work = P.DATASET_DIR / f"{proj.name}.new-{stamp}"
    work.mkdir(parents=True)
    if proj.kind == "maven":
        new_lock, classes, dep_files = fetch_maven(proj, lock, work)
    else:
        new_lock, classes, dep_files = fetch_ghidra(proj, lock, work)
    if lock:
        # Locked runs must reproduce the lock exactly.
        if lock != new_lock:
            raise SystemExit(f"{proj.name}: fetched artifacts differ from its pinned lock")
    else:
        _save_lock(proj.name, new_lock)
    write_jar(classes, work / "classes.jar")
    deps = work / "deps"
    deps.mkdir(exist_ok=True)
    for f in dep_files:
        target = deps / f.name
        if not target.exists():
            shutil.copyfile(f, target)
    meta = {
        "project": proj.name,
        "repo": proj.repo,
        "stars": proj.stars,
        "version": proj.version,
        "modules": proj.modules,
        "javac_jdk": proj.javac_jdk,
        "build_jdk_note": proj.build_jdk_note,
        "lock": new_lock,
        "classes_jar_sha256": sha256_file(work / "classes.jar"),
        "deps": sorted(p.name for p in deps.iterdir()),
        "java_sources": sum(1 for _ in (work / "sources").rglob("*.java")),
        **class_stats(classes),
    }
    (work / "meta.json").write_text(json.dumps(meta, indent=1, sort_keys=True) + "\n")
    if final.exists():
        os.rename(final, P.DATASET_DIR / f"{proj.name}.old-{stamp}")
    os.rename(work, final)
    log(f"{proj.name}: {meta['classes']} classes, {meta['java_sources']} sources, {len(meta['deps'])} deps -> {final}")
    return final


def main(names: list[str] | None, force: bool = False) -> None:
    P.DATASET_DIR.mkdir(parents=True, exist_ok=True)
    ensure_tools()
    for proj in P.selected(names):
        fetch_project(proj, force=force)
