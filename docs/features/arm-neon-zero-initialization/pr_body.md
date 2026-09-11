## The problem

A NEON immediate that fills a stack buffer is decompiled as reads through a null
pointer. The word at offset 0 comes out right; the ones that need an offset do
not.

```sh
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/neonimm_arm_le32 \
      0x10004 --addr --option constspaceload off
```

```c
  v1._0_8_ = 0x100000001;                      // vmov.i32 q8,#1
  v1._0_16_ = v1._0_16_ << 0x40 | v1._0_16_;
  v2 = *(unsigned int *)v1._0_16_;
  v3 = *(unsigned int *)(v1._0_16_ + 4);
  v7 = *(unsigned int *)(0 + 4);               // vmov.i32 q9,#0
  v8 = *(unsigned int *)(0 + 8);
  v9 = *(unsigned int *)(0 + 0xc);
  v6 = 0;
```

The fixture is twelve instructions: two `vmov.i32` immediates, each followed by
a `vst1.64` writeback store into a 32-byte stack buffer whose address is passed
to a callee. Every value above should be a `1` or a `0`. The unfolded `0 + k`
address expressions are the tell that the defect is in the *source* of the store
and not in the store: constant folding would have printed `4`. Filed from
crackmes.one `68d40081224c0ec5dcedc2d2` (ARM32 ELF `trap`, `_start_main`), whose
`__libc_init` structure is zeroed by one `vmov.i32 q8,#0`.

## The fix

- A SLEIGH constructor ending in `export *[const]:N v` is a *dynamic*
  constant-space export and lowers to `LOAD(const, v)`. In the constant space an
  address is its own value, so that LOAD **is** `v`, resized — the identity
  `RuleLoadVarnode` already applies, but only to a constant `v`.
  `RuleCollapseConstants` cannot supply one for a 16-byte `v`, since
  `PcodeOp::isCollapsible` declines any op wider than a `uintb`. New rule
  `RuleConstSpaceLoad` applies the identity to a defined pointer: `COPY p`,
  `SUBPIECE(p, 0)` or `INT_ZEXT p` by width.
- It runs in oppool1, before `ActionLaneDivide` — the point of the change.
  Splitting a const-space LOAD into lane LOADs at `v+4`, `v+8`, `v+0xc` is not a
  narrowing but a different value, which is why lane 0 is right and folding the
  rest where they stand would print the literals `4`, `8`, `0xc`.
- Not done the obvious way: widening `RuleLoadVarnode` to fold a 16-byte
  const-space load works only because this need's immediate is zero, and
  silently truncates every other `simdExpImm_16` arm. `vmov.i32 q8,#1` is in the
  fixture and in the stage test for that reason.
- A constant pointer stays with `RuleLoadVarnode`, which also clears the
  spacebase-placeholder trigger a bare COPY would drop; free pointers and
  placeholder outputs are declined. Shipped as `constspaceload`, default **on**
  (DIV-158).

## The tests

`tests/stages/ghdec-constspaceload.xml` runs the fixture twice and pins both
arms; six unit tests cover the three width arms and each declining clause;
`tests/cli/arm-neon-zero-initialization.json` is the promoted acceptance probe.
`make test` 675/675 and `make test-stages` 763/763 PARITY OK with no
pre-existing assertion moved. `decompile-all` sweep, option off vs default:
over 119 binaries compared under both arms (5 linked binaries from tests/bug-repro incl. betaflight_STM32F405.elf, plus RE-dataset challenge images; 10 exceeded the 150 s cap or failed to load under both arms and were skipped): 3 changed, all three the same class -- an AVX-512 static glibc strlen/memchr family where a k mask-register value is now rendered at its true width (`& 0xff` for the 8-lane `vpcmpd`, `& 0xffffffff` for the 32-lane `vpcmpub`) instead of an opaque cast off an unfoldable const-space LOAD. No statement is deleted in any of them.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
