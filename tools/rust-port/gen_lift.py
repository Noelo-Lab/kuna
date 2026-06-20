#!/usr/bin/env python3
"""Generate per-instruction p-code lift fixtures from the C++ oracle.

This drives the ``decomp_dbg`` console binary as a subprocess and replays a
pinned, architecture-diverse subset of the XML datatest corpus through the
kuna ``golden lift`` / ``golden liftctx`` commands
(``decompiler/cpp/kuna_goldengen.cc``).  The replay mechanics follow
``kuna.goldens``: the raw ``<binaryimage>`` element is extracted byte-exact
from the corpus XML into a temp file, ``load file <tmp>`` recognizes it as an
XML program (resolving the SLEIGH language from its ``arch`` attribute and
auto-reading the loader symbols), and every dump is captured through
``openfile write <tmp>`` / ``closefile`` so prompt noise never reaches the
fixture.

Each fixture (``tests/golden/vectors/lift/<xml-stem>.txt``) contains:
  - a ``#`` comment header written by this script (corpus path, sla path
    relative to specs/, lift points);
  - one ``golden liftctx <first-point>`` dump (language id, sla file,
    endianness, the address-space table, and every context variable
    name=value in effect at the first lift point);
  - one ``golden lift <point> N`` dump per lift point (N capped at
    ``--cap``, default 80): per instruction a ``insn <space>:<offset>
    <length>`` header plus one line per raw p-code op; a decode failure ends
    the dump with a ``lifterror`` line, which is part of the fixture.

Lift points are every ``<symbol>`` of the binary image (document order,
deduplicated); images without symbols fall back to their ``<bytechunk>``
start addresses.

Address syntax wrinkle: XML ``offset`` attributes are BYTE offsets
(AddrSpace::decodeAttributes), but the console's address parser multiplies a
typed offset by the space's wordsize (AddrSpace::read).  The script therefore
runs a bare ``golden liftctx`` first, reads the ``space ... wordsize=`` table,
and converts byte offsets to word offsets before issuing commands.  Fixture
content itself always shows byte offsets (the C++ side prints raw offsets).

Two consecutive runs must produce byte-identical fixtures (the determinism
gate):

    python tools/rust-port/gen_lift.py
    python tools/rust-port/gen_lift.py --out-dir /tmp/lift2
    diff -r tests/golden/vectors/lift /tmp/lift2   # only README timestamps absent: empty

CLI:
    python tools/rust-port/gen_lift.py [--out-dir DIR] [--decomp-dbg PATH]
                                       [--sleighpath DIR] [--cap N] [--timeout S]
"""
import argparse
import os
import re
import subprocess
import sys
import tempfile

try:
    from kuna import paths
except ImportError:  # run from a checkout without the venv: repo root on sys.path
    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    from kuna import paths


class LiftGenError(Exception):
    """Lift-fixture generation failed."""


# The pinned corpus subset: one test per distinct SLEIGH language id, spanning
# every processor family the corpus offers (paths relative to the repo root).
# Corpus inventory at pin time (language id -> available tests) is documented
# in tests/golden/vectors/lift/README.md.
FIXTURES = [
    ("decompiler/datatests/floatprint.xml",            "x86:LE:64:default:gcc"),
    ("decompiler/datatests/promotecompare.xml",        "x86:LE:32:default:gcc"),
    ("tests/stages/gh7139-x86-16-disp16-signed.xml",   "x86:LE:16:Real Mode"),
    ("decompiler/datatests/condexesub.xml",            "ARM:LE:32:v8:default"),
    ("decompiler/datatests/ccmp.xml",                  "AARCH64:LE:64:v8A:default"),
    ("decompiler/datatests/gp.xml",                    "MIPS:BE:32:default:default"),
    ("decompiler/datatests/lzcount.xml",               "PowerPC:BE:32:default:default"),
    ("tests/stages/gh6990-returnpair.xml",             "sparc:BE:32:default:default"),
    ("tests/stages/gh8817-v850indbranch.xml",          "V850:LE:32:default"),
    ("decompiler/datatests/boolless.xml",              "8051:BE:16:default:default"),
    ("tests/stages/gh8844-avr-rjmpwrap.xml",           "avr8:LE:16:default"),
    ("decompiler/datatests/readvolatile.xml",          "68000:BE:32:MC68020:default"),
    ("tests/stages/gh9203-condexe-loopcopy.xml",       "RISCV:LE:64:default"),
    ("tests/stages/gh8913-addcarrychain.xml",          "6502:LE:16:default"),
    ("tests/stages/gh9001-hcs12brn.xml",               "HCS12:BE:24:default"),
    ("decompiler/datatests/skipnext2.xml",             "Toy:BE:32:builder.align2:default"),
]

