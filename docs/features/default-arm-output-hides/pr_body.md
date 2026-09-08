# The problem

An ARM function can return a number that appears nowhere in kuna's output. A wide
immediate is not in the instruction — the compiler parks it in a literal pool in
`.text` and loads it PC-relatively — so the value ends up as an opaque global.

```
$ kuna decompile 1337ARM.bin main | tail -6
    v3 = dat_8458;
  }
  else {
    v3 = 0xffffffff;
  }
  return v3;
}

$ kuna disassemble 1337ARM.bin main | tail -2
0x8440        10309fe5              ldr r3,[0x8458]
0x8458        39050000              .word 0x00000539
```

kuna's own disassembler names the word. The decompiler never hears about it, and
reading the success value means inspecting the pool by hand.

# The fix

- The word is already painted `Varnode::readonly` — `.text` is `SHF_ALLOC` without
  `SHF_WRITE`. What was missing is the *fold*, gated program-wide by `option
  readonly`, which is off because it would fold every `.rodata` read in the program.
- New option `litpoolconst` (default on, DIV-136) folds a read that lies **entirely**
  inside an allocated, executable, non-writable, file-backed region, reusing the
  `dynreloc_const` exception `dynrelocs` and `msvcfpconst` already use. The warrant is
  the mapping: `r-x` memory cannot be written, so the image's bytes are the run-time
  value.
- A non-writable *data* section (`.rodata`, `.data.rel.ro`) stays behind `readonly`
  unless the image maps it executable too, as a one-RX-region firmware image does — the
  warrant is the permission, not the section name. That is the direction that matters on
  packers, whose data-section flags are least trustworthy.
- Ranges come from the loader's section table, or the `PF_X` load segments when the
  image has no sections. A loader that reports neither contributes none, so the option
  is inert on the XML corpus.
- Also fixes a latent out-of-bounds read in `StringManager::getCodepoint` that this
  exposed: a window ending in a multi-byte UTF-8 lead byte panicked instead of
  answering "not a legal encoding", which lost a whole 453-line function.

# The tests

`tests/cli/default-arm-output-hides.json` (the promoted acceptance, re-pointed at the
vendored `picpool_arm_le32` — CI has no dataset) plus `verify_litpoolconst.rs`, a
two-pass gate: off prints `return dat_42c + 0x42c;`, default prints `return 0x300;`.
Unit tests cover the range derivation and the `getCodepoint` truncation.

`make test` PARITY OK 675/675 · `make test-stages` PARITY OK · `make test-cli` 85/85 ·
`make check-spec` green · `kuna catalog --check` OK. Sweep of `decompile-all` on both
arms over 65 decbench binaries: 25 byte-identical, 40 changed, no function lost or
added. Speed 127.4 → 125.7 ms on the witness (median of 7).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
