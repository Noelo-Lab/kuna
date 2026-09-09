## The problem

A self-decrypting stub decompiles to an ordinary `return;`. Nothing in the C, and
nothing in `--json`, says kuna ran into bytes it could not decode.

```
$ kuna decompile Defender.exe 0x401746 --addr
void sub_401746(void)
{
  ...
  while (4 <= v3) {
    *v2 = *v2 ^ dat_406008;
    ...
  }
  switch(0x40190e) {
    case 0x40190e:
      return;

  }
}
```

The switch is right: the packer transfers control with `push 0x40190e; pop ebx;
jmp ebx` and kuna resolves the single constant target. But `0x40190e` is what the
loop just wrote, so at load time it is ciphertext — `kuna disassemble --addr
0x40190e --count 6` ends at `.byte 0xc6`. kuna knew: it planted an artificial
halt (`return(#0x1)` with the `badinstruction` bit). It just printed it as a
return, and `error` stayed `null`.

Reproducible on any bad byte:

```
$ cstool -d x64 85ff7401c306 0x100000 | tail -2
 0x100004  c3  ret
 0x100005  06  (bad)
```

## The fix

- New `decodehalt` option (P2/flow-classification, `on|off`, default **on**,
  DIV-151). It ports the part of upstream `PrintC::opReturn` (printc.cc:774) kuna
  left out: the three decode-failure halt causes render as `halt_baddata()`,
  `halt_unimplemented()` and `halt_missing()` instead of `return;`.
- The two warnings upstream buffers at the same point (`flow.cc:441-475`) were
  `STUB(W4)` comments; they are now emitted, so the statement carries
  `// warn: Bad instruction - Truncating control flow here` and the prototype
  carries `// warn: Control flow encountered bad instruction data`.
- `noreturn`, the fourth halt cause, is left alone. It is not a decode failure,
  it is already reported as `// no-return` on the call it follows, and its
  rendering is pinned by the corpus measurements behind `noreturn_propagate`.
- A printer half travels with it, and it is the half that was already doing
  damage. `voidtailreturn` — which every `kuna decompile` runs, since `auto`
  resolves to `aggressive` — elides a void function's trailing bare `return;`,
  and an artificial halt has exactly that shape. So on a straight-line truncation
  the marker was *deleted* and its warning re-attached to the statement that
  decoded fine:

  ```
  -  dat_101000 = 1; // warn: Bad instruction - Truncating control flow here
  +  dat_101000 = 1;
  +  halt_baddata(); // warn: Bad instruction - Truncating control flow here
  ```

  The three decode-failure causes are excluded from that elision, gated on the
  same option so `decodehalt off` restores both halves together.

Not gating this behind an off-by-default flag was the point: kuna cannot honestly
assert a normal return at an address it could not read. `option decodehalt off`
restores the silent `return;`.

## The tests

`tests/stages/kuna-decodehalt.xml` is the two-pass case, run with
`voidtailreturn on` so it covers the elision seam: off, the truncated path leaves
no statement at all; on, `halt_baddata();` under a warned prototype.
`tests/cli/undecodable-encrypted-code-becomes.json` is the promoted acceptance
probe over a newly vendored `decodehalt_x86_64` fixture that reduces the witness
(XOR loop, computed jump, undecodable ciphertext).

Gates: `make test` PARITY OK 675/675; `make test-stages` PARITY OK 734/734;
`make rust-test` green (6,340 tests); `make test-cli` 117/117; `make check-spec`
green (strict too); `kuna catalog --check` OK.

Sweep, `decompile-all` both arms over 82 binaries / 31,862 functions (27 decbench
ELFs across seven architectures, 55 crackme PE/ELF images): 89 functions in 19
binaries change, +178/-154 lines, **24 gain a statement and none loses one**, and
with comments stripped every hunk is `return;` → `halt_<cause>();` — nothing else
moves. Speed −1.02% on the witness function (median of 7) and +0.88% on a
whole-binary run of a 4,037-function PE (medians of 3, 64.51 s → 65.08 s).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
