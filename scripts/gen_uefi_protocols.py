#!/usr/bin/env python3
"""Generate the UEFI protocol / PPI interface table of the `uefi` analysis pass.

Source: EDK2 (https://github.com/tianocore/edk2), tag edk2-stable202505,
BSD-2-Clause-Patent. Reads the public headers under MdePkg/Include and
MdeModulePkg/Include and writes, for every protocol or PPI GUID a header
declares (`extern EFI_GUID gXxxGuid;`), the layout of its interface struct in
the field vocabulary of `analyzers/uefi/types.rs`:

    = EFI_DISK_IO_PROTOCOL gEfiDiskIoProtocolGuid
    Revision UINT64
    ReadDisk fn EFI_STATUS(EFI_DISK_IO_PROTOCOL* This, UINT32 MediaId, ...)

A struct another one holds by value, and the PI tables named in EXTRA, are
written without a GUID; a pointer to any other struct is written `VOID*`. A member this script cannot lay out (a union, a
bit-field, a packed or unknown by-value type) ends the struct there, so every
offset that is written is exact.

    python3 scripts/gen_uefi_protocols.py /path/to/edk2 \
        > decompiler/crates/kuna-analysis/src/analyzers/uefi/protocols.txt
"""
import re
import sys
from pathlib import Path

TAG = "edk2-stable202505"
ROOTS = ("MdePkg/Include", "MdeModulePkg/Include")
EXTRA = ("EFI_MM_SYSTEM_TABLE", "EFI_SMM_SYSTEM_TABLE2")
# Laid out by hand in types.rs; referenced by name, never written here.
BUILTIN = {
    "EFI_SYSTEM_TABLE", "EFI_BOOT_SERVICES", "EFI_RUNTIME_SERVICES", "EFI_DXE_SERVICES",
    "EFI_PEI_SERVICES", "EFI_PEI_PPI_DESCRIPTOR", "EFI_PEI_NOTIFY_DESCRIPTOR",
    "EFI_CONFIGURATION_TABLE", "EFI_SIMPLE_TEXT_INPUT_PROTOCOL", "EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL",
}
SCALARS = {
    "VOID", "UINT8", "UINT16", "UINT32", "UINT64", "UINTN", "INT8", "INT16", "INT32", "INT64",
    "INTN", "BOOLEAN", "CHAR8", "CHAR16", "EFI_STATUS", "EFI_HANDLE", "EFI_EVENT", "EFI_TPL",
    "EFI_PHYSICAL_ADDRESS", "EFI_GUID", "EFI_TABLE_HEADER",
}
BASE_ALIASES = {
    "RETURN_STATUS": "EFI_STATUS", "GUID": "EFI_GUID", "EFI_VIRTUAL_ADDRESS": "UINT64",
    "PHYSICAL_ADDRESS": "UINT64", "EFI_LBA": "UINT64", "EFI_PEI_FILE_HANDLE": "VOID*",
    "EFI_PEI_FV_HANDLE": "VOID*", "EFI_IPv4_ADDRESS": "UINT8[4]", "EFI_IPv6_ADDRESS": "UINT8[16]",
    "EFI_MAC_ADDRESS": "UINT8[32]",
}
QUALIFIERS = {"IN", "OUT", "OPTIONAL", "CONST", "const", "volatile", "EFIAPI", "struct"}


def strip_comments(text):
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    return re.sub(r"//[^\n]*", " ", text)


