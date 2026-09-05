## The problem

`--define-function START-END` is how an agent tells kuna where a function really
is on a packed or obfuscated image. Declaring an end that any branch crosses
killed the whole function instead of clipping it:

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 \
      0x1070 --addr --define-function 0x1070-0x1098=deregister
error: Could not find op at target address: (ram,0x00001098)
$ echo $?
1
```

`0x1098` is the declared exclusive end, and `0x1081` is `JZ 0x1098` — an
ordinary forward conditional over the tail. `--mode reliable` and `--mode fast`
fail identically. `docs/cli.md` already promised the opposite ("a declared `end`
that cuts real control flow is reported rather than silently truncating the
body"), and the warning it promises exists; nothing could reach it.

The second half is worse and shows up on *correct* boundaries. Declaring the
extent kuna itself derived should be free; it deleted the last instruction:

```console
$ kuna decompile ...aif_gap_x86_64 sub_1129                 # derived extent, undeclared
int sub_1129(int a0)
{
  return (a0 + 10) * 2;
}
$ kuna decompile ...aif_gap_x86_64 sub_1129 --define-function 0x1129-0x1141
void sub_1129(void) // warn: Function flows out of bounds
{ // warn: Function flow out of bounds: r0x00001140 flows to r0x00001140
}
```

## The fix

Both live in `FlowInfo` (`p2_lift/flow.rs`); both are inert unless a caller
declares an extent, because `set_range` is the only thing that narrows the flow
range and only a declared extent calls it.

- **A branch target outside the extent resolves to its stub.**
  `fillin_branch_stubs` already plants a `missing` artificial halt at every
  referenced-but-undecoded address; it is now also registered in `visited` as
  the instruction there, so `collect_edges` hangs the cut edge on it and the
  body ends at the boundary under the `Function flows out of bounds` header.
- **Only the out-of-extent subset gets that.** The addresses
  `handle_out_of_bounds` recorded are tracked separately. An unprocessed
  address *inside* the extent means an op that should exist does not, and
  resolving that to a halt would shorten a function instead of reporting the
  defect — it keeps upstream's throw. (That is the distinct open need
  `default-decompilation-fails-despite`, whose missing target is an in-extent
  NOP; its acceptance still fails, deliberately.)
- **`eaddr` is the last in-body byte, so an instruction starting on it is in
  range.** The fall-through bound tested `bound <= addrlist.back()` with
  `bound == eaddr`, which upstream can only hit at the top of memory but which
  under a declared extent is every function's last instruction. It now decodes
  that instruction and catches the fall-through past it on the next lap, where
  the address really is above `eaddr`.

## The tests

`tests/cli/explicit-function-boundary-aborts.json` is the promoted acceptance,
re-aimed at the in-repo fixture above (the dataset witness is a 7 MB PE CI has
no copy of; same shape, same clauses). Plus three cargo tests, each failing
without the change: the branch-cut case and the declared-derived-extent oracle
in `kuna-console/tests/verify_funcbounds.rs`, and the in-extent/out-of-extent
split in `kuna-decomp/tests/verify_w3_ir_flow.rs`.

Measured over the 112 binary fixtures under `kuna-analysis/tests/fixtures`,
declaring every discovered function at its derived extent (2,765 functions):
bodies differing from the undeclared decompile **403 → 138**, functions that
hard-error under the declaration **142 → 0**. With nothing declared,
`decompile-all --json` over the same 112 binaries is **112/112 byte-identical**.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 635/635 PARITY OK,
`make rust-test` green, `make check-spec` OK (lenient + strict), `make test-cli`
35/35, `kuna catalog --check` OK. No new option, no catalog counter, no stages
XML.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
