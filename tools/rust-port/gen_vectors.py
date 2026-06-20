#!/usr/bin/env python3
"""Generate the golden vectors for the Rust port from the C++ oracle.

This drives the ``decomp_dbg`` console binary as a subprocess (the same pattern
as ``kuna.decompile``), runs the kuna ``golden <kind>`` commands
(``decompiler/cpp/kuna_goldengen.cc``), and captures each vector set via
``openfile write <tmp>`` / ``closefile`` -- the decompiler's bulk-output stream
is redirected to a file while interactive prompts stay on stdout, so the CSVs
are free of prompt/echo noise.

``golden opbehavior`` and ``golden addrsort`` need a loaded architecture (the
float op behaviors hold the program's Translate to map operand sizes to
FloatFormats; addrsort enumerates the program's address spaces), so the script
loads ONE pinned datatest architecture first with ``load test file``:
``decompiler/datatests/floatprint.xml`` (x86:LE:64:default:gcc).  Regeneration
must keep that pin or the addrsort space set (and nothing else) changes.
``golden float`` needs no program (it constructs FloatFormat(4)/(8) directly).

Outputs (committed under tests/golden/vectors/):
    opbehavior.csv  OpBehavior::evaluateUnary/evaluateBinary cells
    float.csv       FloatFormat encode/decode/op/print cells
    addrsort.csv    Address comparator decisions
    README.md       format docs + exact regeneration command + C++ git rev

CLI:
    python tools/rust-port/gen_vectors.py [--out-dir DIR] [--decomp-dbg PATH]
                                          [--sleighpath DIR] [--arch-test XML]
                                          [--timeout S]
"""
import argparse
import os
import subprocess
import sys
import tempfile

try:
    from kuna import paths
except ImportError:  # run from a checkout without the venv: repo root on sys.path
    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    from kuna import paths


class VectorGenError(Exception):
    """Golden-vector generation failed."""


# kind -> (output file, minimum plausible line count)
VECTOR_KINDS = [
    ("opbehavior", "opbehavior.csv", 1000),
    ("float", "float.csv", 500),
    ("addrsort", "addrsort.csv", 100),
]

# The pinned architecture-bearing datatest (relative to the repo root).
DEFAULT_ARCH_TEST = os.path.join("decompiler", "datatests", "floatprint.xml")


