## The problem

A local is declared `void` with no `*` and then used as a number. On the
crackmes.one `crkme.exe` INT3-detection stub at `0x44ac6b`:

```
$ kuna decompile crkme.exe 0x44ac6b --addr
int sub_44ac6b(unsigned int a0,int a1)
{
  void *v1;
  void v4; // al
  ...
  v1 = *(void **)(v3 + 0xd4);
  if (*v1 != 0xcc)
    v5 = (*v1)();
  if (v5) {
    v4 = (void)sub_44ac7a();
    *v1 = v4;
  }
  v4 = (char)((char)v4 + '$');
```

`void v4;` cannot be declared, assigned or added in C, and the byte the stub
patches into its own callee has no type at all. A second tester hit the same
shape on a different binary's byte-XOR decode loop.

## The fix

- `code` is built with **size 1** so that `code *` arithmetic steps one byte at
  a time, and `propagateFromPointer` decides what a dereference yields by
  testing only `ptrto->getSize() == sz`. So a one-byte LOAD or STORE through a
  recovered function pointer adopts `code` as the *value* type, `INT_EQUAL`
  spreads it to the compared `0xcc`, and the C back-end prints that widthless
  scalar `void`.
- `kuna_codescalar.rs (blocks_value_type)` declines a `TYPE_CODE` pointee on the
  value side of the LOAD/STORE transfer function. The *pointer* is untouched, so
  the indirect call still renders `(*v1)()`, and a `code **` load never reaches
  the test.
- The same predicate suppresses `TypeOpStore`'s cast back to the pointee — that
  is what printed `(void)` at the store, and it is meaningless for the same
  reason.
- New option `codescalar`, default **on** (DIV-138); `option codescalar off`
  restores upstream's reading exactly.

The witness now reads `char v4; // al`, `v4 = sub_44ac7a();`, `v4 += '$'`.

## The tests

`tests/stages/kuna-codescalar.xml` is the two-pass case (assertion #1 asserts
the bug with the option off; #3/#4 the fix with it on) over a witness and a
direct-call control that must stay byte-identical in both passes.
`tests/cli/decompiler-emits-void-scalar.json` is the CLI probe over the vendored
`codescalar_x86_64` fixture.

`make test` PARITY OK 675/675 (0 assertions changed), `make test-stages` PARITY
OK (692 → 698 keys, purely additive), `make rust-test` green (6,108 passed / 0
failed), `make test-cli` 91/91, `make check-spec` OK (strict), `kuna catalog
--check` OK. Sweep over 274 crackmes.one images in both arms: 27 changed, 40 of
their 30,731 functions changed, 0 functions added or removed, and no call name
lost or gained anywhere. Speed −0.26% on the witness, −3.6% whole-binary.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
