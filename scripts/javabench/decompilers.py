"""Pluggable decompiler adapters.

An adapter turns ``(input.jar, outdir)`` into ``.java`` files under ``outdir``;
the harness discovers classes by PARSING those files (binary names come from
``package`` + declarations), so the directory layout is not load-bearing.

Contract for kuna (``KUNA_BIN``)::

    $KUNA_BIN decompile-project <input.jar> -o <outdir>

writes one ``.java`` per top-level class at ``<outdir>/<package/path>/<Class>.java``
and exits 0. A non-zero exit or zero ``.java`` files means the binary rejected
the input: the run is recorded as ``skipped`` with the reason, and not scored.
"""

from __future__ import annotations

import os
import re
import shutil
import subprocess
import time
import zipfile
from dataclasses import dataclass
from pathlib import Path

from . import projects as P

def vineflower_jar() -> Path:
    """``$VINEFLOWER_JAR``, else the release jar ``fetch`` puts in the workspace's tools."""
    return Path(os.environ.get("VINEFLOWER_JAR") or P.TOOLS_DIR / "vineflower-1.12.0.jar")


@dataclass
class Adapter:
    name: str
    # per-method "I gave up" marker inside a method body
    failure_regex: str = r"\$VF: Couldn't be decompiled"

    def available(self) -> tuple[bool, str]:
        return True, ""

    def command(self, input_jar: Path, outdir: Path, jobs: int) -> list[str] | None:
        raise NotImplementedError

    def version(self) -> str:
        return ""

    def materialize(self, proj: P.Project, input_jar: Path, outdir: Path) -> None:
        """Adapters that do not shell out (the source oracle) override this."""
        raise NotImplementedError


class Vineflower(Adapter):
    def __init__(self):
        super().__init__("vineflower")

    def available(self):
        return (vineflower_jar().exists(), f"missing {vineflower_jar()}")

    def version(self) -> str:
        return vineflower_jar().name

    def command(self, input_jar, outdir, jobs):
        java = os.path.join(P.JDKS[25], "bin/java") if Path(P.JDKS[25], "bin/java").exists() else "java"
        # Default decompilation options; only the output layout (--folder) and the
        # worker-thread count (output-neutral; the default is one per core) are set.
        # JVM flags keep temporary files (hsperfdata, tmp) inside the run directory.
        tmp = outdir.parent / "jvm-tmp"
        tmp.mkdir(parents=True, exist_ok=True)
        return [java, "-Xmx16g", "-XX:-UsePerfData", f"-Djava.io.tmpdir={tmp}", "-jar", str(vineflower_jar()),
                f"-thr={jobs}", "--folder", str(input_jar), str(outdir)]


class Kuna(Adapter):
    def __init__(self, bin_path: str | None = None):
        super().__init__("kuna", failure_regex=r"\$VF: Couldn't be decompiled|kuna: (?:could not|couldn't) decompile")
        self.bin = bin_path or os.environ.get("KUNA_BIN")

    def available(self):
        if not self.bin:
            return False, "KUNA_BIN not set (use --kuna-bin or export KUNA_BIN)"
        if not (Path(self.bin).exists() and os.access(self.bin, os.X_OK)):
            return False, f"KUNA_BIN={self.bin} is not an executable file"
        return True, ""

    def version(self) -> str:
        try:
            r = subprocess.run([self.bin, "--version"], capture_output=True, text=True, timeout=30)
            return (r.stdout or r.stderr).strip().splitlines()[0] if (r.stdout or r.stderr) else ""
        except Exception:
            return ""

    def command(self, input_jar, outdir, jobs):
        return [self.bin, "decompile-project", str(input_jar), "-o", str(outdir)]


class SourceOracle(Adapter):
    """Pseudo-decompiler that emits the ORIGINAL source files: the ceiling /
    harness self-check (GED 0, types 100%, byte-match = toolchain fidelity)."""

    def __init__(self):
        super().__init__("source", failure_regex=r"(?!x)x")

    def materialize(self, proj, input_jar, outdir):
        from .index import source_path_of
        from .classfile import parse_class

        z = zipfile.ZipFile(input_jar)
        rels = set()
        for n in z.namelist():
            if n.endswith(".class"):
                rels.add(source_path_of(parse_class(z.read(n))))
        for rel in sorted(rels):
            src = proj.dir / "sources" / rel
            if src.exists():
                dst = outdir / rel
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(src, dst)


def get_adapter(name: str, kuna_bin: str | None = None) -> Adapter:
    if name == "vineflower":
        return Vineflower()
    if name == "kuna":
        return Kuna(kuna_bin)
    if name == "source":
        return SourceOracle()
    raise SystemExit(f"unknown decompiler {name!r} (vineflower, kuna, source)")


def run_measured(cmd: list[str], log_path: Path, timeout: int) -> dict:
    """Run ``cmd``; return exit code, wall seconds and peak RSS (MB).

    Peak RSS is the max of the kernel's ru_maxrss for the child (wait4) and a
    0.2 s sampled sum over the whole process tree (psutil), so self-spawning
    tools are measured too.
    """
    try:
        import psutil
    except ImportError:  # pragma: no cover
        psutil = None
    t0 = time.monotonic()
    with open(log_path, "wb") as log:
        proc = subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT)
        peak_tree = 0
        timed_out = False
        status = None
        ru = None
        while True:
            pid, st, r = os.wait4(proc.pid, os.WNOHANG)
            if pid != 0:
                status, ru = st, r
                break
            if psutil is not None:
                try:
                    p = psutil.Process(proc.pid)
                    rss = p.memory_info().rss
                    for c in p.children(recursive=True):
                        try:
                            rss += c.memory_info().rss
                        except psutil.Error:
                            pass
                    peak_tree = max(peak_tree, rss)
                except psutil.Error:
                    pass
            if time.monotonic() - t0 > timeout:
                timed_out = True
                proc.kill()
                _, status, ru = os.wait4(proc.pid, 0)
                break
            time.sleep(0.2)
    wall = time.monotonic() - t0
    rc = os.waitstatus_to_exitcode(status) if status is not None else -1
    proc.returncode = rc
    maxrss_mb = (ru.ru_maxrss / 1024.0) if ru is not None else 0.0
    return {
        "exit_code": rc,
        "timed_out": timed_out,
        "wall_seconds": round(wall, 2),
        "peak_rss_mb": round(max(maxrss_mb, peak_tree / 2**20), 1),
        "cpu_seconds": round((ru.ru_utime + ru.ru_stime), 2) if ru is not None else None,
    }


_JAVA_FILE = re.compile(r"\.java$")


def count_java(outdir: Path) -> int:
    return sum(1 for _ in outdir.rglob("*.java"))
