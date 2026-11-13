"""Build original PC32 wrapping objects and GNU-linked expected bytes."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--assembler", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    source = Path(__file__).resolve().parent
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands = []
    for abi in ("n32", "64"):
        for endian, order in (("be", "b"), ("le", "l")):
            stem = f"pc32-{abi}-{endian}"
            emulation = f"elf32{order}tsmipn32" if abi == "n32" else f"elf64{order}tsmip"
            commands.extend([
                [args.assembler, "-march=mips64r2", f"-mabi={abi}", "-EB" if endian == "be" else "-EL",
                 f"--defsym=ABI64={int(abi == '64')}", "-o", str(output / (stem + ".o")), "pc32.S"],
                [args.linker, "-m", emulation, "-z", "max-page-size=0x1000", "-Ttext=0x400000",
                 "-e", "read_composed_pc32", "--no-relax", "-o", str(output / (stem + ".elf")),
                 str(output / (stem + ".o"))],
            ])
    for command in commands:
        subprocess.run(command, cwd=source, check=True)
    normalized = [[item.replace(str(output) + "/", "") for item in command] for command in commands]
    for command in normalized:
        command[0] = "${AS}" if command[0] == args.assembler else "${LD}"
    report = dict(
        commands=normalized,
        tools={name: subprocess.check_output([path, "--version"], text=True).splitlines()[0]
               for name, path in (("assembler", args.assembler), ("linker", args.linker))},
        sha256={path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in sorted(output.iterdir()) if path.suffix in (".o", ".elf")},
        source_sha256={name: hashlib.sha256((source / name).read_bytes()).hexdigest()
                       for name in ("pc32.S", "build-pc32.py")},
    )
    (output / "provenance-pc32.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
