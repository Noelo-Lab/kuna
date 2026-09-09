## The problem

A finite-field squaring loop is emitted with its bit counter overwritten by the
accumulator's zero before the loop ever tests it, so the loop body is dead for
every input. Build the fixture this PR vendors and decompile it:

```
$ gcc -nostdlib -static -Wl,-e,_start -o covertrim_x86_64 covertrim_x86_64.s
$ kuna decompile ./covertrim_x86_64 gfinv
      v6 = 0;
      v3 = a0;
      while (a0 = v6, v7 = a0 != 0, v7) {
        v6 = v3 ^ a0;
        v1 = a0 & 1;
        ...
        a0 >>= 1;
      }
```

`a0` is doing two jobs at once. The machine has `MOVZX R11D,CL` initialising the
bit counter from the input and `XOR R9B,R9B` zeroing the accumulator; both
registers are live through the loop, and here they share one variable, so the
counter starts at 0 and the loop is skipped. The same shape appears at O0 and O2
in ordinary programs — in `coreutils` `sort` it merged a function's `parent`
parameter with a fresh `node_pool` pointer and emitted the self-referential
`parent = node_pool; ... parent->parent = (merge_node *)parent;`.

## The fix

- `Funcdata::set_varnode_properties` re-allocates a Varnode's `Cover` after
  `op_unset_output` has dropped it. C++ does that through
  `Varnode::calcCover` → `setFlags(coverdirty)`, and `setFlags` forwards the bit
  to `high->coverDirty()`. kuna's arena model cannot reach the HighVariable bank
  from `Varnode::set_flags`, and this one reconciliation site was missing it.
- The consequence is a member Varnode holding a *fresh, empty* `Cover` inside a
  HighVariable that still reads clean, so `HighIntersectTest::update_high` skips
  the rebuild. `Cover::intersect` scores an empty block 0, the member looks live
  nowhere, and an overlapping merge is accepted. It bites the copy trims
  `Merge::merge_op` inserts, whose outputs are re-pointed *after* they have
  joined a high.
- Restoring the forward can only make an intersection test see more cover, so
  merges can only be refused, never accepted — the change cannot move or delete a
  statement.

## The tests

`tests/cli/finite-field-squaring-loop.json` (promoted acceptance probe) plus the
`covertrim_x86_64` fixture it runs on; both of its clauses fail without the fix.
A unit test in `funcdata.rs` pins the dirty-bit forward directly. A 41-image
`decompile-all` sweep left 33 unchanged; of the 8 that moved, every hunk is a
variable split, and two were read against the disassembly and are corrections —
in `libedit`'s `history_truncate_file` the old output dropped an `addq $0x1` on
the stack pointer slot entirely.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 714/714 ·
`make rust-test` green · `make check-spec` OK · `catalog OK` · `make test-cli`
103/103. Speed, median of 5 per image: sort −1.00%, dpkg +1.63%, libedit +0.56%,
minigzip −4.44%.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
