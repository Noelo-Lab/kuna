## The problem

`kuna disassemble` lists instructions that are not in the file. The crackmes.one
`keygenme` (`5ee1f28c33c5d449d91ae7c0`) has one executable `PT_LOAD`
`[0x08048000, 0x080d1904)`, then an unmapped hole until `0x080d2f50`:

```
$ kuna disassemble ./keygenme 0x80d18b0 --addr --count 30
# 30 instructions at 0x80d18b0 (0x80d18b0..0x80d191b, 107 bytes)
...
0x80d1901     0000                  ADD byte ptr [EAX],AL
0x80d1903     0000                  ADD byte ptr [EAX],AL   <-- the segment ends at
0x80d1905     0000                  ADD byte ptr [EAX],AL       0x80d1904; every row
0x80d1907     0000                  ADD byte ptr [EAX],AL       from here down is the
...                                                             loader's zero fill
0x80d1919     0000                  ADD byte ptr [EAX],AL

$ kuna disassemble ./keygenme 0x80d190b --addr --count 4
error: no bytes mapped at 0x80d190b in ./keygenme: no loaded segment covers it
(a packed image maps none of its original addresses -- `kuna unpack` first)
```

The second command is right and the first walked 23 bytes past the same edge, with
`--json` reporting `truncated: false` and no notes. `kuna read` had it too: 384
zeroes across the hole under a header claiming the range was real. Reproducible
without the dataset — `segmentgap_i386`, vendored here, is the same layout in
4.3 KB and unpatched answers `end 0x804803f` with 22 invented rows.

## The fix

- The load image answers a read that *starts* on a mapped address for its whole
  length, zero-filling whatever the segments do not cover — the upstream BFD
  `loadFill` contract, and what makes a `.bss` tail read back as zeroes. Only the
  start was ever checked, so the listing now carries the other end itself: it
  clips to the contiguous mapped **run** holding the start (`mapped_run`, over
  `LoadImage::get_segments`).
- Runs, not segments, because two `PT_LOAD`s that touch are not a hole and a
  listing crossing between them must stay whole. A loader that publishes no
  segments (the XML corpus, a relocatable object) is silence, not a bound.
- An instruction that would *straddle* the end is refused the way a translator
  refusal is, so the mapped bytes under it list as `.byte` rather than as an
  instruction the file only half contains.
- The stop goes in `notes` and on stderr with the next mapped address. `truncated`
  stays false: its contract is "`end` is where to resume", and here resuming errors.
- `kuna read` and `decompile-graph`'s `assembly` share the walk and are fixed with it.

## The tests

`tests/cli/disassembly-fabricates-zero-byte.json` is the promoted acceptance,
re-pointed at `segmentgap_i386`; two e2e cases in `disassemble_cli.rs` (the clip,
and that a listing inside mapped memory is byte-identical and silent) plus unit
tests for the run merge and the note predicate. All fail without the fix.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK ·
`make check-spec` OK · `make test-cli` 94/94 · `kuna catalog --check` OK ·
`make rust-test` 6129 passed / 1 failed — `verify_w10_proto_unlock`, the known
`KUNA_DECOMP_TEST` oracle-env artifact: green under `env -u KUNA_DECOMP_TEST`,
and nothing here touches `kuna-decomp`.

A/B of every function's listing across 95 binaries (the fixture corpus plus 26
dataset images; ELF/PE/Mach-O, i386/x86-64/ARM): **16,069 functions, 11 listings
changed, every one of them the same single row** — a function ending at a mapped
section's edge whose last `ADD byte ptr [EAX],AL` was half zero fill and is now
`.byte 0x00`. Row counts identical everywhere; nothing truncated, no row lost.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
