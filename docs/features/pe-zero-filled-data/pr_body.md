## The problem

Every executable format states a section's RAM footprint separately from the
bytes the file supplies for it, and the excess is zero-filled at load: ELF
`p_memsz` over `p_filesz`, PE `VirtualSize` over `SizeOfRawData`. kuna copied
only the file extent, so reading a zero-initialized global was refused as an
address no segment covers — with advice to unpack an image that was never
packed.

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pe_bsstail_x86_64.py
wrote .../pe_bsstail_x86_64.exe (1536 bytes)

$ kuna read decompiler/crates/kuna-analysis/tests/fixtures/pe_bsstail_x86_64.exe \
      0x140002740 --addr --bytes 64
error: no bytes mapped at 0x140002740 in ...: no loaded segment covers it
       (a packed image maps none of its original addresses -- `kuna unpack` first)
$ echo $?
1
```

That fixture carries the reported image's `.data`: `VirtualSize` `0x7a8` over
`SizeOfRawData` `0x200`, so `0x140002740` is `0x540` bytes into a tail the image
plainly maps. The same `.data` holds the globals the decompiled C already names
(`dat_140008778` in the reported binary) — they were readable to the engine and
unreadable to every surface that asks for bytes.

## The fix

- `ObjectLoadImage`'s segments now record their RAM footprint (`memsz`)
  alongside their file bytes, and the containing-segment walk answers over the
  footprint. The read past the file extent lands in the zero-fill `copy_segment`
  was already doing for a read straddling a segment's tail.
- A section with no file bytes at all — `SizeOfRawData` `0`, or an ELF `.bss` in
  its own `PT_LOAD` — is kept instead of dropped as empty, for the same reason.
- Only where the image says the region holds **data**. An executable
  uninitialized region is a packer's staging area, and materializing zeroes
  there does not recover the code: on the in-repo UPX PE, mapping `UPX0`
  (`VirtualSize` `0x9000` over `SizeOfRawData` `0`) turned the packed entry's
  one-line `Unable to load 512 bytes` into 16,883 lines of `add [eax],al` in
  19.6 s. Executable segments keep exactly today's behaviour: 182 ms, and
  `kuna unpack` still the answer.
- The footprint is trimmed at the next segment's vma, so a declared size can
  never shadow real content. The trim only shortens the *tail*: an image that
  stacks several segments at one address (a COFF `.obj` read through the linked
  path) maps exactly what it mapped before.
- Purely additive on an image that loads today. `decompile-all` is
  byte-identical before and after across the 144 in-repo ELF/PE/Mach-O/COFF
  fixtures and 9 larger binaries (the reported PE, 4 crackmes, 4 fixtures);
  `kuna unpack` output is byte-identical on all three UPX images. The only new
  answers are byte reads at addresses that previously had none.

## The tests

Three unit tests in `loadimage_object.rs`: the PE fixture reads its file extent,
its zero tail, and still refuses the byte past `VirtualSize`; an ELF `p_memsz`
tail does the same, and the same tail on an executable segment stays a hole; and
the trim stops at the next segment without ever shortening a file extent. Plus the promoted acceptance probe
`tests/cli/pe-zero-filled-data.json`, which fails on the unpatched tree with the
error above.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 687/687 ·
`make test-cli` 82/82 · `make check-spec` OK · `kuna catalog --check` OK ·
`make rust-test` 374 test binaries, 6052 passed, 0 failed.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
