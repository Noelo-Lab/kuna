# typedepth — analysis

## 1. The measured symptom

kuna's DWARF type mapper truncates ordinary C declarations to `void`. Counting the
sites directly out of `.debug_info` (simulating both live variants of
`build_datatype` against every defined subprogram's parameters, `DW_OP_fbreg`
locals and every CU-top-level `DW_OP_addr` global) over six `-g` C binaries:

| binary | locals/params truncated | globals truncated | prototype params truncated |
|---|---:|---:|---:|
| coreutils `fmt` O0 | 10 | 4 | 0 |
| coreutils `fmt` O2 | 1 | 4 | 0 |
| grep O0 | 96 | 20 | 0 |
| bzip2 O0 | 6 | 3 | 0 |
| zlib `minigzip` O0 | 114 | 18 | 0 |
| gzip O0 | 11 | 6 | 1 |
| **total** | **238** | **55** | **1** |

**294 sites, and the new rule resolves every one of them** (0 still truncated).
What they look like in the emitted C:

```c
/* before */  void **p;                    /* const char *const *p            */
/* before */  anon_struct *firstpos;       /* position *firstpos              */
/* before */  int4 add_envopt(int4 *argcp,void ***argvp,char *envvar_name)
/* before */  cmp = *(void **)&(&comparison_function_table)[(uint8)option * 8];
/* before */  dat_337e8 = ZEXT18(mask) * ...;   /* the global `unibyte_mask`  */
```

## 2. The root cause — an unfaithful reduction, not a deliberate cap

`MAX_TYPE_DEPTH = 3`, counted over **every** hop of the `DW_AT_type` chain, the
transparent `typedef`/`const`/`volatile`/`restrict` links included. Its comment
cites Ghidra's `DWARFDataTypeImporter.trackRecursion`. That is not what upstream
does:

```java
private boolean trackRecursion(long id, int delta) {
    Integer count = recursionTrackingOffsetToLoopCount.getOrDefault(id, 0);
    count = count + delta;
    switch (count) { case 3: Msg.error(...); return false; }
    recursionTrackingOffsetToLoopCount.put(id, count);
    return true;
}
```

Upstream counts **re-entries of one DIE offset** and refuses the third. That fires
only on a cycle: an acyclic chain visits each offset once, however long it is.
kuna's reduction conflates "the same DIE again" with "a deep but finite chain", and
four DIEs is ordinary C:

| declaration | chain | hops |
|---|---|---:|
| `const char *const *` | ptr -> const -> ptr -> const -> char | 5 |
| `const size_t *` | ptr -> const -> typedef -> base | 4 |
| `char *const []` | array -> const -> ptr -> char | 4 |
| `char ***` | ptr -> ptr -> ptr -> char | 4 |
| `const Account *const` (a `const` method's `this`) | const -> ptr -> const -> class | 4 |

**The budget could not simply be raised.** With no cycle detection under it, any cap
is simultaneously too low for real code and unable to bound a self-referential
chain — it only converts an infinite loop into a wrong answer. The question "what is
the depth budget protecting against?" has an answer, and the answer is *type
cycles*, which upstream detects rather than approximates.

## 3. What was already fixed, and by whom

The brief for this work described `char **authors` / `idx_t *pn` / `mbstate_t *ps`
rendering `void *` in coreutils `fmt`. **On today's `main` those three are already
correct**, and it is worth recording why: #264 (`cppproto`) added
`strip_qualifiers`, which collapses the transparent hops before the depth test —
but only on its own `cpp` path. Because `cppproto` is default-on and rebuilds
*every* subprogram's prototype (C included) with `cpp = true`, binding it by address
over the name-keyed one, plain-C **prototypes** inherited the fix as a side effect.
The same is true of the C++ `const`-method `this` the brief listed as still open:
`Account::available`'s `this` resolves on `main` today.

What #264 did **not** reach, and what this PR is:

1. **Stack locals** — `collect_fbreg_locals` maps with `cpp = false`, no collapse,
   depth budget live. The 238 sites above.
2. **Globals** — the CU-top-level `DW_OP_addr` arm sizes the symbol from the same
   `cpp = false` mapper; a truncated element type sizes the global at **one byte**,
   so the covering-`SymbolEntry` query misses and the global renders `dat_<addr>` or
   address-taken. 55 sites.
3. **Structural depth** — three *pointer* hops exhaust the budget even with the
   qualifiers collapsed. gzip's `add_envopt(int *argcp, char ***argvp, ...)` is the
   witness: `void ***` on `main` today.
4. The `cppproto off` path, which reverts all of the above to the raw budget.

## 4. What the whole-binary sweep found (and cost)

Sweeping `decompile-all` with the gate off vs on over four `-g -O0` C binaries and
classifying every changed line found one **regression** the witness could not have
shown, and it is the reason `intern_aggregate` exists:

```
- anon_struct *next;              zlib inflate_table
+ void *next;
```

Extending the qualifier collapse to the C path also extends the *alias naming* it
carries (an anonymous aggregate is interned under the typedef it was reached
through, so a local `mbstate_t` is not fused with every other unnamed struct under
`anon_struct`). zlib's `inftrees.h` contains

```c
typedef struct { unsigned char op, bits; unsigned short val; } code;
```

and kuna registers a **core type named `code`** (`set_core_type("code", 1,
TYPE_CODE, …)`). `get_type_struct("code")` therefore hits an existing type of
another kind, the factory refuses the redefinition, the aggregate builds as `None`,
and the pointer arm's `.or_else(get_type_void)` degrades `code *next` to
`void *next` — the exact failure this pass exists to remove. (The same collision is
already live on `main` for the *parameter*: `code *base` renders `void *base` today
via #264's cpp path.) The fix: when the borrowed name does not yield an aggregate,
fall back to the anonymous name. Only an aggregate that had **no name of its own**
falls back, so no new name is ever asserted for a genuinely named type.

Everything else in the sweep is an improvement, in four shapes:

| shape | example |
|---|---|
| `void *` -> concrete | `void **p` -> `char **p` |
| `void *` -> concrete, expression reshaped | `s = &((void *)s)[1]` -> `s = &s[1]` |
| `anon_struct` -> the typedef name | `anon_struct *firstpos` -> `position *firstpos` |
| global extent corrected | `&(&crc_table)[i*4]` -> `&crc_table[i*4]`; `dat_337e8` -> `unibyte_mask` |

The one class that is *noisier* rather than cleaner, and is reported as such: a
local now typed as a named **opaque** struct (`z_stream_s *strm`) renders its field
accesses through a cast (`*(char **)strm` where `(char *)*strm` stood before),
because kuna maps an aggregate to a named opaque with no fields. That is the
existing `cppproto` seam (struct field population is a separate increment), not a
new one; the type asserted is the DWARF truth in both cases.

## 5. The benchmark reality — and why the standard corpus reads zero

**The decbench corpus is stripped.** `results/full_run/<opt>/<project>/stripped/*`
is what was decompiled and scored, and `strip` removes `.debug_*` wholesale — so the
DWARF pass produces nothing there and this change cannot move a single scored
function. Verified rather than assumed: `decompile-all` off vs on over the stripped
zlib `minigzip` differs by **0 lines**.

That also re-frames the C `type_match` standing (kuna 5.17%, behind angr 7.29% and
ida 5.55%): that gap is measured with **no debug info present**, so it is a gap in
*inference*, not in DWARF consumption, and no amount of DWARF fidelity addresses it.
The honest measurement for this change is the unstripped arm — decbench's own
`type_match` metric run over the same projects' `compiled/` (DWARF-bearing) copies —
recorded in `record.json`.
