"""Rebuild original MIPS relocation objects and their GNU-linked oracle images."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--assembler", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    source = Path(__file__).resolve().parent
    args.output.mkdir(parents=True, exist_ok=True)
    output = args.output.resolve()
    commands = []
    for abi in ("n32", "64"):
        for endian, order in (("be", "b"), ("le", "l")):
            emulation = f"elf32{order}tsmipn32" if abi == "n32" else f"elf64{order}tsmip"
            for name in ("expressions", "gp", "got", "call-composed"):
                stem = f"{name}-{abi}-{endian}"
                entry = "direct_call" if name == "call-composed" else "_start"
                commands.extend([
                    [args.assembler, "-march=octeon", f"-mabi={abi}", *(["-msoft-float"] if name == "call-composed" else []),
                     "-EB" if endian == "be" else "-EL", f"--defsym=ABI64={int(abi == '64')}", "--defsym=ABI32=0",
                     "-o", str(output / f"{stem}.o"), f"{name}.S"],
                    [args.linker, "-m", emulation, "-z", "max-page-size=0x1000",
                     "-Ttext=0x400000", "-e", entry, "--no-relax",
                     *([f"--defsym={target}=0x{0x402000 + index * 16:x}"
                        for index, target in enumerate(("direct_target", "composed_target", "scalar_target", "cross_target", "relative_target"))]
                       if name == "call-composed" else []),
                     *(["-T", f"{name}.ld"] if name in ("gp", "got") else []),
                     "-o", str(output / f"{stem}.elf"), str(output / f"{stem}.o")],
                ])
                if name == "gp":
                    partial = f"gp-partial-{abi}-{endian}"
                    commands.extend([
                        [args.linker, "-r", "-m", emulation, "-T", "gp.ld",
                         "-o", str(output / f"{partial}.o"), str(output / f"{stem}.o")],
                        [args.linker, "-m", emulation, "-z", "max-page-size=0x1000",
                         "-T", "gp.ld", "--no-relax", "-o", str(output / f"{partial}.elf"),
                         str(output / f"{partial}.o")],
                    ])
    for command in commands:
        subprocess.run(command, cwd=source, check=True)
    for name in ("implicit", "gp", "got"):
        stem = f"{name}-32-be"
        extra = [
            [args.assembler, "-march=octeon", "-mabi=32", "-EB", "--defsym=ABI32=1", "--defsym=ABI64=0",
             "-o", str(output / f"{stem}.o"), f"{name}.S"],
            [args.linker, "-m", "elf32btsmip", "-z", "max-page-size=0x1000",
             "-Ttext=0x400000", "-e", "_start", "--no-relax",
             *(["-T", f"{name}.ld"] if name in ("gp", "got") else []),
             "-o", str(output / f"{stem}.elf"), str(output / f"{stem}.o")],
        ]
        for command in extra:
            subprocess.run(command, cwd=source, check=True)
        commands.extend(extra)
    normalized = [[item.replace(str(output) + "/", "") for item in command] for command in commands]
    for command in normalized:
        command[0] = "${AS}" if command[0] == args.assembler else "${LD}"
    report = {
        "commands": normalized,
        "tools": {
            name: subprocess.check_output([path, "--version"], text=True).splitlines()[0]
            for name, path in (("assembler", args.assembler), ("linker", args.linker))
        },
        "sha256": {path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                   for path in sorted(output.iterdir()) if path.suffix in (".o", ".elf")},
        "source_sha256": {name: hashlib.sha256((source / name).read_bytes()).hexdigest()
                          for name in ("expressions.S", "implicit.S", "gp.S", "gp.ld", "got.S", "got.ld", "call-composed.S", "build.py")},
    }
    (output / "provenance.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
