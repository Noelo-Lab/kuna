## The problem

On a binary with no ELF section table, every function kuna finds reports a size
of `0`, so size-based triage throws the whole binary away. Take any ELF and zero
its three section-header fields — nothing else changes, every mapped byte is
identical:

```
$ cp /bin/true ./noshdr && python3 -c "
import struct; b=bytearray(open('noshdr','rb').read())
struct.pack_into('<Q',b,0x28,0); struct.pack_into('<H',b,0x3c,0); struct.pack_into('<H',b,0x3e,0)
open('noshdr','wb').write(bytes(b))"

$ kuna functions ./noshdr --json | jq '[.functions[].size]'
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

$ kuna functions ./noshdr --min-size 1 --json | jq '{count, total, error}'
{ "count": 0, "total": 11, "error": null }
```

`--min-size 1` is documented triage, and it discards all eleven with no error to
distinguish that from a binary that really holds nothing. `--summary` reports
`code_bytes 0` and a `largest` list of zeroes, so the "which function is the big
one" question a stripped binary is opened with cannot be asked at all.

## The fix

- `funcextent` clips each entry against the loader's CODE **sections**; an image
  with no section table publishes none, so every entry took the "outside every
  CODE section" answer that exists for import pointer slots. When the section
  table yields no CODE span at all, the clip now runs against the executable
  load **segments** instead — still an upper bound clipped at the next entry,
  just a coarser container for the last entry of a segment.
- The loader reports its `PT_LOAD`s through a new `LoadImage::get_segments`,
  separate from the section walk. Teaching the loader to synthesize sections
  from segments instead would have silently changed which entries
  whole-binary decompilation selects.
- The fallback is whole-table, never per-entry. An entry that misses the CODE
  spans an image *does* publish is a pointer slot or an undefined external, and
  choosing a segment for it would hand a body to exactly those.

## The tests

`tests/cli/zero-function-sizes-make.json` and a four-case console gate run on
`noshdr_x86_64`, a new 304-byte ELF64 PIE with no section table whose two
functions report `0` and `0` before the change and `16` and `6` after — the
neighbour clip and the segment-end clip. Two loader unit tests cover the
permission-to-flag translation. Sweeping 135 fixture images and 29 system
binaries through `kuna functions --json`, exactly two change, both sectionless,
both from `0` to a real extent; `kuna functions /bin/bash` stays at a 0.32 s
median.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
