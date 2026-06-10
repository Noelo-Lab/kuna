"""The kuna assertion catalog: the discovery half of the LLM control API.

An external operator (a human, or an LLM agent) needs two things to drive kuna's
sub-stage assertions in production:

  1. discover what is settable, what values it takes, and when to flip it -- this
     module (``python -m kuna.catalog``), and
  2. flip them per-decompilation -- ``kuna.decompile``'s ``--option`` / ``--kassert``
     flags (which an LLM picks using this catalog).

The catalog itself lives in ONE place in the C++ source (``kuna_stages.cc``
``settableTable``) and is emitted as JSON by the ``stage catalog`` console command.
This module runs that command and parses the result, so the docs an agent reads can
never drift from the binary's actual behavior. With a binary/function it also reports
each option's LIVE value for that program.

CLI:
    python -m kuna.catalog                 # human-readable table
    python -m kuna.catalog --json          # the raw JSON array (for an LLM/tool)
    python -m kuna.catalog --markdown      # regenerate docs/assertions.md body
    python -m kuna.catalog --option NAME   # one entry
"""
import argparse
import json
import os
import subprocess
import sys

from . import paths


class CatalogError(Exception):
    """Could not obtain the assertion catalog from the decompiler."""


def catalog(*, option=None, decomp_dbg=None, sleighpath=None, timeout=60):
    """Return the kuna settable-assertion catalog as a list of dicts.

    Drives ``decomp_dbg`` with ``stage catalog`` (no binary needed -- the catalog is
    static documentation) and parses the emitted JSON. With ``option`` set, returns a
    one-element list for that option (or raises if unknown).
    """
    bin_path = paths.decomp_dbg() if decomp_dbg is None else os.path.abspath(str(decomp_dbg))
    if not os.path.exists(bin_path):
        raise CatalogError("decomp_dbg not built at %s -- run `make binaries`" % bin_path)
    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))

    cmd = "stage catalog" if option is None else ("stage catalog %s" % option)
    script = cmd + "\nquit\n"
    env = dict(os.environ)
    env["SLEIGHHOME"] = specs
    try:
        proc = subprocess.run(
            [bin_path, "-s", specs],
            input=script, capture_output=True, text=True, env=env, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        raise CatalogError("`stage catalog` timed out after %ss" % timeout)

    out = proc.stdout or ""
    if "Unknown settable option" in out:
        raise CatalogError("unknown option: %r" % option)
    data = _extract_json(out)
    if data is None:
        raise CatalogError(
            "could not parse `stage catalog` output; decompiler said:\n%s" % out.strip()[:2000]
        )
    if isinstance(data, dict):
        return [data]
    return data


def _extract_json(out):
    """Pull the JSON array/object out of the console transcript.

    The console wraps the output in ``[decomp]> ...`` prompts, whose ``[``/``]`` would
    defeat a naive find/rfind. Strip the prompt, then scan from the first ``[``/``{``
    and balance brackets, ignoring any inside JSON string literals, to find the end.
    """
    out = out.replace("[decomp]>", " ")	# drop the fixed console prompt (its [ ] are not JSON)
    start = -1
    for i, ch in enumerate(out):
        if ch in "[{":
            start = i
            break
    if start == -1:
        return None
    depth = 0
    in_str = False
    escape = False
    for i in range(start, len(out)):
        ch = out[i]
        if in_str:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_str = False
            continue
        if ch == '"':
            in_str = True
        elif ch in "[{":
            depth += 1
        elif ch in "]}":
            depth -= 1
            if depth == 0:
                try:
                    return json.loads(out[start:i + 1])
                except json.JSONDecodeError:
                    return None
    return None


def check_drift(*, decomp_dbg=None, sleighpath=None):
    """Verify the catalog documents exactly the registered kuna options.

    Returns a list of human-readable problem strings (empty == OK). Catches the drift
    that breaks the LLM API silently: a kuna ArchOption added to ``options.cc`` without
    a ``settableTable`` row (undiscoverable), or a stale catalog row for a removed
    option. Reads ``options.cc`` for the ``registerOption(new OptionXxx()); // (kuna)``
    registrations and matches them to catalog entries by the option's ``name=``.
    """
    import re

    entries = catalog(decomp_dbg=decomp_dbg, sleighpath=sleighpath)
    cat = {e["option"] for e in entries}

    cpp = paths.repo_root() / "decompiler" / "cpp"
    reg = (cpp / "options.cc").read_text()
    classes = re.findall(r"registerOption\(new (Option\w+)\(\)\);\s*//\s*\(kuna\)", reg)
    # Resolve each kuna OptionXxx class to its option name= string from the kuna_*.hh.
    headers = "".join(
        p.read_text() for p in cpp.glob("kuna_*.hh")
    )
    registered = set()
    unresolved = []
    for cls in classes:
        m = re.search(re.escape(cls) + r"\(void\)\s*\{\s*name\s*=\s*\"([^\"]+)\"", headers)
        if m:
            registered.add(m.group(1))
        else:
            unresolved.append(cls)

    problems = []
    for name in sorted(registered - cat):
        problems.append("registered kuna option %r has no catalog row (undiscoverable)" % name)
    for name in sorted(cat - registered):
        problems.append("catalog row %r matches no registered kuna option (stale)" % name)
    for cls in unresolved:
        problems.append("could not resolve option name for class %s (check kuna_*.hh)" % cls)
    return problems


def to_markdown(entries):
    """Render the catalog as the body of docs/assertions.md."""
    lines = []
    lines.append("# kuna assertion catalog")
    lines.append("")
    lines.append(
        "**Generated** from the decompiler's `stage catalog` command "
        "(`python -m kuna.catalog --markdown`) -- do not edit by hand; edit "
        "`settableTable` in `decompiler/cpp/kuna_stages.cc` and regenerate."
    )
    lines.append("")
    lines.append(
        "These are the kuna stage-model sub-stage decisions an operator (human or LLM) "
        "can flip per decompilation. Defaults are the shipped values (post-DIV-2, see "
        "`docs/divergences.md`). Set any of them with "
        "`python -m kuna.decompile <bin> <fn> --option <name> <value>` (or, per "
        "function, `--kassert \"<stage> <substage> ...\"`); revert any one with its "
        "`off`/`canonical` value."
    )
    lines.append("")
    lines.append(
        "| Option | Values | Default | Stage / sub-stage | Source | Kind | Decision | When to flip |"
    )
    lines.append("|---|---|---|---|---|---|---|---|")
    for e in entries:
        vals = " \\| ".join(e["values"])
        default = e["default"] + (" ⚠️ opt-in" if e.get("destructive_as_default") else "")
        stage = "%s / %s" % (e["stage"], e["substage"])
        lines.append("| `%s` | %s | `%s` | %s | %s | %s | %s | %s |" % (
            e["option"], vals, default, stage,
            e.get("source_decompiler", ""), e.get("change_kind", ""),
            e["summary"], e["use_when"],
        ))
    lines.append("")
    lines.append("## Programmatic use")
    lines.append("")
    lines.append("```bash")
    lines.append("# discover (machine-readable):")
    lines.append("python -m kuna.catalog --json")
    lines.append("")
    lines.append("# decompile with an assertion flipped (repeatable):")
    lines.append("python -m kuna.decompile ./a.out main --option compareform canonical")
    lines.append("python -m kuna.decompile ./sparc.elf main --option returnpair single")
    lines.append("```")
    lines.append("")
    lines.append(
        "The `⚠️ opt-in` defaults (`returnpair`, `v850indirectbranch`) are "
        "documented as destructive global defaults and ship off; apply them per "
        "function / per program only (see each row's *When to flip*)."
    )
    lines.append("")
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m kuna.catalog",
        description="Show the kuna settable-assertion catalog (the LLM discovery surface).",
    )
    p.add_argument("--option", default=None, help="show only this option")
    g = p.add_mutually_exclusive_group()
    g.add_argument("--json", action="store_true", help="emit the raw JSON catalog")
    g.add_argument("--markdown", action="store_true", help="emit the docs/assertions.md body")
    g.add_argument("--check", action="store_true",
                   help="verify the catalog matches the registered kuna options (exit nonzero on drift)")
    p.add_argument("--decomp-dbg", default=None, help="path to the decomp_dbg binary")
    p.add_argument("--engine", choices=("cpp", "rust"), default=None,
                   help="which port's binaries to run (sets KUNA_ENGINE; default: env, else cpp)")
    p.add_argument("--sleighpath", default=None, help="SLEIGH specs root (default: <repo>/specs)")
    args = p.parse_args(argv)

    if args.engine:
        os.environ["KUNA_ENGINE"] = args.engine  # before any paths.binary() resolution

    if args.check:
        try:
            problems = check_drift(decomp_dbg=args.decomp_dbg, sleighpath=args.sleighpath)
        except CatalogError as e:
            print("error: %s" % e, file=sys.stderr)
            return 1
        if problems:
            print("catalog DRIFT:", file=sys.stderr)
            for pr in problems:
                print("  - %s" % pr, file=sys.stderr)
            return 1
        print("catalog OK: documents exactly the registered kuna options")
        return 0

    try:
        entries = catalog(option=args.option, decomp_dbg=args.decomp_dbg, sleighpath=args.sleighpath)
    except CatalogError as e:
        print("error: %s" % e, file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps(entries, indent=2))
    elif args.markdown:
        print(to_markdown(entries))
    else:
        for e in entries:
            flag = "  (opt-in: destructive as global default)" if e.get("destructive_as_default") else ""
            print("%-20s %-20s default=%-10s [%s/%s]%s" % (
                e["option"], "{" + "|".join(e["values"]) + "}", e["default"],
                e["stage"], e["substage"], flag,
            ))
            print("    %s" % e["summary"])
            print("    when: %s" % e["use_when"])
            src = e.get("source_decompiler", "")
            kind = e.get("change_kind", "")
            if src or kind:
                print("    from: %s (%s)" % (src or "?", kind or "?"))
            insp = e.get("inspiration", "")
            if insp:
                print("    inspiration: %s" % insp)
    return 0


if __name__ == "__main__":
    sys.exit(main())
