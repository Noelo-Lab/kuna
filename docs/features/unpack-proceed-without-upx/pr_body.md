## The problem

`kuna unpack` tells you a UPX-packed Windows binary is not UPX-packed. Every
32-bit PE UPX has ever produced hits this, because the whole PE target is
missing; the file below is a crackme from the RE loop's round-6 corpus, now
vendored as a fixture.

```console
$ kuna unpack decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe
error: .../upx_packed_pe_i386.exe: no UPX PackHeader found
note: kuna unpack handles UPX-packed executables; this file is not one
$ xxd -s 0x3e0 -l 16 decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_pe_i386.exe
000003e0: 5550 5821 0c09 0204 f822 eecb 6910 39ab  UPX!....."..i.9.
```

It is one. `kuna functions` on it reports 1 function, and there is nothing else
to do with the file until something recovers the image.

## The fix

- A PE keeps its `PackHeader` in the header padding immediately before the
  compressed data, not in the tail the ELF arm scans. `upx/pe.rs` looks where
  UPX looks, and proves the candidate with the compressed stream's own Adler-32.
- The decompressed block is not the original file: UPX strips the import
  descriptors, thunk arrays and hint/name entries out of the image and moves
  resource leaves out of it, leaving a trailer its loader replays at run time.
  The unpacker replays it too — 79 imports over 4 DLLs and 3 resource leaves on
  this file — because recovering only the block gives you a PE with no imports,
  which looks fine and disassembles wrong.
- The DLL name strings live in the packed loader's own import table. UPX finds
  that table through the packed image's import data directory; this file points
  it at a decoy, so the table is instead found by agreeing with the trailer
  (descriptor *k* must name the DLL the trailer's *k*-th offset points at) and
  accepted only if unique. The declared directory is still tried first.
- Which trailer steps run depends on the *original* image's data directories, so
  an unimplemented one would desynchronize every later read instead of failing.
  The walk tallies the trailer bytes it consumed against the trailer's real
  length and refuses a mismatch, on top of naming base relocations, TLS and
  bound/delay imports up front. Nothing partial is ever written.

Afterwards:

```console
$ kuna unpack .../upx_packed_pe_i386.exe -o /tmp/out.exe
unpacked ... -> /tmp/out.exe
  format:      9 (win32/pe)
  method:      2 (NRV2B_LE32)
  unpacked:    49152 bytes
$ kuna functions /tmp/out.exe --json | jq .count
167
```

## The tests

Five unit tests on the vendored witness, all failing without the change: the
header-padding `PackHeader`, the end-to-end unpack (section table, entry
prologue at 0x401d91, a filtered call restored, the 79 imports walked back out
of the rebuilt directory, the relocated resource tree), the import table found
with the data directory blanked, and two refusal cases. Plus
`tests/cli/unpack-proceed-without-upx.json`.

Swept `kuna unpack` patched-vs-stock over 156 repo fixtures, 156 dataset PEs and
94 dataset ELFs: the ELF arm is byte-identical everywhere, no non-packed image
changed answer, and one unrelated corpus binary the change was not written
against now unpacks too (7 → 423 functions, 80 imports). Three images are named
refusals that write no file. `make test` 675/675, `make test-stages` 677/677,
`make rust-test` green, `make check-spec` strict OK, `catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
