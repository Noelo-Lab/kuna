"""Thin library shim: ``decompile(binary, target, ...)`` for the pipeline.

The **user-facing CLI** that used to live here was ported to the Rust ``kuna``
binary (``kuna decompile <binary> <func> ...``); see ``decompiler/crates/kuna-cli``.
The Rust binary is the one project paradigm now.

What remains here is **only the library function** the still-Python feature
pipeline (``scripts/pipeline/``) imports as
``from .. import decompile as kdecompile`` and calls as ``kdecompile.decompile(...)``
/ ``kdecompile.DecompileError``. If the pipeline is itself ported to Rust later,
this shim can go.

It drives the (Rust) ``decomp_dbg`` console binary as a subprocess, feeding it the
same command language the datatests use and capturing the decompiled C via
``openfile write`` / ``closefile`` so prompts never pollute the output.
"""
import os
import subprocess
import tempfile

from . import paths


class DecompileError(Exception):
    """Base class for decompilation failures."""


class SpecsNotFound(DecompileError):
    """The decompiler could not locate SLEIGH specs (SLEIGHHOME / -sleighpath)."""


class LoadError(DecompileError):
    """The binary could not be loaded / its architecture could not be built."""


class FunctionNotFound(DecompileError):
    """No function with the requested name (try an address with by_address)."""


class NoOutput(DecompileError):
    """The decompiler produced no C output."""


class DecompileTimeout(DecompileError):
    """The decompiler subprocess exceeded its timeout."""


def _looks_like_addr(target: str) -> bool:
    # Only a 0x-prefixed token auto-selects address mode. A bare hex-looking token
    # (e.g. "add", "dead", "face") is treated as a function name.
    return target.startswith("0x") or target.startswith("0X")


def decompile(
    binary,
    target,
    *,
    by_address=False,
    bfd_target=None,
    raw=False,
    regions=False,
    options=None,
    kasserts=None,
    decomp_dbg=None,
    sleighpath=None,
    timeout=120,
):
    """Decompile ``target`` (a function name or address) in ``binary`` to C source.

    The library entry point the pipeline depends on; see the module docstring.
    Returns the decompiled C as a string (or ``(c, regions)`` with ``regions=True``).
    """
    binary = os.path.abspath(str(binary))
    if not os.path.exists(binary):
        raise LoadError("binary not found: " + binary)

    if decomp_dbg is not None:
        bin_path = os.path.abspath(str(decomp_dbg))
    else:
        bin_path = str(paths.decomp_dbg())
    if not os.path.exists(bin_path):
        raise DecompileError(
            "decomp_dbg not built at %s -- run `make binaries`" % bin_path
        )

    specs = str(paths.specs_dir()) if sleighpath is None else os.path.abspath(str(sleighpath))

    if not by_address and _looks_like_addr(target):
        by_address = True

    out_file = tempfile.NamedTemporaryFile(prefix="kuna_c_", suffix=".c", delete=False)
    out_path = out_file.name
    out_file.close()

    regions_path = None
    if regions:
        regions_file = tempfile.NamedTemporaryFile(
            prefix="kuna_regions_", suffix=".txt", delete=False
        )
        regions_path = regions_file.name
        regions_file.close()

    try:
        script = _build_script(
            binary, target, by_address, bfd_target, raw, out_path, options, kasserts,
            regions_path=regions_path,
        )
        env = dict(os.environ)
        env["SLEIGHHOME"] = specs
        _apply_loadtime_gates(env, options)

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
            raise NoOutput(
                "no C output for %r in %s; decompiler said:\n%s"
                % (target, binary, combined.strip()[:2000])
            )
        if not regions:
            return c_text
        try:
            with open(regions_path, "r") as fh:
                regions_text = fh.read()
        except OSError:
            regions_text = ""
        return c_text, regions_text.strip("\n")
    finally:
        try:
            os.unlink(out_path)
        except OSError:
            pass
        if regions_path is not None:
            try:
                os.unlink(regions_path)
            except OSError:
                pass