class Headers:
    def __init__(self, root):
        self.fn = {}
        self.structs = {}
        self.aliases = dict(BASE_ALIASES)
        self.enums = set()
        self.guids = {}
        for sub in ROOTS:
            for path in sorted((root / sub).rglob("*.h")):
                self.read(path)
        for name, tag in list(self.aliases.items()):
            if tag == "_" + name and tag in self.structs:
                del self.aliases[name]
                self.aliases[tag] = name
        for tag, name in list(self.aliases.items()):
            if tag in self.structs and name not in self.structs:
                self.structs[name] = self.structs[tag]

    def read(self, path):
        text = strip_comments(path.read_text(errors="replace"))
        packed = [m.start() for m in re.finditer(r"#pragma\s+pack\s*\(\s*(?:push\s*,\s*)?1\s*\)", text)]
        unpacked = [m.start() for m in re.finditer(r"#pragma\s+pack\s*\(\s*(?:pop)?\s*\)", text)]

        def is_packed(at):
            start = max((p for p in packed if p < at), default=None)
            return start is not None and not any(start < u < at for u in unpacked)

        for m in re.finditer(r"typedef\s+([\w\s\*]+?)\s*\(\s*(?:EFIAPI\s*)?\*\s*(\w+)\s*\)\s*\(([^;{}]*)\)\s*;", text):
            self.fn.setdefault(m.group(2), (m.group(1), m.group(3)))
        for m in re.finditer(r"typedef\s+enum\s*\w*\s*\{[^{}]*\}\s*(\w+)\s*;", text):
            self.enums.add(m.group(1))
        for m in re.finditer(r"typedef\s+struct\s+(\w+)\s+(\w+)\s*;", text):
            if m.group(1) != m.group(2):
                self.aliases.setdefault(m.group(1), m.group(2))
        for m in re.finditer(r"typedef\s+(\w+)\s+(\w+)\s*;", text):
            if m.group(1) not in ("struct", "union", "enum"):
                self.aliases.setdefault(m.group(2), m.group(1))
        for m in re.finditer(r"typedef\s+(\w+)\s*\*\s*(\w+)\s*;", text):
            self.aliases.setdefault(m.group(2), "VOID*")
        for m in re.finditer(r"\b(typedef\s+)?struct\s*(\w*)\s*\{", text):
            end = self.close(text, m.end())
            if end is None:
                continue
            tail = re.match(r"\s*(\w*)\s*;", text[end + 1:])
            names = [n for n in (m.group(2), tail.group(1) if tail and m.group(1) else "") if n]
            body = None if is_packed(m.start()) else text[m.end():end]
            for name in names:
                self.structs.setdefault(name, body)
            if len(names) == 2:
                self.aliases.setdefault(names[0], names[1])
        if path.parent.name in ("Protocol", "Ppi"):
            for m in re.finditer(r"extern\s+EFI_GUID\s+(\w+)\s*;", text):
                self.guids.setdefault(m.group(1), text)

    @staticmethod
    def close(text, at):
        depth = 1
        for i in range(at, min(len(text), at + 20000)):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    return i
        return None

    def resolve(self, name):
        for _ in range(8):
            if name in SCALARS or (name in self.structs and name not in self.aliases):
                return name
            nxt = self.aliases.get(name)
            if nxt is None:
                return name
            name = nxt
        return name


