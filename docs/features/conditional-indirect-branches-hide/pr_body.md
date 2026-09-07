## The problem

When a dispatch picks its destination with a conditional move instead of a
table, kuna throws away everything behind the branch — even though both
destinations are already constants it printed.

```
$ printf '\x48\x85\xff\x49\xb8\x30\x00\x10\x00\x00\x00\x00\x00\x49\xb9\x20\x00\x10\x00\x00\x00\x00\x00\x4d\x0f\x44\xc8\x41\xff\xe1' \
  | cstool -a 0x100000 x64 -
 0x100000: 48 85 ff        test rdi, rdi
 0x100003: 49 b8 ...       movabs r8, 0x100030
 0x10000d: 49 b9 ...       movabs r9, 0x100020
 0x100017: 4d 0f 44 c8     cmove r9, r8
 0x10001b: 41 ff e1        jmp r9

$ kuna decompile ./a.out csjmp
void csjmp(long a0)
{
  void *v1; // r9

  v1 = (void *)0x100020;
  if (!a0)
    v1 = (void *)0x100030;
  (*v1)(); // jump-as-call
}
```

`0x100020` and `0x100030` are right there, and neither is decoded. On the
witness that filed this (crackmes.one `5b52f6eb33c5d41c0b8ae55f`, the Mach-O
`LOL` decoder at `0x10003c9e0`) that is the *entire* function: nine lines, no
decode loop, and the reverse engineer is done.

## The fix

- New option `constselectjump` (P2 switch-model, transform, default off).
  When both value-range jump-table models decline, walk the `MULTIEQUAL`/`COPY`
  web behind the `BRANCHIND` input; if every reaching definition is a constant,
  install a fixed address table over that set with the target addresses as case
  labels. Both arms are then ordinary intraprocedural flow.
- Why a third model rather than a fix to `JumpBasic`: `isprune` stops the
  backward walk at any marker op, by design, so the `MULTIEQUAL` output *is* the
  normalized switch variable and there is no range to find. The set is not lost,
  it is unpromoted — which is what `JumpModelTrivial`'s shape is for.
- Two guards, because a conditional **tail call** between two function entries is
  the same shape and recovering it would splice both callees into the caller: a
  destination must lie within `0xffff` of the branch (upstream's own
  intraprocedural-vs-thunk yardstick in `JumpTable::sanityCheck`), and every
  destination must have loadable bytes behind it or the table is declined whole.
  Every rejection path returns the branch to the pre-change `CALLIND` rendering.
- Default off because that tail-call ambiguity is a judgment call; carried by
  `--mode aggressive`, which `auto` selects under 500 KiB (DIV-131).

With it on, `csjmp` above becomes:

```c
long csjmp(long a0,long a1)
{
  unsigned long v1; // r9

  v1 = 0x100020;
  if (!a0)
    v1 = 0x100030;
  switch(v1) {
    case 0x100020:
      return a1 + 1;
    case 0x100030:
      return a1 + -1;
  }
}
```

## The tests

`tests/stages/re-constselectjump.xml` is the two-pass case (pass 1 default = the
computed call, pass 2 `option constselectjump on` = the switch plus both arms);
`tests/cli/conditional-indirect-branches-hide.json` is the promoted regression
probe over a vendored 5 KB fixture. Gates: `make test` PARITY OK 675/675
(baseline not re-pinned), `make test-stages` PARITY OK (674 → 677 keys, purely
additive), `make rust-test` green, `make check-spec` OK, `catalog OK`,
`make test-cli` 63/63.

`decompile-all --mode aggressive` over 108 binaries, both arms, all exiting 0 in
both: 106 byte-identical. The two that differ are the witness and one real
function — `replace_problematic_chars` in `O2-noinline/coreutils/df`, where gcc
compiled `(tty_out ? replace_invalid_chars : replace_control_chars)(cell)` into
exactly this dispatch. That is the tail-call trade-off named above rather than a
defect: the C is faithful, but both callee bodies are now inlined as switch arms
where the two callee names used to be visible. It is why the catalog default is
off. Speed +0.82% (fixture, median of 9) and +1.28% (`coreutils/ls main`,
median of 7), inside the 5% budget — the model only runs where recovery had
already failed.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
