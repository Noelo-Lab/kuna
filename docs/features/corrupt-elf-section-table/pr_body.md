## The problem

An ELF whose section table is garbage was rejected outright, even though its
program headers described every loadable byte. One round-3 challenge binary
(`5ab77f6633c5d40ad448cc64`) carries `e_shoff=57005`, `e_shnum=57007`,
`e_shstrndx=47806` in a 161 KB file — a section table 2 MB past EOF — beside nine
intact program headers. `readelf -l` prints the entry point and both LOAD segments
and exits 0; every kuna surface exited 1.

Reproduce it on a 107-byte file built from those same three values:

```console
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/corruptshdr_i386.py /tmp/x
$ kuna functions /tmp/x --json
error: could not build an architecture for /tmp/x: File: /tmp/x : not in
recognized object file format: Invalid ELF section header offset/size/alignment
$ echo $?
1
```

Clearing those three fields by hand got the file to load, and still reported
`count: 0` — so there were two gaps, not one.

## The fix

- The loader drops an unusable section table instead of failing on it
  (`loader/elf_shdr.rs`). The bytes are normalized once, at the canonical read
  point beside the Mach-O fat-slice peel, so the loader and every analysis pass
  see the same view. The test is pure header arithmetic before any parse, so a
  usable table costs one bounds check and is passed on byte for byte; the rewrite
  is kept only if the rewritten copy actually parses, so unrelated corruption
  still reports `object`'s own error.
- Entry discovery falls back to the `PF_X` `PT_LOAD` segments when there is no
  section table. `executable_sections` is the "is this plausible code?" filter
  every oracle passes through, and it reads the section table alone — so with no
  sections it rejected the image's own `e_entry`, which is why clearing the header
  by hand still gave zero. Gated on the table being *absent*, not on the
  executable set being empty: an image that has a section table keeps exactly the
  ranges it had.
- A UPX image is section-less too, but its load segments are a decompressor stub,
  so a segment carrying the `UPX!` magic declines the fallback. Otherwise the
  stub's five routines would displace the `image appears UPX-packed; try
  \`kuna unpack\`` diagnostic, which is the more useful answer and is pinned by
  `tests/cli/zero-functions-exit-0.json`.
- `strings`, `xrefs`, `decompile-graph` and the call graph parse the image
  themselves rather than through the loader, and read it through the same
  normalization.

On the reported binary: `functions` and `decompile-all` go from exit 1 to 24
functions, `strings` from a parse error to 83 strings (`"scanned": "segments"`),
`disassemble 0x80492d0` to 29 instructions.

## The tests

`tests/cli/corrupt-elf-section-table.json` (the promoted acceptance) over a new
107-byte in-repo fixture, plus 8 unit tests on the header repair and 4 on the
discovery fallback — including the UPX image, which must *not* take it. Gates:
`make test` 675/675 PARITY OK, `make test-stages` 635/635 PARITY OK,
`make test-cli` 35/35, `make check-spec` OK, `catalog OK`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
