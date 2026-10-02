#!/usr/bin/env python3
"""Generate `globalptrstores_x86_64.o`: one function that stores a pointer to a
global 200 times, each store followed by a load through the pointer, a store
through another pointer, a load through the global and an early return.

Every store keeps its COPY (chapter 03, `kuna_pointeestorekeep`), so the global
has 200 values, and each early return makes the rule pool ask about all of
them again. `tests/cli/repeated-global-pointer-stores-decompile-fast.json`
times the decompile (issue #818). Regenerate with:

    python3 globalptrstores_x86_64.py > globalptrstores_x86_64.c
    gcc -O2 -c globalptrstores_x86_64.c -o globalptrstores_x86_64.o
"""

N = 200
print("int *gi;\nint big(int *p, int k, int **pp) {\n  int x = 0; int *q;")
for i in range(N):
    print(f"  q = p + (k + {i % 7}); gi = q; x += q[{i % 5}]; *pp = p + {i % 3}; "
          f"x += gi[{i % 4}]; if (x == {i * 3 + 1}) return x;")
print("  return x;\n}")
