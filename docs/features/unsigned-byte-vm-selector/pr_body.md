## The problem

KataVM dispatches opcode `0x8b` through an `unsigned char`, but lowered-switch
recovery prints the matching case as the unreachable signed value `-0x75`:

```bash
decompiler/target/release/kuna decompile \
  decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64 \
  0x12d0 --addr
```

```c
unsigned char v16;
/* ... */
switch(v16) {
  /* ... */
  case -0x75:
```

## The fix

- Carry signedness proven by the comparison cascade into the recovered jump
  table instead of guessing from the high bit of a case value.
- Require range comparisons and `ZEXT`/`SEXT` paths to agree; decline recovery
  on conflicting evidence or a narrowing `SUBPIECE` whose projection cannot be
  preserved across restart.
- Expose the behavior as default-on `loweredswitchlabels`; turning it off keeps
  the historical labels, while turning `loweredswitch` off keeps the comparison
  chain.

## The tests

The exact fixture acceptance now requires `case 0x8b`, and 37 focused tests
cover unsigned and signed selectors, conflicting evidence, extension paths,
and narrowing controls. The stage and promoted CLI suites pass 824/824 and
158/158; catalog, strict spec, counters, and merge guards are also green. PR
#613 closes RE need `unsigned-byte-vm-selector` in round 12.
