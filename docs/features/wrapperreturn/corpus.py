"""Replay the public ARM corpus with a source-built kuna (cross-GNU tools required)."""
from pathlib import Path
import argparse, difflib, json, os, subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--output", type=Path, required=True)
args = parser.parse_args()
here = Path(__file__).resolve().parent
root = here.parents[2]
out = args.output.resolve()
out.mkdir(parents=True, exist_ok=True)
fixtures = out / "fixtures"
fixtures.mkdir(exist_ok=True)
env = {k: v for k, v in os.environ.items() if not k.startswith("KUNA_") and k != "SLEIGHHOME"}
bins = root / "decompiler/target/release"
env.update(TMPDIR=str(out), KUNA_DECOMP_DBG=str(bins / "decomp_dbg"),
           KUNA_SLACOMP=str(bins / "slacomp"), KUNA_SPECS=str(root / "specs"),
           SLEIGHHOME=str(root / "specs"))
(out / "environment.json").write_text(json.dumps({k: v for k, v in env.items()
    if k.startswith("KUNA_") or k in ("TMPDIR", "SLEIGHHOME")}, indent=2) + "\n")
commands = []
def build(*command):
    commands.append(list(map(str, command)))
    subprocess.run(commands[-1], check=True, env=env, cwd=root, timeout=30)

for stem in ("wrapper", "reads"):
    build("arm-linux-gnueabi-as", "-o", fixtures / (stem + ".o"), here / "fixtures" / (stem + ".s"))
for name, flags in (("aliases", []), ("aliases-v7", ["-march=armv7-a"])):
    build("arm-linux-gnueabi-gcc", *flags, "-shared", "-nostdlib", "-o", fixtures / (name + ".so"), here / "fixtures/aliases.s")
for abi, compiler, flags in (("hard", "arm-linux-gnueabihf-gcc", ["-mfpu=vfpv3-d16"]),
                             ("soft", "arm-linux-gnueabi-gcc", [])):
    build(compiler, "-O2", "-marm", "-fno-optimize-sibling-calls", "-fno-inline",
          "-mfloat-abi=" + abi, *flags, "-c", "-o", fixtures / ("returns-" + abi + ".o"), here / "fixtures/returns.c")
files = [fixtures / name for name in ("wrapper.o", "returns-hard.o", "returns-soft.o", "reads.o", "aliases.so", "aliases-v7.so")]
files.append(root / "decompiler/crates/kuna-analysis/tests/fixtures/fmtlf_armhf")
diffs = []
for file in files:
    versions = {}
    for setting in ("off", "on"):
        command = [str(bins / "kuna"), "decompile-all", str(file), "--mode", "aggressive", "--option", 'wrapperreturn', setting]
        commands.append(command)
        result = subprocess.run(command, env=env, cwd=root, capture_output=True, timeout=60)
        (out / (file.name + "-" + setting + ".err")).write_bytes(result.stderr)
        versions[setting] = result.stdout.decode()
        (out / (file.name + "-" + setting + ".out")).write_text(versions[setting])
        if result.returncode or "Error decompiling" in versions[setting] or "// Function:" not in versions[setting]:
            raise RuntimeError(f"{file.name} {setting}: incomplete decompilation")
    diffs.extend(difflib.unified_diff(versions["off"].splitlines(True), versions["on"].splitlines(True),
                 fromfile=file.name + " off", tofile=file.name + " on"))
(out / "commands.json").write_text(json.dumps(commands, indent=2) + "\n")
(out / "corpus.diff").write_text("\n".join(line.rstrip() for line in "".join(diffs).splitlines()).rstrip() + "\n")
