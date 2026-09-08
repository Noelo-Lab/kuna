## The problem

`kuna unpack` can only unpack what its `PackHeader` search finds. A repacker that
strips that header — or a private packer that borrows only UPX's codec — leaves an
image whose payload is intact and whose metadata is gone, and there is no way to tell
kuna where the stream is. Clobber the `UPX!` magic in the vendored witness and the
whole command is a wall:

```console
$ python3 -c "b=bytearray(open('decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_lzma_x86_64','rb').read()); b[0x3194:0x3198]=b'\0'*4; open('/tmp/stripped','wb').write(bytes(b))"
$ kuna unpack /tmp/stripped
error: /tmp/stripped: no UPX PackHeader found
note: kuna unpack handles UPX-packed executables; this file is not one
$ kuna unpack --help
usage: kuna unpack <binary> [-o OUT] [--json]
```

The stream is still at file offset `0x10c`, still LZMA1, still decodable. On the
witness that filed this — a UPX0/UPX1/.rsrc PE with no `PackHeader` — the reader had
already read the decoder's setup code, knew the range and the coder parameters, and
had to leave kuna and finish the job in Python.

## The fix

- `kuna unpack --raw-lzma START:END` skips discovery and decodes that range as one raw
  LZMA1 stream. Endpoints are virtual addresses, resolved through the object's section
  table and clamped to the bytes the file stores; `--raw-offsets` reads them as file
  offsets, which is also the only reading for an image no object parser recognises.
- Nothing declares the uncompressed size here, so the decode runs to the end of the
  *input*: `upx::lzma::decompress_exhaustive` bounds the same decoder by the caller's
  cap (`--raw-max-size`, default 512 MiB) instead of by a `b_info` length, and running
  out of input is an ending rather than the `Truncated` refusal a declared length earns.
  The size is a result, not a parameter — which is what makes it evidence.
- `--lzma-props pb,lp,lc` (or the two prefix bytes as hex) for a stream that carries no
  UPX parameter prefix; the prefix is synthesized so one decoder path serves both.
- The refusal path is untouched on purpose. An image that is genuinely not packed still
  exits `1` naming that: an `unpack` that exited `0` on everything would trade a right
  answer for a reachable one.

## The tests

`the_pack_header_search_is_not_degraded_by_the_override` pins that refusal. Four more
`unpack_cli` cases cover the address form, the file-offset and explicit-property
spellings, an unmapped range, and the cap; three `upx::lzma` cases decode the vendored
witness block with its length withheld — 792 bytes, no end marker, byte-identical to
the length-declared decode — and check that a declared length still refuses in both
directions. `tests/cli/headerless-lzma-payload-unpacked.json` runs the acceptance
against `rawlzma_headerless_pe_x86_64.exe`, a 1 KB PE64 built by its `.py` sidecar out
of that same block, with no `PackHeader` anywhere.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 687/687 ·
`make rust-test` 6,103 passed / 377 targets, 0 failed · `make check-spec` OK ·
`make test-cli` 88/88 · `kuna catalog --check` OK. On the filing witness the override
recovers 5,489,913 bytes in 0.16 s, byte-identical to the reader's Python.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
