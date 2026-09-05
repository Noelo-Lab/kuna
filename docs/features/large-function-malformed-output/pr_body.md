## The problem

A Windows `int 0x29` raises the stack pointer by eight bytes, which wrecks the
frame of every function containing one. Any MSVC-built PE reproduces it; here is
the smallest one, vendored in this PR:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fastfail_x86_64.exe 0x140001000 --addr
void sub_140001000(long long *a0)
{
  char v1 [8];
  char v2 [48];
  char *v3; // rsp
  ...
  if (!v4) {
    (*(void *)swi(0x29))(5);
    v3 = v2;
  }
  *(unsigned long long *)&v3[-8] = 0x14000103d;
  BCryptGenerateSymmetricKey(*(unsigned long long *)&v3[0x38],&v3[0x30],&v3[0x28],0x20);
}
```

`v3` is RSP tracked as a variable, `&v3[-8] = 0x14000103d` is the `call`'s own
return address stored through it, and the API call takes stack blobs where it
takes values. On a real 240 KB MSVC crackme (`selam.exe`, function `0x140002d50`,
eleven `int 0x29`) that is 1,267 lines, 160 declarations and 63 of those stores.

x86 SLEIGH lifts `INT imm8` to `intloc = swi(imm8); call [intloc]` — a `call`
with no matching push, unlike every other x86 `CALL`, which lifts as
`RSP = RSP - 8; push &next; call target`. Nothing downstream tells them apart, so
`x86-64-win.cspec`'s `__fastcall` (`extrapop="8" stackshift="8"`) hands eight
bytes back after it. Once two paths join carrying stack pointers eight apart, the
frame stops being a constant offset from the spacebase and everything above
follows.

## The fix

- New `option fastfailnoreturn` (default on, DIV-119). On Windows, `int 0x29` is
  `__fastfail` — the MSVC `/GS` and STL `_STL_VERIFY` failure path, which
  terminates the process — so its call spec is marked no-return and `flow.rs`
  plants the halt it already plants for a named no-return callee. The block ends
  at the interrupt and the unbalanced stack pointer reaches no join.
- Gated three ways: the Windows compiler spec (`windows`/`clangwindows`), the
  1-byte constant vector `0x29`, and the CALLIND having to read the storage the
  `swi` CALLOTHER wrote in the same instruction. `int 0x80` is a Linux syscall and
  `int1`/`int3`/`into` carry a `return` in their own semantics; none of them match.
- Fixing the *stack model* instead — never apply `extrapop` to a `call` whose
  instruction did not push — is more general and needs no Windows gate, but it
  changes the stack solver for every architecture and `int3` genuinely does
  return. That belongs in its own change.
- No `"Subroutine does not return"` warning: the divergence is definitional, and
  one function can hold a dozen sites.

## The tests

`tests/stages/kuna-fastfailnoreturn.xml` (3 assertions, two-pass: `off` is the
bug, default is the fix) and `tests/cli/large-function-malformed-output.json`
against the vendored PE32+ above; both fail without the change. On the witness
image 22 of 858 functions change, all of them shrinking, gotos 456 → 390, and no
named call is lost; across four further PEs the only non-improvement is
byte-identical output on the one with no `int 0x29`. `make test` 675/675 PARITY
OK (baseline not re-pinned), `make test-stages` 631/631, `make rust-test` green,
`make check-spec` OK, `make test-cli` 30/30, `catalog OK`. 20.5% faster on the
witness function (1,323.65 → 1,052.49 ms, 7 samples): cutting flow decodes less.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
