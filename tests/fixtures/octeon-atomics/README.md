# Original Octeon atomic fixtures

`memory.S` contains original functions for global and stack memory, conditional
stores, repeated ordering operations, partial overlaps, and full-width integer
uses of addresses. They use Octeon II encodings shared with III. The globals
are intentionally at `0x2000` for the XML binary images. Each function starts
on a 32-byte boundary, with the first loaded at `0x1000`.

The stage and Rust tests contain the instruction bytes and need no cross
compiler, SDK, QEMU or external binaries. To regenerate the encoding report:

```sh
python3 tests/fixtures/octeon-atomics/build.py \
  --assembler mips64-linux-gnuabi64-as --objcopy objcopy \
  --output /tmp/octeon-atomics.json
```

`provenance.json` records source and text hashes from GNU as 2.30 and GNU
objcopy 2.47.20260726. These original fixtures are covered by the repository's
license. No third-party implementation or firmware is included.

The tests assert memory preservation and an uncertain SC result, independently
of any executor's reservation choices. SAA/SAAD start at Plus; LA and ZCB/ZCBT
start at II. Generic MIPS collision controls and operand-capture checks live
in `kuna-sleigh/tests/octeon_core.rs`. No hardware concurrency, cache, MMU,
scratch/LMT or device timing claim follows from these tests.
