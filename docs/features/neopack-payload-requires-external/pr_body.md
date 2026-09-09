## The problem

`kuna unpack` handled exactly one packer, so a NEOLite-packed PE — a `.NEOpack`
loader section, the entry point inside it, and every original section compressed
in place — came back as a file kuna does not know anything about, and every other
surface stayed useless on it:

```console
$ kuna unpack ./CryptoME.exe --json
error: ./CryptoME.exe: no UPX PackHeader found
note: kuna unpack handles UPX-packed executables; this file is not one
$ echo $?
1
$ kuna functions ./CryptoME.exe | wc -l
64      # all imports; not one function of the program
```

The tester who hit this recovered the four sections by running the stub's
decompressor under Unicorn.

## The fix

- `kuna-analysis/src/neolite.rs` decodes the format statically. It is an LZX
  derivative: 58 position slots, three repeated offsets, main symbols that merge
  match length and position slot as `256 + slot * 8 + len_slot`, DEFLATE's length
  base/extra tables behind a separate 28-symbol tree, and DEFLATE-style
  code-length transmission delta-coded modulo 16 against the previous block.
- Nothing has to be inferred by running the stub, because NEOLite leaves the
  original section table untouched: every virtual address and virtual size still
  describes the original image, so each section decompresses to a size the file
  already declares.
- Three things are restored beyond the bytes — the entry point (the stub's
  `push OEP; ret` operand), the import data directory (the program's own
  descriptor array, found in the recovered `.rdata` rather than the stub's decoy),
  and `CNT_CODE|MEM_EXECUTE|MEM_READ` on the section the entry lands in. That last
  one is the section-flag problem `docs/cli.md` already records for this exact
  file: NEOLite marks all six sections `INITIALIZED_DATA|READ|WRITE`, `.text`
  included, and a `.text` that does not claim to be code is skipped by the
  executable-section filter.
- Recognition happens **before** UPX is asked, on a `.NEOpack` section that owns
  the entry point. A file that is neither still gets `no UPX PackHeader found`
  and exit `1`, byte for byte.
- A section the packer stored uncompressed (resources usually are) fails the
  code-table completeness test on its first block header and is carried through
  verbatim. Completeness is the only thing separating plaintext from a stream, so
  it is enforced exactly rather than approximately.

On the witness: 3 sections and 139,776 → 447,488 bytes, and `kuna functions` goes
from 0 to 387 with import names, with the recovered entry decompiling to the MSVC
CRT startup.

## The tests

`tests/fixtures/neolite_packed_pe_i386.exe` (3,584 bytes) with a `.py` sidecar
that carries a reference **encoder** for the format — no NEOLite compressor
exists to build a fixture with, and CI has no dataset. It exercises a verbatim
block, an aligned-offset block, extended lengths, a repeated-offset match, a
stored section and the import array. Nine cargo tests in the new module cover the
round trip, the stored-section fallback, import recovery walked end to end, the
code-table refusals, and a PE32+ image being declined by name.
`tests/cli/neopack-payload-requires-external.json` pins packer, entry, section
count, output size and import RVA, so an exit `0` that recovered nothing fails it.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
