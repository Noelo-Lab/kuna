# Lane `ida` — what IDA Pro does instead of a cast

Read-only census. Branch `campaign/castcensus`, worktree `/home/mahaloz/kwt/castcensus`.
No engine change, no PR.

**Headline.** kuna's excess casts are almost entirely the *price of kuna's better type*, not
a symptom of a worse one. IDA types 52% of parameters as a 64-bit integer where kuna types
60% as a pointer; IDA then does address arithmetic in integers (one cast at the load) while
kuna does it in integers too *after converting the pointer back* (two casts at the load).
The single largest lever — 3,771 casts at 2,944 pointer-based sites, half the whole gap —
is to keep pointer
arithmetic in pointer terms. Nothing in the top five levers requires weakening a type, and
one of them (records for `void *` bases) *improves* the type while removing two casts.

---

## 1. The paired corpus

Both decompilers' whole-binary output carries `// Function: <name> @ 0x<addr>`, so functions
match by **address**, name-independently.

- Rival C: `/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/<opt>/<proj>/decompiled/ida_<bin>.c`
- kuna: re-run on **current main** (796939d54, release build in this worktree) with
  `kuna decompile-all <stripped> --json`, one JSON per target in `.scratch/cur/`.
  The 2026-09-11 kuna `.c` files are an old build and are used only for the drift column.

**19 targets** (fmt/ls/sort/du × O0,O2,O2-noinline; grep, gzip, diff, find, bzip2 × O0,O2),
**2,121 address-matched functions**.

| | casts | lines | per 1k lines | per function |
|---|---|---|---|---|
| IDA 9.2 | 18,214 | 100,032 | 143.9 | 8.6 |
| kuna 2026-09-11 | 16,590 | 77,086 | 215.2 | 7.8 |
| **kuna main 796939d54** | **25,659** | 71,091 | **243.8** | **12.1** |

(The three-way row uses the 1,843 functions all three emitted; the two-way rows above use
all 2,121. Numbers: `.scratch/remeasure.py`.)

Two things to note before anything else:

- **kuna got *more* cast-heavy during the type campaign**, not less: 16,590 → 17,332 on the
  identical 1,843 functions (+4.5% in count, +13% per line). Better pointee types mean more
  explicit conversions at the use sites, which is exactly the user's complaint.
- The gap is **not** uniform. kuna emits **fewer** casts than IDA in 665 of 2,121 functions,
  the same in 665, more in 791. The 200 worst functions carry 8,994 casts of gap — **83% of
  all the positive gap**. This is a small number of idioms repeated, not a diffuse style
  difference.

Worst offenders (`opt/bin/kuna-name @addr — ida → kuna`):
`O0/bzip2/sub_b9c1 @0xb9c1 535→2131`, `O0/bzip2/sub_116df 854→2078`,
`O0/bzip2/sub_8660 101→450`, `O0/bzip2/sub_7aad 63→307`, `O2/grep/sub_cec0 422→622`.
bzip2 and gzip O0 dominate (62 of the top 200).

**Counting method.** `.scratch/castlib.py`: a `(TYPE *…)` immediately followed by an
expression start, with the type drawn from a builtin vocabulary plus every type name declared
in the file, rejecting call sites and parenthesised expressions. It counts IDA's
`_QWORD/_BYTE/__int64/_UNKNOWN` spellings and kuna's `int4/uint8` (reliable-mode) spellings
alike. It is deliberately symmetric; it finds *more* IDA casts than the orchestrator's
pre-measured figures did (fmt O0: 158/1k for both, vs the reported 121 ida / 165 kuna), so
the gap reported here is the conservative one.

---

## 2. Where the casts are — bucket by syntactic position

Every cast on both sides, classified by context (`.scratch/classify.py`, `main_analysis.py`).

| bucket | IDA | kuna | delta |
|---|---|---|---|
| `L_scalar_var` — `(T)v`, scalar target | 5,073 | **12,933** | **+7,860** |
| `B_deref_addrof` — `*(T *)&v` | 123 | 1,158 | +1,035 |
| `A_deref_computed` — `*(T *)(x + k)` | 6,216 | 6,954 | +738 |
| `E_ptr_from_const` — `(T *)0x1234` | 0 | 735 | +735 |
| `G_ptr_from_call` — `(T *)f()` | 605 | 945 | +340 |
| `J_scalar_const` | 1 | 8 | +7 |
| `F_ptr_from_addrof` — `(T *)&x` | 441 | 116 | −325 |
| `K_scalar_from_call` — `(T)f()` | 728 | 339 | −389 |
| `I_int_from_addr` — `(long)&x` | 529 | 69 | −460 |
| `D_deref_var` — `*(T *)p` | 943 | 382 | −561 |
| `H_ptr_repoint` — `(T *)p` | 3,555 | 2,020 | **−1,535** |
| **total** | **18,214** | **25,659** | **+7,445** |

