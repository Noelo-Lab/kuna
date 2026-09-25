"""callbacktype census, step A: which functions have their address handed to a
declared libc callback slot, over the campaign's 444 slices.

objdump the stripped binary, walk a bounded window back from each call to a
known callback-taking import, and read the ABI register of the callback slot.
Cross-check against the DWARF twin: is the found target a function decbench
scores, and what does its declaration say?
"""
from __future__ import annotations

import collections
import json
import os
import re
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

CMP2 = ("int", ("void *", "void *"))
CMP3 = ("int", ("void *", "void *", "void *"))
SIG = ("void", ("int",))
THREAD = ("void *", ("void *",))
VOIDPTRFN = ("void", ("void *",))
VOIDFN = ("void", ())
ONEXIT = ("void", ("int", "void *"))
FTWFN = ("int", ("char *", "void *", "int"))
NFTWFN = ("int", ("char *", "void *", "int", "void *"))
GLOBERR = ("int", ("char *", "int"))
TWALKFN = ("void", ("void *", "int", "int"))
DIRFILT = ("int", ("void *",))
DIRCMP = ("int", ("void *", "void *"))

# name -> [(slot, signature)]
TABLE = {
    "qsort": [(3, CMP2)],
    "qsort_r": [(3, CMP3)],
    "bsearch": [(4, CMP2)],
    "lfind": [(4, CMP2)],
    "lsearch": [(4, CMP2)],
    "atexit": [(0, VOIDFN)],
    "__cxa_atexit": [(0, VOIDPTRFN)],
    "on_exit": [(0, ONEXIT)],
    "signal": [(1, SIG)],
    "__sysv_signal": [(1, SIG)],
    "bsd_signal": [(1, SIG)],
    "sigset": [(1, SIG)],
    "pthread_create": [(2, THREAD)],
    "pthread_once": [(1, VOIDFN)],
    "pthread_key_create": [(1, VOIDPTRFN)],
    "pthread_atfork": [(0, VOIDFN), (1, VOIDFN), (2, VOIDFN)],
    "scandir": [(2, DIRFILT), (3, DIRCMP)],
    "scandirat": [(3, DIRFILT), (4, DIRCMP)],
    "ftw": [(1, FTWFN)],
    "nftw": [(1, NFTWFN)],
    "tsearch": [(2, CMP2)],
    "tfind": [(2, CMP2)],
    "tdelete": [(2, CMP2)],
    "twalk": [(1, TWALKFN)],
    "tdestroy": [(1, VOIDPTRFN)],
    "glob": [(2, GLOBERR)],
}
# aliases objdump may print (versioned / 64-bit variants)
ALIAS = {"qsort_r": "qsort_r", "glob64": "glob", "ftw64": "ftw", "nftw64": "nftw",
         "scandir64": "scandir", "sigaction": None}

SLOTREG = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"]
ALIASES = {
    "rdi": {"rdi", "edi", "di", "dil"}, "rsi": {"rsi", "esi", "si", "sil"},
    "rdx": {"rdx", "edx", "dx", "dl", "dh"}, "rcx": {"rcx", "ecx", "cx", "cl", "ch"},
    "r8": {"r8", "r8d", "r8w", "r8b"}, "r9": {"r9", "r9d", "r9w", "r9b"},
}

ALLREG = {}
for _k, _v in {
    "rax": "rax", "rbx": "rbx", "rcx": "rcx", "rdx": "rdx", "rsi": "rsi",
    "rdi": "rdi", "rbp": "rbp", "rsp": "rsp",
}.items():
    ALLREG[_k] = _k


def full_reg(r: str) -> str:
    for k, v in ALIASES.items():
        if r in v:
            return k
    tail = {"eax": "rax", "ax": "rax", "al": "rax", "ah": "rax",
            "ebx": "rbx", "bx": "rbx", "bl": "rbx", "bh": "rbx",
            "ebp": "rbp", "esp": "rsp"}
    if r in tail:
        return tail[r]
    if re.fullmatch(r"r\d+[dwb]", r):
        return r[:-1]
    return r


INSN = re.compile(r"^\s+([0-9a-f]+):\t(?:[0-9a-f ]+\t)?([a-z0-9.]+)\s*(.*)$")
CALL = re.compile(r"^([0-9a-f]+)\s+<([^>]+)>")
LEA_RIP = re.compile(r"^lea\s+(?:0x)?(-?[0-9a-fx]+)\(%rip\),%(\w+)")
MOV_IMM = re.compile(r"^mov[lq]?\s+\$0x([0-9a-f]+),%(\w+)")


