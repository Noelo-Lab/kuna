"""Build original compiler-generated PIC witnesses with matching linker addresses."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess


def allocated_sections(path):
    data = path.read_bytes()
    assert data[:4] == b"\x7fELF"
    wide = data[4] == 2
    endian = ">" if data[5] == 2 else "<"
    table = struct.unpack_from(endian + ("Q" if wide else "I"), data, 40 if wide else 32)[0]
    stride, count, names = struct.unpack_from(endian + "HHH", data, 58 if wide else 46)
    headers = [struct.unpack_from(endian + ("IIQQQQIIQQ" if wide else "10I"), data, table + i * stride) for i in range(count)]
    strings = data[headers[names][4]:headers[names][4] + headers[names][5]]
    for header in headers:
        if header[2] & 2 and header[5]:
            name = strings[header[0]:].split(b"\0", 1)[0].decode("ascii")
            assert re.fullmatch(r"[.A-Za-z0-9_]+", name), name
            yield name, header[5], max(1, header[8])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--tag", required=True, choices=("gcc47", "gcc73"))
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    source = Path(__file__).resolve().parent
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands = []

    def run(command):
        commands.append(command)
        subprocess.run(command, cwd=source, check=True)

    for abi in ("n32", "64"):
        stem = f"pic-{args.tag}-{abi}"
        flags = ["-march=octeon2", f"-mabi={abi}", "-msoft-float", "-EB", "-G0", "-O2", "-g",
                 "-fPIC", "-mabicalls", "-ffreestanding", "-fno-builtin", "-fno-common",
                 "-fno-asynchronous-unwind-tables", "-fno-unwind-tables", f"-fdebug-prefix-map={source}=."]
        obj = output / f"{stem}.o"
        helper = output / f"external-{args.tag}-{abi}.o"
        run([args.compiler, *flags, "-c", "pic.c", "-o", str(obj)])
        run([args.compiler, *flags, "-c", "external.c", "-o", str(helper)])
        cursor = 0x400000
        lines = ["ENTRY(read_global)", "SECTIONS", "{"]
        for name, size, alignment in allocated_sections(obj):
            cursor = (cursor + alignment - 1) & -alignment
            pattern = f"*{obj.name}" if name == ".MIPS.options" else "*"
            lines.append(f"  {name} {cursor:#x} : {{ {pattern}({name}) }}")
            cursor += size
        got = (cursor + 15) & -16
        assert got < 0x401000
        lines.extend([f"  _gp = {got + 0x7ff0:#x};", f"  .got {got:#x} : {{ *(.got) }}",
                      "  .external_text 0x402000 : { *(.external_text) }",
                      f"  /DISCARD/ : {{ *{helper.name}(.MIPS.options) }}", "}"])
        script = output / f"{stem}.ld"
        script.write_text("\n".join(lines) + "\n")
        emulation = "elf32btsmipn32" if abi == "n32" else "elf64btsmip"
        run([args.linker, "-m", emulation, "-z", "max-page-size=0x1000", "--no-relax",
             "-T", str(script), "-o", str(output / f"{stem}.elf"), str(obj), str(helper)])
    normalized = [[part.replace(str(output) + "/", "").replace(str(source), "${SOURCE}") for part in cmd] for cmd in commands]
    for command in normalized:
        command[0] = "${CC}" if command[0] == args.compiler else "${LD}"
    report = {
        "commands": normalized,
        "tools": {key: subprocess.check_output([path, "--version"], text=True).splitlines()[0]
                  for key, path in (("compiler", args.compiler), ("linker", args.linker))},
        "source_sha256": {name: hashlib.sha256((source / name).read_bytes()).hexdigest()
                          for name in ("pic.c", "external.c", "build-pic.py")},
        "sha256": {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(output.iterdir())
                   if args.tag in path.name and path.suffix in (".o", ".elf", ".ld")},
    }
    (output / f"provenance-{args.tag}.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