kuna is already **better** on five buckets, and by a lot: it re-points fewer pointers
(`H`, −1,535) and converts far fewer addresses to integers at call sites (`I`, −460) because
it actually has the prototype. The entire deficit lives in `L_scalar_var`.

### Splitting `L_scalar_var` by what the operand is declared as

Resolving each cast's operand against the declarations in the same function
(`.scratch/scalar2.py`, `decls.py`):

| relation of cast type to operand's declared type | IDA | kuna | delta |
|---|---|---|---|
| **`ptr` — operand is a pointer, cast is to an integer** | 1,101 | **5,362** | **+4,261** |
| **`widen-samesign` — widen, same signedness (C-implicit)** | 85 | **2,568** | **+2,483** |
| operand is a compound expression | 1,894 | 4,320 | +2,426 |
| unresolved operand | 292 | 731 | +439 |
| `narrow` — truncation (load-bearing) | 1,194 | 955 | −239 |
| `sign-only` — same width, sign change (load-bearing) | 919 | 813 | −106 |
| `widen-signflip` | 44 | 45 | +1 |
| `identity` — cast to the operand's own type | 29 | 42 | +13 |

kuna's top `(cast) on (declared)` pairs are unambiguous:
`(long)` on `void *` **2,879**, `(long)` on `int` 1,261, `(long)` on `long *` 1,110,
`(long)` on `unsigned long *` 615, `(long)` on `char *` 295.

kuna emits **fewer** truncating and sign-changing casts than IDA. It emits 4,261 more
pointer→integer conversions and 2,483 more casts that C would perform on its own.

---

## 3. The five things IDA does instead, with counts and examples

### 3.1 It never converts a pointer to an integer to add an offset — 3,771 casts

Measured over the whole paired corpus:

| idiom | IDA | kuna |
|---|---|---|
| `*(T *)( (long)p + … )` — a **pointer-declared** variable converted to an integer inside a deref-cast operand | **22** | **3,771** |
| `*((T *)p + k)` — cast the pointer, then index | **2,202** | **0** |
| `p->field` | 632 | 3,418 |
| `*(T *)(x + k)` where `x` is an integer | 7,542 | 7,266 |

IDA emits this construct 22 times in 100k lines — and all 22 are my matcher firing on
`(int)a1[30]`, i.e. a cast of an *element*, not of `a1`; the true IDA count is zero. kuna
emits it **3,771** times. Restricting to the simple `*(T *)((long)p + rest)` shape with a
bare identifier base gives 3,236 sites, of which **2,944 have a base kuna itself declared as
a pointer**:

| kuna's base | sites | what IDA prints | casts saved per site |
|---|---|---|---|
| `void *` (no pointee) | 1,817 | `a1->field_x`, or `*(T *)(a1 + k)` with `a1` an integer | 1, or **2** with a record |
| typed pointer, access width ≠ pointee width | 1,095 | `*((T *)p + k)` | 1 |
| typed pointer, access width = pointee width | 23 | `p[k]` | **2** |

The same function, same statement, `bzip2 @0x6cae` (`add_pair_to_block`):

```c
ida : *(char *)((int)(*(int *)(a1 + 108))++ + *(long long *)(a1 + 64)) = v3;   // 3 casts
kuna: *(char *)(*(long *)((long)a0 + 0x40) + (long)*(int *)((long)a0 + 0x6c)) = v2;  // 5
```

kuna's two extra casts are exactly the two `(long)a0`. And `bzip2 @0x7aad`
(`unRLE_obuf_to_output_FAST`), where IDA's `a1` is `int *`:

```c
ida : a1[15] = *(int *)(*((long long *)a1 + 394) + 4LL * (unsigned int)a1[15]);
kuna: *(unsigned int *)((long)a0 + 0x3c) = *(unsigned int *)((unsigned long)*(unsigned int *)((long)a0 + 0x3c) * 4 + a0[0x18a]);
```

`a1[15]` and `*(unsigned int *)((long)a0 + 0x3c)` are the same 4-byte load at +60: **0 casts
vs 2**.

