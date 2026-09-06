## The problem

A VM interpreter keeps two four-word register banks in 16 bytes of stack each
and swaps them with a `movdqa`/`movaps` quartet. kuna declares both banks as
`char [16]` and then prints the swap one byte wide.

```
$ kuna decompile KataVM_L1 0x12d0 --addr
unsigned long sub_12d0(void)
{
  ...
  char v30 [16]; // stack - 0x3b78
  char v32 [16]; // stack - 0x3b68

  v30[0] = 0;                     <- a 16-byte movaps zero store
  v32[0] = 0;
  ...
  v30._0_4_ = v32._12_4_;         <- the 4-byte accesses DO carry their width
  ...
label_1410:
  v30[0] = v32[0];                <- a 16-byte movaps transfer
  v32[0] = v13;
```

`v30[0]` names a single `char`. The bytes at `0x1410` are
`movdqa (%rsp),%xmm0 / movdqa 0x10(%rsp),%xmm6 / movaps %xmm6,(%rsp) /
movaps %xmm0,0x10(%rsp)`, and the p-code keeps all sixteen bytes — the loss is
in the printer. The same function four lines earlier prints `v30._0_4_`, so one
variable is spelled two ways for accesses four times apart in width.

To see it without the crackme, on the fixture this PR vendors:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/arraycoverwidth_x86_64 vm \
      --option arraycoverwidth off
  v2[0] = 0;
  v3[0] = 0;
  ...
  v2[0] = v3[0];
```

## The fix

- `PrintC::pushSymbolDetail` sends a symbol-mapped access into
  `pushPartialSymbol`, whose walk breaks out at the top on a whole-symbol cover
  and leaves the caller to render. kuna's caller has a whole-array
  `name[index]` branch computing `index = symboloff / elementAlignSize` that
  never reads the access size, so every full-width cover printed element zero.
- `option arraycoverwidth` (default on) suppresses that top-of-walk break for a
  `TYPE_ARRAY` whose element stride is smaller than the access. The cover then
  falls to the same artificial `._<off>_<size>_` field a *partial* multi-element
  access already gets, so both spellings come from one code path:
  `v30._0_16_ = v32._0_16_;`.
- Fixing the `name[index]` guard instead was rejected: it yields `v30 = v32;`
  and `v30 = 0;`, which is not valid C either, and drops the width rather than
  stating it.
- The predicate is narrow on purpose. Scalars, structs, unions and any access
  that fits inside one element keep the upstream break, so `g[3]` and every
  genuine subscript are untouched.

## The tests

`tests/stages/kuna-arraycoverwidth.xml` is the two-pass case on a
project-authored reduction of the witness (`arraycoverwidth_x86_64.s`): default
pins `v2._0_16_ = v3[0]._0_16_;` and `return v2[3];`, `arraycoverwidth off`
pins the filed `v2[0] = v3[0];`. `tests/cli/16-byte-vm-state.json` is the
promoted probe on the same fixture.

Seven existing stage assertions moved and all seven are the same defect —
`v1[0] = (char[8])msg._0_8_` and `CONCAT124(v3[0],a1)` (a twelve-byte read) are
the clearest. `make test` PARITY OK 675/675 (0 changed), `make test-stages`
PARITY OK 640/640, `make check-spec` OK lenient + strict, `make test-cli` 42/42,
`kuna catalog --check` OK. `decompile-all` over 40 images changed 1,979 lines
and every one is a 1:1 token substitution: after erasing the `._N_M_` suffix and
the `[0]` subscript the removed and added line multisets are identical on every
file, so no statement was added, deleted or re-anchored.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
