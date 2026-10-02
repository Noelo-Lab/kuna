#!/usr/bin/env python3
"""Generate `globalptrstores_x86_64.o`: one function that stores one of two
pointers to a global 400 times, each store followed by a store through another
pointer, a load of the global, and an early return.

Every store keeps its COPY (chapter 03, `kuna_pointeestorekeep`), so the global
has 400 values, 200 COPYs from each pointer, and a 400-way join at the return.
`tests/cli/repeated-global-pointer-stores-decompile-fast.json` times the
decompile (issue #818). Regenerate with:

    python3 globalptrstores_x86_64.py > globalptrstores_x86_64.c
    gcc -O2 -c globalptrstores_x86_64.c -o globalptrstores_x86_64.o
"""

N = 400
print("int *gi;\nint big(int *p, int *r, int **pp) {\n  int x = 0;")
for i in range(N):
    v = "p" if i % 2 == 0 else "r"
    print(f"  gi = {v}; *pp = {v} + 1; x += gi[{i % 4}]; if (x == {i * 3 + 1}) return x;")
print("  return x;\n}")