class Emitter:
    def __init__(self, headers):
        self.h = headers
        self.done = {}
        self.order = []
        self.active = set()

    def split(self, decl):
        """`IN CONST T **Name[3]` -> (T, stars, Name, array)."""
        array = None
        m = re.search(r"\[\s*(\w*)\s*\]\s*$", decl)
        if m:
            if not m.group(1).isdigit():
                return None
            array, decl = int(m.group(1)), decl[:m.start()]
        if ":" in decl or "(" in decl:
            return None
        toks = [t for t in re.findall(r"\w+|\*", decl) if t not in QUALIFIERS]
        stars = toks.count("*")
        words = [t for t in toks if t != "*"]
        if not words or len(words) > 2:
            return None
        return words[0], stars, (words[1] if len(words) == 2 else ""), array

    def value_type(self, base, stars, by_value_ok):
        base = self.h.resolve(base)
        if base.endswith("*") or "[" in base:
            if "[" in base:
                return "VOID" + "*" * stars if stars else (base if by_value_ok else None)
            return base + "*" * stars
        if base in self.h.enums:
            base = "UINT32"
        if base in SCALARS:
            if base == "EFI_TABLE_HEADER" and stars == 0 and not by_value_ok:
                return None
            return base + "*" * stars
        if base in self.h.fn:
            return "VOID" + "*" * stars if stars else self.fn_type(base)
        if base in BUILTIN:
            return base + "*" * stars if stars else None
        if base in self.h.structs:
            interface = base.endswith(("_PROTOCOL", "_PPI")) or base in EXTRA
            if stars and interface and (base in self.active or self.struct(base)):
                return base + "*" * stars
            if stars == 0 and by_value_ok and base not in self.active and self.struct(base) == "whole":
                return base
        return "VOID" + "*" * stars if stars else None

    def fn_type(self, name):
        ret, args = self.h.fn[name]
        parts = self.split(ret + " _")
        out = self.value_type(parts[0], parts[1], False) if parts else None
        if out is None or out.startswith("fn ") or out == "FUNC":
            return "FUNC"
        rendered = []
        args = args.strip()
        if args in ("", "VOID"):
            return f"fn {out}()"
        for arg in args.split(","):
            arg = arg.strip()
            if arg == "...":
                rendered.append("...")
                continue
            parts = self.split(arg)
            if parts is None:
                return "FUNC"
            ty = "VOID*" if parts[3] is not None else self.value_type(parts[0], parts[1], False)
            if ty is not None and (ty.startswith("fn ") or ty == "FUNC"):
                ty = "VOID*"
            if ty is None or ty == "VOID":
                return "FUNC"
            rendered.append(f"{ty} {parts[2]}".strip())
        return f"fn {out}({', '.join(rendered)})"

    def struct(self, name):
        """Lay `name` out once; 'whole' when every member was written."""
        name = self.h.resolve(name)
        if name in self.done:
            return self.done[name]
        body = self.h.structs.get(name)
        if body is None or name in self.active:
            return None
        self.active.add(name)
        fields, whole = [], True
        if "{" in body:
            body, whole = body[:body.index("{")].rsplit(";", 1)[0] + ";", False
        for decl in body.split(";")[:-1]:
            decl = decl.strip()
            if not decl:
                continue
            parts = None if decl.startswith("#") else self.split(decl)
            ty = None
            if parts and parts[2]:
                ty = self.value_type(parts[0], parts[1], True)
                if ty is not None and parts[3] is not None:
                    ty = None if ty.startswith("fn ") or ty == "FUNC" or "[" in ty else f"{ty}[{parts[3]}]"
            if ty is None or ty == "VOID":
                whole = False
                break
            fields.append((parts[2], ty))
        self.active.discard(name)
        if not fields:
            self.done[name] = None
            return None
        self.done[name] = "whole" if whole else "part"
        self.order.append((name, fields))
        return self.done[name]


def norm(name):
    return name.replace("_", "").lower()


def main():
    root = Path(sys.argv[1])
    headers = Headers(root)
    emit = Emitter(headers)
    by_norm = {}
    for name in sorted(list(headers.structs) + list(headers.aliases), key=lambda n: (n.startswith("_"), n)):
        by_norm.setdefault(norm(name), name)
    owners = {}
    for guid, text in sorted(headers.guids.items()):
        stem = guid[1:-4] if guid.startswith("g") and guid.endswith("Guid") else None
        if stem is None:
            continue
        target = by_norm.get(norm(stem))
        if target is None and stem.endswith("ServiceBindingProtocol"):
            target = "EFI_SERVICE_BINDING_PROTOCOL"
        if target is None:
            local = sorted({m.group(1) for m in re.finditer(r"struct\s+_(\w+_(?:PROTOCOL|PPI))\s*\{", text)})
            if len(local) == 1 and len(re.findall(r"extern\s+EFI_GUID\s+\w+", text)) == 1:
                target = local[0]
        if target is None:
            continue
        target = headers.resolve(target)
        if target in BUILTIN or emit.struct(target):
            owners.setdefault(target, []).append(guid)
    for name in EXTRA:
        emit.struct(name)
    print(f"# EDK2 {TAG} MdePkg + MdeModulePkg interface structs (BSD-2-Clause-Patent).")
    print("# Generated by scripts/gen_uefi_protocols.py; do not edit.")
    for name in sorted(BUILTIN & owners.keys()):
        print(" ".join(["=", name] + owners[name]))
    for name, fields in emit.order:
        print(" ".join(["=", name] + owners.get(name, [])))
        for fname, ty in fields:
            print(f"{fname} {ty}")


if __name__ == "__main__":
    main()
