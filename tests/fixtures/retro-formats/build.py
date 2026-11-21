#!/usr/bin/env python3
"""Build the retro-format fixtures from retro.c and m68k/src/lib.rs.

The compilers produce ordinary ELF and XCOFF objects; this script links them
and writes each container, because no installed linker emits these formats:

    hello.psx        PS-X EXE, MIPS I little-endian (clang, ld.lld)
    hello.z64        N64 ROM, MIPS III big-endian n32 (clang, ld.lld); the
                     0x40..0x1000 boot-code area is zero
    hello-m68k.aout  NetBSD/m68k ZMAGIC a.out executable (rustc)
    hello-i386.aout  i386 OMAGIC a.out relocatable object (clang -m32)
    hello.hunk       AmigaDOS hunk load file, 68000 (rustc)
    hello.pef        PEF fragment, PowerPC (clang -target powerpc-ibm-aix)

Tools, overridable from the environment: CLANG (needs the MIPS and PowerPC
back ends), HOST_CLANG, LLD, OBJCOPY, CARGO (a nightly with rust-src, for
-Zbuild-std on m68k-unknown-none-elf).
"""
import os
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
CLANG = os.environ.get("CLANG", "clang")
HOST_CLANG = os.environ.get("HOST_CLANG", "clang")
LLD = os.environ.get("LLD", "ld.lld")
OBJCOPY = os.environ.get("OBJCOPY", "llvm-objcopy")
CARGO = os.environ.get("CARGO", "cargo")
CFLAGS = ["-O1", "-ffreestanding", "-fno-pic", "-fomit-frame-pointer",
          "-fno-asynchronous-unwind-tables", "-c", str(HERE / "retro.c")]


def run(*command, **kwargs):
    subprocess.run([str(part) for part in command], check=True, **kwargs)


def pad(data, to):
    return data + bytes(-len(data) % to)


# ---- ELF relocatable objects ------------------------------------------------

class ElfObject:
    """The allocated sections, symbols and relocations of a 32-bit ELF .o."""

    def __init__(self, path):
        raw = Path(path).read_bytes()
        self.e = ">" if raw[5] == 2 else "<"
        shoff, = struct.unpack_from(self.e + "I", raw, 0x20)
        shentsize, shnum, shstrndx = struct.unpack_from(self.e + "HHH", raw, 0x2e)
        heads = [struct.unpack_from(self.e + "10I", raw, shoff + i * shentsize) for i in range(shnum)]
        names = raw[heads[shstrndx][4]:]
        self.sections = []
        for name, kind, flags, _addr, offset, size, link, info, align, entsize in heads:
            self.sections.append({
                "name": names[name:names.index(b"\0", name)].decode(), "type": kind, "flags": flags,
                "size": size, "link": link, "info": info, "align": max(align, 1), "entsize": entsize,
                "data": b"" if kind == 8 else raw[offset:offset + size],
            })
        symtab = next(s for s in self.sections if s["type"] == 2)
        strings = self.sections[symtab["link"]]["data"]
        self.symbols = []
        for at in range(0, symtab["size"], 16):
            name, value, size, info, _other, shndx = struct.unpack_from(self.e + "IIIBBH", symtab["data"], at)
            self.symbols.append({"name": strings[name:strings.index(b"\0", name)].decode(), "value": value,
                                 "info": info, "shndx": shndx})
        self.relocs = []
        for section in self.sections:
            if section["type"] not in (4, 9):
                continue
            width = 12 if section["type"] == 4 else 8
            for at in range(0, section["size"], width):
                offset, info = struct.unpack_from(self.e + "II", section["data"], at)
                if section["type"] == 4:
                    addend, = struct.unpack_from(self.e + "i", section["data"], at + 8)
                else:
                    addend, = struct.unpack_from(self.e + "i", self.sections[section["info"]]["data"], offset)
                self.relocs.append((section["info"], offset, info & 0xff, info >> 8, addend))


