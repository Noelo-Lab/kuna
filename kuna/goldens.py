"""Stage-boundary differential ("golden") harness for the Rust port.

Snapshots the decompiler's state at fixed pipeline boundaries for every XML
datatest, so the Rust port -- which speaks the same console-command surface as
``decomp_dbg`` -- can be diffed against the C++ oracle boundary-by-boundary
instead of only at final C output.

Program loading (discovered empirically; consolemain.cc / xml_arch.cc):
``load file`` recognizes any file whose first bytes are ``<bi`` as an XML
``<binaryimage>`` program (XmlArchitectureCapability::isFileMatch), resolves
the SLEIGH language from the image's ``arch`` attribute, and -- because the
matching capability is "xml" -- auto-reads the loader symbols. So a datatest
replays standalone by extracting its raw ``<binaryimage>`` element to a temp
file, issuing ``load file <tmp>``, then feeding the test's original ``<com>``
script lines verbatim (minus ``quit``) to the console. (The ``restore``
command is *not* usable here: it requires an ``<xml_savefile>`` root.)

Boundaries, snapshot per (decompile-com index ``seq``, boundary):
  B0  after architecture load: ``list action``  (pins pipeline registration)
  B2  at ``break start heritage`` of each decompile:
      ``print raw`` + ``print tree block``       (pre-SSA dataflow + CFG)
  B3  immediately after the first heritage execution:
      ``print raw`` + ``print tree varnode``     (fresh SSA form)
  B4  after full decompile:
      ``print raw`` + ``print tree varnode`` + ``print tree block``
  B5  after full decompile: ``print C``          (rendered output)

B3 wrinkle (discovered empirically): ``break action heritage`` never fires --
ActionHeritage registers no change count and break_action only triggers after
an "active transformation". The action immediately following heritage in the
mainloop group is ``paramdouble`` (coreaction.cc universalAction), so the
*first* ``break start paramdouble`` is exactly the first-heritage-completion
point (the snapshot shows SSA def links instead of "(free)" varnodes).

Two console invocations per test:
  inv1 (B0/B4/B5): no breakpoints; dumps inserted after each decompile com.
  inv2 (B2/B3): ``break start heritage`` + ``break start paramdouble``.
      Breakpoints cannot be cleared, and heritage re-breaks on every mainloop
      iteration, so after the boundary dumps each decompile is driven to
      completion with a fixed surplus of ``continue`` commands (extras print a
      harmless "already complete" execution error; max observed breaks per
      decompile in the corpus is 16, the surplus is 96). The stdout transcript
      is validated chunk-by-chunk against the fed script -- a dump that was
      not taken at its intended break is recorded as *suspect* in the
      manifest, never trusted silently.

All dumps write through ``openfile write`` (the fileoptr redirect) so prompts
and replayed-script noise never pollute the snapshot files.

Determinism scrub: ``print tree varnode`` leaks process heap addresses, which
differ on every run (and can never match between the C++ and Rust engines):
``(internal=0x...)`` on every varnode, and the raw value of constant varnodes
that hold an AddrSpace pointer (the const space input of LOAD/STORE prints as
e.g. ``#0x57a0a264ed60``). Snapshot files are normalized in place after each
invocation: ``internal=...`` is dropped exactly, and ``#0x``-constants that
look like a Linux user-space heap/mmap pointer (12 hex digits starting 5-7)
become ``#(spaceptr)``. The second rule is a heuristic -- a genuine 48-bit
program constant of that shape would also be masked (soundness is preserved:
both engines are scrubbed identically, only a little diff discrimination is
lost). Ordering is heap-dependent too: constant-space varnodes sort by raw
value, and annotation constants (``fprintf:8``, instruction refs) hold heap
pointers, so their tree order shuffles per run. Each contiguous run of
varnode-tree lines (recognized by the scrubbed ``internal=`` marker) is
therefore sorted before being written -- tree comparison is multiset-style;
op/block order stays exact via ``print raw``/``print tree block``. An exact
fix needs a dump that prints space names instead of pointers (the C++ side's
new ``golden`` command is the follow-up home for that).

Snapshots land in ``tests/golden/snapshots/<engine>/<xml-stem>/<seq>-<Bn>.txt``
(gitignored) plus a ``manifest.json`` per engine (git rev, binary mtime,
per-file status, per-invocation command transcripts). A committed corpus
inventory (no local paths/timestamps) is kept at
``tests/golden/snapshot-manifest.json``.

CLI:
    python -m kuna.goldens snapshot [--xml NAME ...] [--boundary B0,B2,...]
                                    [--engine cpp|rust] [--timeout S]
    python -m kuna.goldens compare  [--xml NAME ...] [--boundary ...]
                                    [--rust-dir DIR] [--timeout S]
"""
import argparse
import difflib
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import xml.etree.ElementTree as ET

