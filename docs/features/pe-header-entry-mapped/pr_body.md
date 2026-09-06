## The problem

Windows maps a PE in two parts: `SizeOfHeaders` file bytes at `ImageBase`, then
each section. kuna mapped the sections only, so an image that puts code in its
header page — a packer stub, a hand-built keygenme — had an entry no surface
could reach, not even an explicit `--define-function`.

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pe_headerentry_i386.py
wrote .../pe_headerentry_i386.exe (1024 bytes)

$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/pe_headerentry_i386.exe \
      0x400154 --addr --define-function 0x400154=entry
error: address 0x400154 is not mapped in this input
$ echo $?
1
```

That fixture reproduces the reported image's layout: `e_lfanew` `0x0c` and a
`0xe0`-byte optional header over two sections, so the section table runs
`0x104..0x154` and `AddressOfEntryPoint` `0x154` is the byte right after it.

## The fix

- `loader/pe_headers.rs` publishes `[ImageBase, ImageBase + SizeOfHeaders)` as
  one more mapping unit, backed by the file bytes at offset 0. It reaches the
  load path through a new `ObjectFormat::header_region`, which defaults to
  `None`: ELF/Mach-O/COFF keep a section- and segment-derived map byte for byte.
- Read-only `DATA`, not `CODE` — what Windows does (`PAGE_READONLY`), and it
  keeps the executable-region scans out of the MZ/PE bytes of every PE, so
  discovery cannot invent entries in a header. Reaching one stays an explicit
  act, and `disassemble --addr 0x400154` says "in a non-executable data section
  … pass `--as code`" rather than pretending.
- The extent is clamped to the file length and to the first section's RVA, so a
  malformed `SizeOfHeaders` — the same field a packer overwrites — can never
  shadow real content. A clamp to zero publishes nothing.
- It is purely additive on an image that loads today. Over 57 PE images (all 17
  in-repo, 40 dataset), `decompile-all --json` and `functions --json` are
  byte-identical before and after: 3,524 decompiled functions, 0 changed.

## The tests

Five unit tests in `loader/pe_headers/tests.rs` (the well-formed extent, both
clamps, a section at RVA 0 leaving no room, a non-PE), two in
`loadimage_object.rs` (the entry's bytes are readable and the region is
published read-only; the page never overlaps a section on a real PE fixture),
and the promoted acceptance probe `tests/cli/pe-header-entry-mapped.json`, which
fails on the unpatched tree with the error above and now emits
`int entry(int a0) { return a0 + 7; }`. On the reported image the same command
goes from exit 1 to the aPLib-style bit-reader stub at `0x400154`.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 644/644 ·
`make test-cli` 50/50 · `make check-spec` OK · `kuna catalog --check` OK ·
`make rust-test` green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