class Linked:
    """An ElfObject laid out as text, data and bss groups."""

    def __init__(self, obj, first=None):
        self.obj = obj
        self.blobs = {"text": bytearray(), "data": bytearray(), "bss": bytearray()}
        self.place = {}
        order = list(range(len(obj.sections)))
        if first is not None:
            lead = next(s["shndx"] for s in obj.symbols if s["name"] == first)
            order.remove(lead)
            order.insert(0, lead)
        for index in order:
            section = obj.sections[index]
            if not section["flags"] & 2 or section["type"] not in (1, 8) or section["size"] == 0:
                continue
            group = "bss" if section["type"] == 8 else "text" if section["flags"] & 4 else "data"
            blob = self.blobs[group]
            blob.extend(bytes(-len(blob) % section["align"]))
            self.place[index] = (group, len(blob))
            blob.extend(section["data"] or bytes(section["size"]))
        for blob in self.blobs.values():
            blob.extend(bytes(-len(blob) % 4))

    def symbol(self, index):
        """(group, offset) of a defined symbol, or its name when undefined."""
        symbol = self.obj.symbols[index]
        if symbol["shndx"] == 0:
            return symbol["name"]
        group, base = self.place[symbol["shndx"]]
        return group, base + symbol["value"]

    def named(self):
        """Defined global functions and objects as (name, group, offset)."""
        out = []
        for index, symbol in enumerate(self.obj.symbols):
            if symbol["info"] >> 4 == 1 and symbol["info"] & 0xf in (1, 2) and symbol["shndx"] in self.place:
                if "rust" in symbol["name"]:
                    continue
                out.append((symbol["name"],) + self.symbol(index))
        return out

    def relocations(self):
        """(group, offset, type, target, addend) for every relocation."""
        for section, offset, kind, symbol, addend in self.obj.relocs:
            if section in self.place:
                group, base = self.place[section]
                yield group, base + offset, kind, self.symbol(symbol), addend


def cargo_m68k(work):
    env = {key: value for key, value in os.environ.items() if key != "RUSTC_WRAPPER"}
    env["RUSTFLAGS"] = "-C relocation-model=static -C code-model=large"
    env["CARGO_TARGET_DIR"] = str(work / "m68k")
    run(*CARGO.split(), "rustc", "--release", "-Zbuild-std=core", "--target", "m68k-unknown-none-elf",
        "--manifest-path", HERE / "m68k" / "Cargo.toml", "--", "--emit", "obj", env=env)
    return max((work / "m68k").rglob("retro-*.o"), key=lambda path: path.stat().st_mtime)


R_68K_32 = 1


