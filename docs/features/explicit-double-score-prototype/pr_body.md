## The problem

A locked `double` return copied into XMM0 can be split into two four-byte cells
before an MSVC cookie check. The low cell survives, but the high cell is killed
and the return becomes an undefined `CONCAT44`:

```bash
decompiler/target/release/kuna decompile \
  decompiler/crates/kuna-analysis/tests/fixtures/pe_explicit_double_score_return_x86_64.exe \
  score \
  --assert "prototype 0x14000904f double log(double x)" \
  --assert "prototype 0x140008140 void cookie(unsigned long long value)" \
  --assert "prototype 0x140003150 double score(void *model,void *password)" \
  --option calleeretpreserves off
```

```c
free((void *)((unsigned long long)(v1 / dat_1400020f0) & 0xffffffff));
cookie(v3 ^ (unsigned long long)v6);
return (double)CONCAT44(v8,v7);
```

## The fix

- Preserve the adjacent refined cell only for an exact cookie-check call and a
  locked caller output that contains the cell.
- Keep generic body proofs limited to justified storage, with write, STORE,
  explicit-effect, scratch-register, and indirect-call refusals unchanged.
- Add a deterministic PE fixture that forces the XMM0 split.

## The tests

The promoted CLI probe requires the division result itself to be returned and
rejects both `CONCAT44` and return-side `SUB84`. Focused unit/negative tests cover
option-off, unlocked callers, writes, scratch storage, STOREs, explicit effects,
and indirect calls; the stage test also retains every cookie call.
