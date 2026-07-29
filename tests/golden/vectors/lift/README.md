# Golden lift fixtures (C++ oracle -> Rust SLEIGH runtime)

Per-instruction raw p-code lifts emitted by the C++ decompiler's
`golden lift` / `golden liftctx` console commands
(`decompiler/cpp/kuna_goldengen.cc`), one fixture per pinned corpus test.
The Rust SLEIGH runtime must reproduce each fixture body byte-for-byte:
decode the same bytes under the same context and emit the same ops.

Generated at C++ tree rev `468ec05` by `tools/rust-port/gen_lift.py`.

## Regeneration

```bash
make binaries        # rebuild decomp_dbg after any C++ change
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_lift.py
```

Determinism gate -- two consecutive runs must diff empty:

```bash
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_lift.py --out-dir /tmp/lift2
diff -r tests/golden/vectors/lift /tmp/lift2
```

## Fixture format

Each `<xml-stem>.txt` is a `#`-comment header (corpus path, sla path relative
to `specs/`, the lift points as `space:byte-offset[=symbol]`) followed by the
verbatim console captures:

1. One `golden liftctx <first-point>` dump:
   - `archid` -- full normalized architecture id (language id + compiler);
   - `languageid` -- the SLEIGH language id (archid minus the compiler field);
   - `slafile` -- .sla file name from the matching `.ldefs` entry;
   - `endian` -- `big` or `little`;
   - `contextsize` -- number of 32-bit words in a context blob;
   - `space <name> index=<i> size=<addrsize> wordsize=<w> bigendian=<0|1>` --
     one per address space, in space-index order;
   - `context <name>=<value>` -- EVERY registered context variable, in name
     order, with the value in effect at the first lift point (a `.pspec`
     `<context_set>` paints values per address range, so the defaults blob
     would read all-zero -- the per-address values are what decoding uses).
2. One `golden lift <point> 80` dump per lift point (the image's `<symbol>`
   addresses in document order, deduplicated; symbol-less images use their
   `<bytechunk>` start addresses):
   - `insn <space>:<offset> <length>` per instruction; `<length>` is
     `Translate::oneInstruction`'s fall-through offset, i.e. it INCLUDES
     delay-slot bytes (SPARC/MIPS), and the next instruction is decoded at
     `<offset>+<length>`;
   - `  <OPNAME> <out> <in0> <in1> ...` per emitted p-code op, in emission
     order.  Varnodes render as `(space,0x<offset>,size)`; a missing output
     renders as `-`.  Opcode names are the canonical enum names
     (`get_opname`, with the `ZPULL`/`SPULL` staleness fix of
     `kuna_goldengen.cc`).
   - Spaceid pointer constants -- input 0 of `LOAD`/`STORE` -- print the
     space NAME instead of a raw offset (`(const,ram,8)`): in C++ that
     offset is the heap address of the `AddrSpace` object, the only
     nondeterministic value in a raw lift.  The Rust port stores the space
     index instead (LOSS-015, see docs/history.md); both engines must
     normalize to the name when emitting this line.
   - A decode failure ends the dump with
     `lifterror <class> <space>:<offset> <message>` where `<class>` is one of
     `dataunavail` / `unimpl` / `baddata` / `lowlevel` (DataUnavailError,
     UnimplError, BadDataError, other LowlevelError).  Running off the end of
     a bytechunk or into non-code bytes is expected -- the error line
     (message text included) is part of the fixture.

Addresses in fixture content are always BYTE offsets.  (The generator
converts them to word offsets when typing console commands -- the console's
address parser multiplies by the space's wordsize -- but nothing word-based
appears in the fixtures themselves.)

## Pinned corpus subset

One test per distinct SLEIGH language id, spanning every processor family the
corpus offers.  Regeneration must keep this pin (the fixture set is part of
the gate):

| corpus test | language id |
| `tests/datatests/floatprint.xml` | x86:LE:64:default:gcc |
| `tests/datatests/promotecompare.xml` | x86:LE:32:default:gcc |
| `tests/stages/gh7139-x86-16-disp16-signed.xml` | x86:LE:16:Real Mode |
| `tests/datatests/condexesub.xml` | ARM:LE:32:v8:default |
| `tests/datatests/ccmp.xml` | AARCH64:LE:64:v8A:default |
| `tests/datatests/gp.xml` | MIPS:BE:32:default:default |
| `tests/datatests/lzcount.xml` | PowerPC:BE:32:default:default |
| `tests/stages/gh6990-returnpair.xml` | sparc:BE:32:default:default |
| `tests/stages/gh8817-v850indbranch.xml` | V850:LE:32:default |
| `tests/datatests/boolless.xml` | 8051:BE:16:default:default |
| `tests/stages/gh8844-avr-rjmpwrap.xml` | avr8:LE:16:default |
| `tests/datatests/readvolatile.xml` | 68000:BE:32:MC68020:default |
| `tests/stages/gh9203-condexe-loopcopy.xml` | RISCV:LE:64:default |
| `tests/stages/gh8913-addcarrychain.xml` | 6502:LE:16:default |
| `tests/stages/gh9001-hcs12brn.xml` | HCS12:BE:24:default |
| `tests/datatests/skipnext2.xml` | Toy:BE:32:builder.align2:default |

Corpus language ids NOT pinned (available for future fixtures):
x86:LE:64:default / x86:LE:64:default:windows / x86:LE:32:default /
x86:LE:32:default:win(dows) / ARM:LE:32:v7 / ARM:LE:32:v8 / ARM:LE:32:v8-m /
AARCH64:LE:64:v8A / MIPS:LE:32:default:default / MIPS:BE:32:default /
PowerPC:BE:32:e500 / sparc:BE:32:default / RISCV:LE:32:default /
8051:BE:16:default / 8085:LE:16:default / PIC-24F:LE:24:default:default /
68000:BE:32:default.
