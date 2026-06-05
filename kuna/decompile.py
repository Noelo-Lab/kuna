"""Decompile a function from a binary using the extracted Ghidra console decompiler.

This drives the ``decomp_dbg`` console binary as a subprocess, feeding it the same
command language the upstream datatests use (``load file`` / ``load function`` /
``decompile`` / ``print C``). The decompiled C is captured via ``openfile write
<tmp>`` / ``closefile``, which redirects the decompiler's bulk-output stream
(``fileoptr``) to a temp file while interactive prompts stay on stdout -- so the
captured C is free of prompt/echo noise.

CLI:
    python -m kuna.decompile <binary> <function-name-or-0xADDR> [--addr]
                             [--target BFD_TARGET] [--raw]
                             [--decomp-dbg PATH] [--sleighpath DIR] [--timeout S]
"""
import argparse
import os
import subprocess
import sys
import tempfile

from . import paths


class DecompileError(Exception):
    """Base class for decompilation failures."""


class SpecsNotFound(DecompileError):
    """The decompiler could not locate SLEIGH specs (SLEIGHHOME / -sleighpath)."""


class LoadError(DecompileError):
    """The binary could not be loaded / its architecture could not be built."""


class FunctionNotFound(DecompileError):
    """No function with the requested name (try an address with --addr)."""


class NoOutput(DecompileError):
    """The decompiler produced no C output."""


class DecompileTimeout(DecompileError):
    """The decompiler subprocess exceeded its timeout."""


def _looks_like_addr(target: str) -> bool:
    # Only a 0x-prefixed token auto-selects address mode. A bare hex-looking token
    # (e.g. "add", "dead", "face") is treated as a function name; use --addr for
    # bare numeric addresses.
    return target.startswith("0x") or target.startswith("0X")


def decompile(
    binary,
    target,
    *,
    by_address=False,
    bfd_target=None,
    raw=False,
    decomp_dbg=None,
    sleighpath=None,
    timeout=120,
):
    """Decompile ``target`` (a function name or address) in ``binary`` to C source.

    Parameters
    ----------
    binary : path to the binary to decompile.
    target : a function name, or an address like ``0x401000`` when ``by_address``.
    by_address : treat ``target`` as an address (uses ``load addr``). Auto-enabled
        if ``target`` looks like ``0x...``.
    bfd_target : optional explicit BFD target for ``load file`` (e.g. ``elf64-x86-64``).
    raw : also emit the raw p-code listing (``print raw``) after the C.
    decomp_dbg : path to the decomp_dbg binary (defaults to the built one).
    sleighpath : SLEIGH specs root (defaults to ``<repo>/specs``).
    timeout : seconds before the subprocess is killed.

    Returns the decompiled C as a string.
    """
    binary = os.path.abspath(str(binary))
    if not os.path.exists(binary):
        raise LoadError("binary not found: " + binary)

    bin_path = paths.decomp_dbg() if decomp_dbg is None else os.path.abspath(str(decomp_dbg))
    if not os.path.exists(bin_path):
        raise DecompileError(
            "decomp_dbg not built at %s -- run `make binaries`" % bin_path
        )

    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))

    if not by_address and _looks_like_addr(target):
        by_address = True

    out_file = tempfile.NamedTemporaryFile(
        prefix="kuna_c_", suffix=".c", delete=False
    )
    out_path = out_file.name
    out_file.close()

    try:
        script = _build_script(binary, target, by_address, bfd_target, raw, out_path)
        env = dict(os.environ)
        # SLEIGHHOME is the load-bearing mechanism: decomp_dbg scans it
        # RECURSIVELY for Ghidra/Processors/*/data/languages dirs (the layout of
        # kuna's specs/ root), overriding any hostile value in the caller's env.
        # "-s" only adds the dir literally (non-recursive), which does nothing
        # for the specs/ root but keeps `--sleighpath <dir-with-.ldefs>` working
        # when pointed directly at a languages directory.
        env["SLEIGHHOME"] = specs

        try:
            proc = subprocess.run(
                [bin_path, "-s", specs],
                input=script,
                capture_output=True,
                text=True,
                env=env,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            raise DecompileTimeout(
                "decompilation of %r in %s timed out after %ss" % (target, binary, timeout)
            )

        combined = (proc.stdout or "") + "\n" + (proc.stderr or "")
        _check_errors(combined, target, binary, by_address)

        try:
            with open(out_path, "r") as fh:
                c_text = fh.read()
        except OSError:
            c_text = ""

        c_text = c_text.strip("\n")
        if not c_text.strip():
            # No C produced and no recognized error string -> generic failure.
            raise NoOutput(
                "no C output for %r in %s; decompiler said:\n%s"
                % (target, binary, combined.strip()[:2000])
            )
        return c_text
    finally:
        try:
            os.unlink(out_path)
        except OSError:
            pass


def _build_script(binary, target, by_address, bfd_target, raw, out_path):
    lines = []
    if bfd_target:
        # Two-token form: `load file <target> <path>` (target first per IfcLoadFile).
        lines.append("load file %s %s" % (bfd_target, binary))
    else:
        lines.append("load file %s" % binary)

    # `load file` does not populate the symbol table; the console needs this
    # explicitly (the XML datatests auto-read symbols, the BFD console path does not).
    lines.append("read symbols")

    if by_address:
        addr = target if (target.startswith("0x") or target.startswith("0X")) else "0x" + target
        lines.append("load addr %s" % addr)
    else:
        lines.append("load function %s" % target)

    lines.append("decompile")
    lines.append("openfile write %s" % out_path)
    lines.append("print C")
    if raw:
        lines.append("print raw")
    lines.append("closefile")
    lines.append("quit")
    return "\n".join(lines) + "\n"


def _check_errors(out, target, binary, by_address):
    if "Could not discover root of Ghidra installation" in out:
        raise SpecsNotFound(
            "decomp_dbg could not find SLEIGH specs; pass --sleighpath or set SLEIGHHOME"
        )
    if "Could not create architecture" in out:
        raise LoadError(
            "could not build an architecture for %s (unsupported/!recognized binary)" % binary
        )
    if not by_address and ("Unknown function name:" in out or "Bad namespace:" in out):
        raise FunctionNotFound(
            "no function %r in %s; for a stripped binary pass an address with --addr"
            % (target, binary)
        )


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m kuna.decompile",
        description="Decompile a function from a binary with Ghidra's extracted C++ decompiler.",
    )
    p.add_argument("binary", help="path to the binary")
    p.add_argument("target", help="function name, or an address like 0x401000 (with --addr)")
    p.add_argument("--addr", action="store_true", help="treat target as an address (load addr)")
    p.add_argument("--target", dest="bfd_target", default=None,
                   help="explicit BFD target for load file (e.g. elf64-x86-64)")
    p.add_argument("--raw", action="store_true", help="also print the raw p-code listing")
    p.add_argument("--decomp-dbg", default=None, help="path to the decomp_dbg binary")
    p.add_argument("--sleighpath", default=None, help="SLEIGH specs root (default: <repo>/specs)")
    p.add_argument("--timeout", type=float, default=120, help="subprocess timeout in seconds")
    args = p.parse_args(argv)

    try:
        c = decompile(
            args.binary,
            args.target,
            by_address=args.addr,
            bfd_target=args.bfd_target,
            raw=args.raw,
            decomp_dbg=args.decomp_dbg,
            sleighpath=args.sleighpath,
            timeout=args.timeout,
        )
    except DecompileError as e:
        print("error: %s" % e, file=sys.stderr)
        return 1
    print(c)
    return 0


if __name__ == "__main__":
    sys.exit(main())
