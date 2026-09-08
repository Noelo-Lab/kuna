## The problem

`kuna strings` ends a run at every byte `>= 0x80`, so a literal that opens with a
non-ASCII character is reported starting at the byte *after* its last multi-byte
sequence. That address is not one the image refers to, so the row also comes back
with no references and no owning function.

```
$ kuna strings ./no-standards --encoding all --json --filter magical
      "address_hex": "0x200c",
      "text": ")/ so what was the magical keycombination? ",
      "length": 43,
      "encoding": "ascii",
      "xrefs_count": 0,
      "functions": []

$ kuna strings ./no-standards --encoding utf16 --json --filter magical
      "count": 0

$ kuna xrefs ./no-standards --to 0x2000 --json          # the literal's real start
      "count": 1,
      "instruction": "LEA RBX,[0x2000]"
```

The prompt is `＿φ( °-°)/ so what was the magical keycombination? ` at 0x2000 —
`ef bc bf` (U+FF3F), `cf 86` (U+03C6), then two `c2 b0` degree signs. The
reference walk already found its true start; only the reported address was wrong,
which is why the row was ownerless.

## The fix

- `--encoding utf8`, a second reading of the existing **1-byte** width rather
  than a third width: the same matcher, the same termination rule, the same
  minimum, with a well-formed UTF-8 sequence admitted as one character when its
  scalar is not a control. `all` now takes it, and `--encoding ascii` — the
  default — is unchanged.
- One 1-byte scan runs, not two. The UTF-8 reading is a provable superset of the
  ASCII one (a continuation byte is never in the 1-byte charset, so no decoded
  sequence can swallow a byte the ASCII matcher would have taken, and an
  ill-formed one costs a single byte), so running both would report the same text
  twice — once at the truncated address.
- A row is labelled from its bytes, not from which scan found it, so a run with
  no multi-byte sequence is still `ascii` and an ASCII-only image reads
  identically under all three values.
- Overlong encodings, surrogates, lead bytes past U+10FFFF and control scalars
  are declined, so a stray byte pair inside `.text` cannot join two neighbouring
  runs.
- The markup passes are untouched — they still scan at the ASCII and 2-byte
  widths — so no `char[N]` fact and no emitted C moves.

The row now reads `0x2000`, 50 characters, `encoding utf8`, `xrefs_count 1`,
`functions: [sub_1011]`.

## Tests

`tests/cli/utf-8-prompt-loses.json` is the promoted acceptance, re-pointed at a
new 9 KB `utf8prompt_x86_64` fixture (CI has no dataset) that reproduces the
witness exactly: unfixed it answers `0x10100c`, 43 characters, no functions.
`strings_cli.rs` adds the two-reading e2e plus the ASCII-only control, and the
analysis crate adds the matcher unit tests and the superset property over both
terminations.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 698/698 ·
`make rust-test` green · `make check-spec` OK · `make test-cli` 95/95 ·
`kuna catalog --check` OK. Sweep over `/usr/bin/ncat`, `/usr/bin/htop` and
`/bin/ls`: `.rodata` is byte-identical on all three under both terminations
(713/713, 856/856, 409/409 rows); every changed row is in `.text`, the documented
cost of scanning code at all. `kuna strings --encoding all --no-xrefs` on htop:
0.01 s before and after.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