# Options whose effect is baked during `load file`, upstream of every `option`
# command, so the console line below cannot reach them: the engine reads them from
# the process environment instead (the same bridge `kuna decompile` puts on the
# decomp_dbg subprocess). Keyed option name -> env var.
_LOADTIME_GATE_ENV = {
    # (kuna `typedepth`) The DWARF type mapper's recursion guard; the types are
    # mapped inside `load file`, so `option typedepth off` alone is too late.
    "typedepth": "KUNA_TYPEDEPTH",
}

# The distinction that decides membership above, because it is easy to get wrong:
# an option needs an entry here only when the load-time work it gates is CONSUMED
# during `load file`. The analysis-pass gates (`rtti`, `itaniumrtti`, `cppproto`,
# `cppsig`, `fdeinterior`, `listing`, ...) also *compute* their facts at load, but
# those facts are COMMITTED later, at `read symbols`
# (`commit_pending_analysis` -> `engine.rs::analysis_pass_enabled`), and the loop
# below emits every `option` line before `read symbols` -- so the console line does
# reach them and an entry here would be redundant. Verified per option by checking
# that a before/after demo renders two DIFFERENT bodies; two identical renderings
# are the symptom that an entry is missing.


def _apply_loadtime_gates(env, options):
    """Export the load-time gates named in ``options`` onto ``env``.

    Without this a before/after demo of a load-time option shows two identical
    renderings: the console `option` line is applied after the image is loaded.
    """
    for name, value in (options or []):
        var = _LOADTIME_GATE_ENV.get(name)
        if var is None:
            continue
        off = str(value).strip().lower() in ("off", "0", "false", "no")
        env[var] = "off" if off else "on"


def _build_script(binary, target, by_address, bfd_target, raw, out_path,
                  options=None, kasserts=None, regions_path=None):
    lines = []
    if bfd_target:
        lines.append("load file %s %s" % (bfd_target, binary))
    else:
        lines.append("load file %s" % binary)

    # `option` lines MUST precede `read symbols` (match the Rust `kuna decompile`
    # CLI, kuna-cli/src/decompile.rs): the kuna_analysis passes are committed —
    # gated by the per-pass `--option <id> on|off` flags — inside `read symbols`
    # (IfcReadSymbols -> commit_pending_analysis). An analysis-tier option set
    # *after* `read symbols` (e.g. `listing on` / `noreturn_propagate on`) is too
    # late to take effect; the upstream/printer options are order-independent
    # here, so emitting all options first is correct for every option.
    for name, value in (options or []):
        lines.append("option %s %s" % (name, value))

    lines.append("read symbols")

    if by_address:
        addr = target if (target.startswith("0x") or target.startswith("0X")) else "0x" + target
        lines.append("load addr %s" % addr)
    else:
        lines.append("load function %s" % target)

    for ka in (kasserts or []):
        lines.append("kassert %s" % ka)

    lines.append("decompile")
    lines.append("openfile write %s" % out_path)
    lines.append("print C")
    if raw:
        lines.append("print raw")
    lines.append("closefile")
    if regions_path is not None:
        lines.append("openfile write %s" % regions_path)
        lines.append("region blocks")
        lines.append("region tree")
        lines.append("closefile")
    lines.append("quit")
    return "\n".join(lines) + "\n"


def _check_errors(out, target, binary, by_address):
    if "Could not discover root of Ghidra installation" in out:
        raise SpecsNotFound(
            "decomp_dbg could not find SLEIGH specs; pass sleighpath or set SLEIGHHOME"
        )
    if "Could not create architecture" in out:
        raise LoadError(
            "could not build an architecture for %s (unsupported/!recognized binary)" % binary
        )
    if not by_address and ("Unknown function name:" in out or "Bad namespace:" in out):
        raise FunctionNotFound(
            "no function %r in %s; for a stripped binary pass an address with by_address"
            % (target, binary)
        )
