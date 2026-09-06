## The problem

On ARM, the address of a string is not in the instruction that uses it — it is in
a literal pool, and the code loads it PC-relatively. kuna files the read of the
pool word and stops there, so the string itself is referenced by nothing and
`kuna strings` cannot say which function prints it.

```
$ kuna strings ./1337ARM.bin --filter 'FATAL: kernel too old' --json
      "address_hex": "0x661bc",
      "text": "FATAL: kernel too old\n",
      "xrefs_count": 0,
      "functions": []

$ kuna disassemble ./1337ARM.bin 0x862c-0x8634
  0x862c  ldr r0,[0x86e4]
  0x8630  bl 0x95cc

$ kuna read ./1337ARM.bin 0x86e4 --bytes 4
  0x86e4        bc 61 06 00                    |.a..|
```

The pool word plainly holds 0x661bc; nothing joins the two. (Reproduce on any A32
binary: `cstool arm 'b0009fe5'` is the load.)

## The fix

- A `read` of a **pointer-sized, pointer-aligned** location in an **allocated,
  non-writable** section files a second edge, from the same instruction, to the
  address that word holds — kind `data`, the address-taken case.
- Filed from the instruction, not from the pool word: a pool word lies in no
  function, so an edge from it would answer "who uses this string?" with another
  address instead of a name. The new edge therefore inherits exactly the
  attribution the existing `read` edge already had.
- Three refusals, each of which would otherwise be a fabricated reference: a
  narrow read (reading a *number* out of a pool), a writable slot (a GOT entry
  holds whatever the loader last wrote, not what the image says), and a word that
  fails the address floor the constant scan already uses.
- The width has to come from the access, not from the address varnode: a `LOAD`'s
  address is pointer-sized whatever the access is, so `ldrh r0,[pool]` reads as a
  pointer dereference otherwise.
- `kuna strings` now reads the function inventory once. `find_entry_at` rebuilds
  and linearly scans it per call, which is free while no string has an owner and
  0.85 s of a 2.5 s answer once 1,792 of them do. Output is byte-identical.

## The tests

`tests/cli/arm-literal-pool-string.json` (the promoted acceptance, re-pointed at a
new 735-byte vendored A32 fixture — CI has no dataset) plus
`kuna-console/tests/verify_poolref.rs`, which pins the follow *and* the three
refusals on that fixture, and unit tests for the dereference and for `data_refs`.
Before this, the promoted probe reports `functions: []`.

Sweep of `kuna strings` over 15 images: **0** attributions lost, **0** edges added
on all 7 x86-64/PE images, and on ARM 2,239 new string-to-function attributions on
u-boot and 763 on the witness. An independent capstone+symtab oracle corroborates
2,234 and 761 of those as real PC-relative pool loads; all 14 remaining outliers
were checked by hand and are loads the oracle's own sweep missed, not fabricated
edges. Speed (interleaved, min-of-7): walk +1.1%, `kuna strings` +1.6% on u-boot,
+3.3% on the witness, −3.8% on an x86-64 control.

Gates: `make test` 675/675 PARITY OK · `make test-stages` 635/635 PARITY OK ·
`make rust-test` green · `make check-spec` OK · `make test-cli` 35/35 ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