from . import paths

ALL_BOUNDARIES = ("B0", "B2", "B3", "B4", "B5")

# Dump commands per boundary; every one writes through status->fileoptr, so
# `openfile write` captures it cleanly (verified against ifacedecomp.cc).
_DUMPS = {
    "B0": ("list action",),
    "B2": ("print raw", "print tree block"),
    "B3": ("print raw", "print tree varnode"),
    "B4": ("print raw", "print tree varnode", "print tree block"),
    "B5": ("print C",),
}

_PROMPT = "[decomp]> "
_DRAIN_CONTINUES = 96
_BINARYIMAGE = re.compile(r"<binaryimage\b.*?</binaryimage>", re.S)
_ARCH = re.compile(r'arch="([^"]*)"')
_SNAPNAME = re.compile(r"^(\d{3})-(B\d)\.txt$")
_NOISE = "[decomp]> continue\nExecution error: Decompilation is already complete\n"
_NOISE_RUN = re.compile("(?:" + re.escape(_NOISE) + "){2,}")

# Heap-address normalization for snapshot files (see module docstring).
_SCRUBS = (
    (re.compile(r"\(internal=0x[0-9a-f]+\)"), "(internal=scrubbed)"),
    (re.compile(r"#0x[5-7][0-9a-f]{11}\b"), "#(spaceptr)"),
)


class GoldenError(Exception):
    """Setup-level failure (missing binary, unknown test, no cpp snapshots)."""


# --- corpus ------------------------------------------------------------------


def corpus():
    """All corpus tests as (stem, path): datatests then stage tests (no stem collisions)."""
    out = []
    for d in (paths.datatests_dir(), paths.stage_datatests_dir()):
        for p in sorted(d.glob("*.xml")):
            out.append((p.stem, p))
    return out


def select_tests(names=None):
    """Resolve --xml selections (stem or filename) against the corpus."""
    everything = corpus()
    if not names:
        return everything
    by_stem = dict(everything)
    picked = []
    for name in names:
        stem = name[:-4] if name.endswith(".xml") else name
        if stem not in by_stem:
            raise GoldenError("unknown test %r (not in datatests/ or tests/stages/)" % name)
        picked.append((stem, by_stem[stem]))
    return picked


def parse_test(path):
    """Return (raw <binaryimage> text, script com list, arch string) for a datatest.

    The image is extracted as a raw byte-exact substring (never re-serialized)
    because the bytechunk text is load-bearing; the script is parsed with
    ElementTree (no corpus com contains markup characters). Com text is
    whitespace-normalized onto one line: the upstream harness hands each com
    to the console as a single istream, so a multiline com (enum.xml's
    ``parse line enum {...}``) is one command there, but the stdin-driven
    console reads line-by-line -- newlines inside a com must become spaces.
    """
    src = open(str(path)).read()
    m = _BINARYIMAGE.search(src)
    if m is None:
        raise ValueError("no <binaryimage> element")
    image = m.group(0)
    root = ET.parse(str(path)).getroot()
    coms = [" ".join((c.text or "").split()) for c in root.findall("./script/com")]
    coms = [c for c in coms if c]
    if not coms:
        raise ValueError("no <script><com> commands")
    arch = _ARCH.search(image)
    return image, coms, (arch.group(1) if arch else "")


def _is_decompile(com):
    # The corpus spells it "decompile" or "dec"; any >=3-char prefix is the
    # unique console expansion (no other command starts with "dec").
    tok = com.split()[0]
    return len(tok) >= 3 and "decompile".startswith(tok)


def _is_quit(com):
    return com.split()[0] == "quit"


# --- script building ---------------------------------------------------------
#
# A script is a list of line dicts:
#   line       the console command text
#   expect     transcript expectation for this command's output chunk:
#              "loaded" | "complete" | "break-heritage" | "break-paramdouble"
#              | "quiet"
#   seq        decompile index the expectation belongs to
#   file       snapshot filename this `openfile write` opens
#   boundaries boundaries invalidated if a "quiet" line errors
#   drain      seq whose surplus `continue` this is


