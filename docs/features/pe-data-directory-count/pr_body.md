## The problem

A PE declares how many data directories it has twice — once as
`NumberOfRvaAndSizes`, once as the room `SizeOfOptionalHeader` leaves for the
array — and packers overwrite the first. Windows reads whichever is smaller;
kuna rejected the whole image, so `functions`, `decompile-all`, `strings`,
`xrefs` and `disassemble` all exited `1` before a byte of code was mapped.

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pe_datadircount_i386.py
wrote .../pe_datadircount_i386.exe (1024 bytes)

$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/pe_datadircount_i386.exe --json
error: could not build an architecture for .../pe_datadircount_i386.exe:
not in recognized object file format: Invalid PE number of RVA and sizes
$ echo $?
1
```

That image's optional header is 224 bytes — room for exactly the 16 directories
it really carries — and declares 1531532893 of them, verbatim from the reported
Invius-packed binary.

## The fix

- `loader/pe_datadirs.rs` clamps the declared count to
  `(SizeOfOptionalHeader - 96 or 112) / 8` in a copy of the bytes, at the same
  canonical read point as the Mach-O fat-slice peel and the ELF section-table
  repair, so the loader and every analysis pass see one recovered view.
- Clamping to what fits rather than to a hard 16 is what Windows does, and it
  means the imports are read from the real directory table instead of a
  fabricated one — the recovered binary resolves `GetProcAddress`/`LoadLibraryA`.
- The clamp is kept even if the copy still does not parse. A count larger than
  its own header is wrong however the rest of the file reads, so keeping it lets
  the caller report what is *actually* unreadable. (The ELF repair next door
  discards itself instead, because there a cleared section table can genuinely
  not have been the problem.)
- It cannot change any image that loads today: it fires only where the declared
  count already made `object` reject the file. Sweeping every PE in reach, 1 of
  17 in-repo and 3 of 163 dataset images clamp — the new fixture and three
  packed crackmes, all previously unloadable.

## The tests

Seven unit tests in `loader/pe_datadirs/tests.rs` (PE32 and PE32+, the exact
boundary at 16 vs 17, a non-PE and a ROM-magic image left byte-identical, and an
image that stays broken after the clamp), plus the promoted acceptance probe
`tests/cli/pe-data-directory-count.json` against a vendored 1024-byte PE32 twin,
which fails on the unpatched tree and now reports 2 functions. On the reported
image `kuna functions` goes from exit 1 to 54 functions, and the entry stub at
`0x40908e` decompiles.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 640/640 ·
`make test-cli` 47/47 · `make check-spec` OK · `kuna catalog --check` OK ·
`make rust-test` green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
