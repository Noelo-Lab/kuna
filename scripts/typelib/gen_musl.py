#!/usr/bin/env python3
"""Build a kuna type library from the musl libc headers.

    scripts/typelib/gen_musl.py --arch x86_64 \
        -o decompiler/crates/kuna-console/typelib/linux-x86_64.ktl

Source: musl libc (MIT, https://musl.libc.org), the release pinned in MUSL
below. The headers are read with clang; nothing is copied by hand. For every
record the script asks the compiler for its size, alignment and member
offsets, so a packed or over-aligned record is carried as it is laid out.

The library also serves images linked against glibc. An entry is offered to a
glibc image only when the two C libraries agree on it, and that is a
measurement, not a claim: `--measure-glibc` compiles one assertion per entry
against the glibc headers installed on this machine and rewrites the
checked-in list of the entries that differ (glibc-<arch>.differs), each with
the width glibc gives the name when it has one. No glibc text enters the
library, only that list. Without the flag the checked-in list is used, so the
output does not depend on the build host.

Needs: clang (any recent release), and a C compiler with glibc headers only
for `--measure-glibc`.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import subprocess
import sys
import tarfile
import tempfile
import urllib.request

MUSL = ("1.2.6", "d585fd3b613c66151fc3249e8ed44f77020cb5e6c1e635a616d3f9f82460512a")
MUSL_URL = "https://musl.libc.org/releases/musl-{}.tar.gz"
FORMAT = 1

# Headers that declare nothing a program links against, need a compiler
# builtin, or only wrap a kernel header musl does not ship.
SKIPPED_HEADERS = {
    "alloca.h", "complex.h", "float.h", "iso646.h", "stdalign.h", "stdarg.h",
    "stdbool.h", "stdc-predef.h", "stdnoreturn.h", "tgmath.h",
    "sys/kd.h", "sys/soundcard.h", "sys/vt.h",
}
ANON = re.compile(r"\b(?:struct|union|enum) \((?:unnamed|anonymous)[^)]*\)")
WORD = re.compile(r"[A-Za-z_][A-Za-z_0-9]*")
UNREADABLE = re.compile(r"_Complex|__int128|_Float|_Atomic|\(\^")
VA_LIST = "__builtin_va_list"
INTEGERS = {1: "char", 2: "short", 4: "int", 8: "long"}


def run(cmd, **kw):
    done = subprocess.run(cmd, capture_output=True, text=True, **kw)
    if done.returncode != 0:
        sys.exit(f"{cmd[0]} failed:\n{done.stderr[-4000:]}")
    return done


def constants(asm: str, label: str):
    """The `.quad` operands the compiler emitted for the array `label`."""
    body = asm.split(f"{label}:")[-1]
    return re.findall(r"^\s*\.quad\s+(\S+)", body, re.M)


def fetch_musl(workdir: pathlib.Path) -> pathlib.Path:
    version, digest = MUSL
    tarball = workdir / f"musl-{version}.tar.gz"
    urllib.request.urlretrieve(MUSL_URL.format(version), tarball)
    if hashlib.sha256(tarball.read_bytes()).hexdigest() != digest:
        sys.exit(f"{tarball}: sha256 does not match the pinned musl {version}")
    with tarfile.open(tarball) as tar:
        tar.extractall(workdir, filter="data")
    return workdir / f"musl-{version}"


def prepare(src: pathlib.Path, arch: str, workdir: pathlib.Path):
    """The two generated headers of a musl build, and one file including the rest."""
    bits = workdir / "gen" / "bits"
    bits.mkdir(parents=True, exist_ok=True)
    alltypes = run([
        "sed", "-f", str(src / "tools/mkalltypes.sed"),
        str(src / f"arch/{arch}/bits/alltypes.h.in"), str(src / "include/alltypes.h.in"),
    ]).stdout
    (bits / "alltypes.h").write_text(alltypes)
    (bits / "syscall.h").write_text((src / f"arch/{arch}/bits/syscall.h.in").read_text())
    headers = sorted(
        str(p.relative_to(src / "include"))
        for p in (src / "include").rglob("*.h")
        if str(p.relative_to(src / "include")) not in SKIPPED_HEADERS
    )
    umbrella = workdir / "umbrella.c"
    umbrella.write_text("".join(f"#include <{h}>\n" for h in headers))
    flags = [
        f"--target={arch}-linux-musl", "-nostdinc", "-D_GNU_SOURCE", "-w",
        f"-I{src}/arch/{arch}", f"-I{src}/arch/generic", f"-I{workdir}/gen", f"-I{src}/include",
    ]
    return umbrella, flags, headers


class Model:
    """Records, typedefs and functions of one translation unit.

    `records[name]` is `(kind, probe, members)`: `probe` spells the record's
    type in C, and each member is `(name, type, path)` with `path` the member
    access from the record that reaches it.
    """

    def __init__(self, ast):
        self.records = {}
        self.typedefs = {}
        self.functions = {}
        self.enums = {}
        self.by_id = {}
        top = ast["inner"]
        naming, tags = {}, {d["id"]: d.get("name") for d in top if d["kind"] == "RecordDecl"}
        for decl in top:
            if decl["kind"] == "TypedefDecl" and not decl.get("isImplicit"):
                owned = self._owned(decl)
                if owned and tags.get(owned) in (None, decl["name"]):
                    naming.setdefault(owned, decl["name"])
        for decl in top:
            if decl["kind"] == "EnumDecl":
                signed = any('"value": "-' in json.dumps(e) for e in decl.get("inner", []))
                spelled = "int" if signed else "unsigned int"
                self.by_id[decl["id"]] = spelled
                if "name" in decl:
                    self.enums[decl["name"]] = spelled
        for decl in top:
            if decl["kind"] != "RecordDecl" or not decl.get("completeDefinition"):
                continue
            name = decl.get("name") or naming.get(decl["id"])
            if name:
                self.by_id[decl["id"]] = name
                self._record(name, decl, name if decl["id"] in naming else f'{decl["tagUsed"]} {name}')
        for decl in top:
            if decl.get("isImplicit"):
                continue
            if decl["kind"] == "TypedefDecl":
                self._typedef(decl)
            elif decl["kind"] == "FunctionDecl" and decl.get("storageClass") != "static":
                text = self._function(decl["name"], decl["type"]["qualType"])
                if text is not None:
                    self.functions.setdefault(decl["name"], text)

    @staticmethod
    def _owned(typedef):
        node = typedef
        while node.get("inner") and node["kind"] not in ("RecordType", "EnumType"):
            node = node["inner"][0]
        return node.get("decl", {}).get("id") or node.get("ownedTagDecl", {}).get("id")

    def _typedef(self, decl):
        name, qual = decl["name"], decl["type"]["qualType"]
        owned = self._owned(decl)
        if self.by_id.get(owned) == name:
            return
        text = self.by_id[owned] if owned in self.by_id and ANON.search(qual) else self._type(qual)
        if text is not None and text not in (f"struct {name}", f"union {name}"):
            self.typedefs[name] = text

    def _type(self, text):
        if UNREADABLE.search(text) or ANON.search(text):
            return None
        text = re.sub(r"\s*__attribute__\(\(.*?\)\)", "", text)
        text = re.sub(r"\b(?:restrict|__restrict|volatile)\b ?", "", text)
        text = re.sub(r"\benum ([A-Za-z_0-9]+)", lambda m: self.enums.get(m.group(1), "int"), text)
        text = text.replace("struct __va_list_tag *", VA_LIST).replace("()", "(void)")
        text = re.sub(r"\b__(size_t|ptrdiff_t)\b", r"\1", text)
        return re.sub(r"\s+", " ", text).replace("* *", "**").strip()

    def _function(self, name, text):
        text = self._type(text)
        if text is None:
            return None
        nested = text.find("(*(")
        at = nested + 2 if nested >= 0 else text.find("(")
        head = text[:at].rstrip() if nested < 0 else text[:at]
        gap = " " if nested < 0 and not head.endswith("*") else ""
        return f"{head}{gap}{name}{text[at:]}"

    def _record(self, name, decl, probe):
        members = []
        self._members(name, decl, probe, "", members)
        self.records[name] = (decl["tagUsed"], probe, members)

    def _members(self, name, decl, probe, prefix, members):
        nested = None
        for item in decl.get("inner", []):
            if item["kind"] == "RecordDecl":
                if "name" in item and item.get("completeDefinition") and item["name"] not in self.records:
                    self.by_id[item["id"]] = item["name"]
                    self._record(item["name"], item, f'{item["tagUsed"]} {item["name"]}')
                nested = item
                continue
            if item["kind"] != "FieldDecl":
                continue
            qual = item["type"]["qualType"]
            if "name" not in item:
                if nested is not None:
                    self._members(name, nested, probe, prefix, members)
                continue
            path = prefix + item["name"]
            if item.get("isBitfield") or qual.endswith("[]") or qual.endswith("[0]"):
                continue
            if ANON.search(qual) and nested is not None and "name" not in nested:
                inner = f'{name}__{item["name"]}'
                element = f"__typeof__((({probe} *)0)->{path}{'[0]' * qual.count('[')})"
                self._record(inner, nested, element)
                text = ANON.sub(inner, qual)
            else:
                text = self._type(qual)
            reserved = item["name"].startswith("__") and text not in self.records
            if text is not None and not reserved:
                members.append((item["name"], text, path))

    def integer(self, text, depth=0):
        """Is `text` an integer type (through typedefs)?"""
        words = set(WORD.findall(text))
        if "*" in text or "(" in text or "[" in text or depth > 16:
            return False
        if words <= {"unsigned", "signed", "char", "short", "int", "long", "const"}:
            return True
        return len(words) == 1 and text in self.typedefs and self.integer(self.typedefs[text], depth + 1)

    def member_names(self):
        return sorted({member for _, _, members in self.records.values() for member, _, _ in members})


def measure(model: Model, umbrella: pathlib.Path, flags, workdir: pathlib.Path):
    """`(size, alignment, [(offset, size) per member])` of every record, from the compiler."""
    lines = [f'#include "{umbrella}"']
    lines += [f"#undef {member}" for member in model.member_names()]
    lines.append("const unsigned long kuna_layout[] = {")
    for _, probe, members in model.records.values():
        lines.append(f"sizeof({probe}), _Alignof({probe}),")
        for _, _, path in members:
            lines.append(f"__builtin_offsetof({probe}, {path}), sizeof((({probe} *)0)->{path}),")
    lines.append("0};")
    probe = workdir / "layout.c"
    probe.write_text("\n".join(lines) + "\n")
    asm = run(["clang", *flags, "-S", "-o", "-", str(probe)]).stdout
    values = [int(v) for v in constants(asm, "kuna_layout")]
    layouts, at = {}, 0
    for name, (_, _, members) in model.records.items():
        end = at + 2 + 2 * len(members)
        pairs = values[at + 2:end]
        layouts[name] = (values[at], values[at + 1], list(zip(pairs[0::2], pairs[1::2])))
        at = end
    if at + 1 != len(values):
        sys.exit("layout probe: the compiler emitted an unexpected number of constants")
    return layouts


def api_members(model: Model, layouts, name, base=0):
    """The members a program names, as `(name, offset from the start of `name`, size)`.

    A member the C library reserves for itself (`__sa_handler`) is looked
    through: what it holds is reached under the public names (`sa_handler`).
    """
    _, _, members = model.records[name]
    for (member, text, _), (off, size) in zip(members, layouts[name][2]):
        if not member.startswith("__"):
            yield member, base + off, size
        elif text in model.records:
            yield from api_members(model, layouts, text, base + off)


class Glibc:
    """What the installed glibc says about the entries of a musl library."""

    def __init__(self, model: Model, layouts, headers, workdir: pathlib.Path, cc: str):
        self.model, self.layouts, self.cc = model, layouts, cc
        self.path = workdir / "glibc.c"
        self.prelude = ["#include <stddef.h>"]
        for header in headers:
            self.prelude += [f"#if __has_include(<{header}>)", f"#include <{header}>", "#endif"]

    def passing(self, feature, conditions):
        """The names whose assertion holds under one feature-test macro."""
        lines = self.prelude + [f'_Static_assert({condition}, "");' for _, condition in conditions]
        self.path.write_text("\n".join(lines) + "\n")
        out = subprocess.run(
            [self.cc, feature, "-fsyntax-only", "-fmax-errors=0", "-w", str(self.path)],
            capture_output=True, text=True,
        ).stderr
        fatal = re.search(r"fatal error: (.*)", out)
        if fatal:
            sys.exit(f"--measure-glibc: {fatal.group(1)}")
        failed = {int(m.group(1)) for m in re.finditer(rf"^{re.escape(str(self.path))}:(\d+):\d+: error", out, re.M)}
        first = len(self.prelude) + 1
        return [name for at, (name, _) in enumerate(conditions, first) if at not in failed]

    def table(self, feature, label, rows):
        lines = self.prelude + [f"const void *const {label}[] = {{"] + [f"(void *)({row})," for row in rows] + ["};"]
        self.path.write_text("\n".join(lines) + "\n")
        asm = run([self.cc, feature, "-w", "-O0", "-fno-pic", "-S", "-o", "-", str(self.path)]).stdout
        return constants(asm, label)

    def agreement(self):
        model = self.model
        conditions = []
        for name, (_, probe, _) in model.records.items():
            if "__typeof__" in probe:
                continue
            size, align, _ = self.layouts[name]
            terms = [f"sizeof({probe}) == {size}", f"_Alignof({probe}) == {align}"]
            for member, off, width in api_members(model, self.layouts, name):
                terms.append(f"offsetof({probe}, {member}) == {off}")
                terms.append(f"sizeof((({probe} *)0)->{member}) == {width}")
            conditions.append((f"S {name}", " && ".join(terms)))
        for name, text in model.typedefs.items():
            if model.integer(text):
                conditions.append((f"T {name}", f"sizeof({name}) == sizeof({text}) && (({name})-1 < 0) == (({text})-1 < 0)"))
            else:
                conditions.append((f"T {name}", f"__builtin_types_compatible_p({name}, {text})"))
        for name, text in model.functions.items():
            declared = re.sub(rf"\b{re.escape(name)}\(", "(", text, count=1)
            conditions.append((f"F {name}", f"__builtin_types_compatible_p(__typeof__({name}), __typeof__({declared}))"))
        agree = set()
        for feature in ("-D_GNU_SOURCE", "-D_DEFAULT_SOURCE"):
            passed = self.passing(feature, conditions)
            bound = [key[2:] for key in passed if key.startswith("F ")]
            symbols = self.table(feature, "kuna_symbols", bound)
            rebound = {f"F {name}" for name, symbol in zip(bound, symbols) if name != symbol}
            agree |= set(passed) - rebound
        return sorted({key for key, _ in conditions} - agree)

    def widths(self, keys):
        """What glibc holds under a differing type: an integer or a record, and how wide."""
        feature = "-D_GNU_SOURCE"
        spelled = {}
        for key in keys:
            kind, name = key.split(" ")
            if kind == "T":
                spelled[key] = name
            elif kind == "S":
                spelled[key] = self.model.records[name][1]
        integers = self.passing(feature, [(n, f"(({s})-1 < 0) || 1") for n, s in spelled.items() if n.startswith("T ")])
        sized = self.passing(feature, [(n, f"sizeof({s}) > 0") for n, s in spelled.items() if n not in integers])
        records = self.passing(feature, [(n, f"__builtin_classify_type(*({spelled[n]} *)0) >= 12") for n in sized])
        out = {}
        rows = [f"sizeof({spelled[n]})" for n in integers] + [f"(({spelled[n]})-1 < 0)" for n in integers]
        values = [int(v) for v in self.table(feature, "kuna_integers", rows)]
        for at, name in enumerate(integers):
            width, signed = values[at], values[len(integers) + at]
            if width in INTEGERS:
                out[name] = ("" if signed else "unsigned ") + INTEGERS[width]
        rows = [f"sizeof({spelled[n]})" for n in records] + [f"_Alignof({spelled[n]})" for n in records]
        values = [int(v) for v in self.table(feature, "kuna_records", rows)]
        for at, name in enumerate(records):
            out[name] = f"{values[at]},{values[len(records) + at]}"
        return out

    def version(self):
        text = run([self.cc, "-E", "-x", "c", "-"], input="#include <features.h>\n__GLIBC__ __GLIBC_MINOR__\n").stdout
        return ".".join(text.split()[-2:])


def write_library(model: Model, layouts, differs, arch, out: pathlib.Path):
    def flag(key):
        if key not in differs:
            return "*"
        return f"m={differs[key]}" if differs[key] else "m"

    lines = [f"kuna-typelib {FORMAT}", f"target linux {arch}", f"source musl {MUSL[0]}"]
    for name in sorted(model.typedefs):
        text = model.typedefs[name].replace(VA_LIST, "void *")
        lines.append(f"T {name} {flag('T ' + name)}\t{text}")
    for name in sorted(model.records):
        kind, probe, members = model.records[name]
        size, align, placed = layouts[name]
        nested = "__typeof__" in probe
        held = flag("S " + name.split("__")[0]).split("=")[0] if nested else flag("S " + name)
        lines.append(f"{'U' if kind == 'union' else 'S'} {name} {held} {size} {align}")
        lines.extend(f" {off} {member}\t{text}" for (member, text, _), (off, _) in zip(members, placed))
    for name in sorted(model.functions):
        lines.append(f"F {name} {flag('F ' + name)}\t{model.functions[name].replace(VA_LIST, 'void *')}")
    out.write_text("\n".join(lines) + "\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--arch", default="x86_64")
    ap.add_argument("-o", "--output", required=True, type=pathlib.Path)
    ap.add_argument("--musl-src", type=pathlib.Path, help="an unpacked musl tree (default: download the pinned release)")
    ap.add_argument("--measure-glibc", action="store_true", help="rewrite the list of entries glibc declares differently")
    ap.add_argument("--cc", default="cc", help="compiler with glibc headers, for --measure-glibc")
    args = ap.parse_args()
    differs_file = pathlib.Path(__file__).resolve().parent / f"glibc-{args.arch}.differs"
    with tempfile.TemporaryDirectory(prefix="kuna-typelib-") as tmp:
        workdir = pathlib.Path(tmp)
        src = args.musl_src or fetch_musl(workdir)
        umbrella, flags, headers = prepare(src, args.arch, workdir)
        ast = json.loads(run(["clang", *flags, "-fsyntax-only", "-Xclang", "-ast-dump=json", str(umbrella)]).stdout)
        model = Model(ast)
        layouts = measure(model, umbrella, flags, workdir)
        if args.measure_glibc:
            glibc = Glibc(model, layouts, headers, workdir, args.cc)
            keys = glibc.agreement()
            widths = glibc.widths(keys)
            differs_file.write_text(
                f"# Entries of the musl {MUSL[0]} library that glibc {glibc.version()} ({args.arch}) declares\n"
                "# differently, binds to another symbol, or lacks: F a function, T a typedef,\n"
                "# S a record. A type glibc holds as an integer or a record is followed by that\n"
                "# integer or by the record's size and alignment. Written by gen_musl.py\n"
                "# --measure-glibc.\n"
                + "".join(f"{key}\t{widths[key]}\n" if key in widths else f"{key}\n" for key in keys)
            )
        differs = {}
        for line in differs_file.read_text().split("\n"):
            if line and not line.startswith("#"):
                key, _, width = line.partition("\t")
                differs[key] = width
        write_library(model, layouts, differs, args.arch, args.output)
    print(f"{args.output}: {len(model.typedefs)} typedefs, {len(model.records)} records, {len(model.functions)} functions")


if __name__ == "__main__":
    main()