def generate(out_dir=None, decomp_dbg=None, sleighpath=None, arch_test=None, timeout=300):
    """Run the golden commands and write the vector CSVs plus README.md.

    Returns the list of paths written.
    """
    root = paths.repo_root()
    out_dir = (
        os.path.join(str(root), "tests", "golden", "vectors")
        if out_dir is None
        else os.path.abspath(str(out_dir))
    )
    bin_path = paths.decomp_dbg() if decomp_dbg is None else os.path.abspath(str(decomp_dbg))
    if not os.path.exists(str(bin_path)):
        raise VectorGenError(
            "decomp_dbg not built at %s -- run `make binaries`" % bin_path
        )
    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))
    arch_test = (
        os.path.join(str(root), DEFAULT_ARCH_TEST)
        if arch_test is None
        else os.path.abspath(str(arch_test))
    )
    if not os.path.exists(arch_test):
        raise VectorGenError("architecture datatest not found: " + arch_test)

    tmp_paths = {}
    for kind, _, _ in VECTOR_KINDS:
        tmp = tempfile.NamedTemporaryFile(prefix="kuna_golden_%s_" % kind, suffix=".csv", delete=False)
        tmp_paths[kind] = tmp.name
        tmp.close()

    try:
        lines = ["load test file %s" % arch_test]
        for kind, _, _ in VECTOR_KINDS:
            lines.append("openfile write %s" % tmp_paths[kind])
            lines.append("golden %s" % kind)
            lines.append("closefile")
        lines.append("quit")
        script = "\n".join(lines) + "\n"

        env = dict(os.environ)
        # SLEIGHHOME is the load-bearing spec-discovery mechanism (see
        # kuna/decompile.py): decomp_dbg scans it recursively for the
        # Ghidra/Processors/*/data/languages layout of the specs/ root.
        env["SLEIGHHOME"] = specs
        try:
            proc = subprocess.run(
                [str(bin_path), "-s", specs],
                input=script,
                capture_output=True,
                text=True,
                env=env,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            raise VectorGenError("decomp_dbg timed out after %ss" % timeout)

        combined = (proc.stdout or "") + "\n" + (proc.stderr or "")
        if proc.returncode != 0:
            raise VectorGenError(
                "decomp_dbg exited %d; output:\n%s" % (proc.returncode, combined.strip()[:2000])
            )
        for marker in ("Unable to parse command", "ERROR", "Low-level ERROR"):
            if marker in combined:
                raise VectorGenError(
                    "decomp_dbg reported an error; output:\n%s" % combined.strip()[:2000]
                )

        os.makedirs(out_dir, exist_ok=True)
        written = []
        for kind, fname, min_lines in VECTOR_KINDS:
            with open(tmp_paths[kind], "r") as fh:
                text = fh.read()
            nlines = text.count("\n")
            if nlines < min_lines:
                raise VectorGenError(
                    "golden %s produced only %d lines (expected >= %d); output:\n%s"
                    % (kind, nlines, min_lines, combined.strip()[:2000])
                )
            if not text.startswith("# golden %s" % kind):
                raise VectorGenError(
                    "golden %s capture is missing its header (got %r...)" % (kind, text[:60])
                )
            out_path = os.path.join(out_dir, fname)
            with open(out_path, "w") as fh:
                fh.write(text)
            written.append(out_path)

        readme = os.path.join(out_dir, "README.md")
        with open(readme, "w") as fh:
            fh.write(_readme_text(arch_test))
        written.append(readme)
        return written
    finally:
        for tmp in tmp_paths.values():
            try:
                os.unlink(tmp)
            except OSError:
                pass


def _git_rev():
    """Short rev of the C++ tree the vectors were generated from ('' if unknown)."""
    try:
        out = subprocess.run(
            ["git", "-C", str(paths.repo_root()), "rev-parse", "--short", "HEAD"],
            capture_output=True,
            text=True,
            timeout=10,
        )
    except OSError:
        return ""
    return out.stdout.strip() if out.returncode == 0 else ""


def _readme_text(arch_test):
    rel_arch = os.path.relpath(arch_test, str(paths.repo_root()))
    rev = _git_rev() or "unknown"
    return """\
# Golden vectors (C++ oracle -> Rust port)

Deterministic low-level semantics vectors emitted by the C++ decompiler's
`golden <kind>` console commands (`decompiler/cpp/kuna_goldengen.cc`).  The
Rust port must reproduce these byte-for-byte (same CSV rows from its own
OpBehavior / FloatFormat / Address implementations).

Generated at C++ tree rev `%s` against the pinned architecture datatest
`%s` (x86:LE:64:default:gcc).

## Regeneration

```bash
make binaries        # rebuild decomp_dbg after any C++ change
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_vectors.py
```

(Equivalently at the console: `load test file %s`,
then `golden opbehavior` / `golden float` / `golden addrsort` under
`openfile write <path>` / `closefile`.)

## Files

### opbehavior.csv

One row per `OpBehavior` evaluation cell, in opcode enum order, then size
combination, then input index.

- binary rows: `op,sizein,sizeout,in0,in1,result` over sizein in {1,2,4,8}
  with sizeout in {sizein,1}, inputs the fixed 8-value edge list
  (0, 1, 2, signbit-1, signbit, all-ones, 0xdeadbeef..., 0x12345678...
  masked to size) crossed with itself.
- unary rows: `op,sizein,sizeout,in0,result` over all 16 (sizein,sizeout)
  pairs and the same edge list.
- `result` values: hex output, `ERR` (the evaluation throws -- EvaluationError
  or the base-class "emulation unimplemented" LowlevelError), or `TRAP`
  (the INT64_MIN / -1 cell of INT_SDIV / INT_SREM, which would raise SIGFPE
  in the C++ host's signed division and is never evaluated).
- Special (non-evaluatable) opcodes -- branches, calls, LOAD/STORE,
  MULTIEQUAL, INDIRECT, CAST, SEGMENTOP, CPOOLREF, NEW -- are skipped.
- Row names are the canonical enum names: `ZPULL`/`SPULL` are named directly
  because upstream's `get_opname` table is stale there (index 71 still reads
  "EXTRACT"; index 74 is past the end of the table).
- Requires a loaded architecture: the CPUI_FLOAT_* behaviors look up
  FloatFormat by operand size through the program's Translate.  Sizes
  without a registered format (1 and 2 here) fall back to ERR.

### float.csv

`FloatFormat(4)` and `FloatFormat(8)` (default IEEE 754 layouts; no program
needed) over a fixed 15-encoding list (+-0, +-1, 0.5, pi, 1e-10, 1e10 via
getEncoding; subnormal min/max, min normal, max finite, +-inf, quiet NaN as
raw bit patterns).

- `dec,size,enc,class,decimal`: getClass + the iostream default-precision
  rendering of getHostFloat (printc relies on iostream formatting).
- `<unop>,size,a,res` for neg, abs, sqrt, ceil, floor, round, nan.
- `<binop>,size,a,b,res` for add, sub, mult, div, equal, less, lessequal
  over the full encoding cross product.
- `trunc,size,sizeout,a,res` for sizeout in {1,2,4,8}.
- `int2float,sizein,size,a,res` for sizein in {1,2,4,8} over the opbehavior
  edge-input list.
- `float2float,sizein,sizeout,a,res` for 4->8 and 8->4.

### addrsort.csv

`spaceA,offA,spaceB,offB,less,equal,lessequal` for every ordered pair of
Address(space, offset) over all address spaces of the pinned architecture
(space-index order) x offsets {0, 1, 0x1000, 0xffffffff}.  The space set is
a property of the loaded program -- regeneration must keep the pinned
datatest above.

## Determinism caveats

The vectors are deterministic for a fixed binary on a fixed host (the gate:
two consecutive generator runs diff empty).  Cross-platform reproduction
notes for the Rust port:

- `trunc` of NaN/infinity/out-of-range values goes through a host
  `(int64_t)double` cast in C++ (x86: 0x8000000000000000); Rust's saturating
  `as` casts differ and must special-case these cells.
- NaN-producing arithmetic re-encodes through the host's double pipeline;
  the C++ side canonicalizes NaNs via getNaNEncoding, so payloads are stable,
  but the sign of a NaN *result* follows host semantics.
- The decimal column of `dec` rows is C++ iostream `operator<<(double)` with
  default (6 significant digit) precision, e.g. `3.14159`, `1e+10`, `-inf`,
  `nan`.
""" % (rev, rel_arch, rel_arch)


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python tools/rust-port/gen_vectors.py",
        description="Generate the golden vectors for the Rust port from the C++ decompiler.",
    )
    p.add_argument("--out-dir", default=None,
                   help="output directory (default: <repo>/tests/golden/vectors)")
    p.add_argument("--decomp-dbg", default=None, help="path to the decomp_dbg binary")
    p.add_argument("--sleighpath", default=None, help="SLEIGH specs root (default: <repo>/specs)")
    p.add_argument("--arch-test", default=None,
                   help="architecture-bearing datatest XML to pin (default: %s)" % DEFAULT_ARCH_TEST)
    p.add_argument("--timeout", type=float, default=300, help="subprocess timeout in seconds")
    args = p.parse_args(argv)

    try:
        written = generate(
            out_dir=args.out_dir,
            decomp_dbg=args.decomp_dbg,
            sleighpath=args.sleighpath,
            arch_test=args.arch_test,
            timeout=args.timeout,
        )
    except VectorGenError as e:
        print("error: %s" % e, file=sys.stderr)
        return 1
    for path in written:
        with open(path, "r") as fh:
            nlines = fh.read().count("\n")
        print("wrote %s (%d lines)" % (path, nlines))
    return 0


if __name__ == "__main__":
    sys.exit(main())