_BINARYIMAGE = re.compile(r"<binaryimage\b.*?</binaryimage>", re.S)
_SYMBOL = re.compile(r"<symbol\b[^>]*>")
_BYTECHUNK = re.compile(r"<bytechunk\b[^>]*>")
_ATTR_SPACE = re.compile(r'\bspace="([^"]*)"')
_ATTR_OFFSET = re.compile(r'\boffset="([^"]*)"')
_ATTR_NAME = re.compile(r'\bname="([^"]*)"')
_SPACE_LINE = re.compile(r"^space (\S+) index=\d+ size=\d+ wordsize=(\d+)", re.M)
_SLAFILE_LINE = re.compile(r"^slafile (\S+)$", re.M)


def _parse_image(xml_path):
    """Return (raw <binaryimage> text, [(space, byte-offset, label), ...]).

    The image is a byte-exact substring (never re-serialized): the bytechunk
    text is load-bearing.  Lift points are the image's <symbol> elements in
    document order (deduplicated by address); images without symbols fall
    back to <bytechunk> start addresses.
    """
    src = open(str(xml_path)).read()
    m = _BINARYIMAGE.search(src)
    if m is None:
        raise LiftGenError("no <binaryimage> element in %s" % xml_path)
    image = m.group(0)

    def attrs(tag, want_name):
        sp = _ATTR_SPACE.search(tag)
        off = _ATTR_OFFSET.search(tag)
        if sp is None or off is None:
            raise LiftGenError("element missing space/offset in %s: %s" % (xml_path, tag))
        nm = _ATTR_NAME.search(tag) if want_name else None
        return sp.group(1), int(off.group(1), 0), (nm.group(1) if nm else "")

    points = [attrs(t, True) for t in _SYMBOL.findall(image)]
    if not points:
        points = [(sp, off, "(bytechunk)") for sp, off, _ in
                  (attrs(t, False) for t in _BYTECHUNK.findall(image))]
    if not points:
        raise LiftGenError("no <symbol> or <bytechunk> in %s" % xml_path)
    seen, dedup = set(), []
    for sp, off, nm in points:
        if (sp, off) in seen:
            continue
        seen.add((sp, off))
        dedup.append((sp, off, nm))
    return image, dedup


