"""Replay the armfloatreturn corpus: option off vs on over every ARM ELF fixture
in kuna-analysis/tests/fixtures plus fixtures/*.c built with clang as hard-float
ARM, hard-float Thumb, softfp and soft-float objects. Run after `make binaries`."""
from pathlib import Path
import argparse, difflib, json, os, re, subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--output", type=Path, required=True)
args = parser.parse_args()
here = Path(__file__).resolve().parent
root = here.parents[2]
out = args.output.resolve()
built = out / "built"
built.mkdir(parents=True, exist_ok=True)
bins = root / "decompiler/target/release"
env = dict(os.environ)
env.setdefault("KUNA_SPECS", str(root / "specs"))
env.setdefault("SLEIGHHOME", env["KUNA_SPECS"])

abis = {
    "hard-arm": ["--target=armv7a-linux-gnueabihf", "-marm", "-mfloat-abi=hard", "-mfpu=vfpv3-d16"],
    "hard-thumb": ["--target=armv7a-linux-gnueabihf", "-mthumb", "-mfloat-abi=hard", "-mfpu=vfpv3-d16"],
    "softfp-arm": ["--target=armv7a-linux-gnueabi", "-marm", "-mfloat-abi=softfp", "-mfpu=vfpv3-d16"],
    "soft-arm": ["--target=armv7a-linux-gnueabi", "-marm", "-mfloat-abi=soft"],
}
files = []
for source in sorted((here / "fixtures").glob("*.c")):
    for abi, flags in abis.items():
        obj = built / f"{source.stem}-{abi}.o"
        subprocess.run(["clang", *flags, "-O2", "-fno-optimize-sibling-calls", "-c", str(source), "-o", str(obj)],
                       check=True, timeout=60)
        files.append(obj)
fixtures = root / "decompiler/crates/kuna-analysis/tests/fixtures"
for path in sorted(fixtures.iterdir()):
    head = path.read_bytes()[:20] if path.is_file() else b""
    if head[:4] == b"\x7fELF" and head[5] == 1 and int.from_bytes(head[18:20], "little") == 40:
        files.append(path)

summary, diffs = [], []
for path in files:
    text = {}
    for setting in ("off", "on"):
        result = subprocess.run([str(bins / "kuna"), "decompile-all", str(path), "--mode", "aggressive",
                                 "--option", "armfloatreturn", setting],
                                env=env, capture_output=True, timeout=600)
        if result.returncode:
            raise RuntimeError(f"{path.name} {setting}: exit {result.returncode}")
        text[setting] = result.stdout.decode()
    split = {s: dict(re.findall(r"// Function: (\S+) @[^\n]*\n(.*?)(?=\n// Function: |\Z)", t, re.S))
             for s, t in text.items()}
    changed = sorted(name for name in split["off"] if split["off"][name] != split["on"].get(name))
    summary.append({"file": path.name, "functions": len(split["off"]), "changed": changed})
    diffs.extend(difflib.unified_diff(text["off"].splitlines(True), text["on"].splitlines(True),
                                      fromfile=path.name + " off", tofile=path.name + " on"))
(out / "summary.json").write_text(json.dumps(summary, indent=1) + "\n")
(out / "corpus.diff").write_text("\n".join(line.rstrip() for line in "".join(diffs).splitlines()).rstrip() + "\n")
print(f"{len(files)} files, {sum(s['functions'] for s in summary)} functions, "
      f"{sum(len(s['changed']) for s in summary)} changed")