def _dump_block(lines, snapdir, seq, boundary):
    fname = "%03d-%s.txt" % (seq, boundary)
    lines.append({"line": "openfile write %s" % os.path.join(snapdir, fname),
                  "file": fname})
    for cmd in _DUMPS[boundary]:
        lines.append({"line": cmd})
    lines.append({"line": "closefile"})


def _build_plain(image_path, coms, snapdir, boundaries):
    """inv1: B0 right after load; B4/B5 right after each decompile completes."""
    lines = [{"line": "load file %s" % image_path, "expect": "loaded"}]
    if "B0" in boundaries:
        _dump_block(lines, snapdir, 0, "B0")
    if boundaries & {"B4", "B5"}:
        seq = 0
        for com in coms:
            if _is_quit(com):
                continue
            if _is_decompile(com):
                lines.append({"line": com, "expect": "complete", "seq": seq})
                for b in ("B4", "B5"):
                    if b in boundaries:
                        _dump_block(lines, snapdir, seq, b)
                seq += 1
            else:
                lines.append({"line": com})
    lines.append({"line": "quit"})
    return lines


def _build_break(image_path, coms, snapdir, boundaries):
    """inv2: B2 at `break start heritage`, B3 at first `break start paramdouble`."""
    lines = [{"line": "load file %s" % image_path, "expect": "loaded"}]
    if "B2" in boundaries:
        lines.append({"line": "break start heritage", "expect": "quiet",
                      "boundaries": ("B2", "B3")})
    if "B3" in boundaries:
        lines.append({"line": "break start paramdouble", "expect": "quiet",
                      "boundaries": ("B3",)})
    seq = 0
    for com in coms:
        if _is_quit(com):
            continue
        if _is_decompile(com):
            first = "break-heritage" if "B2" in boundaries else "break-paramdouble"
            lines.append({"line": com, "expect": first, "seq": seq})
            if "B2" in boundaries:
                _dump_block(lines, snapdir, seq, "B2")
                if "B3" in boundaries:
                    lines.append({"line": "continue", "expect": "break-paramdouble",
                                  "seq": seq})
            if "B3" in boundaries:
                _dump_block(lines, snapdir, seq, "B3")
            for _ in range(_DRAIN_CONTINUES):
                lines.append({"line": "continue", "drain": seq})
            seq += 1
        else:
            lines.append({"line": com})
    lines.append({"line": "quit"})
    return lines


# --- execution + transcript validation ----------------------------------------


