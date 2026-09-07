## The problem

On a stripped, non-PIE ARM32 ELF, `main` is never discovered — so most of
`.text` is never decoded, and every string the missing code loads reports no
reader. On the witness (`arm_kgme1`, a stripped ARM keygen-me) the prompt the
program prints has no owner at all:

```
$ kuna strings ./arm_kgme1 --json --filter 'Enter your key'
      "address_hex": "0x9124",
      "text": "Enter your key: \n",
      "xrefs_count": 0,
      "functions": []
```

`kuna xrefs --to 0x9124` finds nothing either, although the disassembly loads it
four instructions before the `printf` call:

```
$ cstool arm 04002de5\ 10009fe5\ 10309fe5 ...   # or, in the image:
0x8d20   ldr r0,[0x9024]      ; 0x9024 holds 0x9124, the prompt
0x8d24   bl  0x835c           ; printf@plt
```

`kuna functions` looks like it covers `0x8d20` — it reports `_INIT_0` at
`0x83e8` with a 3368-byte extent — but that extent is gap-fill. The walk follows
`_INIT_0`'s real control flow and stops at the `b 0x8350` 36 bytes in, so
everything past `0x8404` is never decoded. The function that should own the
prompt, `main` at `0x8cf8`, is not in the inventory.

## The fix

- Entry-discovery oracle 4 already reads the `_start` → `__libc_start_main(main, …)`
  idiom on ARM — but only the PIE form, because it identifies the GOT slot by the
  `R_ARM_RELATIVE` that relocates it. A non-PIE executable carries no such
  relocation; the linker knows the final address and stores it. The new pass
  (`kuna_armlibcmain.rs`) is the arm of that oracle that reads the value out of
  the image instead of out of the relocation table.
- It decodes both shapes glibc's ARM `crt1.o` ships: the classic literal-pool
  word (`ldr r0,[pc,#imm]`), and the GOT-indexed slot a modern `-no-pie` link
  fills statically (`ldr r0,[pc,#imm]; ldr r0,[rN,r0]`). Shipping only the first
  would leave every binary a current cross-toolchain produces broken.
- Both are anchored on the `bl` to the PLT stub the import table names
  `__libc_start_main`, rather than on the pool load alone. AAPCS puts the first
  argument in `r0`, so the last write to `r0` before that call is `main` by the C
  runtime's own contract — which is what makes it safe to apply the name. No
  landmark, no claim: a static link or a non-C entry point is refused, as are a
  Thumb `_start`, a target outside every executable section, and an address that
  already carries a function symbol.
- A prologue-signature scan over the undiscovered gap was considered and not
  taken: the image states where `main` is, so nothing has to be guessed.
- `--option armlibcmain off` restores the previous inventory exactly.

## The tests

`verify_armlibcmain.rs` is the two-pass gate (off: no function covers `main` at
all; default: both crt1 shapes recover it and its body reaches the `printf`/`fgets`
calls; plus a count guard that nothing is added on a PIE ARM image), over two new
vendored fixtures — one per shape. Five unit tests cover the refusals and the
three A32 decoders. `tests/cli/arm-prompt-string-owning.json` pins the CLI symptom
and additionally asserts the owner is named `main`, which the gap-fill entry could
not satisfy.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 677/677 PARITY OK,
`make rust-test` green, `make check-spec` OK (strict), `make test-cli` 66/66,
`kuna catalog --check` OK. Sweep: `kuna functions` in both arms over 143 images
(129 repo fixtures + every ARM binary in the RE corpus) fires on 2 — the witness
and one unrelated stripped ARM challenge — and a full `decompile-all --json` diff
on both shows one function added and zero emitted-C hunks moved. Speed −0.17%
(median of 9).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
