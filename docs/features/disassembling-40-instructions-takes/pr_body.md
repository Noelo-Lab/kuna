## The problem

`kuna disassemble` at a raw address pays for a whole-image function-discovery
walk before it prints a single row, so on a large binary a 40-instruction
listing takes as long as decompiling the program would. On a 9.4 MB PE32+ whose
`.text` is 99.4% of the file:

```
$ time kuna disassemble ./crackme_shroud.exe 0x140001000 --addr --count 40 --json | head -3
{
  "binary": "./crackme_shroud.exe",
  "kind": "code",

real    0m20.128s
```

Nothing in that is per-instruction: `--count 1` costs 20.4 s and `--count 400`
costs 20.2 s. It is the discovery walk `--mode auto` turns on — `auto` selects
`fast` from 2 MiB up, and the pass `fast` retains (`fast_funcdisc`) decodes every
executable byte in the image.

## The fix

- A window the caller bounded — `--count`, `--bytes`, or an explicit
  `start-end` range — is loaded with the analysis tier's two discovery gates
  (`listing`, `fast_funcdisc`) off. Naming either option yourself still wins.
- The walk is deferred, not dropped. It reaches a bounded listing through
  exactly two values — the target's name, and whether it resolved to an entry
  (which forces the instruction view) — so the windowed answer is kept only when
  the program named the target from a fact it already held **and** the view it
  chose does not depend on there being an entry at that address. Anything else
  reloads with the full bundle: a name only discovery invents (`sub_1190` on a
  stripped image), an address only it knows, a bare address in a data section.
  That fallback is why this is not the variant swap that breaks
  `kuna disassemble <generated name>`.
- A listing whose length is the function extent was never caller-bounded, so it
  always takes the full walk.
- Loader tolerance notes (`[kuna] ELF section table unusable…`, the PE
  data-directory clamp, the ET_REL report) are now said once per process per
  image. They are facts about the image, and a surface that loads it twice was
  announcing each of them twice.

## The tests

`tests/cli/disassembling-40-instructions-takes.json` is the acceptance probe,
repointed from its 9.4 MB dataset image to the largest in-repo fixture that
reproduces the shape (CI has no dataset) and recalibrated on it. Two cargo
tests in `kuna-cli`: `sub_1190` — a name only the discovery walk invents — still
resolves by both spellings, and a bounded listing is byte-identical to the one
the walk it skipped would have produced. Three unit tests pin the three
predicates.

Measured: 20.1 s → 1.4 s on the witness, byte-identical stdout. Sweep of 3,838
`disassemble`/`read` invocations over 139 in-repo binaries (every format and
architecture in the tree, name / address / range / mid-function / `--as data`
targets): **0 stdout differences, 0 exit-code differences**.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
