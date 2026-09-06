## The problem

On a position-independent ARM binary, `kuna strings` reports every literal the
program prints as referenced by nothing — no xrefs, no owning function — even
though the disassembly plainly shows which function loads it.

```
$ kuna strings ./trap --json --filter 'Benar!'
      "address_hex": "0x4ab",
      "text": "Benar! Flag: 98CTF{%s}\n",
      "xrefs_count": 0,
      "functions": []
```

The address is in neither instruction that forms it, and the pool word is not an
address at all — it is the signed distance from the `add` to the datum:

```
$ rasm2 -a arm -b 32 -D 38009fe500008fe0
0x00000000   4   38009fe5  ldr r0, [pc, 0x38]     ; at 0x660: reads 0x6a0 = 0xfffffe3f = -0x1c1
0x00000004   4   00008fe0  add r0, pc, r0         ; at 0x664: 0x66c - 0x1c1 = 0x4ab
```

It is not one site either: `main` forms all four of its string references this
way, so the whole function references nothing.

## The fix

- `kuna_picpool.rs` composes the two decode-time constants the pair already
  carries: the read-only pool word and the constant the `add` materialised from
  its own address. The composed address is filed as a `Data` edge from the
  `add`, which is what gives it an owning function.
- The word is admitted as a *displacement* only where `kuna_poolref` **declined**
  it as a pointer, so the two rules never claim one word.
- The carry is keyed by the address it reaches, not held as "the previous
  instruction's state" — the walk is breadth-first, so at a conditional branch
  the two successors interleave. It is dropped at the first write of the
  register and at any branch, call or return, and bounded to eight instructions,
  so a scheduled `ldr … ; add rX,pc,rX` is still one pair.
- Both a pool word and the instruction's own PC must contribute to a reported
  value. Without that second taint `add r0,r0,#4` on the same word would land on
  a real literal and be filed as a reference to it.
- The `checkOperands` "below 4096 could be a number" floor is deliberately not
  applied to the composed address: the filing image is an Android PIE mapped
  entirely under 0x2828, so the floor would discard every reference in it.

No option and no `phases.toml` row: the xref index is read-only, commits nothing
into the engine, and its consumers are `kuna strings` / `kuna xrefs` /
`kuna decompile-all`. Emitted C cannot change, which both parity gates confirm.

## The tests

`verify_picpool.rs` over a new 1,297-byte vendored fixture: the adjacent pair,
the pair a scheduler separated, and the refusal (`add r0,r0,#4` whose sum *is* a
mapped literal — only the missing PC separates it from a reference). Eight unit
tests in `kuna_picpool.rs` cover the fold and the four other refusals.
`tests/cli/arm-pic-literal-pool.json` is the promoted acceptance, re-pointed at
that fixture; it reports `functions: []` on the unpatched build.

Sweep: 417 images (the crackme corpus, the vendored fixtures, host binaries).
Seven ARM images gain 42 edges, every one hand-verified — the AES S-box, inverse
S-box and Rcon table on a statically linked crackme; the four literals `main`
prints plus `__atexit_handler_wrapper` on the witness; `_GLOBAL_OFFSET_TABLE_`
on four others, landing exactly on each image's `.got` address. Zero edges on
every x86-64, i386, ARM64 and PE image, six string references gained, none lost,
no owner dropped. `kuna strings` on the heaviest firer is 216 ms → 219 ms
(median of 11, warmup discarded).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