This is the biggest single item and it costs nothing in type quality. `*(T *)((long)p + K)`
→ `*(T *)((char *)p + K)` is already −1 and value-identical (`char *` arithmetic is
byte-wise). `*((T *)p + K/sizeof(T))` when `K % sizeof(T) == 0` is also −1 and is what IDA
prints. `p->field` when a record exists is −2.

### 3.2 It gives every referenced static address a name — 817 sites

`E_ptr_from_const`: IDA 0, kuna 735 casts over 817 constant-address operands (322 distinct
addresses). kuna prints the raw address with a cast where IDA prints a symbol that needs
none:

```
kuna (char *)0x21da0            ida  byte_21DA0 / inName     fprintf(stderr,"...",dat_229f0,(char *)0x21da0);
kuna (unsigned long *)0x21180   ida  qword_21180              sub_415c((unsigned long *)0x21180,&v11);
kuna (unsigned int *)0x21148    ida  dword_21148              sub_102cd(getenv("DU_BLOCK_SIZE"),(unsigned int *)0x21148,(long *)0x21170);
kuna (long *)0x180d0            ida  &unk_180D0               if (dat_18120 == (long *)0x180d0) {
```

kuna *does* have the `dat_<addr>` scheme — `dat_229f0` appears in the same statement as
`(char *)0x21da0`. It just doesn't reach every referenced address (a name is only minted for
some), and an address with no symbol becomes an integer literal that then needs a cast.
Fixing the naming removes the cast as a side effect and makes the C readable.

### 3.3 It relies on C's own implicit widening — ~750 strictly-removable sites

`widen-samesign` is 2,568 in kuna against **85** in IDA. Not all are removable: `(long)v * 0x18`
with `v` an `int` forces the multiply to 64-bit and *must* stay (IDA keeps those too — that is
most of its 85, e.g. `((long long)i << 7) + a1`). The strictly-removable subset, each verified
against the surrounding declaration:

| site | count | why it is safe |
|---|---|---|
| assignment RHS, LHS declared as exactly the cast type | **238** | `v24 = (unsigned long)v29;` with `v24` `unsigned long` |
| `return (T)v` with the function returning exactly `T` | **57** | `return (unsigned long)v1;` |
| cast to the operand's own declared type (`identity`) | **42** | `(unsigned int)a2` with `a2` `unsigned int` |
| inner cast of a stack `(A)(B)x` with width(B) ≥ width(A) | **410** | `v3 = (long)(long)v5;` |
| **total** | **747** | none of these can change a value |

IDA's counts for the same four: 2, 0, 29, 267.

Stacked casts overall: kuna 1,152 adjacent pairs vs IDA 546. kuna's worst are
`(long)(int)(unsigned int)*(unsigned char *)(...)` — a byte load zero-extended to 64 bits in
three steps where IDA writes the load and lets promotion do the rest, and
`v3 = (long)(long)v5` (76 occurrences of a self-stacked cast).

### 3.4 It splits the variable, or uses a width macro, for sub-word writes — 1,035 gap

`B_deref_addrof` (`*(T *)&v`): IDA 123, kuna 1,158. IDA covers ~562 of those with
`LOBYTE(v7) = a2;`, `BYTE1(v5) = BYTE1(a4) | 8;`, `HIDWORD(v41) = 17;`, `LODWORD(v2) =
putchar_unlocked(10);` (full histogram in §4). kuna's equivalent is
`*(unsigned char *)&v7 = a2;`.

**This one is mostly not worth copying as-is.** The macros are not C — they need
`ida_defs.h` — and they are no less explicit than kuna's form; the cast count drops without
the type getting better. The part worth copying is the *other* part: where IDA has split the
storage into two variables and simply assigns `v7b = a2`, there is no partial write at all.
That is a real variable-recovery improvement, and it is the same lever as `structlocals`.

### 3.5 It applies a prototype at the call — ~300 sites

kuna's call-argument casts: 788 scalar + 554 pointer-repoint + 492 constant-address + 81
address-of = 1,915. IDA's: 972 + 364 + 298 + 70 = 1,704. IDA is *worse* here in the scalar
column, for a bad reason (§5), but it wins on the pointer columns because a known prototype
means the argument needs no conversion at all. kuna already recovers prototypes well
(`H_ptr_repoint` −1,535, `I_int_from_addr` −460 are exactly that win). The remaining
opportunity is narrow: casts whose type equals the recovered parameter type, which C would
apply implicitly. Conservative estimate **−300**.

---

## 4. The constructs the campaign already touched

