## The problem

kuna had no way to be told what a self-modifying program writes over itself. A
stage-1 unpacker's plaintext exists only once that stage has run, so no loader
can read it — and the assertion vocabulary had no directive for it:

```console
$ kuna decompile ./crackme.exe 0x43d060 --addr \
    --define-function '0x43d060-0x43d0c6=decrypt_stage1' \
    --assert 'bytes 0x43d0c6 8bd581c21f324000' --assert-strict --json
error: --assert "bytes 0x43d0c6 8bd581c21f324000": unknown directive "bytes" (want one of function, typedef, prototype, data, param, return, comment, flow, name, type, readonly, volatile)
$ echo $?
2
```

`decrypt_stage1` rewrites the 2967 bytes above itself. Reported once (major):
recovering that layer took Python and a patched copy of the executable, and every
later run started from the copy instead of from the original plus a statement.

## The fix

- New `--assert 'bytes <addr> <hex|@FILE>'` directive: the bytes mapped at
  `<addr>` are these, whatever the file holds. `@FILE` takes raw bytes, so a
  whole recovered layer is one directive. The load image is what changes —
  **nothing is written to disk**.
- The console spelling is `override bytes <addr> <hex>`, so the text surface
  (which drives `decomp_dbg`) and `--json` (in-process) apply the same fact.
- It lands in the image slot, ahead of the analysis commit / `read symbols`. That
  ordering is forced: the stated bytes are the input to every later decode and
  nothing re-reads an address it has already lifted. `readonly` is the same
  constraint inverted, and gets it wrong in the other direction.
- `LoadImage::kuna_overlay_bytes` is a defaulted trait method (loaders that
  cannot take an overlay say so); `ObjectLoadImage` resolves the span through
  `find_section`, so an overlay lands in exactly the segment a read at the same
  address comes from, materialises a writable segment's zero-filled tail, and
  **refuses** — naming the span — anything no segment maps.

With it, the reported workflow is one command:

```console
$ kuna decompile ./crackme.exe 0x43d0c6 --addr --assert 'bytes 0x43d0c6 @stage1.bin'
void sub_43d0c6(void)
{
  ...
  *(unsigned int *)(v2 + 0x40321b) = sub_43d401();
  *(int *)(v2 + 0x403393) = v2 + 0x40294c;
  ...
}
```

## The tests

`kuna-console/tests/verify_byteoverlay.rs` (4) is the end-to-end bar: overlaying
`b8 2a 00 00 00 c3` on `assertranges_x86_64`'s `sample` must turn
`return scale * a0 + bias + …` into `return 0x2a`, an unmapped address must be
rejected naming the span, and the console command must say what it did. Plus two
loader cases in `kuna-analysis` (a warm read window is dropped; a `p_memsz` tail
is materialised, past it is not) and three parser cases in `kuna-cli`. The
promoted probe `tests/cli/byte-overlay-assertion-recovered.json` asserts the
`return 0x2a`, so it fails both if the directive is rejected and if it is
accepted and inert.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 721/721 ·
`make rust-test` 6,271 passed / 0 failed · `make check-spec` OK · `make test-cli` 106/106 ·
`kuna catalog --check` OK. Acceptance probe `a-09471914203f` PASS on the witness.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
