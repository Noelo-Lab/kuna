"""Build original N32 REL fixtures and their GNU-linked expected bytes."""

import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess


def implicit_records(data):
    """Convert zero-addend ELF32 RELA tables to REL without moving sections."""
    data = bytearray(data)
    assert data[:5] == b"\x7fELF\x01" and data[5] in (1, 2)
    order = "<" if data[5] == 1 else ">"
    get = lambda at: struct.unpack_from(order + "I", data, at)[0]
    put = lambda at, value: struct.pack_into(order + "I", data, at, value)
    table = get(32)
    stride, count, names = struct.unpack_from(order + "HHH", data, 46)
    assert stride == 40
    names_header = table + names * stride
    strings = get(names_header + 16)
    old_names = bytes(data[strings:strings + get(names_header + 20)])
    new_names = bytearray(old_names)
    converted = []
    for index in range(count):
        header = table + index * stride
        if get(header + 4) != 4:
            continue
        start, size = get(header + 16), get(header + 20)
        assert get(header + 36) == 12 and size % 12 == 0
        records = [bytes(data[at:at + 8]) for at in range(start, start + size, 12)]
        assert all(get(at + 8) == 0 for at in range(start, start + size, 12))
        packed = b"".join(records)
        data[start:start + size] = packed.ljust(size, b"\0")
        put(header + 4, 9)
        put(header + 20, len(packed))
        put(header + 36, 8)
        name_at = get(header)
        end = old_names.index(0, name_at)
        old = old_names[name_at:end]
        assert old.startswith(b".rela.")
        name = b".rel." + old[6:]
        put(header, len(new_names))
        new_names.extend(name + b"\0")
        converted.append(dict(section=name.decode(), records=len(records)))
    assert len(converted) == 2
    put(names_header + 16, len(data))
    put(names_header + 20, len(new_names))
    data.extend(new_names)
    return bytes(data), converted


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--assembler", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    source = Path(__file__).resolve().parent
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    commands, conversions = [], {}
    for endian, order in (("be", "b"), ("le", "l")):
        stem = f"rel-chain-n32-{endian}"
        obj = output / (stem + ".o")
        assemble = [args.assembler, "-march=mips64r2", "-mabi=n32", "-EB" if endian == "be" else "-EL",
                    "-o", str(obj), "rel-chain.S"]
        subprocess.run(assemble, cwd=source, check=True)
        rela = obj.read_bytes()
        rel, sections = implicit_records(rela)
        obj.write_bytes(rel)
        conversions[stem] = dict(rela_sha256=hashlib.sha256(rela).hexdigest(), sections=sections)
        link = [args.linker, "-m", f"elf32{order}tsmipn32", "-z", "max-page-size=0x1000",
                "-Ttext=0x400000", "-e", "read_combined", "--no-relax",
                "-o", str(output / (stem + ".elf")), str(obj)]
        subprocess.run(link, cwd=source, check=True)
        commands.extend((assemble, link))
    normalized = [[item.replace(str(output) + "/", "") for item in command] for command in commands]
    for command in normalized:
        command[0] = "${AS}" if command[0] == args.assembler else "${LD}"
    report = dict(
        commands=normalized,
        conversion="Zero explicit addends removed; field bits, offsets, symbols and record order preserved.",
        conversions=conversions,
        tools={name: subprocess.check_output([path, "--version"], text=True).splitlines()[0]
               for name, path in (("assembler", args.assembler), ("linker", args.linker))},
        sha256={path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in sorted(output.iterdir()) if path.suffix in (".o", ".elf")},
        source_sha256={name: hashlib.sha256((source / name).read_bytes()).hexdigest()
                       for name in ("rel-chain.S", "build-rel.py")},
    )
    (output / "provenance-rel.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
