# Original Octeon atomic fixtures

`memory.S` contains original functions for global and stack memory, conditional
stores, repeated ordering operations, partial overlaps, and full-width integer
uses of addresses. They use Octeon II encodings shared with III. The globals
are intentionally at `0x2000` for the XML binary images. Each function starts
on a 32-byte boundary, with the first loaded at `0x1000`.

The snapshot functions retain a pointer read before an atomic or an ordered
barrier, including a conditional update and a later arithmetic use. The two
zero-neighbor functions read unescaped stack slots on either side of the ZCB
operand. A cache-block operation may affect those slots even though their own
addresses never escape. Their tests retain conservative guards without assuming
a particular frame alignment or interpreting every guarded local as zero.
The console companion also compiles and executes the unchanged recovered
pointer-snapshot C with available native C compilers at O0/O2, checking old
values and updated memory, including wrapping arithmetic and both branches.

The stage and Rust tests contain the instruction bytes and need no cross
compiler, SDK, QEMU or external binaries. To regenerate the encoding report:

```sh
python3 tests/fixtures/octeon-atomics/build.py \
  --assembler mips64-linux-gnuabi64-as --objcopy objcopy \
  --output /tmp/octeon-atomics.json
```

`provenance.json` records source and text hashes and the tool versions, GNU as
2.30 and GNU objcopy 2.47.20260726; the same tools reproduce it byte for byte.
`--objcopy` needs big-endian MIPS ELF support. These original fixtures are
covered by the repository's license. No third-party implementation or firmware
is included.

The tests assert memory preservation and an uncertain SC result, independently
of any executor's reservation choices. SAA/SAAD start at Plus; LA and ZCB/ZCBT
start at II. Generic MIPS collision controls and operand-capture checks live
in `kuna-sleigh/tests/octeon_core.rs`. No hardware concurrency, cache, MMU,
scratch/LMT or device timing claim follows from these tests.