def disasm(path):
    out = subprocess.run(["objdump", "-d", "--no-show-raw-insn", str(path)],
                         capture_output=True, text=True).stdout
    rows = []
    for line in out.splitlines():
        m = re.match(r"^\s+([0-9a-f]+):\t(.*)$", line)
        if not m:
            continue
        rows.append((int(m.group(1), 16), m.group(2).strip()))
    return rows


def scan(path, entries):
    """[(callee_name, slot, target_addr, call_addr)] for every callback slot
    whose argument is a constant code address."""
    rows = disasm(path)
    # last constant written to each 64-bit register, and its instruction index
    hits = []
    held = {}
    for i, (pc, text) in enumerate(rows):
        mnem = text.split()[0] if text else ""
        ops = text[len(mnem):].strip()
        if mnem in ("call", "callq", "jmp", "jmpq"):
            m = re.match(r"^(?:0x)?([0-9a-f]+)\s*<([^>]+)>", ops)
            name = None
            if m:
                sym = m.group(2)
                if "+" not in sym:
                    name = sym.split("@")[0]
            base = ALIAS.get(name, name)
            if base in TABLE:
                for slot, sigt in TABLE[base]:
                    v = held.get(SLOTREG[slot])
                    if v is not None:
                        hits.append((base, slot, v, pc, sigt))
            held = {}
            continue
        if mnem.startswith("j") or mnem in ("ret", "retq", "hlt", "ud2"):
            held = {}
            continue
        m = re.match(r"^lea\s+[^,]+\(%rip\),%(\w+)\s*#\s*([0-9a-f]+)", text)
        if m:
            held[full_reg(m.group(1))] = int(m.group(2), 16)
            continue
        m = MOV_IMM.match(text)
        if m:
            held[full_reg(m.group(2))] = int(m.group(1), 16)
            continue
        m = re.match(r"^mov[lqw]?\s+%(\w+),%(\w+)$", text)
        if m:
            src, dst = full_reg(m.group(1)), full_reg(m.group(2))
            v = held.get(src)
            if v is None:
                held.pop(dst, None)
            else:
                held[dst] = v
            continue
        # any other instruction: invalidate the registers it could write
        dst = text.rsplit(",", 1)[-1].strip()
        dm = re.match(r"^%(\w+)$", dst)
        if dm:
            full = next((k for k, v in ALIASES.items() if dm.group(1) in v), None)
            if full:
                held.pop(full, None)
    return hits


def other_refs(path, target, entries):
    """How many OTHER code sites materialise `target`, and is it stored in a
    data word of the image?"""
    return None


def work(task):
    project, opt, stem, unstripped, a2n = task
    stripped = unstripped.replace("/compiled/", "/stripped/")
    if not os.path.exists(stripped):
        return None
    entries = set(a2n)
    try:
        hits = scan(stripped, entries)
    except Exception as e:  # noqa: BLE001
        return {"slice": f"{project}::{opt}::{stem}", "error": str(e)[:120]}
    return {"slice": f"{project}::{opt}::{stem}", "bin": stripped,
            "hits": [{"callee": c, "slot": s, "target": t, "at": a,
                      "name": a2n.get(t), "scored": t in entries,
                      "sig": list(sg[1]), "ret": sg[0]}
                     for (c, s, t, a, sg) in hits]}


def main():
    sys.path.insert(0, "/home/mahaloz/kwt/callbacktype")
    sys.path.insert(0, "/home/mahaloz/kwt/callbacktype/docs/decbench/typecampaign/final-g")
    import pindb
    os.environ.setdefault("DECBENCH_PIN", "/home/mahaloz/kwt/_final-d/db625")
    pindb.pin()
    from scripts.decbench import config, typesweep as ts
    projects = ["coreutils", "grep", "gzip", "diffutils", "bzip2", "findutils",
                "tar", "shadow", "x"]
    opts = ["O0", "O2", "O2-noinline"]
    slices = ts.collect_slices(config.results_root(), projects, opts)
    print(f"slices: {len(slices)}", file=sys.stderr)
    out = []
    with ProcessPoolExecutor(max_workers=12) as ex:
        futs = [ex.submit(work, s) for s in slices]
        for f in as_completed(futs):
            r = f.result()
            if r:
                out.append(r)
    Path(sys.argv[1]).write_text(json.dumps(out, indent=1))
    tot = sum(len(r.get("hits", [])) for r in out)
    print(f"slices scanned {len(out)}, callback-slot hits {tot}", file=sys.stderr)


if __name__ == "__main__":
    main()
