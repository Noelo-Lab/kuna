## The problem

`kuna strings` took only NUL-terminated runs, so a length-prefixed name table —
each identifier preceded by its own length byte, none of them terminated — was
reported as no strings at all. That is the shape a bundled JavaScript or bytecode
payload carries, and on the reported 977 KB Node bundle it is most of `.rodata`:

```console
$ xxd -s 0xb2020 -l 32 crackme.x86_64.elf
000b2020: 43c7 050c 6f75 742e 6a73 0673 7464 046f  C...out.js.std.o
000b2030: 7312 5f30 7838 6563 3662 3312 5f30 7833  s._0x8ec6b3._0x3

$ strings -a crackme.x86_64.elf | grep -cE '^(_0x|out\.js)'
635

$ kuna strings crackme.x86_64.elf --section .rodata --filter '_0x|out.js' \
      --encoding all --min-length 4 --json
{ ..., "count": 0, "strings": [] }
```

## The fix

- The matcher takes a termination policy (`strings/mod.rs (scan_runs)`, mirrored
  at 2-byte width in `kuna_widestrings.rs`). `Nul` is Ghidra's `requireNullEnd`;
  `Any` also accepts a run closed by an ordinary byte or by the end of its region.
- `kuna strings` gains `--termination nul|any` and **defaults to `any`** — the
  probe's command line carries no new flag, and `strings(1)`'s rule is what an
  analyst is comparing against. `--termination nul` restores the previous view.
  On the reported image the default now reports 5,138 rows where `strings -a -n 5`
  reports 5,164; the difference is the non-allocated sections kuna does not scan.
- Rows say which ending they had: `nul_terminated` per row, `termination` at the
  top level and in the text header. An unterminated row's `byte_length` stops at
  its last visible byte, so an xref extent never claims a terminator that is not
  there.
- The markup passes are untouched. `StringLiteralPass` and `widestrings` ask for
  `Nul` and commit exactly the facts they always did, because only a NUL-ended run
  describes a `char[N]`. No emitted C moves — `make test` is 675/675.

The cost is the one `strings(1)` pays: a printable run inside code is usually
instruction bytes. 2,565 of the 3,362 added rows on that image are in `.text`;
`--section`, `--filter` and `--termination nul` each remove them.

## The tests

`tests/cli/string-inventory-include-non.json` is the acceptance, retargeted onto a
new 8,528-byte vendored fixture (`nametable_x86_64`, built by the `.py` beside it)
whose `.rodata` carries the same table plus one ordinary literal; it answers
`count 0` under `--termination nul` and pre-fix. Four cargo tests cover the
matcher, three the CLI. Sweep: over all 141 scannable vendored fixtures, every one
of 19,375 `--termination nul` rows is reproduced byte-identically by the new
default (address, text, length, byte_length, section) and 2,352 rows are added —
none lost, none changed.

Two existing expectations moved with the default and are updated, not weakened:
`strings-rejects-mach-o.json` and its cargo twin counted 5 rows at `--min-length
2` on `macho_fat` and now count 8 (the added test asserts every row is still in
the x86-64 slice); the fauxware header count is 13 → 14.

Gates: `make test` PARITY OK (675/675) · `make test-stages` PARITY OK (683/683) ·
`make test-cli` 78/78 · `make check-spec` OK · `kuna catalog --check` OK ·
`make rust-test` green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
