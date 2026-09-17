#!/usr/bin/env python3
"""Compile every synthesized struct declaration out of an exported project header
and assert offsetof(struct_N, field_0xK) == K for every field.

    kuna decompile-project <binary> -o proj-<tag> --option structsynth param
    python3 docs/features/structsynth/offsetcheck.py [--m32] proj-<tag> [proj-<tag2> ...]

`--m32` compiles with 32-bit pointers, which is what a project exported from a
32-bit target needs: the header spells a pointer member `T *`, so a host
compiler reading it back gives it 8 bytes and moves every later field.

A synthesized layout is only worth anything if a compiler reading the exported
header back puts each field where the body says it is; nothing in the body text
says whether it does.
"""
import os, re, subprocess, sys, tempfile

M32 = []


def check_header(hpath, tag):
    txt = open(hpath).read()
    # the generated typedef prelude: everything up to the first prototype block
    cut = txt.find("/* function prototypes */")
    prelude = txt[:cut if cut > 0 else len(txt)]
    prelude = "\n".join(l for l in prelude.splitlines()
                        if not l.startswith("#ifndef") and not l.startswith("#define")
                        and not l.startswith("#endif"))
    blocks = re.findall(r"^struct (struct_\d+) \{(.*?)^\};", txt, re.S | re.M)
    if not blocks:
        return (tag, 0, 0, [])
    asserts, names = [], []
    for name, body in blocks:
        names.append(name)
        for line in body.splitlines():
            m = re.search(r"\bfield_0x([0-9a-f]+)\s*(\[[^\]]*\])?\s*;", line)
            if not m:
                continue
            off = int(m.group(1), 16)
            asserts.append(
                f'_Static_assert(offsetof(struct {name}, field_0x{off:x}) == 0x{off:x},'
                f' "{name}.field_0x{off:x}");')
    src = "#include <stddef.h>\n#include <stdbool.h>\n" + prelude + "\n" + "\n".join(asserts) + "\nint main(void){return 0;}\n"
    with tempfile.NamedTemporaryFile("w", suffix=".c", delete=False) as f:
        f.write(src); path = f.name
    r = subprocess.run(["gcc", *M32, "-c", "-o", "/dev/null", path],
                       capture_output=True, text=True)
    bad = sorted(set(re.findall(r'static assertion failed: "(struct_\d+)\.', r.stderr)))
    nfail = len(re.findall(r"static assertion failed", r.stderr))
    if r.returncode and not nfail:
        print(r.stderr[:2000], file=sys.stderr)
    os.unlink(path)
    return (tag, len(names), nfail, bad)

if __name__ == "__main__":
    tot_s = tot_f = 0
    rows = []
    args = sys.argv[1:]
    if args and args[0] == "--m32":
        M32.append("-m32")
        args = args[1:]
    for arg in args:
        tag, nst, nfail, bad = check_header(os.path.join(arg, [f for f in os.listdir(arg) if f.endswith(".h")][0]), os.path.basename(arg))
        rows.append((tag, nst, nfail, bad))
        tot_s += nst
        tot_f += len(bad)
    for tag, nst, nfail, bad in rows:
        print(f"{tag:24s} {nst:4d} structs  {len(bad):3d} with a misplaced field  {nfail:4d} bad offsets  {bad[:6]}")
    print(f"TOTAL: {tot_f}/{tot_s} synthesized structs have a field the C compiler moves")