def _run_console(bin_path, specs, script, timeout):
    env = dict(os.environ)
    # SLEIGHHOME is the load-bearing spec-discovery mechanism (see
    # kuna/decompile.py): decomp_dbg scans it recursively for the
    # Ghidra/Processors/*/data/languages layout of the specs/ root.
    env["SLEIGHHOME"] = specs
    try:
        proc = subprocess.run(
            [str(bin_path), "-s", specs],
            input=script, capture_output=True, text=True, env=env, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        raise LiftGenError("decomp_dbg timed out after %ss" % timeout)
    combined = (proc.stdout or "") + "\n" + (proc.stderr or "")
    if proc.returncode != 0:
        raise LiftGenError("decomp_dbg exited %d; output:\n%s"
                           % (proc.returncode, combined.strip()[:2000]))
    if "successfully loaded" not in combined:
        raise LiftGenError("program load failed; output:\n%s" % combined.strip()[:2000])
    for marker in ("ERROR", "error:", "Execution error", "Parse error",
                   "Unable to parse command"):
        if marker in combined:
            raise LiftGenError("decomp_dbg reported an error; output:\n%s"
                               % combined.strip()[:2000])
    return proc


def _console_session(bin_path, specs, image_path, commands, timeout):
    """Run one decomp_dbg session capturing ``commands`` via openfile write."""
    fd, cap_path = tempfile.mkstemp(prefix="kuna_lift_", suffix=".txt")
    os.close(fd)
    try:
        lines = ["load file %s" % image_path, "openfile write %s" % cap_path]
        lines.extend(commands)
        lines.extend(["closefile", "quit"])
        _run_console(bin_path, specs, "\n".join(lines) + "\n", timeout)
        with open(cap_path) as fh:
            return fh.read()
    finally:
        try:
            os.unlink(cap_path)
        except OSError:
            pass


def _machaddr(space, byte_off, wordsizes):
    """Render a parse_machaddr-form address, converting bytes -> words."""
    if space not in wordsizes:
        raise LiftGenError("space %r not in the liftctx space table %s"
                           % (space, sorted(wordsizes)))
    ws = wordsizes[space]
    if byte_off % ws != 0:
        raise LiftGenError("byte offset 0x%x not word-aligned (wordsize %d) in space %s"
                           % (byte_off, ws, space))
    return "[%s,0x%x]" % (space, byte_off // ws)


def _sla_relpath(slafile, specs):
    """Locate the .sla under specs/ and return its specs-relative path."""
    hits = []
    for root, _, files in os.walk(specs):
        if slafile in files:
            hits.append(os.path.relpath(os.path.join(root, slafile), specs))
    if len(hits) != 1:
        raise LiftGenError("expected exactly one %s under %s, found %s"
                           % (slafile, specs, hits or "none"))
    return hits[0]


def generate_fixture(rel_xml, bin_path, specs, root, cap, timeout):
    """Generate one fixture; returns (fixture text, stats dict)."""
    xml_path = os.path.join(root, rel_xml)
    if not os.path.exists(xml_path):
        raise LiftGenError("corpus test not found: " + xml_path)
    image, points = _parse_image(xml_path)

    fd, image_path = tempfile.mkstemp(prefix="kuna_img_", suffix=".xml")
    with os.fdopen(fd, "w") as fh:
        fh.write(image)
    try:
        # Pass 1: bare liftctx for the space table (byte->word conversion).
        probe = _console_session(bin_path, specs, image_path, ["golden liftctx"], timeout)
        if not probe.startswith("# golden liftctx"):
            raise LiftGenError("liftctx probe capture is malformed (got %r...)" % probe[:60])
        wordsizes = {m.group(1): int(m.group(2)) for m in _SPACE_LINE.finditer(probe)}
        slafile = _SLAFILE_LINE.search(probe)
        if slafile is None:
            raise LiftGenError("liftctx probe has no slafile line for %s" % rel_xml)

        # Pass 2: the fixture body -- liftctx at the first lift point (context
        # values in effect where decoding starts), then one lift per point.
        commands = ["golden liftctx %s" % _machaddr(points[0][0], points[0][1], wordsizes)]
        for sp, off, _ in points:
            commands.append("golden lift %s %d" % (_machaddr(sp, off, wordsizes), cap))
        body = _console_session(bin_path, specs, image_path, commands, timeout)
    finally:
        try:
            os.unlink(image_path)
        except OSError:
            pass

    if not body.startswith("# golden liftctx"):
        raise LiftGenError("fixture capture is malformed (got %r...)" % body[:60])
    n_lifts = body.count("\n# golden lift ")
    if n_lifts != len(points):
        raise LiftGenError("expected %d 'golden lift' dumps in %s, captured %d"
                           % (len(points), rel_xml, n_lifts))

    sla_rel = _sla_relpath(slafile.group(1), specs)
    header = [
        "# kuna golden lift fixture (per-instruction raw p-code from the C++ oracle)",
        "# corpus: %s" % rel_xml.replace(os.sep, "/"),
        "# sla: %s (relative to specs/)" % sla_rel.replace(os.sep, "/"),
        "# cap: %d instructions per lift point" % cap,
        "# lift-points (space:byte-offset name): %s" % " ".join(
            "%s:0x%x%s" % (sp, off, ("=" + nm) if nm and nm != "(bytechunk)" else "")
            for sp, off, nm in points),
        "# regenerate: python tools/rust-port/gen_lift.py  (see tests/golden/vectors/lift/README.md)",
        "",
    ]
    stats = {
        "instructions": body.count("\ninsn ") + body.startswith("insn "),
        "ops": len(re.findall(r"^  \S", body, re.M)),
        "lifterrors": body.count("\nlifterror "),
        "points": len(points),
    }
    return "\n".join(header) + body, stats


def generate(out_dir=None, decomp_dbg=None, sleighpath=None, cap=80, timeout=300,
             write_readme=True):
    """Generate every pinned fixture; returns [(path, stats)]."""
    root = str(paths.repo_root())
    out_dir = (
        os.path.join(root, "tests", "golden", "vectors", "lift")
        if out_dir is None
        else os.path.abspath(str(out_dir))
    )
    bin_path = paths.decomp_dbg() if decomp_dbg is None else os.path.abspath(str(decomp_dbg))
    if not os.path.exists(str(bin_path)):
        raise LiftGenError("decomp_dbg not built at %s -- run `make binaries`" % bin_path)
    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))

    os.makedirs(out_dir, exist_ok=True)
    written = []
    for rel_xml, langid in FIXTURES:
        stem = os.path.splitext(os.path.basename(rel_xml))[0]
        text, stats = generate_fixture(rel_xml, bin_path, specs, root, cap, timeout)
        out_path = os.path.join(out_dir, stem + ".txt")
        with open(out_path, "w") as fh:
            fh.write(text)
        stats["langid"] = langid
        written.append((out_path, stats))
    if write_readme:
        readme = os.path.join(out_dir, "README.md")
        with open(readme, "w") as fh:
            fh.write(_readme_text(cap))
        written.append((readme, None))
    return written


