## The problem

`kuna unpack` refuses every UPX image compressed with LZMA — which is what
`upx --lzma` and `upx --best` write, so it is most of what is in the wild. The
packed file is then a stub and a blob, and `kuna functions` reports nothing:

```console
$ upx --lzma -o /tmp/x.packed /usr/bin/head
$ kuna unpack /tmp/x.packed
error: /tmp/x.packed: unsupported UPX image: compression method 14 (LZMA)

$ kuna functions /tmp/x.packed --json
{ "binary": "/tmp/x.packed", "count": 0, "functions": [] }
```

## The fix

- `kuna-analysis/src/upx/lzma.rs`: an LZMA1 decoder, written out rather than
  taken as a dependency, for the same reason `nrv.rs` is — the input is a
  hostile file, so every array index, match distance and output write is
  bounds-checked and returns an error instead of panicking or emitting a
  partial image.
- UPX emits no container: a block is a raw LZMA1 stream with no
  properties/size preamble and no end-of-stream marker, prefixed by two UPX
  bytes — `pb` in the low three bits of the first, `lc` and `lp` in the low and
  high nibbles of the second. The uncompressed length comes from the block's
  `b_info`, so the decoder runs to a known size.
- `method_codec` returns a `Codec` instead of an NRV pair, so the ELF and PE
  walks pick a back-end through one place; nothing else about either walk moves.
- It does *not* inherit the NRV arm's exact-input-consumption rule: a range
  coder holds lookahead bytes it never uses, so trailing slack is a property of
  a valid stream. The packer's Adler-32 over the decoded bytes is what proves
  the block, and it is already checked.

## The tests

`upx_packed_lzma_x86_64` is the existing NRV witness's *own program* repacked
with `upx --lzma`, so `unpacks_the_lzma_witness_byte_for_byte` asserts the two
recoveries agree on all 31,640 bytes — a check a subtly wrong decoder cannot
pass by accident. Plus the decoder's own unit tests, a mislabelled-stream case
(an NRV block re-headed as method 14 must still fail, now in the decoder rather
than by name), and `tests/cli/upx-lzma-compression-blocks.json`.

On the need's witness (crackmes.one `68ae3bfc8fac2855fe6fb8ee`), `kuna unpack`
now writes an 8,501,872-byte ELF that is **byte-identical to `upx -d`**, in
0.24 s; `kuna functions` on it goes from 0 to 16,198.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
