## The problem

`kuna disassemble` on an ARM function walks past the epilogue into the function's
own literal pool. The first pool word is a bit pattern the ARM translator will
not decode, and the walk recovers from that by advancing **one byte** — so every
row after it starts at an address no ARM instruction can begin at:

```
$ kuna disassemble decompiler/crates/kuna-analysis/tests/fixtures/armpoolgrid_le32 0x10000 --addr
# 13 instructions at sub_10000 @ 0x10000 (0x10000..0x10031, 49 bytes)
...
0x10008       10009fe5              ldr r0,[0x10020]
0x1000c       10109fe5              ldr r1,[0x10024]
0x10010       10209fe5              ldr r2,[0x10028]
0x10014       10309fe5              ldr r3,[0x1002c]
0x10018       010080e0              add r0,r0,r1
0x1001c       0088bde8              ldmia sp!,{r11,pc}
0x10020       b8                    .byte 0xb8
0x10021       feffff89              ldmibhi pc!,{r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,sp,lr,pc}^
0x10025       feffff84              ldrbthi pc,[pc],#0xffe
0x10029       fdffff3f              swicc 0xfffffd
0x1002d       feffff00              ldrshteq pc,[pc],#0xfe
```

(the fixture is added by this PR; `python3 …/armpoolgrid_le32.py` regenerates it.)

Four `ldr`s name `0x10020`/`0x10024`/`0x10028`/`0x1002c` as constants, but after
one byte of drift none of them starts a decoded row, and pool-word folding
requires an exact tiling of whole rows — so all four are declined and the listing
hands back four invented instructions in place of the four constants. The
reported extent, `0x10031`, is not even 4-aligned. On the witness this cost a
whole five-word pool to one refused byte.

## The fix

- The recovery row now runs **to the next instruction-alignment boundary** and
  the walk resumes there, instead of advancing one byte. It is one row spanning
  the gap, not a run of one-byte rows — a row left at `0x10021` would keep the
  listing off the grid just the same.
- The grid is the alignment the listing's own decoded rows all share (the OR of
  their addresses and sizes), floored by the architecture's SLEIGH
  `define alignment`. An ARM listing of 4-byte rows resumes on 4; a Thumb listing
  that has decoded a 2-byte row resumes on 2; an architecture that declares no
  alignment, or a walk with nothing decoded yet, keeps the byte-at-a-time
  recovery unchanged — so x86 listings are byte-identical.
- Not read from the ISA alignment alone: ARM declares `alignment=2` because of
  Thumb, which is not enough to put an A32 listing back on its 4-byte grid.
- `.byte` rows now spell every byte they cover (`.byte 0xb8,0xfe,0xff,0xff`).

Nothing else changed: this is the listing walk in `kuna-cli`, no engine pass and
no emitted C.

## The tests

`tests/cli/arm-literal-pool-disassembly.json` (the promoted acceptance) and three
new cases in `disassemble_cli.rs` — the grid-resume listing, the "nothing decoded
yet" fallback, and unit cases for `resume_grid`/`recovery_span` including the
`alignment == 1` short-circuit. All fail without the fix. A/B over the witness
binary's 13 functions: 9 byte-identical, 4 changed, and each of the four is this
same defect fixed — its pool words back as `.word` rows and its extent 4-aligned
again.

Gates: `make test` 675/675 PARITY OK · `make test-stages` 654/654 PARITY OK ·
`make rust-test` 5,802 passed · `make check-spec` OK · `make test-cli` 58/58 ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
