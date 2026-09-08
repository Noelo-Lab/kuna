## The problem

Telling kuna that a `ret` is really a call recovers exactly one call, and the
rest of the function disappears. `push <continuation>; push <target>; ret` is a
call written without a `call` instruction, and packed images build whole bodies
out of them — one such `ret` per callee. Reclassifying one link leaves the next
link a return, so everything behind it is dead:

```console
$ kuna decompile ./bm3.exe 0x401757 --addr --assert 'flow 0x40176f call'
void sub_401757(void)
{
  LoadLibraryA(s_40151e);
}
```

That body is 22 links long. `--json` reports the directive `applied`, and
nothing in the output says the chain continues, so there is no way to find the
next `ret` to override except by disassembling by hand.

The same shape, reduced, is `retcallchain_i386` in this PR:

```console
$ kuna decompile retcallchain_i386 chain_entry --assert 'flow 0x8049019 call'
void chain_entry(void)
{
  (*dat_804a000)();          # and nothing for the two links after it
}
```

## The fix

- `call` and `callreturn` overrides now walk the chain the named link starts and
  plant themselves on the rest of it (`kuna-console/src/kuna_retcallchain.rs`).
  Both surfaces run it: the console command and the in-process seed.
- The test at each `ret` is whether the run that reached it stored that `ret`'s
  **own fall-through address** as a literal. An ordinary epilogue never pushes
  the address of the instruction after itself, and a real `call` does push its
  own fall-through but is not a `ret` — its literals are dropped for that
  reason. Nothing is reported unless the overridden address is itself a link, so
  an override anywhere in ordinary code extends to nothing.
- The walk follows unconditional direct branches, falls through everything else,
  and stops at the first non-link `ret`, an indirect branch, a decode failure, a
  revisited address, or the caps (256 links / 4096 instructions).
- Literals are tracked per varnode inside the instruction, because SLEIGH's
  `push` macros hand the immediate to a unique before the `STORE`, so the
  `STORE` value operand is never the constant.

The filed diagnosis ("the override does not extend the return-terminated
worklist") is refuted in the need record: `print raw` shows flow resuming at the
pushed continuation and running on to the next link. The continuation was never
lost; the next link was never reclassified.

## The tests

`tests/cli/flow-call-override-retain.json` (the need's acceptance, re-cut onto
the vendored fixture) plus `retcallchain_cli.rs` and `verify_retcallchain.rs`,
which cover both surfaces, each link reporting the others, the site cap, and the
two negative cases (a plain epilogue and an address outside the walk extend to
nothing). Unpatched, `overriding_the_first_link_recovers_the_whole_chain` fails
on the second call.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 687/687 PARITY OK,
`make check-spec` OK, `kuna catalog --check` OK, `make test-cli` 83/83,
`make rust-test` exit 0 (375 test binaries). The six
`override flow … callreturn` cases already in the two corpora do not move — an
ordinary `ret` is not a link.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
