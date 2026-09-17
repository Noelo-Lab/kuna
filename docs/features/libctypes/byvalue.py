"""Count named-libc-aggregate values in BY-VALUE positions in a `decompile-all`
dump, and show which of them are read through a piece accessor.

    python3 docs/features/libctypes/byvalue.py <dir>

`<dir>` holds one `<tag>.off.c` / `<tag>.on.c` pair per binary, each the output
of `kuna decompile-all <bin> --option libctypes off` and of the default arm.
Prints, per binary, the by-value local DECLARATIONS of a shell name in each arm,
then every such declaration that is read as `vN._<off>_<size>_`.
"""

import glob
import os
import re
import sys

NAMES = [
    "DIR", "FILE", "dirent", "group", "mbstate_t", "option", "passwd",
    "pthread_mutex_t", "sigaction", "sigset_t", "sockaddr", "stat", "termios",
    "timespec", "timeval", "tm",
]
DECL = re.compile(r"^\s{2,}(" + "|".join(NAMES) + r")\s+([A-Za-z_]\w*)\s*(\[\s*\d+\s*\])?\s*;")
HDR = re.compile(r"^// Function: (\S+) @ (0x[0-9a-fA-F]+)")


def scan(path):
    """[(func_addr, var, type, is_array, piece_reads)] for one dump."""
    out, cur, declared, body = [], None, {}, []

    def flush():
        txt = "\n".join(body)
        for var, (ty, arr) in declared.items():
            pieces = sorted(set(re.findall(re.escape(var) + r"\._\d+_\d+_", txt)))
            out.append((cur, var, ty, arr, pieces))

    for line in open(path, errors="replace"):
        m = HDR.match(line)
        if m:
            flush()
            cur, declared, body = m.group(2), {}, []
            continue
        d = DECL.match(line)
        if d:
            declared[d.group(2)] = (d.group(1), bool(d.group(3)))
        body.append(line)
    flush()
    return out


def main(root):
    tot_off = tot_on = arrays = 0
    for off in sorted(glob.glob(os.path.join(root, "*.off.c"))):
        tag = os.path.basename(off)[:-6]
        a, b = scan(off), scan(off[:-6] + ".on.c")
        tot_off += len(a)
        tot_on += len(b)
        arrays += sum(1 for r in b if r[3])
        print(f"{tag:14s} off={len(a):3d} on={len(b):3d}")
        for addr, var, ty, arr, pieces in b:
            tail = f"  {len(pieces)} piece reads {pieces[:8]}" if pieces else ""
            print(f"      + {ty} {var}{'[]' if arr else ''} @ {addr}{tail}")
    print(f"TOTAL off={tot_off} on={tot_on} (of which arrays: {arrays})")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".scratch/corpus")