- **Struct field vs `*(T *)((char *)p + k)`** — kuna already emits 5.4× more `->` than IDA
  (3,418 vs 632). structsynth works. The gap is the **1,817 `void *` bases** where no record
  was synthesized at all; those are the same sites as §3.1's best case. `bzip2/sub_6871`
  takes `void *a0` and does `*(unsigned int *)((long)a0 + 0x6c) = 0;` — IDA calls the same
  parameter `long long a1` and writes `*(_DWORD *)(a1 + 108) = 0;`.
- **`char *` / `void *` parameters** — kuna's 60%-pointer parameter policy is why kuna leads
  type_match; it is also the direct cause of the `(long)p` tax. Keep the policy, fix the
  printing.
- **Call arguments against a known prototype** — covered in §3.5; kuna is already ahead on
  the pointer half.
- **Sub-word reads and writes** — reads are fine (`A_deref_computed` +738 is small and mostly
  the §3.1 idiom). Writes are §3.4.
- **Sign changes** — kuna emits **fewer** than IDA (813 vs 919). Not a problem. These are
  load-bearing: a sign change alters `/`, `%`, `>>` and every comparison. Do not touch them.
- **Pointer/integer mixing** — §3.1. This is the whole story.
- **Truncations** — kuna 955 vs IDA 1,194. kuna is not over-truncating either.

IDA's sub-word macro histogram (16-binary sample): `LODWORD` 231, `LOBYTE` 158, `BYTE1` 47,
`HIBYTE` 34, `LOWORD` 27, `HIDWORD` 18, `COERCE_DOUBLE` 15, `BYTE2..BYTE6` 21, `HIWORD` 4,
`__PAIR128__` 2 — 562 total, kuna 0.

---

## 5. Where IDA is wrong, and where copying it would lose information

**IDA emits 195 casts that truncate an address.** In `I_int_from_addr`, 194 of IDA's 555
address-to-integer casts narrow a 64-bit address to 32 bits and one narrows to 8:

```c
ida : sub_9E0A((unsigned int)&v7, v12, 0, (unsigned int)&v3, ..., (long long)&v6);
ida : sub_11AB0(stdout, (char)"Paul Eggert");
ida : printf(v3, (unsigned int)"GNU coreutils");     // fmt/main, varargs
```

Compiled, these change the value. kuna has 7 comparable sites (and those are `(int)str[i]`,
which is fine). So IDA's headline total is *inflated* by 195 casts it should not have, and
its varargs policy — casting every unknown vararg to `unsigned int` — is a thing to avoid,
not to copy. The true kuna-vs-IDA gap is slightly wider than 7,445.

**Copying IDA's parameter typing would be a regression, not a fix.** 52% of IDA's parameters
are a bare 64-bit integer against kuna's 25%; that is precisely how IDA avoids `(long)p`.
Retyping kuna's `void *a0` to `long a0` would delete ~2,900 casts and simultaneously destroy
the pointer facts that put kuna first on type_match. It is the definition of hiding a cast
rather than removing it.

**IDA's `a1[15]` hides the field identity.** When the object is really a struct and IDA types
the parameter `int *`, `a1[15]` is cast-free but says "element 15 of an int array" where the
truth is "the field at +60". Value-preserving, information-losing at the type level. Prefer
`p->field_0x3c`, which is also cast-free.

**IDA's width macros are not C.** `LOBYTE(v)=x` needs `ida_defs.h`; kuna's output is meant to
compile. If the partial write survives at all, kuna's form is the right one — the fix is to
make the partial write disappear by splitting the variable.

**What must never be dropped.** kuna's 955 `narrow`, 813 `sign-only` and 45 `widen-signflip`
casts all change the value or the operation. The rule holds: a cast may go only because the
type became right, or because the conversion is one C performs anyway on the same types.

---

## 6. Ranked list — what to copy, and what it is worth

Estimates are casts removed on this 2,121-function paired corpus (kuna baseline 25,659,
gap to IDA +7,445).

