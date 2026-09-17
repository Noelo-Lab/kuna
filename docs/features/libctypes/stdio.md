# `libctypes` — the stdio stream slots as typed data symbols

Everything else in this table types a stream where a CALL says what it is, which
leaves the streams themselves untyped storage. `stdin`, `stdout` and `stderr` are
named from the symbol table (`datasyms`) and carry an `undefined<size>` word, so
a function that hands `stdout` to one of the image's own helpers learns nothing
about it, and a shared object's stream — which has no symbol at its GOT slot at
all — renders `(FILE *)*dat_2bfe8`.

This step gives each stream slot its own typed data symbol, keyed on the dynamic
relocation that binds it.

```
$ kuna decompile-all <O2/coreutils/stripped/libstdbuf.so> --addr 0x1280     # before
    __fprintf_chk((FILE *)*dat_3ff8,1,v2,v5,a1);
    sub_1160(*dat_3ff8,v1);   sub_1160(*dat_3fd8,v2);   sub_1160(*dat_3fd0,v3);
                                                                            # after
    __fprintf_chk(*stderr_ptr,1,v2,v5,a1);
    sub_1160(*stderr_ptr,v1); sub_1160(*stdin_ptr,v2);  sub_1160(*stdout_ptr,v3);
```

```
$ kuna decompile-all <O0/coreutils/stripped/join> --addr 0x2bb1             # before
  long v22; // stack - 0x40                 long v23; // stack - 0x38
  v22 = sub_53b9(dat_10140,"r");            v22 = stdin;
                                                                            # after
  FILE *v22; // stack - 0x40                FILE *v23; // stack - 0x38
  v22 = (FILE *)sub_53b9(dat_10140,"r");    v22 = stdin;
```

Ground truth for `join`'s `main` is `FILE *fp1, *fp2`; `xfopen` is the image's own
helper and nothing else in the function says what it returns.

## Two shapes, and they hold different things

The relocation is both the address and the evidence. It says the name belongs to
the C library rather than to a global the program happens to spell `stdout`, so
an image that defines its own is never reached.

| shape | slot | type | name |
|---|---|---|---|
| `R_*_COPY`, symbol defined, `st_size` = pointer width | the `.bss` word the run-time loader fills with libc's `FILE *stdout` | `FILE *` | `stdout` |
| `R_*_GLOB_DAT`, symbol **undefined** | the GOT word holding `&stdout` | `FILE **` | `stdout_ptr` |
| `R_*_GLOB_DAT`, symbol defined | the same image's own copy slot | — | declined |

```
000000000000c088  R_X86_64_COPY      stdout@GLIBC_2.2.5 + 0   coreutils fmt, .bss
000000000002bf38  R_X86_64_GLOB_DAT  stdout@GLIBC_2.2.5 + 0   libselinux.so.1, .got
```

The `_ptr` name is not decoration. The GOT word is not the stream, and naming it
`stdout` would make the emitted `*stdout` read as an indirection the program does
not perform. The declined third row is the mixed executable — shadow `useradd`
copy-relocates `stdout` and `stderr` AND keeps a GOT slot pointing at each copy,
while its `stdin` is undefined and reaches the program only through the GOT.
Those defined GOT slots are already filled by the relocation pass and folded by
RELRO; a second claim on them would say nothing new.

Linked ELF only, on the four architectures whose relocation numbering the loader
knows (x86-64, AArch64, i386, ARM). A spelling that occurs more than once in
`.dynsym` is declined outright. The `FILE` the slots point at is the one
`named_aggregate` hands the rest of the table, so `opaque` prints
`stdout->field_0x28`, `glibc` prints `stdout->_IO_write_ptr`, and an image whose
own debug info holds a different `FILE` contributes no stream symbol at all
rather than a contradictory one.

## Where it is committed, and why there

`AnalysisOutput::typed_data` (new), committed in `commit_analysis_output` as arm
4b with `typelock | namelock`:

* **after** the DWARF data globals (arm 1a) and the detected string literals (arm
  4), so a richer source keeps any address it claims — the `occupied` guard is
  the same one those arms use;
* **before** the loader's own `.dynsym` data stream (arm 4a), whose untyped
  `undefined<size>` naming of the same slot then stands down on its own guard.

The typelock is the point. Without it the two renderings of one global disagree
inside a single binary: a function that also calls `__overflow(FILE *, int)`
infers the stream and one that only hands `stdout` to a helper does not.

The arm also answers to `datasyms`. That option's contract is that `off` restores
the raw `dat_<addr>` rendering for every global the DWARF pass does not name —
to see which names came from the image's own tables, or because a hostile one
plants misleading names — and a stream slot's name is a `.dynstr` string like any
other. So naming a data object stays `datasyms`'s call and `libctypes` only
decides what the named object is; there is no half of it to keep, because the
type rides on the symbol. `kuna-console`'s own
`option_datasyms_off_restores_dat_addr` is the pin, and pass 10 of the stage test
says the same thing on this feature's fixture.

## What it moves in emitted C

`kuna decompile-all` over twelve stripped binaries, this branch against
origin/main's build (`39fdc804`). Full classification, with the residue printed
in full, in `corpus-hunk-classification-stdio.txt`.