def _run_console(bin_path, specs, script, timeout):
    """Run one decomp_dbg session; return CompletedProcess or None on timeout."""
    env = dict(os.environ)
    env["SLEIGHHOME"] = str(specs)  # recursive spec discovery (see kuna.decompile)
    try:
        return subprocess.run(
            [str(bin_path), "-s", str(specs)],
            input=script, capture_output=True, text=True, env=env, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return None


def _signal_line(chunk):
    """First informative output line of a transcript chunk (after the echo)."""
    for ln in chunk.splitlines()[1:]:
        if ln.strip():
            return ln.strip()[:200]
    return "(no output)"


def _validate(lines, stdout):
    """Walk the prompt-delimited transcript against the fed script.

    Returns (fatal, issues, warnings, executed):
      fatal    reason string making the whole test unreplayable, or None
      issues   [(boundaries, seq_or_None, reason)] -> snapshot files to mark suspect
      warnings test-level notes that do not invalidate any specific file
      executed how many script lines produced an output chunk
    """
    body = stdout.split(_PROMPT)[1:]
    issues, warnings = [], []
    executed = min(len(body), len(lines))
    if len(body) < len(lines):
        warnings.append("console session ended after %d of %d commands"
                        % (len(body), len(lines)))
    drain_seen = {}  # seq -> completed before next replayed command
    for i in range(executed):
        ln, chunk = lines[i], body[i]
        if "drain" in ln:
            seq = ln["drain"]
            if "Decompilation complete" in chunk:
                drain_seen[seq] = True
            else:
                drain_seen.setdefault(seq, False)
        exp = ln.get("expect")
        if exp is None:
            continue
        if exp == "loaded":
            if "successfully loaded" not in chunk:
                return ("program load failed: " + _signal_line(chunk),
                        issues, warnings, executed)
        elif exp == "complete":
            if "Decompilation complete" not in chunk:
                issues.append((("B4", "B5"), ln["seq"],
                               "decompile %d did not complete: %s"
                               % (ln["seq"], _signal_line(chunk))))
        elif exp == "break-heritage":
            if "heritage start" not in chunk:
                issues.append((("B2", "B3"), ln["seq"],
                               "no break at heritage start: " + _signal_line(chunk)))
        elif exp == "break-paramdouble":
            if "paramdouble start" not in chunk:
                issues.append((("B3",), ln["seq"],
                               "no break at paramdouble start: " + _signal_line(chunk)))
        elif exp == "quiet":
            tail = [t for t in chunk.splitlines()[1:] if t.strip()]
            if tail:
                issues.append((ln["boundaries"], None,
                               "breakpoint setup failed: " + tail[0].strip()[:200]))
    for seq, done in sorted(drain_seen.items()):
        if not done:
            warnings.append("decompile %d not driven to completion before the "
                            "next replayed command (later functions may differ "
                            "from the upstream-harness session)" % seq)
    return None, issues, warnings, executed


def _collapse(stdout):
    """Collapse runs of surplus continue/already-complete noise for the manifest."""
    def repl(m):
        n = len(m.group(0)) // len(_NOISE) - 1
        return _NOISE + "[... %d more identical continue exchanges elided ...]\n" % n
    return _NOISE_RUN.sub(repl, stdout)


# --- per-test snapshot ---------------------------------------------------------


def _scrub_file(path):
    """Normalize run-varying heap artifacts in a snapshot file (in place).

    Regex-scrubs heap addresses, then sorts each contiguous run of
    varnode-tree lines: constant-space varnodes are ordered by their raw
    value, which for annotation constants is itself a heap pointer, so the
    printed order shuffles between runs (and between engines).
    """
    try:
        with open(path) as fh:
            text = fh.read()
    except OSError:
        return
    scrubbed = text
    for pat, repl in _SCRUBS:
        scrubbed = pat.sub(repl, scrubbed)
    lines = scrubbed.splitlines(True)
    out, run = [], []
    for ln in lines + [""]:  # sentinel flushes the last run
        if "(internal=scrubbed)" in ln:
            run.append(ln)
        else:
            if run:
                out.extend(sorted(run))
                run = []
            if ln:
                out.append(ln)
    scrubbed = "".join(out)
    if scrubbed != text:
        with open(path, "w") as fh:
            fh.write(scrubbed)


def _clear_stale(snapdir, boundaries):
    if not os.path.isdir(snapdir):
        return
    for f in os.listdir(snapdir):
        m = _SNAPNAME.match(f)
        if m and m.group(2) in boundaries:
            os.unlink(os.path.join(snapdir, f))


def snapshot_test(stem, path, boundaries, snapdir, bin_path, specs, timeout):
    """Snapshot one datatest; returns its manifest entry (never raises on replay errors)."""
    entry = {
        "source": os.path.relpath(str(path), str(paths.repo_root())),
        "status": "ok", "files": {}, "warnings": [], "invocations": [],
    }
    try:
        image, coms, arch = parse_test(path)
    except (ValueError, ET.ParseError, OSError) as e:
        entry["status"] = "unreplayable"
        entry["reason"] = "parse failure: %s" % e
        entry["decompiles"] = 0
        return entry
    entry["arch"] = arch
    entry["decompiles"] = sum(1 for c in coms if _is_decompile(c))

    os.makedirs(snapdir, exist_ok=True)
    _clear_stale(snapdir, boundaries)

    fd, image_path = tempfile.mkstemp(prefix="kuna_img_", suffix=".xml")
    with os.fdopen(fd, "w") as fh:
        fh.write(image)

    plans = []
    if boundaries & {"B0", "B4", "B5"}:
        plans.append(_build_plain(image_path, coms, snapdir, boundaries))
    if boundaries & {"B2", "B3"} and entry["decompiles"]:
        plans.append(_build_break(image_path, coms, snapdir, boundaries))

    fatal = None
    issues = []
    interrupted = []  # filenames whose dump block did not fully execute
    try:
        for lines in plans:
            script = "\n".join(l["line"] for l in lines) + "\n"
            t0 = time.time()
            proc = _run_console(bin_path, specs, script, timeout)
            if proc is None:
                entry["invocations"].append({"script": script, "timeout": True})
                fatal = "timeout after %ss" % timeout
                break
            entry["invocations"].append({
                "script": script,
                "stdout": _collapse(proc.stdout or ""),
                "stderr": (proc.stderr or "")[-4000:],
                "returncode": proc.returncode,
                "seconds": round(time.time() - t0, 2),
            })
            f, iss, warns, executed = _validate(lines, proc.stdout or "")
            if f is None and executed == 0:
                first = ((proc.stderr or "") + (proc.stdout or "")).strip()
                f = ("console produced no prompts (rc=%s): %s"
                     % (proc.returncode,
                        first.splitlines()[0][:200] if first else "(no output)"))
            issues.extend(iss)
            entry["warnings"].extend(warns)
            if f is not None:
                fatal = f
                break
            for i, ln in enumerate(lines):
                if "file" in ln and i + len(_DUMPS[ln["file"][4:6]]) + 1 >= executed:
                    interrupted.append(ln["file"])
    finally:
        try:
            os.unlink(image_path)
        except OSError:
            pass

    if fatal is not None:
        entry["status"] = "unreplayable"
        entry["reason"] = fatal
        _clear_stale(snapdir, boundaries)  # never leave half-trusted files behind
        return entry

    expected = [ln["file"] for lines in plans for ln in lines if "file" in ln]
    for fname in expected:
        _scrub_file(os.path.join(snapdir, fname))
    suspect = {}
    for bs, seq, reason in issues:
        for fname in expected:
            m = _SNAPNAME.match(fname)
            if m.group(2) in bs and (seq is None or int(m.group(1)) == seq):
                suspect.setdefault(fname, reason)
    for fname in interrupted:
        suspect.setdefault(fname, "console session ended during this dump")
    for fname in expected:
        if not os.path.exists(os.path.join(snapdir, fname)):
            entry["files"][fname] = "missing"
        elif fname in suspect:
            entry["files"][fname] = "suspect: " + suspect[fname]
        else:
            entry["files"][fname] = "ok"
    if any(v != "ok" for v in entry["files"].values()):
        entry["status"] = "suspect"
    return entry


# --- manifest ------------------------------------------------------------------


def _snap_root():
    return paths.repo_root() / "tests" / "golden" / "snapshots"


def _git_rev():
    try:
        return subprocess.run(
            ["git", "-C", str(paths.repo_root()), "rev-parse", "HEAD"],
            capture_output=True, text=True, timeout=30,
        ).stdout.strip()
    except (OSError, subprocess.TimeoutExpired):
        return ""


def _write_manifest(engine_dir, engine, boundaries, bin_path, specs, results):
    """Merge this run's entries into <engine_dir>/manifest.json."""
    man_path = os.path.join(str(engine_dir), "manifest.json")
    manifest = {"schema": 1, "tests": {}}
    if os.path.exists(man_path):
        try:
            with open(man_path) as fh:
                manifest = json.load(fh)
        except (OSError, ValueError):
            pass
    st = os.stat(str(bin_path))
    manifest.update({
        "schema": 1,
        "engine": engine,
        "generated": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "git_rev": _git_rev(),
        "binary": {"path": str(bin_path), "mtime": int(st.st_mtime), "size": st.st_size},
        "specs": str(specs),
        "boundaries": sorted(boundaries),
    })
    manifest.setdefault("tests", {}).update(results)
    with open(man_path, "w") as fh:
        json.dump(manifest, fh, indent=1, sort_keys=True)
        fh.write("\n")
    return manifest


def _write_inventory(manifest):
    """Committed corpus inventory (tests/golden/snapshot-manifest.json): no
    local paths, mtimes or transcripts, so its diffs stay reviewable."""
    inv = {"schema": 1, "git_rev": manifest.get("git_rev", ""),
           "boundaries": manifest.get("boundaries", []), "tests": {}}
    for stem, e in sorted(manifest.get("tests", {}).items()):
        per_boundary = {}
        for fname, st in e.get("files", {}).items():
            if st == "ok":
                b = _SNAPNAME.match(fname).group(2)
                per_boundary[b] = per_boundary.get(b, 0) + 1
        inv["tests"][stem] = {
            "source": e.get("source", ""),
            "arch": e.get("arch", ""),
            "decompiles": e.get("decompiles", 0),
            "status": e.get("status", ""),
            "files": per_boundary,
        }
        if e.get("status") == "unreplayable":
            inv["tests"][stem]["reason"] = e.get("reason", "")
    path = paths.repo_root() / "tests" / "golden" / "snapshot-manifest.json"
    os.makedirs(str(path.parent), exist_ok=True)
    with open(str(path), "w") as fh:
        json.dump(inv, fh, indent=1, sort_keys=True)
        fh.write("\n")


# --- snapshot / compare drivers --------------------------------------------------


def _resolve_engine(engine):
    """Point kuna.paths at the requested engine and return (bin_path, specs)."""
    os.environ["KUNA_ENGINE"] = engine
    bin_path = paths.decomp_dbg()
    if not bin_path.exists():
        hint = "run `make binaries`" if engine == "cpp" else "build the rust port first"
        raise GoldenError("%s decomp_dbg not built at %s -- %s" % (engine, bin_path, hint))
    return bin_path, paths.specs_dir()


def run_snapshots(tests, boundaries, engine, engine_dir, timeout, verbose=True):
    """Snapshot ``tests`` for ``engine`` into ``engine_dir``; returns {stem: entry}."""
    bin_path, specs = _resolve_engine(engine)
    os.makedirs(str(engine_dir), exist_ok=True)
    results = {}
    for stem, path in tests:
        snapdir = os.path.join(str(engine_dir), stem)
        entry = snapshot_test(stem, path, boundaries, snapdir, bin_path, specs, timeout)
        results[stem] = entry
        if verbose:
            n_ok = sum(1 for v in entry["files"].values() if v == "ok")
            if entry["status"] == "unreplayable":
                print("  %-32s UNREPLAYABLE: %s" % (stem, entry["reason"]))
            elif entry["status"] == "suspect":
                bad = [f for f, v in sorted(entry["files"].items()) if v != "ok"]
                print("  %-32s %d files ok, suspect/missing: %s"
                      % (stem, n_ok, ", ".join(bad)))
            else:
                print("  %-32s %d files ok" % (stem, n_ok))
    _write_manifest(engine_dir, engine, boundaries, bin_path, specs, results)
    return results


def _parse_boundaries(spec):
    if not spec:
        return set(ALL_BOUNDARIES)
    out = set()
    for tok in spec.split(","):
        tok = tok.strip().upper()
        if tok not in ALL_BOUNDARIES:
            raise GoldenError("unknown boundary %r (choose from %s)"
                              % (tok, ",".join(ALL_BOUNDARIES)))
        out.add(tok)
    return out


def _summarize(results, engine_dir, boundaries, elapsed):
    statuses = [e["status"] for e in results.values()]
    print("snapshot summary (%s):" % engine_dir)
    print("  tests: %d total, %d ok, %d suspect, %d unreplayable"
          % (len(results), statuses.count("ok"), statuses.count("suspect"),
             statuses.count("unreplayable")))
    counts, size = {}, 0
    for stem, e in results.items():
        for fname, st in e["files"].items():
            p = os.path.join(str(engine_dir), stem, fname)
            if st != "missing" and os.path.exists(p):
                b = _SNAPNAME.match(fname).group(2)
                counts[b] = counts.get(b, 0) + 1
                size += os.path.getsize(p)
    for b in sorted(boundaries):
        print("  %s: %d files" % (b, counts.get(b, 0)))
    print("  %.1f MB on disk, %.1f s wall" % (size / 1e6, elapsed))


def cmd_snapshot(args):
    boundaries = _parse_boundaries(args.boundary)
    tests = select_tests(args.xml)
    engine = args.engine
    engine_dir = _snap_root() / engine
    t0 = time.time()
    results = run_snapshots(tests, boundaries, engine, engine_dir, args.timeout)
    _summarize(results, engine_dir, boundaries, time.time() - t0)
    if engine == "cpp":
        with open(os.path.join(str(engine_dir), "manifest.json")) as fh:
            _write_inventory(json.load(fh))
    return 0


def cmd_compare(args):
    boundaries = _parse_boundaries(args.boundary)
    tests = select_tests(args.xml)
    cpp_dir = _snap_root() / "cpp"
    man_path = cpp_dir / "manifest.json"
    if not man_path.exists():
        raise GoldenError("no cpp snapshots at %s -- run "
                          "`python -m kuna.goldens snapshot` first" % cpp_dir)
    with open(str(man_path)) as fh:
        cpp_manifest = json.load(fh)
    missing = [s for s, _ in tests if s not in cpp_manifest.get("tests", {})]
    if missing:
        raise GoldenError("no cpp snapshot for: %s -- run snapshot first"
                          % ", ".join(missing))

    rust_dir = args.rust_dir or str(_snap_root() / "rust")
    print("snapshotting rust engine into %s ..." % rust_dir)
    rust_results = run_snapshots(tests, boundaries, "rust", rust_dir, args.timeout)

    n_div = n_clean = n_skip = 0
    for stem, _ in tests:
        centry = cpp_manifest["tests"][stem]
        if centry["status"] == "unreplayable":
            print("  %-32s SKIP (cpp: unreplayable: %s)" % (stem, centry.get("reason")))
            n_skip += 1
            continue
        rentry = rust_results[stem]
        if rentry["status"] == "unreplayable":
            print("  %-32s DIVERGED (rust replay failed: %s)" % (stem, rentry.get("reason")))
            n_div += 1
            continue
        first = None
        for fname in sorted(centry["files"]):
            if _SNAPNAME.match(fname).group(2) not in boundaries:
                continue
            cpath = os.path.join(str(cpp_dir), stem, fname)
            rpath = os.path.join(rust_dir, stem, fname)
            ctext = open(cpath).read() if os.path.exists(cpath) else None
            rtext = open(rpath).read() if os.path.exists(rpath) else None
            if ctext != rtext:
                first = (fname, ctext, rtext)
                break
        if first is None:
            n_clean += 1
            continue
        n_div += 1
        fname, ctext, rtext = first
        print("  %-32s DIVERGED first at %s/%s" % (stem, stem, fname))
        if ctext is None or rtext is None:
            print("    %s snapshot missing" % ("cpp" if ctext is None else "rust"))
            continue
        diff = difflib.unified_diff(
            ctext.splitlines(True), rtext.splitlines(True),
            "cpp/%s/%s" % (stem, fname), "rust/%s/%s" % (stem, fname))
        for i, line in enumerate(diff):
            if i >= args.diff_lines:
                print("    [... diff truncated ...]")
                break
            print("    " + line.rstrip("\n"))
    print("compare summary: %d identical, %d divergent, %d skipped"
          % (n_clean, n_div, n_skip))
    return 1 if n_div else 0


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m kuna.goldens",
        description="Snapshot/diff decompiler state at stage boundaries over the "
                    "XML datatest corpus (the Rust-port differential harness).",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    ps = sub.add_parser("snapshot", help="record boundary snapshots for one engine")
    ps.add_argument("--xml", action="append", metavar="NAME",
                    help="restrict to this test (stem or filename; repeatable; "
                         "default: full corpus)")
    ps.add_argument("--boundary", default=None, metavar="B0,B2,...",
                    help="comma-separated boundary subset (default: %s)"
                         % ",".join(ALL_BOUNDARIES))
    ps.add_argument("--engine", choices=("cpp", "rust"), default="cpp",
                    help="which port's decomp_dbg to drive (sets KUNA_ENGINE)")
    ps.add_argument("--timeout", type=float, default=120,
                    help="seconds per console invocation (default 120)")
    ps.set_defaults(func=cmd_snapshot)

    pc = sub.add_parser("compare", help="snapshot the rust engine and diff it "
                                        "against the recorded cpp snapshots")
    pc.add_argument("--xml", action="append", metavar="NAME",
                    help="restrict to this test (repeatable; default: full corpus)")
    pc.add_argument("--boundary", default=None, metavar="B0,B2,...",
                    help="comma-separated boundary subset (default: all)")
    pc.add_argument("--rust-dir", default=None,
                    help="side directory for the rust snapshots "
                         "(default: tests/golden/snapshots/rust)")
    pc.add_argument("--diff-lines", type=int, default=20,
                    help="max unified-diff lines shown per divergent test")
    pc.add_argument("--timeout", type=float, default=120,
                    help="seconds per console invocation (default 120)")
    pc.set_defaults(func=cmd_compare)

    args = p.parse_args(argv)
    try:
        return args.func(args)
    except GoldenError as e:
        print("error: %s" % e, file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