| # | behaviour to copy | estimated removal | risk |
|---|---|---|---|
| 1 | **Never print `(long)p` for a pointer `p`.** Emit `*(T *)((char *)p + K)`, or `*((T *)p + K/sizeof T)` when `K` divides, in place of `*(T *)((long)p + K)`. | **−3,771** (one per occurrence; 2,944 of them are the simple `p + K` shape, and 1,817 become −2 if #2 also lands) | none — `char *` arithmetic is byte-wise and value-identical |
| 2 | **Synthesize a record for the `void *` bases that #1 leaves.** 1,817 sites whose base is an untyped pointer with constant field offsets. | **−1,800 additional** (the second cast at each site), and a *better* type | the known `struct_N` naming cost under decbench; already an accepted trade |
| 3 | **Name every referenced static address** so a global operand is a symbol, not `(T *)0x1234`. | **−817** | none; improves readability too |
| 4 | **Delete the casts that are already provably redundant**: assign-RHS with LHS == cast type (238), `return` with return type == cast type (57), identity casts (42), removable inner of a stacked pair (410). | **−747** | none — verified against the declarations at each site |
| 5 | **Split partially-written variables** instead of `*(T *)&v = x` (do *not* adopt `LOBYTE`). | **−400 to −1,000** | needs real variable splitting; the macro shortcut is rejected |
| 6 | **Drop call-argument casts whose type equals the recovered parameter type.** | **−300** | only where the prototype is trusted |
| | **total** | **≈ −7,800 of a +7,445 gap** (the levers overlap at the `void *` sites, so the realistic floor is parity with IDA, not below it) | |

Levers 1–4 are ~5,300 casts of pure printing and naming work with no semantic exposure at
all, and lever 2 makes the type strictly better while removing casts. None of the six
requires weakening a declared type, and the one behaviour that would (IDA's `long long`
parameters) is explicitly excluded.

---

## 7. Reproducing

All in `/home/mahaloz/kwt/castcensus/.scratch/`, run from that directory with plain `python3`:

```
castlib.py       cast detection + function splitting by address
decls.py         declaration/parameter parsing, width + signedness tables, relation()
classify.py      syntactic bucket (A..L) + call-argument position
census.py        full 89,805-function pairing over all 770 binaries (old kuna vs ida)
runkuna.sh       current-main `kuna decompile-all --json` over the 22 targets -> cur/
remeasure.py     the three-way per-1k table
main_analysis.py the paired bucket table; writes rows_cur.json
scalar2.py       the declared-type relation table; writes scalar2.json
widen.py         widening-cast contexts
```

Build used: `make binaries` at 796939d54 in this worktree, `SLEIGHHOME`/`KUNA_SPECS` pointed
at the worktree's linked `specs/`.

---

## 8. Implementation anchors (for whoever acts on §6)

Not part of the census, but found while checking that a lever is not already shipped:

- **`PrintCOptions::hide_exts` is already `true` by default** (`p9_emit/printc.rs:497`). The
  2,568 widening casts in §3.3 are the ones `is_extension_cast_implied` declines
  (`p9_emit/cast.rs:622`, port of `CastStrategyC::isExtensionCastImplied`, cast.cc:249). Its
  first branch is the one that matters here: when the extension's output is **explicit** —
  that is, it got its own declared variable — the predicate falls straight through to
  `false`. That is exactly the §3.3 "assign-RHS with LHS == cast type" (238) and
  "`return` with return type == cast type" (57) populations. Upstream declines them
  unconditionally; accepting them when the explicit variable's declared type *equals* the
  extension result type is value-preserving by construction.
- The other declines are: the read op is not one of the listed INT arithmetic/compare ops, the
  other operand is neither explicit nor a small constant, or the two metatypes differ.
- **`structsynth`'s own symptom line** in `docs/options.md` is literally "the body is a wall
  of `*(int *)(p + 0x18)` casts over one pointer" — §3.1 and §4 are that symptom, still
  unfixed at 1,817 `void *`-based sites.
- **`arraynotation`** (default on) already renders standalone pointer arithmetic as
  `&base[index]`; it is not reaching the `*(T *)((long)p + K)` form.
- `nocastprinting` exists and is the thing the user explicitly does not want: it suppresses
  casts wholesale rather than removing them.

## 9. Measurement hygiene

**This worktree is shared with another castcensus lane**, which committed
`271cf0d07 wip(castcensus): scratch cast-attribution instrumentation (NOT for merge)` at
18:10 and rebuilt `target/release` at 18:19. My `make binaries` finished ~18:04 and every
`decompile-all` run completed by 18:10, so all numbers above come from a build of clean
`796939d54`.

Verified independently anyway: re-running `O0/fmt` and `O0/bzip2` through the **main tree's**
untouched release `kuna` (built 2026-09-21 at `69a947074`) gives 296 vs 296 casts on fmt's
191 common functions and 6,805 vs 6,775 on bzip2's 154 — a 0.4% drift attributable to the
three PRs merged between the two commits. `.scratch/verify_*.json`.

The sibling lane's instrumentation is env-gated (`KUNA_CASTCENSUS`, `census::` in
`p9_emit/cast.rs`) and emits nothing when the variable is unset, so it cannot have moved any
of these counts even where it was compiled in.