| | lines |
|---|---:|
| a GOT slot gains its name, and the `(FILE *)` cast it needed disappears | 31 |
| a GOT slot gains its name | 7 |
| a declaration takes `FILE *` / `FILE **` | 5 |
| a cast appears on a genuine `FILE *` global | 1 |
| a return type becomes `FILE *` | 1 |
| **anything else** | **0** |

Ten of the twelve binaries are byte-identical end to end, and every one of those
is a plain executable: there the type already arrived by inference from a typed
stdio call in the same function. The reach this step extends is the function that
makes no such call, and the shared object, whose stream never had a name.

The wrong-output lens, checked across all twelve: `fflush(stdout)`,
`fputs_unlocked(s,stdout)`, `fwrite_unlocked(…,stdout)`, `putc_unlocked`'s
inlined `__overflow(stdout,c)` refill and every `*(char **)&stdout->field_0x28`
store are byte-identical in both arms. No statement, address or control-flow edge
moves anywhere.

One shape is worth naming because it costs a declaration. On the i386
`i386_pie_nl` fixture, `main`'s `v2 = *dat_9fd8; rpl_fclose(v2)` — where `v2` was
a 4-byte `unsigned int` shared with other values — becomes its own
`FILE *v3 = *stdin_ptr`. One more local, with the right type on it, and the whole
local numbering shifts behind it.

## Type recovery

`scripts.decbench.typesweep`, decbench main's `TypeMatchMetric`,
`DECBENCH_NO_CACHE=1`, `--baseline-only` run twice with `KUNA_BIN` pinned first to
a release build of `origin/main` (`39fdc804`, its own worktree) and then to this
branch's. 8 projects × `O0` / `O2` / `O2 -fno-inline` = **444 slices, 10,748
scored functions**.

| | main | branch |
|---|---:|---:|
| `type_match` PERFECT | 959 | 959 (+0) |
| aggregate `type_match` | 3037.05 | 3037.19 (+0.14) |
| moved ONTO perfect | | 0 |
| moved OFF perfect | | 0 |
| improved | | **1** |
| worsened | | **0** |

Control: the main arm agrees with the tree's published `type_match` on 9,505 of
10,748 (88.44%, the tree having been scored before this campaign's merges), and
the branch arm on 9,504 — the one function that differs is the one that moved.

The single moved row is `coreutils::O0::join::main`, .7857 → .9286 (the two
`FILE *fp` stack slots above). Every other project and optimisation level is
identical to four decimal places.

**The metric is nearly blind to this and that is the expected shape.** decbench
scores `variables[]` — arguments and stack symbols. A global is not one, and
neither is the register local that carries it; only a stream that reaches a stack
slot can score, and only where the ground truth for that slot is a stream. tar's
`argp_parse` gains `FILE *v70` on exactly such a slot and does not move the
number, because that function's ground truth does not describe it. So the sweep
is the guard, not the case: it says nothing regressed *that this metric scores*,
and the case for the step is the hunk classification above.

## Speed

Interleaved main/branch pairs on identical commands, minimum and median of 15
each, alternating which build runs first. The box was shared throughout, which is
why the minimum is the number to read.

| case | min main | min branch | min Δ | median Δ |
|---|---:|---:|---:|---:|
| `fmt decompile-all` | 4142.6 ms | 4099.3 ms | -1.04% | +0.83% |
| `useradd decompile-all` | 6584.3 ms | 6689.6 ms | +1.60% | +2.42% |
| `libstdbuf.so decompile-all` | 116.5 ms | 116.3 ms | -0.20% | +2.78% |
| `fmt functions (load only)` | 146.5 ms | 146.1 ms | -0.26% | -2.90% |

Every minimum is inside ±2%, well inside the +5% budget.

The work added is one walk of the dynamic relocation table per program, filtered
to three names, at load.

## Tests

`tests/stages/kuna-libctypes.xml` passes 6-9 — the executable and the shared
object built from one source (`libctypes_streams_x86_64.c`), each once `off` and
once `opaque` — with assertions #16-#25, plus pass 10 for the `datasyms off` interaction (#26):

* `pending` reads two stream fields and calls nothing, so the relocation is the
  only evidence in the image (#16-#19);
* `flush_mark` stores THROUGH the stream pointer and must still store to the same
  address (#20/#21);
* `hand_off` reads the stream OBJECT into one of the image's own functions and
  must still read it whole — `sink(stdin)`, never `sink(*stdin)` (#22/#23);
* `say` passes it to a libc call the table already typed, which must not move
  (#24), and whose library form only loses the cast the untyped slot needed
  (#25).

Six unit tests in `protos/kuna_libctypes/tests.rs`: the two shapes' addresses,
names and indirection depth; that neither borrows the other's naming; that the
slots share the table's own interned `FILE`; that `glibc` reaches them; and that
a relocatable object and a PE yield nothing.

`tests/cli/structdefs-json-types-array.json` moved: `main` of the `i386_pie_nl`
fixture now references two libc shells rather than one, so the per-function
`types` array carries `FILE` ahead of `option`. The probe pins both.
