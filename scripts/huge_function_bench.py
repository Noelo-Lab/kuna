"""Generate an x86-64 ELF stress fixture and optionally measure Kuna on it."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import signal
import subprocess
import time
from pathlib import Path


def assembly(blocks: int) -> str:
    lines = [".text", ".globl stress", ".type stress,@function", "stress:",
             "mov %edi, %eax"]
    for i in range(blocks):
        lines.extend([f"test ${1 << (i % 16)}, %eax", f"jz .Lskip{i}",
                      f"imul ${3 + 2 * (i % 61)}, %eax, %eax",
                      f"add ${i % 32749 + 1}, %eax", f".Lskip{i}:",
                      f"xor ${i % 65521 + 1}, %eax"])
    lines.extend(["ret", ".size stress, .-stress", ".globl _start", "_start:",
                  "mov $1, %edi", "call stress", "mov %eax, %edi",
                  "mov $60, %eax", "syscall",
                  '.section .note.GNU-stack,"",@progbits'])
    return "\n".join(lines) + "\n"


def escaped_locals(blocks: int) -> str:
    lines = ["struct S { int a; long b; char c[24]; };",
             "__attribute__((noinline)) int cb(void *p) { return *(int *)p; }",
             "__attribute__((noinline)) int reg(const char *n, void *p, int i, int (*f)(void *)) {",
             "return f(p) + i + *n; }", "int stress(void) { int r = 0;"]
    for i in range(blocks):
        lines.append(f"struct S s{i}; s{i}.a = {i}; s{i}.b = r; "
                     f'r += reg("n", &s{i}, {i}, cb);')
    lines.extend(["return r; }", "int after(int x) { return x * 3 + 1; }"])
    return "\n".join(lines) + "\n"


def measure(cmd: list[str], stem: Path, seconds: float) -> dict:
    """Measure one process tree; enforce a wall deadline outside Kuna."""
    start = time.monotonic()
    timed_out = False
    with stem.with_suffix(".decompile.json").open("w") as output, \
            stem.with_suffix(".stderr").open("w") as error:
        proc = subprocess.Popen(cmd, stdout=output, stderr=error, start_new_session=True)
        try:
            while True:
                pid, status, usage = os.wait4(proc.pid, os.WNOHANG)
                if pid:
                    proc.returncode = os.waitstatus_to_exitcode(status)
                    break
                if time.monotonic() - start >= seconds:
                    timed_out = True
                    os.killpg(proc.pid, signal.SIGKILL)
                    _, status, usage = os.wait4(proc.pid, 0)
                    proc.returncode = os.waitstatus_to_exitcode(status)
                    break
                time.sleep(0.05)
        finally:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            if proc.returncode is None:
                proc.wait()
    return {"exit_code": proc.returncode, "wall_seconds": round(time.monotonic() - start, 4),
            "cpu_seconds": round(usage.ru_utime + usage.ru_stime, 4),
            "peak_rss_kib": usage.ru_maxrss, "timed_out": timed_out, "command": cmd}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--blocks", type=int, nargs="+", default=[100, 1000, 10000])
    parser.add_argument("--shape", choices=["branches", "escaped-locals"], default="branches")
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--cc", default="gcc")
    parser.add_argument("--kuna", help="Optional path to a built Kuna binary")
    parser.add_argument("--seconds", type=int, default=30)
    parser.add_argument("--memory-mib", type=int, default=1024)
    parser.add_argument("--option", nargs=2, action="append", default=[], metavar=("NAME", "VALUE"))
    args = parser.parse_args()
    if min(args.blocks) < 1 or args.seconds < 1 or args.memory_mib < 1:
        parser.error("blocks, seconds and memory-mib must be positive")
    kuna = shutil.which(args.kuna) if args.kuna else None
    if args.kuna and not kuna:
        parser.error(f"Kuna executable not found: {args.kuna}")
    args.out.mkdir(parents=True, exist_ok=True)
    metadata = {"platform": platform.platform(), "machine": platform.machine(),
                "compiler": subprocess.check_output([args.cc, "--version"], text=True).splitlines()[0]}
    if kuna:
        metadata["kuna_version"] = subprocess.check_output([kuna, "--version"], text=True).strip()
        metadata["kuna_sha256"] = hashlib.sha256(Path(kuna).read_bytes()).hexdigest()
    (args.out / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    records = []
    for blocks in args.blocks:
        stem = args.out.resolve() / f"{args.shape}-{blocks}"
        source = stem.with_suffix(".S" if args.shape == "branches" else ".input.c")
        source.write_text(assembly(blocks) if args.shape == "branches" else escaped_locals(blocks))
        build_cmd = [args.cc, "-O0", "-fno-stack-protector", "-fstack-reuse=none",
                     "-nostdlib", "-no-pie", "-Wl,--build-id=none", "-Wl,-e,stress",
                     str(source), "-o", str(stem)]
        subprocess.run(build_cmd, check=True)
        symbols = subprocess.check_output(["nm", "-S", str(stem)], text=True)
        symbol = next(line.split() for line in symbols.splitlines()
                      if line.split()[-1] == "stress")
        record = {"shape": args.shape, "blocks": blocks, "function_bytes": int(symbol[1], 16),
                  "binary": str(stem), "build_command": build_cmd,
                  "binary_sha256": hashlib.sha256(stem.read_bytes()).hexdigest()}
        if kuna:
            cmd = [kuna, "decompile-all", str(stem), "--functions", "stress", "--json", "--jobs", "1",
                   "--max-fn-seconds", str(args.seconds),
                   "--max-fn-memory", str(args.memory_mib)]
            for name, value in args.option:
                cmd.extend(["--option", name, value])
            record.update(measure(cmd, stem, args.seconds + 10))
            try:
                payload = json.loads(stem.with_suffix(".decompile.json").read_text())
                functions = payload.get("functions", [])
                record["functions"] = [{"name": f.get("name"), "error": f.get("error"),
                                        "code_bytes": len((f.get("code") or "").encode())}
                                       for f in functions]
                record["error"] = payload.get("error")
                record["warnings"] = payload.get("warnings", [])
            except (ValueError, AttributeError, TypeError):
                record["error"] = "No valid decompilation JSON; inspect diagnostics"
        records.append(record)
        (args.out / "results.json").write_text(json.dumps(records, indent=2) + "\n")
        print(json.dumps(record), flush=True)


if __name__ == "__main__":
    main()
