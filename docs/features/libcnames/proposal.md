# libcnames: name locals from the library calls they meet

Issue #1076. kuna types the arguments of known library calls from its built-in
signature tables (`libproto`, `libcsigs`, `libctypes`, `win32sigs`), but those
tables carry types only, so the local that holds `open()`'s result stays `v1` and
the buffer handed to `read()` stays `v5`. IDA and Ghidra name
such locals from the callee's declared parameter names.

## Example (public fixture)

```c
int copy_file(const char *from, const char *to) {
    int in = open(from, O_RDONLY);
    int out = open(to, O_WRONLY | O_CREAT, 0644);
    char block[512];
    ssize_t got, put = 0;
    while ((got = read(in, block, sizeof block)) > 0)
        put += write(out, block, got);
    close(in);
    close(out);
    return (int)put;
}
```

`gcc -O2 -fno-inline`, `kuna decompile ./two copy_file`, main:

```c
  v1 = open(a0,0);
  v2 = open(a1,0x41,0x1a4);
  while (v3 = read(v1,v5,0x200), 1 <= v3) {
    v4 += write(v2,v5,v3);
  }
  close(v1);
  close(v2);
```

The measurement prototype (part 1 only; it spells a clash with the existing
`_00` suffix, which part 1 changes to `_2`):

```c
  fd = open(a0,0);
  fd_00 = open(a1,0x41,0x1a4);
  while (n = read(fd,v2,0x200), 1 <= n) {
    v1 += write(fd_00,v2,n);
  }
  close(fd);
  close(fd_00);
```

Part 2 would also name `v2` (`buf`, the address of a stack array passed to `read`).

## Design

**Sources of evidence**, for a local that would otherwise get a `vN` default:

1. the value a library call returns (`fd = open(...)`), named from a short,
   curated return-name table (`fd` for `open`/`socket`/`dup`/`mkstemp`, `n` for
   `read`/`write`/`fread`, `len` for `strlen`, `p` for `malloc`/`calloc`/`realloc`,
   `fp` for `fopen`/`fdopen`, `pid` for `fork`/`getpid`/`waitpid`, `c` for
   `getc`/`getchar`, `dirp` for `opendir`, ...);
2. the local passed whole as a library argument (`read(fd, ...)`), named from the
   callee's declared parameter name;
3. (part 2) the address of a stack object passed as a pointer argument;
4. (part 3) a parameter `aN` that reaches a library parameter unchanged.

Evidence follows a value through the implied copies and casts the printer folds
away, so `x = (T)call()` and `call((T)x)` count, and nothing else does: a value
used inside an expression (`malloc(len + 1)`) names nothing. Only a call whose
prototype came from a library table counts: the same imports-only rule the types
already follow, so an image's own `error()` never gets glibc's names.

**Ranking and conflicts**, deterministic:

- return-value evidence beats argument evidence (`len = strlen(s); p = malloc(len)`
  keeps `len`, not `size`);
- among arguments, the earliest call in address order, then the lowest slot;
- call-relative role names (`dest`/`src`, `s1`/`s2`, `haystack`/`needle`) name a
  local only when every library use agrees: a buffer that is one call's `dest` and
  another's `src` gets no name;
- a clash takes a numeric suffix in location order (`fd`, `fd_2`, `fd_3`), and a
  name never shadows a parameter, a global or a function the body calls.

**What always wins over a propagated name**: a user rename, a DWARF/PDB local, a
declared Symbol, and a callee prototype from DWARF or a directive (its own
parameter names keep flowing through Ghidra's `lookForFuncParamNames` path, as
today). A propagated name is a default name: it is computed in the naming pass,
never becomes a Symbol and never locks anything.

**Stable numbering and renames.** The prototype shows the hazard: after
`name fd src_fd`, the second local (`fd_00`) took the freed name `fd`, so a page
directive aimed at `fd_00` would miss. Part 1 extends `kuna_stablenames` so a
propagated spelling is carried like a number: a local keeps the spelling the
first decompile printed, and a name a rename vacated is not handed to another
local. Without a directive the output is exactly the first decompile's.

**Types never move in part 1.** Part 4 adds types along the same edges (`size_t`
for `nbytes`, `FILE *` for `stream`) only where the local's type is not locked and
the table's is strictly more specific.

**Tables.** Parameter names come from the glibc headers by a fixed rule (the
declared name with leading underscores stripped, read from a `clang
-ast-dump=json` of the platform headers, non-FORTIFY declarations): 532 of the
649 libc table entries have named declarations; the rest are zero-argument
functions or builtins. The FORTIFY `__*_chk` and `__isoc99_*` entries take their
base function's names plus `flag`/`destlen` for the extra slots, as glibc's own
`debug/` definitions name them. A unit test checks every name entry's arity
against its signature. The 147 Win32 entries take the documented SDK names
(part 5). Mach-O imports already reach the libc tables.

## Prototype measurements

Prototype: the names table above plus return names for ~80 functions,
evidence (1) and (2), ranking by return-then-earliest-call, no role-name rule.
Six stripped decbench binaries (O0 gzip, crontab, wc; O2 bzip2, init, login):

| | |
|---|---|
| functions / `vN` locals | 1,063 / 2,697 |
| locals named | 232 (8.6%) in 131 functions; 117 from a return, 112 from an argument |
| source located for | 131 (the `compiled/` twin's `.c`, same call, same slot) |
| same name as the source | 31 |
| different spelling, same meaning | 100 (`fp` vs `f`/`inStr`, `len` vs `dirlen`, `p` vs `bzf`, `pid` vs `rc`) |
| misleading | 0 of the 131; 1 elsewhere (`src` naming a buffer that is also a `strcpy` destination, the role-name rule above) |
| locals with more than one candidate | 62: 60 synonyms (`fp`/`stream`, `p`/`ptr`, `len`/`n`), 2 contradictory roles |

kuna's own DWARF-twin output names only 61 of these positions; the rest are
register temporaries its DWARF binding does not reach, which is why the
comparison is against the source.

Remaining headroom on the same six binaries: 140 more locals are stack objects
whose address goes to a library call (part 2), and 226 of 1,700 parameters reach
a library parameter unchanged (part 3).

Forced on: `make test` PARITY OK (0/675 move); 10 of 2,887 stage assertions
move, all in other features' tests that spell a `vN` this names
(`strlen(v7)` becomes `strlen(s)`). Speed: `decompile-all` of O0 gzip, three
interleaved runs, 2.77–3.58 s off vs 2.99–3.24 s on (noise).

## Parts (one PR each)

1. **libcnames**: parameter names in the libc tables + return names; P6 option
   `libcnames`, evidence (1) and (2), ranking, role-name rule, `_2` suffixes,
   stable-names carry. Default off (the 10 stage assertions above), on under
   `--mode aggressive`, so the web page and `--mode auto` below 500 KiB get it.
   Two-pass stage test, CLI and wasm tests, the page checked on a public fixture
   (rename a propagated name; nothing renumbers), decbench footprint + source
   agreement, speed.
2. Address-of-stack-object evidence (library callees only).
3. Parameters that flow unchanged into a library parameter. Separate because it
   changes the printed signature, and a named parameter must not cascade into the
   callers through `protoorder`.
4. Types along the same edges.
5. Win32 SDK names; DWARF/PDB callee names verified through the existing path.
6. The default flip, decided on the measured evidence once 1–3 have landed.

## Speed plan

One pass over the function's call specs at naming time, static sorted tables,
no new analysis. Each part runs `speedab` (default and `--mode aggressive`)
against main, budget +2%.
