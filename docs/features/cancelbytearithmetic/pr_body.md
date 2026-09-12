## The problem

A byte that is algebraically `'w'` split a declared string initializer into
three statements. The checked-in fixture reproduces the original output:

```bash
kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/pe_subcommuteshift_x86_64.exe 0x1406fa160 \
  --assert 'data 0x1408f1c08 char usage[22]' --option cancelbytearithmetic off
```

```c
builtin_strncpy(usage,"Usage: %s <pass",0xf);
usage[0xf] = (char)(x & y) * '\xfe' + 'w' + (char)((x & y) << 1);
builtin_strncpy(&usage[0x10],"ord>\n",5);
```

## The fix

- Recognize only the complete one-byte cancellation tree and replace its outer
  add with the preserved value.
- Require exact source, width, offset and coefficient agreement plus a
  sole-consumer chain; never introduce a narrow left shift.
- Gate the divergence with default-on `cancelbytearithmetic`; `off` restores the
  upstream output above.

## The tests

The matcher/guard matrix is 11/11 green. The target acceptance and off control
pass, stages are 828/828 with parity, and the promoted CLI corpus is 160/160.
A 1,590-arm sweep compared 90,097 bodies per arm with zero differences; 15
speed samples measured -5.78%, within the 5% regression budget.
