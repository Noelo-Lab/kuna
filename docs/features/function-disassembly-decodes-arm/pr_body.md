## The problem

An ARM function's extent contains its own literal pool, so a straight-line
disassembly of `main` walks off the end of the code and decodes the constant as
an instruction. On the `1337ARM` crackme (crackmes.one
`5ab77f5733c5d40ad448c380`), the word an agent is looking for — the success value
`0x539`, i.e. 1337 — is the last row of `kuna disassemble main`, spelled as a
bitwise-and:

```
$ kuna disassemble ./1337ARM.bin main | tail -3
0x8450        10d04be2              sub sp,r11,#0x10
0x8454        10a89de8              ldmia sp,{r4,r11,sp,pc}
0x8458        39050000              andeq r0,r0,r9, lsr r5
```

Nothing executes `0x8458`; the `ldr r3,[0x8458]` five rows above reads it. The
same shape is in the repo already, in `cortexm_poolentry_le32`, where a pool word
lists as `asrs r0,r0,#0x20` / `movs r0,#0x0`:

```
$ kuna disassemble decompiler/crates/kuna-analysis/tests/fixtures/cortexm_poolentry_le32 0x8000140 --addr
0x8000142     0148                  ldr r0,[0x8000148]
...
0x8000148     0010                  asrs r0,r0,#0x20
0x800014a     0020                  movs r0,#0x0
```

## The fix

- A word the **listed range's own instructions** read at a fixed address, and
  none of them branch to, lists as the constant it holds: `.word 0x00000539`.
  The evidence comes from the p-code of each row as it is decoded
  (`ConsoleProgram::add_fixed_refs_at`) — the constant addresses it reads, in the
  two shapes SLEIGH spells one in, and the ones its flow ops name.
- Keeping the evidence inside the range is what makes the rule predictable and
  gives it an escape hatch: `kuna disassemble ./1337ARM.bin 0x8458-0x845c`
  contains no such load, so it decodes exactly as before.
- Four refusals bound it: a writable section, an address a function symbol sits
  on, an unaligned or non-scalar width, and a width that does not tile whole
  decoded rows. The last is why no address in the listing can move — a fold only
  merges whole rows over the same bytes, so a wrong fold costs one row, never a
  re-aligned listing. A `notes` line (and stderr) says a fold happened.
- No option, no emitted C: this is the listing surface, and `kuna disassemble`
  cannot change a decompile.

## The tests

`tests/cli/function-disassembly-decodes-arm.json` (the promoted acceptance,
re-pointed at the in-repo `cortexm_poolentry_le32` — CI has no dataset) plus
three `disassemble_cli.rs` cases: the fold and its escape hatch, the JSON row,
and an x86-64 body that is byte-identical, plus the halfword-read refusal on
`poolref_arm_le32`. `litpool.rs` unit-tests each refusal and `engine.rs` the
p-code projection. Sweeping every fixture in `kuna-analysis/tests/fixtures`
old-vs-new, the only listings that change are ARM/Thumb and every changed row is
a pool word. On a 32,768-instruction range of the witness, 315 rows change, and
an independent A32 literal-load decoder (no kuna in the reference path) backs
all 315 and disagrees with none of their values. Cost: one extra decode per row —
+0.5 ms on a default 115-instruction listing (below the noise floor), +0.09 s
on that 32k range (1.408 s → 1.501 s, min of 9, load-dominated).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