def _git_rev():
    """Short rev of the C++ tree the fixtures were generated from ('' if unknown)."""
    try:
        out = subprocess.run(
            ["git", "-C", str(paths.repo_root()), "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, timeout=10,
        )
    except OSError:
        return ""
    return out.stdout.strip() if out.returncode == 0 else ""


def _readme_text(cap):
    rev = _git_rev() or "unknown"
    pins = "\n".join("| `%s` | %s |" % (p, l) for p, l in FIXTURES)
    return """\
# Golden lift fixtures (C++ oracle -> Rust SLEIGH runtime)

Per-instruction raw p-code lifts emitted by the C++ decompiler's
`golden lift` / `golden liftctx` console commands
(`decompiler/cpp/kuna_goldengen.cc`), one fixture per pinned corpus test.
The Rust SLEIGH runtime must reproduce each fixture body byte-for-byte:
decode the same bytes under the same context and emit the same ops.

Generated at C++ tree rev `%s` by `tools/rust-port/gen_lift.py`.

## Regeneration

```bash
make binaries        # rebuild decomp_dbg after any C++ change
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_lift.py
```

Determinism gate -- two consecutive runs must diff empty:

```bash
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_lift.py --out-dir /tmp/lift2
diff -r tests/golden/vectors/lift /tmp/lift2
```

## Fixture format

Each `<xml-stem>.txt` is a `#`-comment header (corpus path, sla path relative
to `specs/`, the lift points as `space:byte-offset[=symbol]`) followed by the
verbatim console captures:

1. One `golden liftctx <first-point>` dump:
   - `archid` -- full normalized architecture id (language id + compiler);
   - `languageid` -- the SLEIGH language id (archid minus the compiler field);
   - `slafile` -- .sla file name from the matching `.ldefs` entry;
   - `endian` -- `big` or `little`;
   - `contextsize` -- number of 32-bit words in a context blob;
   - `space <name> index=<i> size=<addrsize> wordsize=<w> bigendian=<0|1>` --
     one per address space, in space-index order;
   - `context <name>=<value>` -- EVERY registered context variable, in name
     order, with the value in effect at the first lift point (a `.pspec`
     `<context_set>` paints values per address range, so the defaults blob
     would read all-zero -- the per-address values are what decoding uses).
2. One `golden lift <point> %d` dump per lift point (the image's `<symbol>`
   addresses in document order, deduplicated; symbol-less images use their
   `<bytechunk>` start addresses):
   - `insn <space>:<offset> <length>` per instruction; `<length>` is
     `Translate::oneInstruction`'s fall-through offset, i.e. it INCLUDES
     delay-slot bytes (SPARC/MIPS), and the next instruction is decoded at
     `<offset>+<length>`;
   - `  <OPNAME> <out> <in0> <in1> ...` per emitted p-code op, in emission
     order.  Varnodes render as `(space,0x<offset>,size)`; a missing output
     renders as `-`.  Opcode names are the canonical enum names
     (`get_opname`, with the `ZPULL`/`SPULL` staleness fix of
     `kuna_goldengen.cc`).
   - Spaceid pointer constants -- input 0 of `LOAD`/`STORE` -- print the
     space NAME instead of a raw offset (`(const,ram,8)`): in C++ that
     offset is the heap address of the `AddrSpace` object, the only
     nondeterministic value in a raw lift.  The Rust port stores the space
     index instead (docs/rust-port/losses.md LOSS-015); both engines must
     normalize to the name when emitting this line.
   - A decode failure ends the dump with
     `lifterror <class> <space>:<offset> <message>` where `<class>` is one of
     `dataunavail` / `unimpl` / `baddata` / `lowlevel` (DataUnavailError,
     UnimplError, BadDataError, other LowlevelError).  Running off the end of
     a bytechunk or into non-code bytes is expected -- the error line
     (message text included) is part of the fixture.

Addresses in fixture content are always BYTE offsets.  (The generator
converts them to word offsets when typing console commands -- the console's
address parser multiplies by the space's wordsize -- but nothing word-based
appears in the fixtures themselves.)

## Pinned corpus subset

One test per distinct SLEIGH language id, spanning every processor family the
corpus offers.  Regeneration must keep this pin (the fixture set is part of
the gate):

| corpus test | language id |
%s

Corpus language ids NOT pinned (available for future fixtures):
x86:LE:64:default / x86:LE:64:default:windows / x86:LE:32:default /
x86:LE:32:default:win(dows) / ARM:LE:32:v7 / ARM:LE:32:v8 / ARM:LE:32:v8-m /
AARCH64:LE:64:v8A / MIPS:LE:32:default:default / MIPS:BE:32:default /
PowerPC:BE:32:e500 / sparc:BE:32:default / RISCV:LE:32:default /
8051:BE:16:default / 8085:LE:16:default / PIC-24F:LE:24:default:default /
68000:BE:32:default.
""" % (rev, cap, pins)


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python tools/rust-port/gen_lift.py",
        description="Generate per-instruction p-code lift fixtures from the C++ decompiler.",
    )
    p.add_argument("--out-dir", default=None,
                   help="output directory (default: <repo>/tests/golden/vectors/lift)")
    p.add_argument("--decomp-dbg", default=None, help="path to the decomp_dbg binary")
    p.add_argument("--sleighpath", default=None, help="SLEIGH specs root (default: <repo>/specs)")
    p.add_argument("--cap", type=int, default=80,
                   help="max instructions per lift point (default 80; part of the pin)")
    p.add_argument("--timeout", type=float, default=300, help="seconds per console session")
    args = p.parse_args(argv)

    try:
        written = generate(out_dir=args.out_dir, decomp_dbg=args.decomp_dbg,
                           sleighpath=args.sleighpath, cap=args.cap, timeout=args.timeout)
    except LiftGenError as e:
        print("error: %s" % e, file=sys.stderr)
        return 1
    total_insn = total_err = 0
    for path, stats in written:
        if stats is None:
            print("wrote %s" % path)
            continue
        total_insn += stats["instructions"]
        total_err += stats["lifterrors"]
        print("wrote %s (%s: %d points, %d insns, %d ops, %d lifterrors)"
              % (path, stats["langid"], stats["points"], stats["instructions"],
                 stats["ops"], stats["lifterrors"]))
    print("total: %d fixtures, %d instructions, %d lifterrors"
          % (sum(1 for _, s in written if s is not None), total_insn, total_err))
    return 0


if __name__ == "__main__":
    sys.exit(main())