def write_hunk(linked):
    """Code, data and bss hunks; RELOC32 words hold offsets into their target hunk."""
    order = ["text", "data", "bss"]
    relocs = {group: {} for group in order}
    for group, offset, kind, target, addend in linked.relocations():
        assert kind == R_68K_32, f"unsupported m68k relocation {kind}"
        struct.pack_into(">I", linked.blobs[group], offset, target[1] + addend)
        relocs[group].setdefault(order.index(target[0]), []).append(offset)
    out = struct.pack(">6I", 0x3f3, 0, 3, 0, 2, len(linked.blobs["text"]) // 4)
    out += struct.pack(">2I", len(linked.blobs["data"]) // 4, len(linked.blobs["bss"]) // 4)
    for index, group in enumerate(order):
        blob = linked.blobs[group]
        if group == "bss":
            out += struct.pack(">2I", 0x3eb, len(blob) // 4)
        else:
            out += struct.pack(">2I", 0x3e9 if group == "text" else 0x3ea, len(blob) // 4) + blob
        if relocs[group] and group == "text":
            out += struct.pack(">I", 0x3ec)
            for target, offsets in sorted(relocs[group].items()):
                out += struct.pack(f">{len(offsets) + 2}I", len(offsets), target, *offsets)
            out += struct.pack(">I", 0)
        elif relocs[group]:
            short = b""
            for target, offsets in sorted(relocs[group].items()):
                short += struct.pack(f">{len(offsets) + 2}H", len(offsets), target, *offsets)
            out += struct.pack(">I", 0x3fc) + pad(short + struct.pack(">H", 0), 4)
        names = [(name, offset) for name, where, offset in linked.named() if where == group]
        if names:
            out += struct.pack(">I", 0x3f0)
            for name, offset in names:
                text = pad(name.encode(), 4)
                out += struct.pack(">I", len(text) // 4) + text + struct.pack(">I", offset)
            out += struct.pack(">I", 0)
        out += struct.pack(">I", 0x3f2)
        assert index == order.index(group)
    return out


N_UNDF, N_TEXT, N_DATA, N_BSS, N_EXT = 0, 4, 6, 8, 1
SEGMENT_TYPE = {"text": N_TEXT, "data": N_DATA, "bss": N_BSS}


def nlist(endian, entries):
    """The symbol table and string table for (name, type, value) entries."""
    table, strings = b"", b""
    for name, kind, value in entries:
        table += struct.pack(endian + "IBBHI", 4 + len(strings), kind, 0, 0, value)
        strings += b"_" + name.encode() + b"\0"
    return table, struct.pack(endian + "I", 4 + len(strings)) + strings


def write_aout_m68k(linked):
    """NetBSD ZMAGIC: the header is the first 32 bytes of a text segment at 0x2000."""
    page, header = 0x2000, 32
    a_text = len(pad(bytes(header) + linked.blobs["text"], page))
    base = {"text": page + header, "data": page + a_text}
    base["bss"] = base["data"] + len(linked.blobs["data"])
    for group, offset, kind, target, addend in linked.relocations():
        assert kind == R_68K_32, f"unsupported m68k relocation {kind}"
        struct.pack_into(">I", linked.blobs[group], offset, base[target[0]] + target[1] + addend)
    names = linked.named()
    table, strings = nlist(">", [(name, SEGMENT_TYPE[group] | N_EXT, base[group] + offset)
                                 for name, group, offset in names])
    entry = next(base[group] + offset for name, group, offset in names if name == "main")
    midmag = (0x87 << 16) | 0o413
    head = struct.pack(">8I", midmag, a_text, len(linked.blobs["data"]), len(linked.blobs["bss"]),
                       len(table), entry, 0, 0)
    return pad(head + linked.blobs["text"], page) + linked.blobs["data"] + table + strings


R_386_32, R_386_PC32, R_386_PLT32 = 1, 2, 4


def write_aout_i386(linked):
    """Linux OMAGIC object: text at 0, data after it, 8-byte relocation records."""
    base = {"text": 0, "data": len(linked.blobs["text"])}
    base["bss"] = base["data"] + len(linked.blobs["data"])
    names = linked.named()
    undefined = sorted({target for *_, target, _ in linked.relocations() if isinstance(target, str)})
    entries = [(name, SEGMENT_TYPE[group] | N_EXT, base[group] + offset) for name, group, offset in names]
    first_undefined = len(entries)
    entries += [(name, N_UNDF | N_EXT, 0) for name in undefined]
    records = {"text": b"", "data": b""}
    for group, offset, kind, target, addend in linked.relocations():
        assert kind in (R_386_32, R_386_PC32, R_386_PLT32), f"unsupported i386 relocation {kind}"
        relative = kind != R_386_32
        place = base[group] + offset if relative else 0
        if isinstance(target, str):
            symbol, external, value = first_undefined + undefined.index(target), 1, addend - place
        else:
            symbol, external = SEGMENT_TYPE[target[0]], 0
            value = base[target[0]] + target[1] + addend - place
        struct.pack_into("<i", linked.blobs[group], offset, value)
        records[group] += struct.pack("<II", offset, symbol | relative << 24 | 2 << 25 | external << 27)
    table, strings = nlist("<", entries)
    head = struct.pack("<8I", 0o407 | 100 << 16, len(linked.blobs["text"]), len(linked.blobs["data"]),
                       len(linked.blobs["bss"]), len(table), 0, len(records["text"]), len(records["data"]))
    return head + linked.blobs["text"] + linked.blobs["data"] + records["text"] + records["data"] + table + strings


# ---- MIPS images ------------------------------------------------------------

def link_mips(work, name, triple, flags, base, entry):
    """Compile and link retro.c at `base`; returns (image bytes, symbol values)."""
    obj, elf, image = work / f"{name}.o", work / f"{name}.elf", work / f"{name}.bin"
    run(CLANG, "-target", triple, *flags, *CFLAGS, "-mno-abicalls", "-o", obj, stderr=subprocess.DEVNULL)
    run(LLD, f"-Ttext={base:#x}", "-T", HERE / "image.ld", "-e", entry, obj, "-o", elf)
    run(OBJCOPY, "-O", "binary", elf, image)
    linked = ElfObject(elf)
    symbols = {symbol["name"]: symbol["value"] for symbol in linked.symbols}
    bss = next(s for s in linked.sections if s["name"] == ".bss")
    return image.read_bytes(), symbols, bss["size"]


def write_psx(work):
    base = 0x80010000
    image, symbols, bss_size = link_mips(work, "psx", "mipsel-unknown-elf", ["-march=mips1", "-G8"], base, "main")
    text = pad(image, 0x800)
    head = bytearray(0x800)
    head[:8] = b"PS-X EXE"
    struct.pack_into("<4I", head, 0x10, symbols["main"], symbols["_gp"], base, len(text))
    struct.pack_into("<2I", head, 0x28, base + len(text), (bss_size + 3) & ~3)
    struct.pack_into("<I", head, 0x30, 0x801ffff0)
    return bytes(head) + text


def write_z64(work):
    base = 0x80000400
    flags = ["-march=mips3", "-mabi=n32", "-G0", "-DSTART"]
    image, symbols, _ = link_mips(work, "n64", "mips64-unknown-elf", flags, base, "_start")
    assert symbols["_start"] == base
    head = bytearray(0x1000)
    struct.pack_into(">3I", head, 0, 0x80371240, 0xf, base)
    head[0x20:0x34] = b"KUNA RETRO".ljust(20)
    return bytes(head) + pad(image, 16)


# ---- PEF from an AIX XCOFF object ---------------------------------------------

R_POS, R_TOC, R_BR, R_RBR = 0x00, 0x03, 0x0a, 0x1a
C_EXT = 2
XMC_DS = 10
GLUE = bytes.fromhex("81820000" "90410014" "800c0000" "804c0004" "7c0903a6" "4e800420")


class Xcoff:
    """Sections, symbols and relocations of a 32-bit XCOFF .o."""

    def __init__(self, path):
        raw = Path(path).read_bytes()
        _magic, nscns, _time, symptr, nsyms, opthdr, _flags = struct.unpack_from(">HHIIIHH", raw, 0)
        strings = raw[symptr + nsyms * 18:]
        self.sections, self.relocs = {}, []
        for index in range(nscns):
            at = 20 + opthdr + index * 40
            name = raw[at:at + 8].rstrip(b"\0").decode()
            _paddr, vaddr, size, scnptr, relptr, _ln, nreloc, _nln, _fl = struct.unpack_from(">6IHHI", raw, at + 8)
            self.sections[name] = {"vaddr": vaddr, "size": size, "data": bytearray(raw[scnptr:scnptr + size])}
            for record in range(nreloc):
                where, symbol, _size, kind = struct.unpack_from(">IIBB", raw, relptr + record * 10)
                self.relocs.append((name, where, symbol, kind))
        self.symbols, index = {}, 0
        while index < nsyms:
            at = symptr + index * 18
            if raw[at:at + 4] == bytes(4):
                offset, = struct.unpack_from(">I", raw, at + 4)
                name = strings[offset:strings.index(b"\0", offset)].decode()
            else:
                name = raw[at:at + 8].rstrip(b"\0").decode()
            value, scnum, _type, sclass, numaux = struct.unpack_from(">IhHBB", raw, at + 8)
            storage = raw[at + numaux * 18 + 11] if numaux else None
            self.symbols[index] = {"name": name, "value": value, "scnum": scnum, "sclass": sclass,
                                   "class": storage}
            index += 1 + numaux


def pef_hash(name):
    value = 0
    for char in name.encode():
        value = ((value << 1) - (value >> 16)) ^ char
        value = (value + 2**31) % 2**32 - 2**31
    return len(name) << 16 | ((value ^ (value >> 16)) & 0xffff)


def pack_pattern(data):
    """Pattern-initialized data: zero runs, repeated blocks and literal copies."""
    def argument(value):
        out = [value & 0x7f]
        while value := value >> 7:
            out.insert(0, 0x80 | (value & 0x7f))
        return bytes(out)

    def op(opcode, count, *arguments):
        head = bytes([opcode << 5 | count]) if count < 32 else bytes([opcode << 5]) + argument(count)
        return head + b"".join(argument(value) for value in arguments)

    out, literal, at = b"", b"", 0

    def flush():
        nonlocal out, literal
        if literal:
            out += op(1, len(literal)) + literal
            literal = b""

    while at < len(data):
        zeros = len(data) - at - len(data[at:].lstrip(b"\0"))
        if zeros >= 3:
            flush()
            out += op(0, zeros)
            at += zeros
            continue
        for size in (4, 8):
            block, repeats = data[at:at + size], 1
            while data[at + repeats * size:at + (repeats + 1) * size] == block:
                repeats += 1
            if len(block) == size and repeats >= 3:
                flush()
                out += op(2, size, repeats - 1) + block
                at += size * repeats
                break
        else:
            literal += data[at:at + 1]
            at += 1
    flush()
    return out


def relocation_program(relocs):
    """Relocation instructions for sorted (offset, kind) pairs; kind is "C",
    "D" or an import index."""
    out, position, index = [], 0, 0
    while index < len(relocs):
        offset, kind = relocs[index]
        kinds = [k for _, k in relocs[index:]]
        offsets = [o for o, _ in relocs[index:]]
        vectors = 0
        while (kinds[vectors * 2:vectors * 2 + 2] == ["C", "D"]
               and offsets[vectors * 2:vectors * 2 + 2] == [offset + vectors * 12, offset + vectors * 12 + 4]):
            vectors += 1
        gap = offset - position
        if kind == "D" and gap % 4 == 0 and gap // 4 < 256 and not vectors:
            run_length = 1
            while (run_length < 63 and index + run_length < len(relocs)
                   and relocs[index + run_length] == (offset + 4 * run_length, "D")):
                run_length += 1
            out.append(gap // 4 << 6 | run_length)
            position, index = offset + 4 * run_length, index + run_length
            continue
        if gap:
            out.append(0x8000 | (gap - 1))
        if vectors:
            out.append(0x4400 | (vectors - 1))
            position, index = offset + 12 * vectors, index + 2 * vectors
        elif kind == "C":
            out.append(0x4000)
            position, index = offset + 4, index + 1
        else:
            out.append(0x6000 | kind)
            position, index = offset + 4, index + 1
    return struct.pack(f">{len(out)}H", *out)


def write_pef(work):
    obj = work / "ppc.o"
    run(CLANG, "-target", "powerpc-ibm-aix", "-DHOSTED", *CFLAGS, "-o", obj)
    xcoff = Xcoff(obj)
    code, data = xcoff.sections[".text"]["data"], xcoff.sections[".data"]["data"]
    data_at = xcoff.sections[".data"]["vaddr"]
    toc = next(s["value"] for s in xcoff.symbols.values() if s["name"] == "TOC") - data_at
    imports, relocs = [], []
    for section, where, symbol, kind in xcoff.relocs:
        target = xcoff.symbols[symbol]
        if section == ".data":
            assert kind == R_POS
            where -= data_at
            value, = struct.unpack_from(">I", data, where)
            in_data = target["scnum"] == 2
            struct.pack_into(">I", data, where, value - data_at if in_data else value)
            relocs.append((where, "D" if in_data else "C"))
        elif kind in (R_BR, R_RBR) and target["scnum"] == 0:
            name = target["name"].lstrip(".")
            if name not in imports:
                imports.append(name)
                slot = len(data)
                data += bytes(4)
                relocs.append((slot, imports.index(name)))
                glue = bytearray(GLUE)
                struct.pack_into(">h", glue, 2, slot - toc)
                target["glue"] = len(code)
                code += glue
            glue_at = next(s["glue"] for s in xcoff.symbols.values() if s["name"] == target["name"] and "glue" in s)
            struct.pack_into(">I", code, where, 0x48000001 | ((glue_at - where) & 0x03fffffc))
            assert code[where + 4:where + 8] == bytes.fromhex("60000000")
            code[where + 4:where + 8] = bytes.fromhex("80410014")
        else:
            assert kind in (R_TOC, R_BR, R_RBR), f"unsupported XCOFF relocation {kind:#x}"
    exports = []
    for symbol in xcoff.symbols.values():
        if symbol["sclass"] == C_EXT and symbol["scnum"] == 2:
            exports.append((symbol["name"], 2 if symbol["class"] == XMC_DS else 1, symbol["value"] - data_at))
    main = next(value for name, _, value in exports if name == "main")

    strings = b""

    def string(text, terminated=True):
        nonlocal strings
        at = len(strings)
        strings += text.encode() + (b"\0" if terminated else b"")
        return at

    libraries = struct.pack(">5I4x", string("StdCLib"), 0, 0, len(imports), 0)
    import_table = b"".join(struct.pack(">I", 2 << 24 | string(name)) for name in imports)
    program = relocation_program(sorted(relocs, key=lambda reloc: reloc[0]))
    reloc_headers = struct.pack(">HHII", 1, 0, len(program) // 2, 0)
    keys = b"".join(struct.pack(">I", pef_hash(name)) for name, _, _ in exports)
    export_table = b"".join(struct.pack(">IIh", kind << 24 | string(name, False), value, 1)
                            for name, kind, value in exports)
    reloc_at = 56 + len(libraries) + len(import_table) + len(reloc_headers)
    strings_at = reloc_at + len(program)
    hash_at = (strings_at + len(strings) + 3) & ~3
    head = struct.pack(">iIiIiI8I", 1, main, -1, 0, -1, 0, 1, len(imports), 1, reloc_at, strings_at, hash_at, 0,
                       len(exports))
    loader = pad(head + libraries + import_table + reloc_headers + program + strings, 4)
    loader += struct.pack(">I", len(exports) << 18) + keys + export_table

    pattern = pack_pattern(bytes(data).rstrip(b"\0"))
    unpacked = len(bytes(data).rstrip(b"\0"))
    code_at = 40 + 3 * 28
    code_at += -code_at % 16
    pattern_at = code_at + len(pad(bytes(code), 4))
    loader_at = pattern_at + len(pad(pattern, 4))
    out = b"Joy!peffpwpc" + struct.pack(">5IHHI", 1, 0, 0, 0, 0, 3, 2, 0)
    out += struct.pack(">i5I4B", -1, 0, len(code), len(code), len(code), code_at, 0, 4, 4, 0)
    out += struct.pack(">i5I4B", -1, 0, len(data), unpacked, len(pattern), pattern_at, 2, 1, 4, 0)
    out += struct.pack(">i5I4B", -1, 0, 0, 0, len(loader), loader_at, 4, 4, 2, 0)
    return pad(pad(out, 16) + bytes(code), 4) + pad(pattern, 4) + loader


def main():
    with tempfile.TemporaryDirectory() as scratch:
        work = Path(scratch)
        m68k = cargo_m68k(work)
        i386 = work / "i386.o"
        run(HOST_CLANG, "-m32", "-DHOSTED", *CFLAGS, "-o", i386)
        outputs = {
            "hello.psx": write_psx(work),
            "hello.z64": write_z64(work),
            "hello-m68k.aout": write_aout_m68k(Linked(ElfObject(m68k))),
            "hello-i386.aout": write_aout_i386(Linked(ElfObject(i386))),
            "hello.hunk": write_hunk(Linked(ElfObject(m68k), first="main")),
            "hello.pef": write_pef(work),
        }
    for name, data in outputs.items():
        (HERE / name).write_bytes(data)
        print(f"{name}: {len(data)} bytes")


if __name__ == "__main__":
    sys.exit(main())
